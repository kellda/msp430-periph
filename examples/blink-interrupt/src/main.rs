#![no_std]
#![no_main]
#![feature(abi_msp430_interrupt)]

use core::cell::RefCell;
use msp430::interrupt as mspint;
use msp430_rt::entry;
use panic_msp430 as _;

use msp430_periph::devices::msp430fr5969::MSP430FR5969;
use msp430_periph::interrupt;
use msp430_periph::peripherals::{
    cs_2 as cs, pmm_4 as pmm, portb_3i1 as p1, portb_3i2 as p4, timer_a3_1 as timer0,
    watchdog_timer_2 as wdt,
};
use msp430_periph::utils::Value;

static PERIPHERALS: mspint::Mutex<RefCell<Option<MSP430FR5969>>> =
    mspint::Mutex::new(RefCell::new(None));

#[entry]
fn main(cs: CriticalSection) -> ! {
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

    // Unlock clock system register
    p.cs.ctl0.write(Value::reset() | cs::CTL0Field(0xA500));
    // Use LFO (10 kHz low frequency oscillator) as ACLK (auxiliary clock source)
    p.cs.ctl2.modify(cs::SELA::SELA_1);

    // Set the period
    p.timer0_a3
        .ccr0
        .write(Value::reset() | timer0::CCR0Field(1200));
    // Use ACLK as clock source and count upward until TACCR0
    p.timer0_a3
        .ctl
        .modify(timer0::SSEL::SSEL_1 | timer0::MC::MC_1);
    // Enable timer interrupt
    p.timer0_a3.cctl1.modify(timer0::C1CCIE(true));

    // Share peripherals with the interrupt
    *PERIPHERALS.borrow(&cs).borrow_mut() = Some(p);

    // Enable interrupts
    msp430::interrupt::enable_cs(cs);

    loop {}
}

#[interrupt]
fn TIMER0_A1(cs: CriticalSection) {
    let mut p = PERIPHERALS.borrow(&cs).borrow_mut();
    let p = p.as_mut().unwrap();

    // Clear interrupt flag
    p.timer0_a3.cctl1.modify(timer0::C1CCIFG(false));

    // Toggle outputs
    p.port_1.out.toggle(p1::OUT::OUT0);
    p.port_4.out.toggle(p4::OUT::OUT6);
}

#[no_mangle]
extern "C" fn abort() -> ! {
    loop {}
}
