//! MSP430F47163
use crate::peripherals::*;

utils::device! {
    /// MSP430F47163
    #[all:cfg_attr(not(feature = "MSP430F47163-all"), non_exhaustive)]
    MSP430F47163;
    /// Special Function
    #[all:cfg(feature = "special_function_16")]
    Special_Function @ 0x0000: special_function_16::SpecialFunction;
    /// Port 9/10
    #[all:cfg(feature = "port_9_10_2")]
    Port_9_10 @ 0x0008: port_9_10_2::Port910;
    /// Port 3/4
    #[all:cfg(feature = "port_3_4_3")]
    Port_3_4 @ 0x0010: port_3_4_3::Port34;
    /// Port 5
    #[all:cfg(feature = "port_5_1")]
    Port_5 @ 0x0012: port_5_1::Port5;
    /// Port 7/8
    #[all:cfg(feature = "port_7_8_2")]
    Port_7_8 @ 0x0014: port_7_8_2::Port78;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_2")]
    Port_1_2 @ 0x0020: port_1_2_2::Port12;
    /// Basic Timer / RTC
    #[all:cfg(feature = "basic_timer_rtc")]
    Basic_Timer_RTC @ 0x0040: basic_timer_rtc::BasicTimerRTC;
    /// System Clock FLLPLUS
    #[all:cfg(feature = "system_clock_fllplus_5")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_5::SystemClockFLLPLUS;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0056: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
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
    #[all:cfg(feature = "usci_b_i2c_2")]
    USCI_B0_I2C @ 0x0068: usci_b_i2c_2::USCI_B_I2C;
    /// LCD_A
    #[all:cfg(feature = "lcd_a_1")]
    LCD_A @ 0x0090: lcd_a_1::LCD_A;
    /// SD16_A3
    #[all:cfg(feature = "sd16_a3_1")]
    SD16_A3 @ 0x00b0: sd16_a3_1::SD16_A3;
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
    /// Timer B3
    #[all:cfg(feature = "timer_b3_1")]
    Timer_B3 @ 0x011e: timer_b3_1::TimerB3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// DMA
    #[all:cfg(feature = "dma_4")]
    DMA @ 0x0122: dma_4::DMA;
    /// Flash
    #[all:cfg(feature = "flash_5")]
    Flash @ 0x0128: flash_5::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// Multiplier  16 Bit Mode
    #[all:cfg(feature = "multiplier_16")]
    Multiplier_16 @ 0x0130: multiplier_16::Multiplier16;
    /// Multiplier  32 Bit Mode
    #[all:cfg(feature = "multiplier_32")]
    Multiplier_32 @ 0x0140: multiplier_32::Multiplier32;
}
