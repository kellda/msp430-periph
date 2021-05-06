//! P10

utils::periph! {
    /// P10
    P10;
    /// Port 10 Input
    rw P10IN @ 0x00: u8 = 0_0 {
        /// Port 10 Input
        P10IN: 0..7 = struct P10INField(u8);
    }
    /// Port 10 Output
    rw P10OUT @ 0x02: u8 = 0_0 {
        /// Port 10 Output
        P10OUT: 0..7 = struct P10OUTField(u8);
    }
    /// Port 10 Direction
    rw P10DIR @ 0x04: u8 = 0_0 {
        /// Port 10 Direction
        P10DIR: 0..7 = struct P10DIRField(u8);
    }
    /// Port 10 Resistor Enable
    rw P10REN @ 0x06: u8 = 0_0 {
        /// Port 10 Resistor Enable
        P10REN: 0..7 = struct P10RENField(u8);
    }
    /// Port 10 Select 0
    rw P10SEL0 @ 0x0a: u8 = 0_0 {
        /// Port 10 Select 0
        P10SEL0: 0..7 = struct P10SEL0Field(u8);
    }
    /// Port 10 Select 1
    rw P10SEL1 @ 0x0c: u8 = 0_0 {
        /// Port 10 Select 1
        P10SEL1: 0..7 = struct P10SEL1Field(u8);
    }
    /// Port 10 Complement Select
    rw P10SELC @ 0x16: u8 = 0_0 {
        /// Port 10 Complement Select
        P10SELC: 0..7 = struct P10SELCField(u8);
    }
    /// Port 10 Interrupt Edge Select
    rw P10IES @ 0x18: u8 = 0_0 {
        /// Port 10 Interrupt Edge Select
        P10IES: 0..7 = struct P10IESField(u8);
    }
    /// Port 10 Interrupt Enable
    rw P10IE @ 0x1a: u8 = 0_0 {
        /// Port 10 Interrupt Enable
        P10IE: 0..7 = struct P10IEField(u8);
    }
    /// Port 10 Interrupt Flag
    rw P10IFG @ 0x1c: u8 = 0_0 {
        /// Port 10 Interrupt Flag
        P10IFG: 0..7 = struct P10IFGField(u8);
    }
}
