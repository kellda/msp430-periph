//! MSP430FR2433
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2433
    #[all:cfg_attr(not(feature = "MSP430FR2433-all"), non_exhaustive)]
    MSP430FR2433;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_3")]
    SFR @ 0x0100: sfr_3::SFR;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_7")]
    PMM @ 0x0120: pmm_7::PMM;
    /// SYS  System Module
    #[all:cfg(feature = "sys_5")]
    SYS @ 0x0140: sys_5::SYS;
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
    #[all:cfg(feature = "port_a_6")]
    Port_A @ 0x0200: port_a_6::PortA;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_10")]
    Port_1_2 @ 0x0200: port_1_2_10::Port12;
    /// Port B
    #[all:cfg(feature = "port_b_6")]
    Port_B @ 0x0220: port_b_6::PortB;
    /// Port 3
    #[all:cfg(feature = "port_3_3")]
    Port_3 @ 0x0220: port_3_3::Port3;
    /// Real-Time Clock
    #[all:cfg(feature = "realtime_clock")]
    RealTime_Clock @ 0x0300: realtime_clock::RealTimeClock;
    /// Timer0_A3
    #[all:cfg(feature = "timer0_a3_2")]
    Timer0_A3 @ 0x0380: timer0_a3_2::Timer0_A3;
    /// Timer1_A3
    #[all:cfg(feature = "timer1_a3_1")]
    Timer1_A3 @ 0x03c0: timer1_a3_1::Timer1_A3;
    /// Timer2_A2
    #[all:cfg(feature = "timer2_a2_1")]
    Timer2_A2 @ 0x0400: timer2_a2_1::Timer2_A2;
    /// Timer3_A2
    #[all:cfg(feature = "timer3_a2_1")]
    Timer3_A2 @ 0x0440: timer3_a2_1::Timer3_A2;
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
}
