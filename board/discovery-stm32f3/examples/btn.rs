#![no_std]
#![no_main]
#![feature(asm)]

extern crate discovery_stm32f3 as board;
extern crate examples;

use board::prelude::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();
    brd.run(|brd| {
        let btn = brd.btn0();
        let led = brd.led0();
    
        let app = examples::btn::BtnExample::new(btn, led, brd.tick(), 500, 100);
        app.run()
    })
}
