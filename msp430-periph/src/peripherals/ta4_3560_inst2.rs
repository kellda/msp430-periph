//! TA4

utils::periph! {
    /// TA4
    TA4;
    /// TimerAx Control Register
    rw TA4CTL @ 0x00: u16 = 0_0 {
        /// TimerA interrupt flag
        TAIFG: 0..0 = enum TAIFG {
            /// No interrupt pending
            TAIFG_0 = 0b0,
            /// Interrupt pending
            TAIFG_1 = 0b1,
        }
        /// TimerA interrupt enable
        TAIE: 1..1 = enum TAIE {
            /// Interrupt disabled
            TAIE_0 = 0b0,
            /// Interrupt enabled
            TAIE_1 = 0b1,
        }
        /// TimerA clear
        TACLR: 2 = struct TACLR(bool);
        /// Mode control
        MC: 4..5 = enum MC {
            /// Stop mode: Timer is halted
            STOP = 0b00,
            /// Up mode: Timer counts up to TAxCCR0
            UP = 0b01,
            /// Continuous mode: Timer counts up to 0FFFFh
            CONTINUOUS = 0b10,
            /// Up/down mode: Timer counts up to TAxCCR0 then down to 0000h
            UPDOWN = 0b11,
        }
        /// Input divider
        ID: 6..7 = enum ID {
            /// /1
            _1 = 0b00,
            /// /2
            _2 = 0b01,
            /// /4
            _4 = 0b10,
            /// /8
            _8 = 0b11,
        }
        /// TimerA clock source select
        TASSEL: 8..9 = enum TASSEL {
            /// TAxCLK
            TACLK = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// SMCLK
            SMCLK = 0b10,
            /// INCLK
            INCLK = 0b11,
        }
    }
    /// Timer_A Capture/Compare Control Register
    rw TA4CCTL0 @ 0x02: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA4CCTL0_CCIFG: 0..0 = enum TA4CCTL0_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TA4CCTL0_COV: 1..1 = enum TA4CCTL0_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TA4CCTL0_OUT: 2..2 = enum TA4CCTL0_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TA4CCTL0_CCI: 3 = struct TA4CCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TA4CCTL0_CCIE: 4..4 = enum TA4CCTL0_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TA4CCTL0_OUTMOD: 5..7 = enum TA4CCTL0_OUTMOD {
            /// OUT bit value
            OUTMOD_0 = 0b000,
            /// Set
            OUTMOD_1 = 0b001,
            /// Toggle/reset
            OUTMOD_2 = 0b010,
            /// Set/reset
            OUTMOD_3 = 0b011,
            /// Toggle
            OUTMOD_4 = 0b100,
            /// Reset
            OUTMOD_5 = 0b101,
            /// Toggle/set
            OUTMOD_6 = 0b110,
            /// Reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode
        TA4CCTL0_CAP: 8..8 = enum TA4CCTL0_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        TA4CCTL0_SCCI: 10 = struct TA4CCTL0_SCCI(bool);
        /// Synchronize capture source
        TA4CCTL0_SCS: 11..11 = enum TA4CCTL0_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TA4CCTL0_CCIS: 12..13 = enum TA4CCTL0_CCIS {
            /// CCIxA
            CCIA = 0b00,
            /// CCIxB
            CCIB = 0b01,
            /// GND
            GND = 0b10,
            /// VCC
            VCC = 0b11,
        }
        /// Capture mode
        TA4CCTL0_CM: 14..15 = enum TA4CCTL0_CM {
            /// No capture
            NONE = 0b00,
            /// Capture on rising edge
            RISING = 0b01,
            /// Capture on falling edge
            FALLING = 0b10,
            /// Capture on both rising and falling edges
            BOTH = 0b11,
        }
    }
    /// Timer_A Capture/Compare Control Register
    rw TA4CCTL1 @ 0x04: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA4CCTL1_CCIFG: 0..0 = enum TA4CCTL1_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TA4CCTL1_COV: 1..1 = enum TA4CCTL1_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TA4CCTL1_OUT: 2..2 = enum TA4CCTL1_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TA4CCTL1_CCI: 3 = struct TA4CCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TA4CCTL1_CCIE: 4..4 = enum TA4CCTL1_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TA4CCTL1_OUTMOD: 5..7 = enum TA4CCTL1_OUTMOD {
            /// OUT bit value
            OUTMOD_0 = 0b000,
            /// Set
            OUTMOD_1 = 0b001,
            /// Toggle/reset
            OUTMOD_2 = 0b010,
            /// Set/reset
            OUTMOD_3 = 0b011,
            /// Toggle
            OUTMOD_4 = 0b100,
            /// Reset
            OUTMOD_5 = 0b101,
            /// Toggle/set
            OUTMOD_6 = 0b110,
            /// Reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode
        TA4CCTL1_CAP: 8..8 = enum TA4CCTL1_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        TA4CCTL1_SCCI: 10 = struct TA4CCTL1_SCCI(bool);
        /// Synchronize capture source
        TA4CCTL1_SCS: 11..11 = enum TA4CCTL1_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TA4CCTL1_CCIS: 12..13 = enum TA4CCTL1_CCIS {
            /// CCIxA
            CCIA = 0b00,
            /// CCIxB
            CCIB = 0b01,
            /// GND
            GND = 0b10,
            /// VCC
            VCC = 0b11,
        }
        /// Capture mode
        TA4CCTL1_CM: 14..15 = enum TA4CCTL1_CM {
            /// No capture
            NONE = 0b00,
            /// Capture on rising edge
            RISING = 0b01,
            /// Capture on falling edge
            FALLING = 0b10,
            /// Capture on both rising and falling edges
            BOTH = 0b11,
        }
    }
    /// TimerA register
    rw TA4R @ 0x10: u16 = 0_0 {
        /// TimerA register
        TA4R: 0..15 = struct TA4RField(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw TA4CCR0 @ 0x12: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        TA4CCR0: 0..15 = struct TA4CCR0Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw TA4CCR1 @ 0x14: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        TA4CCR1: 0..15 = struct TA4CCR1Field(u16);
    }
    /// TimerAx Expansion 0 Register
    rw TA4EX0 @ 0x20: u16 = 0_0 {
        /// Input divider expansion
        TAIDEX: 0..2 = enum TAIDEX {
            /// Divide by 1
            _1 = 0b000,
            /// Divide by 2
            _2 = 0b001,
            /// Divide by 3
            _3 = 0b010,
            /// Divide by 4
            _4 = 0b011,
            /// Divide by 5
            _5 = 0b100,
            /// Divide by 6
            _6 = 0b101,
            /// Divide by 7
            _7 = 0b110,
            /// Divide by 8
            _8 = 0b111,
        }
    }
    /// TimerAx Interrupt Vector Register
    r TA4IV @ 0x2e: u16 = 0_0 {
        /// TimerA interrupt vector value
        TAIV: 0..15 = enum TAIV {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest
            TACCR1 = 0b0000000000000010,
            /// Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG
            TACCR2 = 0b0000000000000100,
            /// Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG
            TACCR3 = 0b0000000000000110,
            /// Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG
            TACCR4 = 0b0000000000001000,
            /// Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG
            TACCR5 = 0b0000000000001010,
            /// Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG
            TACCR6 = 0b0000000000001100,
            /// Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest
            TAIFG = 0b0000000000001110,
        }
    }
}
