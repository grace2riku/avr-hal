use arduino_mega2560::{hal::port, prelude::_embedded_hal_PwmPin};
use arduino_mega2560::pwm;
use avr_hal_generic::{hal::digital::v2::OutputPin, void::ResultVoidExt};

use crate::hall_sensor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriveState {
    Stop,
    Drive
}

static mut DRIVE_STATE: DriveState = DriveState::Stop; 
static mut SPEED_CONTROL_TIMING: bool = false; 

static mut PWM_DUTY: u8 = 0;
//static mut PWM_DUTY: u16 = 0;

// ホールセンサポジション ホールセンサ入力ポートから読み取った値
pub const HALL_SENSOR_POSITION_1:u8 = 1;
pub const HALL_SENSOR_POSITION_2:u8 = 2;
pub const HALL_SENSOR_POSITION_3:u8 = 3;
pub const HALL_SENSOR_POSITION_4:u8 = 4;
pub const HALL_SENSOR_POSITION_5:u8 = 5;
pub const HALL_SENSOR_POSITION_6:u8 = 6;

// PWM
// FET U hi:D5 PE3/timer:0C3A
// FET V hi:D2 PE4/timer:0C3B
// FET W hi:D3 PE5/timer:0C3C
static mut FET_U_HIGH_PIN: Option<port::porte::PE3<port::mode::Pwm<pwm::Timer3Pwm>>> = None;
static mut FET_V_HIGH_PIN: Option<port::porte::PE4<port::mode::Pwm<pwm::Timer3Pwm>>> = None;
static mut FET_W_HIGH_PIN: Option<port::porte::PE5<port::mode::Pwm<pwm::Timer3Pwm>>> = None;
static mut FET_U_LOW_PIN: Option<port::porth::PH3<port::mode::Output>> = None;
static mut FET_V_LOW_PIN: Option<port::porth::PH4<port::mode::Output>> = None;
static mut FET_W_LOW_PIN: Option<port::porth::PH5<port::mode::Output>> = None;

pub fn pwm_init(fet_u_high_pin:port::porte::PE3<port::mode::Pwm<pwm::Timer3Pwm>>, 
                fet_v_high_pin:port::porte::PE4<port::mode::Pwm<pwm::Timer3Pwm>>,
                fet_w_high_pin:port::porte::PE5<port::mode::Pwm<pwm::Timer3Pwm>>,
                fet_u_low_pin:port::porth::PH3<port::mode::Output>,
                fet_v_low_pin:port::porth::PH4<port::mode::Output>,
                fet_w_low_pin:port::porth::PH5<port::mode::Output>){

    unsafe {
        FET_U_HIGH_PIN = Some(fet_u_high_pin);
        FET_V_HIGH_PIN = Some(fet_v_high_pin);
        FET_W_HIGH_PIN = Some(fet_w_high_pin);
        FET_U_LOW_PIN = Some(fet_u_low_pin);
        FET_V_LOW_PIN = Some(fet_v_low_pin);
        FET_W_LOW_PIN = Some(fet_w_low_pin);
    }

    set_fet_stop_pattern();

    enable_pwm_fet_u_high();
    enable_pwm_fet_v_high();
    enable_pwm_fet_w_high();
}

pub fn enable_pwm_fet_u_high(){
    unsafe {
        FET_U_HIGH_PIN.as_mut().unwrap().enable();
    }
}

pub fn enable_pwm_fet_v_high(){
    unsafe {
        FET_V_HIGH_PIN.as_mut().unwrap().enable();
    }
}

pub fn enable_pwm_fet_w_high(){
    unsafe {
        FET_W_HIGH_PIN.as_mut().unwrap().enable();
    }
}

pub fn save_pwm_duty(pwn_duty:u8){
//pub fn save_pwm_duty(pwn_duty:u16){
    unsafe {
        PWM_DUTY = pwn_duty;
    }
}

pub fn load_pwm_duty() -> u8 {
//pub fn load_pwm_duty() -> u16 {
    unsafe {
        PWM_DUTY
    }
}

pub fn set_drive_state(state:DriveState){
    unsafe {
        DRIVE_STATE = state;
    }
}

pub fn get_drive_state() -> DriveState {
    unsafe {
        DRIVE_STATE
    }
}

pub fn set_speed_control_timing(flg:bool){
    unsafe {
        SPEED_CONTROL_TIMING = flg;
    }
}

pub fn get_speed_control_timing() -> bool {
    unsafe {
        SPEED_CONTROL_TIMING
    }
}

fn get_fet_drive_pattern(hall_sensor_potion:u8) -> (u8, u8, u8, bool, bool, bool) {
//fn get_fet_drive_pattern(hall_sensor_potion:u8) -> (u16, u16, u16, bool, bool, bool) {
    match hall_sensor_potion {
        HALL_SENSOR_POSITION_5 => (load_pwm_duty(), 0, 0, false, true, false),
        HALL_SENSOR_POSITION_1 => (load_pwm_duty(), 0, 0, false, false, true),
        HALL_SENSOR_POSITION_3 => (0, load_pwm_duty(), 0, false, false, true),
        HALL_SENSOR_POSITION_2 => (0, load_pwm_duty(), 0, true, false, false),
        HALL_SENSOR_POSITION_6 => (0, 0, load_pwm_duty(), true, false, false),
        HALL_SENSOR_POSITION_4 => (0, 0, load_pwm_duty(), false, true, false),
        _ => (0, 0, 0, false, false, false),
    }
}

fn drive_fet(uvw_phase_values: (u8, u8, u8, bool, bool, bool)){
//fn drive_fet(uvw_phase_values: (u16, u16, u16, bool, bool, bool)){
    let (u_high, v_high, w_high, u_low, v_low, w_low) = uvw_phase_values;
    unsafe {
        FET_U_HIGH_PIN.as_mut().unwrap().set_duty(u_high);
        FET_V_HIGH_PIN.as_mut().unwrap().set_duty(v_high);
        FET_W_HIGH_PIN.as_mut().unwrap().set_duty(w_high);
        if u_low == true { FET_U_LOW_PIN.as_mut().unwrap().set_high().void_unwrap(); }
        else {FET_U_LOW_PIN.as_mut().unwrap().set_low().void_unwrap();}

        if v_low == true { FET_V_LOW_PIN.as_mut().unwrap().set_high().void_unwrap(); }
        else {FET_V_LOW_PIN.as_mut().unwrap().set_low().void_unwrap();}

        if w_low == true { FET_W_LOW_PIN.as_mut().unwrap().set_high().void_unwrap(); }
        else {FET_W_LOW_PIN.as_mut().unwrap().set_low().void_unwrap();}
    }
}

pub fn set_fet_drive_pattern(){
    // ホールセンサの位置を取得する
    let _hall_sensor_position = hall_sensor::get_position();
    // ホールセンサの位置からFET各通電パターンを取得しFETを通電する
    drive_fet(get_fet_drive_pattern(_hall_sensor_position));
}

pub fn set_fet_stop_pattern(){
    drive_fet((0, 0, 0, false, false, false));
}
