//! MSP430FR2311
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2311
    #[all:cfg_attr(not(feature = "MSP430FR2311-all"), non_exhaustive)]
    MSP430FR2311;
    /// SFR
    #[all:cfg(feature = "sfr_2080")]
    SFR @ 0x0100: sfr_2080::SFR;
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
    #[all:cfg(feature = "frctl_3670")]
    FRCTL @ 0x01a0: frctl_3670::FRCTL;
    /// CRC
    #[all:cfg(feature = "crc_2080")]
    CRC @ 0x01c0: crc_2080::CRC;
    /// WDT_A
    #[all:cfg(feature = "wdt_a_3560")]
    WDT_A @ 0x01cc: wdt_a_3560::WDT_A;
    /// PA
    #[all:cfg(feature = "pa_2720")]
    PA @ 0x0200: pa_2720::PA;
    /// P1
    #[all:cfg(feature = "p1_2720")]
    P1 @ 0x0200: p1_2720::P1;
    /// P2
    #[all:cfg(feature = "p2_2720")]
    P2 @ 0x0201: p2_2720::P2;
    /// CAPTIO
    #[all:cfg(feature = "captio_3670")]
    CAPTIO @ 0x02ee: captio_3670::CAPTIO;
    /// RTC
    #[all:cfg(feature = "rtc_445")]
    RTC @ 0x0300: rtc_445::RTC;
    /// PJ
    #[all:cfg(feature = "pj_2720")]
    PJ @ 0x0320: pj_2720::PJ;
    /// TB0
    #[all:cfg(feature = "tb0_3670_inst3")]
    TB0 @ 0x0380: tb0_3670_inst3::TB0;
    /// TB1
    #[all:cfg(feature = "tb1_3670_inst3")]
    TB1 @ 0x03c0: tb1_3670_inst3::TB1;
    /// eUSCI_A0
    #[all:cfg(feature = "eusci_a0_445")]
    eUSCI_A0 @ 0x0500: eusci_a0_445::eUSCI_A0;
    /// eUSCI_B0
    #[all:cfg(feature = "eusci_b0_445")]
    eUSCI_B0 @ 0x0540: eusci_b0_445::eUSCI_B0;
    /// BKMEM
    #[all:cfg(feature = "bkmem_44508")]
    BKMEM @ 0x0660: bkmem_44508::BKMEM;
    /// ADC
    #[all:cfg(feature = "adc_445")]
    ADC @ 0x0700: adc_445::ADC;
    /// eCOMP0
    #[all:cfg(feature = "ecomp0_445")]
    eCOMP0 @ 0x08e0: ecomp0_445::eCOMP0;
    /// SAC0
    #[all:cfg(feature = "sac0_45508_config1")]
    SAC0 @ 0x0c80: sac0_45508_config1::SAC0;
    /// TRI0
    #[all:cfg(feature = "tri0_44508")]
    TRI0 @ 0x0f00: tri0_44508::TRI0;
}
