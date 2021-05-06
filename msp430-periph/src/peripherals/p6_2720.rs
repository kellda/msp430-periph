//! P6

utils::periph! {
    /// P6
    P6;
    /// Port 6 Interrupt Vector Register
    r P6IV @ 0x1d: u16 = 0_0 {
        /// Port 6 interrupt vector value
        P6IV: 0..4 = enum P6IVField {
            /// No interrupt pending
            NONE = 0b00000,
            /// Interrupt Source: Port 6.0 interrupt; Interrupt Flag: P6IFG0; Interrupt Priority: Highest
            P6IFG0 = 0b00010,
            /// Interrupt Source: Port 6.1 interrupt; Interrupt Flag: P6IFG1
            P6IFG1 = 0b00100,
            /// Interrupt Source: Port 6.2 interrupt; Interrupt Flag: P6IFG2
            P6IFG2 = 0b00110,
            /// Interrupt Source: Port 6.3 interrupt; Interrupt Flag: P6IFG3
            P6IFG3 = 0b01000,
            /// Interrupt Source: Port 6.4 interrupt; Interrupt Flag: P6IFG4
            P6IFG4 = 0b01010,
            /// Interrupt Source: Port 6.5 interrupt; Interrupt Flag: P6IFG5
            P6IFG5 = 0b01100,
            /// Interrupt Source: Port 6.6 interrupt; Interrupt Flag: P6IFG6
            P6IFG6 = 0b01110,
            /// Interrupt Source: Port 6.7 interrupt; Interrupt Flag: P6IFG7; Interrupt Priority: Lowest
            P6IFG7 = 0b10000,
        }
    }
    /// Port 6 Input
    rw P6IN @ 0x00: u8 = 0_0 {
        /// Port 6 Input
        P6IN: 0..7 = struct P6INField(u8);
    }
    /// Port 6 Output
    rw P6OUT @ 0x02: u8 = 0_0 {
        /// Port 6 Output
        P6OUT: 0..7 = struct P6OUTField(u8);
    }
    /// Port 6 Direction
    rw P6DIR @ 0x04: u8 = 0_0 {
        /// Port 6 Direction
        P6DIR: 0..7 = struct P6DIRField(u8);
    }
    /// Port 6 Resistor Enable
    rw P6REN @ 0x06: u8 = 0_0 {
        /// Port 6 Resistor Enable
        P6REN: 0..7 = struct P6RENField(u8);
    }
    /// Port 6 Select 0
    rw P6SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 6 Select 0
        P6SEL0: 0..7 = struct P6SEL0Field(u8);
    }
    /// Port 6 Select 1
    rw P6SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 6 Select 1
        P6SEL1: 0..7 = struct P6SEL1Field(u8);
    }
    /// Port 6 Complement Select
    rw P6SELC @ 0x16: u8 = 0_0 {
        /// Port 6 Complement Select
        P6SELC: 0..7 = struct P6SELCField(u8);
    }
    /// Port 6 Interrupt Edge Select
    rw P6IES @ 0x18: u8 = 0_0 {
        /// Port 6 Interrupt Edge Select
        P6IES: 0..7 = struct P6IESField(u8);
    }
    /// Port 6 Interrupt Enable
    rw P6IE @ 0x1a: u8 = 0_0 {
        /// Port 6 Interrupt Enable
        P6IE: 0..7 = struct P6IEField(u8);
    }
    /// Port 6 Interrupt Flag
    rw P6IFG @ 0x1c: u8 = 0_0 {
        /// Port 6 Interrupt Flag
        P6IFG: 0..7 = struct P6IFGField(u8);
    }
}
