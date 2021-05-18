#![no_std]
#![no_main]
#![feature(abi_msp430_interrupt)]
#![feature(global_asm)]
#![feature(llvm_asm)]

mod rt;

use msp430_periph::devices::msp430fr5969::MSP430FR5969;
use msp430_periph::peripherals::{pmm_4::*, port_1_2_7::*, port_3_4_7::*, watchdog_timer_2::*};
use msp430_periph::utils::Value;

#[no_mangle]
#[link_section = ".start"]
extern "C" fn main() -> ! {
    let mut p: MSP430FR5969 = unsafe { core::mem::transmute(()) };

    // Disable watchdog
    p.watchdog_timer
        .wdtctl
        .write(unsafe { Value::from_raw(0x5a00) } | WDTHOLD(true));

    // Set P1.0 and P4.6 as output
    p.port_1_2.p1out.modify(P1OUT0(false));
    p.port_3_4.p4out.modify(P4OUT6(true));
    p.port_1_2.p1dir.modify(P1DIR0(true));
    p.port_3_4.p4dir.modify(P4DIR6(true));

    // Enable I/Os
    p.pmm.pm5ctl0.modify(LOCKLPM5(false));

    loop {
        let mut i = 0;
        loop {
            unsafe {
                llvm_asm!("");
            }
            i += 1;
            if i == 10_000u16 {
                break;
            }
        }
        // toggle outputs
        p.port_1_2.p1out.toggle(P1OUT::P1OUT0);
        p.port_3_4.p4out.toggle(P4OUT::P4OUT6);
    }
}
