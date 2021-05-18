//! MSP430C091
use crate::peripherals::*;

utils::device! {
    /// MSP430C091
    #[all:cfg_attr(not(feature = "MSP430C091-all"), non_exhaustive)]
    MSP430C091;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_2")]
    SFR @ 0x0100: sfr_2::SFR;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CCS  Compact System Clock
    #[all:cfg(feature = "ccs")]
    CCS @ 0x0160: ccs::CCS;
    /// CSYS  Compact System Module
    #[all:cfg(feature = "csys_1")]
    CSYS @ 0x0180: csys_1::CSYS;
    /// Analog Pool
    #[all:cfg(feature = "analog_pool")]
    Analog_Pool @ 0x01a0: analog_pool::AnalogPool;
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
