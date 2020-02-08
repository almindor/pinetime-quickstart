#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::entry;
use nrf52832_hal::prelude::*;
use nrf52832_hal::Delay;
use nrf52832_hal::nrf52832_pac::CorePeripherals;
use cortex_m_semihosting::{hprint, hprintln};

#[entry]
unsafe fn main() -> ! {
    let core = CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST);

    hprintln!("Start of main").unwrap();

    loop {
        delay.delay_ms(1000u16);
        hprint!(".").unwrap();
    }
}
