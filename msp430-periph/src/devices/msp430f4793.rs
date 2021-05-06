//! MSP430F4793
use crate::peripherals::*;

utils::device! {
    /// MSP430F4793
    #[cfg_attr(not(feature = "MSP430F4793-all"), non_exhaustive)]
    MSP430F4793;
    /// Special Function
    #[cfg(feature = "special_function_16")]
    Special_Function @ 0x0000: special_function_16::SpecialFunction;
    /// Port 9/10
    #[cfg(feature = "port_9_10_2")]
    Port_9_10 @ 0x0008: port_9_10_2::Port910;
    /// Port 3/4
    #[cfg(feature = "port_3_4_3")]
    Port_3_4 @ 0x0010: port_3_4_3::Port34;
    /// Port 5
    #[cfg(feature = "port_5_1")]
    Port_5 @ 0x0012: port_5_1::Port5;
    /// Port 7/8
    #[cfg(feature = "port_7_8_2")]
    Port_7_8 @ 0x0014: port_7_8_2::Port78;
    /// Port 1/2
    #[cfg(feature = "port_1_2_2")]
    Port_1_2 @ 0x0020: port_1_2_2::Port12;
    /// Basic Timer
    #[cfg(feature = "basic_timer_1")]
    Basic_Timer @ 0x0040: basic_timer_1::BasicTimer;
    /// System Clock FLLPLUS
    #[cfg(feature = "system_clock_fllplus_5")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_5::SystemClockFLLPLUS;
    /// Supply Voltage Supervisor
    #[cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0056: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// Comparator A
    #[cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
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
    #[cfg(feature = "usci_b0__i2c_mode_2")]
    USCI_B0__I2C_Mode @ 0x0068: usci_b0__i2c_mode_2::USCI_B0I2CMode;
    /// LCD_A
    #[cfg(feature = "lcd_a_1")]
    LCD_A @ 0x0090: lcd_a_1::LCD_A;
    /// SD16_A3
    #[cfg(feature = "sd16_a3_1")]
    SD16_A3 @ 0x00b0: sd16_a3_1::SD16_A3;
    /// USCI_A1  UART Mode
    #[cfg(feature = "usci_a1__uart_mode_1")]
    USCI_A1__UART_Mode @ 0x00cd: usci_a1__uart_mode_1::USCI_A1UARTMode;
    /// USCI_A1  SPI Mode
    #[cfg(feature = "usci_a1__spi_mode_1")]
    USCI_A1__SPI_Mode @ 0x00d0: usci_a1__spi_mode_1::USCI_A1SPIMode;
    /// USCI_B1  SPI Mode
    #[cfg(feature = "usci_b1__spi_mode_1")]
    USCI_B1__SPI_Mode @ 0x00d8: usci_b1__spi_mode_1::USCI_B1SPIMode;
    /// USCI_B1  I2C Mode
    #[cfg(feature = "usci_b1__i2c_mode_1")]
    USCI_B1__I2C_Mode @ 0x00d8: usci_b1__i2c_mode_1::USCI_B1I2CMode;
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
    /// Multiplier  16 Bit Mode
    #[cfg(feature = "multiplier__16_bit_mode_1")]
    Multiplier__16_Bit_Mode @ 0x0130: multiplier__16_bit_mode_1::Multiplier16BitMode;
    /// Multiplier  32 Bit Mode
    #[cfg(feature = "multiplier__32_bit_mode_1")]
    Multiplier__32_Bit_Mode @ 0x0140: multiplier__32_bit_mode_1::Multiplier32BitMode;
}
