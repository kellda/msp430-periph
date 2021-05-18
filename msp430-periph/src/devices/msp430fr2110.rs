//! MSP430FR2110
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2110
    #[all:cfg_attr(not(feature = "MSP430FR2110-all"), non_exhaustive)]
    MSP430FR2110;
    /// SFR
    #[all:cfg(feature = "sfr_3")]
    SFR @ 0x0100: sfr_3::SFR;
    /// PMM
    #[all:cfg(feature = "pmm_45")]
    PMM @ 0x0120: pmm_45::PMM;
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
    /// eUSCI_A0
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A0 @ 0x0500: eusci_a::eUSCI_A;
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
