//! MSP430G2402
use crate::peripherals::*;

utils::device! {
    /// MSP430G2402
    #[all:cfg_attr(not(feature = "MSP430G2402-all"), non_exhaustive)]
    MSP430G2402;
    /// Special Function
    #[all:cfg(feature = "special_function_10")]
    Special_Function @ 0x0000: special_function_10::SpecialFunction;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_3")]
    Port_1_2 @ 0x0020: port_1_2_3::Port12;
    /// System Clock
    #[all:cfg(feature = "system_clock_3")]
    System_Clock @ 0x0053: system_clock_3::SystemClock;
    /// USI
    #[all:cfg(feature = "usi_1")]
    USI @ 0x0078: usi_1::USI;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_3")]
    Flash @ 0x0128: flash_3::Flash;
    /// Timer0_A3
    #[all:cfg(feature = "timer0_a3_1")]
    Timer0_A3 @ 0x012e: timer0_a3_1::Timer0_A3;
    /// TLV Calibration Data
    #[all:cfg(feature = "tlv_calibration_data_1")]
    TLV_Calibration_Data @ 0x10c0: tlv_calibration_data_1::TLVCalibrationData;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_1")]
    Calibration_Data @ 0x10f8: calibration_data_1::CalibrationData;
}
