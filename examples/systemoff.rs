#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use nrf52832_hal::nrf52832_pac::{CorePeripherals, Peripherals, power};
use nrf52832_hal::prelude::*;
use nrf52832_hal::Delay;
use rt::entry;

#[entry]
unsafe fn main() -> ! {
    let core = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST);

    // set systemoff bit in the CSR for a shutdown request
    p.POWER.systemoff.write(|w| w.systemoff().set_bit());

    // a loop is still required also because systemoff takes some time, we don't
    // want to cause UB
    loop {
        delay.delay_ms(1000u16);
    }
}
