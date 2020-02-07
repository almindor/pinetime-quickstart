#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::entry;
use nrf52832_hal::prelude::*;

#[entry]
unsafe fn main() -> ! {    
    loop {
        // Delay pause;

        // pause.delay_ms(1000u16);
    }
}
