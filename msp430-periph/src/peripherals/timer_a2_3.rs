//! Timer A2

utils::periph! {
    /// Timer A2
    TimerA2;
    /// Timer A2 Interrupt Vector Word
    rw IV @ 0x00: u16 = 0_0 {
        /// Timer A Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
    /// Timer A2 Control
    rw CTL @ 0x32: u16 = 0_0 {
        /// Timer A counter interrupt flag
        IFG: 0 = struct IFG(bool);
        /// Timer A counter interrupt enable
        IE: 1 = struct IE(bool);
        /// Timer A counter clear
        CLR: 2 = struct CLR(bool);
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
        SSEL: 8..9 = enum SSEL {
            /// Timer A clock source select: 0 - TACLK
            SSEL_0 = 0b00,
            /// Timer A clock source select: 1 - ACLK
            SSEL_1 = 0b01,
            /// Timer A clock source select: 2 - SMCLK
            SSEL_2 = 0b10,
            /// Timer A clock source select: 3 - INCLK
            SSEL_3 = 0b11,
        }
    }
    /// Timer A2 Capture/Compare Control 0
    rw CCTL0 @ 0x34: u16 = 0_0 {
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
        /// Latched capture signal (read)
        C0SCCI: 10 = struct C0SCCI(bool);
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
    /// Timer A2 Capture/Compare Control 1
    rw CCTL1 @ 0x36: u16 = 0_0 {
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
        /// Latched capture signal (read)
        C1SCCI: 10 = struct C1SCCI(bool);
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
    /// Timer A2 Counter Register
    rw R @ 0x42: u16 = 0_0 {
        /// Timer A2 Counter Register
        R: 0..15 = struct RField(u16);
    }
    /// Timer A2 Capture/Compare 0
    rw CCR0 @ 0x44: u16 = 0_0 {
        /// Timer A2 Capture/Compare 0
        CCR0: 0..15 = struct CCR0Field(u16);
    }
    /// Timer A2 Capture/Compare 1
    rw CCR1 @ 0x46: u16 = 0_0 {
        /// Timer A2 Capture/Compare 1
        CCR1: 0..15 = struct CCR1Field(u16);
    }
}
