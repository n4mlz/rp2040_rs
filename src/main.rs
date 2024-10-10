//! XIAO RP2040 Blinking LED Example
//!
//! https://tutoduino.fr/
//!
//! Blinks the LED on a Seeed Studio XIAO RP2040 board.
//!
#![no_std]
#![no_main]
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use seeeduino_xiao_rp2040::entry;
use seeeduino_xiao_rp2040::hal;
use seeeduino_xiao_rp2040::hal::pac;
use seeeduino_xiao_rp2040::hal::prelude::*;
/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        seeeduino_xiao_rp2040::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);
    // Set the pins up according to their function on this particular board
    let pins = seeeduino_xiao_rp2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    // Configure the pins to operate as a push-pull output
    let mut led_blue_pin = pins.led_blue.into_push_pull_output();
    let mut led_green_pin = pins.led_green.into_push_pull_output();
    let mut led_red_pin = pins.led_red.into_push_pull_output();
    // Set green and red led OFF
    led_green_pin.set_high().unwrap();
    led_red_pin.set_high().unwrap();
    loop {
        // Turn on blue LED for 1s
        led_blue_pin.set_low().unwrap();
        delay.delay_ms(1000);
        // Turn off blue LED for 500ms
        led_blue_pin.set_high().unwrap();
        delay.delay_ms(500);
    }
}
