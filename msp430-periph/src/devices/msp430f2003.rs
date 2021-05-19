//! MSP430F2003
use crate::peripherals::*;

utils::device! {
    /// MSP430F2003
    #[all:cfg_attr(not(feature = "MSP430F2003-all"), non_exhaustive)]
    MSP430F2003;
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
    /// USI
    #[all:cfg(feature = "usi")]
    USI @ 0x0078: usi::USI;
    /// SD16_A1
    #[all:cfg(feature = "sd16_a1_1")]
    SD16_A1 @ 0x00b0: sd16_a1_1::SD16_A1;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_3")]
    Flash @ 0x0128: flash_3::Flash;
    /// Timer A2
    #[all:cfg(feature = "timer_a2_1")]
    Timer_A2 @ 0x012e: timer_a2_1::TimerA2;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_1")]
    Calibration_Data @ 0x10f8: calibration_data_1::CalibrationData;
}
