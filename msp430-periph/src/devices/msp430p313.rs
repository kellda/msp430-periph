//! MSP430P313
use crate::peripherals::*;

utils::device! {
    /// MSP430P313
    #[all:cfg_attr(not(feature = "MSP430P313-all"), non_exhaustive)]
    MSP430P313;
    /// Special Function
    #[all:cfg(feature = "special_function_3")]
    Special_Function @ 0x0000: special_function_3::SpecialFunction;
    /// Port 0
    #[all:cfg(feature = "port_0_1")]
    Port_0 @ 0x0010: port_0_1::Port0;
    /// LCD
    #[all:cfg(feature = "lcd_1")]
    LCD @ 0x0030: lcd_1::LCD;
    /// Basic Timer
    #[all:cfg(feature = "basic_timer_1")]
    Basic_Timer @ 0x0040: basic_timer_1::BasicTimer;
    /// 8BIT TIMER/COUNTER
    #[all:cfg(feature = "_8bit_timer_counter_1")]
    _8BIT_TIMER_COUNTER @ 0x0042: _8bit_timer_counter_1::_8BITTIMERCOUNTER;
    /// Timer/Port
    #[all:cfg(feature = "timer_port_1")]
    Timer_Port @ 0x004b: timer_port_1::TimerPort;
    /// System Clock
    #[all:cfg(feature = "system_clock_2")]
    System_Clock @ 0x0050: system_clock_2::SystemClock;
    /// EPROM
    #[all:cfg(feature = "eprom_1")]
    EPROM @ 0x0054: eprom_1::EPROM;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
}
