#![no_std]
#![no_main]

use rp2040_hal as hal;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    // Watchdog + clocks
    let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000,  // external crystal (12 MHz)
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let sio = hal::Sio::new(pac.SIO);

    // Pins
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Onboard LED
    let mut led = pins.gpio25.into_push_pull_output();

    loop {
        led.set_high().unwrap();
        for _ in 0..500_000 { cortex_m::asm::nop(); }
        led.set_low().unwrap();
        for _ in 0..500_000 { cortex_m::asm::nop(); }
    }
}
