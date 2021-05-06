//! # msp430-periph
//!
//! Peripheral definition for all MSP430s
//!
//! # Usage
//!
//! Since this crate includes 621 devices and 481 peripherals, everything is feature-gated.
//! That means you have to enable every microcontroller and peripheral you want to use, for example:
//!
//! ```
//! [dependencies.msp430-periph]
//! version = "0.0.1"
//! features = [
//!     # your microcontroller
//!     "msp430fr5969",
//!     # every peripheral you need
//!     "watchdog_timer_2",
//!     "pmm__power_management_system_4",
//!     "port_1_2_7",
//!     "port_3_4_7",
//! ]
//! ```
//!
//! There is also an `-all` version of microcontroller features to enable all peripherals from this microcontroller:
//!
//! ```
//! [dependencies]
//! msp430-periph = { version = "0.0.1", features = [ "msp430fr5969-all" ] }
//! ```
//!
//! # Documentation
//!
//! It is not reasonable to build documentation for the whole crate with all features enabled. You can either look at the
//! source code or build documentation for the features you use by running `cargo doc` in your project directory.
//!

#![allow(bad_style)]
#![no_std]

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
