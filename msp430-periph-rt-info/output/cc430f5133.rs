
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFDA AES
    AES = 45,
    /// 0xFFDC RTC
    RTC = 46,
    /// 0xFFE0 Port 2
    PORT2 = 48,
    /// 0xFFE2 Port 1
    PORT1 = 49,
    /// 0xFFE4 Timer1_A3 CC1-2, TA1
    TIMER1_A1 = 50,
    /// 0xFFE6 Timer1_A3 CC0
    TIMER1_A0 = 51,
    /// 0xFFE8 DMA
    DMA = 52,
    /// 0xFFEA CC1101 Radio Interface
    CC1101 = 53,
    /// 0xFFEC Timer0_A5 CC1-4, TA
    TIMER0_A1 = 54,
    /// 0xFFEE Timer0_A5 CC0
    TIMER0_A0 = 55,
    /// 0xFFF0 ADC
    ADC12 = 56,
    /// 0xFFF2 USCI B0 Receive/Transmit
    USCI_B0 = 57,
    /// 0xFFF4 USCI A0 Receive/Transmit
    USCI_A0 = 58,
    /// 0xFFF6 Watchdog Timer
    WDT = 59,
    /// 0xFFF8 Comparator B
    COMP_B = 60,
    /// 0xFFFA User Non-maskable
    UNMI = 61,
    /// 0xFFFC System Non-maskable
    SYSNMI = 62,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn AES();
    fn RTC();
    fn PORT2();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn CC1101();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC12();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
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
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: AES },
    crate::Vector { _handler: RTC },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: CC1101 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC12 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMP_B },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
