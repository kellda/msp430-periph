
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE0 Basic Timer
    BASICTIMER = 0,
    /// 0xFFE2 Port 2
    PORT2 = 1,
    /// 0xFFE6 DAC 12
    DAC12_DMA = 3,
    /// 0xFFE8 Port 1
    PORT1 = 4,
    /// 0xFFEA Timer A CC1-2, TA
    TIMERA1 = 5,
    /// 0xFFEC Timer A CC0
    TIMERA0 = 6,
    /// 0xFFEE ADC SD16A
    SD16A = 7,
    /// 0xFFF0 USCI A0/B0 Transmit
    USCIAB0TX = 8,
    /// 0xFFF2 USCI A0/B0 Receive
    USCIAB0RX = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Comparator A
    COMPARATORA = 11,
    /// 0xFFF8 Timer B CC1-2, TB
    TIMERB1 = 12,
    /// 0xFFFA Timer B CC0
    TIMERB0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn BASICTIMER();
    fn PORT2();
    fn DAC12_DMA();
    fn PORT1();
    fn TIMERA1();
    fn TIMERA0();
    fn SD16A();
    fn USCIAB0TX();
    fn USCIAB0RX();
    fn WDT();
    fn COMPARATORA();
    fn TIMERB1();
    fn TIMERB0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: DAC12_DMA },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _handler: SD16A },
    crate::Vector { _handler: USCIAB0TX },
    crate::Vector { _handler: USCIAB0RX },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMERB1 },
    crate::Vector { _handler: TIMERB0 },
    crate::Vector { _handler: NMI },
];
