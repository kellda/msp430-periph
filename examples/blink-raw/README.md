# `bilnk-raw`

> Blink alternatively both LEDs, along with a minimal runtime

This is an example program using the `msp430-periph` crate. It is written for a MSP430FR5969 LaunchPad, which has LEDs
on pins P1.0 and P4.6, as well as push buttons on pins P1.1 and P4.5.

This example embeeds a minimal runtime and depends only on the `msp430-periph` crate. Please note that it dones't
initialize `static`s.

## Files

- `Cargo.toml`: description of the crate with tweaked optimisations
- `.cargo/config`: how to compile this crate
- `link.x`: linker script (how to make an executable from compiled code)
- `src/rt.rs`: the runtime (initialisation of the microcontroller to run the `main` function)
- `src/main.rs`: the main program that binks LEDs.
- `mspdebug.gdb`: a script for GDB to load the program to the target
