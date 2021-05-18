//! MSP430FR2353
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2353
    #[all:cfg_attr(not(feature = "MSP430FR2353-all"), non_exhaustive)]
    MSP430FR2353;
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
    /// CAPTIO
    #[all:cfg(feature = "captio")]
    CAPTIO @ 0x02ee: captio::CAPTIO;
    /// RTC
    #[all:cfg(feature = "rtc_445")]
    RTC @ 0x0300: rtc_445::RTC;
    /// PJ
    #[all:cfg(feature = "pj_2720")]
    PJ @ 0x0320: pj_2720::PJ;
    /// TB0
    #[all:cfg(feature = "tb_3")]
    TB0 @ 0x0380: tb_3::TB;
    /// TB1
    #[all:cfg(feature = "tb_3")]
    TB1 @ 0x03c0: tb_3::TB;
    /// TB2
    #[all:cfg(feature = "tb_3")]
    TB2 @ 0x0400: tb_3::TB;
    /// TB3
    #[all:cfg(feature = "tb3_3670_inst7")]
    TB3 @ 0x0440: tb3_3670_inst7::TB3;
    /// MPY32
    #[all:cfg(feature = "mpy32")]
    MPY32 @ 0x04c0: mpy32::MPY32;
    /// eUSCI_A0
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A0 @ 0x0500: eusci_a::eUSCI_A;
    /// eUSCI_B0
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B0 @ 0x0540: eusci_b::eUSCI_B;
    /// eUSCI_A1
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A1 @ 0x0580: eusci_a::eUSCI_A;
    /// eUSCI_B1
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B1 @ 0x05c0: eusci_b::eUSCI_B;
    /// BKMEM
    #[all:cfg(feature = "backup_memory")]
    BKMEM @ 0x0660: backup_memory::BackupMemory;
    /// ICC
    #[all:cfg(feature = "icc")]
    ICC @ 0x06c0: icc::ICC;
    /// ADC
    #[all:cfg(feature = "adc_3")]
    ADC @ 0x0700: adc_3::ADC;
    /// eCOMP0
    #[all:cfg(feature = "ecomp")]
    eCOMP0 @ 0x08e0: ecomp::eCOMP;
    /// eCOMP1
    #[all:cfg(feature = "ecomp")]
    eCOMP1 @ 0x0900: ecomp::eCOMP;
    /// SAC0
    #[all:cfg(feature = "sac_3")]
    SAC0 @ 0x0c80: sac_3::SAC;
    /// SAC1
    #[all:cfg(feature = "sac_3")]
    SAC1 @ 0x0c90: sac_3::SAC;
    /// SAC2
    #[all:cfg(feature = "sac_3")]
    SAC2 @ 0x0ca0: sac_3::SAC;
    /// SAC3
    #[all:cfg(feature = "sac_3")]
    SAC3 @ 0x0cb0: sac_3::SAC;
}
