//! MSP430C1331
use crate::peripherals::*;

utils::device! {
    /// MSP430C1331
    #[all:cfg_attr(not(feature = "MSP430C1331-all"), non_exhaustive)]
    MSP430C1331;
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
    /// System Clock
    #[all:cfg(feature = "system_clock_1")]
    System_Clock @ 0x0056: system_clock_1::SystemClock;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
    /// USART 0  UART/SPI Mode
    #[all:cfg(feature = "usart")]
    USART_0 @ 0x0070: usart::USART;
    /// Timer B3
    #[all:cfg(feature = "timer_b3_2")]
    Timer_B3 @ 0x011e: timer_b3_2::TimerB3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer_A3 @ 0x012e: timer_a3_3::TimerA3;
}
