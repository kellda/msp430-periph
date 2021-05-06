//! P4

utils::periph! {
    /// P4
    P4;
    /// Port 4 Interrupt Vector Register
    r P4IV @ 0x1d: u16 = 0_0 {
        /// Port 4 interrupt vector value
        P4IV: 0..4 = enum P4IVField {
            /// No interrupt pending
            NONE = 0b00000,
            /// Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest
            P4IFG0 = 0b00010,
            /// Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1
            P4IFG1 = 0b00100,
            /// Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2
            P4IFG2 = 0b00110,
            /// Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3
            P4IFG3 = 0b01000,
            /// Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4
            P4IFG4 = 0b01010,
            /// Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5
            P4IFG5 = 0b01100,
            /// Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6
            P4IFG6 = 0b01110,
            /// Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest
            P4IFG7 = 0b10000,
        }
    }
    /// Port 4 Input
    rw P4IN @ 0x00: u8 = 0_0 {
        /// Port 4 Input
        P4IN: 0..7 = struct P4INField(u8);
    }
    /// Port 4 Output
    rw P4OUT @ 0x02: u8 = 0_0 {
        /// Port 4 Output
        P4OUT: 0..7 = struct P4OUTField(u8);
    }
    /// Port 4 Direction
    rw P4DIR @ 0x04: u8 = 0_0 {
        /// Port 4 Direction
        P4DIR: 0..7 = struct P4DIRField(u8);
    }
    /// Port 4 Resistor Enable
    rw P4REN @ 0x06: u8 = 0_0 {
        /// Port 4 Resistor Enable
        P4REN: 0..7 = struct P4RENField(u8);
    }
    /// Port 4 Select 0
    rw P4SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 4 Select 0
        P4SEL0: 0..7 = struct P4SEL0Field(u8);
    }
    /// Port 4 Select 1
    rw P4SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 4 Select 1
        P4SEL1: 0..7 = struct P4SEL1Field(u8);
    }
    /// Port 4 Complement Select
    rw P4SELC @ 0x16: u8 = 0_0 {
        /// Port 4 Complement Select
        P4SELC: 0..7 = struct P4SELCField(u8);
    }
    /// Port 4 Interrupt Edge Select
    rw P4IES @ 0x18: u8 = 0_0 {
        /// Port 4 Interrupt Edge Select
        P4IES: 0..7 = struct P4IESField(u8);
    }
    /// Port 4 Interrupt Enable
    rw P4IE @ 0x1a: u8 = 0_0 {
        /// Port 4 Interrupt Enable
        P4IE: 0..7 = struct P4IEField(u8);
    }
    /// Port 4 Interrupt Flag
    rw P4IFG @ 0x1c: u8 = 0_0 {
        /// Port 4 Interrupt Flag
        P4IFG: 0..7 = struct P4IFGField(u8);
    }
}
