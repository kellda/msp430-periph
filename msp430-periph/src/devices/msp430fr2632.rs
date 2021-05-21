//! MSP430FR2632
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2632
    #[all:cfg_attr(not(feature = "msp430fr2632-all"), non_exhaustive)]
    MSP430FR2632;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_3")]
    SFR @ 0x0100: sfr_3::SFR;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_7")]
    PMM @ 0x0120: pmm_7::PMM;
    /// SYS  System Module
    #[all:cfg(feature = "sys_6")]
    SYS @ 0x0140: sys_6::SYS;
    /// CS  Clock System
    #[all:cfg(feature = "cs_5")]
    CS @ 0x0180: cs_5::CS;
    /// FRAM
    #[all:cfg(feature = "fram_2")]
    FRAM @ 0x01a0: fram_2::FRAM;
    /// CRC16
    #[all:cfg(feature = "crc16_2")]
    CRC16 @ 0x01c0: crc16_2::CRC16;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x01cc: watchdog_timer_2::WatchdogTimer;
    /// Port A
    #[all:cfg(feature = "portw_6i")]
    Port_A @ 0x0200: portw_6i::Port;
    /// Port 1
    #[all:cfg(feature = "portb_6i1")]
    Port_1 @ 0x0200: portb_6i1::Port;
    /// Port 2
    #[all:cfg(feature = "portb_6i2")]
    Port_2 @ 0x0201: portb_6i2::Port;
    /// Port B
    #[all:cfg(feature = "portw_6")]
    Port_B @ 0x0220: portw_6::Port;
    /// Port 3
    #[all:cfg(feature = "portb_6")]
    Port_3 @ 0x0220: portb_6::Port;
    /// Real-Time Clock
    #[all:cfg(feature = "realtime_clock")]
    RealTime_Clock @ 0x0300: realtime_clock::RealTimeClock;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer0_A3 @ 0x0380: timer_a3_1::TimerA3;
    /// Timer1_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer1_A3 @ 0x03c0: timer_a3_1::TimerA3;
    /// Timer2_A2
    #[all:cfg(feature = "timer_a2_1")]
    Timer2_A2 @ 0x0400: timer_a2_1::TimerA2;
    /// Timer3_A2
    #[all:cfg(feature = "timer_a2_1")]
    Timer3_A2 @ 0x0440: timer_a2_1::TimerA2;
    /// MPY 16  Multiplier  16 Bit Mode
    #[all:cfg(feature = "mpy_16")]
    MPY_16 @ 0x04c0: mpy_16::MPY16;
    /// MPY 32  Multiplier  32 Bit Mode
    #[all:cfg(feature = "mpy_32")]
    MPY_32 @ 0x04d0: mpy_32::MPY32;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A0_UART @ 0x0500: usci_a_uart_3::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A0_SPI @ 0x0500: usci_a_spi_3::USCI_A_SPI;
    /// USCI_A1  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A1_UART @ 0x0520: usci_a_uart_3::USCI_A_UART;
    /// USCI_A1  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A1_SPI @ 0x0520: usci_a_spi_3::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_3")]
    USCI_B0_SPI @ 0x0540: usci_b_spi_3::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_5")]
    USCI_B0_I2C @ 0x0540: usci_b_i2c_5::USCI_B_I2C;
    /// Backup Memory
    #[all:cfg(feature = "backup_memory")]
    Backup_Memory @ 0x0660: backup_memory::BackupMemory;
    /// ADC
    #[all:cfg(feature = "adc_2")]
    ADC @ 0x0700: adc_2::ADC;
    /// Captivate
    #[all:cfg(feature = "captivate")]
    Captivate @ 0x0b20: captivate::CAPTIVATE;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFD8 Captivate
    CAPTIVATE = 40,
    /// 0xFFDA Port 2
    PORT2 = 41,
    /// 0xFFDC Port 1
    PORT1 = 42,
    /// 0xFFDE ADC
    ADC = 43,
    /// 0xFFE0 USCI B0 Receive/Transmit
    USCI_B0 = 44,
    /// 0xFFE2 USCI A1 Receive/Transmit
    USCI_A1 = 45,
    /// 0xFFE4 USCI A0 Receive/Transmit
    USCI_A0 = 46,
    /// 0xFFE6 Watchdog Timer
    WDT = 47,
    /// 0xFFE8 RTC
    RTC = 48,
    /// 0xFFEA Timer3_A2 CC1, TA
    TIMER3_A1 = 49,
    /// 0xFFEC Timer3_A2 CC0
    TIMER3_A0 = 50,
    /// 0xFFEE Timer2_A2 CC1, TA
    TIMER2_A1 = 51,
    /// 0xFFF0 Timer2_A2 CC0
    TIMER2_A0 = 52,
    /// 0xFFF2 Timer1_A3 CC1-2, TA
    TIMER1_A1 = 53,
    /// 0xFFF4 Timer1_A3 CC0
    TIMER1_A0 = 54,
    /// 0xFFF6 Timer0_A3 CC1-2, TA
    TIMER0_A1 = 55,
    /// 0xFFE8 Timer0_A3 CC0
    TIMER0_A0 = 56,
    /// 0xFFFA User Non-maskable
    UNMI = 57,
    /// 0xFFFC System Non-maskable
    SYSNMI = 58,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn CAPTIVATE();
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn USCI_B0();
    fn USCI_A1();
    fn USCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 59] = [
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
    crate::Vector { _handler: CAPTIVATE },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: ADC },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A1 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: TIMER3_A1 },
    crate::Vector { _handler: TIMER3_A0 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
