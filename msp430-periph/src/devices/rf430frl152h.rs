//! RF430FRL152H
use crate::peripherals::*;

utils::device! {
    /// RF430FRL152H
    #[cfg_attr(not(feature = "RF430FRL152H-all"), non_exhaustive)]
    RF430FRL152H;
    /// SFR  Special Function Registers
    #[cfg(feature = "sfr__special_function_registers_3")]
    SFR__Special_Function_Registers @ 0x0100: sfr__special_function_registers_3::SFRSpecialFunctionRegisters;
    /// PMM  Power Management System
    #[cfg(feature = "pmm__power_management_system_6")]
    PMM__Power_Management_System @ 0x0120: pmm__power_management_system_6::PMMPowerManagementSystem;
    /// CRC16
    #[cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CCS  Compact System Clock
    #[cfg(feature = "ccs__compact_system_clock_1")]
    CCS__Compact_System_Clock @ 0x0160: ccs__compact_system_clock_1::CCSCompactSystemClock;
    /// CSYS  Compact System Module
    #[cfg(feature = "csys__compact_system_module_2")]
    CSYS__Compact_System_Module @ 0x0180: csys__compact_system_module_2::CSYSCompactSystemModule;
    /// Port A
    #[cfg(feature = "port_a_2")]
    Port_A @ 0x0200: port_a_2::PortA;
    /// Port 1/2
    #[cfg(feature = "port_1_2_6")]
    Port_1_2 @ 0x0200: port_1_2_6::Port12;
    /// Timer0_A3
    #[cfg(feature = "timer0_a3_2")]
    Timer0_A3 @ 0x0340: timer0_a3_2::Timer0_A3;
    /// USCI_B0  SPI Mode
    #[cfg(feature = "usci_b0__spi_mode_3")]
    USCI_B0__SPI_Mode @ 0x0640: usci_b0__spi_mode_3::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[cfg(feature = "usci_b0__i2c_mode_4")]
    USCI_B0__I2C_Mode @ 0x0640: usci_b0__i2c_mode_4::USCI_B0I2CMode;
    /// SD14 Module
    #[cfg(feature = "sd14_module_1")]
    SD14_Module @ 0x0700: sd14_module_1::SD14Module;
    /// RF13M Module
    #[cfg(feature = "rf13m_module_1")]
    RF13M_Module @ 0x0800: rf13m_module_1::RF13MModule;
}
