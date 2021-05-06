//! MSP430FR2000
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2000
    #[cfg_attr(not(feature = "MSP430FR2000-all"), non_exhaustive)]
    MSP430FR2000;
    /// SFR
    #[cfg(feature = "sfr_2080")]
    SFR @ 0x0100: sfr_2080::SFR;
    /// PMM
    #[cfg(feature = "pmm_45")]
    PMM @ 0x0120: pmm_45::PMM;
    /// SYS
    #[cfg(feature = "sys_445")]
    SYS @ 0x0140: sys_445::SYS;
    /// CS
    #[cfg(feature = "cs_445")]
    CS @ 0x0180: cs_445::CS;
    /// FRCTL
    #[cfg(feature = "frctl_3670")]
    FRCTL @ 0x01a0: frctl_3670::FRCTL;
    /// CRC
    #[cfg(feature = "crc_2080")]
    CRC @ 0x01c0: crc_2080::CRC;
    /// WDT_A
    #[cfg(feature = "wdt_a_3560")]
    WDT_A @ 0x01cc: wdt_a_3560::WDT_A;
    /// PA
    #[cfg(feature = "pa_2720")]
    PA @ 0x0200: pa_2720::PA;
    /// P1
    #[cfg(feature = "p1_2720")]
    P1 @ 0x0200: p1_2720::P1;
    /// P2
    #[cfg(feature = "p2_2720")]
    P2 @ 0x0201: p2_2720::P2;
    /// CAPTIO
    #[cfg(feature = "captio_3670")]
    CAPTIO @ 0x02ee: captio_3670::CAPTIO;
    /// RTC
    #[cfg(feature = "rtc_445")]
    RTC @ 0x0300: rtc_445::RTC;
    /// PJ
    #[cfg(feature = "pj_2720")]
    PJ @ 0x0320: pj_2720::PJ;
    /// TB0
    #[cfg(feature = "tb0_3670_inst3")]
    TB0 @ 0x0380: tb0_3670_inst3::TB0;
    /// eUSCI_A0
    #[cfg(feature = "eusci_a0_445")]
    eUSCI_A0 @ 0x0500: eusci_a0_445::eUSCI_A0;
    /// BKMEM
    #[cfg(feature = "bkmem_44508")]
    BKMEM @ 0x0660: bkmem_44508::BKMEM;
    /// eCOMP0
    #[cfg(feature = "ecomp0_445")]
    eCOMP0 @ 0x08e0: ecomp0_445::eCOMP0;
}
