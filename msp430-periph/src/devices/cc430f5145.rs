//! CC430F5145
use crate::peripherals::*;

utils::device! {
    /// CC430F5145
    #[all:cfg_attr(not(feature = "CC430F5145-all"), non_exhaustive)]
    CC430F5145;
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
    #[all:cfg(feature = "ucs_4")]
    UCS @ 0x0160: ucs_4::UCS;
    /// SYS  System Module
    #[all:cfg(feature = "sys_2")]
    SYS @ 0x0180: sys_2::SYS;
    /// Shared Reference
    #[all:cfg(feature = "shared_reference_1")]
    Shared_Reference @ 0x01b0: shared_reference_1::SharedReference;
    /// Port Mapping Control
    #[all:cfg(feature = "port_mapping_control")]
    Port_Mapping_Control @ 0x01c0: port_mapping_control::PortMappingControl;
    /// Port Mapping Port 1
    #[all:cfg(feature = "port_mapping")]
    Port_Mapping_Port_1 @ 0x01c8: port_mapping::PortMapping;
    /// Port Mapping Port 2
    #[all:cfg(feature = "port_mapping")]
    Port_Mapping_Port_2 @ 0x01d0: port_mapping::PortMapping;
    /// Port Mapping Port 3
    #[all:cfg(feature = "port_mapping")]
    Port_Mapping_Port_3 @ 0x01d8: port_mapping::PortMapping;
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
    #[all:cfg(feature = "portw_1")]
    Port_B @ 0x0220: portw_1::Port;
    /// Port 3
    #[all:cfg(feature = "portb_1")]
    Port_3 @ 0x0220: portb_1::Port;
    /// Port 4
    #[all:cfg(feature = "portb_1")]
    Port_4 @ 0x0221: portb_1::Port;
    /// Port C
    #[all:cfg(feature = "portw_1")]
    Port_C @ 0x0240: portw_1::Port;
    /// Port 5
    #[all:cfg(feature = "portb_1")]
    Port_5 @ 0x0240: portb_1::Port;
    /// Port J
    #[all:cfg(feature = "port_j_1")]
    Port_J @ 0x0320: port_j_1::Port;
    /// Timer0_A5
    #[all:cfg(feature = "timer0_a5_1")]
    Timer0_A5 @ 0x0340: timer0_a5_1::Timer0_A5;
    /// Timer1_A3
    #[all:cfg(feature = "timer1_a3_1")]
    Timer1_A3 @ 0x0380: timer1_a3_1::Timer1_A3;
    /// RTC_D  Real Time Clock
    #[all:cfg(feature = "rtc_d")]
    RTC_D @ 0x04a0: rtc_d::RTC_D;
    /// MPY 16  Multiplier  16 Bit Mode
    #[all:cfg(feature = "mpy_16")]
    MPY_16 @ 0x04c0: mpy_16::MPY16;
    /// MPY 32  Multiplier  32 Bit Mode
    #[all:cfg(feature = "mpy_32")]
    MPY_32 @ 0x04d0: mpy_32::MPY32;
    /// DMA
    #[all:cfg(feature = "dma_9")]
    DMA @ 0x0500: dma_9::DMA;
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
    /// ADC10_A
    #[all:cfg(feature = "adc10_ab")]
    ADC10_A @ 0x0740: adc10_ab::ADC10_AB;
    /// Comparator B
    #[all:cfg(feature = "comparator_b")]
    Comparator_B @ 0x08c0: comparator_b::ComparatorB;
    /// AES Accelerator
    #[all:cfg(feature = "aes_accelerator_1")]
    AES_Accelerator @ 0x09c0: aes_accelerator_1::AESAccelerator;
    /// CC1101 Radio Interface
    #[all:cfg(feature = "cc1101")]
    CC1101 @ 0x0f00: cc1101::CC1101;
}
