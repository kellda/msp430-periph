//! MSP430FR2475
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2475
    #[cfg_attr(not(feature = "MSP430FR2475-all"), non_exhaustive)]
    MSP430FR2475;
    /// SFR
    #[cfg(feature = "sfr_4450")]
    SFR @ 0x0100: sfr_4450::SFR;
    /// PMM
    #[cfg(feature = "pmm_445")]
    PMM @ 0x0120: pmm_445::PMM;
    /// SYS
    #[cfg(feature = "sys_10")]
    SYS @ 0x0140: sys_10::SYS;
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
    /// PB
    #[cfg(feature = "pb_2720")]
    PB @ 0x0220: pb_2720::PB;
    /// P3
    #[cfg(feature = "p3_2720")]
    P3 @ 0x0220: p3_2720::P3;
    /// P4
    #[cfg(feature = "p4_2720")]
    P4 @ 0x0221: p4_2720::P4;
    /// PC
    #[cfg(feature = "pc_2720")]
    PC @ 0x0240: pc_2720::PC;
    /// P5
    #[cfg(feature = "p5_2720")]
    P5 @ 0x0240: p5_2720::P5;
    /// P6
    #[cfg(feature = "p6_2720")]
    P6 @ 0x0241: p6_2720::P6;
    /// PD
    #[cfg(feature = "pd_2720")]
    PD @ 0x0260: pd_2720::PD;
    /// P7
    #[cfg(feature = "p7_2720")]
    P7 @ 0x0260: p7_2720::P7;
    /// P8
    #[cfg(feature = "p8_2720")]
    P8 @ 0x0261: p8_2720::P8;
    /// PE
    #[cfg(feature = "pe_2720")]
    PE @ 0x0280: pe_2720::PE;
    /// P9
    #[cfg(feature = "p9_2720")]
    P9 @ 0x0280: p9_2720::P9;
    /// P10
    #[cfg(feature = "p10_2720")]
    P10 @ 0x0281: p10_2720::P10;
    /// RTC
    #[cfg(feature = "rtc_445")]
    RTC @ 0x0300: rtc_445::RTC;
    /// PJ
    #[cfg(feature = "pj_2720")]
    PJ @ 0x0320: pj_2720::PJ;
    /// TA0
    #[cfg(feature = "ta0_3560_inst3")]
    TA0 @ 0x0380: ta0_3560_inst3::TA0;
    /// TA1
    #[cfg(feature = "ta1_3560_inst3")]
    TA1 @ 0x03c0: ta1_3560_inst3::TA1;
    /// TA2
    #[cfg(feature = "ta2_3560_inst3")]
    TA2 @ 0x0400: ta2_3560_inst3::TA2;
    /// TA3
    #[cfg(feature = "ta3_3560_inst3")]
    TA3 @ 0x0440: ta3_3560_inst3::TA3;
    /// TB0
    #[cfg(feature = "tb0_3670_inst7")]
    TB0 @ 0x0480: tb0_3670_inst7::TB0;
    /// MPY32
    #[cfg(feature = "mpy32_2080")]
    MPY32 @ 0x04c0: mpy32_2080::MPY32;
    /// eUSCI_A0
    #[cfg(feature = "eusci_a0_445")]
    eUSCI_A0 @ 0x0500: eusci_a0_445::eUSCI_A0;
    /// eUSCI_A1
    #[cfg(feature = "eusci_a1_445")]
    eUSCI_A1 @ 0x0520: eusci_a1_445::eUSCI_A1;
    /// eUSCI_B0
    #[cfg(feature = "eusci_b0_445")]
    eUSCI_B0 @ 0x0540: eusci_b0_445::eUSCI_B0;
    /// eUSCI_B1
    #[cfg(feature = "eusci_b1_445")]
    eUSCI_B1 @ 0x0580: eusci_b1_445::eUSCI_B1;
    /// BKMEM
    #[cfg(feature = "bkmem_44508")]
    BKMEM @ 0x0660: bkmem_44508::BKMEM;
    /// ADC
    #[cfg(feature = "adc_445")]
    ADC @ 0x0700: adc_445::ADC;
    /// eCOMP0
    #[cfg(feature = "ecomp0_445")]
    eCOMP0 @ 0x08e0: ecomp0_445::eCOMP0;
}
