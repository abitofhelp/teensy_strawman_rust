# teensy_strawman_rust

A bare-metal **Rust** project that establishes a working build, boot, and runtime
configuration for the **Teensy 4.1** development board (NXP i.MX RT1062,
ARM Cortex-M7). There are two crates: 1) A minimal LED blinking application; and 2) A LED blinking application based on **RTIC `async` tasking**. These crates prove the toolchain, boot image, and flash chain end-to-end on real hardware.

> This is the Rust counterpart of the Ada strawman (`teensy_strawman` /
> `teensy_strawman_ada`). The two mirror each other to compare the toolchains on
> identical hardware.

## Status

| Stage | State |
|-------|-------|
| Simple LED blink (`blink/src/main.rs`) | ✅ **Proven on hardware** |
| RTIC `async` tasking blink (`tasking/src/main.rs`) | ✅ **Proven on hardware** — an async task on a SysTick monotonic timer |

Both binaries blink at a rapid **~0.2 s** period, deliberately distinct from the
**1 s** factory app and the Ada strawman's **~0.5 s**, so the three are easy to
tell apart at a glance.

## Hardware

- **Board:** PJRC Teensy 4.1
- **MCU:** NXP i.MX RT1062 — ARM Cortex-M7, double-precision FPU
- **Boot flash:** Winbond W25Q64JV-DTR (8 MB QSPI NOR), loaded via FlexSPI
- **LED:** onboard, Arduino pin 13 → pad `GPIO_B0_03` → `GPIO2` bit 3

## Requirements

- [rustup](https://rustup.rs) with the bare-metal target:
  `rustup target add thumbv7em-none-eabihf`
- [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli)
  (`brew install teensy_loader_cli` on macOS) — for flashing
- An `objcopy` for ELF → Intel HEX. The `Makefile` prefers the Rust-native
  `rust-objcopy` (`rustup component add llvm-tools && cargo install cargo-binutils`)
  and falls back to a GNU `arm-eabi-objcopy` if one is on `PATH`.

## Build & flash

The `Makefile` wraps the common steps (mirrors the Ada strawman):

```sh
make                # build the simple blink (default)
make burn-blink     # build + HEX + flash the simple blink
make burn-tasking   # build + HEX + flash the RTIC tasking blink
make size / make clean / make help
```

Or run the steps directly:

```sh
# Build a workspace member (release is the embedded default).
cargo build --release -p blink        # simple blink   -> .../release/blink
cargo build --release -p tasking      # RTIC tasking   -> .../release/tasking

# ELF -> HEX (rust-objcopy; a GNU arm-eabi-objcopy also works).
rust-objcopy -O ihex \
  target/thumbv7em-none-eabihf/release/tasking bin/tasking.hex

# Flash, then press the Teensy's onboard button when prompted.
teensy_loader_cli --mcu=TEENSY41 -w -v bin/tasking.hex
```

## Project layout

```
teensy_strawman_rust/            Cargo workspace root
├── Cargo.toml                   [workspace] members + shared deps + release profile
├── .cargo/config.toml           target (thumbv7em-none-eabihf) + linker arg (t4link.x)
├── Makefile                     build / hex / burn / size / clean / help
├── blink/
│   ├── Cargo.toml               package "blink"
│   └── src/main.rs              simple loop blink
└── tasking/
    ├── Cargo.toml               package "tasking"
    └── src/main.rs              RTIC async tasking blink (SysTick monotonic)
```

## How it works (briefly)

The `teensy4-bsp` crate's `rt` feature pulls in `imxrt-rt`, which generates the
i.MX RT **boot image** (FlexSPI Configuration Block + Image Vector Table), the
**memory map**, and the **linker script** (`t4link.x`). At startup the runtime
copies code into **ITCM** and data into **DTCM**, and places the **vector table
in DTCM** (RAM) — so the application source is just the blink itself.

That ITCM-code / DTCM-vectors layout is the standard, hardware-proven model for
this chip and is a useful reference point when bringing up other runtimes on the
same board.

## Debugging

The Teensy has **no on-board debugger and no SWD header** — the i.MX RT's SWD pins
are small underside test pads. For interactive `gdb`, wire a **Raspberry Pi Debug
Probe** (or any CMSIS-DAP / J-Link probe) to those pads and bridge it with
`openocd`; the same probe also exposes a **UART** for a serial console. For quick
logging without a probe, a 3.3 V USB-to-serial adapter on `Serial1` (pin 1 = TX)
works too.

## Roadmap

1. ✅ Simple blink — de-risk the toolchain / boot / flash chain (done).
2. ✅ RTIC `async` tasking blink — a scheduled task on a SysTick monotonic (done).
3. Graft the hybrid (DDD/Clean/Hex) architecture to seed `hybrid_teensy_app_rust`.

## Attribution

Built on the **teensy4-rs** and **imxrt-rs** crate ecosystems and the **RTIC**
framework. See **[NOTICE](NOTICE)** for the components and their licenses.
NXP/ARM reference documents are not redistributed.

## License

BSD 3-Clause — see **[LICENSE](LICENSE)**. Third-party crates retain their own
(permissive) licenses as described in **[NOTICE](NOTICE)**.

## AI Assistance & Authorship

This project was developed by **Michael Gardner (A Bit of Help, Inc.)** with the
assistance of AI coding tools (Anthropic's Claude, via Claude Code). All
engineering decisions, reviews, and the final implementation are the author's
responsibility; AI was used as a tool for research, drafting, and verification.
Hardware facts were cross-checked against vendor documentation, and the result
was validated on physical hardware.
