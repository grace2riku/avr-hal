#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use panic_halt as _;

const BATT_ADC_ADDR: u8 = 0x50;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output(&mut pins.ddr);

    led.set_high().void_unwrap();

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );
    let mut i2c = arduino_uno::I2cMaster::new(
        dp.TWI,
        pins.a4.into_pull_up_input(&mut pins.ddr),
        pins.a5.into_pull_up_input(&mut pins.ddr),
        100000,
    );
	
    loop {
		let adc_hedder: [u8; 1] = [0x00];
    	i2c.write(BATT_ADC_ADDR, &adc_hedder);    

		let mut receive_buffer: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
		i2c.read(BATT_ADC_ADDR, &mut receive_buffer[0..2]);

        ufmt::uwriteln!(&mut serial, "[0]={}, [1]={}, [2]={}, [3]={}\r", receive_buffer[0], receive_buffer[1], receive_buffer[2], receive_buffer[3]).void_unwrap();

		// when ADC is not connected, read values are 0xFF: 
		if receive_buffer[0] == 0xff && receive_buffer[1] == 0xff {
			receive_buffer[0] = 0x00;
			receive_buffer[1] = 0x00;
		}

		// voltage mV = adcVal * Vref(3.3V) / resolution(8bit) * Vdiv(2) 
		let temp_millivolt = ( ( (receive_buffer[0] << 4) | (receive_buffer[1] >> 4) ) as u32 * 3300 * 2) / 256; 
//		let data_batt = temp_millivolt / 1000;

/*
		let temp_millivolt = ( ( (receive_buffer[0] << 4) | (receive_buffer[1] >> 4) ) as f32 * 3300.0 * 2.0) / 256.0; 
		let data_batt = (temp_millivolt / 1000.0);
*/
        ufmt::uwriteln!(&mut serial, "Batt[V] = {}\r", temp_millivolt).void_unwrap();

        led.toggle().void_unwrap();
        arduino_uno::delay_ms(1000);
    }
}
