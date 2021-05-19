//! MSP430FG4619
use crate::peripherals::*;

utils::device! {
    /// MSP430FG4619
    #[all:cfg_attr(not(feature = "MSP430FG4619-all"), non_exhaustive)]
    MSP430FG4619;
    /// Special Function
    #[all:cfg(feature = "special_function_7")]
    Special_Function @ 0x0000: special_function_7::SpecialFunction;
    /// Port 9
    #[all:cfg(feature = "port_x_2")]
    Port_9 @ 0x0008: port_x_2::Port;
    /// Port 10
    #[all:cfg(feature = "port_x_2")]
    Port_10 @ 0x0009: port_x_2::Port;
    /// Port B
    #[all:cfg(feature = "port_ab")]
    Port_B @ 0x0008: port_ab::Port;
    /// Port 3
    #[all:cfg(feature = "port_x_1")]
    Port_3 @ 0x0018: port_x_1::Port;
    /// Port 4
    #[all:cfg(feature = "port_x_1")]
    Port_4 @ 0x001c: port_x_1::Port;
    /// Port 1
    #[all:cfg(feature = "port_12_1")]
    Port_1 @ 0x0020: port_12_1::Port;
    /// Port 2
    #[all:cfg(feature = "port_12_1")]
    Port_2 @ 0x0028: port_12_1::Port;
    /// Port 5
    #[all:cfg(feature = "port_x_1")]
    Port_5 @ 0x0030: port_x_1::Port;
    /// Port 6
    #[all:cfg(feature = "port_x_1")]
    Port_6 @ 0x0034: port_x_1::Port;
    /// Port 7
    #[all:cfg(feature = "port_x_2")]
    Port_7 @ 0x0038: port_x_2::Port;
    /// Port 8
    #[all:cfg(feature = "port_x_2")]
    Port_8 @ 0x0039: port_x_2::Port;
    /// Port A
    #[all:cfg(feature = "port_ab")]
    Port_A @ 0x0038: port_ab::Port;
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
    /// ADC12
    #[all:cfg(feature = "adc12_1")]
    ADC12 @ 0x0080: adc12_1::ADC12;
    /// LCD_A
    #[all:cfg(feature = "lcd_a_1")]
    LCD_A @ 0x0090: lcd_a_1::LCD_A;
    /// Operational Amplifier
    #[all:cfg(feature = "operational_amplifier_1")]
    Operational_Amplifier @ 0x00c0: operational_amplifier_1::OperationalAmplifier;
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
    /// DAC12
    #[all:cfg(feature = "dac12_1")]
    DAC12 @ 0x01c0: dac12_1::DAC12;
}
