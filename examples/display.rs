#![no_std]
#![no_main]

extern crate cortex_m_rt as rt; // v0.5.x

extern crate nrf52832_hal;
extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use nrf52832_hal::gpio::Level;
use nrf52832_hal::gpio::*;
use nrf52832_hal::spim;
use nrf52832_hal::Delay;
use st7735_lcd::ST7735;

#[entry]
fn main() -> ! {
    let core = nrf52832_hal::nrf52832_pac::CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST);

    let p = nrf52832_hal::nrf52832_pac::Peripherals::take().unwrap();
    let port0 = p.P0.split();

    let _backlight = port0.p0_22.into_push_pull_output(Level::Low); // set medium backlight on
    let rst = port0.p0_26.into_push_pull_output(Level::Low); // reset pin
    let _cs = port0.p0_25.into_push_pull_output(Level::Low); // keep low while drivign display
    let cd = port0.p0_18.into_push_pull_output(Level::Low); // clock/data switch

    let spiclk = port0.p0_02.into_push_pull_output(Level::Low).degrade();
    let spimosi = port0.p0_03.into_push_pull_output(Level::Low).degrade();

    let pins = spim::Pins {
        sck: spiclk,
        miso: None,
        mosi: Some(spimosi),
    };

    let spi = spim::Spim::new(p.SPIM0, pins, spim::Frequency::M8, spim::MODE_3, 122);

    let mut display = ST7735::new(spi, cd, rst, true, false);
    let rgb = (128, 128, 128);

    display.init(&mut delay).unwrap();

    display.draw(Circle::<Rgb565>::new(Coord::new(64, 64), 64).stroke(Some(rgb.into())));

    loop {}
}
