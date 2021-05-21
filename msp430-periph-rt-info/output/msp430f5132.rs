
/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFDA Port 2
    PORT2 = 45,
    /// 0xFFDC Port 1
    PORT1 = 46,
    /// 0xFFDE Timer1_D3 CC1-2, TA1
    TIMER1_D1 = 47,
    /// 0xFFE0 Timer1_D3 CC0
    TIMER1_D0 = 48,
    /// 0xFFF2 Timer Event Controller 1
    TEC1 = 49,
    /// 0xFFE4 DMA
    DMA = 50,
    /// 0xFFE6 Timer0_A3 CC1-2, TA
    TIMER0_A1 = 51,
    /// 0xFFE8 Timer0_A3 CC0
    TIMER0_A0 = 52,
    /// 0xFFEA ADC
    ADC10 = 53,
    /// 0xFFEC USCI B0 Receive/Transmit
    USCI_B0 = 54,
    /// 0xFFEE USCI A0 Receive/Transmit
    USCI_A0 = 55,
    /// 0xFFF0 Watchdog Timer
    WDT = 56,
    /// 0xFFE2 Timer0_D3 CC1-2, TA
    TIMER0_D1 = 57,
    /// 0xFFE4 Timer0_D3 CC0
    TIMER0_D0 = 58,
    /// 0xFFF6 Timer Event Controller 0
    TEC0 = 59,
    /// 0xFFF8 Watchdog Timer
    COMP_B = 60,
    /// 0xFFFA User Non-maskable
    UNMI = 61,
    /// 0xFFFC System Non-maskable
    SYSNMI = 62,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT2();
    fn PORT1();
    fn TIMER1_D1();
    fn TIMER1_D0();
    fn TEC1();
    fn DMA();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC10();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_D1();
    fn TIMER0_D0();
    fn TEC0();
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
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_D1 },
    crate::Vector { _handler: TIMER1_D0 },
    crate::Vector { _handler: TEC1 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC10 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_D1 },
    crate::Vector { _handler: TIMER0_D0 },
    crate::Vector { _handler: TEC0 },
    crate::Vector { _handler: COMP_B },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
