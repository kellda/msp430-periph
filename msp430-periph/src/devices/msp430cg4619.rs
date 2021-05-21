//! MSP430CG4619
use crate::peripherals::*;

utils::device! {
    /// MSP430CG4619
    #[all:cfg_attr(not(feature = "msp430cg4619-all"), non_exhaustive)]
    MSP430CG4619;
    /// Special Function
    #[all:cfg(feature = "special_function_7")]
    Special_Function @ 0x0000: special_function_7::SpecialFunction;
    /// Port 9
    #[all:cfg(feature = "port_x_2")]
    Port_9 @ 0x0008: port_x_2::Port;
    /// Port 10
    #[all:cfg(feature = "port_x_2")]
    Port_10 @ 0x0009: port_x_2::Port;
    /// Port B
    #[all:cfg(feature = "port_ab")]
    Port_B @ 0x0008: port_ab::Port;
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
    /// Port 7
    #[all:cfg(feature = "port_x_2")]
    Port_7 @ 0x0038: port_x_2::Port;
    /// Port 8
    #[all:cfg(feature = "port_x_2")]
    Port_8 @ 0x0039: port_x_2::Port;
    /// Port A
    #[all:cfg(feature = "port_ab")]
    Port_A @ 0x0038: port_ab::Port;
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
    /// USART 1
    #[all:cfg(feature = "usart")]
    USART_1 @ 0x0078: usart::USART;
    /// ADC12
    #[all:cfg(feature = "adc12_1")]
    ADC12 @ 0x0080: adc12_1::ADC12;
    /// LCD_A
    #[all:cfg(feature = "lcd_a_1")]
    LCD_A @ 0x0090: lcd_a_1::LCD_A;
    /// Operational Amplifier
    #[all:cfg(feature = "operational_amplifier_1")]
    Operational_Amplifier @ 0x00c0: operational_amplifier_1::OperationalAmplifier;
    /// Timer B7
    #[all:cfg(feature = "timer_b7_1")]
    Timer_B7 @ 0x011e: timer_b7_1::TimerB7;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// DMA
    #[all:cfg(feature = "dma_1")]
    DMA @ 0x0122: dma_1::DMA;
    /// Flash
    #[all:cfg(feature = "flash_2")]
    Flash @ 0x0128: flash_2::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
    /// DAC12
    #[all:cfg(feature = "dac12_1")]
    DAC12 @ 0x01c0: dac12_1::DAC12;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFDC DAC 12
    DAC12 = 14,
    /// 0xFFDE DMA
    DMA = 15,
    /// 0xFFE0 Basic Timer / RTC
    BASICTIMER = 16,
    /// 0xFFE2 Port 2
    PORT2 = 17,
    /// 0xFFE4 USART 1 Transmit
    USART1TX = 18,
    /// 0xFFE6 USART 1 Receive
    USART1RX = 19,
    /// 0xFFE8 Port 1
    PORT1 = 20,
    /// 0xFFEA Timer A CC1-2, TA
    TIMERA1 = 21,
    /// 0xFFEC Timer A CC0
    TIMERA0 = 22,
    /// 0xFFEE ADC
    ADC12 = 23,
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
    fn DAC12();
    fn DMA();
    fn BASICTIMER();
    fn PORT2();
    fn USART1TX();
    fn USART1RX();
    fn PORT1();
    fn TIMERA1();
    fn TIMERA0();
    fn ADC12();
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
    crate::Vector { _handler: DAC12 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: USART1TX },
    crate::Vector { _handler: USART1RX },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _handler: ADC12 },
    crate::Vector { _handler: USCIAB0TX },
    crate::Vector { _handler: USCIAB0RX },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMERB1 },
    crate::Vector { _handler: TIMERB0 },
    crate::Vector { _handler: NMI },
];
