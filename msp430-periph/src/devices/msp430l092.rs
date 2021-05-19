//! MSP430L092
use crate::peripherals::*;

utils::device! {
    /// MSP430L092
    #[all:cfg_attr(not(feature = "MSP430L092-all"), non_exhaustive)]
    MSP430L092;
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
    #[all:cfg(feature = "portw_2i")]
    Port_A @ 0x0200: portw_2i::Port;
    /// Port 1
    #[all:cfg(feature = "portb_2i1")]
    Port_1 @ 0x0200: portb_2i1::Port;
    /// Port 2
    #[all:cfg(feature = "portb_2i2")]
    Port_2 @ 0x0201: portb_2i2::Port;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer0_A3 @ 0x0340: timer_a3_1::TimerA3;
    /// Timer1_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer1_A3 @ 0x0380: timer_a3_1::TimerA3;
}
