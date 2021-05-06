//! P1

utils::periph! {
    /// P1
    P1;
    /// Port 1 Interrupt Vector Register
    r P1IV @ 0x0e: u16 = 0_0 {
        /// Port 1 interrupt vector value
        P1IV: 0..4 = enum P1IVField {
            /// No interrupt pending
            NONE = 0b00000,
            /// Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest
            P1IFG0 = 0b00010,
            /// Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1
            P1IFG1 = 0b00100,
            /// Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2
            P1IFG2 = 0b00110,
            /// Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3
            P1IFG3 = 0b01000,
            /// Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4
            P1IFG4 = 0b01010,
            /// Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5
            P1IFG5 = 0b01100,
            /// Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6
            P1IFG6 = 0b01110,
            /// Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest
            P1IFG7 = 0b10000,
        }
    }
    /// Port 1 Input
    rw P1IN @ 0x00: u8 = 0_0 {
        /// Port 1 Input
        P1IN: 0..7 = struct P1INField(u8);
    }
    /// Port 1 Output
    rw P1OUT @ 0x02: u8 = 0_0 {
        /// Port 1 Output
        P1OUT: 0..7 = struct P1OUTField(u8);
    }
    /// Port 1 Direction
    rw P1DIR @ 0x04: u8 = 0_0 {
        /// Port 1 Direction
        P1DIR: 0..7 = struct P1DIRField(u8);
    }
    /// Port 1 Resistor Enable
    rw P1REN @ 0x06: u8 = 0_0 {
        /// Port 1 Resistor Enable
        P1REN: 0..7 = struct P1RENField(u8);
    }
    /// Port 1 Select 0
    rw P1SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 1 Select 0
        P1SEL0: 0..7 = struct P1SEL0Field(u8);
    }
    /// Port 1 Select 1
    rw P1SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 1 Select 1
        P1SEL1: 0..7 = struct P1SEL1Field(u8);
    }
    /// Port 1 Complement Select
    rw P1SELC @ 0x16: u8 = 0_0 {
        /// Port 1 Complement Select
        P1SELC: 0..7 = struct P1SELCField(u8);
    }
    /// Port 1 Interrupt Edge Select
    rw P1IES @ 0x18: u8 = 0_0 {
        /// Port 1 Interrupt Edge Select
        P1IES: 0..7 = struct P1IESField(u8);
    }
    /// Port 1 Interrupt Enable
    rw P1IE @ 0x1a: u8 = 0_0 {
        /// Port 1 Interrupt Enable
        P1IE: 0..7 = struct P1IEField(u8);
    }
    /// Port 1 Interrupt Flag
    rw P1IFG @ 0x1c: u8 = 0_0 {
        /// Port 1 Interrupt Flag
        P1IFG: 0..7 = struct P1IFGField(u8);
    }
}
