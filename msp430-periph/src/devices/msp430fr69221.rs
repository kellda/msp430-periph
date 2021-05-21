//! MSP430FR69221
use crate::peripherals::*;

utils::device! {
    /// MSP430FR69221
    #[all:cfg_attr(not(feature = "msp430fr69221-all"), non_exhaustive)]
    MSP430FR69221;
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
    /// RC  RAM Control Module (FRAM)
    #[all:cfg(feature = "rc_fram")]
    RC @ 0x0158: rc_fram::RC;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CS  Clock System
    #[all:cfg(feature = "cs_3")]
    CS @ 0x0160: cs_3::CS;
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
    /// Port C
    #[all:cfg(feature = "portw_3")]
    Port_C @ 0x0240: portw_3::Port;
    /// Port 5
    #[all:cfg(feature = "portb_3")]
    Port_5 @ 0x0240: portb_3::Port;
    /// Port 6
    #[all:cfg(feature = "portb_3")]
    Port_6 @ 0x0241: portb_3::Port;
    /// Port D
    #[all:cfg(feature = "portw_3")]
    Port_D @ 0x0260: portw_3::Port;
    /// Port 7
    #[all:cfg(feature = "portb_3")]
    Port_7 @ 0x0260: portb_3::Port;
    /// Port E
    #[all:cfg(feature = "portw_3")]
    Port_E @ 0x0280: portw_3::Port;
    /// Port 9
    #[all:cfg(feature = "portb_3")]
    Port_9 @ 0x0280: portb_3::Port;
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
    /// Timer3_A5
    #[all:cfg(feature = "timer_a5_1")]
    Timer3_A5 @ 0x0440: timer_a5_1::TimerA5;
    /// Capacitive_Touch_IO 1
    #[all:cfg(feature = "capacitive_touch_io")]
    Capacitive_Touch_IO_1 @ 0x047e: capacitive_touch_io::Capacitive_Touch_IO;
    /// RTC_C  Real Time Clock
    #[all:cfg(feature = "rtc_c_3")]
    RTC_C @ 0x04a0: rtc_c_3::RTC_C;
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
    /// USCI_B1  SPI Mode
    #[all:cfg(feature = "usci_b_spi_3")]
    USCI_B1_SPI @ 0x0680: usci_b_spi_3::USCI_B_SPI;
    /// USCI_B1  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_5")]
    USCI_B1_I2C @ 0x0680: usci_b_i2c_5::USCI_B_I2C;
    /// ADC12
    #[all:cfg(feature = "adc12_3")]
    ADC12 @ 0x0800: adc12_3::ADC12;
    /// Comparator E
    #[all:cfg(feature = "comparator_e")]
    Comparator_E @ 0x08c0: comparator_e::ComparatorE;
    /// CRC32
    #[all:cfg(feature = "crc32_1")]
    CRC32 @ 0x0980: crc32_1::CRC32;
    /// AES Accelerator
    #[all:cfg(feature = "aes_accelerator_2")]
    AES_Accelerator @ 0x09c0: aes_accelerator_2::AESAccelerator;
    /// LCD_C
    #[all:cfg(feature = "lcd_c_2")]
    LCD_C @ 0x0a00: lcd_c_2::LCD_C;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFC6 AES256
    AES256 = 27,
    /// 0xFFC8 RTC
    RTC = 28,
    /// 0xFFCA LCD C
    LCD_C = 29,
    /// 0xFFCC Port 4
    PORT4 = 30,
    /// 0xFFCE Port 3
    PORT3 = 31,
    /// 0xFFD0 Timer3_A2 CC1, TA
    TIMER3_A1 = 32,
    /// 0xFFD2 Timer3_A2 CC0
    TIMER3_A0 = 33,
    /// 0xFFD4 Port 2
    PORT2 = 34,
    /// 0xFFD6 Timer2_A3 CC1, TA
    TIMER2_A1 = 35,
    /// 0xFFD8 Timer2_A3 CC0
    TIMER2_A0 = 36,
    /// 0xFFDA Port 1
    PORT1 = 37,
    /// 0xFFDC Timer1_A3 CC1-2, TA1
    TIMER1_A1 = 38,
    /// 0xFFDE Timer1_A3 CC0
    TIMER1_A0 = 39,
    /// 0xFFE0 DMA
    DMA = 40,
    /// 0xFFE2 USCI B1 Receive/Transmit
    USCI_B1 = 41,
    /// 0xFFE4 USCI A1 Receive/Transmit
    USCI_A1 = 42,
    /// 0xFFE6 Timer0_A5 CC1-4, TA
    TIMER0_A1 = 43,
    /// 0xFFE8 Timer0_A5 CC0
    TIMER0_A0 = 44,
    /// 0xFFEA ADC
    ADC12 = 45,
    /// 0xFFEC USCI B0 Receive/Transmit
    USCI_B0 = 46,
    /// 0xFFEE USCI A0 Receive/Transmit
    USCI_A0 = 47,
    /// 0xFFF2 Watchdog Timer
    WDT = 49,
    /// 0xFFF4 Timer0_B3 CC1-2, TB
    TIMER0_B1 = 50,
    /// 0xFFF6 Timer0_B3 CC0
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
    fn AES256();
    fn RTC();
    fn LCD_C();
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
    fn USCI_B1();
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
    crate::Vector { _handler: AES256 },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: LCD_C },
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
    crate::Vector { _handler: USCI_B1 },
    crate::Vector { _handler: USCI_A1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC12 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: COMP_E },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
