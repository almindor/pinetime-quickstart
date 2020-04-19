#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use nrf52832_hal::pac::{CorePeripherals, Peripherals};
use nrf52832_hal::prelude::*;
use nrf52832_hal::uicr::Uicr;
use nrf52832_hal::Delay;
use cortex_m_semihosting::hprintln;
use rt::entry;

const VAL: u32 = 0b0000_1111_0000_1111_0000_1111_0000_1111;
const VAL2: u32 = 0u32;
const VAL3: u32 = 0b1111_0000_1111_0000_1111_0000_1111_0000;

#[entry]
unsafe fn main() -> ! {
    let core = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST);
    let mut uicr = Uicr::new(p.UICR);
    let mut nvmc = p.NVMC;
    let mut buffer = [42u32];

    uicr.erase(&mut nvmc);

    uicr.load_customer(0, &mut buffer);
    hprintln!("Original value: {}", buffer[0]);

    uicr.store_customer(&mut nvmc, 0, &[VAL]);

    uicr.load_customer(0, &mut buffer);
    hprintln!("Store1 value: {}", buffer[0]);

    uicr.store_customer(&mut nvmc, 0, &[VAL2]);

    uicr.load_customer(0, &mut buffer);
    hprintln!("Store2 value: {}", buffer[0]);


    uicr.store_customer(&mut nvmc, 0, &[VAL3]);

    uicr.load_customer(0, &mut buffer);
    hprintln!("Store3 value: {}", buffer[0]);

    // a loop is still required also because systemoff takes some time, we don't
    // want to cause UB
    loop {
        delay.delay_ms(1000u16);
    }
}
