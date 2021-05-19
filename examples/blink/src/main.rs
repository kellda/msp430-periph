#![no_std]
#![no_main]
#![feature(abi_msp430_interrupt)]
#![feature(global_asm)]
#![feature(llvm_asm)]

mod rt;

use msp430_periph::devices::msp430fr5969::MSP430FR5969;
use msp430_periph::peripherals::{pmm_4 as pmm, portb_3i1 as p1, portb_3i2 as p4, watchdog_timer_2 as wdt};
use msp430_periph::utils::Value;

#[no_mangle]
#[link_section = ".start"]
extern "C" fn main() -> ! {
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
        p.port_1.pout.toggle(p1::POUT::POUT0);
        p.port_4.pout.toggle(p4::POUT::POUT6);
    }
}
