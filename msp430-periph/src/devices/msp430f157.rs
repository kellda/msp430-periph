//! MSP430F157
use crate::peripherals::*;

utils::device! {
    /// MSP430F157
    #[all:cfg_attr(not(feature = "MSP430F157-all"), non_exhaustive)]
    MSP430F157;
    /// Special Function
    #[all:cfg(feature = "special_function_2")]
    Special_Function @ 0x0000: special_function_2::SpecialFunction;
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
    /// USART 0  I2C Mode
    #[all:cfg(feature = "usart_i2c")]
    USART_0_I2C @ 0x0050: usart_i2c::USART_I2C;
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
    #[all:cfg(feature = "usart")]
    USART_0 @ 0x0070: usart::USART;
    /// ADC12
    #[all:cfg(feature = "adc12_1")]
    ADC12 @ 0x0080: adc12_1::ADC12;
    /// Timer B3
    #[all:cfg(feature = "timer_b3_1")]
    Timer_B3 @ 0x011e: timer_b3_1::TimerB3;
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
    /// DAC12
    #[all:cfg(feature = "dac12_2")]
    DAC12 @ 0x01c0: dac12_2::DAC12;
}
