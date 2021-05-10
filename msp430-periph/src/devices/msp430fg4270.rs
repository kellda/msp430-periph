//! MSP430FG4270
use crate::peripherals::*;

utils::device! {
    /// MSP430FG4270
    #[all:cfg_attr(not(feature = "MSP430FG4270-all"), non_exhaustive)]
    MSP430FG4270;
    /// Special Function
    #[all:cfg(feature = "special_function_17")]
    Special_Function @ 0x0000: special_function_17::SpecialFunction;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// Port 5/6
    #[all:cfg(feature = "port_5_6_1")]
    Port_5_6 @ 0x0030: port_5_6_1::Port56;
    /// Basic Timer
    #[all:cfg(feature = "basic_timer_1")]
    Basic_Timer @ 0x0040: basic_timer_1::BasicTimer;
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
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// DAC12
    #[all:cfg(feature = "dac12_3")]
    DAC12 @ 0x01c0: dac12_3::DAC12;
}
