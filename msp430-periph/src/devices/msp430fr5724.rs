//! MSP430FR5724
use crate::peripherals::*;

utils::device! {
    /// MSP430FR5724
    #[all:cfg_attr(not(feature = "msp430fr5724-all"), non_exhaustive)]
    MSP430FR5724;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_3")]
    SFR @ 0x0100: sfr_3::SFR;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_3")]
    PMM @ 0x0120: pmm_3::PMM;
    /// FRAM
    #[all:cfg(feature = "fram_1")]
    FRAM @ 0x0140: fram_1::FRAM;
    /// CRC16
    #[all:cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CS  Clock System
    #[all:cfg(feature = "cs_1")]
    CS @ 0x0160: cs_1::CS;
    /// SYS  System Module
    #[all:cfg(feature = "sys_2")]
    SYS @ 0x0180: sys_2::SYS;
    /// Shared Reference
    #[all:cfg(feature = "shared_reference_2")]
    Shared_Reference @ 0x01b0: shared_reference_2::SharedReference;
    /// Port A
    #[all:cfg(feature = "portw_3i")]
    Port_A @ 0x0200: portw_3i::Port;
    /// Port 1
    #[all:cfg(feature = "portb_3i1")]
    Port_1 @ 0x0200: portb_3i1::Port;
    /// Port 2
    #[all:cfg(feature = "portb_3i2")]
    Port_2 @ 0x0201: portb_3i2::Port;
    /// Port J
    #[all:cfg(feature = "port_j_4")]
    Port_J @ 0x0320: port_j_4::Port;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer0_A3 @ 0x0340: timer_a3_1::TimerA3;
    /// Timer1_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer1_A3 @ 0x0380: timer_a3_1::TimerA3;
    /// Timer0_B3
    #[all:cfg(feature = "timer_b3_1")]
    Timer0_B3 @ 0x03c0: timer_b3_1::TimerB3;
    /// RTC_B  Real Time Clock
    #[all:cfg(feature = "rtc_b")]
    RTC_B @ 0x04a0: rtc_b::RTC_B;
    /// MPY 16  Multiplier  16 Bit Mode
    #[all:cfg(feature = "mpy_16")]
    MPY_16 @ 0x04c0: mpy_16::MPY16;
    /// MPY 32  Multiplier  32 Bit Mode
    #[all:cfg(feature = "mpy_32")]
    MPY_32 @ 0x04d0: mpy_32::MPY32;
    /// DMA
    #[all:cfg(feature = "dma_18")]
    DMA @ 0x0500: dma_18::DMA;
    /// MPU
    #[all:cfg(feature = "mpu_1")]
    MPU @ 0x05a0: mpu_1::MPU;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A0_UART @ 0x05c0: usci_a_uart_3::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A0_SPI @ 0x05c0: usci_a_spi_3::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_3")]
    USCI_B0_SPI @ 0x0640: usci_b_spi_3::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_5")]
    USCI_B0_I2C @ 0x0640: usci_b_i2c_5::USCI_B_I2C;
    /// ADC10_B
    #[all:cfg(feature = "adc10_ab")]
    ADC10_B @ 0x0700: adc10_ab::ADC10_AB;
    /// Comparator D
    #[all:cfg(feature = "comparator_d")]
    Comparator_D @ 0x08c0: comparator_d::ComparatorD;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFCE RTC
    RTC = 31,
    /// 0xFFD8 Port 2
    PORT2 = 36,
    /// 0xFFDE Port 1
    PORT1 = 39,
    /// 0xFFE0 Timer1_A3 CC1-2, TA1
    TIMER1_A1 = 40,
    /// 0xFFE2 Timer1_A3 CC0
    TIMER1_A0 = 41,
    /// 0xFFE4 DMA
    DMA = 42,
    /// 0xFFE8 Timer0_A5 CC1-4, TA
    TIMER0_A1 = 44,
    /// 0xFFEA Timer0_A5 CC0
    TIMER0_A0 = 45,
    /// 0xFFEC ADC
    ADC10 = 46,
    /// 0xFFEE USCI B0 Receive/Transmit
    USCI_B0 = 47,
    /// 0xFFF0 USCI A0 Receive/Transmit
    USCI_A0 = 48,
    /// 0xFFF2 Watchdog Timer
    WDT = 49,
    /// 0xFFF4 Timer0_B3 CC1-2, TB
    TIMER0_B1 = 50,
    /// 0xFFF6 Timer0_B3 CC0
    TIMER0_B0 = 51,
    /// 0xFFF8 Comparator D
    COMP_D = 52,
    /// 0xFFFA User Non-maskable
    UNMI = 53,
    /// 0xFFFC System Non-maskable
    SYSNMI = 54,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn RTC();
    fn PORT2();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC10();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_D();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 55] = [
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
    crate::Vector { _handler: RTC },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC10 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: COMP_D },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
