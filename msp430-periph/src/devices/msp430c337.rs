//! MSP430C337
use crate::peripherals::*;

utils::device! {
    /// MSP430C337
    #[all:cfg_attr(not(feature = "MSP430C337-all"), non_exhaustive)]
    MSP430C337;
    /// Special Function
    #[all:cfg(feature = "special_function_5")]
    Special_Function @ 0x0000: special_function_5::SpecialFunction;
    /// Port 0
    #[all:cfg(feature = "port_0_1")]
    Port_0 @ 0x0010: port_0_1::Port;
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
    /// LCD
    #[all:cfg(feature = "lcd_1")]
    LCD @ 0x0030: lcd_1::LCD;
    /// Basic Timer
    #[all:cfg(feature = "basic_timer")]
    Basic_Timer @ 0x0040: basic_timer::BasicTimer;
    /// 8BIT TIMER/COUNTER
    #[all:cfg(feature = "_8bit_timer_counter_1")]
    _8BIT_TIMER_COUNTER @ 0x0042: _8bit_timer_counter_1::_8BITTIMERCOUNTER;
    /// Timer/Port
    #[all:cfg(feature = "timer_port")]
    Timer_Port @ 0x004b: timer_port::TimerPort;
    /// System Clock
    #[all:cfg(feature = "system_clock_2")]
    System_Clock @ 0x0050: system_clock_2::SystemClock;
    /// EPROM
    #[all:cfg(feature = "eprom")]
    EPROM @ 0x0054: eprom::EPROM;
    /// UART
    #[all:cfg(feature = "usart")]
    UART @ 0x0070: usart::USART;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Timer A5
    #[all:cfg(feature = "timer_a5_1")]
    Timer_A5 @ 0x012e: timer_a5_1::TimerA5;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
}
