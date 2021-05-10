//! MSP430F423A
use crate::peripherals::*;

utils::device! {
    /// MSP430F423A
    #[all:cfg_attr(not(feature = "MSP430F423A-all"), non_exhaustive)]
    MSP430F423A;
    /// Special Function
    #[all:cfg(feature = "special_function_14")]
    Special_Function @ 0x0000: special_function_14::SpecialFunction;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// Basic Timer
    #[all:cfg(feature = "basic_timer_1")]
    Basic_Timer @ 0x0040: basic_timer_1::BasicTimer;
    /// System Clock FLLPLUS
    #[all:cfg(feature = "system_clock_fllplus_1")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_1::SystemClockFLLPLUS;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0056: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// USART 0  UART/SPI Mode
    #[all:cfg(feature = "usart_0__uart_spi_mode_1")]
    USART_0__UART_SPI_Mode @ 0x0070: usart_0__uart_spi_mode_1::USART0UARTSPIMode;
    /// LCD
    #[all:cfg(feature = "lcd_2")]
    LCD @ 0x0090: lcd_2::LCD;
    /// SD16
    #[all:cfg(feature = "sd16_1")]
    SD16 @ 0x00b0: sd16_1::SD16;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier_1")]
    Multiplier @ 0x0130: multiplier_1::Multiplier;
}
