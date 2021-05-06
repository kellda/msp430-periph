//! Timer0_A5

utils::periph! {
    /// Timer0_A5
    Timer0_A5;
    /// Timer0_A5 Control
    rw TA0CTL @ 0x00: u16 = 0_0 {
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
    /// Timer0_A5 Capture/Compare Control 0
    rw TA0CCTL0 @ 0x02: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA0CCTL0_CCIFG: 0 = struct TA0CCTL0_CCIFG(bool);
        /// Capture/compare overflow flag
        TA0CCTL0_COV: 1 = struct TA0CCTL0_COV(bool);
        /// PWM Output signal if output mode 0
        TA0CCTL0_OUT: 2 = struct TA0CCTL0_OUT(bool);
        /// Capture input signal (read)
        TA0CCTL0_CCI: 3 = struct TA0CCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TA0CCTL0_CCIE: 4 = struct TA0CCTL0_CCIE(bool);
        /// Output mode 2
        TA0CCTL0_OUTMOD: 5..7 = enum TA0CCTL0_OUTMOD {
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
        TA0CCTL0_CAP: 8 = struct TA0CCTL0_CAP(bool);
        /// Latched capture signal (read)
        TA0CCTL0_SCCI: 10 = struct TA0CCTL0_SCCI(bool);
        /// Capture sychronize
        TA0CCTL0_SCS: 11 = struct TA0CCTL0_SCS(bool);
        /// Capture input select 1
        TA0CCTL0_CCIS: 12..13 = enum TA0CCTL0_CCIS {
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
        TA0CCTL0_CM: 14..15 = enum TA0CCTL0_CM {
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
    /// Timer0_A5 Capture/Compare Control 1
    rw TA0CCTL1 @ 0x04: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA0CCTL1_CCIFG: 0 = struct TA0CCTL1_CCIFG(bool);
        /// Capture/compare overflow flag
        TA0CCTL1_COV: 1 = struct TA0CCTL1_COV(bool);
        /// PWM Output signal if output mode 0
        TA0CCTL1_OUT: 2 = struct TA0CCTL1_OUT(bool);
        /// Capture input signal (read)
        TA0CCTL1_CCI: 3 = struct TA0CCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TA0CCTL1_CCIE: 4 = struct TA0CCTL1_CCIE(bool);
        /// Output mode 2
        TA0CCTL1_OUTMOD: 5..7 = enum TA0CCTL1_OUTMOD {
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
        TA0CCTL1_CAP: 8 = struct TA0CCTL1_CAP(bool);
        /// Latched capture signal (read)
        TA0CCTL1_SCCI: 10 = struct TA0CCTL1_SCCI(bool);
        /// Capture sychronize
        TA0CCTL1_SCS: 11 = struct TA0CCTL1_SCS(bool);
        /// Capture input select 1
        TA0CCTL1_CCIS: 12..13 = enum TA0CCTL1_CCIS {
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
        TA0CCTL1_CM: 14..15 = enum TA0CCTL1_CM {
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
    /// Timer0_A5 Capture/Compare Control 2
    rw TA0CCTL2 @ 0x06: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA0CCTL2_CCIFG: 0 = struct TA0CCTL2_CCIFG(bool);
        /// Capture/compare overflow flag
        TA0CCTL2_COV: 1 = struct TA0CCTL2_COV(bool);
        /// PWM Output signal if output mode 0
        TA0CCTL2_OUT: 2 = struct TA0CCTL2_OUT(bool);
        /// Capture input signal (read)
        TA0CCTL2_CCI: 3 = struct TA0CCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TA0CCTL2_CCIE: 4 = struct TA0CCTL2_CCIE(bool);
        /// Output mode 2
        TA0CCTL2_OUTMOD: 5..7 = enum TA0CCTL2_OUTMOD {
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
        TA0CCTL2_CAP: 8 = struct TA0CCTL2_CAP(bool);
        /// Latched capture signal (read)
        TA0CCTL2_SCCI: 10 = struct TA0CCTL2_SCCI(bool);
        /// Capture sychronize
        TA0CCTL2_SCS: 11 = struct TA0CCTL2_SCS(bool);
        /// Capture input select 1
        TA0CCTL2_CCIS: 12..13 = enum TA0CCTL2_CCIS {
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
        TA0CCTL2_CM: 14..15 = enum TA0CCTL2_CM {
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
    /// Timer0_A5 Capture/Compare Control 3
    rw TA0CCTL3 @ 0x08: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA0CCTL3_CCIFG: 0 = struct TA0CCTL3_CCIFG(bool);
        /// Capture/compare overflow flag
        TA0CCTL3_COV: 1 = struct TA0CCTL3_COV(bool);
        /// PWM Output signal if output mode 0
        TA0CCTL3_OUT: 2 = struct TA0CCTL3_OUT(bool);
        /// Capture input signal (read)
        TA0CCTL3_CCI: 3 = struct TA0CCTL3_CCI(bool);
        /// Capture/compare interrupt enable
        TA0CCTL3_CCIE: 4 = struct TA0CCTL3_CCIE(bool);
        /// Output mode 2
        TA0CCTL3_OUTMOD: 5..7 = enum TA0CCTL3_OUTMOD {
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
        TA0CCTL3_CAP: 8 = struct TA0CCTL3_CAP(bool);
        /// Latched capture signal (read)
        TA0CCTL3_SCCI: 10 = struct TA0CCTL3_SCCI(bool);
        /// Capture sychronize
        TA0CCTL3_SCS: 11 = struct TA0CCTL3_SCS(bool);
        /// Capture input select 1
        TA0CCTL3_CCIS: 12..13 = enum TA0CCTL3_CCIS {
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
        TA0CCTL3_CM: 14..15 = enum TA0CCTL3_CM {
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
    /// Timer0_A5 Capture/Compare Control 4
    rw TA0CCTL4 @ 0x0a: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA0CCTL4_CCIFG: 0 = struct TA0CCTL4_CCIFG(bool);
        /// Capture/compare overflow flag
        TA0CCTL4_COV: 1 = struct TA0CCTL4_COV(bool);
        /// PWM Output signal if output mode 0
        TA0CCTL4_OUT: 2 = struct TA0CCTL4_OUT(bool);
        /// Capture input signal (read)
        TA0CCTL4_CCI: 3 = struct TA0CCTL4_CCI(bool);
        /// Capture/compare interrupt enable
        TA0CCTL4_CCIE: 4 = struct TA0CCTL4_CCIE(bool);
        /// Output mode 2
        TA0CCTL4_OUTMOD: 5..7 = enum TA0CCTL4_OUTMOD {
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
        TA0CCTL4_CAP: 8 = struct TA0CCTL4_CAP(bool);
        /// Latched capture signal (read)
        TA0CCTL4_SCCI: 10 = struct TA0CCTL4_SCCI(bool);
        /// Capture sychronize
        TA0CCTL4_SCS: 11 = struct TA0CCTL4_SCS(bool);
        /// Capture input select 1
        TA0CCTL4_CCIS: 12..13 = enum TA0CCTL4_CCIS {
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
        TA0CCTL4_CM: 14..15 = enum TA0CCTL4_CM {
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
    /// Timer0_A5
    rw TA0R @ 0x10: u16 = 0_0 {
        /// Timer0_A5
        TA0R: 0..15 = struct TA0RField(u16);
    }
    /// Timer0_A5 Capture/Compare 0
    rw TA0CCR0 @ 0x12: u16 = 0_0 {
        /// Timer0_A5 Capture/Compare 0
        TA0CCR0: 0..15 = struct TA0CCR0Field(u16);
    }
    /// Timer0_A5 Capture/Compare 1
    rw TA0CCR1 @ 0x14: u16 = 0_0 {
        /// Timer0_A5 Capture/Compare 1
        TA0CCR1: 0..15 = struct TA0CCR1Field(u16);
    }
    /// Timer0_A5 Capture/Compare 2
    rw TA0CCR2 @ 0x16: u16 = 0_0 {
        /// Timer0_A5 Capture/Compare 2
        TA0CCR2: 0..15 = struct TA0CCR2Field(u16);
    }
    /// Timer0_A5 Capture/Compare 3
    rw TA0CCR3 @ 0x18: u16 = 0_0 {
        /// Timer0_A5 Capture/Compare 3
        TA0CCR3: 0..15 = struct TA0CCR3Field(u16);
    }
    /// Timer0_A5 Capture/Compare 4
    rw TA0CCR4 @ 0x1a: u16 = 0_0 {
        /// Timer0_A5 Capture/Compare 4
        TA0CCR4: 0..15 = struct TA0CCR4Field(u16);
    }
    /// Timer0_A5 Interrupt Vector Word
    rw TA0IV @ 0x2e: u16 = 0_0 {
        /// Timer0_A5 Interrupt Vector Word
        TA0IV: 0..15 = struct TA0IVField(u16);
    }
    /// Timer0_A5 Expansion Register 0
    rw TA0EX0 @ 0x20: u16 = 0_0 {
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
