//! MSP430F1111A
use crate::peripherals::*;

utils::device! {
    /// MSP430F1111A
    #[cfg_attr(not(feature = "MSP430F1111A-all"), non_exhaustive)]
    MSP430F1111A;
    /// Special Function
    #[cfg(feature = "special_function_1")]
    Special_Function @ 0x0000: special_function_1::SpecialFunction;
    /// Port 1/2
    #[cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// System Clock
    #[cfg(feature = "system_clock_1")]
    System_Clock @ 0x0056: system_clock_1::SystemClock;
    /// Comparator A
    #[cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
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
