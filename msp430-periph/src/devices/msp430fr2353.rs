//! MSP430FR2353
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2353
    #[all:cfg_attr(not(feature = "MSP430FR2353-all"), non_exhaustive)]
    MSP430FR2353;
    /// SFR
    #[all:cfg(feature = "sfr_2080")]
    SFR @ 0x0100: sfr_2080::SFR;
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
    /// PB
    #[all:cfg(feature = "pb_2720")]
    PB @ 0x0220: pb_2720::PB;
    /// P3
    #[all:cfg(feature = "p3_2720")]
    P3 @ 0x0220: p3_2720::P3;
    /// P4
    #[all:cfg(feature = "p4_2720")]
    P4 @ 0x0221: p4_2720::P4;
    /// PC
    #[all:cfg(feature = "pc_2720")]
    PC @ 0x0240: pc_2720::PC;
    /// P5
    #[all:cfg(feature = "p5_2720")]
    P5 @ 0x0240: p5_2720::P5;
    /// P6
    #[all:cfg(feature = "p6_2720")]
    P6 @ 0x0241: p6_2720::P6;
    /// PD
    #[all:cfg(feature = "pd_2720")]
    PD @ 0x0260: pd_2720::PD;
    /// P7
    #[all:cfg(feature = "p7_2720")]
    P7 @ 0x0260: p7_2720::P7;
    /// P8
    #[all:cfg(feature = "p8_2720")]
    P8 @ 0x0261: p8_2720::P8;
    /// PE
    #[all:cfg(feature = "pe_2720")]
    PE @ 0x0280: pe_2720::PE;
    /// P9
    #[all:cfg(feature = "p9_2720")]
    P9 @ 0x0280: p9_2720::P9;
    /// P10
    #[all:cfg(feature = "p10_2720")]
    P10 @ 0x0281: p10_2720::P10;
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
    /// TB2
    #[all:cfg(feature = "tb2_3670_inst3")]
    TB2 @ 0x0400: tb2_3670_inst3::TB2;
    /// TB3
    #[all:cfg(feature = "tb3_3670_inst7")]
    TB3 @ 0x0440: tb3_3670_inst7::TB3;
    /// MPY32
    #[all:cfg(feature = "mpy32_2080")]
    MPY32 @ 0x04c0: mpy32_2080::MPY32;
    /// eUSCI_A0
    #[all:cfg(feature = "eusci_a0_445")]
    eUSCI_A0 @ 0x0500: eusci_a0_445::eUSCI_A0;
    /// eUSCI_B0
    #[all:cfg(feature = "eusci_b0_445")]
    eUSCI_B0 @ 0x0540: eusci_b0_445::eUSCI_B0;
    /// eUSCI_A1
    #[all:cfg(feature = "eusci_a1_445")]
    eUSCI_A1 @ 0x0580: eusci_a1_445::eUSCI_A1;
    /// eUSCI_B1
    #[all:cfg(feature = "eusci_b1_445")]
    eUSCI_B1 @ 0x05c0: eusci_b1_445::eUSCI_B1;
    /// BKMEM
    #[all:cfg(feature = "bkmem_44508")]
    BKMEM @ 0x0660: bkmem_44508::BKMEM;
    /// ICC
    #[all:cfg(feature = "icc_445")]
    ICC @ 0x06c0: icc_445::ICC;
    /// ADC
    #[all:cfg(feature = "adc_445")]
    ADC @ 0x0700: adc_445::ADC;
    /// eCOMP0
    #[all:cfg(feature = "ecomp0_445")]
    eCOMP0 @ 0x08e0: ecomp0_445::eCOMP0;
    /// eCOMP1
    #[all:cfg(feature = "ecomp1_445")]
    eCOMP1 @ 0x0900: ecomp1_445::eCOMP1;
    /// SAC0
    #[all:cfg(feature = "sac0_45508_config3")]
    SAC0 @ 0x0c80: sac0_45508_config3::SAC0;
    /// SAC1
    #[all:cfg(feature = "sac1_45508_config3")]
    SAC1 @ 0x0c90: sac1_45508_config3::SAC1;
    /// SAC2
    #[all:cfg(feature = "sac2_45508_config3")]
    SAC2 @ 0x0ca0: sac2_45508_config3::SAC2;
    /// SAC3
    #[all:cfg(feature = "sac3_45508_config3")]
    SAC3 @ 0x0cb0: sac3_45508_config3::SAC3;
}
