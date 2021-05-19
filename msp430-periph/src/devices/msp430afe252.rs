//! MSP430AFE252
use crate::peripherals::*;

utils::device! {
    /// MSP430AFE252
    #[all:cfg_attr(not(feature = "MSP430AFE252-all"), non_exhaustive)]
    MSP430AFE252;
    /// Special Function
    #[all:cfg(feature = "special_function_19")]
    Special_Function @ 0x0000: special_function_19::SpecialFunction;
    /// Port 1
    #[all:cfg(feature = "port_1_1")]
    Port_1 @ 0x0020: port_1_1::Port;
    /// Port 2
    #[all:cfg(feature = "port_2_1")]
    Port_2 @ 0x0028: port_2_1::Port;
    /// System Clock
    #[all:cfg(feature = "system_clock_3")]
    System_Clock @ 0x0053: system_clock_3::SystemClock;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0055: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// USART 0  UART/SPI Mode
    #[all:cfg(feature = "usart")]
    USART_0 @ 0x0070: usart::USART;
    /// SD24_A2
    #[all:cfg(feature = "sd24_a2_1")]
    SD24_A2 @ 0x00b0: sd24_a2_1::SD24_A2;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_3")]
    Flash @ 0x0128: flash_3::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_3")]
    Calibration_Data @ 0x10fa: calibration_data_3::CalibrationData;
}
