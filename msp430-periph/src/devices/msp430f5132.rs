//! MSP430F5132
use crate::peripherals::*;

utils::device! {
    /// MSP430F5132
    #[all:cfg_attr(not(feature = "msp430f5132-all"), non_exhaustive)]
    MSP430F5132;
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
    #[all:cfg(feature = "ucs_5")]
    UCS @ 0x0160: ucs_5::UCS;
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
    /// Port J
    #[all:cfg(feature = "port_j_2")]
    Port_J @ 0x0320: port_j_2::Port;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer0_A3 @ 0x03c0: timer_a3_1::TimerA3;
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
    /// Timer0_D3
    #[all:cfg(feature = "timer_d3_1")]
    Timer0_D3 @ 0x0b00: timer_d3_1::TimerD3;
    /// Timer1_D3
    #[all:cfg(feature = "timer_d3_1")]
    Timer1_D3 @ 0x0b40: timer_d3_1::TimerD3;
    /// Timer_Event_Control
    #[all:cfg(feature = "timer_event_control")]
    Timer_Event_Control @ 0x0c00: timer_event_control::Timer_Event_Control;
    /// Timer_Event_Control
    #[all:cfg(feature = "timer_event_control")]
    Timer_Event_Control1 @ 0x0c20: timer_event_control::Timer_Event_Control;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFDA Port 2
    PORT2 = 45,
    /// 0xFFDC Port 1
    PORT1 = 46,
    /// 0xFFDE Timer1_D3 CC1-2, TA1
    TIMER1_D1 = 47,
    /// 0xFFE0 Timer1_D3 CC0
    TIMER1_D0 = 48,
    /// 0xFFF2 Timer Event Controller 1
    TEC1 = 49,
    /// 0xFFE4 DMA
    DMA = 50,
    /// 0xFFE6 Timer0_A3 CC1-2, TA
    TIMER0_A1 = 51,
    /// 0xFFE8 Timer0_A3 CC0
    TIMER0_A0 = 52,
    /// 0xFFEA ADC
    ADC10 = 53,
    /// 0xFFEC USCI B0 Receive/Transmit
    USCI_B0 = 54,
    /// 0xFFEE USCI A0 Receive/Transmit
    USCI_A0 = 55,
    /// 0xFFF0 Watchdog Timer
    WDT = 56,
    /// 0xFFE2 Timer0_D3 CC1-2, TA
    TIMER0_D1 = 57,
    /// 0xFFE4 Timer0_D3 CC0
    TIMER0_D0 = 58,
    /// 0xFFF6 Timer Event Controller 0
    TEC0 = 59,
    /// 0xFFF8 Watchdog Timer
    COMP_B = 60,
    /// 0xFFFA User Non-maskable
    UNMI = 61,
    /// 0xFFFC System Non-maskable
    SYSNMI = 62,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT2();
    fn PORT1();
    fn TIMER1_D1();
    fn TIMER1_D0();
    fn TEC1();
    fn DMA();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC10();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_D1();
    fn TIMER0_D0();
    fn TEC0();
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
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_D1 },
    crate::Vector { _handler: TIMER1_D0 },
    crate::Vector { _handler: TEC1 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC10 },
    crate::Vector { _handler: USCI_B0 },
    crate::Vector { _handler: USCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_D1 },
    crate::Vector { _handler: TIMER0_D0 },
    crate::Vector { _handler: TEC0 },
    crate::Vector { _handler: COMP_B },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
