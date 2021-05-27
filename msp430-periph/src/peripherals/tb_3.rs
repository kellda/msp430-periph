//! TBx

utils::periph! {
    /// TBx
    TB;
    /// Timer_B Control Register
    rw CTL @ 0x00: u16 = 0_0 {
        /// TimerB interrupt flag
        IFG: 0 = enum IFG {
            /// No interrupt pending
            IFG_0 = 0b0,
            /// Interrupt pending
            IFG_1 = 0b1,
        }
        /// TimerB interrupt enable
        IE: 1 = enum IE {
            /// Interrupt disabled
            IE_0 = 0b0,
            /// Interrupt enabled
            IE_1 = 0b1,
        }
        /// TimerB clear
        CLR: 2 = struct CLR(bool);
        /// Mode control
        MC: 4..5 = enum MC {
            /// Stop mode: Timer is halted
            STOP = 0b00,
            /// Up mode: Timer counts up to TBxCL0
            UP = 0b01,
            /// Continuous mode: Timer counts up to the value set by CNTL
            CONTINUOUS = 0b10,
            /// Up/down mode: Timer counts up to TBxCL0 then down to 0000h
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
        /// TimerB clock source select
        SSEL: 8..9 = enum SSEL {
            /// TBxCLK
            TBCLK = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// SMCLK
            SMCLK = 0b10,
            /// INCLK
            INCLK = 0b11,
        }
        /// Counter length
        CNTL: 11..12 = enum CNTL {
            /// 16-bit, TBxR(max) = 0FFFFh
            _16 = 0b00,
            /// 12-bit, TBxR(max) = 0FFFh
            _12 = 0b01,
            /// 10-bit, TBxR(max) = 03FFh
            _10 = 0b10,
            /// 8-bit, TBxR(max) = 0FFh
            _8 = 0b11,
        }
        /// TBxCLn group
        CLGRP: 13..14 = enum CLGRP {
            /// Each TBxCLn latch loads independently
            CLGRP_0 = 0b00,
            /// TBxCL1+TBxCL2 (TBxCCR1 CLLD bits control the update); TBxCL3+TBxCL4 (TBxCCR3 CLLD bits control the update); TBxCL5+TBxCL6 (TBxCCR5 CLLD bits control the update); TBxCL0 independent
            CLGRP_1 = 0b01,
            /// TBxCL1+TBxCL2+TBxCL3 (TBxCCR1 CLLD bits control the update); TBxCL4+TBxCL5+TBxCL6 (TBxCCR4 CLLD bits control the update); TBxCL0 independent
            CLGRP_2 = 0b10,
            /// TBxCL0+TBxCL1+TBxCL2+TBxCL3+TBxCL4+TBxCL5+TBxCL6 (TBxCCR1 CLLD bits control the update)
            CLGRP_3 = 0b11,
        }
    }
    /// Timer_B Capture/Compare Control Register
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
        /// Compare latch load
        C0CLLD: 9..10 = enum C0CLLD {
            /// xCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
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
    /// Timer_B Capture/Compare Control Register
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
        /// Compare latch load
        C1CLLD: 9..10 = enum C1CLLD {
            /// TBxCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
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
    /// Timer_B Capture/Compare Control Register
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
        /// Compare latch load
        C2CLLD: 9..10 = enum C2CLLD {
            /// TBxCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
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
    /// Timer_B count register
    rw R @ 0x10: u16 = 0_0 {
        /// Timer_B count register
        R: 0..15 = struct RField(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw CCR0 @ 0x12: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        CCR0: 0..15 = struct CCR0Field(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw CCR1 @ 0x14: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        CCR1: 0..15 = struct CCR1Field(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw CCR2 @ 0x16: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        CCR2: 0..15 = struct CCR2Field(u16);
    }
    /// Timer_Bx Expansion Register 0
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
    /// Timer_Bx Interrupt Vector Register
    r IV @ 0x2e: u16 = 0_0 {
        /// Timer_B interrupt vector value
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Capture/compare 1; Interrupt Flag: TBxCCR1 CCIFG; Interrupt Priority: Highest
            CCR1 = 0b0000000000000010,
            /// Interrupt Source: Capture/compare 2; Interrupt Flag: TBxCCR2 CCIFG
            CCR2 = 0b0000000000000100,
            /// Interrupt Source: Capture/compare 3; Interrupt Flag: TBxCCR3 CCIFG
            CCR3 = 0b0000000000000110,
            /// Interrupt Source: Capture/compare 4; Interrupt Flag: TBxCCR4 CCIFG
            CCR4 = 0b0000000000001000,
            /// Interrupt Source: Capture/compare 5; Interrupt Flag: TBxCCR5 CCIFG
            CCR5 = 0b0000000000001010,
            /// Interrupt Source: Capture/compare 6; Interrupt Flag: TBxCCR6 CCIFG
            CCR6 = 0b0000000000001100,
            /// Interrupt Source: Timer overflow; Interrupt Flag: TBxCTL TBIFG; Interrupt Priority: Lowest
            IFG = 0b0000000000001110,
        }
    }
}
