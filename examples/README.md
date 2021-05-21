# `msp430-periph` examples

This directory contains example programs using the `msp430-periph` crate. There are written for a
MSP430FR5969 LaunchPad, which has LEDs on pins P1.0 and P4.6, as well as push buttons on pins P1.1
and P4.5.

- `blink-raw`: Blink alternatively both LEDs, along with a minimal runtime
- `blink-rt`: Blink alternatively both LEDs, using the `msp430-rt` crate
- `blink-interrupt`: Blink alternatively both LEDs, using a timer and an interrupt

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
