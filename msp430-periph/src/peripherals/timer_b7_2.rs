//! Timer B7

utils::periph! {
    /// Timer B7
    TimerB7;
    /// Timer B7 Interrupt Vector Word
    rw TBIV @ 0x00: u16 = 0_0 {
        /// Timer B7 Interrupt Vector Word
        TBIV: 0..15 = struct TBIVField(u16);
    }
    /// Timer B7 Control
    rw TBCTL @ 0x62: u16 = 0_0 {
        /// Timer B interrupt flag
        TBIFG: 0 = struct TBIFG(bool);
        /// Timer B interrupt enable
        TBIE: 1 = struct TBIE(bool);
        /// Timer B counter clear
        TBCLR: 2 = struct TBCLR(bool);
        /// Timer B mode control 1
        MC: 4..5 = enum MC {
            /// Timer A mode control: 0 - Stop
            MC_0 = 0b00,
            /// Timer A mode control: 1 - Up to CCR0
            MC_1 = 0b01,
            /// Timer A mode control: 2 - Continuous up
            MC_2 = 0b10,
            /// Timer A mode control: 3 - Up/Down
            MC_3 = 0b11,
        }
        /// Timer B clock input divider 1
        ID: 6..7 = enum ID {
            /// Timer A input divider: 0 - /1
            ID_0 = 0b00,
            /// Timer A input divider: 1 - /2
            ID_1 = 0b01,
            /// Timer A input divider: 2 - /4
            ID_2 = 0b10,
            /// Timer A input divider: 3 - /8
            ID_3 = 0b11,
        }
        /// Clock source 1
        TBSSEL: 8..9 = enum TBSSEL {
            /// Clock Source: TBCLK
            TBSSEL_0 = 0b00,
            /// Clock Source: ACLK
            TBSSEL_1 = 0b01,
            /// Clock Source: SMCLK
            TBSSEL_2 = 0b10,
            /// Clock Source: INCLK
            TBSSEL_3 = 0b11,
        }
        /// Counter lenght 1
        CNTL: 11..12 = enum CNTL {
            /// Counter lenght: 16 bit
            CNTL_0 = 0b00,
            /// Counter lenght: 12 bit
            CNTL_1 = 0b01,
            /// Counter lenght: 10 bit
            CNTL_2 = 0b10,
            /// Counter lenght:  8 bit
            CNTL_3 = 0b11,
        }
        /// Timer B Compare latch load group 1
        TBCLGRP: 13..14 = enum TBCLGRP {
            /// Timer B Group: 0 - individually
            TBCLGRP_0 = 0b00,
            /// Timer B Group: 1 - 3 groups (1-2
            TBCLGRP_1 = 0b01,
            /// Timer B Group: 2 - 2 groups (1-3
            TBCLGRP_2 = 0b10,
            /// Timer B Group: 3 - 1 group (all)
            TBCLGRP_3 = 0b11,
        }
    }
    /// Timer B7 Capture/Compare Control 0
    rw TBCCTL0 @ 0x64: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TBCCTL0_CCIFG: 0 = struct TBCCTL0_CCIFG(bool);
        /// Capture/compare overflow flag
        TBCCTL0_COV: 1 = struct TBCCTL0_COV(bool);
        /// PWM Output signal if output mode 0
        TBCCTL0_OUT: 2 = struct TBCCTL0_OUT(bool);
        /// Capture input signal (read)
        TBCCTL0_CCI: 3 = struct TBCCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TBCCTL0_CCIE: 4 = struct TBCCTL0_CCIE(bool);
        /// Output mode 2
        TBCCTL0_OUTMOD: 5..7 = enum TBCCTL0_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TBCCTL0_CAP: 8 = struct TBCCTL0_CAP(bool);
        /// Compare latch load source 1
        TBCCTL0_CLLD: 9..10 = enum TBCCTL0_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TBCCTL0_SCS: 11 = struct TBCCTL0_SCS(bool);
        /// Capture input select 1
        TBCCTL0_CCIS: 12..13 = enum TBCCTL0_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TBCCTL0_CM: 14..15 = enum TBCCTL0_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer B7 Capture/Compare Control 1
    rw TBCCTL1 @ 0x66: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TBCCTL1_CCIFG: 0 = struct TBCCTL1_CCIFG(bool);
        /// Capture/compare overflow flag
        TBCCTL1_COV: 1 = struct TBCCTL1_COV(bool);
        /// PWM Output signal if output mode 0
        TBCCTL1_OUT: 2 = struct TBCCTL1_OUT(bool);
        /// Capture input signal (read)
        TBCCTL1_CCI: 3 = struct TBCCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TBCCTL1_CCIE: 4 = struct TBCCTL1_CCIE(bool);
        /// Output mode 2
        TBCCTL1_OUTMOD: 5..7 = enum TBCCTL1_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TBCCTL1_CAP: 8 = struct TBCCTL1_CAP(bool);
        /// Compare latch load source 1
        TBCCTL1_CLLD: 9..10 = enum TBCCTL1_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TBCCTL1_SCS: 11 = struct TBCCTL1_SCS(bool);
        /// Capture input select 1
        TBCCTL1_CCIS: 12..13 = enum TBCCTL1_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TBCCTL1_CM: 14..15 = enum TBCCTL1_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer B7 Capture/Compare Control 2
    rw TBCCTL2 @ 0x68: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TBCCTL2_CCIFG: 0 = struct TBCCTL2_CCIFG(bool);
        /// Capture/compare overflow flag
        TBCCTL2_COV: 1 = struct TBCCTL2_COV(bool);
        /// PWM Output signal if output mode 0
        TBCCTL2_OUT: 2 = struct TBCCTL2_OUT(bool);
        /// Capture input signal (read)
        TBCCTL2_CCI: 3 = struct TBCCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TBCCTL2_CCIE: 4 = struct TBCCTL2_CCIE(bool);
        /// Output mode 2
        TBCCTL2_OUTMOD: 5..7 = enum TBCCTL2_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TBCCTL2_CAP: 8 = struct TBCCTL2_CAP(bool);
        /// Compare latch load source 1
        TBCCTL2_CLLD: 9..10 = enum TBCCTL2_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TBCCTL2_SCS: 11 = struct TBCCTL2_SCS(bool);
        /// Capture input select 1
        TBCCTL2_CCIS: 12..13 = enum TBCCTL2_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TBCCTL2_CM: 14..15 = enum TBCCTL2_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer B7 Capture/Compare Control 3
    rw TBCCTL3 @ 0x6a: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TBCCTL3_CCIFG: 0 = struct TBCCTL3_CCIFG(bool);
        /// Capture/compare overflow flag
        TBCCTL3_COV: 1 = struct TBCCTL3_COV(bool);
        /// PWM Output signal if output mode 0
        TBCCTL3_OUT: 2 = struct TBCCTL3_OUT(bool);
        /// Capture input signal (read)
        TBCCTL3_CCI: 3 = struct TBCCTL3_CCI(bool);
        /// Capture/compare interrupt enable
        TBCCTL3_CCIE: 4 = struct TBCCTL3_CCIE(bool);
        /// Output mode 2
        TBCCTL3_OUTMOD: 5..7 = enum TBCCTL3_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TBCCTL3_CAP: 8 = struct TBCCTL3_CAP(bool);
        /// Compare latch load source 1
        TBCCTL3_CLLD: 9..10 = enum TBCCTL3_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TBCCTL3_SCS: 11 = struct TBCCTL3_SCS(bool);
        /// Capture input select 1
        TBCCTL3_CCIS: 12..13 = enum TBCCTL3_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TBCCTL3_CM: 14..15 = enum TBCCTL3_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer B7 Capture/Compare Control 4
    rw TBCCTL4 @ 0x6c: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TBCCTL4_CCIFG: 0 = struct TBCCTL4_CCIFG(bool);
        /// Capture/compare overflow flag
        TBCCTL4_COV: 1 = struct TBCCTL4_COV(bool);
        /// PWM Output signal if output mode 0
        TBCCTL4_OUT: 2 = struct TBCCTL4_OUT(bool);
        /// Capture input signal (read)
        TBCCTL4_CCI: 3 = struct TBCCTL4_CCI(bool);
        /// Capture/compare interrupt enable
        TBCCTL4_CCIE: 4 = struct TBCCTL4_CCIE(bool);
        /// Output mode 2
        TBCCTL4_OUTMOD: 5..7 = enum TBCCTL4_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TBCCTL4_CAP: 8 = struct TBCCTL4_CAP(bool);
        /// Compare latch load source 1
        TBCCTL4_CLLD: 9..10 = enum TBCCTL4_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TBCCTL4_SCS: 11 = struct TBCCTL4_SCS(bool);
        /// Capture input select 1
        TBCCTL4_CCIS: 12..13 = enum TBCCTL4_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TBCCTL4_CM: 14..15 = enum TBCCTL4_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer B7 Capture/Compare Control 5
    rw TBCCTL5 @ 0x6e: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TBCCTL5_CCIFG: 0 = struct TBCCTL5_CCIFG(bool);
        /// Capture/compare overflow flag
        TBCCTL5_COV: 1 = struct TBCCTL5_COV(bool);
        /// PWM Output signal if output mode 0
        TBCCTL5_OUT: 2 = struct TBCCTL5_OUT(bool);
        /// Capture input signal (read)
        TBCCTL5_CCI: 3 = struct TBCCTL5_CCI(bool);
        /// Capture/compare interrupt enable
        TBCCTL5_CCIE: 4 = struct TBCCTL5_CCIE(bool);
        /// Output mode 2
        TBCCTL5_OUTMOD: 5..7 = enum TBCCTL5_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TBCCTL5_CAP: 8 = struct TBCCTL5_CAP(bool);
        /// Compare latch load source 1
        TBCCTL5_CLLD: 9..10 = enum TBCCTL5_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TBCCTL5_SCS: 11 = struct TBCCTL5_SCS(bool);
        /// Capture input select 1
        TBCCTL5_CCIS: 12..13 = enum TBCCTL5_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TBCCTL5_CM: 14..15 = enum TBCCTL5_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer B7 Capture/Compare Control 6
    rw TBCCTL6 @ 0x70: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TBCCTL6_CCIFG: 0 = struct TBCCTL6_CCIFG(bool);
        /// Capture/compare overflow flag
        TBCCTL6_COV: 1 = struct TBCCTL6_COV(bool);
        /// PWM Output signal if output mode 0
        TBCCTL6_OUT: 2 = struct TBCCTL6_OUT(bool);
        /// Capture input signal (read)
        TBCCTL6_CCI: 3 = struct TBCCTL6_CCI(bool);
        /// Capture/compare interrupt enable
        TBCCTL6_CCIE: 4 = struct TBCCTL6_CCIE(bool);
        /// Output mode 2
        TBCCTL6_OUTMOD: 5..7 = enum TBCCTL6_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TBCCTL6_CAP: 8 = struct TBCCTL6_CAP(bool);
        /// Compare latch load source 1
        TBCCTL6_CLLD: 9..10 = enum TBCCTL6_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TBCCTL6_SCS: 11 = struct TBCCTL6_SCS(bool);
        /// Capture input select 1
        TBCCTL6_CCIS: 12..13 = enum TBCCTL6_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TBCCTL6_CM: 14..15 = enum TBCCTL6_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer B7 Counter Register
    rw TBR @ 0x72: u16 = 0_0 {
        /// Timer B7 Counter Register
        TBR: 0..15 = struct TBRField(u16);
    }
    /// Timer B7 Capture/Compare 0
    rw TBCCR0 @ 0x74: u16 = 0_0 {
        /// Timer B7 Capture/Compare 0
        TBCCR0: 0..15 = struct TBCCR0Field(u16);
    }
    /// Timer B7 Capture/Compare 1
    rw TBCCR1 @ 0x76: u16 = 0_0 {
        /// Timer B7 Capture/Compare 1
        TBCCR1: 0..15 = struct TBCCR1Field(u16);
    }
    /// Timer B7 Capture/Compare 2
    rw TBCCR2 @ 0x78: u16 = 0_0 {
        /// Timer B7 Capture/Compare 2
        TBCCR2: 0..15 = struct TBCCR2Field(u16);
    }
    /// Timer B7 Capture/Compare 3
    rw TBCCR3 @ 0x7a: u16 = 0_0 {
        /// Timer B7 Capture/Compare 3
        TBCCR3: 0..15 = struct TBCCR3Field(u16);
    }
    /// Timer B7 Capture/Compare 4
    rw TBCCR4 @ 0x7c: u16 = 0_0 {
        /// Timer B7 Capture/Compare 4
        TBCCR4: 0..15 = struct TBCCR4Field(u16);
    }
    /// Timer B7 Capture/Compare 5
    rw TBCCR5 @ 0x7e: u16 = 0_0 {
        /// Timer B7 Capture/Compare 5
        TBCCR5: 0..15 = struct TBCCR5Field(u16);
    }
    /// Timer B7 Capture/Compare 6
    rw TBCCR6 @ 0x80: u16 = 0_0 {
        /// Timer B7 Capture/Compare 6
        TBCCR6: 0..15 = struct TBCCR6Field(u16);
    }
}
