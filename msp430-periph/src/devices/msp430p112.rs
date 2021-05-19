//! MSP430P112
use crate::peripherals::*;

utils::device! {
    /// MSP430P112
    #[all:cfg_attr(not(feature = "MSP430P112-all"), non_exhaustive)]
    MSP430P112;
    /// Special Function
    #[all:cfg(feature = "special_function_1")]
    Special_Function @ 0x0000: special_function_1::SpecialFunction;
    /// Port 1
    #[all:cfg(feature = "port_12_1")]
    Port_1 @ 0x0020: port_12_1::Port;
    /// Port 2
    #[all:cfg(feature = "port_12_1")]
    Port_2 @ 0x0028: port_12_1::Port;
    /// EPROM
    #[all:cfg(feature = "eprom")]
    EPROM @ 0x0054: eprom::EPROM;
    /// System Clock
    #[all:cfg(feature = "system_clock_1")]
    System_Clock @ 0x0056: system_clock_1::SystemClock;
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
