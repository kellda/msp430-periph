//! P2

utils::periph! {
    /// P2
    P2;
    /// Port 2 Interrupt Vector Register
    r P2IV @ 0x1d: u16 = 0_0 {
        /// Port 2 interrupt vector value
        P2IV: 0..4 = enum P2IVField {
            /// No interrupt pending
            NONE = 0b00000,
            /// Interrupt Source: Port 2.0 interrupt; Interrupt Flag: P2IFG0; Interrupt Priority: Highest
            P2IFG0 = 0b00010,
            /// Interrupt Source: Port 2.1 interrupt; Interrupt Flag: P2IFG1
            P2IFG1 = 0b00100,
            /// Interrupt Source: Port 2.2 interrupt; Interrupt Flag: P2IFG2
            P2IFG2 = 0b00110,
            /// Interrupt Source: Port 2.3 interrupt; Interrupt Flag: P2IFG3
            P2IFG3 = 0b01000,
            /// Interrupt Source: Port 2.4 interrupt; Interrupt Flag: P2IFG4
            P2IFG4 = 0b01010,
            /// Interrupt Source: Port 2.5 interrupt; Interrupt Flag: P2IFG5
            P2IFG5 = 0b01100,
            /// Interrupt Source: Port 2.6 interrupt; Interrupt Flag: P2IFG6
            P2IFG6 = 0b01110,
            /// Interrupt Source: Port 2.7 interrupt; Interrupt Flag: P2IFG7; Interrupt Priority: Lowest
            P2IFG7 = 0b10000,
        }
    }
    /// Port 2 Input
    rw P2IN @ 0x00: u8 = 0_0 {
        /// Port 2 Input
        P2IN: 0..7 = struct P2INField(u8);
    }
    /// Port 2 Output
    rw P2OUT @ 0x02: u8 = 0_0 {
        /// Port 2 Output
        P2OUT: 0..7 = struct P2OUTField(u8);
    }
    /// Port 2 Direction
    rw P2DIR @ 0x04: u8 = 0_0 {
        /// Port 2 Direction
        P2DIR: 0..7 = struct P2DIRField(u8);
    }
    /// Port 2 Resistor Enable
    rw P2REN @ 0x06: u8 = 0_0 {
        /// Port 2 Resistor Enable
        P2REN: 0..7 = struct P2RENField(u8);
    }
    /// Port 2 Select 0
    rw P2SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 2 Select 0
        P2SEL0: 0..7 = struct P2SEL0Field(u8);
    }
    /// Port 2 Select 1
    rw P2SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 2 Select 1
        P2SEL1: 0..7 = struct P2SEL1Field(u8);
    }
    /// Port 2 Complement Select
    rw P2SELC @ 0x16: u8 = 0_0 {
        /// Port 2 Complement Select
        P2SELC: 0..7 = struct P2SELCField(u8);
    }
    /// Port 2 Interrupt Edge Select
    rw P2IES @ 0x18: u8 = 0_0 {
        /// Port 2 Interrupt Edge Select
        P2IES: 0..7 = struct P2IESField(u8);
    }
    /// Port 2 Interrupt Enable
    rw P2IE @ 0x1a: u8 = 0_0 {
        /// Port 2 Interrupt Enable
        P2IE: 0..7 = struct P2IEField(u8);
    }
    /// Port 2 Interrupt Flag
    rw P2IFG @ 0x1c: u8 = 0_0 {
        /// Port 2 Interrupt Flag
        P2IFG: 0..7 = struct P2IFGField(u8);
    }
}
