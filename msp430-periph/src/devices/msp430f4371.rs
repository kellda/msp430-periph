//! MSP430F4371
use crate::peripherals::*;

utils::device! {
    /// MSP430F4371
    #[all:cfg_attr(not(feature = "MSP430F4371-all"), non_exhaustive)]
    MSP430F4371;
    /// Special Function
    #[all:cfg(feature = "special_function_14")]
    Special_Function @ 0x0000: special_function_14::SpecialFunction;
    /// Port 3
    #[all:cfg(feature = "port_x_1")]
    Port_3 @ 0x0018: port_x_1::Port;
    /// Port 4
    #[all:cfg(feature = "port_x_1")]
    Port_4 @ 0x001c: port_x_1::Port;
    /// Port 1
    #[all:cfg(feature = "port_12_1")]
    Port_1 @ 0x0020: port_12_1::Port;
    /// Port 2
    #[all:cfg(feature = "port_12_1")]
    Port_2 @ 0x0028: port_12_1::Port;
    /// Port 5
    #[all:cfg(feature = "port_x_1")]
    Port_5 @ 0x0030: port_x_1::Port;
    /// Port 6
    #[all:cfg(feature = "port_x_1")]
    Port_6 @ 0x0034: port_x_1::Port;
    /// Basic Timer
    #[all:cfg(feature = "basic_timer")]
    Basic_Timer @ 0x0040: basic_timer::BasicTimer;
    /// System Clock FLLPLUS
    #[all:cfg(feature = "system_clock_fllplus_4")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_4::SystemClockFLLPLUS;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0056: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
    /// USART 0  UART/SPI Mode
    #[all:cfg(feature = "usart")]
    USART_0 @ 0x0070: usart::USART;
    /// LCD
    #[all:cfg(feature = "lcd_2")]
    LCD @ 0x0090: lcd_2::LCD;
    /// Timer B3
    #[all:cfg(feature = "timer_b3_1")]
    Timer_B3 @ 0x011e: timer_b3_1::TimerB3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
}
