//! Bare-metal Rust blink for the Teensy 4.1 (i.MX RT1062) -- the Rust strawman.
//!
//! Toggles the onboard LED (pin 13) rapidly (~0.2 s period) -- deliberately
//! distinct from the 1 s factory blink and the Ada strawman's ~0.5 s. The
//! `teensy4-bsp` `rt` feature supplies the runtime, the i.MX RT boot image
//! (FlexSPI config block / IVT), and the linker script, so this file is just
//! the application -- the Rust counterpart of the Ada ZFP blink.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::board;

/// Entry point: configure pin 13 as an output, then toggle it forever with a
/// calibrated busy-wait between toggles. Never returns (`-> !`).
#[bsp::rt::entry]
fn main() -> ! {
    // Teensy 4.1 board resources (use `t41`; `t40` is the Teensy 4.0).
    let board::Resources {
        pins, mut gpio2, ..
    } = board::t41(board::instances());

    let led = board::led(&mut gpio2, pins.p13);

    loop {
        led.toggle();
        // cortex_m::asm::delay counts core cycles; ARM_FREQUENCY/10 ~= 0.1 s
        // toggle -> ~0.2 s period (rapid, to differ from factory and Ada).
        cortex_m::asm::delay(board::ARM_FREQUENCY / 10);
    }
}
