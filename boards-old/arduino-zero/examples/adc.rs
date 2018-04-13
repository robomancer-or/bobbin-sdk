#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

use board::common::bits::*;
use board::pin;
use board::hal::port::Ain;
use board::hal::adc::{self, AnalogRead};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Analog Test");
    adc::init();
    let a0 = pin::A0;
    let a1 = pin::A1;

    let ch0 = adc::ADC_CH0;
    let ch2 = adc::ADC_CH2;

    a0.mode_ain(&ch0);
    a1.mode_ain(&ch2);

    loop {
        let v0: U12 = ch0.analog_read();
        let v1: U8 = ch2.analog_read();
        println!("{}, {}", v0, v1);
        board::delay(500);
    }

}