//! MSP430L092
use crate::peripherals::*;

utils::device! {
    /// MSP430L092
    #[all:cfg_attr(not(feature = "MSP430L092-all"), non_exhaustive)]
    MSP430L092;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr__special_function_registers_2")]
    SFR__Special_Function_Registers @ 0x0100: sfr__special_function_registers_2::SFRSpecialFunctionRegisters;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CCS  Compact System Clock
    #[all:cfg(feature = "ccs__compact_system_clock_1")]
    CCS__Compact_System_Clock @ 0x0160: ccs__compact_system_clock_1::CCSCompactSystemClock;
    /// CSYS  Compact System Module
    #[all:cfg(feature = "csys__compact_system_module_1")]
    CSYS__Compact_System_Module @ 0x0180: csys__compact_system_module_1::CSYSCompactSystemModule;
    /// Analog Pool
    #[all:cfg(feature = "analog_pool_1")]
    Analog_Pool @ 0x01a0: analog_pool_1::AnalogPool;
    /// Port A
    #[all:cfg(feature = "port_a_2")]
    Port_A @ 0x0200: port_a_2::PortA;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_6")]
    Port_1_2 @ 0x0200: port_1_2_6::Port12;
    /// Timer0_A3
    #[all:cfg(feature = "timer0_a3_2")]
    Timer0_A3 @ 0x0340: timer0_a3_2::Timer0_A3;
    /// Timer1_A3
    #[all:cfg(feature = "timer1_a3_1")]
    Timer1_A3 @ 0x0380: timer1_a3_1::Timer1_A3;
}
