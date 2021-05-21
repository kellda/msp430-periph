
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFC6 AES256
    AES256 = 27,
    /// 0xFFC8 RTC
    RTC = 28,
    /// 0xFFCC Port 4
    PORT4 = 30,
    /// 0xFFCE Port 3
    PORT3 = 31,
    /// 0xFFD0 Timer3_A2 CC1, TA
    TIMER3_A1 = 32,
    /// 0xFFD2 Timer3_A2 CC0
    TIMER3_A0 = 33,
    /// 0xFFD4 Port 2
    PORT2 = 34,
    /// 0xFFD6 Timer2_A3 CC1, TA
    TIMER2_A1 = 35,
    /// 0xFFD8 Timer2_A3 CC0
    TIMER2_A0 = 36,
    /// 0xFFDA Port 1
    PORT1 = 37,
    /// 0xFFDC Timer1_A3 CC1-2, TA1
    TIMER1_A1 = 38,
    /// 0xFFDE Timer1_A3 CC0
    TIMER1_A0 = 39,
    /// 0xFFE0 DMA
    DMA = 40,
    /// 0xFFE2 USCI B1 Receive/Transmit
    USCI_B1 = 41,
    /// 0xFFE4 USCI A1 Receive/Transmit
    USCI_A1 = 42,
    /// 0xFFE6 Timer0_A5 CC1-4, TA
    TIMER0_A1 = 43,
    /// 0xFFE8 Timer0_A5 CC0
    TIMER0_A0 = 44,
    /// 0xFFEA ADC
    ADC12 = 45,
    /// 0xFFEC USCI B0 Receive/Transmit
    USCI_B0 = 46,
    /// 0xFFEE USCI A0 Receive/Transmit
    USCI_A0 = 47,
    /// 0xFFF2 Watchdog Timer
    WDT = 49,
    /// 0xFFF4 Timer0_B3 CC1-2, TB
    TIMER0_B1 = 50,
    /// 0xFFF6 Timer0_B3 CC0
    TIMER0_B0 = 51,
    /// 0xFFF8 Comparator E
    COMP_E = 52,
    /// 0xFFFA User Non-maskable
    UNMI = 53,
    /// 0xFFFC System Non-maskable
    SYSNMI = 54,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn AES256();
    fn RTC();
    fn PORT4();
    fn PORT3();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn PORT2();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn USCI_B1();
    fn USCI_A1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC12();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_E();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 55] = [
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
    crate::Vector { _handler: AES256 },
    crate::Vector { _handler: RTC },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT4 },
    crate::Vector { _handler: PORT3 },
    crate::Vector { _handler: TIMER3_A1 },
    crate::Vector { _handler: TIMER3_A0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: USCI_B1 },
    crate::Vector { _handler: USCI_A1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC12 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: COMP_E },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
