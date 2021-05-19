//! MSP430FG6625
use crate::peripherals::*;

utils::device! {
    /// MSP430FG6625
    #[all:cfg_attr(not(feature = "MSP430FG6625-all"), non_exhaustive)]
    MSP430FG6625;
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
    #[all:cfg(feature = "rc_2")]
    RC @ 0x0158: rc_2::RC;
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
    /// Port Mapping Control
    #[all:cfg(feature = "port_mapping_control")]
    Port_Mapping_Control @ 0x01c0: port_mapping_control::PortMappingControl;
    /// Port Mapping Port 2
    #[all:cfg(feature = "port_mapping")]
    Port_Mapping_Port_2 @ 0x01d0: port_mapping::PortMapping;
    /// Port A
    #[all:cfg(feature = "portw_1i")]
    Port_A @ 0x0200: portw_1i::Port;
    /// Port 1
    #[all:cfg(feature = "portb_1i1")]
    Port_1 @ 0x0200: portb_1i1::Port;
    /// Port 2
    #[all:cfg(feature = "portb_1i2")]
    Port_2 @ 0x0201: portb_1i2::Port;
    /// Port B
    #[all:cfg(feature = "portw_1i")]
    Port_B @ 0x0220: portw_1i::Port;
    /// Port 3
    #[all:cfg(feature = "portb_1i1")]
    Port_3 @ 0x0220: portb_1i1::Port;
    /// Port 4
    #[all:cfg(feature = "portb_1i2")]
    Port_4 @ 0x0221: portb_1i2::Port;
    /// Port C
    #[all:cfg(feature = "portw_1")]
    Port_C @ 0x0240: portw_1::Port;
    /// Port 5
    #[all:cfg(feature = "portb_1")]
    Port_5 @ 0x0240: portb_1::Port;
    /// Port 6
    #[all:cfg(feature = "portb_1")]
    Port_6 @ 0x0241: portb_1::Port;
    /// Port D
    #[all:cfg(feature = "portw_1")]
    Port_D @ 0x0260: portw_1::Port;
    /// Port 7
    #[all:cfg(feature = "portb_1")]
    Port_7 @ 0x0260: portb_1::Port;
    /// Port 8
    #[all:cfg(feature = "portb_1")]
    Port_8 @ 0x0261: portb_1::Port;
    /// Port E
    #[all:cfg(feature = "portw_1")]
    Port_E @ 0x0280: portw_1::Port;
    /// Port 9
    #[all:cfg(feature = "portb_1")]
    Port_9 @ 0x0280: portb_1::Port;
    /// Port J
    #[all:cfg(feature = "port_j_1")]
    Port_J @ 0x0320: port_j_1::Port;
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
    #[all:cfg(feature = "backup_ram")]
    Backup_RAM @ 0x0480: backup_ram::BackupRAM;
    /// Battery Charger
    #[all:cfg(feature = "battery_charger")]
    Battery_Charger @ 0x049c: battery_charger::BatteryCharger;
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
    #[all:cfg(feature = "dma_16")]
    DMA @ 0x0500: dma_16::DMA;
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
    /// DAC12
    #[all:cfg(feature = "dac12_4")]
    DAC12 @ 0x0780: dac12_4::DAC12;
    /// Comparator B
    #[all:cfg(feature = "comparator_b")]
    Comparator_B @ 0x08c0: comparator_b::ComparatorB;
    /// USB Control
    #[all:cfg(feature = "usb_control")]
    USB_Control @ 0x0900: usb_control::USBControl;
    /// LCD_B
    #[all:cfg(feature = "lcd_b_1")]
    LCD_B @ 0x0a00: lcd_b_1::LCD_B;
    /// CTSD16_B4
    #[all:cfg(feature = "ctsd16_b4")]
    CTSD16_B4 @ 0x0a80: ctsd16_b4::CTSD16_B4;
    /// Operational Amplifier 0
    #[all:cfg(feature = "operational_amplifier")]
    Operational_Amplifier_0 @ 0x0ae0: operational_amplifier::OperationalAmplifier;
    /// Operational Amplifier 1
    #[all:cfg(feature = "operational_amplifier")]
    Operational_Amplifier_1 @ 0x0af0: operational_amplifier::OperationalAmplifier;
    /// USB Operation
    #[all:cfg(feature = "usb_operation")]
    USB_Operation @ 0x1c00: usb_operation::USBOperation;
}
