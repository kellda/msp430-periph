//! MSP430FR2522
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2522
    #[all:cfg_attr(not(feature = "msp430fr2522-all"), non_exhaustive)]
    MSP430FR2522;
    /// SFR
    #[all:cfg(feature = "sfr_3")]
    SFR @ 0x0100: sfr_3::SFR;
    /// PMM
    #[all:cfg(feature = "pmm_445")]
    PMM @ 0x0120: pmm_445::PMM;
    /// SYS
    #[all:cfg(feature = "sys_445")]
    SYS @ 0x0140: sys_445::SYS;
    /// CS
    #[all:cfg(feature = "cs_445")]
    CS @ 0x0180: cs_445::CS;
    /// FRCTL
    #[all:cfg(feature = "frctl")]
    FRCTL @ 0x01a0: frctl::FRCTL;
    /// CRC
    #[all:cfg(feature = "crc16_2")]
    CRC @ 0x01c0: crc16_2::CRC16;
    /// WDT_A
    #[all:cfg(feature = "wdt_a")]
    WDT_A @ 0x01cc: wdt_a::WDT_A;
    /// PA
    #[all:cfg(feature = "portw_3i")]
    PA @ 0x0200: portw_3i::Port;
    /// P1
    #[all:cfg(feature = "portb_3i1")]
    P1 @ 0x0200: portb_3i1::Port;
    /// P2
    #[all:cfg(feature = "portb_3i2")]
    P2 @ 0x0201: portb_3i2::Port;
    /// PB
    #[all:cfg(feature = "portw_3i")]
    PB @ 0x0220: portw_3i::Port;
    /// P3
    #[all:cfg(feature = "portb_3i1")]
    P3 @ 0x0220: portb_3i1::Port;
    /// P4
    #[all:cfg(feature = "portb_3i2")]
    P4 @ 0x0221: portb_3i2::Port;
    /// PC
    #[all:cfg(feature = "portw_3i")]
    PC @ 0x0240: portw_3i::Port;
    /// P5
    #[all:cfg(feature = "portb_3i1")]
    P5 @ 0x0240: portb_3i1::Port;
    /// P6
    #[all:cfg(feature = "portb_3i2")]
    P6 @ 0x0241: portb_3i2::Port;
    /// PD
    #[all:cfg(feature = "portw_3i")]
    PD @ 0x0260: portw_3i::Port;
    /// P7
    #[all:cfg(feature = "portb_3i1")]
    P7 @ 0x0260: portb_3i1::Port;
    /// P8
    #[all:cfg(feature = "portb_3i2")]
    P8 @ 0x0261: portb_3i2::Port;
    /// PE
    #[all:cfg(feature = "portw_3i")]
    PE @ 0x0280: portw_3i::Port;
    /// P9
    #[all:cfg(feature = "portb_3i1")]
    P9 @ 0x0280: portb_3i1::Port;
    /// P10
    #[all:cfg(feature = "portb_3i_2720")]
    P10 @ 0x0281: portb_3i_2720::P1x0;
    /// RTC
    #[all:cfg(feature = "rtc_445")]
    RTC @ 0x0300: rtc_445::RTC;
    /// PJ
    #[all:cfg(feature = "portw_3_2720")]
    PJ @ 0x0320: portw_3_2720::Port;
    /// TA0
    #[all:cfg(feature = "ta_7")]
    TA0 @ 0x0380: ta_7::TA;
    /// TA1
    #[all:cfg(feature = "ta_7")]
    TA1 @ 0x03c0: ta_7::TA;
    /// MPY32
    #[all:cfg(feature = "mpy32")]
    MPY32 @ 0x04c0: mpy32::MPY32;
    /// eUSCI_A0
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A0 @ 0x0500: eusci_a::eUSCI_A;
    /// eUSCI_B0
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B0 @ 0x0540: eusci_b::eUSCI_B;
    /// BKMEM
    #[all:cfg(feature = "backup_memory")]
    BKMEM @ 0x0660: backup_memory::BackupMemory;
    /// ADC
    #[all:cfg(feature = "adc_3")]
    ADC @ 0x0700: adc_3::ADC;
    /// CAPTIVATE
    #[all:cfg(feature = "captivate")]
    CAPTIVATE @ 0x0b20: captivate::CAPTIVATE;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE2
    CAPTIVATE = 44,
    /// 0xFFE4
    PORT2 = 45,
    /// 0xFFE6
    PORT1 = 46,
    /// 0xFFE8
    ADC = 47,
    /// 0xFFEA
    EUSCI_B0 = 48,
    /// 0xFFEC
    EUSCI_A0 = 49,
    /// 0xFFEE
    WDT = 50,
    /// 0xFFF0
    RTC = 51,
    /// 0xFFF2
    TIMER1_A1 = 52,
    /// 0xFFF4
    TIMER1_A0 = 53,
    /// 0xFFF6
    TIMER0_A1 = 54,
    /// 0xFFF8
    TIMER0_A0 = 55,
    /// 0xFFFA
    UNMI = 56,
    /// 0xFFFC
    SYSNMI = 57,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn CAPTIVATE();
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn EUSCI_B0();
    fn EUSCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 58] = [
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
    crate::Vector { _handler: CAPTIVATE },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: ADC },
    crate::Vector { _handler: EUSCI_B0 },
    crate::Vector { _handler: EUSCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
