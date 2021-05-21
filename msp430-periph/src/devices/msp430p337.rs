//! MSP430P337
use crate::peripherals::*;

utils::device! {
    /// MSP430P337
    #[all:cfg_attr(not(feature = "msp430p337-all"), non_exhaustive)]
    MSP430P337;
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
    #[all:cfg(feature = "timer_a5_3")]
    Timer_A5 @ 0x012e: timer_a5_3::TimerA5;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE0 Port 0 Bits 2-7 [Lowest Priority]
    PORT0 = 0,
    /// 0xFFE2 Basic Timer
    BASICTIMER = 1,
    /// 0xFFE4 Port 1
    PORT1 = 2,
    /// 0xFFE6 Port 2
    PORT2 = 3,
    /// 0xFFE8 Timer/Port
    TIMERPORT = 4,
    /// 0xFFEC USART Transmit
    USARTTX = 6,
    /// 0xFFEE USART Receive
    USARTRX = 7,
    /// 0xFFF0 Timer A CC1-4, TA
    TIMERA1 = 8,
    /// 0xFFF2 Timer A CC0
    TIMERA0 = 9,
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
    fn PORT1();
    fn PORT2();
    fn TIMERPORT();
    fn USARTTX();
    fn USARTRX();
    fn TIMERA1();
    fn TIMERA0();
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
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TIMERPORT },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: USARTTX },
    crate::Vector { _handler: USARTRX },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: IO1 },
    crate::Vector { _handler: IO0 },
    crate::Vector { _handler: NMI },
];
