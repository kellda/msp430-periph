
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFDA Port 2
    PORT2 = 41,
    /// 0xFFDC Port 1
    PORT1 = 42,
    /// 0xFFDE ADC
    ADC = 43,
    /// 0xFFE0 USCI B0 Receive/Transmit
    USCI_B0 = 44,
    /// 0xFFE2 USCI A1 Receive/Transmit
    USCI_A1 = 45,
    /// 0xFFE4 USCI A0 Receive/Transmit
    USCI_A0 = 46,
    /// 0xFFE6 Watchdog Timer
    WDT = 47,
    /// 0xFFE8 RTC
    RTC = 48,
    /// 0xFFEA Timer3_A2 CC1, TA
    TIMER3_A1 = 49,
    /// 0xFFEC Timer3_A2 CC0
    TIMER3_A0 = 50,
    /// 0xFFEE Timer2_A2 CC1, TA
    TIMER2_A1 = 51,
    /// 0xFFF0 Timer2_A2 CC0
    TIMER2_A0 = 52,
    /// 0xFFF2 Timer1_A3 CC1-2, TA
    TIMER1_A1 = 53,
    /// 0xFFF4 Timer1_A3 CC0
    TIMER1_A0 = 54,
    /// 0xFFF6 Timer0_A3 CC1-2, TA
    TIMER0_A1 = 55,
    /// 0xFFE8 Timer0_A3 CC0
    TIMER0_A0 = 56,
    /// 0xFFFA User Non-maskable
    UNMI = 57,
    /// 0xFFFC System Non-maskable
    SYSNMI = 58,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn USCI_B0();
    fn USCI_A1();
    fn USCI_A0();
    fn WDT();
    fn RTC();
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
static __INTERRUPTS: [crate::Vector; 59] = [
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
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: ADC },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A1 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: RTC },
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
