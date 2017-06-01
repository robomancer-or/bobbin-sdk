#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]

extern crate r0;
extern crate compiler_builtins;

extern crate log;

#[macro_use]
pub mod console;

#[macro_use]
pub mod logger;

extern crate stm32f20x;
pub use stm32f20x::{chip, hal};

pub mod exceptions;
pub mod lang_items;

pub mod clock;
pub mod pin;
pub mod led;
pub mod btn;
pub mod tim;

pub use tim::delay;

pub fn init() {
    clock::init();
    led::init();
    btn::init();
    tim::init();
    console::init();
}