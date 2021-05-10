//! MSP430F168
use crate::peripherals::*;

utils::device! {
    /// MSP430F168
    #[all:cfg_attr(not(feature = "MSP430F168-all"), non_exhaustive)]
    MSP430F168;
    /// Special Function
    #[all:cfg(feature = "special_function_9")]
    Special_Function @ 0x0000: special_function_9::SpecialFunction;
    /// Port 3/4
    #[all:cfg(feature = "port_3_4_1")]
    Port_3_4 @ 0x0018: port_3_4_1::Port34;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// Port 5/6
    #[all:cfg(feature = "port_5_6_1")]
    Port_5_6 @ 0x0030: port_5_6_1::Port56;
    /// USART 0  I2C Mode
    #[all:cfg(feature = "usart_0__i2c_mode_1")]
    USART_0__I2C_Mode @ 0x0050: usart_0__i2c_mode_1::USART0I2CMode;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0055: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// System Clock
    #[all:cfg(feature = "system_clock_1")]
    System_Clock @ 0x0056: system_clock_1::SystemClock;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
    /// USART 0  UART/SPI Mode
    #[all:cfg(feature = "usart_0__uart_spi_mode_1")]
    USART_0__UART_SPI_Mode @ 0x0070: usart_0__uart_spi_mode_1::USART0UARTSPIMode;
    /// USART 1
    #[all:cfg(feature = "usart_1_1")]
    USART_1 @ 0x0078: usart_1_1::USART1;
    /// ADC12
    #[all:cfg(feature = "adc12_1")]
    ADC12 @ 0x0080: adc12_1::ADC12;
    /// Timer B7
    #[all:cfg(feature = "timer_b7_1")]
    Timer_B7 @ 0x011e: timer_b7_1::TimerB7;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// DMA
    #[all:cfg(feature = "dma_2")]
    DMA @ 0x0122: dma_2::DMA;
    /// Flash
    #[all:cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier_1")]
    Multiplier @ 0x0130: multiplier_1::Multiplier;
    /// DAC12
    #[all:cfg(feature = "dac12_2")]
    DAC12 @ 0x01c0: dac12_2::DAC12;
}
