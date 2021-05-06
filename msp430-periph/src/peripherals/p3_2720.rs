//! P3

utils::periph! {
    /// P3
    P3;
    /// Port 3 Interrupt Vector Register
    r P3IV @ 0x0e: u16 = 0_0 {
        /// Port 3 interrupt vector value
        P3IV: 0..4 = enum P3IVField {
            /// No interrupt pending
            NONE = 0b00000,
            /// Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest
            P3IFG0 = 0b00010,
            /// Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1
            P3IFG1 = 0b00100,
            /// Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2
            P3IFG2 = 0b00110,
            /// Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3
            P3IFG3 = 0b01000,
            /// Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4
            P3IFG4 = 0b01010,
            /// Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5
            P3IFG5 = 0b01100,
            /// Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6
            P3IFG6 = 0b01110,
            /// Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest
            P3IFG7 = 0b10000,
        }
    }
    /// Port 3 Input
    rw P3IN @ 0x00: u8 = 0_0 {
        /// Port 3 Input
        P3IN: 0..7 = struct P3INField(u8);
    }
    /// Port 3 Output
    rw P3OUT @ 0x02: u8 = 0_0 {
        /// Port 3 Output
        P3OUT: 0..7 = struct P3OUTField(u8);
    }
    /// Port 3 Direction
    rw P3DIR @ 0x04: u8 = 0_0 {
        /// Port 3 Direction
        P3DIR: 0..7 = struct P3DIRField(u8);
    }
    /// Port 3 Resistor Enable
    rw P3REN @ 0x06: u8 = 0_0 {
        /// Port 3 Resistor Enable
        P3REN: 0..7 = struct P3RENField(u8);
    }
    /// Port 3 Select 0
    rw P3SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 3 Select 0
        P3SEL0: 0..7 = struct P3SEL0Field(u8);
    }
    /// Port 3 Select 1
    rw P3SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 3 Select 1
        P3SEL1: 0..7 = struct P3SEL1Field(u8);
    }
    /// Port 3 Complement Select
    rw P3SELC @ 0x16: u8 = 0_0 {
        /// Port 3 Complement Select
        P3SELC: 0..7 = struct P3SELCField(u8);
    }
    /// Port 3 Interrupt Edge Select
    rw P3IES @ 0x18: u8 = 0_0 {
        /// Port 3 Interrupt Edge Select
        P3IES: 0..7 = struct P3IESField(u8);
    }
    /// Port 3 Interrupt Enable
    rw P3IE @ 0x1a: u8 = 0_0 {
        /// Port 3 Interrupt Enable
        P3IE: 0..7 = struct P3IEField(u8);
    }
    /// Port 3 Interrupt Flag
    rw P3IFG @ 0x1c: u8 = 0_0 {
        /// Port 3 Interrupt Flag
        P3IFG: 0..7 = struct P3IFGField(u8);
    }
}
