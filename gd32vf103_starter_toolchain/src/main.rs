#![no_main]
#![no_std]
#![allow(unused)]

extern crate panic_halt;
use riscv_rt::entry;
use crate::driver::gpio;
mod driver;

#[entry]
fn main() -> ! {
    
    let freq_hz = 1;
    let timer_clock = 8_000_000 as u32;
    let ticks = timer_clock / freq_hz;
    let psc = ((ticks - 1) / (1 << 16)) as u16;
    loop{
        // to be added later
    }
}