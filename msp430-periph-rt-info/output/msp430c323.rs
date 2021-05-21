
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE0 Port 0 Bits 2-7 [Lowest Priority]
    PORT0 = 0,
    /// 0xFFE2 Basic Timer
    BASICTIMER = 1,
    /// 0xFFE8 Timer/Port
    TIMERPORT = 4,
    /// 0xFFEA ADC
    ADC = 5,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF8 Dedicated IO (P0.1)
    IO1 = 12,
    /// 0xFFFA Dedicated IO (P0.0)
    IO0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT0();
    fn BASICTIMER();
    fn TIMERPORT();
    fn ADC();
    fn WDT();
    fn IO1();
    fn IO0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _handler: PORT0 },
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: TIMERPORT },
    crate::Vector { _handler: ADC },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: IO1 },
    crate::Vector { _handler: IO0 },
    crate::Vector { _handler: NMI },
];
