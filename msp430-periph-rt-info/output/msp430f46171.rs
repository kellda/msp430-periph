
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFDE DMA
    DMA = 15,
    /// 0xFFE0 Basic Timer / RTC
    BASICTIMER = 16,
    /// 0xFFE2 Port 2
    PORT2 = 17,
    /// 0xFFE4 USART 1 Transmit
    USART1TX = 18,
    /// 0xFFE6 USART 1 Receive
    USART1RX = 19,
    /// 0xFFE8 Port 1
    PORT1 = 20,
    /// 0xFFEA Timer A CC1-2, TA
    TIMERA1 = 21,
    /// 0xFFEC Timer A CC0
    TIMERA0 = 22,
    /// 0xFFF0 USCI A0/B0 Transmit
    USCIAB0TX = 24,
    /// 0xFFF2 USCI A0/B0 Receive
    USCIAB0RX = 25,
    /// 0xFFF4 Watchdog Timer
    WDT = 26,
    /// 0xFFF6 Comparator A
    COMPARATORA = 27,
    /// 0xFFF8 Timer B CC1-2, TB
    TIMERB1 = 28,
    /// 0xFFFA Timer B CC0
    TIMERB0 = 29,
    /// 0xFFFC Non-maskable
    NMI = 30,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn DMA();
    fn BASICTIMER();
    fn PORT2();
    fn USART1TX();
    fn USART1RX();
    fn PORT1();
    fn TIMERA1();
    fn TIMERA0();
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
static __INTERRUPTS: [crate::Vector; 31] = [
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: USART1TX },
    crate::Vector { _handler: USART1RX },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: USCIAB0TX },
    crate::Vector { _handler: USCIAB0RX },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMERB1 },
    crate::Vector { _handler: TIMERB0 },
    crate::Vector { _handler: NMI },
];
