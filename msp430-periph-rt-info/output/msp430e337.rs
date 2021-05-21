
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE0 Port 0 Bits 2-7 [Lowest Priority]
    PORT0 = 0,
    /// 0xFFE2 Basic Timer
    BASICTIMER = 1,
    /// 0xFFE4 Port 1
    PORT1 = 2,
    /// 0xFFE6 Port 2
    PORT2 = 3,
    /// 0xFFE8 Timer/Port
    TIMERPORT = 4,
    /// 0xFFEC USART Transmit
    USARTTX = 6,
    /// 0xFFEE USART Receive
    USARTRX = 7,
    /// 0xFFF0 Timer A CC1-4, TA
    TIMERA1 = 8,
    /// 0xFFF2 Timer A CC0
    TIMERA0 = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF8 Dedicated IO (P0.1)
    IO1 = 12,
    /// 0xFFFA Dedicated IO (P0.0)
    IO0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT0();
    fn BASICTIMER();
    fn PORT1();
    fn PORT2();
    fn TIMERPORT();
    fn USARTTX();
    fn USARTRX();
    fn TIMERA1();
    fn TIMERA0();
    fn WDT();
    fn IO1();
    fn IO0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _handler: PORT0 },
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TIMERPORT },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: USARTTX },
    crate::Vector { _handler: USARTRX },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: IO1 },
    crate::Vector { _handler: IO0 },
    crate::Vector { _handler: NMI },
];
