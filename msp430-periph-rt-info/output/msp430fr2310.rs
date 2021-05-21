
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE2
    ECOMP0 = 44,
    /// 0xFFE4
    PORT2 = 45,
    /// 0xFFE6
    PORT1 = 46,
    /// 0xFFE8
    ADC = 47,
    /// 0xFFEA
    EUSCI_B0 = 48,
    /// 0xFFEC
    EUSCI_A0 = 49,
    /// 0xFFEE
    WDT = 50,
    /// 0xFFF0
    RTC = 51,
    /// 0xFFF2
    TIMER1_B1 = 52,
    /// 0xFFF4
    TIMER1_B0 = 53,
    /// 0xFFF6
    TIMER0_B1 = 54,
    /// 0xFFF8
    TIMER0_B0 = 55,
    /// 0xFFFA
    UNMI = 56,
    /// 0xFFFC
    SYSNMI = 57,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn ECOMP0();
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn EUSCI_B0();
    fn EUSCI_A0();
    fn WDT();
    fn RTC();
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
static __INTERRUPTS: [crate::Vector; 58] = [
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
    crate::Vector { _handler: ECOMP0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: ADC },
    crate::Vector { _handler: EUSCI_B0 },
    crate::Vector { _handler: EUSCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: TIMER1_B1 },
    crate::Vector { _handler: TIMER1_B0 },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
