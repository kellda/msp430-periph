//! MSP430FR2433
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2433
    #[all:cfg_attr(not(feature = "MSP430FR2433-all"), non_exhaustive)]
    MSP430FR2433;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr__special_function_registers_3")]
    SFR__Special_Function_Registers @ 0x0100: sfr__special_function_registers_3::SFRSpecialFunctionRegisters;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm__power_management_system_7")]
    PMM__Power_Management_System @ 0x0120: pmm__power_management_system_7::PMMPowerManagementSystem;
    /// SYS  System Module
    #[all:cfg(feature = "sys__system_module_5")]
    SYS__System_Module @ 0x0140: sys__system_module_5::SYSSystemModule;
    /// CS  Clock System
    #[all:cfg(feature = "cs__clock_system_5")]
    CS__Clock_System @ 0x0180: cs__clock_system_5::CSClockSystem;
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
    #[all:cfg(feature = "realtime_clock_1")]
    RealTime_Clock @ 0x0300: realtime_clock_1::RealTimeClock;
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
    #[all:cfg(feature = "mpy_16__multiplier__16_bit_mode_1")]
    MPY_16__Multiplier__16_Bit_Mode @ 0x04c0: mpy_16__multiplier__16_bit_mode_1::MPY16Multiplier16BitMode;
    /// MPY 32  Multiplier  32 Bit Mode
    #[all:cfg(feature = "mpy_32__multiplier__32_bit_mode_1")]
    MPY_32__Multiplier__32_Bit_Mode @ 0x04d0: mpy_32__multiplier__32_bit_mode_1::MPY32Multiplier32BitMode;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a0__uart_mode_3")]
    USCI_A0__UART_Mode @ 0x0500: usci_a0__uart_mode_3::USCI_A0UARTMode;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a0__spi_mode_3")]
    USCI_A0__SPI_Mode @ 0x0500: usci_a0__spi_mode_3::USCI_A0SPIMode;
    /// USCI_A1  UART Mode
    #[all:cfg(feature = "usci_a1__uart_mode_3")]
    USCI_A1__UART_Mode @ 0x0520: usci_a1__uart_mode_3::USCI_A1UARTMode;
    /// USCI_A1  SPI Mode
    #[all:cfg(feature = "usci_a1__spi_mode_3")]
    USCI_A1__SPI_Mode @ 0x0520: usci_a1__spi_mode_3::USCI_A1SPIMode;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b0__spi_mode_3")]
    USCI_B0__SPI_Mode @ 0x0540: usci_b0__spi_mode_3::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b0__i2c_mode_4")]
    USCI_B0__I2C_Mode @ 0x0540: usci_b0__i2c_mode_4::USCI_B0I2CMode;
    /// Backup Memory
    #[all:cfg(feature = "backup_memory_1")]
    Backup_Memory @ 0x0660: backup_memory_1::BackupMemory;
    /// ADC
    #[all:cfg(feature = "adc_2")]
    ADC @ 0x0700: adc_2::ADC;
}
