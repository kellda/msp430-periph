
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFEA RFPMM
    RFPMM = 5,
    /// 0xFFEC Port 1
    PORT1 = 6,
    /// 0xFFEE RF-SD14 Sigma-Delta ADC
    SD14 = 7,
    /// 0xFFEE RF-SD14 Sigma-Delta ADC
    SD_ADC = 7,
    /// 0xFFF2 ISO Module
    RF13M = 9,
    /// 0xFFF2 ISO Module
    ISO = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Timer0_A3 CC1-2, TA1
    TIMER0_A1 = 11,
    /// 0xFFF8 Timer0_A3 CC0
    TIMER0_A0 = 12,
    /// 0xFFFA User Non-maskable
    UNMI = 13,
    /// 0xFFFC System Non-maskable
    SYSNMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn RFPMM();
    fn PORT1();
    fn SD14();
    fn SD_ADC();
    fn RF13M();
    fn ISO();
    fn WDT();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 16] = [
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: RFPMM },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: SD14 },
    crate::Vector { _handler: SD_ADC },
    crate::Vector { _handler: RF13M },
    crate::Vector { _handler: ISO },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
