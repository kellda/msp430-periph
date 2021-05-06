//! MSP430FR4133
use crate::peripherals::*;

utils::device! {
    /// MSP430FR4133
    #[cfg_attr(not(feature = "MSP430FR4133-all"), non_exhaustive)]
    MSP430FR4133;
    /// SFR  Special Function Registers
    #[cfg(feature = "sfr__special_function_registers_3")]
    SFR__Special_Function_Registers @ 0x0100: sfr__special_function_registers_3::SFRSpecialFunctionRegisters;
    /// PMM  Power Management System
    #[cfg(feature = "pmm__power_management_system_7")]
    PMM__Power_Management_System @ 0x0120: pmm__power_management_system_7::PMMPowerManagementSystem;
    /// SYS  System Module
    #[cfg(feature = "sys__system_module_4")]
    SYS__System_Module @ 0x0140: sys__system_module_4::SYSSystemModule;
    /// CS  Clock System
    #[cfg(feature = "cs__clock_system_5")]
    CS__Clock_System @ 0x0180: cs__clock_system_5::CSClockSystem;
    /// FRAM
    #[cfg(feature = "fram_3")]
    FRAM @ 0x01a0: fram_3::FRAM;
    /// CRC16
    #[cfg(feature = "crc16_3")]
    CRC16 @ 0x01c0: crc16_3::CRC16;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_3")]
    Watchdog_Timer @ 0x01cc: watchdog_timer_3::WatchdogTimer;
    /// Port A
    #[cfg(feature = "port_a_5")]
    Port_A @ 0x0200: port_a_5::PortA;
    /// Port 1/2
    #[cfg(feature = "port_1_2_9")]
    Port_1_2 @ 0x0200: port_1_2_9::Port12;
    /// Port B
    #[cfg(feature = "port_b_5")]
    Port_B @ 0x0220: port_b_5::PortB;
    /// Port 3/4
    #[cfg(feature = "port_3_4_10")]
    Port_3_4 @ 0x0220: port_3_4_10::Port34;
    /// Port C
    #[cfg(feature = "port_c_4")]
    Port_C @ 0x0240: port_c_4::PortC;
    /// Port 5/6
    #[cfg(feature = "port_5_6_6")]
    Port_5_6 @ 0x0240: port_5_6_6::Port56;
    /// Port D
    #[cfg(feature = "port_d_5")]
    Port_D @ 0x0260: port_d_5::PortD;
    /// Port 7/8
    #[cfg(feature = "port_7_8_6")]
    Port_7_8 @ 0x0260: port_7_8_6::Port78;
    /// Capacitive_Touch_IO 0
    #[cfg(feature = "capacitive_touch_io_0_2")]
    Capacitive_Touch_IO_0 @ 0x02ee: capacitive_touch_io_0_2::Capacitive_Touch_IO0;
    /// Timer0_A3
    #[cfg(feature = "timer0_a3_5")]
    Timer0_A3 @ 0x0300: timer0_a3_5::Timer0_A3;
    /// Timer1_A3
    #[cfg(feature = "timer1_a3_5")]
    Timer1_A3 @ 0x0340: timer1_a3_5::Timer1_A3;
    /// Real-Time Clock
    #[cfg(feature = "realtime_clock_1")]
    RealTime_Clock @ 0x03c0: realtime_clock_1::RealTimeClock;
    /// USCI_A0  UART Mode
    #[cfg(feature = "usci_a0__uart_mode_5")]
    USCI_A0__UART_Mode @ 0x0500: usci_a0__uart_mode_5::USCI_A0UARTMode;
    /// USCI_A0  SPI Mode
    #[cfg(feature = "usci_a0__spi_mode_5")]
    USCI_A0__SPI_Mode @ 0x0500: usci_a0__spi_mode_5::USCI_A0SPIMode;
    /// USCI_B0  SPI Mode
    #[cfg(feature = "usci_b0__spi_mode_5")]
    USCI_B0__SPI_Mode @ 0x0540: usci_b0__spi_mode_5::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[cfg(feature = "usci_b0__i2c_mode_6")]
    USCI_B0__I2C_Mode @ 0x0540: usci_b0__i2c_mode_6::USCI_B0I2CMode;
    /// LCD_E
    #[cfg(feature = "lcd_e_1")]
    LCD_E @ 0x0600: lcd_e_1::LCD_E;
    /// Backup Memory
    #[cfg(feature = "backup_memory_1")]
    Backup_Memory @ 0x0660: backup_memory_1::BackupMemory;
    /// ADC
    #[cfg(feature = "adc_2")]
    ADC @ 0x0700: adc_2::ADC;
}
