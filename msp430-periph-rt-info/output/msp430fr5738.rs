
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFCE RTC
    RTC = 31,
    /// 0xFFD8 Port 2
    PORT2 = 36,
    /// 0xFFDE Port 1
    PORT1 = 39,
    /// 0xFFE0 Timer1_A3 CC1-2, TA1
    TIMER1_A1 = 40,
    /// 0xFFE2 Timer1_A3 CC0
    TIMER1_A0 = 41,
    /// 0xFFE4 DMA
    DMA = 42,
    /// 0xFFE8 Timer0_A5 CC1-4, TA
    TIMER0_A1 = 44,
    /// 0xFFEA Timer0_A5 CC0
    TIMER0_A0 = 45,
    /// 0xFFEC ADC
    ADC10 = 46,
    /// 0xFFEE USCI B0 Receive/Transmit
    USCI_B0 = 47,
    /// 0xFFF0 USCI A0 Receive/Transmit
    USCI_A0 = 48,
    /// 0xFFF2 Watchdog Timer
    WDT = 49,
    /// 0xFFF4 Timer0_B3 CC1-2, TB
    TIMER0_B1 = 50,
    /// 0xFFF6 Timer0_B3 CC0
    TIMER0_B0 = 51,
    /// 0xFFF8 Comparator D
    COMP_D = 52,
    /// 0xFFFA User Non-maskable
    UNMI = 53,
    /// 0xFFFC System Non-maskable
    SYSNMI = 54,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn RTC();
    fn PORT2();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC10();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_D();
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
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: RTC },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC10 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: COMP_D },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
