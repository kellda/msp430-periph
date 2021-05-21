//! MSP430F47127
use crate::peripherals::*;

utils::device! {
    /// MSP430F47127
    #[all:cfg_attr(not(feature = "msp430f47127-all"), non_exhaustive)]
    MSP430F47127;
    /// Special Function
    #[all:cfg(feature = "special_function_16")]
    Special_Function @ 0x0000: special_function_16::SpecialFunction;
    /// Port 9
    #[all:cfg(feature = "port_910")]
    Port_9 @ 0x0008: port_910::Port;
    /// Port 10
    #[all:cfg(feature = "port_910")]
    Port_10 @ 0x0009: port_910::Port;
    /// Port B
    #[all:cfg(feature = "port_b")]
    Port_B @ 0x0008: port_b::Port;
    /// Port 3
    #[all:cfg(feature = "port_x_1")]
    Port_3 @ 0x0010: port_x_1::Port;
    /// Port 4
    #[all:cfg(feature = "port_4_1")]
    Port_4 @ 0x0011: port_4_1::Port;
    /// Port 5
    #[all:cfg(feature = "port_5_1")]
    Port_5 @ 0x0012: port_5_1::Port;
    /// Port 7
    #[all:cfg(feature = "port_78")]
    Port_7 @ 0x0014: port_78::Port;
    /// Port 8
    #[all:cfg(feature = "port_78")]
    Port_8 @ 0x0015: port_78::Port;
    /// Port A
    #[all:cfg(feature = "port_a")]
    Port_A @ 0x0014: port_a::Port;
    /// Port 1
    #[all:cfg(feature = "port_12_2")]
    Port_1 @ 0x0020: port_12_2::Port;
    /// Port 2
    #[all:cfg(feature = "port_12_2")]
    Port_2 @ 0x0028: port_12_2::Port;
    /// Basic Timer / RTC
    #[all:cfg(feature = "basic_timer_rtc")]
    Basic_Timer_RTC @ 0x0040: basic_timer_rtc::BasicTimerRTC;
    /// System Clock FLLPLUS
    #[all:cfg(feature = "system_clock_fllplus_5")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_5::SystemClockFLLPLUS;
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
    #[all:cfg(feature = "usci_b_i2c_2")]
    USCI_B0_I2C @ 0x0068: usci_b_i2c_2::USCI_B_I2C;
    /// LCD_A
    #[all:cfg(feature = "lcd_a_1")]
    LCD_A @ 0x0090: lcd_a_1::LCD_A;
    /// SD16_A7
    #[all:cfg(feature = "sd16_a7_1")]
    SD16_A7 @ 0x00b0: sd16_a7_1::SD16_A7;
    /// USCI_A1  UART Mode
    #[all:cfg(feature = "usci_a_uart_1")]
    USCI_A1_UART @ 0x00cd: usci_a_uart_1::USCI_A_UART;
    /// USCI_A1  SPI Mode
    #[all:cfg(feature = "usci_a_spi_1")]
    USCI_A1_SPI @ 0x00d0: usci_a_spi_1::USCI_A_SPI;
    /// USCI_B1  SPI Mode
    #[all:cfg(feature = "usci_b_spi_1")]
    USCI_B1_SPI @ 0x00d8: usci_b_spi_1::USCI_B_SPI;
    /// USCI_B1  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_3")]
    USCI_B1_I2C @ 0x00d8: usci_b_i2c_3::USCI_B_I2C;
    /// Timer B3
    #[all:cfg(feature = "timer_b3_2")]
    Timer_B3 @ 0x011e: timer_b3_2::TimerB3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// DMA
    #[all:cfg(feature = "dma_4")]
    DMA @ 0x0122: dma_4::DMA;
    /// Flash
    #[all:cfg(feature = "flash_5")]
    Flash @ 0x0128: flash_5::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// Multiplier  16 Bit Mode
    #[all:cfg(feature = "multiplier_16")]
    Multiplier_16 @ 0x0130: multiplier_16::Multiplier16;
    /// Multiplier  32 Bit Mode
    #[all:cfg(feature = "multiplier_32")]
    Multiplier_32 @ 0x0140: multiplier_32::Multiplier32;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFDE DMA
    DMA = 15,
    /// 0xFFE0 Basic Timer / RTC
    BASICTIMER = 16,
    /// 0xFFE2 Port 2
    PORT2 = 17,
    /// 0xFFE4 USCI A1/B1 Transmit
    USCIAB1TX = 18,
    /// 0xFFE6 USCI A1/B1 Receive
    USCIAB1RX = 19,
    /// 0xFFE8 Port 1
    PORT1 = 20,
    /// 0xFFEA Timer A CC1-2, TA
    TIMERA1 = 21,
    /// 0xFFEC Timer A CC0
    TIMERA0 = 22,
    /// 0xFFEE ADC
    SD16A = 23,
    /// 0xFFF0 USCI A0/B0 Transmit
    USCIAB0TX = 24,
    /// 0xFFF2 USCI A0/B0 Receive
    USCIAB0RX = 25,
    /// 0xFFF4 Watchdog Timer
    WDT = 26,
    /// 0xFFF6 Comparator A
    COMPARATORA = 27,
    /// 0xFFF8 Timer B CC1-2, TB
    TIMERB1 = 28,
    /// 0xFFFA Timer B CC0
    TIMERB0 = 29,
    /// 0xFFFC Non-maskable
    NMI = 30,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn DMA();
    fn BASICTIMER();
    fn PORT2();
    fn USCIAB1TX();
    fn USCIAB1RX();
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
static __INTERRUPTS: [crate::Vector; 31] = [
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: USCIAB1TX },
    crate::Vector { _handler: USCIAB1RX },
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
