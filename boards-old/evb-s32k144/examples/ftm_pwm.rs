#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::led::*;

use board::common::Channel;
use board::hal::port::*;
use board::hal::ftm::*;
use board::hal::pcc;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    let led0 = LED0;

    println!("PWM Test");
    
    let ch = FTM0_CH2;
    let t0 = ch.periph();

    led0.port_pin().mode_ftm(&ch);

    t0
        .pcc_set_clock_source(pcc::ClockSource::SPLLDIV2)
        .set_prescale(64);


    // LED is active low, use pwm_low

    ch.pwm_low(0, 2048);

    println!("PWM Enabled, Pauses at Zero");

        
    let max = 2000u16;
    let step = 20u16;
    let mut i: u16 = step; 
    let mut dir: bool = true;
    loop {       
        ch.set_compare(i);
        
        if i == max { dir = false } else if i == 0 { dir = true; board::delay(1000); }
        if dir {
            i += step 
        } else {
            i -= step;
        }
        board::delay(10);
    }
}