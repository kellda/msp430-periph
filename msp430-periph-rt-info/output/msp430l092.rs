
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
