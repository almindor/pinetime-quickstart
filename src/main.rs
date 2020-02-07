#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::entry;
use nrf52832_hal::prelude::*;
use cortex_m_semihosting::hprintln;

#[entry]
unsafe fn main() -> ! {
    hprintln!("Start of main").unwrap();
    loop {
        continue;
    }
}
