//! Timer0_B7

utils::periph! {
    /// Timer0_B7
    Timer0_B7;
    /// Timer0_B7 Control
    rw TB0CTL @ 0x00: u16 = 0_0 {
        /// Timer0_B7 interrupt flag
        TBIFG: 0 = struct TBIFG(bool);
        /// Timer0_B7 interrupt enable
        TBIE: 1 = struct TBIE(bool);
        /// Timer0_B7 counter clear
        TBCLR: 2 = struct TBCLR(bool);
        /// Timer0_B7 mode control 1
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
        /// Timer0_B7 clock input divider 1
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
        /// Timer0_B7 Compare latch load group 1
        TBCLGRP: 13..14 = enum TBCLGRP {
            /// Timer0_B7 Group: 0 - individually
            TBCLGRP_0 = 0b00,
            /// Timer0_B7 Group: 1 - 3 groups (1-2
            TBCLGRP_1 = 0b01,
            /// Timer0_B7 Group: 2 - 2 groups (1-3
            TBCLGRP_2 = 0b10,
            /// Timer0_B7 Group: 3 - 1 group (all)
            TBCLGRP_3 = 0b11,
        }
    }
    /// Timer0_B7 Capture/Compare Control 0
    rw TB0CCTL0 @ 0x02: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL0_CCIFG: 0 = struct TB0CCTL0_CCIFG(bool);
        /// Capture/compare overflow flag
        TB0CCTL0_COV: 1 = struct TB0CCTL0_COV(bool);
        /// PWM Output signal if output mode 0
        TB0CCTL0_OUT: 2 = struct TB0CCTL0_OUT(bool);
        /// Capture input signal (read)
        TB0CCTL0_CCI: 3 = struct TB0CCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL0_CCIE: 4 = struct TB0CCTL0_CCIE(bool);
        /// Output mode 2
        TB0CCTL0_OUTMOD: 5..7 = enum TB0CCTL0_OUTMOD {
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
        TB0CCTL0_CAP: 8 = struct TB0CCTL0_CAP(bool);
        /// Compare latch load source 1
        TB0CCTL0_CLLD: 9..10 = enum TB0CCTL0_CLLD {
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
        TB0CCTL0_SCS: 11 = struct TB0CCTL0_SCS(bool);
        /// Capture input select 1
        TB0CCTL0_CCIS: 12..13 = enum TB0CCTL0_CCIS {
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
        TB0CCTL0_CM: 14..15 = enum TB0CCTL0_CM {
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
    /// Timer0_B7 Capture/Compare Control 1
    rw TB0CCTL1 @ 0x04: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL1_CCIFG: 0 = struct TB0CCTL1_CCIFG(bool);
        /// Capture/compare overflow flag
        TB0CCTL1_COV: 1 = struct TB0CCTL1_COV(bool);
        /// PWM Output signal if output mode 0
        TB0CCTL1_OUT: 2 = struct TB0CCTL1_OUT(bool);
        /// Capture input signal (read)
        TB0CCTL1_CCI: 3 = struct TB0CCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL1_CCIE: 4 = struct TB0CCTL1_CCIE(bool);
        /// Output mode 2
        TB0CCTL1_OUTMOD: 5..7 = enum TB0CCTL1_OUTMOD {
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
        TB0CCTL1_CAP: 8 = struct TB0CCTL1_CAP(bool);
        /// Compare latch load source 1
        TB0CCTL1_CLLD: 9..10 = enum TB0CCTL1_CLLD {
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
        TB0CCTL1_SCS: 11 = struct TB0CCTL1_SCS(bool);
        /// Capture input select 1
        TB0CCTL1_CCIS: 12..13 = enum TB0CCTL1_CCIS {
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
        TB0CCTL1_CM: 14..15 = enum TB0CCTL1_CM {
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
    /// Timer0_B7 Capture/Compare Control 2
    rw TB0CCTL2 @ 0x06: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL2_CCIFG: 0 = struct TB0CCTL2_CCIFG(bool);
        /// Capture/compare overflow flag
        TB0CCTL2_COV: 1 = struct TB0CCTL2_COV(bool);
        /// PWM Output signal if output mode 0
        TB0CCTL2_OUT: 2 = struct TB0CCTL2_OUT(bool);
        /// Capture input signal (read)
        TB0CCTL2_CCI: 3 = struct TB0CCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL2_CCIE: 4 = struct TB0CCTL2_CCIE(bool);
        /// Output mode 2
        TB0CCTL2_OUTMOD: 5..7 = enum TB0CCTL2_OUTMOD {
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
        TB0CCTL2_CAP: 8 = struct TB0CCTL2_CAP(bool);
        /// Compare latch load source 1
        TB0CCTL2_CLLD: 9..10 = enum TB0CCTL2_CLLD {
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
        TB0CCTL2_SCS: 11 = struct TB0CCTL2_SCS(bool);
        /// Capture input select 1
        TB0CCTL2_CCIS: 12..13 = enum TB0CCTL2_CCIS {
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
        TB0CCTL2_CM: 14..15 = enum TB0CCTL2_CM {
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
    /// Timer0_B7 Capture/Compare Control 3
    rw TB0CCTL3 @ 0x08: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL3_CCIFG: 0 = struct TB0CCTL3_CCIFG(bool);
        /// Capture/compare overflow flag
        TB0CCTL3_COV: 1 = struct TB0CCTL3_COV(bool);
        /// PWM Output signal if output mode 0
        TB0CCTL3_OUT: 2 = struct TB0CCTL3_OUT(bool);
        /// Capture input signal (read)
        TB0CCTL3_CCI: 3 = struct TB0CCTL3_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL3_CCIE: 4 = struct TB0CCTL3_CCIE(bool);
        /// Output mode 2
        TB0CCTL3_OUTMOD: 5..7 = enum TB0CCTL3_OUTMOD {
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
        TB0CCTL3_CAP: 8 = struct TB0CCTL3_CAP(bool);
        /// Compare latch load source 1
        TB0CCTL3_CLLD: 9..10 = enum TB0CCTL3_CLLD {
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
        TB0CCTL3_SCS: 11 = struct TB0CCTL3_SCS(bool);
        /// Capture input select 1
        TB0CCTL3_CCIS: 12..13 = enum TB0CCTL3_CCIS {
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
        TB0CCTL3_CM: 14..15 = enum TB0CCTL3_CM {
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
    /// Timer0_B7 Capture/Compare Control 4
    rw TB0CCTL4 @ 0x0a: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL4_CCIFG: 0 = struct TB0CCTL4_CCIFG(bool);
        /// Capture/compare overflow flag
        TB0CCTL4_COV: 1 = struct TB0CCTL4_COV(bool);
        /// PWM Output signal if output mode 0
        TB0CCTL4_OUT: 2 = struct TB0CCTL4_OUT(bool);
        /// Capture input signal (read)
        TB0CCTL4_CCI: 3 = struct TB0CCTL4_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL4_CCIE: 4 = struct TB0CCTL4_CCIE(bool);
        /// Output mode 2
        TB0CCTL4_OUTMOD: 5..7 = enum TB0CCTL4_OUTMOD {
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
        TB0CCTL4_CAP: 8 = struct TB0CCTL4_CAP(bool);
        /// Compare latch load source 1
        TB0CCTL4_CLLD: 9..10 = enum TB0CCTL4_CLLD {
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
        TB0CCTL4_SCS: 11 = struct TB0CCTL4_SCS(bool);
        /// Capture input select 1
        TB0CCTL4_CCIS: 12..13 = enum TB0CCTL4_CCIS {
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
        TB0CCTL4_CM: 14..15 = enum TB0CCTL4_CM {
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
    /// Timer0_B7 Capture/Compare Control 5
    rw TB0CCTL5 @ 0x0c: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL5_CCIFG: 0 = struct TB0CCTL5_CCIFG(bool);
        /// Capture/compare overflow flag
        TB0CCTL5_COV: 1 = struct TB0CCTL5_COV(bool);
        /// PWM Output signal if output mode 0
        TB0CCTL5_OUT: 2 = struct TB0CCTL5_OUT(bool);
        /// Capture input signal (read)
        TB0CCTL5_CCI: 3 = struct TB0CCTL5_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL5_CCIE: 4 = struct TB0CCTL5_CCIE(bool);
        /// Output mode 2
        TB0CCTL5_OUTMOD: 5..7 = enum TB0CCTL5_OUTMOD {
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
        TB0CCTL5_CAP: 8 = struct TB0CCTL5_CAP(bool);
        /// Compare latch load source 1
        TB0CCTL5_CLLD: 9..10 = enum TB0CCTL5_CLLD {
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
        TB0CCTL5_SCS: 11 = struct TB0CCTL5_SCS(bool);
        /// Capture input select 1
        TB0CCTL5_CCIS: 12..13 = enum TB0CCTL5_CCIS {
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
        TB0CCTL5_CM: 14..15 = enum TB0CCTL5_CM {
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
    /// Timer0_B7 Capture/Compare Control 6
    rw TB0CCTL6 @ 0x0e: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL6_CCIFG: 0 = struct TB0CCTL6_CCIFG(bool);
        /// Capture/compare overflow flag
        TB0CCTL6_COV: 1 = struct TB0CCTL6_COV(bool);
        /// PWM Output signal if output mode 0
        TB0CCTL6_OUT: 2 = struct TB0CCTL6_OUT(bool);
        /// Capture input signal (read)
        TB0CCTL6_CCI: 3 = struct TB0CCTL6_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL6_CCIE: 4 = struct TB0CCTL6_CCIE(bool);
        /// Output mode 2
        TB0CCTL6_OUTMOD: 5..7 = enum TB0CCTL6_OUTMOD {
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
        TB0CCTL6_CAP: 8 = struct TB0CCTL6_CAP(bool);
        /// Compare latch load source 1
        TB0CCTL6_CLLD: 9..10 = enum TB0CCTL6_CLLD {
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
        TB0CCTL6_SCS: 11 = struct TB0CCTL6_SCS(bool);
        /// Capture input select 1
        TB0CCTL6_CCIS: 12..13 = enum TB0CCTL6_CCIS {
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
        TB0CCTL6_CM: 14..15 = enum TB0CCTL6_CM {
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
    /// Timer0_B7
    rw TB0R @ 0x10: u16 = 0_0 {
        /// Timer0_B7
        TB0R: 0..15 = struct TB0RField(u16);
    }
    /// Timer0_B7 Capture/Compare 0
    rw TB0CCR0 @ 0x12: u16 = 0_0 {
        /// Timer0_B7 Capture/Compare 0
        TB0CCR0: 0..15 = struct TB0CCR0Field(u16);
    }
    /// Timer0_B7 Capture/Compare 1
    rw TB0CCR1 @ 0x14: u16 = 0_0 {
        /// Timer0_B7 Capture/Compare 1
        TB0CCR1: 0..15 = struct TB0CCR1Field(u16);
    }
    /// Timer0_B7 Capture/Compare 2
    rw TB0CCR2 @ 0x16: u16 = 0_0 {
        /// Timer0_B7 Capture/Compare 2
        TB0CCR2: 0..15 = struct TB0CCR2Field(u16);
    }
    /// Timer0_B7 Capture/Compare 3
    rw TB0CCR3 @ 0x18: u16 = 0_0 {
        /// Timer0_B7 Capture/Compare 3
        TB0CCR3: 0..15 = struct TB0CCR3Field(u16);
    }
    /// Timer0_B7 Capture/Compare 4
    rw TB0CCR4 @ 0x1a: u16 = 0_0 {
        /// Timer0_B7 Capture/Compare 4
        TB0CCR4: 0..15 = struct TB0CCR4Field(u16);
    }
    /// Timer0_B7 Capture/Compare 5
    rw TB0CCR5 @ 0x1c: u16 = 0_0 {
        /// Timer0_B7 Capture/Compare 5
        TB0CCR5: 0..15 = struct TB0CCR5Field(u16);
    }
    /// Timer0_B7 Capture/Compare 6
    rw TB0CCR6 @ 0x1e: u16 = 0_0 {
        /// Timer0_B7 Capture/Compare 6
        TB0CCR6: 0..15 = struct TB0CCR6Field(u16);
    }
    /// Timer0_B7 Expansion Register 0
    rw TB0EX0 @ 0x20: u16 = 0_0 {
        /// Timer0_B7 Input divider expansion Bit: 0
        TBIDEX: 0..2 = enum TBIDEX {
            /// Timer0_B7 Input divider expansion : /1
            TBIDEX_0 = 0b000,
            /// Timer0_B7 Input divider expansion : /2
            TBIDEX_1 = 0b001,
            /// Timer0_B7 Input divider expansion : /3
            TBIDEX_2 = 0b010,
            /// Timer0_B7 Input divider expansion : /4
            TBIDEX_3 = 0b011,
            /// Timer0_B7 Input divider expansion : /5
            TBIDEX_4 = 0b100,
            /// Timer0_B7 Input divider expansion : /6
            TBIDEX_5 = 0b101,
            /// Timer0_B7 Input divider expansion : /7
            TBIDEX_6 = 0b110,
            /// Timer0_B7 Input divider expansion : /8
            TBIDEX_7 = 0b111,
        }
    }
    /// Timer0_B7 Interrupt Vector Word
    rw TB0IV @ 0x2e: u16 = 0_0 {
        /// Timer0_B7 Interrupt Vector Word
        TB0IV: 0..15 = struct TB0IVField(u16);
    }
}
