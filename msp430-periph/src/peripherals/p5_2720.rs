//! P5

utils::periph! {
    /// P5
    P5;
    /// Port 5 Interrupt Vector Register
    r P5IV @ 0x0e: u16 = 0_0 {
        /// Port 5 interrupt vector value
        P5IV: 0..4 = enum P5IVField {
            /// No interrupt pending
            NONE = 0b00000,
            /// Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest
            P5IFG0 = 0b00010,
            /// Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1
            P5IFG1 = 0b00100,
            /// Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2
            P5IFG2 = 0b00110,
            /// Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3
            P5IFG3 = 0b01000,
            /// Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4
            P5IFG4 = 0b01010,
            /// Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5
            P5IFG5 = 0b01100,
            /// Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6
            P5IFG6 = 0b01110,
            /// Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest
            P5IFG7 = 0b10000,
        }
    }
    /// Port 5 Input
    rw P5IN @ 0x00: u8 = 0_0 {
        /// Port 5 Input
        P5IN: 0..7 = struct P5INField(u8);
    }
    /// Port 5 Output
    rw P5OUT @ 0x02: u8 = 0_0 {
        /// Port 5 Output
        P5OUT: 0..7 = struct P5OUTField(u8);
    }
    /// Port 5 Direction
    rw P5DIR @ 0x04: u8 = 0_0 {
        /// Port 5 Direction
        P5DIR: 0..7 = struct P5DIRField(u8);
    }
    /// Port 5 Resistor Enable
    rw P5REN @ 0x06: u8 = 0_0 {
        /// Port 5 Resistor Enable
        P5REN: 0..7 = struct P5RENField(u8);
    }
    /// Port 5 Select 0
    rw P5SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 5 Select 0
        P5SEL0: 0..7 = struct P5SEL0Field(u8);
    }
    /// Port 5 Select 1
    rw P5SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 5 Select 1
        P5SEL1: 0..7 = struct P5SEL1Field(u8);
    }
    /// Port 5 Complement Select
    rw P5SELC @ 0x16: u8 = 0_0 {
        /// Port 5 Complement Select
        P5SELC: 0..7 = struct P5SELCField(u8);
    }
    /// Port 5 Interrupt Edge Select
    rw P5IES @ 0x18: u8 = 0_0 {
        /// Port 5 Interrupt Edge Select
        P5IES: 0..7 = struct P5IESField(u8);
    }
    /// Port 5 Interrupt Enable
    rw P5IE @ 0x1a: u8 = 0_0 {
        /// Port 5 Interrupt Enable
        P5IE: 0..7 = struct P5IEField(u8);
    }
    /// Port 5 Interrupt Flag
    rw P5IFG @ 0x1c: u8 = 0_0 {
        /// Port 5 Interrupt Flag
        P5IFG: 0..7 = struct P5IFGField(u8);
    }
}
