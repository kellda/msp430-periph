//! MSP430FR60371
use crate::peripherals::*;

utils::device! {
    /// MSP430FR60371
    #[all:cfg_attr(not(feature = "msp430fr60371-all"), non_exhaustive)]
    MSP430FR60371;
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
    #[all:cfg(feature = "ramctl_2")]
    RAMCTL @ 0x0158: ramctl_2::RAMCTL;
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
    /// PJ
    #[all:cfg(feature = "portw_3_2720")]
    PJ @ 0x0320: portw_3_2720::Port;
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
    /// TA4
    #[all:cfg(feature = "ta_2")]
    TA4 @ 0x07c0: ta_2::TA;
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
    /// LCD_C
    #[all:cfg(feature = "lcd_c_3")]
    LCD_C @ 0x0a00: lcd_c_3::LCD_C;
    /// LEA
    #[all:cfg(feature = "lea")]
    LEA @ 0x0a80: lea::LEA;
    /// MTIF
    #[all:cfg(feature = "mtif")]
    MTIF @ 0x0f00: mtif::MTIF;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFB4
    LEA = 17,
    /// 0xFFB6
    PORT9 = 18,
    /// 0xFFB8
    PORT8 = 19,
    /// 0xFFBA
    PORT7 = 20,
    /// 0xFFBC
    EUSCI_B1 = 21,
    /// 0xFFBE
    EUSCI_A3 = 22,
    /// 0xFFC0
    EUSCI_A2 = 23,
    /// 0xFFC2
    PORT6 = 24,
    /// 0xFFC4
    PORT5 = 25,
    /// 0xFFC6
    TIMER4_A1 = 26,
    /// 0xFFC8
    TIMER4_A0 = 27,
    /// 0xFFCA
    AES256 = 28,
    /// 0xFFCC
    RTC_C = 29,
    /// 0xFFCE
    LCD_C = 30,
    /// 0xFFD0
    PORT4 = 31,
    /// 0xFFD2
    PORT3 = 32,
    /// 0xFFD4
    TIMER3_A1 = 33,
    /// 0xFFD6
    TIMER3_A0 = 34,
    /// 0xFFD8
    PORT2 = 35,
    /// 0xFFDA
    TIMER2_A1 = 36,
    /// 0xFFDC
    TIMER2_A0 = 37,
    /// 0xFFDE
    PORT1 = 38,
    /// 0xFFE0
    TIMER1_A1 = 39,
    /// 0xFFE2
    TIMER1_A0 = 40,
    /// 0xFFE4
    DMA = 41,
    /// 0xFFE6
    EUSCI_A1 = 42,
    /// 0xFFE8
    TIMER0_A1 = 43,
    /// 0xFFEA
    TIMER0_A0 = 44,
    /// 0xFFEC
    ADC12_B = 45,
    /// 0xFFEE
    EUSCI_B0 = 46,
    /// 0xFFF0
    EUSCI_A0 = 47,
    /// 0xFFF2
    WDT = 48,
    /// 0xFFF4
    TIMER0_B1 = 49,
    /// 0xFFF6
    TIMER0_B0 = 50,
    /// 0xFFF8
    COMP_E = 51,
    /// 0xFFFA
    UNMI = 52,
    /// 0xFFFC
    SYSNMI = 53,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn LEA();
    fn PORT9();
    fn PORT8();
    fn PORT7();
    fn EUSCI_B1();
    fn EUSCI_A3();
    fn EUSCI_A2();
    fn PORT6();
    fn PORT5();
    fn TIMER4_A1();
    fn TIMER4_A0();
    fn AES256();
    fn RTC_C();
    fn LCD_C();
    fn PORT4();
    fn PORT3();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn PORT2();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn EUSCI_A1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC12_B();
    fn EUSCI_B0();
    fn EUSCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_E();
    fn UNMI();
    fn SYSNMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 54] = [
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
    crate::Vector { _handler: LEA },
    crate::Vector { _handler: PORT9 },
    crate::Vector { _handler: PORT8 },
    crate::Vector { _handler: PORT7 },
    crate::Vector { _handler: EUSCI_B1 },
    crate::Vector { _handler: EUSCI_A3 },
    crate::Vector { _handler: EUSCI_A2 },
    crate::Vector { _handler: PORT6 },
    crate::Vector { _handler: PORT5 },
    crate::Vector { _handler: TIMER4_A1 },
    crate::Vector { _handler: TIMER4_A0 },
    crate::Vector { _handler: AES256 },
    crate::Vector { _handler: RTC_C },
    crate::Vector { _handler: LCD_C },
    crate::Vector { _handler: PORT4 },
    crate::Vector { _handler: PORT3 },
    crate::Vector { _handler: TIMER3_A1 },
    crate::Vector { _handler: TIMER3_A0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _handler: TIMER2_A1 },
    crate::Vector { _handler: TIMER2_A0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: DMA },
    crate::Vector { _handler: EUSCI_A1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _handler: ADC12_B },
    crate::Vector { _handler: EUSCI_B0 },
    crate::Vector { _handler: EUSCI_A0 },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: TIMER0_B1 },
    crate::Vector { _handler: TIMER0_B0 },
    crate::Vector { _handler: COMP_E },
    crate::Vector { _handler: UNMI },
    crate::Vector { _handler: SYSNMI },
];
