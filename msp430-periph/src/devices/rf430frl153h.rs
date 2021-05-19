//! RF430FRL153H
use crate::peripherals::*;

utils::device! {
    /// RF430FRL153H
    #[all:cfg_attr(not(feature = "RF430FRL153H-all"), non_exhaustive)]
    RF430FRL153H;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_3")]
    SFR @ 0x0100: sfr_3::SFR;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_6")]
    PMM @ 0x0120: pmm_6::PMM;
    /// CRC16
    #[all:cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CCS  Compact System Clock
    #[all:cfg(feature = "ccs")]
    CCS @ 0x0160: ccs::CCS;
    /// CSYS  Compact System Module
    #[all:cfg(feature = "csys_2")]
    CSYS @ 0x0180: csys_2::CSYS;
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
    /// SD14 Module
    #[all:cfg(feature = "sd14_module_1")]
    SD14_Module @ 0x0700: sd14_module_1::SD14Module;
    /// RF13M Module
    #[all:cfg(feature = "rf13m")]
    RF13M @ 0x0800: rf13m::RF13M;
}
