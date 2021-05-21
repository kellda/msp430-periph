
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFD2 RTC
    RTC = 41,
    /// 0xFFD4 Port 2
    PORT2 = 42,
    /// 0xFFD6 Timer2_A5 CC1-4, TA
    TIMER2_A1 = 43,
    /// 0xFFD8 Timer2_A5 CC0
    TIMER2_A0 = 44,
    /// 0xFFDA USCI B1 Receive/Transmit
    USCI_B1 = 45,
    /// 0xFFDC USCI A1 Receive/Transmit
    USCI_A1 = 46,
    /// 0xFFDE Port 1
    PORT1 = 47,
    /// 0xFFE0 Timer1_A3 CC1-2, TA1
    TIMER1_A1 = 48,
    /// 0xFFE2 Timer1_A3 CC0
    TIMER1_A0 = 49,
    /// 0xFFE4 DMA
    DMA = 50,
    /// 0xFFE8 Timer0_A5 CC1-4, TA
    TIMER0_A1 = 52,
    /// 0xFFEA Timer0_A5 CC0
    TIMER0_A0 = 53,
    /// 0xFFEE USCI B0 Receive/Transmit
    USCI_B0 = 55,
    /// 0xFFF0 USCI A0 Receive/Transmit
    USCI_A0 = 56,
    /// 0xFFF2 Watchdog Timer
    WDT = 57,
    /// 0xFFF4 Timer0_B7 CC1-6, TB
    TIMER0_B1 = 58,
    /// 0xFFF6 Timer0_B7 CC0
    TIMER0_B0 = 59,
    /// 0xFFF8 Comparator B
    COMP_B = 60,
    /// 0xFFFA User Non-maskable
    UNMI = 61,
    /// 0xFFFC System Non-maskable
    SYSNMI = 62,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn RTC();
    fn PORT2();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn USCI_B1();
    fn USCI_A1();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_B();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 63] = [
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
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _handler: USCI_B1 },
    crate::Vector { _handler: USCI_A1 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: COMP_B },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
