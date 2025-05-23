#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use hal::{gpio::Level, pac};
use nrf52833_hal as hal;
use panic_halt as _;

// row1 po.21, col1 po.28 from high to low
#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let _col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);
    let mut is_on: bool = false;
    loop {
        let _ = row1.set_state(PinState::from(is_on));
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
    }
}
