use crate::motor_control;
use crate::led;

const PRESCALER: u32 = 1024;    // タイマカウント周波数 = 16(MHz) / 1024(分周) 
const TIMER_COUNTS: u32 = 156;  // 10msタイマカウント値 = 10(ms) / (1 / タイマカウント周波数)
const COUNT_1SEC: u8 = 100;     // 10msタイマで1秒のカウント値 = 1000ms / 10ms
static mut COUNTER_1SEC:u8 = 0; // 1秒カウンタ


#[avr_device::interrupt(atmega2560)]
fn TIMER0_COMPA() {
    unsafe {
        motor_control::set_speed_control_timing(true);

        COUNTER_1SEC += 1;
        if COUNTER_1SEC >= COUNT_1SEC {
            COUNTER_1SEC = 0;
            led::toggle_user();
        }
    }
}


pub fn init(tc0: arduino_mega2560::pac::TC0) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(TIMER_COUNTS as u8) });
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());
}
