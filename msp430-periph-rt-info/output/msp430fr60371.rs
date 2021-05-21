
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFB4
    LEA = 17,
    /// 0xFFB6
    PORT9 = 18,
    /// 0xFFB8
    PORT8 = 19,
    /// 0xFFBA
    PORT7 = 20,
    /// 0xFFBC
    EUSCI_B1 = 21,
    /// 0xFFBE
    EUSCI_A3 = 22,
    /// 0xFFC0
    EUSCI_A2 = 23,
    /// 0xFFC2
    PORT6 = 24,
    /// 0xFFC4
    PORT5 = 25,
    /// 0xFFC6
    TIMER4_A1 = 26,
    /// 0xFFC8
    TIMER4_A0 = 27,
    /// 0xFFCA
    AES256 = 28,
    /// 0xFFCC
    RTC_C = 29,
    /// 0xFFCE
    LCD_C = 30,
    /// 0xFFD0
    PORT4 = 31,
    /// 0xFFD2
    PORT3 = 32,
    /// 0xFFD4
    TIMER3_A1 = 33,
    /// 0xFFD6
    TIMER3_A0 = 34,
    /// 0xFFD8
    PORT2 = 35,
    /// 0xFFDA
    TIMER2_A1 = 36,
    /// 0xFFDC
    TIMER2_A0 = 37,
    /// 0xFFDE
    PORT1 = 38,
    /// 0xFFE0
    TIMER1_A1 = 39,
    /// 0xFFE2
    TIMER1_A0 = 40,
    /// 0xFFE4
    DMA = 41,
    /// 0xFFE6
    EUSCI_A1 = 42,
    /// 0xFFE8
    TIMER0_A1 = 43,
    /// 0xFFEA
    TIMER0_A0 = 44,
    /// 0xFFEC
    ADC12_B = 45,
    /// 0xFFEE
    EUSCI_B0 = 46,
    /// 0xFFF0
    EUSCI_A0 = 47,
    /// 0xFFF2
    WDT = 48,
    /// 0xFFF4
    TIMER0_B1 = 49,
    /// 0xFFF6
    TIMER0_B0 = 50,
    /// 0xFFF8
    COMP_E = 51,
    /// 0xFFFA
    UNMI = 52,
    /// 0xFFFC
    SYSNMI = 53,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn LEA();
    fn PORT9();
    fn PORT8();
    fn PORT7();
    fn EUSCI_B1();
    fn EUSCI_A3();
    fn EUSCI_A2();
    fn PORT6();
    fn PORT5();
    fn TIMER4_A1();
    fn TIMER4_A0();
    fn AES256();
    fn RTC_C();
    fn LCD_C();
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
    fn EUSCI_A1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC12_B();
    fn EUSCI_B0();
    fn EUSCI_A0();
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
static __INTERRUPTS: [crate::Vector; 54] = [
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
    crate::Vector { _handler: LEA },
    crate::Vector { _handler: PORT9 },
    crate::Vector { _handler: PORT8 },
    crate::Vector { _handler: PORT7 },
    crate::Vector { _handler: EUSCI_B1 },
    crate::Vector { _handler: EUSCI_A3 },
    crate::Vector { _handler: EUSCI_A2 },
    crate::Vector { _handler: PORT6 },
    crate::Vector { _handler: PORT5 },
    crate::Vector { _handler: TIMER4_A1 },
    crate::Vector { _handler: TIMER4_A0 },
    crate::Vector { _handler: AES256 },
    crate::Vector { _handler: RTC_C },
    crate::Vector { _handler: LCD_C },
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
    crate::Vector { _handler: EUSCI_A1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC12_B },
    crate::Vector { _handler: EUSCI_B0 },
    crate::Vector { _handler: EUSCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: COMP_E },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
