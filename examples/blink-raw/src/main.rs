// Use only `core`, not `std`, because there is no OS
#![no_std]

// We define how to start the program ourselves
#![no_main]

// Things that are not yet stable but that we need to write this program
#![feature(abi_msp430_interrupt)]
#![feature(global_asm)]
#![feature(llvm_asm)]

// Our runtime is in this module
mod rt;

// Import our microcontroller and its peripherals
use msp430_periph::devices::msp430fr5969::MSP430FR5969;
use msp430_periph::peripherals::{pmm_4 as pmm, portb_3i1 as p1, portb_3i2 as p4, watchdog_timer_2 as wdt};
use msp430_periph::utils::Value;

// This prevents `main` from being optimised out
#[no_mangle]
// `link_section` (along with the linker script) tells the linker to put this at the beginning of the ROM
#[link_section = ".start"]
// The main function. It will be called from assembly, so it needs to be `extern`. It also never returns.
extern "C" fn main() -> ! {
    // Get access to the microcontroller's peripherals
    let mut p: MSP430FR5969 = unsafe { core::mem::transmute(()) };

    // Disable watchdog
    p.watchdog_timer
        .wdtctl
        .write(unsafe { Value::from_raw(0x5a00) } | wdt::WDTHOLD(true));

    // Set P1.0 and P4.6 as output
    p.port_1.pout.modify(p1::POUT0(false));
    p.port_4.pout.modify(p4::POUT6(true));
    p.port_1.pdir.modify(p1::PDIR0(true));
    p.port_4.pdir.modify(p4::PDIR6(true));

    // Enable I/Os
    p.pmm.pm5ctl0.modify(pmm::LOCKLPM5(false));

    loop {
        // Do nothing for a while, to avoid blinking too fast (not using `for` because it's really slow in debug mode)
        let mut i = 0;
        while i < 10_000u16 {
            i += 1;
            // This prevents the loop from being optimised
            unsafe { llvm_asm!("") };
        }

        // Toggle outputs
        p.port_1.pout.toggle(p1::POUT::POUT0);
        p.port_4.pout.toggle(p4::POUT::POUT6);
    }
}
