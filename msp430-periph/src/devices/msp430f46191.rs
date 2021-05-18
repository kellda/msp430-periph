//! MSP430F46191
use crate::peripherals::*;

utils::device! {
    /// MSP430F46191
    #[all:cfg_attr(not(feature = "MSP430F46191-all"), non_exhaustive)]
    MSP430F46191;
    /// Special Function
    #[all:cfg(feature = "special_function_7")]
    Special_Function @ 0x0000: special_function_7::SpecialFunction;
    /// Port 9/10
    #[all:cfg(feature = "port_9_10_1")]
    Port_9_10 @ 0x0008: port_9_10_1::Port910;
    /// Port 3/4
    #[all:cfg(feature = "port_3_4_1")]
    Port_3_4 @ 0x0018: port_3_4_1::Port34;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// Port 5/6
    #[all:cfg(feature = "port_5_6_1")]
    Port_5_6 @ 0x0030: port_5_6_1::Port56;
    /// Port 7/8
    #[all:cfg(feature = "port_7_8_1")]
    Port_7_8 @ 0x0038: port_7_8_1::Port78;
    /// Basic Timer / RTC
    #[all:cfg(feature = "basic_timer_rtc")]
    Basic_Timer_RTC @ 0x0040: basic_timer_rtc::BasicTimerRTC;
    /// System Clock FLLPLUS
    #[all:cfg(feature = "system_clock_fllplus_2")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_2::SystemClockFLLPLUS;
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
    #[all:cfg(feature = "usci_b_i2c_1")]
    USCI_B0_I2C @ 0x0068: usci_b_i2c_1::USCI_B_I2C;
    /// USART 1
    #[all:cfg(feature = "usart")]
    USART_1 @ 0x0078: usart::USART;
    /// LCD_A
    #[all:cfg(feature = "lcd_a_1")]
    LCD_A @ 0x0090: lcd_a_1::LCD_A;
    /// Timer B7
    #[all:cfg(feature = "timer_b7_1")]
    Timer_B7 @ 0x011e: timer_b7_1::TimerB7;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// DMA
    #[all:cfg(feature = "dma_1")]
    DMA @ 0x0122: dma_1::DMA;
    /// Flash
    #[all:cfg(feature = "flash_2")]
    Flash @ 0x0128: flash_2::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
}
