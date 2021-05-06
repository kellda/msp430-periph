//! P9

utils::periph! {
    /// P9
    P9;
    /// Port 9 Interrupt Vector Register
    r P9IV @ 0x0e: u16 = 0_0 {
        /// Port 9 interrupt vector value
        P9IV: 0..4 = enum P9IVField {
            /// No interrupt pending
            NONE = 0b00000,
            /// Interrupt Source: Port 9.0 interrupt; Interrupt Flag: P9IFG0; Interrupt Priority: Highest
            P9IFG0 = 0b00010,
            /// Interrupt Source: Port 9.1 interrupt; Interrupt Flag: P9IFG1
            P9IFG1 = 0b00100,
            /// Interrupt Source: Port 9.2 interrupt; Interrupt Flag: P9IFG2
            P9IFG2 = 0b00110,
            /// Interrupt Source: Port 9.3 interrupt; Interrupt Flag: P9IFG3
            P9IFG3 = 0b01000,
            /// Interrupt Source: Port 9.4 interrupt; Interrupt Flag: P9IFG4
            P9IFG4 = 0b01010,
            /// Interrupt Source: Port 9.5 interrupt; Interrupt Flag: P9IFG5
            P9IFG5 = 0b01100,
            /// Interrupt Source: Port 9.6 interrupt; Interrupt Flag: P9IFG6
            P9IFG6 = 0b01110,
            /// Interrupt Source: Port 9.7 interrupt; Interrupt Flag: P9IFG7; Interrupt Priority: Lowest
            P9IFG7 = 0b10000,
        }
    }
    /// Port 9 Input
    rw P9IN @ 0x00: u8 = 0_0 {
        /// Port 9 Input
        P9IN: 0..7 = struct P9INField(u8);
    }
    /// Port 9 Output
    rw P9OUT @ 0x02: u8 = 0_0 {
        /// Port 9 Output
        P9OUT: 0..7 = struct P9OUTField(u8);
    }
    /// Port 9 Direction
    rw P9DIR @ 0x04: u8 = 0_0 {
        /// Port 9 Direction
        P9DIR: 0..7 = struct P9DIRField(u8);
    }
    /// Port 9 Resistor Enable
    rw P9REN @ 0x06: u8 = 0_0 {
        /// Port 9 Resistor Enable
        P9REN: 0..7 = struct P9RENField(u8);
    }
    /// Port 9 Select 0
    rw P9SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 9 Select 0
        P9SEL0: 0..7 = struct P9SEL0Field(u8);
    }
    /// Port 9 Select 1
    rw P9SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 9 Select 1
        P9SEL1: 0..7 = struct P9SEL1Field(u8);
    }
    /// Port 9 Complement Select
    rw P9SELC @ 0x16: u8 = 0_0 {
        /// Port 9 Complement Select
        P9SELC: 0..7 = struct P9SELCField(u8);
    }
    /// Port 9 Interrupt Edge Select
    rw P9IES @ 0x18: u8 = 0_0 {
        /// Port 9 Interrupt Edge Select
        P9IES: 0..7 = struct P9IESField(u8);
    }
    /// Port 9 Interrupt Enable
    rw P9IE @ 0x1a: u8 = 0_0 {
        /// Port 9 Interrupt Enable
        P9IE: 0..7 = struct P9IEField(u8);
    }
    /// Port 9 Interrupt Flag
    rw P9IFG @ 0x1c: u8 = 0_0 {
        /// Port 9 Interrupt Flag
        P9IFG: 0..7 = struct P9IFGField(u8);
    }
}
