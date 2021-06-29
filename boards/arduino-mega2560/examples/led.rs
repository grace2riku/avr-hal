use arduino_mega2560::prelude::*;
use arduino_mega2560::hal::port;

// mega2560 user LED  d13, PB7
static mut USER_LED_PIN: Option<port::portb::PB7<port::mode::Output>> = None;
// hall sensor-U led d23, PA1
static mut HALL_U_LED_PIN: Option<port::porta::PA1<port::mode::Output>> = None;
// hall sensor-V led d25, PA3
static mut HALL_V_LED_PIN: Option<port::porta::PA3<port::mode::Output>> = None;
// hall sensor-W led d27, PA5
static mut HALL_W_LED_PIN: Option<port::porta::PA5<port::mode::Output>> = None;


pub fn init(user_led:port::portb::PB7<port::mode::Output>,
            hall_u_led:port::porta::PA1<port::mode::Output>,
            hall_v_led:port::porta::PA3<port::mode::Output>,
            hall_w_led:port::porta::PA5<port::mode::Output>){
    unsafe {
        USER_LED_PIN = Some(user_led);
        HALL_U_LED_PIN = Some(hall_u_led);
        HALL_V_LED_PIN = Some(hall_v_led);
        HALL_W_LED_PIN = Some(hall_w_led);
    }
}


pub fn set_hall_u(level:bool){
    unsafe {
        if level == true { HALL_U_LED_PIN.as_mut().unwrap().set_high().void_unwrap(); }
        else {HALL_U_LED_PIN.as_mut().unwrap().set_low().void_unwrap();}
    } 
}

pub fn set_hall_v(level:bool){
    unsafe {
        if level == true { HALL_V_LED_PIN.as_mut().unwrap().set_high().void_unwrap(); }
        else {HALL_V_LED_PIN.as_mut().unwrap().set_low().void_unwrap();}
    } 
}

pub fn set_hall_w(level:bool){
    unsafe {
        if level == true { HALL_W_LED_PIN.as_mut().unwrap().set_high().void_unwrap(); }
        else {HALL_W_LED_PIN.as_mut().unwrap().set_low().void_unwrap();}
    } 
}

pub fn toggle_user(){
    unsafe {
        USER_LED_PIN.as_mut().unwrap().toggle().void_unwrap();
    }
}

