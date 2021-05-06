//! MSP430F1232
use crate::peripherals::*;

utils::device! {
    /// MSP430F1232
    #[cfg_attr(not(feature = "MSP430F1232-all"), non_exhaustive)]
    MSP430F1232;
    /// Special Function
    #[cfg(feature = "special_function_8")]
    Special_Function @ 0x0000: special_function_8::SpecialFunction;
    /// Port 3
    #[cfg(feature = "port_3_1")]
    Port_3 @ 0x0018: port_3_1::Port3;
    /// Port 1/2
    #[cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// ADC10
    #[cfg(feature = "adc10_1")]
    ADC10 @ 0x0048: adc10_1::ADC10;
    /// System Clock
    #[cfg(feature = "system_clock_1")]
    System_Clock @ 0x0056: system_clock_1::SystemClock;
    /// USART 0  UART/SPI Mode
    #[cfg(feature = "usart_0__uart_spi_mode_1")]
    USART_0__UART_SPI_Mode @ 0x0070: usart_0__uart_spi_mode_1::USART0UARTSPIMode;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
}
