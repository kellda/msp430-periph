//! MSP430G2755
use crate::peripherals::*;

utils::device! {
    /// MSP430G2755
    #[all:cfg_attr(not(feature = "msp430g2755-all"), non_exhaustive)]
    MSP430G2755;
    /// Special Function
    #[all:cfg(feature = "special_function_11")]
    Special_Function @ 0x0000: special_function_11::SpecialFunction;
    /// Port 3
    #[all:cfg(feature = "port_3_2")]
    Port_3 @ 0x0010: port_3_2::Port;
    /// Port 4
    #[all:cfg(feature = "port_4_2")]
    Port_4 @ 0x0011: port_4_2::Port;
    /// Port 1
    #[all:cfg(feature = "port_1_1")]
    Port_1 @ 0x0020: port_1_1::Port;
    /// Port 2
    #[all:cfg(feature = "port_2_1")]
    Port_2 @ 0x0028: port_2_1::Port;
    /// ADC10
    #[all:cfg(feature = "adc10_3")]
    ADC10 @ 0x0048: adc10_3::ADC10;
    /// System Clock
    #[all:cfg(feature = "system_clock_5")]
    System_Clock @ 0x0053: system_clock_5::SystemClock;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_2")]
    Comparator_A @ 0x0059: comparator_a_2::ComparatorA;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_1")]
    USCI_A0_UART @ 0x005d: usci_a_uart_1::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_1")]
    USCI_A0_SPI @ 0x0060: usci_a_spi_1::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_1")]
    USCI_B0_SPI @ 0x0068: usci_b_spi_1::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_1")]
    USCI_B0_I2C @ 0x0068: usci_b_i2c_1::USCI_B_I2C;
    /// Timer1_A3
    #[all:cfg(feature = "timer_a3_4")]
    Timer1_A3 @ 0x011c: timer_a3_4::TimerA3;
    /// Timer B3
    #[all:cfg(feature = "timer_b3_2")]
    Timer_B3 @ 0x011e: timer_b3_2::TimerB3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_3")]
    Flash @ 0x0128: flash_3::Flash;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer0_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// TLV Calibration Data
    #[all:cfg(feature = "tlv_calibration_data_1")]
    TLV_Calibration_Data @ 0x10c0: tlv_calibration_data_1::TLVCalibrationData;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_1")]
    Calibration_Data @ 0x10f8: calibration_data_1::CalibrationData;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE0 Timer1_A CC1-4, TA1
    TIMER1_A1 = 0,
    /// 0xFFE2 Timer1_A CC0
    TIMER1_A0 = 1,
    /// 0xFFE4 Port 1
    PORT1 = 2,
    /// 0xFFE6 Port 2
    PORT2 = 3,
    /// 0xFFE8 TRAPINT
    TRAPINT = 4,
    /// 0xFFEA ADC10
    ADC10 = 5,
    /// 0xFFEC USCI A0/B0 Transmit
    USCIAB0TX = 6,
    /// 0xFFEE USCI A0/B0 Receive
    USCIAB0RX = 7,
    /// 0xFFF0 Timer0_A CC1, TA0
    TIMER0_A1 = 8,
    /// 0xFFF2 Timer0_A CC0
    TIMER0_A0 = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Comparator A
    COMPARATORA = 11,
    /// 0xFFF8 Timer B CC1-6, TB
    TIMERB1 = 12,
    /// 0xFFFA Timer B CC0
    TIMERB0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn PORT1();
    fn PORT2();
    fn TRAPINT();
    fn ADC10();
    fn USCIAB0TX();
    fn USCIAB0RX();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn WDT();
    fn COMPARATORA();
    fn TIMERB1();
    fn TIMERB0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TRAPINT },
    crate::Vector { _handler: ADC10 },
    crate::Vector { _handler: USCIAB0TX },
    crate::Vector { _handler: USCIAB0RX },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMERB1 },
    crate::Vector { _handler: TIMERB0 },
    crate::Vector { _handler: NMI },
];
