//! MSP430G2352
use crate::peripherals::*;

utils::device! {
    /// MSP430G2352
    #[all:cfg_attr(not(feature = "MSP430G2352-all"), non_exhaustive)]
    MSP430G2352;
    /// Special Function
    #[all:cfg(feature = "special_function_10")]
    Special_Function @ 0x0000: special_function_10::SpecialFunction;
    /// Port 1
    #[all:cfg(feature = "port_1_1")]
    Port_1 @ 0x0020: port_1_1::Port;
    /// Port 2
    #[all:cfg(feature = "port_2_1")]
    Port_2 @ 0x0028: port_2_1::Port;
    /// ADC10
    #[all:cfg(feature = "adc10_2")]
    ADC10 @ 0x0048: adc10_2::ADC10;
    /// System Clock
    #[all:cfg(feature = "system_clock_3")]
    System_Clock @ 0x0053: system_clock_3::SystemClock;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_2")]
    Comparator_A @ 0x0059: comparator_a_2::ComparatorA;
    /// USI
    #[all:cfg(feature = "usi")]
    USI @ 0x0078: usi::USI;
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
