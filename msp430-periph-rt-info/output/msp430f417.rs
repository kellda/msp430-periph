
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE0 Basic Timer
    BASICTIMER = 0,
    /// 0xFFE2 Port 2
    PORT2 = 1,
    /// 0xFFE8 Port 1
    PORT1 = 4,
    /// 0xFFEA Timer0_A CC1-2, TA0
    TIMER0_A1 = 5,
    /// 0xFFEC Timer0_A CC0
    TIMER0_A0 = 6,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Comparator A
    COMPARATORA = 11,
    /// 0xFFF8 Timer1_A CC1-4, TA1
    TIMER1_A1 = 12,
    /// 0xFFFA Timer1_A CC0
    TIMER1_A0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn BASICTIMER();
    fn PORT2();
    fn PORT1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn WDT();
    fn COMPARATORA();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: NMI },
];
