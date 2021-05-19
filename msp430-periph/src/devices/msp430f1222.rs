//! MSP430F1222
use crate::peripherals::*;

utils::device! {
    /// MSP430F1222
    #[all:cfg_attr(not(feature = "MSP430F1222-all"), non_exhaustive)]
    MSP430F1222;
    /// Special Function
    #[all:cfg(feature = "special_function_8")]
    Special_Function @ 0x0000: special_function_8::SpecialFunction;
    /// Port 3
    #[all:cfg(feature = "port_3_1")]
    Port_3 @ 0x0018: port_3_1::Port;
    /// Port 1
    #[all:cfg(feature = "port_12_1")]
    Port_1 @ 0x0020: port_12_1::Port;
    /// Port 2
    #[all:cfg(feature = "port_12_1")]
    Port_2 @ 0x0028: port_12_1::Port;
    /// ADC10
    #[all:cfg(feature = "adc10_2")]
    ADC10 @ 0x0048: adc10_2::ADC10;
    /// System Clock
    #[all:cfg(feature = "system_clock_1")]
    System_Clock @ 0x0056: system_clock_1::SystemClock;
    /// USART 0  UART/SPI Mode
    #[all:cfg(feature = "usart")]
    USART_0 @ 0x0070: usart::USART;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
}
