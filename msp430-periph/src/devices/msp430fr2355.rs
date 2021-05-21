//! MSP430FR2355
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2355
    #[all:cfg_attr(not(feature = "msp430fr2355-all"), non_exhaustive)]
    MSP430FR2355;
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
    /// CAPTIO
    #[all:cfg(feature = "captio")]
    CAPTIO @ 0x02ee: captio::CAPTIO;
    /// RTC
    #[all:cfg(feature = "rtc_445")]
    RTC @ 0x0300: rtc_445::RTC;
    /// PJ
    #[all:cfg(feature = "portw_3_2720")]
    PJ @ 0x0320: portw_3_2720::Port;
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

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFCE
    PORT4 = 21,
    /// 0xFFD0
    PORT3 = 22,
    /// 0xFFD2
    PORT2 = 23,
    /// 0xFFD4
    PORT1 = 24,
    /// 0xFFD6
    SAC1_SAC3 = 25,
    /// 0xFFD8
    SAC0_SAC2 = 26,
    /// 0xFFDA
    ECOMP0_ECOMP1 = 27,
    /// 0xFFDC
    ADC = 28,
    /// 0xFFDE
    EUSCI_B1 = 29,
    /// 0xFFE0
    EUSCI_B0 = 30,
    /// 0xFFE2
    EUSCI_A1 = 31,
    /// 0xFFE4
    EUSCI_A0 = 32,
    /// 0xFFE6
    WDT = 33,
    /// 0xFFE8
    RTC = 34,
    /// 0xFFEA
    TIMER3_B1 = 35,
    /// 0xFFEC
    TIMER3_B0 = 36,
    /// 0xFFEE
    TIMER2_B1 = 37,
    /// 0xFFF0
    TIMER2_B0 = 38,
    /// 0xFFF2
    TIMER1_B1 = 39,
    /// 0xFFF4
    TIMER1_B0 = 40,
    /// 0xFFF6
    TIMER0_B1 = 41,
    /// 0xFFF8
    TIMER0_B0 = 42,
    /// 0xFFFA
    UNMI = 43,
    /// 0xFFFC
    SYSNMI = 44,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT4();
    fn PORT3();
    fn PORT2();
    fn PORT1();
    fn SAC1_SAC3();
    fn SAC0_SAC2();
    fn ECOMP0_ECOMP1();
    fn ADC();
    fn EUSCI_B1();
    fn EUSCI_B0();
    fn EUSCI_A1();
    fn EUSCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER3_B1();
    fn TIMER3_B0();
    fn TIMER2_B1();
    fn TIMER2_B0();
    fn TIMER1_B1();
    fn TIMER1_B0();
    fn TIMER0_B1();
    fn TIMER0_B0();
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
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT4 },
    crate::Vector { _handler: PORT3 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: SAC1_SAC3 },
    crate::Vector { _handler: SAC0_SAC2 },
    crate::Vector { _handler: ECOMP0_ECOMP1 },
    crate::Vector { _handler: ADC },
    crate::Vector { _handler: EUSCI_B1 },
    crate::Vector { _handler: EUSCI_B0 },
    crate::Vector { _handler: EUSCI_A1 },
    crate::Vector { _handler: EUSCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: RTC },
    crate::Vector { _handler: TIMER3_B1 },
    crate::Vector { _handler: TIMER3_B0 },
    crate::Vector { _handler: TIMER2_B1 },
    crate::Vector { _handler: TIMER2_B0 },
    crate::Vector { _handler: TIMER1_B1 },
    crate::Vector { _handler: TIMER1_B0 },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
