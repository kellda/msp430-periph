//! MSP430F235
use crate::peripherals::*;

utils::device! {
    /// MSP430F235
    #[cfg_attr(not(feature = "MSP430F235-all"), non_exhaustive)]
    MSP430F235;
    /// Special Function
    #[cfg(feature = "special_function_11")]
    Special_Function @ 0x0000: special_function_11::SpecialFunction;
    /// Port 3/4
    #[cfg(feature = "port_3_4_3")]
    Port_3_4 @ 0x0010: port_3_4_3::Port34;
    /// Port 5/6
    #[cfg(feature = "port_5_6_2")]
    Port_5_6 @ 0x0012: port_5_6_2::Port56;
    /// Port 1/2
    #[cfg(feature = "port_1_2_2")]
    Port_1_2 @ 0x0020: port_1_2_2::Port12;
    /// System Clock
    #[cfg(feature = "system_clock_5")]
    System_Clock @ 0x0053: system_clock_5::SystemClock;
    /// Supply Voltage Supervisor
    #[cfg(feature = "supply_voltage_supervisor_3")]
    Supply_Voltage_Supervisor @ 0x0055: supply_voltage_supervisor_3::SupplyVoltageSupervisor;
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
    /// ADC12
    #[cfg(feature = "adc12_1")]
    ADC12 @ 0x0080: adc12_1::ADC12;
    /// Timer B3
    #[cfg(feature = "timer_b3_1")]
    Timer_B3 @ 0x011e: timer_b3_1::TimerB3;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[cfg(feature = "flash_5")]
    Flash @ 0x0128: flash_5::Flash;
    /// Timer A3
    #[cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// Multiplier
    #[cfg(feature = "multiplier_1")]
    Multiplier @ 0x0130: multiplier_1::Multiplier;
    /// TLV Calibration Data
    #[cfg(feature = "tlv_calibration_data_2")]
    TLV_Calibration_Data @ 0x10c0: tlv_calibration_data_2::TLVCalibrationData;
    /// Calibration Data
    #[cfg(feature = "calibration_data_1")]
    Calibration_Data @ 0x10f8: calibration_data_1::CalibrationData;
}
