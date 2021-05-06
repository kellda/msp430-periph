//! Timer A5

utils::periph! {
    /// Timer A5
    TimerA5;
    /// Timer A Interrupt Vector Word
    rw TAIV @ 0x00: u16 = 0_0 {
        /// Timer A Interrupt Vector Word
        TAIV: 0..15 = struct TAIVField(u16);
    }
    /// Timer A Control
    rw TACTL @ 0x32: u16 = 0_0 {
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
    /// Timer A Capture/Compare Control 0
    rw TACCTL0 @ 0x34: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TACCTL0_CCIFG: 0 = struct TACCTL0_CCIFG(bool);
        /// Capture/compare overflow flag
        TACCTL0_COV: 1 = struct TACCTL0_COV(bool);
        /// PWM Output signal if output mode 0
        TACCTL0_OUT: 2 = struct TACCTL0_OUT(bool);
        /// Capture input signal (read)
        TACCTL0_CCI: 3 = struct TACCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TACCTL0_CCIE: 4 = struct TACCTL0_CCIE(bool);
        /// Output mode 2
        TACCTL0_OUTMOD: 5..7 = enum TACCTL0_OUTMOD {
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
        TACCTL0_CAP: 8 = struct TACCTL0_CAP(bool);
        /// Latched capture signal (read)
        TACCTL0_SCCI: 10 = struct TACCTL0_SCCI(bool);
        /// Capture sychronize
        TACCTL0_SCS: 11 = struct TACCTL0_SCS(bool);
        /// Capture input select 1
        TACCTL0_CCIS: 12..13 = enum TACCTL0_CCIS {
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
        TACCTL0_CM: 14..15 = enum TACCTL0_CM {
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
    /// Timer A Capture/Compare Control 1
    rw TACCTL1 @ 0x36: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TACCTL1_CCIFG: 0 = struct TACCTL1_CCIFG(bool);
        /// Capture/compare overflow flag
        TACCTL1_COV: 1 = struct TACCTL1_COV(bool);
        /// PWM Output signal if output mode 0
        TACCTL1_OUT: 2 = struct TACCTL1_OUT(bool);
        /// Capture input signal (read)
        TACCTL1_CCI: 3 = struct TACCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TACCTL1_CCIE: 4 = struct TACCTL1_CCIE(bool);
        /// Output mode 2
        TACCTL1_OUTMOD: 5..7 = enum TACCTL1_OUTMOD {
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
        TACCTL1_CAP: 8 = struct TACCTL1_CAP(bool);
        /// Latched capture signal (read)
        TACCTL1_SCCI: 10 = struct TACCTL1_SCCI(bool);
        /// Capture sychronize
        TACCTL1_SCS: 11 = struct TACCTL1_SCS(bool);
        /// Capture input select 1
        TACCTL1_CCIS: 12..13 = enum TACCTL1_CCIS {
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
        TACCTL1_CM: 14..15 = enum TACCTL1_CM {
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
    /// Timer A Capture/Compare Control 2
    rw TACCTL2 @ 0x38: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TACCTL2_CCIFG: 0 = struct TACCTL2_CCIFG(bool);
        /// Capture/compare overflow flag
        TACCTL2_COV: 1 = struct TACCTL2_COV(bool);
        /// PWM Output signal if output mode 0
        TACCTL2_OUT: 2 = struct TACCTL2_OUT(bool);
        /// Capture input signal (read)
        TACCTL2_CCI: 3 = struct TACCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TACCTL2_CCIE: 4 = struct TACCTL2_CCIE(bool);
        /// Output mode 2
        TACCTL2_OUTMOD: 5..7 = enum TACCTL2_OUTMOD {
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
        TACCTL2_CAP: 8 = struct TACCTL2_CAP(bool);
        /// Latched capture signal (read)
        TACCTL2_SCCI: 10 = struct TACCTL2_SCCI(bool);
        /// Capture sychronize
        TACCTL2_SCS: 11 = struct TACCTL2_SCS(bool);
        /// Capture input select 1
        TACCTL2_CCIS: 12..13 = enum TACCTL2_CCIS {
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
        TACCTL2_CM: 14..15 = enum TACCTL2_CM {
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
    /// Timer A Capture/Compare Control 3
    rw TACCTL3 @ 0x3a: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TACCTL3_CCIFG: 0 = struct TACCTL3_CCIFG(bool);
        /// Capture/compare overflow flag
        TACCTL3_COV: 1 = struct TACCTL3_COV(bool);
        /// PWM Output signal if output mode 0
        TACCTL3_OUT: 2 = struct TACCTL3_OUT(bool);
        /// Capture input signal (read)
        TACCTL3_CCI: 3 = struct TACCTL3_CCI(bool);
        /// Capture/compare interrupt enable
        TACCTL3_CCIE: 4 = struct TACCTL3_CCIE(bool);
        /// Output mode 2
        TACCTL3_OUTMOD: 5..7 = enum TACCTL3_OUTMOD {
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
        TACCTL3_CAP: 8 = struct TACCTL3_CAP(bool);
        /// Latched capture signal (read)
        TACCTL3_SCCI: 10 = struct TACCTL3_SCCI(bool);
        /// Capture sychronize
        TACCTL3_SCS: 11 = struct TACCTL3_SCS(bool);
        /// Capture input select 1
        TACCTL3_CCIS: 12..13 = enum TACCTL3_CCIS {
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
        TACCTL3_CM: 14..15 = enum TACCTL3_CM {
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
    /// Timer A Capture/Compare Control 4
    rw TACCTL4 @ 0x3c: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TACCTL4_CCIFG: 0 = struct TACCTL4_CCIFG(bool);
        /// Capture/compare overflow flag
        TACCTL4_COV: 1 = struct TACCTL4_COV(bool);
        /// PWM Output signal if output mode 0
        TACCTL4_OUT: 2 = struct TACCTL4_OUT(bool);
        /// Capture input signal (read)
        TACCTL4_CCI: 3 = struct TACCTL4_CCI(bool);
        /// Capture/compare interrupt enable
        TACCTL4_CCIE: 4 = struct TACCTL4_CCIE(bool);
        /// Output mode 2
        TACCTL4_OUTMOD: 5..7 = enum TACCTL4_OUTMOD {
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
        TACCTL4_CAP: 8 = struct TACCTL4_CAP(bool);
        /// Latched capture signal (read)
        TACCTL4_SCCI: 10 = struct TACCTL4_SCCI(bool);
        /// Capture sychronize
        TACCTL4_SCS: 11 = struct TACCTL4_SCS(bool);
        /// Capture input select 1
        TACCTL4_CCIS: 12..13 = enum TACCTL4_CCIS {
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
        TACCTL4_CM: 14..15 = enum TACCTL4_CM {
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
    /// Timer A Counter Register
    rw TAR @ 0x42: u16 = 0_0 {
        /// Timer A Counter Register
        TAR: 0..15 = struct TARField(u16);
    }
    /// Timer A Capture/Compare 0
    rw TACCR0 @ 0x44: u16 = 0_0 {
        /// Timer A Capture/Compare 0
        TACCR0: 0..15 = struct TACCR0Field(u16);
    }
    /// Timer A Capture/Compare 1
    rw TACCR1 @ 0x46: u16 = 0_0 {
        /// Timer A Capture/Compare 1
        TACCR1: 0..15 = struct TACCR1Field(u16);
    }
    /// Timer A Capture/Compare 2
    rw TACCR2 @ 0x48: u16 = 0_0 {
        /// Timer A Capture/Compare 2
        TACCR2: 0..15 = struct TACCR2Field(u16);
    }
    /// Timer A Capture/Compare 3
    rw TACCR3 @ 0x4a: u16 = 0_0 {
        /// Timer A Capture/Compare 3
        TACCR3: 0..15 = struct TACCR3Field(u16);
    }
    /// Timer A Capture/Compare 4
    rw TACCR4 @ 0x4c: u16 = 0_0 {
        /// Timer A Capture/Compare 4
        TACCR4: 0..15 = struct TACCR4Field(u16);
    }
}
