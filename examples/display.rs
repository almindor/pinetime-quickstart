#![no_std]
#![no_main]

extern crate cortex_m_rt as rt; // v0.5.x

extern crate nrf52832_hal;
extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use nrf52832_hal::gpio::Level;
use nrf52832_hal::gpio::*;
use nrf52832_hal::spim;
use nrf52832_hal::Delay;
use st7735_lcd::{ST7735, Orientation};
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let core = nrf52832_hal::nrf52832_pac::CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST);

    let p = nrf52832_hal::nrf52832_pac::Peripherals::take().unwrap();
    let port0 = p.P0.split();

    let _backlight = port0.p0_22.into_push_pull_output(Level::Low); // set medium backlight on
    let rst = port0.p0_26.into_push_pull_output(Level::Low); // reset pin
    let _cs = port0.p0_25.into_push_pull_output(Level::Low); // keep low while drivign display
    let dc = port0.p0_18.into_push_pull_output(Level::Low); // data/clock switch

    let spiclk = port0.p0_02.into_push_pull_output(Level::Low).degrade(); // SPI clock to LCD
    let spimosi = port0.p0_03.into_push_pull_output(Level::Low).degrade(); // SPI MOSI to LCD

    let pins = spim::Pins {
        sck: spiclk,
        miso: None,
        mosi: Some(spimosi),
    };

    // create SPI interface
    let spi = spim::Spim::new(p.SPIM0, pins, spim::Frequency::M8, spim::MODE_3, 122);

    // create driver
    let mut display = ST7735::new(spi, dc, rst, true, true);

    // initialize
    display.init(&mut delay).unwrap();
    // set default orientation
    display.set_orientation(&Orientation::Portrait).unwrap();

    let green = (0, 255, 0);
    let blue = (0, 0, 255);
    let red = (255, 0, 0);
    
    let blank = Rectangle::new(Coord::new(0, 0), Coord::new(239, 239)).fill(Some(blue.into()));
    let circle1 = Circle::new(Coord::new(128, 64), 64).fill(Some(red.into()));
    let circle2 = Circle::new(Coord::new(64, 64), 64).stroke(Some(green.into()));

    // draw two circles on blue background
    display.draw(blank);
    display.draw(circle1);
    display.draw(circle2);

    hprintln!("Rendering done").unwrap();

    loop {
        continue; // keep optimizer from removing in --release
    }
}
