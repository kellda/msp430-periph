
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFCA
    ECOMP0 = 19,
    /// 0xFFCC
    PORT6 = 20,
    /// 0xFFCE
    PORT5 = 21,
    /// 0xFFD0
    PORT4 = 22,
    /// 0xFFD2
    PORT3 = 23,
    /// 0xFFD4
    PORT2 = 24,
    /// 0xFFD6
    PORT1 = 25,
    /// 0xFFD8
    ADC = 26,
    /// 0xFFDA
    EUSCI_B1 = 27,
    /// 0xFFDC
    EUSCI_B0 = 28,
    /// 0xFFDE
    EUSCI_A1 = 29,
    /// 0xFFE0
    EUSCI_A0 = 30,
    /// 0xFFE2
    WDT = 31,
    /// 0xFFE4
    RTC = 32,
    /// 0xFFE6
    TIMER0_B1 = 33,
    /// 0xFFE8
    TIMER0_B0 = 34,
    /// 0xFFEA
    TIMER3_A1 = 35,
    /// 0xFFEC
    TIMER3_A0 = 36,
    /// 0xFFEE
    TIMER2_A1 = 37,
    /// 0xFFF0
    TIMER2_A0 = 38,
    /// 0xFFF2
    TIMER1_A1 = 39,
    /// 0xFFF4
    TIMER1_A0 = 40,
    /// 0xFFF6
    TIMER0_A1 = 41,
    /// 0xFFF8
    TIMER0_A0 = 42,
    /// 0xFFFA
    UNMI = 43,
    /// 0xFFFC
    SYSNMI = 44,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn ECOMP0();
    fn PORT6();
    fn PORT5();
    fn PORT4();
    fn PORT3();
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn EUSCI_B1();
    fn EUSCI_B0();
    fn EUSCI_A1();
    fn EUSCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 45] = [
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
    crate::Vector { _handler: ECOMP0 },
    crate::Vector { _handler: PORT6 },
    crate::Vector { _handler: PORT5 },
    crate::Vector { _handler: PORT4 },
    crate::Vector { _handler: PORT3 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: ADC },
    crate::Vector { _handler: EUSCI_B1 },
    crate::Vector { _handler: EUSCI_B0 },
    crate::Vector { _handler: EUSCI_A1 },
    crate::Vector { _handler: EUSCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: TIMER3_A1 },
    crate::Vector { _handler: TIMER3_A0 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
