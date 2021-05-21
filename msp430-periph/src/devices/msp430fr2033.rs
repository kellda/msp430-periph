//! MSP430FR2033
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2033
    #[all:cfg_attr(not(feature = "msp430fr2033-all"), non_exhaustive)]
    MSP430FR2033;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_3")]
    SFR @ 0x0100: sfr_3::SFR;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_7")]
    PMM @ 0x0120: pmm_7::PMM;
    /// SYS  System Module
    #[all:cfg(feature = "sys_4")]
    SYS @ 0x0140: sys_4::SYS;
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
    #[all:cfg(feature = "portw_5i")]
    Port_A @ 0x0200: portw_5i::Port;
    /// Port 1
    #[all:cfg(feature = "portb_5i1")]
    Port_1 @ 0x0200: portb_5i1::Port;
    /// Port 2
    #[all:cfg(feature = "portb_5i2")]
    Port_2 @ 0x0201: portb_5i2::Port;
    /// Port B
    #[all:cfg(feature = "portw_5")]
    Port_B @ 0x0220: portw_5::Port;
    /// Port 3
    #[all:cfg(feature = "portb_5")]
    Port_3 @ 0x0220: portb_5::Port;
    /// Port 4
    #[all:cfg(feature = "portb_5")]
    Port_4 @ 0x0221: portb_5::Port;
    /// Port C
    #[all:cfg(feature = "portw_5")]
    Port_C @ 0x0240: portw_5::Port;
    /// Port 5
    #[all:cfg(feature = "portb_5")]
    Port_5 @ 0x0240: portb_5::Port;
    /// Port 6
    #[all:cfg(feature = "portb_5")]
    Port_6 @ 0x0241: portb_5::Port;
    /// Port D
    #[all:cfg(feature = "portw_5")]
    Port_D @ 0x0260: portw_5::Port;
    /// Port 7
    #[all:cfg(feature = "portb_5")]
    Port_7 @ 0x0260: portb_5::Port;
    /// Port 8
    #[all:cfg(feature = "portb_5")]
    Port_8 @ 0x0261: portb_5::Port;
    /// Capacitive_Touch_IO 0
    #[all:cfg(feature = "capacitive_touch_io")]
    Capacitive_Touch_IO_0 @ 0x02ee: capacitive_touch_io::Capacitive_Touch_IO;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer0_A3 @ 0x0300: timer_a3_1::TimerA3;
    /// Timer1_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer1_A3 @ 0x0340: timer_a3_1::TimerA3;
    /// Real-Time Clock
    #[all:cfg(feature = "realtime_clock")]
    RealTime_Clock @ 0x03c0: realtime_clock::RealTimeClock;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A0_UART @ 0x0500: usci_a_uart_3::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A0_SPI @ 0x0500: usci_a_spi_3::USCI_A_SPI;
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
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE4 Port 2
    PORT2 = 46,
    /// 0xFFE6 Port 1
    PORT1 = 47,
    /// 0xFFE8 ADC
    ADC = 48,
    /// 0xFFEA USCI B0 Receive/Transmit
    USCI_B0 = 49,
    /// 0xFFEC USCI A0 Receive/Transmit
    USCI_A0 = 50,
    /// 0xFFEE Watchdog Timer
    WDT = 51,
    /// 0xFFF0 RTC
    RTC = 52,
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
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn RTC();
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
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: ADC },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
