//! # msp430-periph
//!
//! Peripheral definition for all MSP430s
//!
//! # Usage
//!
//! Since this crate includes 625 devices and 336 peripherals, everything is feature-gated.
//! That means you have to enable every microcontroller and peripheral you want to use, for example:
//!
//! ```
//! [dependencies.msp430-periph]
//! version = "0.0.3"
//! features = [
//!     # your microcontroller
//!     "msp430fr5969",
//!     # every peripheral you need
//!     "watchdog_timer_2",
//!     "pmm_4",
//!     "portb_3i1",
//!     "portb_3i2",
//! ]
//! ```
//!
//! There is also an `-all` version of microcontroller features to enable all peripherals from this microcontroller:
//!
//! ```
//! [dependencies]
//! msp430-periph = { version = "0.0.3", features = [ "msp430fr5969-all" ] }
//! ```
//!
//! To use with the `msp430-rt` runtime, also enable the `rt` feature. No `memory.x` files are needed.
//!
//! # Documentation
//!
//! It is not reasonable to build documentation for the whole crate with all features enabled. You can either look at the
//! source code or build documentation for the features you use by running `cargo doc --open` in your project directory.
//! To locally build documentation only for this crate, run `cargo doc -p msp430-periph --no-deps --open`

#![no_std]
#![allow(bad_style)]
#![recursion_limit="512"]
#![cfg_attr(feature = "rt", feature(abi_msp430_interrupt))]

pub use utils;

/// All devices
///
/// Each devices needs to be enabled by its feature flag.
/// The feature flag with `-all` appended alse enables all related peripherals
///
/// There is also a `device-all` flag, that you likely shouldn't use
pub mod devices;

/// All peripherals
///
/// Each peripherals needs to be enabled by its feature flag.
/// Peripherals are also enabled by the _`devices`_`-all` feature flag.
///
/// There is also a `periph-all` flag, that you likely shouldn't use
pub mod peripherals;

#[cfg(feature = "rt")]
include!(concat!(env!("OUT_DIR"), "/use_interrupt.rs"));

#[cfg(feature = "rt")]
union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
