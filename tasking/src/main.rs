//! Teensy 4.1 tasking blink in Rust -- the counterpart of the Ada Jorvik task.
//!
//! An RTIC `async` software task toggles the onboard LED every 100 ms (~0.2 s
//! period -- rapid, distinct from the 1 s factory blink and the Ada ~0.5 s),
//! using a SysTick monotonic timer (`Systick::delay(...).await`). RTIC + the BSP is
//! the standard, hardware-proven tasking setup for this board, and serves as a
//! reference for how the M7 SysTick / scheduler should be configured.

#![no_std]
#![no_main]

use teensy4_panic as _;

// `dispatchers` lists interrupts RTIC borrows to run software (`async`) tasks.
// KPP (the keypad port) is unused on this board -- pick any IRQ you are NOT
// using; do not later enable a peripheral that needs KPP without changing this.
#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::board;
    use teensy4_bsp as bsp;

    use rtic_monotonics::systick::{Systick, *};

    /// Resources shared across tasks -- none in this single-task demo.
    #[shared]
    struct Shared {}

    /// Resources owned by a single task.
    #[local]
    struct Local {
        /// The onboard LED (pin 13).
        led: board::Led,
    }

    /// Boot-time setup: bring up the Teensy 4.1 board, start the SysTick
    /// monotonic timer, and spawn the periodic `blink` task.
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut gpio2, pins, ..
        } = board::t41(cx.device);

        let led = board::led(&mut gpio2, pins.p13);

        // SysTick monotonic at the configured ARM clock -- the time base for
        // `delay`, exactly the piece the Ada runtime needs to get right.
        Systick::start(
            cx.core.SYST,
            board::ARM_FREQUENCY,
            rtic_monotonics::create_systick_token!(),
        );

        blink::spawn().unwrap();
        (Shared {}, Local { led })
    }

    /// Periodic task: toggle the LED, then `await` the SysTick delay so the
    /// scheduler can run other work (or idle) in between.
    #[task(local = [led])]
    async fn blink(cx: blink::Context) {
        loop {
            cx.local.led.toggle();
            Systick::delay(100.millis()).await; //  ~0.2 s period (rapid)
        }
    }
}
