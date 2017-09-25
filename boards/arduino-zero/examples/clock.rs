#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

use board::hal::clock::Clock;
use board::hal::sercom::SERCOM5;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let clk = board::clock::CLK;
    println!("Clock Test");
    println!("{:?}", clk);
    // println!("XOSC:        {:?}", clk.xosc());
    // println!("XOSC32K:     {:?}", clk.xosc32k());
    // println!("OSC32K:      {:?}", clk.osc32k());
    // println!("OSCULP32K:   {:?}", clk.osculp32k());
    // println!("OSC8M:       {:?} EN: {} ONDEMAND: {} RDY: {} PRES: {:?}", clk.osc8m(), clk.osc8m_enabled(), clk.osc8m_ondemand(), clk.osc8m_rdy(), clk.osc8m_pre());
    // println!("DFLL:        {:?} EN: {} MUL: {}", clk.dfll(), clk.dfll_enabled(), clk.dfll_mul());
    // println!("DPLL:        {:?} EN: {} ONDEMAND: {} RDY: {} MUL: {} DIV: {} REF: {:?}", 
    //     clk.dpll(), clk.dpll_enabled(), clk.dpll_ondemand(), clk.dpll_rdy(), clk.dpll_mul(), clk.dpll_div(), clk.dpll_refclk());

    // println!("Disable OSC8M ONDEMAND");
    // clk.set_osc8m_ondemand(false);    
    // board::delay(10);
    // println!("OSC8M:       {:?} EN: {} ONDEMAND: {} RDY: {} PRES: {:?}", clk.osc8m(), clk.osc8m_enabled(), clk.osc8m_ondemand(), clk.osc8m_rdy(), clk.osc8m_pre());
    // clk
    //     .set_dpll_refclk(clock::DpllRefClock::Xosc32k)
    //     .set_dpll_mul(1499)
    //     .set_dpll_div(0)
    //     .set_dpll_ondemand(false);

    // println!("Enabling DPLL");
    // clk
    //     .set_dpll_enabled(true);
    // while !clk.dpll_enabled() {}

    // println!("DPLL:        {:?} EN: {} ONDEMAND: {} RDY: {} MUL: {} DIV: {} REF: {:?}", 
    //     clk.dpll(), clk.dpll_enabled(), clk.dpll_ondemand(), clk.dpll_rdy(), clk.dpll_mul(), clk.dpll_div(), clk.dpll_refclk());
    // while !clk.dpll_lock() {}

    // println!("DPLL:        {:?} EN: {} ONDEMAND: {} RDY: {} MUL: {} DIV: {} REF: {:?}", 
    //     clk.dpll(), clk.dpll_enabled(), clk.dpll_ondemand(), clk.dpll_rdy(), clk.dpll_mul(), clk.dpll_div(), clk.dpll_refclk());

    // println!("Generators");
    // for i in 0..8 {
    //     let ctrl = clk.generator_ctrl(i);
    //     if ctrl.genen() != 0 {
    //         println!("  {}: {:?} <= {:?} - {:?} {:?}", i, clk.generator(i), clock::Source::from(ctrl.src() as u8), clk.generator_div(i), ctrl);
    //     }
    // }

    // println!("Clock Multiplexers");
    // for i in 0..26 {
    //     let ctrl = clk.clockmux_ctrl(i);
    //     if ctrl.clken() != 0 {
    //         let clk_id = clock::ClockMux::from(i);
    //         println!("  {:?}: {:?} {:?}", clk_id, clk.clockmux(clk_id), ctrl);
    //     }
    // }    

    println!("SERCOM5 Clock: {:?}", SERCOM5.clock(&clk));

        
    println!("DONE");
    loop {}
}