//! P8

utils::periph! {
    /// P8
    P8;
    /// Port 8 Interrupt Vector Register
    r P8IV @ 0x1d: u16 = 0_0 {
        /// Port 8 interrupt vector value
        P8IV: 0..4 = enum P8IVField {
            /// No interrupt pending
            NONE = 0b00000,
            /// Interrupt Source: Port 8.0 interrupt; Interrupt Flag: P8IFG0; Interrupt Priority: Highest
            P8IFG0 = 0b00010,
            /// Interrupt Source: Port 8.1 interrupt; Interrupt Flag: P8IFG1
            P8IFG1 = 0b00100,
            /// Interrupt Source: Port 8.2 interrupt; Interrupt Flag: P8IFG2
            P8IFG2 = 0b00110,
            /// Interrupt Source: Port 8.3 interrupt; Interrupt Flag: P8IFG3
            P8IFG3 = 0b01000,
            /// Interrupt Source: Port 8.4 interrupt; Interrupt Flag: P8IFG4
            P8IFG4 = 0b01010,
            /// Interrupt Source: Port 8.5 interrupt; Interrupt Flag: P8IFG5
            P8IFG5 = 0b01100,
            /// Interrupt Source: Port 8.6 interrupt; Interrupt Flag: P8IFG6
            P8IFG6 = 0b01110,
            /// Interrupt Source: Port 8.7 interrupt; Interrupt Flag: P8IFG7; Interrupt Priority: Lowest
            P8IFG7 = 0b10000,
        }
    }
    /// Port 8 Input
    rw P8IN @ 0x00: u8 = 0_0 {
        /// Port 8 Input
        P8IN: 0..7 = struct P8INField(u8);
    }
    /// Port 8 Output
    rw P8OUT @ 0x02: u8 = 0_0 {
        /// Port 8 Output
        P8OUT: 0..7 = struct P8OUTField(u8);
    }
    /// Port 8 Direction
    rw P8DIR @ 0x04: u8 = 0_0 {
        /// Port 8 Direction
        P8DIR: 0..7 = struct P8DIRField(u8);
    }
    /// Port 8 Resistor Enable
    rw P8REN @ 0x06: u8 = 0_0 {
        /// Port 8 Resistor Enable
        P8REN: 0..7 = struct P8RENField(u8);
    }
    /// Port 8 Select 0
    rw P8SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 8 Select 0
        P8SEL0: 0..7 = struct P8SEL0Field(u8);
    }
    /// Port 8 Select 1
    rw P8SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 8 Select 1
        P8SEL1: 0..7 = struct P8SEL1Field(u8);
    }
    /// Port 8 Complement Select
    rw P8SELC @ 0x16: u8 = 0_0 {
        /// Port 8 Complement Select
        P8SELC: 0..7 = struct P8SELCField(u8);
    }
    /// Port 8 Interrupt Edge Select
    rw P8IES @ 0x18: u8 = 0_0 {
        /// Port 8 Interrupt Edge Select
        P8IES: 0..7 = struct P8IESField(u8);
    }
    /// Port 8 Interrupt Enable
    rw P8IE @ 0x1a: u8 = 0_0 {
        /// Port 8 Interrupt Enable
        P8IE: 0..7 = struct P8IEField(u8);
    }
    /// Port 8 Interrupt Flag
    rw P8IFG @ 0x1c: u8 = 0_0 {
        /// Port 8 Interrupt Flag
        P8IFG: 0..7 = struct P8IFGField(u8);
    }
}
