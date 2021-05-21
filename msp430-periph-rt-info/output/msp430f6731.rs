
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFD0 RTC
    RTC = 40,
    /// 0xFFD2 LCD C
    LCD_C = 41,
    /// 0xFFD4 Timer3_A2 CC1, TA
    TIMER3_A1 = 42,
    /// 0xFFD8 Timer3_A2 CC0
    TIMER3_A0 = 43,
    /// 0xFFDA Port 2
    PORT2 = 44,
    /// 0xFFDC Timer2_A2 CC1, TA
    TIMER2_A1 = 45,
    /// 0xFFDE Timer2_A2 CC0
    TIMER2_A0 = 46,
    /// 0xFFDE Port 1
    PORT1 = 47,
    /// 0xFFE0 Timer1_A2 CC1, TA1
    TIMER1_A1 = 48,
    /// 0xFFE2 Timer1_A2 CC0
    TIMER1_A0 = 49,
    /// 0xFFE4 DMA
    DMA = 50,
    /// 0xFFE6 AUX Supply
    AUX = 51,
    /// 0xFFE8 USCI A2 Receive/Transmit
    USCI_A2 = 52,
    /// 0xFFEA USCI A1 Receive/Transmit
    USCI_A1 = 53,
    /// 0xFFEC Timer0_A2 CC1-CC2, TA
    TIMER0_A1 = 54,
    /// 0xFFEE Timer0_A2 CC0
    TIMER0_A0 = 55,
    /// 0xFFF0 SD24B ADC
    SD24B = 56,
    /// 0xFFF2 ADC
    ADC10 = 57,
    /// 0xFFF4 USCI B0 Receive/Transmit
    USCI_B0 = 58,
    /// 0xFFF6 USCI A0 Receive/Transmit
    USCI_A0 = 59,
    /// 0xFFF8 Watchdog Timer
    WDT = 60,
    /// 0xFFFA User Non-maskable
    UNMI = 61,
    /// 0xFFFC System Non-maskable
    SYSNMI = 62,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn RTC();
    fn LCD_C();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn PORT2();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn AUX();
    fn USCI_A2();
    fn USCI_A1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn SD24B();
    fn ADC10();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
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
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: LCD_C },
    crate::Vector { _handler: TIMER3_A1 },
    crate::Vector { _handler: TIMER3_A0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: AUX },
    crate::Vector { _handler: USCI_A2 },
    crate::Vector { _handler: USCI_A1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: SD24B },
    crate::Vector { _handler: ADC10 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
