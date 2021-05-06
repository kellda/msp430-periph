//! MSP430FG438
use crate::peripherals::*;

utils::device! {
    /// MSP430FG438
    #[cfg_attr(not(feature = "MSP430FG438-all"), non_exhaustive)]
    MSP430FG438;
    /// Special Function
    #[cfg(feature = "special_function_14")]
    Special_Function @ 0x0000: special_function_14::SpecialFunction;
    /// Port 3/4
    #[cfg(feature = "port_3_4_1")]
    Port_3_4 @ 0x0018: port_3_4_1::Port34;
    /// Port 1/2
    #[cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// Port 5/6
    #[cfg(feature = "port_5_6_1")]
    Port_5_6 @ 0x0030: port_5_6_1::Port56;
    /// Basic Timer
    #[cfg(feature = "basic_timer_1")]
    Basic_Timer @ 0x0040: basic_timer_1::BasicTimer;
    /// System Clock FLLPLUS
    #[cfg(feature = "system_clock_fllplus_4")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_4::SystemClockFLLPLUS;
    /// Supply Voltage Supervisor
    #[cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0056: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// Comparator A
    #[cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
    /// USART 0  UART/SPI Mode
    #[cfg(feature = "usart_0__uart_spi_mode_1")]
    USART_0__UART_SPI_Mode @ 0x0070: usart_0__uart_spi_mode_1::USART0UARTSPIMode;
    /// ADC12
    #[cfg(feature = "adc12_1")]
    ADC12 @ 0x0080: adc12_1::ADC12;
    /// LCD
    #[cfg(feature = "lcd_2")]
    LCD @ 0x0090: lcd_2::LCD;
    /// Operational Amplifier
    #[cfg(feature = "operational_amplifier_1")]
    Operational_Amplifier @ 0x00c0: operational_amplifier_1::OperationalAmplifier;
    /// Timer B3
    #[cfg(feature = "timer_b3_1")]
    Timer_B3 @ 0x011e: timer_b3_1::TimerB3;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// DMA
    #[cfg(feature = "dma_5")]
    DMA @ 0x0122: dma_5::DMA;
    /// Flash
    #[cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// DAC12
    #[cfg(feature = "dac12_1")]
    DAC12 @ 0x01c0: dac12_1::DAC12;
}
