# `msp430-periph`

Peripheral definition for all MSP430s

# Usage

Add this crate to your dependencies:

```
[dependencies.msp430-periph]
version = "0.0.1"
```

Then add the features you need, e.g.

```
features = [
    # your microcontroller
    "msp430fr5969",
    # every peripheral you need
    "watchdog_timer_2",
    "pmm__power_management_system_4",
    "port_1_2_7",
    "port_3_4_7",
]
```

or append `-all` to the microcontroller feature to enable all peripherals from this microcontroller

```
features = [ "msp430fr5969-all" ]
```

# Note

This is currently the raw output converted from DSLite files. The hope is that errors from DSLite files
can be fixed directly in the generated code, and that peripherals similar enought can be merged.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
