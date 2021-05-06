//! Timer1_A3

utils::periph! {
    /// Timer1_A3
    Timer1_A3;
    /// Timer1_A3 Interrupt Vector Word
    rw TA1IV @ 0x00: u16 = 0_0 {
        /// Timer1_A3 Interrupt Vector Word
        TA1IV: 0..15 = struct TA1IVField(u16);
    }
    /// Timer1_A3 Control
    rw TA1CTL @ 0x62: u16 = 0_0 {
        /// Timer A counter interrupt flag
        TAIFG: 0 = struct TAIFG(bool);
        /// Timer A counter interrupt enable
        TAIE: 1 = struct TAIE(bool);
        /// Timer A counter clear
        TACLR: 2 = struct TACLR(bool);
        /// Timer A mode control 1
        MC: 4..5 = enum MC {
            /// Timer A mode control: 0 - Stop
            MC_0 = 0b00,
            /// Timer A mode control: 1 - Up to CCR0
            MC_1 = 0b01,
            /// Timer A mode control: 2 - Continous up
            MC_2 = 0b10,
            /// Timer A mode control: 3 - Up/Down
            MC_3 = 0b11,
        }
        /// Timer A clock input divider 1
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
        /// Timer A clock source select 1
        TASSEL: 8..9 = enum TASSEL {
            /// Timer A clock source select: 0 - TACLK
            TASSEL_0 = 0b00,
            /// Timer A clock source select: 1 - ACLK
            TASSEL_1 = 0b01,
            /// Timer A clock source select: 2 - SMCLK
            TASSEL_2 = 0b10,
            /// Timer A clock source select: 3 - INCLK
            TASSEL_3 = 0b11,
        }
    }
    /// Timer1_A3 Capture/Compare Control 0
    rw TA1CCTL0 @ 0x64: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL0_CCIFG: 0 = struct TA1CCTL0_CCIFG(bool);
        /// Capture/compare overflow flag
        TA1CCTL0_COV: 1 = struct TA1CCTL0_COV(bool);
        /// PWM Output signal if output mode 0
        TA1CCTL0_OUT: 2 = struct TA1CCTL0_OUT(bool);
        /// Capture input signal (read)
        TA1CCTL0_CCI: 3 = struct TA1CCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL0_CCIE: 4 = struct TA1CCTL0_CCIE(bool);
        /// Output mode 2
        TA1CCTL0_OUTMOD: 5..7 = enum TA1CCTL0_OUTMOD {
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
        TA1CCTL0_CAP: 8 = struct TA1CCTL0_CAP(bool);
        /// Latched capture signal (read)
        TA1CCTL0_SCCI: 10 = struct TA1CCTL0_SCCI(bool);
        /// Capture sychronize
        TA1CCTL0_SCS: 11 = struct TA1CCTL0_SCS(bool);
        /// Capture input select 1
        TA1CCTL0_CCIS: 12..13 = enum TA1CCTL0_CCIS {
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
        TA1CCTL0_CM: 14..15 = enum TA1CCTL0_CM {
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
    /// Timer1_A3 Capture/Compare Control 1
    rw TA1CCTL1 @ 0x66: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL1_CCIFG: 0 = struct TA1CCTL1_CCIFG(bool);
        /// Capture/compare overflow flag
        TA1CCTL1_COV: 1 = struct TA1CCTL1_COV(bool);
        /// PWM Output signal if output mode 0
        TA1CCTL1_OUT: 2 = struct TA1CCTL1_OUT(bool);
        /// Capture input signal (read)
        TA1CCTL1_CCI: 3 = struct TA1CCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL1_CCIE: 4 = struct TA1CCTL1_CCIE(bool);
        /// Output mode 2
        TA1CCTL1_OUTMOD: 5..7 = enum TA1CCTL1_OUTMOD {
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
        TA1CCTL1_CAP: 8 = struct TA1CCTL1_CAP(bool);
        /// Latched capture signal (read)
        TA1CCTL1_SCCI: 10 = struct TA1CCTL1_SCCI(bool);
        /// Capture sychronize
        TA1CCTL1_SCS: 11 = struct TA1CCTL1_SCS(bool);
        /// Capture input select 1
        TA1CCTL1_CCIS: 12..13 = enum TA1CCTL1_CCIS {
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
        TA1CCTL1_CM: 14..15 = enum TA1CCTL1_CM {
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
    /// Timer1_A3 Capture/Compare Control 2
    rw TA1CCTL2 @ 0x68: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL2_CCIFG: 0 = struct TA1CCTL2_CCIFG(bool);
        /// Capture/compare overflow flag
        TA1CCTL2_COV: 1 = struct TA1CCTL2_COV(bool);
        /// PWM Output signal if output mode 0
        TA1CCTL2_OUT: 2 = struct TA1CCTL2_OUT(bool);
        /// Capture input signal (read)
        TA1CCTL2_CCI: 3 = struct TA1CCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL2_CCIE: 4 = struct TA1CCTL2_CCIE(bool);
        /// Output mode 2
        TA1CCTL2_OUTMOD: 5..7 = enum TA1CCTL2_OUTMOD {
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
        TA1CCTL2_CAP: 8 = struct TA1CCTL2_CAP(bool);
        /// Latched capture signal (read)
        TA1CCTL2_SCCI: 10 = struct TA1CCTL2_SCCI(bool);
        /// Capture sychronize
        TA1CCTL2_SCS: 11 = struct TA1CCTL2_SCS(bool);
        /// Capture input select 1
        TA1CCTL2_CCIS: 12..13 = enum TA1CCTL2_CCIS {
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
        TA1CCTL2_CM: 14..15 = enum TA1CCTL2_CM {
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
    /// Timer1_A3 Counter Register
    rw TA1R @ 0x72: u16 = 0_0 {
        /// Timer1_A3 Counter Register
        TA1R: 0..15 = struct TA1RField(u16);
    }
    /// Timer1_A3 Capture/Compare 0
    rw TA1CCR0 @ 0x74: u16 = 0_0 {
        /// Timer1_A3 Capture/Compare 0
        TA1CCR0: 0..15 = struct TA1CCR0Field(u16);
    }
    /// Timer1_A3 Capture/Compare 1
    rw TA1CCR1 @ 0x76: u16 = 0_0 {
        /// Timer1_A3 Capture/Compare 1
        TA1CCR1: 0..15 = struct TA1CCR1Field(u16);
    }
    /// Timer1_A3 Capture/Compare 2
    rw TA1CCR2 @ 0x78: u16 = 0_0 {
        /// Timer1_A3 Capture/Compare 2
        TA1CCR2: 0..15 = struct TA1CCR2Field(u16);
    }
}
