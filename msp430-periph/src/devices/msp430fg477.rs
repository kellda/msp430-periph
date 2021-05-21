//! MSP430FG477
use crate::peripherals::*;

utils::device! {
    /// MSP430FG477
    #[all:cfg_attr(not(feature = "msp430fg477-all"), non_exhaustive)]
    MSP430FG477;
    /// Special Function
    #[all:cfg(feature = "special_function_18")]
    Special_Function @ 0x0000: special_function_18::SpecialFunction;
    /// Port 3
    #[all:cfg(feature = "port_x_1")]
    Port_3 @ 0x0018: port_x_1::Port;
    /// Port 4
    #[all:cfg(feature = "port_x_1")]
    Port_4 @ 0x001c: port_x_1::Port;
    /// Port 1
    #[all:cfg(feature = "port_1_2")]
    Port_1 @ 0x0020: port_1_2::Port;
    /// Port 2
    #[all:cfg(feature = "port_12_1")]
    Port_2 @ 0x0028: port_12_1::Port;
    /// Port 5
    #[all:cfg(feature = "port_x_1")]
    Port_5 @ 0x0030: port_x_1::Port;
    /// Port 6
    #[all:cfg(feature = "port_x_1")]
    Port_6 @ 0x0034: port_x_1::Port;
    /// Basic Timer / RTC
    #[all:cfg(feature = "basic_timer_rtc")]
    Basic_Timer_RTC @ 0x0040: basic_timer_rtc::BasicTimerRTC;
    /// System Clock FLLPLUS
    #[all:cfg(feature = "system_clock_fllplus_2")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_2::SystemClockFLLPLUS;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0056: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_1")]
    USCI_A0_UART @ 0x005d: usci_a_uart_1::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_1")]
    USCI_A0_SPI @ 0x0060: usci_a_spi_1::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_1")]
    USCI_B0_SPI @ 0x0068: usci_b_spi_1::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_1")]
    USCI_B0_I2C @ 0x0068: usci_b_i2c_1::USCI_B_I2C;
    /// LCD_A
    #[all:cfg(feature = "lcd_a_1")]
    LCD_A @ 0x0090: lcd_a_1::LCD_A;
    /// SD16_A1
    #[all:cfg(feature = "sd16_a1_2")]
    SD16_A1 @ 0x00b0: sd16_a1_2::SD16_A1;
    /// Operational Amplifier
    #[all:cfg(feature = "operational_amplifier_4")]
    Operational_Amplifier @ 0x00c0: operational_amplifier_4::OperationalAmplifier;
    /// Timer B3
    #[all:cfg(feature = "timer_b3_2")]
    Timer_B3 @ 0x011e: timer_b3_2::TimerB3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_5")]
    Flash @ 0x0128: flash_5::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// DAC12
    #[all:cfg(feature = "dac12_1")]
    DAC12 @ 0x01c0: dac12_1::DAC12;
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
    DAC12_DMA = 3,
    /// 0xFFE8 Port 1
    PORT1 = 4,
    /// 0xFFEA Timer A CC1-2, TA
    TIMERA1 = 5,
    /// 0xFFEC Timer A CC0
    TIMERA0 = 6,
    /// 0xFFEE ADC SD16A
    SD16A = 7,
    /// 0xFFF0 USCI A0/B0 Transmit
    USCIAB0TX = 8,
    /// 0xFFF2 USCI A0/B0 Receive
    USCIAB0RX = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Comparator A
    COMPARATORA = 11,
    /// 0xFFF8 Timer B CC1-2, TB
    TIMERB1 = 12,
    /// 0xFFFA Timer B CC0
    TIMERB0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn BASICTIMER();
    fn PORT2();
    fn DAC12_DMA();
    fn PORT1();
    fn TIMERA1();
    fn TIMERA0();
    fn SD16A();
    fn USCIAB0TX();
    fn USCIAB0RX();
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
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: DAC12_DMA },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _handler: SD16A },
    crate::Vector { _handler: USCIAB0TX },
    crate::Vector { _handler: USCIAB0RX },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMERB1 },
    crate::Vector { _handler: TIMERB0 },
    crate::Vector { _handler: NMI },
];
