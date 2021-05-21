//! MSP430C091
use crate::peripherals::*;

utils::device! {
    /// MSP430C091
    #[all:cfg_attr(not(feature = "msp430c091-all"), non_exhaustive)]
    MSP430C091;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_2")]
    SFR @ 0x0100: sfr_2::SFR;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CCS  Compact System Clock
    #[all:cfg(feature = "ccs")]
    CCS @ 0x0160: ccs::CCS;
    /// CSYS  Compact System Module
    #[all:cfg(feature = "csys_1")]
    CSYS @ 0x0180: csys_1::CSYS;
    /// Analog Pool
    #[all:cfg(feature = "analog_pool")]
    Analog_Pool @ 0x01a0: analog_pool::AnalogPool;
    /// Port A
    #[all:cfg(feature = "portw_2i")]
    Port_A @ 0x0200: portw_2i::Port;
    /// Port 1
    #[all:cfg(feature = "portb_2i1")]
    Port_1 @ 0x0200: portb_2i1::Port;
    /// Port 2
    #[all:cfg(feature = "portb_2i2")]
    Port_2 @ 0x0201: portb_2i2::Port;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer0_A3 @ 0x0340: timer_a3_1::TimerA3;
    /// Timer1_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer1_A3 @ 0x0380: timer_a3_1::TimerA3;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFEA Port 2
    PORT2 = 5,
    /// 0xFFEC Timer0_A3 CC1-2, TA1
    TIMER0_A1 = 6,
    /// 0xFFEE Timer0_A3 CC0
    TIMER0_A0 = 7,
    /// 0xFFF0 Port 1
    PORT1 = 8,
    /// 0xFFF2 Analog Pool
    APOOL = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Timer1_A5 CC1-4, TA
    TIMER1_A1 = 11,
    /// 0xFFF8 Timer1_A5 CC0
    TIMER1_A0 = 12,
    /// 0xFFFA User Non-maskable
    UNMI = 13,
    /// 0xFFFC System Non-maskable
    SYSNMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT2();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn PORT1();
    fn APOOL();
    fn WDT();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: APOOL },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
