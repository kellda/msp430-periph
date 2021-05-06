//! MSP430G2553
use crate::peripherals::*;

utils::device! {
    /// MSP430G2553
    #[cfg_attr(not(feature = "MSP430G2553-all"), non_exhaustive)]
    MSP430G2553;
    /// Special Function
    #[cfg(feature = "special_function_11")]
    Special_Function @ 0x0000: special_function_11::SpecialFunction;
    /// Port 3/4
    #[cfg(feature = "port_3_4_8")]
    Port_3_4 @ 0x0010: port_3_4_8::Port34;
    /// Port 1/2
    #[cfg(feature = "port_1_2_3")]
    Port_1_2 @ 0x0020: port_1_2_3::Port12;
    /// ADC10
    #[cfg(feature = "adc10_2")]
    ADC10 @ 0x0048: adc10_2::ADC10;
    /// System Clock
    #[cfg(feature = "system_clock_3")]
    System_Clock @ 0x0053: system_clock_3::SystemClock;
    /// Comparator A
    #[cfg(feature = "comparator_a_2")]
    Comparator_A @ 0x0059: comparator_a_2::ComparatorA;
    /// USCI_A0  UART Mode
    #[cfg(feature = "usci_a0__uart_mode_1")]
    USCI_A0__UART_Mode @ 0x005d: usci_a0__uart_mode_1::USCI_A0UARTMode;
    /// USCI_A0  SPI Mode
    #[cfg(feature = "usci_a0__spi_mode_1")]
    USCI_A0__SPI_Mode @ 0x0060: usci_a0__spi_mode_1::USCI_A0SPIMode;
    /// USCI_B0  SPI Mode
    #[cfg(feature = "usci_b0__spi_mode_1")]
    USCI_B0__SPI_Mode @ 0x0068: usci_b0__spi_mode_1::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[cfg(feature = "usci_b0__i2c_mode_1")]
    USCI_B0__I2C_Mode @ 0x0068: usci_b0__i2c_mode_1::USCI_B0I2CMode;
    /// Timer1_A3
    #[cfg(feature = "timer1_a3_2")]
    Timer1_A3 @ 0x011e: timer1_a3_2::Timer1_A3;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[cfg(feature = "flash_3")]
    Flash @ 0x0128: flash_3::Flash;
    /// Timer0_A3
    #[cfg(feature = "timer0_a3_1")]
    Timer0_A3 @ 0x012e: timer0_a3_1::Timer0_A3;
    /// TLV Calibration Data
    #[cfg(feature = "tlv_calibration_data_1")]
    TLV_Calibration_Data @ 0x10c0: tlv_calibration_data_1::TLVCalibrationData;
    /// Calibration Data
    #[cfg(feature = "calibration_data_1")]
    Calibration_Data @ 0x10f8: calibration_data_1::CalibrationData;
}
