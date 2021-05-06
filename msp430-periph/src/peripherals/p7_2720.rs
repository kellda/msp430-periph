//! P7

utils::periph! {
    /// P7
    P7;
    /// Port 7 Interrupt Vector Register
    r P7IV @ 0x0e: u16 = 0_0 {
        /// Port 7 interrupt vector value
        P7IV: 0..4 = enum P7IVField {
            /// No interrupt pending
            NONE = 0b00000,
            /// Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest
            P7IFG0 = 0b00010,
            /// Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1
            P7IFG1 = 0b00100,
            /// Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2
            P7IFG2 = 0b00110,
            /// Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3
            P7IFG3 = 0b01000,
            /// Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4
            P7IFG4 = 0b01010,
            /// Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5
            P7IFG5 = 0b01100,
            /// Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6
            P7IFG6 = 0b01110,
            /// Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest
            P7IFG7 = 0b10000,
        }
    }
    /// Port 7 Input
    rw P7IN @ 0x00: u8 = 0_0 {
        /// Port 7 Input
        P7IN: 0..7 = struct P7INField(u8);
    }
    /// Port 7 Output
    rw P7OUT @ 0x02: u8 = 0_0 {
        /// Port 7 Output
        P7OUT: 0..7 = struct P7OUTField(u8);
    }
    /// Port 7 Direction
    rw P7DIR @ 0x04: u8 = 0_0 {
        /// Port 7 Direction
        P7DIR: 0..7 = struct P7DIRField(u8);
    }
    /// Port 7 Resistor Enable
    rw P7REN @ 0x06: u8 = 0_0 {
        /// Port 7 Resistor Enable
        P7REN: 0..7 = struct P7RENField(u8);
    }
    /// Port 7 Select 0
    rw P7SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 7 Select 0
        P7SEL0: 0..7 = struct P7SEL0Field(u8);
    }
    /// Port 7 Select 1
    rw P7SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 7 Select 1
        P7SEL1: 0..7 = struct P7SEL1Field(u8);
    }
    /// Port 7 Complement Select
    rw P7SELC @ 0x16: u8 = 0_0 {
        /// Port 7 Complement Select
        P7SELC: 0..7 = struct P7SELCField(u8);
    }
    /// Port 7 Interrupt Edge Select
    rw P7IES @ 0x18: u8 = 0_0 {
        /// Port 7 Interrupt Edge Select
        P7IES: 0..7 = struct P7IESField(u8);
    }
    /// Port 7 Interrupt Enable
    rw P7IE @ 0x1a: u8 = 0_0 {
        /// Port 7 Interrupt Enable
        P7IE: 0..7 = struct P7IEField(u8);
    }
    /// Port 7 Interrupt Flag
    rw P7IFG @ 0x1c: u8 = 0_0 {
        /// Port 7 Interrupt Flag
        P7IFG: 0..7 = struct P7IFGField(u8);
    }
}
