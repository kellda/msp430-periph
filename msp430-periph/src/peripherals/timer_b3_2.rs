//! Timer B3

utils::periph! {
    /// Timer B3
    TimerB3;
    /// Timer B3 Interrupt Vector Word
    rw IV @ 0x00: u16 = 0_0 {
        /// Timer B3 Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
    /// Timer B3 Control
    rw CTL @ 0x62: u16 = 0_0 {
        /// Timer B interrupt flag
        IFG: 0 = struct IFG(bool);
        /// Timer B interrupt enable
        IE: 1 = struct IE(bool);
        /// Timer B counter clear
        CLR: 2 = struct CLR(bool);
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
        SSEL: 8..9 = enum SSEL {
            /// Clock Source: TBCLK
            SSEL_0 = 0b00,
            /// Clock Source: ACLK
            SSEL_1 = 0b01,
            /// Clock Source: SMCLK
            SSEL_2 = 0b10,
            /// Clock Source: INCLK
            SSEL_3 = 0b11,
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
        CLGRP: 13..14 = enum CLGRP {
            /// Timer B Group: 0 - individually
            CLGRP_0 = 0b00,
            /// Timer B Group: 1 - 3 groups (1-2
            CLGRP_1 = 0b01,
            /// Timer B Group: 2 - 2 groups (1-3
            CLGRP_2 = 0b10,
            /// Timer B Group: 3 - 1 group (all)
            CLGRP_3 = 0b11,
        }
    }
    /// Timer B3 Capture/Compare Control 0
    rw CCTL0 @ 0x64: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C0CCIFG: 0 = struct C0CCIFG(bool);
        /// Capture/compare overflow flag
        C0COV: 1 = struct C0COV(bool);
        /// PWM Output signal if output mode 0
        C0OUT: 2 = struct C0OUT(bool);
        /// Capture input signal (read)
        C0CCI: 3 = struct C0CCI(bool);
        /// Capture/compare interrupt enable
        C0CCIE: 4 = struct C0CCIE(bool);
        /// Output mode 2
        C0OUTMOD: 5..7 = enum C0OUTMOD {
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
        C0CAP: 8 = struct C0CAP(bool);
        /// Compare latch load source 1
        C0CLLD: 9..10 = enum C0CLLD {
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
        C0SCS: 11 = struct C0SCS(bool);
        /// Capture input select 1
        C0CCIS: 12..13 = enum C0CCIS {
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
        C0CM: 14..15 = enum C0CM {
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
    /// Timer B3 Capture/Compare Control 1
    rw CCTL1 @ 0x66: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C1CCIFG: 0 = struct C1CCIFG(bool);
        /// Capture/compare overflow flag
        C1COV: 1 = struct C1COV(bool);
        /// PWM Output signal if output mode 0
        C1OUT: 2 = struct C1OUT(bool);
        /// Capture input signal (read)
        C1CCI: 3 = struct C1CCI(bool);
        /// Capture/compare interrupt enable
        C1CCIE: 4 = struct C1CCIE(bool);
        /// Output mode 2
        C1OUTMOD: 5..7 = enum C1OUTMOD {
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
        C1CAP: 8 = struct C1CAP(bool);
        /// Compare latch load source 1
        C1CLLD: 9..10 = enum C1CLLD {
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
        C1SCS: 11 = struct C1SCS(bool);
        /// Capture input select 1
        C1CCIS: 12..13 = enum C1CCIS {
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
        C1CM: 14..15 = enum C1CM {
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
    /// Timer B3 Capture/Compare Control 2
    rw CCTL2 @ 0x68: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C2CCIFG: 0 = struct C2CCIFG(bool);
        /// Capture/compare overflow flag
        C2COV: 1 = struct C2COV(bool);
        /// PWM Output signal if output mode 0
        C2OUT: 2 = struct C2OUT(bool);
        /// Capture input signal (read)
        C2CCI: 3 = struct C2CCI(bool);
        /// Capture/compare interrupt enable
        C2CCIE: 4 = struct C2CCIE(bool);
        /// Output mode 2
        C2OUTMOD: 5..7 = enum C2OUTMOD {
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
        C2CAP: 8 = struct C2CAP(bool);
        /// Compare latch load source 1
        C2CLLD: 9..10 = enum C2CLLD {
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
        C2SCS: 11 = struct C2SCS(bool);
        /// Capture input select 1
        C2CCIS: 12..13 = enum C2CCIS {
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
        C2CM: 14..15 = enum C2CM {
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
    /// Timer B3 Counter Register
    rw R @ 0x72: u16 = 0_0 {
        /// Timer B3 Counter Register
        R: 0..15 = struct RField(u16);
    }
    /// Timer B3 Capture/Compare 0
    rw CCR0 @ 0x74: u16 = 0_0 {
        /// Timer B3 Capture/Compare 0
        CCR0: 0..15 = struct CCR0Field(u16);
    }
    /// Timer B3 Capture/Compare 1
    rw CCR1 @ 0x76: u16 = 0_0 {
        /// Timer B3 Capture/Compare 1
        CCR1: 0..15 = struct CCR1Field(u16);
    }
    /// Timer B3 Capture/Compare 2
    rw CCR2 @ 0x78: u16 = 0_0 {
        /// Timer B3 Capture/Compare 2
        CCR2: 0..15 = struct CCR2Field(u16);
    }
}
