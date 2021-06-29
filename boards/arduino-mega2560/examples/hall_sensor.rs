use arduino_mega2560::prelude::*;
use arduino_mega2560::hal::port;
use crate::led::set_hall_u;
use crate::led::set_hall_v;
use crate::led::set_hall_w;
use crate::motor_control::set_fet_drive_pattern;

// hall sensor-U d19, PD2
static mut HALL_U_PIN: Option<port::portd::PD2<port::mode::Input<port::mode::PullUp>>> = None;
// hall sensor-V d20, PD1
static mut HALL_V_PIN: Option<port::portd::PD1<port::mode::Input<port::mode::PullUp>>> = None;
// hall sensor-W d21, PD0
static mut HALL_W_PIN: Option<port::portd::PD0<port::mode::Input<port::mode::PullUp>>> = None;

#[avr_device::interrupt(atmega2560)]
unsafe fn INT2() {
    set_fet_drive_pattern();
    set_hall_u(get_u_level());
}

#[avr_device::interrupt(atmega2560)]
unsafe fn INT1() {
    set_fet_drive_pattern();
    set_hall_v(get_v_level());
}

#[avr_device::interrupt(atmega2560)]
unsafe fn INT0() {
    set_fet_drive_pattern();
    set_hall_w(get_w_level());
}

pub fn init(ei: arduino_mega2560::pac::EXINT, 
            u_phase: port::portd::PD2<port::mode::Input<port::mode::PullUp>>,
            v_phase: port::portd::PD1<port::mode::Input<port::mode::PullUp>>,
            w_phase: port::portd::PD0<port::mode::Input<port::mode::PullUp>>){

    unsafe {
        HALL_U_PIN = Some(u_phase);
        HALL_V_PIN = Some(v_phase);
        HALL_W_PIN = Some(w_phase);
    }

    // INT2 hall sensor-U 両エッジ割り込みに設定
    ei.eicra.write(|w| w.isc2().bits(0x01));

    // INT1 hall sensor-V 両エッジ割り込みに設定
    ei.eicra.write(|w| w.isc1().bits(0x01));

    // INT0 hall sensor-W 両エッジ割り込みに設定
    ei.eicra.write(|w| w.isc0().bits(0x01));

    // INT2,1,0 interrupt enable
    ei.eimsk.write(|w| w.int().bits(0x07));

}


pub fn get_position() -> u8{
    unsafe {
        (HALL_W_PIN.as_mut().unwrap().is_high().void_unwrap() as u8) << 2 |
        (HALL_V_PIN.as_mut().unwrap().is_high().void_unwrap() as u8) << 1 |
        HALL_U_PIN.as_mut().unwrap().is_high().void_unwrap() as u8
    }
}

pub fn get_u_level() -> bool{
    unsafe {
        HALL_U_PIN.as_mut().unwrap().is_high().void_unwrap()
    }
}

pub fn get_v_level() -> bool{
    unsafe {
        HALL_V_PIN.as_mut().unwrap().is_high().void_unwrap()
    }
}

pub fn get_w_level() -> bool{
    unsafe {
        HALL_W_PIN.as_mut().unwrap().is_high().void_unwrap()
    }
}
