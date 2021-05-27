//! MSP430C315
use crate::peripherals::*;

utils::device! {
    /// MSP430C315
    #[all:cfg_attr(not(feature = "msp430c315-all"), non_exhaustive)]
    MSP430C315;
    /// Special Function
    #[all:cfg(feature = "special_function_3")]
    Special_Function @ 0x0000: special_function_3::SpecialFunction;
    /// Port 0
    #[all:cfg(feature = "port_0_1")]
    Port_0 @ 0x0010: port_0_1::Port;
    /// LCD
    #[all:cfg(feature = "lcd_1")]
    LCD @ 0x0030: lcd_1::LCD;
    /// Basic Timer
    #[all:cfg(feature = "basic_timer")]
    Basic_Timer @ 0x0040: basic_timer::BasicTimer;
    /// 8BIT TIMER/COUNTER
    #[all:cfg(feature = "timer_counter")]
    TC @ 0x0042: timer_counter::TC;
    /// Timer/Port
    #[all:cfg(feature = "timer_port")]
    Timer_Port @ 0x004b: timer_port::TimerPort;
    /// System Clock
    #[all:cfg(feature = "system_clock_2")]
    System_Clock @ 0x0050: system_clock_2::SystemClock;
    /// EPROM
    #[all:cfg(feature = "eprom")]
    EPROM @ 0x0054: eprom::EPROM;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE0 Port 0 Bits 2-7 [Lowest Priority]
    PORT0 = 0,
    /// 0xFFE2 Basic Timer
    BASICTIMER = 1,
    /// 0xFFE8 Timer/Port
    TIMERPORT = 5,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF8 Dedicated IO (P0.1)
    IO1 = 12,
    /// 0xFFFA Dedicated IO (P0.0)
    IO0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT0();
    fn BASICTIMER();
    fn TIMERPORT();
    fn WDT();
    fn IO1();
    fn IO0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _handler: PORT0 },
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: TIMERPORT },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: IO1 },
    crate::Vector { _handler: IO0 },
    crate::Vector { _handler: NMI },
];
