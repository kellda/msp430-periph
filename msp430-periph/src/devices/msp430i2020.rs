//! MSP430i2020
use crate::peripherals::*;

utils::device! {
    /// MSP430i2020
    #[all:cfg_attr(not(feature = "msp430i2020-all"), non_exhaustive)]
    MSP430i2020;
    /// Special Function
    #[all:cfg(feature = "special_function_20")]
    Special_Function @ 0x0000: special_function_20::SpecialFunction;
    /// Port A
    #[all:cfg(feature = "portw_4i")]
    Port_A @ 0x0010: portw_4i::Port;
    /// Port 1
    #[all:cfg(feature = "portb_4i1")]
    Port_1 @ 0x0010: portb_4i1::Port;
    /// Port 2
    #[all:cfg(feature = "portb_4i2")]
    Port_2 @ 0x0011: portb_4i2::Port;
    /// CS  Clock System
    #[all:cfg(feature = "cs_4")]
    CS @ 0x0050: cs_4::CS;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_5")]
    PMM @ 0x0060: pmm_5::PMM;
    /// SD24_A2
    #[all:cfg(feature = "sd24_a2_2")]
    SD24_A2 @ 0x00b0: sd24_a2_2::SD24_A2;
    /// Timer1_A3
    #[all:cfg(feature = "timer_a3_2")]
    Timer1_A3 @ 0x011e: timer_a3_2::TimerA3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_7")]
    Flash @ 0x0128: flash_7::Flash;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer0_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A0_UART @ 0x0140: usci_a_uart_3::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A0_SPI @ 0x0140: usci_a_spi_3::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_3")]
    USCI_B0_SPI @ 0x01c0: usci_b_spi_3::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_5")]
    USCI_B0_I2C @ 0x01c0: usci_b_i2c_5::USCI_B_I2C;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE2 Port 2
    PORT2 = 1,
    /// 0xFFE8 Port 1
    PORT1 = 4,
    /// 0xFFEA Timer0_A CC1-2, TA0
    TIMER0_A1 = 5,
    /// 0xFFEC Timer0_A CC0
    TIMER0_A0 = 6,
    /// 0xFFEE Sigma Delta ADC
    SD24 = 7,
    /// 0xFFF0 USCI B0 Receive/Transmit
    USCI_B0 = 8,
    /// 0xFFF2 USCI A0 Receive/Transmit
    USCI_A0 = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Voltage Monitor
    VMON = 11,
    /// 0xFFF8 Timer1_A CC1-4, TA1
    TIMER1_A1 = 12,
    /// 0xFFFA Timer1_A CC0
    TIMER1_A0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT2();
    fn PORT1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn SD24();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn VMON();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: SD24 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: VMON },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: NMI },
];
