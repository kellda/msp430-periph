#![no_std]
#![no_main]

use msp430::asm;
use msp430_rt::entry;
use panic_msp430 as _;

use msp430_periph::devices::msp430fr5969::MSP430FR5969;
use msp430_periph::peripherals::{pmm_4 as pmm, portb_3i1 as p1, portb_3i2 as p4, watchdog_timer_2 as wdt};
use msp430_periph::utils::Value;

#[entry]
fn main() -> ! {
    let mut p: MSP430FR5969 = unsafe { core::mem::transmute(()) };

    // Disable watchdog
    p.watchdog_timer
        .ctl
        .write(unsafe { Value::from_raw(0x5a00) } | wdt::HOLD(true));

    // Set P1.0 and P4.6 as output
    p.port_1.out.modify(p1::OUT0(false));
    p.port_4.out.modify(p4::OUT6(true));
    p.port_1.dir.modify(p1::DIR0(true));
    p.port_4.dir.modify(p4::DIR6(true));

    // Enable I/Os
    p.pmm.pm5ctl0.modify(pmm::LOCKLPM5(false));

    loop {
        let mut i = 0u16;
        while i < 10_000u16 {
            i += 1;
            asm::nop();
        }

        // Toggle outputs
        p.port_1.out.toggle(p1::OUT::OUT0);
        p.port_4.out.toggle(p4::OUT::OUT6);
    }
}

#[no_mangle]
extern "C" fn abort() -> ! {
    loop{}
}
