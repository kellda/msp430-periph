
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE4 Port 1
    PORT1 = 2,
    /// 0xFFE6 Port 2
    PORT2 = 3,
    /// 0xFFEA ADC10
    ADC10 = 5,
    /// 0xFFEC USCI A0/B0 Transmit
    USCIAB0TX = 6,
    /// 0xFFEE USCI A0/B0 Receive
    USCIAB0RX = 7,
    /// 0xFFF0 Timer0 A CC1-2, TA
    TIMER0_A1 = 8,
    /// 0xFFF2 Timer0 A CC0
    TIMER0_A0 = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Comparator A
    COMPARATORA = 11,
    /// 0xFFF8 Timer1 A CC1-2, TA
    TIMER1_A1 = 12,
    /// 0xFFFA Timer1 A CC0
    TIMER1_A0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT1();
    fn PORT2();
    fn ADC10();
    fn USCIAB0TX();
    fn USCIAB0RX();
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
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: ADC10 },
    crate::Vector { _handler: USCIAB0TX },
    crate::Vector { _handler: USCIAB0RX },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: NMI },
];
