#![no_std]
#![no_main]

use rp2040_hal as hal;
use hal::pac;
use hal::Sio;
use hal::gpio::Pin;
use hal::gpio::bank0::Gpio15;
use hal::gpio::{PullDownInput, Output, PushPull};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    info!("Booting up...");

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Clocks
    let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Onboard LED
    let mut led = pins.gpio25.into_push_pull_output();

    // PIR sensor on GP15
    let pir = pins.gpio15.into_pull_down_input();

    info!("Init complete. Waiting for motion...");

    loop {
        if pir.is_high().unwrap() {
            info!("Motion detected!");
            led.set_high().ok();
        } else {
            led.set_low().ok();
        }
    }
}
