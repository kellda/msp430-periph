//! MSP430FR5868
use crate::peripherals::*;

utils::device! {
    /// MSP430FR5868
    #[all:cfg_attr(not(feature = "msp430fr5868-all"), non_exhaustive)]
    MSP430FR5868;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_3")]
    SFR @ 0x0100: sfr_3::SFR;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_4")]
    PMM @ 0x0120: pmm_4::PMM;
    /// FRAM
    #[all:cfg(feature = "fram_2")]
    FRAM @ 0x0140: fram_2::FRAM;
    /// CRC16
    #[all:cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CS  Clock System
    #[all:cfg(feature = "cs_2")]
    CS @ 0x0160: cs_2::CS;
    /// SYS  System Module
    #[all:cfg(feature = "sys_3")]
    SYS @ 0x0180: sys_3::SYS;
    /// Shared Reference
    #[all:cfg(feature = "shared_reference_3")]
    Shared_Reference @ 0x01b0: shared_reference_3::SharedReference;
    /// Port A
    #[all:cfg(feature = "portw_3i")]
    Port_A @ 0x0200: portw_3i::Port;
    /// Port 1
    #[all:cfg(feature = "portb_3i1")]
    Port_1 @ 0x0200: portb_3i1::Port;
    /// Port 2
    #[all:cfg(feature = "portb_3i2")]
    Port_2 @ 0x0201: portb_3i2::Port;
    /// Port B
    #[all:cfg(feature = "portw_3i")]
    Port_B @ 0x0220: portw_3i::Port;
    /// Port 3
    #[all:cfg(feature = "portb_3i1")]
    Port_3 @ 0x0220: portb_3i1::Port;
    /// Port 4
    #[all:cfg(feature = "portb_3i2")]
    Port_4 @ 0x0221: portb_3i2::Port;
    /// Port J
    #[all:cfg(feature = "port_j_5")]
    Port_J @ 0x0320: port_j_5::Port;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer0_A3 @ 0x0340: timer_a3_1::TimerA3;
    /// Timer1_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer1_A3 @ 0x0380: timer_a3_1::TimerA3;
    /// Timer0_B7
    #[all:cfg(feature = "timer_b7_1")]
    Timer0_B7 @ 0x03c0: timer_b7_1::TimerB7;
    /// Timer2_A2
    #[all:cfg(feature = "timer_a2_1")]
    Timer2_A2 @ 0x0400: timer_a2_1::TimerA2;
    /// Capacitive_Touch_IO 0
    #[all:cfg(feature = "capacitive_touch_io")]
    Capacitive_Touch_IO_0 @ 0x043e: capacitive_touch_io::Capacitive_Touch_IO;
    /// Timer3_A2
    #[all:cfg(feature = "timer_a2_1")]
    Timer3_A2 @ 0x0440: timer_a2_1::TimerA2;
    /// Capacitive_Touch_IO 1
    #[all:cfg(feature = "capacitive_touch_io")]
    Capacitive_Touch_IO_1 @ 0x047e: capacitive_touch_io::Capacitive_Touch_IO;
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
    #[all:cfg(feature = "mpu_2")]
    MPU @ 0x05a0: mpu_2::MPU;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A0_UART @ 0x05c0: usci_a_uart_3::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A0_SPI @ 0x05c0: usci_a_spi_3::USCI_A_SPI;
    /// USCI_A1  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A1_UART @ 0x05e0: usci_a_uart_3::USCI_A_UART;
    /// USCI_A1  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A1_SPI @ 0x05e0: usci_a_spi_3::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_3")]
    USCI_B0_SPI @ 0x0640: usci_b_spi_3::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_5")]
    USCI_B0_I2C @ 0x0640: usci_b_i2c_5::USCI_B_I2C;
    /// ADC12
    #[all:cfg(feature = "adc12_3")]
    ADC12 @ 0x0800: adc12_3::ADC12;
    /// Comparator E
    #[all:cfg(feature = "comparator_e")]
    Comparator_E @ 0x08c0: comparator_e::ComparatorE;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFCE RTC
    RTC = 31,
    /// 0xFFD0 Port 4
    PORT4 = 32,
    /// 0xFFD2 Port 3
    PORT3 = 33,
    /// 0xFFD4 Timer3_A2 CC1, TA
    TIMER3_A1 = 34,
    /// 0xFFD6 Timer3_A2 CC0
    TIMER3_A0 = 35,
    /// 0xFFD8 Port 2
    PORT2 = 36,
    /// 0xFFDA Timer2_A2 CC1, TA
    TIMER2_A1 = 37,
    /// 0xFFDC Timer2_A2 CC0
    TIMER2_A0 = 38,
    /// 0xFFDE Port 1
    PORT1 = 39,
    /// 0xFFE0 Timer1_A3 CC1-2, TA
    TIMER1_A1 = 40,
    /// 0xFFE2 Timer1_A3 CC0
    TIMER1_A0 = 41,
    /// 0xFFE4 DMA
    DMA = 42,
    /// 0xFFE6 USCI A1 Receive/Transmit
    USCI_A1 = 43,
    /// 0xFFE8 Timer0_A3 CC1-2, TA
    TIMER0_A1 = 44,
    /// 0xFFEA Timer0_A3 CC0
    TIMER0_A0 = 45,
    /// 0xFFEC ADC
    ADC12 = 46,
    /// 0xFFEE USCI B0 Receive/Transmit
    USCI_B0 = 47,
    /// 0xFFF0 USCI A0 Receive/Transmit
    USCI_A0 = 48,
    /// 0xFFF2 Watchdog Timer
    WDT = 49,
    /// 0xFFF4 Timer0_B7 CC1-6, TB
    TIMER0_B1 = 50,
    /// 0xFFF6 Timer0_B7 CC0
    TIMER0_B0 = 51,
    /// 0xFFF8 Comparator E
    COMP_E = 52,
    /// 0xFFFA User Non-maskable
    UNMI = 53,
    /// 0xFFFC System Non-maskable
    SYSNMI = 54,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn RTC();
    fn PORT4();
    fn PORT3();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn PORT2();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn USCI_A1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC12();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_E();
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
    crate::Vector { _handler: PORT4 },
    crate::Vector { _handler: PORT3 },
    crate::Vector { _handler: TIMER3_A1 },
    crate::Vector { _handler: TIMER3_A0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: USCI_A1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC12 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: COMP_E },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
