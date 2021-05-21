//! MSP430F67791
use crate::peripherals::*;

utils::device! {
    /// MSP430F67791
    #[all:cfg_attr(not(feature = "msp430f67791-all"), non_exhaustive)]
    MSP430F67791;
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
    #[all:cfg(feature = "ucs_3")]
    UCS @ 0x0160: ucs_3::UCS;
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
    /// Port Mapping Port 3
    #[all:cfg(feature = "port_mapping")]
    Port_Mapping_Port_3 @ 0x01d8: port_mapping::PortMapping;
    /// Port Mapping Port 4
    #[all:cfg(feature = "port_mapping")]
    Port_Mapping_Port_4 @ 0x01e0: port_mapping::PortMapping;
    /// Port A
    #[all:cfg(feature = "portw_2i")]
    Port_A @ 0x0200: portw_2i::Port;
    /// Port 1
    #[all:cfg(feature = "portb_2i1")]
    Port_1 @ 0x0200: portb_2i1::Port;
    /// Port 2
    #[all:cfg(feature = "portb_2i2")]
    Port_2 @ 0x0201: portb_2i2::Port;
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
    #[all:cfg(feature = "portw_2")]
    Port_C @ 0x0240: portw_2::Port;
    /// Port 5
    #[all:cfg(feature = "portb_2")]
    Port_5 @ 0x0240: portb_2::Port;
    /// Port 6
    #[all:cfg(feature = "portb_2")]
    Port_6 @ 0x0241: portb_2::Port;
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
    #[all:cfg(feature = "port_e_2")]
    Port_E @ 0x0280: port_e_2::Port;
    /// Port 9
    #[all:cfg(feature = "portb_1")]
    Port_9 @ 0x0280: portb_1::Port;
    /// Port 10
    #[all:cfg(feature = "portb_1")]
    Port_10 @ 0x0281: portb_1::Port;
    /// Port F
    #[all:cfg(feature = "portw_1")]
    Port_F @ 0x02a0: portw_1::Port;
    /// Port 11
    #[all:cfg(feature = "port_11_2")]
    Port_11 @ 0x02a0: port_11_2::Port;
    /// Port J
    #[all:cfg(feature = "port_j_3")]
    Port_J @ 0x0320: port_j_3::Port;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer0_A3 @ 0x0340: timer_a3_1::TimerA3;
    /// Timer1_A2
    #[all:cfg(feature = "timer_a2_1")]
    Timer1_A2 @ 0x0380: timer_a2_1::TimerA2;
    /// Timer2_A2
    #[all:cfg(feature = "timer_a2_1")]
    Timer2_A2 @ 0x0400: timer_a2_1::TimerA2;
    /// Timer3_A2
    #[all:cfg(feature = "timer_a2_1")]
    Timer3_A2 @ 0x0440: timer_a2_1::TimerA2;
    /// Backup RAM
    #[all:cfg(feature = "backup_ram")]
    Backup_RAM @ 0x0480: backup_ram::BackupRAM;
    /// MPY 16  Multiplier  16 Bit Mode
    #[all:cfg(feature = "mpy_16")]
    MPY_16 @ 0x04c0: mpy_16::MPY16;
    /// MPY 32  Multiplier  32 Bit Mode
    #[all:cfg(feature = "mpy_32")]
    MPY_32 @ 0x04d0: mpy_32::MPY32;
    /// DMA
    #[all:cfg(feature = "dma_17")]
    DMA @ 0x0500: dma_17::DMA;
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
    /// USCI_A2  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A2_UART @ 0x0600: usci_a_uart_3::USCI_A_UART;
    /// USCI_A2  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A2_SPI @ 0x0600: usci_a_spi_3::USCI_A_SPI;
    /// USCI_A3  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A3_UART @ 0x0620: usci_a_uart_3::USCI_A_UART;
    /// USCI_A3  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A3_SPI @ 0x0620: usci_a_spi_3::USCI_A_SPI;
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
    /// ADC10_A
    #[all:cfg(feature = "adc10_ab")]
    ADC10_A @ 0x0740: adc10_ab::ADC10_AB;
    /// SD24_B7
    #[all:cfg(feature = "sd24_b7_1")]
    SD24_B7 @ 0x0800: sd24_b7_1::SD24_B7;
    /// Comparator B
    #[all:cfg(feature = "comparator_b")]
    Comparator_B @ 0x08c0: comparator_b::ComparatorB;
    /// Auxilary Supply
    #[all:cfg(feature = "auxilary_supply")]
    Auxilary_Supply @ 0x09e0: auxilary_supply::AuxilarySupply;
    /// LCD_C
    #[all:cfg(feature = "lcd_c_1")]
    LCD_C @ 0x0a00: lcd_c_1::LCD_C;
    /// RTC_CE  Real Time Clock
    #[all:cfg(feature = "rtc_ce_1")]
    RTC_CE @ 0x0c80: rtc_ce_1::RTC_CE;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFCA Comparator B
    COMP_B = 37,
    /// 0xFFCC RTC
    RTC = 38,
    /// 0xFFCE LCD C
    LCD_C = 39,
    /// 0xFFD0 Timer3_A2 CC1, TA
    TIMER3_A1 = 40,
    /// 0xFFD2 Timer3_A2 CC0
    TIMER3_A0 = 41,
    /// 0xFFD4 Port 2
    PORT2 = 42,
    /// 0xFFD6 Timer2_A2 CC1, TA
    TIMER2_A1 = 43,
    /// 0xFFD8 Timer2_A2 CC0
    TIMER2_A0 = 44,
    /// 0xFFDA Port 1
    PORT1 = 45,
    /// 0xFFDC USCI B1 Receive/Transmit
    USCI_B1 = 46,
    /// 0xFFDE USCI A3 Receive/Transmit
    USCI_A3 = 47,
    /// 0xFFE0 Timer1_A2 CC1, TA1
    TIMER1_A1 = 48,
    /// 0xFFE2 Timer1_A2 CC0
    TIMER1_A0 = 49,
    /// 0xFFE4 DMA
    DMA = 50,
    /// 0xFFE6 AUX Supply
    AUX = 51,
    /// 0xFFE8 USCI A2 Receive/Transmit
    USCI_A2 = 52,
    /// 0xFFEA USCI A1 Receive/Transmit
    USCI_A1 = 53,
    /// 0xFFEC Timer0_A2 CC1-CC2, TA
    TIMER0_A1 = 54,
    /// 0xFFEE Timer0_A2 CC0
    TIMER0_A0 = 55,
    /// 0xFFF0 SD24B ADC
    SD24B = 56,
    /// 0xFFF2 ADC
    ADC10 = 57,
    /// 0xFFF4 USCI B0 Receive/Transmit
    USCI_B0 = 58,
    /// 0xFFF6 USCI A0 Receive/Transmit
    USCI_A0 = 59,
    /// 0xFFF8 Watchdog Timer
    WDT = 60,
    /// 0xFFFA User Non-maskable
    UNMI = 61,
    /// 0xFFFC System Non-maskable
    SYSNMI = 62,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn COMP_B();
    fn RTC();
    fn LCD_C();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn PORT2();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn PORT1();
    fn USCI_B1();
    fn USCI_A3();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn AUX();
    fn USCI_A2();
    fn USCI_A1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn SD24B();
    fn ADC10();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 63] = [
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: COMP_B },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: LCD_C },
    crate::Vector { _handler: TIMER3_A1 },
    crate::Vector { _handler: TIMER3_A0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: USCI_B1 },
    crate::Vector { _handler: USCI_A3 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: AUX },
    crate::Vector { _handler: USCI_A2 },
    crate::Vector { _handler: USCI_A1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: SD24B },
    crate::Vector { _handler: ADC10 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
