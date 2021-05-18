//! MSP430FR2476
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2476
    #[all:cfg_attr(not(feature = "MSP430FR2476-all"), non_exhaustive)]
    MSP430FR2476;
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
    #[all:cfg(feature = "px")]
    PA @ 0x0200: px::Px;
    /// P1
    #[all:cfg(feature = "p1x")]
    P1 @ 0x0200: p1x::P1x;
    /// P2
    #[all:cfg(feature = "p2x")]
    P2 @ 0x0201: p2x::P2x;
    /// PB
    #[all:cfg(feature = "px")]
    PB @ 0x0220: px::Px;
    /// P3
    #[all:cfg(feature = "p1x")]
    P3 @ 0x0220: p1x::P1x;
    /// P4
    #[all:cfg(feature = "p2x")]
    P4 @ 0x0221: p2x::P2x;
    /// PC
    #[all:cfg(feature = "px")]
    PC @ 0x0240: px::Px;
    /// P5
    #[all:cfg(feature = "p1x")]
    P5 @ 0x0240: p1x::P1x;
    /// P6
    #[all:cfg(feature = "p2x")]
    P6 @ 0x0241: p2x::P2x;
    /// PD
    #[all:cfg(feature = "px")]
    PD @ 0x0260: px::Px;
    /// P7
    #[all:cfg(feature = "p1x")]
    P7 @ 0x0260: p1x::P1x;
    /// P8
    #[all:cfg(feature = "p2x")]
    P8 @ 0x0261: p2x::P2x;
    /// PE
    #[all:cfg(feature = "px")]
    PE @ 0x0280: px::Px;
    /// P9
    #[all:cfg(feature = "p1x")]
    P9 @ 0x0280: p1x::P1x;
    /// P10
    #[all:cfg(feature = "p10_2720")]
    P10 @ 0x0281: p10_2720::P1x0;
    /// RTC
    #[all:cfg(feature = "rtc_445")]
    RTC @ 0x0300: rtc_445::RTC;
    /// PJ
    #[all:cfg(feature = "pj_2720")]
    PJ @ 0x0320: pj_2720::PJ;
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
}
