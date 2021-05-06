//! Timer2_A3

utils::periph! {
    /// Timer2_A3
    Timer2_A3;
    /// Timer2_A3 Control
    rw TA2CTL @ 0x00: u16 = 0_0 {
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
            /// Timer A mode control: 2 - Continuous up
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
    /// Timer2_A3 Capture/Compare Control 0
    rw TA2CCTL0 @ 0x02: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA2CCTL0_CCIFG: 0 = struct TA2CCTL0_CCIFG(bool);
        /// Capture/compare overflow flag
        TA2CCTL0_COV: 1 = struct TA2CCTL0_COV(bool);
        /// PWM Output signal if output mode 0
        TA2CCTL0_OUT: 2 = struct TA2CCTL0_OUT(bool);
        /// Capture input signal (read)
        TA2CCTL0_CCI: 3 = struct TA2CCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TA2CCTL0_CCIE: 4 = struct TA2CCTL0_CCIE(bool);
        /// Output mode 2
        TA2CCTL0_OUTMOD: 5..7 = enum TA2CCTL0_OUTMOD {
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
        TA2CCTL0_CAP: 8 = struct TA2CCTL0_CAP(bool);
        /// Latched capture signal (read)
        TA2CCTL0_SCCI: 10 = struct TA2CCTL0_SCCI(bool);
        /// Capture sychronize
        TA2CCTL0_SCS: 11 = struct TA2CCTL0_SCS(bool);
        /// Capture input select 1
        TA2CCTL0_CCIS: 12..13 = enum TA2CCTL0_CCIS {
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
        TA2CCTL0_CM: 14..15 = enum TA2CCTL0_CM {
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
    /// Timer2_A3 Capture/Compare Control 1
    rw TA2CCTL1 @ 0x04: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA2CCTL1_CCIFG: 0 = struct TA2CCTL1_CCIFG(bool);
        /// Capture/compare overflow flag
        TA2CCTL1_COV: 1 = struct TA2CCTL1_COV(bool);
        /// PWM Output signal if output mode 0
        TA2CCTL1_OUT: 2 = struct TA2CCTL1_OUT(bool);
        /// Capture input signal (read)
        TA2CCTL1_CCI: 3 = struct TA2CCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TA2CCTL1_CCIE: 4 = struct TA2CCTL1_CCIE(bool);
        /// Output mode 2
        TA2CCTL1_OUTMOD: 5..7 = enum TA2CCTL1_OUTMOD {
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
        TA2CCTL1_CAP: 8 = struct TA2CCTL1_CAP(bool);
        /// Latched capture signal (read)
        TA2CCTL1_SCCI: 10 = struct TA2CCTL1_SCCI(bool);
        /// Capture sychronize
        TA2CCTL1_SCS: 11 = struct TA2CCTL1_SCS(bool);
        /// Capture input select 1
        TA2CCTL1_CCIS: 12..13 = enum TA2CCTL1_CCIS {
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
        TA2CCTL1_CM: 14..15 = enum TA2CCTL1_CM {
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
    /// Timer2_A3 Capture/Compare Control 2
    rw TA2CCTL2 @ 0x06: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA2CCTL2_CCIFG: 0 = struct TA2CCTL2_CCIFG(bool);
        /// Capture/compare overflow flag
        TA2CCTL2_COV: 1 = struct TA2CCTL2_COV(bool);
        /// PWM Output signal if output mode 0
        TA2CCTL2_OUT: 2 = struct TA2CCTL2_OUT(bool);
        /// Capture input signal (read)
        TA2CCTL2_CCI: 3 = struct TA2CCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TA2CCTL2_CCIE: 4 = struct TA2CCTL2_CCIE(bool);
        /// Output mode 2
        TA2CCTL2_OUTMOD: 5..7 = enum TA2CCTL2_OUTMOD {
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
        TA2CCTL2_CAP: 8 = struct TA2CCTL2_CAP(bool);
        /// Latched capture signal (read)
        TA2CCTL2_SCCI: 10 = struct TA2CCTL2_SCCI(bool);
        /// Capture sychronize
        TA2CCTL2_SCS: 11 = struct TA2CCTL2_SCS(bool);
        /// Capture input select 1
        TA2CCTL2_CCIS: 12..13 = enum TA2CCTL2_CCIS {
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
        TA2CCTL2_CM: 14..15 = enum TA2CCTL2_CM {
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
    /// Timer2_A3
    rw TA2R @ 0x10: u16 = 0_0 {
        /// Timer2_A3
        TA2R: 0..15 = struct TA2RField(u16);
    }
    /// Timer2_A3 Capture/Compare 0
    rw TA2CCR0 @ 0x12: u16 = 0_0 {
        /// Timer2_A3 Capture/Compare 0
        TA2CCR0: 0..15 = struct TA2CCR0Field(u16);
    }
    /// Timer2_A3 Capture/Compare 1
    rw TA2CCR1 @ 0x14: u16 = 0_0 {
        /// Timer2_A3 Capture/Compare 1
        TA2CCR1: 0..15 = struct TA2CCR1Field(u16);
    }
    /// Timer2_A3 Capture/Compare 2
    rw TA2CCR2 @ 0x16: u16 = 0_0 {
        /// Timer2_A3 Capture/Compare 2
        TA2CCR2: 0..15 = struct TA2CCR2Field(u16);
    }
    /// Timer2_A3 Interrupt Vector Word
    rw TA2IV @ 0x2e: u16 = 0_0 {
        /// Timer2_A3 Interrupt Vector Word
        TA2IV: 0..15 = struct TA2IVField(u16);
    }
    /// Timer2_A3 Expansion Register 0
    rw TA2EX0 @ 0x20: u16 = 0_0 {
        /// Timer A Input divider expansion Bit: 0
        TAIDEX: 0..2 = enum TAIDEX {
            /// Timer A Input divider expansion : /1
            TAIDEX_0 = 0b000,
            /// Timer A Input divider expansion : /2
            TAIDEX_1 = 0b001,
            /// Timer A Input divider expansion : /3
            TAIDEX_2 = 0b010,
            /// Timer A Input divider expansion : /4
            TAIDEX_3 = 0b011,
            /// Timer A Input divider expansion : /5
            TAIDEX_4 = 0b100,
            /// Timer A Input divider expansion : /6
            TAIDEX_5 = 0b101,
            /// Timer A Input divider expansion : /7
            TAIDEX_6 = 0b110,
            /// Timer A Input divider expansion : /8
            TAIDEX_7 = 0b111,
        }
    }
}
