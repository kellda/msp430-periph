//! MSP430F2617
use crate::peripherals::*;

utils::device! {
    /// MSP430F2617
    #[all:cfg_attr(not(feature = "msp430f2617-all"), non_exhaustive)]
    MSP430F2617;
    /// Special Function
    #[all:cfg(feature = "special_function_12")]
    Special_Function @ 0x0000: special_function_12::SpecialFunction;
    /// Port 3
    #[all:cfg(feature = "port_x_1")]
    Port_3 @ 0x0010: port_x_1::Port;
    /// Port 4
    #[all:cfg(feature = "port_4_1")]
    Port_4 @ 0x0011: port_4_1::Port;
    /// Port 5
    #[all:cfg(feature = "port_5_1")]
    Port_5 @ 0x0012: port_5_1::Port;
    /// Port 6
    #[all:cfg(feature = "port_6_1")]
    Port_6 @ 0x0013: port_6_1::Port;
    /// Port 7
    #[all:cfg(feature = "port_78")]
    Port_7 @ 0x0014: port_78::Port;
    /// Port 8
    #[all:cfg(feature = "port_78")]
    Port_8 @ 0x0015: port_78::Port;
    /// Port A
    #[all:cfg(feature = "port_a")]
    Port_A @ 0x0014: port_a::Port;
    /// Port 1
    #[all:cfg(feature = "port_12_2")]
    Port_1 @ 0x0020: port_12_2::Port;
    /// Port 2
    #[all:cfg(feature = "port_12_2")]
    Port_2 @ 0x0028: port_12_2::Port;
    /// System Clock
    #[all:cfg(feature = "system_clock_5")]
    System_Clock @ 0x0053: system_clock_5::SystemClock;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0055: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
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
    /// ADC12
    #[all:cfg(feature = "adc12_1")]
    ADC12 @ 0x0080: adc12_1::ADC12;
    /// USCI_A1  UART Mode
    #[all:cfg(feature = "usci_a_uart_1")]
    USCI_A1_UART @ 0x00cd: usci_a_uart_1::USCI_A_UART;
    /// USCI_A1  SPI Mode
    #[all:cfg(feature = "usci_a_spi_1")]
    USCI_A1_SPI @ 0x00d0: usci_a_spi_1::USCI_A_SPI;
    /// USCI_B1  SPI Mode
    #[all:cfg(feature = "usci_b_spi_1")]
    USCI_B1_SPI @ 0x00d8: usci_b_spi_1::USCI_B_SPI;
    /// USCI_B1  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_3")]
    USCI_B1_I2C @ 0x00d8: usci_b_i2c_3::USCI_B_I2C;
    /// Timer B7
    #[all:cfg(feature = "timer_b7_1")]
    Timer_B7 @ 0x011e: timer_b7_1::TimerB7;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// DMA
    #[all:cfg(feature = "dma_3")]
    DMA @ 0x0122: dma_3::DMA;
    /// Flash
    #[all:cfg(feature = "flash_5")]
    Flash @ 0x0128: flash_5::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
    /// DAC12
    #[all:cfg(feature = "dac12_1")]
    DAC12 @ 0x01c0: dac12_1::DAC12;
    /// TLV Calibration Data
    #[all:cfg(feature = "tlv_calibration_data_2")]
    TLV_Calibration_Data @ 0x10c0: tlv_calibration_data_2::TLVCalibrationData;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_1")]
    Calibration_Data @ 0x10f8: calibration_data_1::CalibrationData;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFC0 Reserved Int. Vector 0
    RESERVED0 = 0,
    /// 0xFFC2 Reserved Int. Vector 1
    RESERVED1 = 1,
    /// 0xFFC4 Reserved Int. Vector 2
    RESERVED2 = 2,
    /// 0xFFC6 Reserved Int. Vector 3
    RESERVED3 = 3,
    /// 0xFFC8 Reserved Int. Vector 4
    RESERVED4 = 4,
    /// 0xFFCA Reserved Int. Vector 5
    RESERVED5 = 5,
    /// 0xFFCC Reserved Int. Vector 6
    RESERVED6 = 6,
    /// 0xFFCE Reserved Int. Vector 7
    RESERVED7 = 7,
    /// 0xFFD0 Reserved Int. Vector 8
    RESERVED8 = 8,
    /// 0xFFD2 Reserved Int. Vector 9
    RESERVED9 = 9,
    /// 0xFFD4 Reserved Int. Vector 10
    RESERVED10 = 10,
    /// 0xFFD6 Reserved Int. Vector 11
    RESERVED11 = 11,
    /// 0xFFD8 Reserved Int. Vector 12
    RESERVED12 = 12,
    /// 0xFFDA Reserved Int. Vector 13
    RESERVED13 = 13,
    /// 0xFFDC DAC12
    DAC12 = 14,
    /// 0xFFDE DMA
    DMA = 15,
    /// 0xFFE0 USCI A1/B1 Transmit
    USCIAB1TX = 16,
    /// 0xFFE2 USCI A1/B1 Receive
    USCIAB1RX = 17,
    /// 0xFFE4 Port 1
    PORT1 = 18,
    /// 0xFFE6 Port 2
    PORT2 = 19,
    /// 0xFFE8 Reserved Int. Vector 20
    RESERVED20 = 20,
    /// 0xFFEA ADC
    ADC12 = 21,
    /// 0xFFEC USCI A0/B0 Transmit
    USCIAB0TX = 22,
    /// 0xFFEE USCI A0/B0 Receive
    USCIAB0RX = 23,
    /// 0xFFF0 Timer A CC1-2, TA
    TIMERA1 = 24,
    /// 0xFFF2 Timer A CC0
    TIMERA0 = 25,
    /// 0xFFF4 Watchdog Timer
    WDT = 26,
    /// 0xFFF6 Comparator A
    COMPARATORA = 27,
    /// 0xFFF8 Timer B CC1-6, TB
    TIMERB1 = 28,
    /// 0xFFFA Timer B CC0
    TIMERB0 = 29,
    /// 0xFFFC Non-maskable
    NMI = 30,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn RESERVED0();
    fn RESERVED1();
    fn RESERVED2();
    fn RESERVED3();
    fn RESERVED4();
    fn RESERVED5();
    fn RESERVED6();
    fn RESERVED7();
    fn RESERVED8();
    fn RESERVED9();
    fn RESERVED10();
    fn RESERVED11();
    fn RESERVED12();
    fn RESERVED13();
    fn DAC12();
    fn DMA();
    fn USCIAB1TX();
    fn USCIAB1RX();
    fn PORT1();
    fn PORT2();
    fn RESERVED20();
    fn ADC12();
    fn USCIAB0TX();
    fn USCIAB0RX();
    fn TIMERA1();
    fn TIMERA0();
    fn WDT();
    fn COMPARATORA();
    fn TIMERB1();
    fn TIMERB0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 31] = [
    crate::Vector { _handler: RESERVED0 },
    crate::Vector { _handler: RESERVED1 },
    crate::Vector { _handler: RESERVED2 },
    crate::Vector { _handler: RESERVED3 },
    crate::Vector { _handler: RESERVED4 },
    crate::Vector { _handler: RESERVED5 },
    crate::Vector { _handler: RESERVED6 },
    crate::Vector { _handler: RESERVED7 },
    crate::Vector { _handler: RESERVED8 },
    crate::Vector { _handler: RESERVED9 },
    crate::Vector { _handler: RESERVED10 },
    crate::Vector { _handler: RESERVED11 },
    crate::Vector { _handler: RESERVED12 },
    crate::Vector { _handler: RESERVED13 },
    crate::Vector { _handler: DAC12 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: USCIAB1TX },
    crate::Vector { _handler: USCIAB1RX },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: RESERVED20 },
    crate::Vector { _handler: ADC12 },
    crate::Vector { _handler: USCIAB0TX },
    crate::Vector { _handler: USCIAB0RX },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMERB1 },
    crate::Vector { _handler: TIMERB0 },
    crate::Vector { _handler: NMI },
];
