//! MSP430FR2673
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2673
    #[all:cfg_attr(not(feature = "msp430fr2673-all"), non_exhaustive)]
    MSP430FR2673;
    /// SFR
    #[all:cfg(feature = "sfr_4")]
    SFR @ 0x0100: sfr_4::SFR;
    /// PMM
    #[all:cfg(feature = "pmm_445")]
    PMM @ 0x0120: pmm_445::PMM;
    /// SYS
    #[all:cfg(feature = "sys_10")]
    SYS @ 0x0140: sys_10::SYS;
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
    #[all:cfg(feature = "ta_3")]
    TA0 @ 0x0380: ta_3::TA;
    /// TA1
    #[all:cfg(feature = "ta_3")]
    TA1 @ 0x03c0: ta_3::TA;
    /// TA2
    #[all:cfg(feature = "ta_3")]
    TA2 @ 0x0400: ta_3::TA;
    /// TA3
    #[all:cfg(feature = "ta_3")]
    TA3 @ 0x0440: ta_3::TA;
    /// TB0
    #[all:cfg(feature = "tb_7")]
    TB0 @ 0x0480: tb_7::TB;
    /// MPY32
    #[all:cfg(feature = "mpy32")]
    MPY32 @ 0x04c0: mpy32::MPY32;
    /// eUSCI_A0
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A0 @ 0x0500: eusci_a::eUSCI_A;
    /// eUSCI_A1
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A1 @ 0x0520: eusci_a::eUSCI_A;
    /// eUSCI_B0
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B0 @ 0x0540: eusci_b::eUSCI_B;
    /// eUSCI_B1
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B1 @ 0x0580: eusci_b::eUSCI_B;
    /// BKMEM
    #[all:cfg(feature = "backup_memory")]
    BKMEM @ 0x0660: backup_memory::BackupMemory;
    /// ADC
    #[all:cfg(feature = "adc_3")]
    ADC @ 0x0700: adc_3::ADC;
    /// eCOMP0
    #[all:cfg(feature = "ecomp")]
    eCOMP0 @ 0x08e0: ecomp::eCOMP;
    /// CAPTIVATE
    #[all:cfg(feature = "captivate")]
    CAPTIVATE @ 0x0b20: captivate::CAPTIVATE;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFC8
    CAPTIVATE = 18,
    /// 0xFFCA
    ECOMP0 = 19,
    /// 0xFFCC
    PORT6 = 20,
    /// 0xFFCE
    PORT5 = 21,
    /// 0xFFD0
    PORT4 = 22,
    /// 0xFFD2
    PORT3 = 23,
    /// 0xFFD4
    PORT2 = 24,
    /// 0xFFD6
    PORT1 = 25,
    /// 0xFFD8
    ADC = 26,
    /// 0xFFDA
    EUSCI_B1 = 27,
    /// 0xFFDC
    EUSCI_B0 = 28,
    /// 0xFFDE
    EUSCI_A1 = 29,
    /// 0xFFE0
    EUSCI_A0 = 30,
    /// 0xFFE2
    WDT = 31,
    /// 0xFFE4
    RTC = 32,
    /// 0xFFE6
    TIMER0_B1 = 33,
    /// 0xFFE8
    TIMER0_B0 = 34,
    /// 0xFFEA
    TIMER3_A1 = 35,
    /// 0xFFEC
    TIMER3_A0 = 36,
    /// 0xFFEE
    TIMER2_A1 = 37,
    /// 0xFFF0
    TIMER2_A0 = 38,
    /// 0xFFF2
    TIMER1_A1 = 39,
    /// 0xFFF4
    TIMER1_A0 = 40,
    /// 0xFFF6
    TIMER0_A1 = 41,
    /// 0xFFF8
    TIMER0_A0 = 42,
    /// 0xFFFA
    UNMI = 43,
    /// 0xFFFC
    SYSNMI = 44,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn CAPTIVATE();
    fn ECOMP0();
    fn PORT6();
    fn PORT5();
    fn PORT4();
    fn PORT3();
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn EUSCI_B1();
    fn EUSCI_B0();
    fn EUSCI_A1();
    fn EUSCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn TIMER2_A1();
    fn TIMER2_A0();
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
static __INTERRUPTS: [crate::Vector; 45] = [
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
    crate::Vector { _handler: ECOMP0 },
    crate::Vector { _handler: PORT6 },
    crate::Vector { _handler: PORT5 },
    crate::Vector { _handler: PORT4 },
    crate::Vector { _handler: PORT3 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: ADC },
    crate::Vector { _handler: EUSCI_B1 },
    crate::Vector { _handler: EUSCI_B0 },
    crate::Vector { _handler: EUSCI_A1 },
    crate::Vector { _handler: EUSCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: TIMER3_A1 },
    crate::Vector { _handler: TIMER3_A0 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
