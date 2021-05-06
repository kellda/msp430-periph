# `msp430gen`

DSLite is a [Texas Instruments-internal](https://e2e.ti.com/support/tools/ccs/f/81/p/520698/1895346#1895346)
file format for describing their microcontroller address layout and peripherals.

`msp430gen` can generate a crate using macros from the [`peripherals`](crate.io/crates/peripherals) crate to
access the peripherals of msp430 microcontrollers.

This is based on [`msp430_svd`](https://github.com/pftbest/msp430_svd/).

## Notes

This repository supplies a copy of DSLite files for the msp430 family. All commands are run from the root directory of this repository.

Currently reset values for all registers are set to 0, because I don't have a good way to get them yet.
