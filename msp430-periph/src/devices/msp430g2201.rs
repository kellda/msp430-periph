//! MSP430G2201
use crate::peripherals::*;

utils::device! {
    /// MSP430G2201
    #[all:cfg_attr(not(feature = "MSP430G2201-all"), non_exhaustive)]
    MSP430G2201;
    /// Special Function
    #[all:cfg(feature = "special_function_10")]
    Special_Function @ 0x0000: special_function_10::SpecialFunction;
    /// Port 1
    #[all:cfg(feature = "port_12_2")]
    Port_1 @ 0x0020: port_12_2::Port;
    /// Port 2
    #[all:cfg(feature = "port_12_2")]
    Port_2 @ 0x0028: port_12_2::Port;
    /// System Clock
    #[all:cfg(feature = "system_clock_3")]
    System_Clock @ 0x0053: system_clock_3::SystemClock;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_3")]
    Flash @ 0x0128: flash_3::Flash;
    /// Timer A2
    #[all:cfg(feature = "timer_a2_3")]
    Timer_A2 @ 0x012e: timer_a2_3::TimerA2;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_2")]
    Calibration_Data @ 0x10fe: calibration_data_2::CalibrationData;
}
