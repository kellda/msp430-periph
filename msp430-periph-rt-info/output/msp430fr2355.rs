
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFCE
    PORT4 = 21,
    /// 0xFFD0
    PORT3 = 22,
    /// 0xFFD2
    PORT2 = 23,
    /// 0xFFD4
    PORT1 = 24,
    /// 0xFFD6
    SAC1_SAC3 = 25,
    /// 0xFFD8
    SAC0_SAC2 = 26,
    /// 0xFFDA
    ECOMP0_ECOMP1 = 27,
    /// 0xFFDC
    ADC = 28,
    /// 0xFFDE
    EUSCI_B1 = 29,
    /// 0xFFE0
    EUSCI_B0 = 30,
    /// 0xFFE2
    EUSCI_A1 = 31,
    /// 0xFFE4
    EUSCI_A0 = 32,
    /// 0xFFE6
    WDT = 33,
    /// 0xFFE8
    RTC = 34,
    /// 0xFFEA
    TIMER3_B1 = 35,
    /// 0xFFEC
    TIMER3_B0 = 36,
    /// 0xFFEE
    TIMER2_B1 = 37,
    /// 0xFFF0
    TIMER2_B0 = 38,
    /// 0xFFF2
    TIMER1_B1 = 39,
    /// 0xFFF4
    TIMER1_B0 = 40,
    /// 0xFFF6
    TIMER0_B1 = 41,
    /// 0xFFF8
    TIMER0_B0 = 42,
    /// 0xFFFA
    UNMI = 43,
    /// 0xFFFC
    SYSNMI = 44,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT4();
    fn PORT3();
    fn PORT2();
    fn PORT1();
    fn SAC1_SAC3();
    fn SAC0_SAC2();
    fn ECOMP0_ECOMP1();
    fn ADC();
    fn EUSCI_B1();
    fn EUSCI_B0();
    fn EUSCI_A1();
    fn EUSCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER3_B1();
    fn TIMER3_B0();
    fn TIMER2_B1();
    fn TIMER2_B0();
    fn TIMER1_B1();
    fn TIMER1_B0();
    fn TIMER0_B1();
    fn TIMER0_B0();
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
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT4 },
    crate::Vector { _handler: PORT3 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: SAC1_SAC3 },
    crate::Vector { _handler: SAC0_SAC2 },
    crate::Vector { _handler: ECOMP0_ECOMP1 },
    crate::Vector { _handler: ADC },
    crate::Vector { _handler: EUSCI_B1 },
    crate::Vector { _handler: EUSCI_B0 },
    crate::Vector { _handler: EUSCI_A1 },
    crate::Vector { _handler: EUSCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: TIMER3_B1 },
    crate::Vector { _handler: TIMER3_B0 },
    crate::Vector { _handler: TIMER2_B1 },
    crate::Vector { _handler: TIMER2_B0 },
    crate::Vector { _handler: TIMER1_B1 },
    crate::Vector { _handler: TIMER1_B0 },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
