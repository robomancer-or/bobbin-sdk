#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f302r8 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for nucleo-f302r8");
    for i in 0..5 {
        println!("[pass] {}", i);
        board::delay(100);
    }
    println!("[done] All tests passed");
    loop {}
}