
use arduino_mega2560::{Peripherals, prelude::*};
use arduino_mega2560::pwm;

pub fn init(pins:&arduino_mega2560::Pins, peripherals_adc:arduino_mega2560::pac::ADC){
    let mut adc = arduino_mega2560::adc::Adc::new(peripherals_adc, Default::default());
    let mut a0 = pins.a0.into_analog_input(&mut adc);
}

