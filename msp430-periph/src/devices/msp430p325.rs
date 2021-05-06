//! MSP430P325
use crate::peripherals::*;

utils::device! {
    /// MSP430P325
    #[cfg_attr(not(feature = "MSP430P325-all"), non_exhaustive)]
    MSP430P325;
    /// Special Function
    #[cfg(feature = "special_function_4")]
    Special_Function @ 0x0000: special_function_4::SpecialFunction;
    /// Port 0
    #[cfg(feature = "port_0_1")]
    Port_0 @ 0x0010: port_0_1::Port0;
    /// LCD
    #[cfg(feature = "lcd_1")]
    LCD @ 0x0030: lcd_1::LCD;
    /// Basic Timer
    #[cfg(feature = "basic_timer_1")]
    Basic_Timer @ 0x0040: basic_timer_1::BasicTimer;
    /// 8BIT TIMER/COUNTER
    #[cfg(feature = "_8bit_timer_counter_1")]
    _8BIT_TIMER_COUNTER @ 0x0042: _8bit_timer_counter_1::_8BITTIMERCOUNTER;
    /// Timer/Port
    #[cfg(feature = "timer_port_1")]
    Timer_Port @ 0x004b: timer_port_1::TimerPort;
    /// System Clock
    #[cfg(feature = "system_clock_2")]
    System_Clock @ 0x0050: system_clock_2::SystemClock;
    /// EPROM
    #[cfg(feature = "eprom_1")]
    EPROM @ 0x0054: eprom_1::EPROM;
    /// ADC
    #[cfg(feature = "adc_1")]
    ADC @ 0x0110: adc_1::ADC;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
}
