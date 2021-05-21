//! MSP430FG4260
use crate::peripherals::*;

utils::device! {
    /// MSP430FG4260
    #[all:cfg_attr(not(feature = "msp430fg4260-all"), non_exhaustive)]
    MSP430FG4260;
    /// Special Function
    #[all:cfg(feature = "special_function_17")]
    Special_Function @ 0x0000: special_function_17::SpecialFunction;
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
    /// Basic Timer
    #[all:cfg(feature = "basic_timer")]
    Basic_Timer @ 0x0040: basic_timer::BasicTimer;
    /// System Clock FLLPLUS
    #[all:cfg(feature = "system_clock_fllplus_1")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_1::SystemClockFLLPLUS;
    /// LCD_A
    #[all:cfg(feature = "lcd_a_1")]
    LCD_A @ 0x0090: lcd_a_1::LCD_A;
    /// SD16_A1
    #[all:cfg(feature = "sd16_a1_3")]
    SD16_A1 @ 0x00b0: sd16_a1_3::SD16_A1;
    /// Operational Amplifier
    #[all:cfg(feature = "operational_amplifier_3")]
    Operational_Amplifier @ 0x00c0: operational_amplifier_3::OperationalAmplifier;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// DAC12
    #[all:cfg(feature = "dac12_3")]
    DAC12 @ 0x01c0: dac12_3::DAC12;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE0 Basic Timer
    BASICTIMER = 0,
    /// 0xFFE2 Port 2
    PORT2 = 1,
    /// 0xFFE6 DAC 12
    DAC12 = 3,
    /// 0xFFE8 Port 1
    PORT1 = 4,
    /// 0xFFEA Timer A CC1-2, TA
    TIMERA1 = 5,
    /// 0xFFEC Timer A CC0
    TIMERA0 = 6,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF8 Sigma Delta ADC
    SD16 = 12,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn BASICTIMER();
    fn PORT2();
    fn DAC12();
    fn PORT1();
    fn TIMERA1();
    fn TIMERA0();
    fn WDT();
    fn SD16();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: DAC12 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: SD16 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: NMI },
];
