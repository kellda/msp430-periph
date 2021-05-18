//! MSP430FR5727
use crate::peripherals::*;

utils::device! {
    /// MSP430FR5727
    #[all:cfg_attr(not(feature = "MSP430FR5727-all"), non_exhaustive)]
    MSP430FR5727;
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
    /// Port J
    #[all:cfg(feature = "port_j_4")]
    Port_J @ 0x0320: port_j_4::PortJ;
    /// Timer0_A3
    #[all:cfg(feature = "timer0_a3_2")]
    Timer0_A3 @ 0x0340: timer0_a3_2::Timer0_A3;
    /// Timer1_A3
    #[all:cfg(feature = "timer1_a3_1")]
    Timer1_A3 @ 0x0380: timer1_a3_1::Timer1_A3;
    /// Timer0_B3
    #[all:cfg(feature = "timer0_b3_1")]
    Timer0_B3 @ 0x03c0: timer0_b3_1::Timer0_B3;
    /// Timer1_B3
    #[all:cfg(feature = "timer1_b3_1")]
    Timer1_B3 @ 0x0400: timer1_b3_1::Timer1_B3;
    /// Timer2_B3
    #[all:cfg(feature = "timer2_b3_1")]
    Timer2_B3 @ 0x0440: timer2_b3_1::Timer2_B3;
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
    /// Comparator D
    #[all:cfg(feature = "comparator_d")]
    Comparator_D @ 0x08c0: comparator_d::ComparatorD;
}
