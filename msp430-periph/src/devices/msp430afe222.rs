//! MSP430AFE222
use crate::peripherals::*;

utils::device! {
    /// MSP430AFE222
    #[all:cfg_attr(not(feature = "MSP430AFE222-all"), non_exhaustive)]
    MSP430AFE222;
    /// Special Function
    #[all:cfg(feature = "special_function_19")]
    Special_Function @ 0x0000: special_function_19::SpecialFunction;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_3")]
    Port_1_2 @ 0x0020: port_1_2_3::Port12;
    /// System Clock
    #[all:cfg(feature = "system_clock_3")]
    System_Clock @ 0x0053: system_clock_3::SystemClock;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0055: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// USART 0  UART/SPI Mode
    #[all:cfg(feature = "usart_0__uart_spi_mode_1")]
    USART_0__UART_SPI_Mode @ 0x0070: usart_0__uart_spi_mode_1::USART0UARTSPIMode;
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
    #[all:cfg(feature = "multiplier_1")]
    Multiplier @ 0x0130: multiplier_1::Multiplier;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_3")]
    Calibration_Data @ 0x10fa: calibration_data_3::CalibrationData;
}
