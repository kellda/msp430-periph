//! MSP430F5438A
use crate::peripherals::*;

utils::device! {
    /// MSP430F5438A
    #[all:cfg_attr(not(feature = "MSP430F5438A-all"), non_exhaustive)]
    MSP430F5438A;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_1")]
    SFR @ 0x0100: sfr_1::SFR;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_2")]
    PMM @ 0x0120: pmm_2::PMM;
    /// Flash
    #[all:cfg(feature = "flash_6")]
    Flash @ 0x0140: flash_6::Flash;
    /// CRC16
    #[all:cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// RC  RAM Control Module
    #[all:cfg(feature = "rc_1")]
    RC @ 0x0158: rc_1::RC;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// UCS  Unified System Clock
    #[all:cfg(feature = "ucs_1")]
    UCS @ 0x0160: ucs_1::UCS;
    /// SYS  System Module
    #[all:cfg(feature = "sys_2")]
    SYS @ 0x0180: sys_2::SYS;
    /// Shared Reference
    #[all:cfg(feature = "shared_reference_1")]
    Shared_Reference @ 0x01b0: shared_reference_1::SharedReference;
    /// Port A
    #[all:cfg(feature = "port_a_1")]
    Port_A @ 0x0200: port_a_1::PortA;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_5")]
    Port_1_2 @ 0x0200: port_1_2_5::Port12;
    /// Port B
    #[all:cfg(feature = "port_b_1")]
    Port_B @ 0x0220: port_b_1::PortB;
    /// Port 3/4
    #[all:cfg(feature = "port_3_4_4")]
    Port_3_4 @ 0x0220: port_3_4_4::Port34;
    /// Port C
    #[all:cfg(feature = "port_c_1")]
    Port_C @ 0x0240: port_c_1::PortC;
    /// Port 5/6
    #[all:cfg(feature = "port_5_6_3")]
    Port_5_6 @ 0x0240: port_5_6_3::Port56;
    /// Port D
    #[all:cfg(feature = "port_d_1")]
    Port_D @ 0x0260: port_d_1::PortD;
    /// Port 7/8
    #[all:cfg(feature = "port_7_8_3")]
    Port_7_8 @ 0x0260: port_7_8_3::Port78;
    /// Port E
    #[all:cfg(feature = "port_e_1")]
    Port_E @ 0x0280: port_e_1::PortE;
    /// Port 9/10
    #[all:cfg(feature = "port_9_10_3")]
    Port_9_10 @ 0x0280: port_9_10_3::Port910;
    /// Port F
    #[all:cfg(feature = "port_f_1")]
    Port_F @ 0x02a0: port_f_1::PortF;
    /// Port 11
    #[all:cfg(feature = "port_11_1")]
    Port_11 @ 0x02a0: port_11_1::Port11;
    /// Port J
    #[all:cfg(feature = "port_j_1")]
    Port_J @ 0x0320: port_j_1::PortJ;
    /// Timer0_A5
    #[all:cfg(feature = "timer0_a5_1")]
    Timer0_A5 @ 0x0340: timer0_a5_1::Timer0_A5;
    /// Timer1_A3
    #[all:cfg(feature = "timer1_a3_1")]
    Timer1_A3 @ 0x0380: timer1_a3_1::Timer1_A3;
    /// Timer0_B7
    #[all:cfg(feature = "timer0_b7_1")]
    Timer0_B7 @ 0x03c0: timer0_b7_1::Timer0_B7;
    /// RTC  Real Time Clock
    #[all:cfg(feature = "rtc_1")]
    RTC @ 0x04a0: rtc_1::RTC;
    /// MPY 16  Multiplier  16 Bit Mode
    #[all:cfg(feature = "mpy_16")]
    MPY_16 @ 0x04c0: mpy_16::MPY16;
    /// MPY 32  Multiplier  32 Bit Mode
    #[all:cfg(feature = "mpy_32")]
    MPY_32 @ 0x04d0: mpy_32::MPY32;
    /// DMA
    #[all:cfg(feature = "dma_6")]
    DMA @ 0x0500: dma_6::DMA;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_2")]
    USCI_A0_UART @ 0x05c0: usci_a_uart_2::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_2")]
    USCI_A0_SPI @ 0x05c0: usci_a_spi_2::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_2")]
    USCI_B0_SPI @ 0x05e0: usci_b_spi_2::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_4")]
    USCI_B0_I2C @ 0x05e0: usci_b_i2c_4::USCI_B_I2C;
    /// USCI_A1  UART Mode
    #[all:cfg(feature = "usci_a_uart_2")]
    USCI_A1_UART @ 0x0600: usci_a_uart_2::USCI_A_UART;
    /// USCI_A1  SPI Mode
    #[all:cfg(feature = "usci_a_spi_2")]
    USCI_A1_SPI @ 0x0600: usci_a_spi_2::USCI_A_SPI;
    /// USCI_B1  SPI Mode
    #[all:cfg(feature = "usci_b_spi_2")]
    USCI_B1_SPI @ 0x0620: usci_b_spi_2::USCI_B_SPI;
    /// USCI_B1  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_4")]
    USCI_B1_I2C @ 0x0620: usci_b_i2c_4::USCI_B_I2C;
    /// USCI_A2  UART Mode
    #[all:cfg(feature = "usci_a_uart_2")]
    USCI_A2_UART @ 0x0640: usci_a_uart_2::USCI_A_UART;
    /// USCI_A2  SPI Mode
    #[all:cfg(feature = "usci_a_spi_2")]
    USCI_A2_SPI @ 0x0640: usci_a_spi_2::USCI_A_SPI;
    /// USCI_B2  SPI Mode
    #[all:cfg(feature = "usci_b_spi_2")]
    USCI_B2_SPI @ 0x0660: usci_b_spi_2::USCI_B_SPI;
    /// USCI_B2  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_4")]
    USCI_B2_I2C @ 0x0660: usci_b_i2c_4::USCI_B_I2C;
    /// USCI_A3  UART Mode
    #[all:cfg(feature = "usci_a_uart_2")]
    USCI_A3_UART @ 0x0680: usci_a_uart_2::USCI_A_UART;
    /// USCI_A3  SPI Mode
    #[all:cfg(feature = "usci_a_spi_2")]
    USCI_A3_SPI @ 0x0680: usci_a_spi_2::USCI_A_SPI;
    /// USCI_B3  SPI Mode
    #[all:cfg(feature = "usci_b_spi_2")]
    USCI_B3_SPI @ 0x06a0: usci_b_spi_2::USCI_B_SPI;
    /// USCI_B3  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_4")]
    USCI_B3_I2C @ 0x06a0: usci_b_i2c_4::USCI_B_I2C;
    /// ADC12
    #[all:cfg(feature = "adc12_2")]
    ADC12 @ 0x0700: adc12_2::ADC12;
}
