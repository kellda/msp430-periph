//! MSP430F5635
use crate::peripherals::*;

utils::device! {
    /// MSP430F5635
    #[all:cfg_attr(not(feature = "msp430f5635-all"), non_exhaustive)]
    MSP430F5635;
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
    #[all:cfg(feature = "timer_a5_1")]
    Timer0_A5 @ 0x0340: timer_a5_1::TimerA5;
    /// Timer1_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer1_A3 @ 0x0380: timer_a3_1::TimerA3;
    /// Timer0_B7
    #[all:cfg(feature = "timer_b7_1")]
    Timer0_B7 @ 0x03c0: timer_b7_1::TimerB7;
    /// Timer2_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer2_A3 @ 0x0400: timer_a3_1::TimerA3;
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
    #[all:cfg(feature = "dma_12")]
    DMA @ 0x0500: dma_12::DMA;
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
    /// ADC12
    #[all:cfg(feature = "adc12_2")]
    ADC12 @ 0x0700: adc12_2::ADC12;
    /// Comparator B
    #[all:cfg(feature = "comparator_b")]
    Comparator_B @ 0x08c0: comparator_b::ComparatorB;
    /// USB Control
    #[all:cfg(feature = "usb_control")]
    USB_Control @ 0x0900: usb_control::USBControl;
    /// USB Operation
    #[all:cfg(feature = "usb_operation")]
    USB_Operation @ 0x1c00: usb_operation::USBOperation;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFCA Port 4
    PORT4 = 37,
    /// 0xFFCC Port 3
    PORT3 = 38,
    /// 0xFFCE Timer2_A5 CC1-4, TA
    TIMER2_A1 = 39,
    /// 0xFFD0 Timer2_A5 CC0
    TIMER2_A0 = 40,
    /// 0xFFD4 RTC
    RTC = 42,
    /// 0xFFD8 Port 2
    PORT2 = 44,
    /// 0xFFDA USCI B1 Receive/Transmit
    USCI_B1 = 45,
    /// 0xFFDC USCI A1 Receive/Transmit
    USCI_A1 = 46,
    /// 0xFFDE Port 1
    PORT1 = 47,
    /// 0xFFE0 Timer1_A3 CC1-2, TA1
    TIMER1_A1 = 48,
    /// 0xFFE2 Timer1_A3 CC0
    TIMER1_A0 = 49,
    /// 0xFFE4 DMA
    DMA = 50,
    /// 0xFFE6 USB Timer / cable event / USB reset
    USB_UBM = 51,
    /// 0xFFE8 Timer0_A5 CC1-4, TA
    TIMER0_A1 = 52,
    /// 0xFFEA Timer0_A5 CC0
    TIMER0_A0 = 53,
    /// 0xFFEC ADC
    ADC12 = 54,
    /// 0xFFEE USCI B0 Receive/Transmit
    USCI_B0 = 55,
    /// 0xFFF0 USCI A0 Receive/Transmit
    USCI_A0 = 56,
    /// 0xFFF2 Watchdog Timer
    WDT = 57,
    /// 0xFFF4 Timer0_B7 CC1-6, TB
    TIMER0_B1 = 58,
    /// 0xFFF6 Timer0_B7 CC0
    TIMER0_B0 = 59,
    /// 0xFFF8 Comparator B
    COMP_B = 60,
    /// 0xFFFA User Non-maskable
    UNMI = 61,
    /// 0xFFFC System Non-maskable
    SYSNMI = 62,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT4();
    fn PORT3();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn RTC();
    fn PORT2();
    fn USCI_B1();
    fn USCI_A1();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn USB_UBM();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC12();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_B();
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
    crate::Vector { _handler: PORT4 },
    crate::Vector { _handler: PORT3 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: RTC },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: USCI_B1 },
    crate::Vector { _handler: USCI_A1 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: USB_UBM },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC12 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: COMP_B },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
