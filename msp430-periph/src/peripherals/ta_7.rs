//! TAx

utils::periph! {
    /// TAx
    TA;
    /// TimerAx Control Register
    rw CTL @ 0x00: u16 = 0_0 {
        /// TimerA interrupt flag
        IFG: 0 = enum IFG {
            /// No interrupt pending
            IFG_0 = 0b0,
            /// Interrupt pending
            IFG_1 = 0b1,
        }
        /// TimerA interrupt enable
        IE: 1 = enum IE {
            /// Interrupt disabled
            IE_0 = 0b0,
            /// Interrupt enabled
            IE_1 = 0b1,
        }
        /// TimerA clear
        CLR: 2 = struct CLR(bool);
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
        SSEL: 8..9 = enum SSEL {
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
    rw CCTL0 @ 0x02: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C0CCIFG: 0 = enum C0CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        C0COV: 1 = enum C0COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        C0OUT: 2 = enum C0OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        C0CCI: 3 = struct C0CCI(bool);
        /// Capture/compare interrupt enable
        C0CCIE: 4 = enum C0CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        C0OUTMOD: 5..7 = enum C0OUTMOD {
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
        C0CAP: 8 = enum C0CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        C0SCCI: 10 = struct C0SCCI(bool);
        /// Synchronize capture source
        C0SCS: 11 = enum C0SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        C0CCIS: 12..13 = enum C0CCIS {
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
        C0CM: 14..15 = enum C0CM {
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
    rw CCTL1 @ 0x04: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C1CCIFG: 0 = enum C1CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        C1COV: 1 = enum C1COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        C1OUT: 2 = enum C1OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        C1CCI: 3 = struct C1CCI(bool);
        /// Capture/compare interrupt enable
        C1CCIE: 4 = enum C1CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        C1OUTMOD: 5..7 = enum C1OUTMOD {
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
        C1CAP: 8 = enum C1CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        C1SCCI: 10 = struct C1SCCI(bool);
        /// Synchronize capture source
        C1SCS: 11 = enum C1SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        C1CCIS: 12..13 = enum C1CCIS {
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
        C1CM: 14..15 = enum C1CM {
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
    rw CCTL2 @ 0x06: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C2CCIFG: 0 = enum C2CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        C2COV: 1 = enum C2COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        C2OUT: 2 = enum C2OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        C2CCI: 3 = struct C2CCI(bool);
        /// Capture/compare interrupt enable
        C2CCIE: 4 = enum C2CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        C2OUTMOD: 5..7 = enum C2OUTMOD {
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
        C2CAP: 8 = enum C2CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        C2SCCI: 10 = struct C2SCCI(bool);
        /// Synchronize capture source
        C2SCS: 11 = enum C2SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        C2CCIS: 12..13 = enum C2CCIS {
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
        C2CM: 14..15 = enum C2CM {
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
    rw CCTL3 @ 0x08: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C3CCIFG: 0 = enum C3CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        C3COV: 1 = enum C3COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        C3OUT: 2 = enum C3OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        C3CCI: 3 = struct C3CCI(bool);
        /// Capture/compare interrupt enable
        C3CCIE: 4 = enum C3CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        C3OUTMOD: 5..7 = enum C3OUTMOD {
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
        C3CAP: 8 = enum C3CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        C3SCCI: 10 = struct C3SCCI(bool);
        /// Synchronize capture source
        C3SCS: 11 = enum C3SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        C3CCIS: 12..13 = enum C3CCIS {
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
        C3CM: 14..15 = enum C3CM {
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
    rw CCTL4 @ 0x0a: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C4CCIFG: 0 = enum C4CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        C4COV: 1 = enum C4COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        C4OUT: 2 = enum C4OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        C4CCI: 3 = struct C4CCI(bool);
        /// Capture/compare interrupt enable
        C4CCIE: 4 = enum C4CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        C4OUTMOD: 5..7 = enum C4OUTMOD {
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
        C4CAP: 8 = enum C4CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        C4SCCI: 10 = struct C4SCCI(bool);
        /// Synchronize capture source
        C4SCS: 11 = enum C4SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        C4CCIS: 12..13 = enum C4CCIS {
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
        C4CM: 14..15 = enum C4CM {
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
    rw CCTL5 @ 0x0c: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C5CCIFG: 0 = enum C5CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        C5COV: 1 = enum C5COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        C5OUT: 2 = enum C5OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        C5CCI: 3 = struct C5CCI(bool);
        /// Capture/compare interrupt enable
        C5CCIE: 4 = enum C5CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        C5OUTMOD: 5..7 = enum C5OUTMOD {
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
        C5CAP: 8 = enum C5CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        C5SCCI: 10 = struct C5SCCI(bool);
        /// Synchronize capture source
        C5SCS: 11 = enum C5SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        C5CCIS: 12..13 = enum C5CCIS {
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
        C5CM: 14..15 = enum C5CM {
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
    rw CCTL6 @ 0x0e: u16 = 0_0 {
        /// Capture/compare interrupt flag
        C6CCIFG: 0 = enum C6CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        C6COV: 1 = enum C6COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        C6OUT: 2 = enum C6OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        C6CCI: 3 = struct C6CCI(bool);
        /// Capture/compare interrupt enable
        C6CCIE: 4 = enum C6CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        C6OUTMOD: 5..7 = enum C6OUTMOD {
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
        C6CAP: 8 = enum C6CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        C6SCCI: 10 = struct C6SCCI(bool);
        /// Synchronize capture source
        C6SCS: 11 = enum C6SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        C6CCIS: 12..13 = enum C6CCIS {
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
        C6CM: 14..15 = enum C6CM {
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
    rw R @ 0x10: u16 = 0_0 {
        /// TimerA register
        R: 0..15 = struct RField(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw CCR0 @ 0x12: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        CCR0: 0..15 = struct CCR0Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw CCR1 @ 0x14: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        CCR1: 0..15 = struct CCR1Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw CCR2 @ 0x16: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        CCR2: 0..15 = struct CCR2Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw CCR3 @ 0x18: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        CCR3: 0..15 = struct CCR3Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw CCR4 @ 0x1a: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        CCR4: 0..15 = struct CCR4Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw CCR5 @ 0x1c: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        CCR5: 0..15 = struct CCR5Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw CCR6 @ 0x1e: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        CCR6: 0..15 = struct CCR6Field(u16);
    }
    /// TimerAx Expansion 0 Register
    rw EX0 @ 0x20: u16 = 0_0 {
        /// Input divider expansion
        IDEX: 0..2 = enum IDEX {
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
    r IV @ 0x2e: u16 = 0_0 {
        /// TimerA interrupt vector value
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest
            CCR1 = 0b0000000000000010,
            /// Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG
            CCR2 = 0b0000000000000100,
            /// Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG
            CCR3 = 0b0000000000000110,
            /// Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG
            CCR4 = 0b0000000000001000,
            /// Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG
            CCR5 = 0b0000000000001010,
            /// Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG
            CCR6 = 0b0000000000001100,
            /// Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest
            IFG = 0b0000000000001110,
        }
    }
}
