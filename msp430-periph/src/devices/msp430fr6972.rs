//! MSP430FR6972
use crate::peripherals::*;

utils::device! {
    /// MSP430FR6972
    #[all:cfg_attr(not(feature = "MSP430FR6972-all"), non_exhaustive)]
    MSP430FR6972;
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
    #[all:cfg(feature = "cs_2")]
    CS @ 0x0160: cs_2::CS;
    /// SYS  System Module
    #[all:cfg(feature = "sys_3")]
    SYS @ 0x0180: sys_3::SYS;
    /// Shared Reference
    #[all:cfg(feature = "shared_reference_3")]
    Shared_Reference @ 0x01b0: shared_reference_3::SharedReference;
    /// Port A
    #[all:cfg(feature = "port_a_3")]
    Port_A @ 0x0200: port_a_3::PortA;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_7")]
    Port_1_2 @ 0x0200: port_1_2_7::Port12;
    /// Port B
    #[all:cfg(feature = "port_b_4")]
    Port_B @ 0x0220: port_b_4::PortB;
    /// Port 3/4
    #[all:cfg(feature = "port_3_4_7")]
    Port_3_4 @ 0x0220: port_3_4_7::Port34;
    /// Port C
    #[all:cfg(feature = "port_c_3")]
    Port_C @ 0x0240: port_c_3::PortC;
    /// Port 5/6
    #[all:cfg(feature = "port_5_6_5")]
    Port_5_6 @ 0x0240: port_5_6_5::Port56;
    /// Port D
    #[all:cfg(feature = "port_d_4")]
    Port_D @ 0x0260: port_d_4::PortD;
    /// Port 7
    #[all:cfg(feature = "port_7_3")]
    Port_7 @ 0x0260: port_7_3::Port7;
    /// Port E
    #[all:cfg(feature = "port_e_4")]
    Port_E @ 0x0280: port_e_4::PortE;
    /// Port 9
    #[all:cfg(feature = "port_9_2")]
    Port_9 @ 0x0280: port_9_2::Port9;
    /// Port J
    #[all:cfg(feature = "port_j_5")]
    Port_J @ 0x0320: port_j_5::PortJ;
    /// Timer0_A3
    #[all:cfg(feature = "timer0_a3_2")]
    Timer0_A3 @ 0x0340: timer0_a3_2::Timer0_A3;
    /// Timer1_A3
    #[all:cfg(feature = "timer1_a3_1")]
    Timer1_A3 @ 0x0380: timer1_a3_1::Timer1_A3;
    /// Timer0_B7
    #[all:cfg(feature = "timer0_b7_1")]
    Timer0_B7 @ 0x03c0: timer0_b7_1::Timer0_B7;
    /// Timer2_A2
    #[all:cfg(feature = "timer2_a2_1")]
    Timer2_A2 @ 0x0400: timer2_a2_1::Timer2_A2;
    /// Capacitive_Touch_IO 0
    #[all:cfg(feature = "capacitive_touch_io")]
    Capacitive_Touch_IO_0 @ 0x043e: capacitive_touch_io::Capacitive_Touch_IO;
    /// Timer3_A5
    #[all:cfg(feature = "timer3_a5_1")]
    Timer3_A5 @ 0x0440: timer3_a5_1::Timer3_A5;
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
