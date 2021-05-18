//! Timer B3

utils::periph! {
    /// Timer B3
    TimerB3;
    /// Timer B Interrupt Vector Word
    rw TBIV @ 0x00: u16 = 0_0 {
        /// Timer B Interrupt Vector Word
        TBIV: 0..15 = struct TBIVField(u16);
    }
    /// Timer B Control
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
    /// Timer B Capture/Compare Control 0
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
    /// Timer B Capture/Compare Control 1
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
    /// Timer B Capture/Compare Control 2
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
    /// Timer B Counter Register
    rw TBR @ 0x72: u16 = 0_0 {
        /// Timer B Counter Register
        TBR: 0..15 = struct TBRField(u16);
    }
    /// Timer B Capture/Compare 0
    rw TBCCR0 @ 0x74: u16 = 0_0 {
        /// Timer B Capture/Compare 0
        TBCCR0: 0..15 = struct TBCCR0Field(u16);
    }
    /// Timer B Capture/Compare 1
    rw TBCCR1 @ 0x76: u16 = 0_0 {
        /// Timer B Capture/Compare 1
        TBCCR1: 0..15 = struct TBCCR1Field(u16);
    }
    /// Timer B Capture/Compare 2
    rw TBCCR2 @ 0x78: u16 = 0_0 {
        /// Timer B Capture/Compare 2
        TBCCR2: 0..15 = struct TBCCR2Field(u16);
    }
}
