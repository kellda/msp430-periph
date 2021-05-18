//! MSP430FR5964
use crate::peripherals::*;

utils::device! {
    /// MSP430FR5964
    #[all:cfg_attr(not(feature = "MSP430FR5964-all"), non_exhaustive)]
    MSP430FR5964;
    /// SFR
    #[all:cfg(feature = "sfr_3")]
    SFR @ 0x0100: sfr_3::SFR;
    /// PMM
    #[all:cfg(feature = "pmm_3670")]
    PMM @ 0x0120: pmm_3670::PMM;
    /// FRCTL_A
    #[all:cfg(feature = "frctl_a")]
    FRCTL_A @ 0x0140: frctl_a::FRCTL_A;
    /// CRC
    #[all:cfg(feature = "crc16_2")]
    CRC @ 0x0150: crc16_2::CRC16;
    /// RAMCTL
    #[all:cfg(feature = "ramctl_1")]
    RAMCTL @ 0x0158: ramctl_1::RAMCTL;
    /// WDT_A
    #[all:cfg(feature = "wdt_a")]
    WDT_A @ 0x015c: wdt_a::WDT_A;
    /// CS
    #[all:cfg(feature = "cs_3670")]
    CS @ 0x0160: cs_3670::CS;
    /// SYS
    #[all:cfg(feature = "sys_2080")]
    SYS @ 0x0180: sys_2080::SYS;
    /// REF_A
    #[all:cfg(feature = "ref_a")]
    REF_A @ 0x01b0: ref_a::REF_A;
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
    /// PJ
    #[all:cfg(feature = "pj_2720")]
    PJ @ 0x0320: pj_2720::PJ;
    /// TA0
    #[all:cfg(feature = "ta_3")]
    TA0 @ 0x0340: ta_3::TA;
    /// TA1
    #[all:cfg(feature = "ta_3")]
    TA1 @ 0x0380: ta_3::TA;
    /// TB0
    #[all:cfg(feature = "tb_7")]
    TB0 @ 0x03c0: tb_7::TB;
    /// TA2
    #[all:cfg(feature = "ta_2")]
    TA2 @ 0x0400: ta_2::TA;
    /// CAPTIO0
    #[all:cfg(feature = "captio")]
    CAPTIO0 @ 0x043e: captio::CAPTIO;
    /// TA3
    #[all:cfg(feature = "ta_2")]
    TA3 @ 0x0440: ta_2::TA;
    /// CAPTIO1
    #[all:cfg(feature = "captio")]
    CAPTIO1 @ 0x047e: captio::CAPTIO;
    /// RTC_C
    #[all:cfg(feature = "rtc_c_2080")]
    RTC_C @ 0x04a0: rtc_c_2080::RTC_C;
    /// MPY32
    #[all:cfg(feature = "mpy32")]
    MPY32 @ 0x04c0: mpy32::MPY32;
    /// MPU
    #[all:cfg(feature = "mpu_3")]
    MPU @ 0x05a0: mpu_3::MPU;
    /// eUSCI_A0
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A0 @ 0x05c0: eusci_a::eUSCI_A;
    /// eUSCI_A1
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A1 @ 0x05e0: eusci_a::eUSCI_A;
    /// eUSCI_A2
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A2 @ 0x0600: eusci_a::eUSCI_A;
    /// eUSCI_A3
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A3 @ 0x0620: eusci_a::eUSCI_A;
    /// eUSCI_B0
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B0 @ 0x0640: eusci_b::eUSCI_B;
    /// eUSCI_B1
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B1 @ 0x0680: eusci_b::eUSCI_B;
    /// eUSCI_B2
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B2 @ 0x06c0: eusci_b::eUSCI_B;
    /// eUSCI_B3
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B3 @ 0x0700: eusci_b::eUSCI_B;
    /// TA4
    #[all:cfg(feature = "ta_3")]
    TA4 @ 0x07c0: ta_3::TA;
    /// ADC12_B
    #[all:cfg(feature = "adc12_b")]
    ADC12_B @ 0x0800: adc12_b::ADC12_B;
    /// COMP_E
    #[all:cfg(feature = "comp_e")]
    COMP_E @ 0x08c0: comp_e::COMP_E;
    /// CRC32
    #[all:cfg(feature = "crc32_2")]
    CRC32 @ 0x0980: crc32_2::CRC32;
    /// AES256
    #[all:cfg(feature = "aes256")]
    AES256 @ 0x09c0: aes256::AES256;
}
