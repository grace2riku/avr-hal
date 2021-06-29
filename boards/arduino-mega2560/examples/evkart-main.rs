#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

//extern crate panic_halt;
use core::u8;

use arduino_mega2560::prelude::*;
use arduino_mega2560::pwm;

mod motor_control;
mod hall_sensor;
mod led;
mod timer;

const VOL_0PCT_POINT: u8 = 51; // スロットル電圧 1.0V 205/4
//const VOL_0PCT_POINT: u16 = 51; // スロットル電圧 1.0V 205/4

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();

    let mut pins = arduino_mega2560::Pins::new(
        dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF, dp.PORTG, dp.PORTH,
        dp.PORTJ, dp.PORTK, dp.PORTL,
    );

    let mut adc = arduino_mega2560::adc::Adc::new(dp.ADC, Default::default());
    let mut a0 = pins.a0.into_analog_input(&mut adc);

    let mut delay = arduino_mega2560::Delay::new();

    timer::init(dp.TC0);

    let mut serial = arduino_mega2560::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    let hall_u = pins.d19.into_pull_up_input(&mut pins.ddr);
    let hall_v = pins.d20.into_pull_up_input(&mut pins.ddr);
    let hall_w = pins.d21.into_pull_up_input(&mut pins.ddr);
    hall_sensor::init(dp.EXINT, hall_u, hall_v, hall_w);

    let user_led = pins.d13.into_output(&mut pins.ddr);
    let hall_u_led = pins.d23.into_output(&mut pins.ddr);
    let hall_v_led = pins.d25.into_output(&mut pins.ddr);
    let hall_w_led = pins.d27.into_output(&mut pins.ddr);
    led::init(user_led, hall_u_led, hall_v_led, hall_w_led);

	// PWM
	// FET U hi:D5 PE3/timer:0C3A
	// FET V hi:D2 PE4/timer:0C3B
	// FET W hi:D3 PE5/timer:0C3C
	// F_pwm = CLK_io / (Prescaler * 256);
	// https://rahix.github.io/avr-hal/arduino_mega2560/pwm/enum.Prescaler.html
    let mut timer3 = pwm::Timer3Pwm::new(dp.TC3, pwm::Prescaler::Prescale64);
    let fet_u_high_pin = pins.d5.into_output(&mut pins.ddr).into_pwm(&mut timer3);
    let fet_v_high_pin = pins.d2.into_output(&mut pins.ddr).into_pwm(&mut timer3);
    let fet_w_high_pin = pins.d3.into_output(&mut pins.ddr).into_pwm(&mut timer3);
    let fet_u_low_pin = pins.d6.into_output(&mut pins.ddr);
    let fet_v_low_pin = pins.d7.into_output(&mut pins.ddr);
    let fet_w_low_pin = pins.d8.into_output(&mut pins.ddr);

    motor_control::pwm_init(fet_u_high_pin, fet_v_high_pin, fet_w_high_pin,
                            fet_u_low_pin, fet_v_low_pin, fet_w_low_pin);

    // Enable interrupts
    unsafe {
        avr_device::interrupt::enable();
    }

//    ufmt::uwriteln!(&mut serial, "Hello from evkart app!!!\r").void_unwrap();

    loop {
        if motor_control::get_speed_control_timing() == true {
            let ad0:u16 = nb::block!(adc.read(&mut a0)).void_unwrap();
            motor_control::save_pwm_duty((ad0 / 4) as u8);
//            motor_control::save_pwm_duty(ad0);

            if motor_control::load_pwm_duty() > VOL_0PCT_POINT {
                if motor_control::get_drive_state() == motor_control::DriveState::Stop {
                    motor_control::set_drive_state(motor_control::DriveState::Drive);
                    //モータ停止中のため強制駆動する
                    motor_control::set_fet_drive_pattern();
                }
            } else {
                motor_control::set_drive_state(motor_control::DriveState::Stop);
                motor_control::save_pwm_duty(0);
                // モータを停止する
                motor_control::set_fet_stop_pattern();
            }
    
            motor_control::set_speed_control_timing(false);
        }

//        ufmt::uwrite!(&mut serial, "pwm_duty={}\r\n", motor_control::load_pwm_duty()).void_unwrap();
//        delay.delay_ms(1000u16);
    }
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial: arduino_mega2560::Serial<arduino_mega2560::hal::port::mode::Floating> = unsafe {
        core::mem::MaybeUninit::uninit().assume_init()
    };

    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();

    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        ).void_unwrap();
    }

    loop {}
}
