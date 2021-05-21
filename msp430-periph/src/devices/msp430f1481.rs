//! MSP430F1481
use crate::peripherals::*;

utils::device! {
    /// MSP430F1481
    #[all:cfg_attr(not(feature = "msp430f1481-all"), non_exhaustive)]
    MSP430F1481;
    /// Special Function
    #[all:cfg(feature = "special_function_9")]
    Special_Function @ 0x0000: special_function_9::SpecialFunction;
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
    /// System Clock
    #[all:cfg(feature = "system_clock_1")]
    System_Clock @ 0x0056: system_clock_1::SystemClock;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
    /// USART 0  UART/SPI Mode
    #[all:cfg(feature = "usart")]
    USART_0 @ 0x0070: usart::USART;
    /// USART 1
    #[all:cfg(feature = "usart")]
    USART_1 @ 0x0078: usart::USART;
    /// Timer B7
    #[all:cfg(feature = "timer_b7_1")]
    Timer_B7 @ 0x011e: timer_b7_1::TimerB7;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE2 Port 2
    PORT2 = 1,
    /// 0xFFE4 USART 1 Transmit
    USART1TX = 2,
    /// 0xFFE6 USART 1 Receive
    USART1RX = 3,
    /// 0xFFE8 Port 1
    PORT1 = 4,
    /// 0xFFEA Timer A CC1-2, TA
    TIMERA1 = 5,
    /// 0xFFEC Timer A CC0
    TIMERA0 = 6,
    /// 0xFFF0 USART 0 Transmit
    USART0TX = 8,
    /// 0xFFF2 USART 0 Receive
    USART0RX = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Comparator A
    COMPARATORA = 11,
    /// 0xFFF8 Timer B CC1-6, TB
    TIMERB1 = 12,
    /// 0xFFFA Timer B CC0
    TIMERB0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT2();
    fn USART1TX();
    fn USART1RX();
    fn PORT1();
    fn TIMERA1();
    fn TIMERA0();
    fn USART0TX();
    fn USART0RX();
    fn WDT();
    fn COMPARATORA();
    fn TIMERB1();
    fn TIMERB0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: USART1TX },
    crate::Vector { _handler: USART1RX },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: USART0TX },
    crate::Vector { _handler: USART0RX },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMERB1 },
    crate::Vector { _handler: TIMERB0 },
    crate::Vector { _handler: NMI },
];
