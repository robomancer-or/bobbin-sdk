#![no_std]
#![feature(asm, lang_items, global_allocator)]
#![feature(naked_functions)]

extern crate r0;

extern crate log;

extern crate s32;
pub use s32::{chip, hal, cortexm, common};

#[macro_use] pub mod itm;
#[macro_use] pub mod console;
#[macro_use] pub mod logger;

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

pub mod pin;
pub mod clock;
pub mod led;
pub mod btn;
pub mod pot;
pub mod tim;
//pub mod serial;
//pub mod can;
//pub mod spi;
pub mod uja1169;

pub use common::heap::Heap;

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

pub unsafe fn init_allocator(buf: &'static mut [u8]) {
    ALLOCATOR.init(buf);
}

pub use tim::delay;

// pub fn delay(n: u32) {
//     for _ in 0..10_000*n {
//         unsafe { asm!("nop") }
//     }
// }

pub fn init() {
    clock::init();
    led::init();
    btn::init();
    pot::init();
    tim::init();
    console::init();
}