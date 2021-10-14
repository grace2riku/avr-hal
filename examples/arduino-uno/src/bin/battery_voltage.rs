#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
use embedded_hal::prelude::_embedded_hal_blocking_i2c_Write;
use embedded_hal::prelude::_embedded_hal_blocking_i2c_Read;

const BATT_ADC_ADDR: u8 = 0x50;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();

    led.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
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

        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
