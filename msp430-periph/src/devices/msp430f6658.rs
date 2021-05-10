//! MSP430F6658
use crate::peripherals::*;

utils::device! {
    /// MSP430F6658
    #[all:cfg_attr(not(feature = "MSP430F6658-all"), non_exhaustive)]
    MSP430F6658;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr__special_function_registers_1")]
    SFR__Special_Function_Registers @ 0x0100: sfr__special_function_registers_1::SFRSpecialFunctionRegisters;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm__power_management_system_2")]
    PMM__Power_Management_System @ 0x0120: pmm__power_management_system_2::PMMPowerManagementSystem;
    /// Flash
    #[all:cfg(feature = "flash_6")]
    Flash @ 0x0140: flash_6::Flash;
    /// CRC16
    #[all:cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// RC  RAM Control Module
    #[all:cfg(feature = "rc__ram_control_module_2")]
    RC__RAM_Control_Module @ 0x0158: rc__ram_control_module_2::RCRAMControlModule;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// UCS  Unified System Clock
    #[all:cfg(feature = "ucs__unified_system_clock_1")]
    UCS__Unified_System_Clock @ 0x0160: ucs__unified_system_clock_1::UCSUnifiedSystemClock;
    /// SYS  System Module
    #[all:cfg(feature = "sys__system_module_2")]
    SYS__System_Module @ 0x0180: sys__system_module_2::SYSSystemModule;
    /// Shared Reference
    #[all:cfg(feature = "shared_reference_1")]
    Shared_Reference @ 0x01b0: shared_reference_1::SharedReference;
    /// Port Mapping Control
    #[all:cfg(feature = "port_mapping_control_1")]
    Port_Mapping_Control @ 0x01c0: port_mapping_control_1::PortMappingControl;
    /// Port Mapping Port 2
    #[all:cfg(feature = "port_mapping_port_2_1")]
    Port_Mapping_Port_2 @ 0x01d0: port_mapping_port_2_1::PortMappingPort2;
    /// Port A
    #[all:cfg(feature = "port_a_1")]
    Port_A @ 0x0200: port_a_1::PortA;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_5")]
    Port_1_2 @ 0x0200: port_1_2_5::Port12;
    /// Port B
    #[all:cfg(feature = "port_b_2")]
    Port_B @ 0x0220: port_b_2::PortB;
    /// Port 3/4
    #[all:cfg(feature = "port_3_4_5")]
    Port_3_4 @ 0x0220: port_3_4_5::Port34;
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
    /// Port 9
    #[all:cfg(feature = "port_9_1")]
    Port_9 @ 0x0280: port_9_1::Port9;
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
    /// Timer2_A3
    #[all:cfg(feature = "timer2_a3_1")]
    Timer2_A3 @ 0x0400: timer2_a3_1::Timer2_A3;
    /// Backup RAM
    #[all:cfg(feature = "backup_ram_1")]
    Backup_RAM @ 0x0480: backup_ram_1::BackupRAM;
    /// Battery Charger
    #[all:cfg(feature = "battery_charger_1")]
    Battery_Charger @ 0x049c: battery_charger_1::BatteryCharger;
    /// RTC_B  Real Time Clock
    #[all:cfg(feature = "rtc_b__real_time_clock_1")]
    RTC_B__Real_Time_Clock @ 0x04a0: rtc_b__real_time_clock_1::RTC_BRealTimeClock;
    /// MPY 16  Multiplier  16 Bit Mode
    #[all:cfg(feature = "mpy_16__multiplier__16_bit_mode_1")]
    MPY_16__Multiplier__16_Bit_Mode @ 0x04c0: mpy_16__multiplier__16_bit_mode_1::MPY16Multiplier16BitMode;
    /// MPY 32  Multiplier  32 Bit Mode
    #[all:cfg(feature = "mpy_32__multiplier__32_bit_mode_1")]
    MPY_32__Multiplier__32_Bit_Mode @ 0x04d0: mpy_32__multiplier__32_bit_mode_1::MPY32Multiplier32BitMode;
    /// DMA
    #[all:cfg(feature = "dma_14")]
    DMA @ 0x0500: dma_14::DMA;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a0__uart_mode_2")]
    USCI_A0__UART_Mode @ 0x05c0: usci_a0__uart_mode_2::USCI_A0UARTMode;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a0__spi_mode_2")]
    USCI_A0__SPI_Mode @ 0x05c0: usci_a0__spi_mode_2::USCI_A0SPIMode;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b0__spi_mode_2")]
    USCI_B0__SPI_Mode @ 0x05e0: usci_b0__spi_mode_2::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b0__i2c_mode_3")]
    USCI_B0__I2C_Mode @ 0x05e0: usci_b0__i2c_mode_3::USCI_B0I2CMode;
    /// USCI_A1  UART Mode
    #[all:cfg(feature = "usci_a1__uart_mode_2")]
    USCI_A1__UART_Mode @ 0x0600: usci_a1__uart_mode_2::USCI_A1UARTMode;
    /// USCI_A1  SPI Mode
    #[all:cfg(feature = "usci_a1__spi_mode_2")]
    USCI_A1__SPI_Mode @ 0x0600: usci_a1__spi_mode_2::USCI_A1SPIMode;
    /// USCI_B1  SPI Mode
    #[all:cfg(feature = "usci_b1__spi_mode_2")]
    USCI_B1__SPI_Mode @ 0x0620: usci_b1__spi_mode_2::USCI_B1SPIMode;
    /// USCI_B1  I2C Mode
    #[all:cfg(feature = "usci_b1__i2c_mode_2")]
    USCI_B1__I2C_Mode @ 0x0620: usci_b1__i2c_mode_2::USCI_B1I2CMode;
    /// USCI_A2  UART Mode
    #[all:cfg(feature = "usci_a2__uart_mode_1")]
    USCI_A2__UART_Mode @ 0x0640: usci_a2__uart_mode_1::USCI_A2UARTMode;
    /// USCI_A2  SPI Mode
    #[all:cfg(feature = "usci_a2__spi_mode_1")]
    USCI_A2__SPI_Mode @ 0x0640: usci_a2__spi_mode_1::USCI_A2SPIMode;
    /// USCI_B2  SPI Mode
    #[all:cfg(feature = "usci_b2__spi_mode_1")]
    USCI_B2__SPI_Mode @ 0x0660: usci_b2__spi_mode_1::USCI_B2SPIMode;
    /// USCI_B2  I2C Mode
    #[all:cfg(feature = "usci_b2__i2c_mode_1")]
    USCI_B2__I2C_Mode @ 0x0660: usci_b2__i2c_mode_1::USCI_B2I2CMode;
    /// ADC12
    #[all:cfg(feature = "adc12_2")]
    ADC12 @ 0x0700: adc12_2::ADC12;
    /// DAC12
    #[all:cfg(feature = "dac12_4")]
    DAC12 @ 0x0780: dac12_4::DAC12;
    /// Comparator B
    #[all:cfg(feature = "comparator_b_1")]
    Comparator_B @ 0x08c0: comparator_b_1::ComparatorB;
    /// USB Control
    #[all:cfg(feature = "usb_control_1")]
    USB_Control @ 0x0900: usb_control_1::USBControl;
    /// LCD_B
    #[all:cfg(feature = "lcd_b_1")]
    LCD_B @ 0x0a00: lcd_b_1::LCD_B;
    /// USB Operation
    #[all:cfg(feature = "usb_operation_1")]
    USB_Operation @ 0x1c00: usb_operation_1::USBOperation;
}
