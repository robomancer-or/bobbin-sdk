#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    println!("Running");
    loop {}
}