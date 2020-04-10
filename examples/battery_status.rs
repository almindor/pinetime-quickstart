#![no_std]
#![no_main]

extern crate cortex_m_rt as rt; // v0.5.x

extern crate nrf52832_hal;
extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;
use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::InputPin;
use nrf52832_hal::gpio::*;
use nrf52832_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use nrf52832_hal::saadc::*;
use nrf52832_hal::Delay;

#[entry]
fn main() -> ! {
    let core = nrf52832_hal::nrf52832_pac::CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST);

    let p = nrf52832_hal::nrf52832_pac::Peripherals::take().unwrap();
    let mut saadc = Saadc::new(p.SAADC, SaadcConfig::default());
    let port0 = p.P0.split();

    let cip = port0.p0_12.into_floating_input().degrade(); // charge indication pin
    let mut adc = port0.p0_31.into_floating_input(); // adc voltage analog input pin

    let charging = !cip.is_high().unwrap();

    hprintln!("Charging: {}", charging).unwrap();

    let adc_val: u32 = saadc.read(&mut adc).unwrap().into(); // keep as 32bit for multiplication

    hprintln!("Raw ADC_V: {}", adc_val).unwrap();

    let battery_voltage: u32 = (adc_val * 2000) / 4965; // we multiply the ADC value by 2 * 1000 for mV and divide by (2 ^ 14 / 3.3V reference)

    hprintln!("Battery voltage: {}mV", battery_voltage).unwrap();

    loop {
        delay.delay_ms(2000u16);
        continue; // keep optimizer from removing in --release
    }
}
