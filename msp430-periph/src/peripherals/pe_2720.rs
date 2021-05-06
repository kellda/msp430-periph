//! PE

utils::periph! {
    /// PE
    PE;
    /// Port E Input
    rw PEIN @ 0x00: u16 = 0_0 {
        /// Port E Input
        PEIN: 0..15 = struct PEINField(u16);
    }
    /// Port E Output
    rw PEOUT @ 0x02: u16 = 0_0 {
        /// Port E Output
        PEOUT: 0..15 = struct PEOUTField(u16);
    }
    /// Port E Direction
    rw PEDIR @ 0x04: u16 = 0_0 {
        /// Port E Direction
        PEDIR: 0..15 = struct PEDIRField(u16);
    }
    /// Port E Resistor Enable
    rw PEREN @ 0x06: u16 = 0_0 {
        /// Port E Resistor Enable
        PEREN: 0..15 = struct PERENField(u16);
    }
    /// Port E Select 0
    rw PESEL0 @ 0x0a: u16 = 0_0 {
        /// Port E Select 0
        PESEL0: 0..15 = struct PESEL0Field(u16);
    }
    /// Port E Select 1
    rw PESEL1 @ 0x0c: u16 = 0_0 {
        /// Port E Select 1
        PESEL1: 0..15 = struct PESEL1Field(u16);
    }
    /// Port E Complement Select
    rw PESELC @ 0x16: u16 = 0_0 {
        /// Port E Complement Select
        PESELC: 0..15 = struct PESELCField(u16);
    }
    /// Port E Interrupt Edge Select
    rw PEIES @ 0x18: u16 = 0_0 {
        /// Port E Interrupt Edge Select
        PEIES: 0..15 = struct PEIESField(u16);
    }
    /// Port E Interrupt Enable
    rw PEIE @ 0x1a: u16 = 0_0 {
        /// Port E Interrupt Enable
        PEIE: 0..15 = struct PEIEField(u16);
    }
    /// Port E Interrupt Flag
    rw PEIFG @ 0x1c: u16 = 0_0 {
        /// Port E Interrupt Flag
        PEIFG: 0..15 = struct PEIFGField(u16);
    }
}
