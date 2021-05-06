//! MSP430P112
use crate::peripherals::*;

utils::device! {
    /// MSP430P112
    #[cfg_attr(not(feature = "MSP430P112-all"), non_exhaustive)]
    MSP430P112;
    /// Special Function
    #[cfg(feature = "special_function_1")]
    Special_Function @ 0x0000: special_function_1::SpecialFunction;
    /// Port 1/2
    #[cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// EPROM
    #[cfg(feature = "eprom_1")]
    EPROM @ 0x0054: eprom_1::EPROM;
    /// System Clock
    #[cfg(feature = "system_clock_1")]
    System_Clock @ 0x0056: system_clock_1::SystemClock;
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
