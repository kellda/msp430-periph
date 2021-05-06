//! PA

utils::periph! {
    /// PA
    PA;
    /// Port A Input
    rw PAIN @ 0x00: u16 = 0_0 {
        /// Port A Input
        PAIN: 0..15 = struct PAINField(u16);
    }
    /// Port A Output
    rw PAOUT @ 0x02: u16 = 0_0 {
        /// Port A Output
        PAOUT: 0..15 = struct PAOUTField(u16);
    }
    /// Port A Direction
    rw PADIR @ 0x04: u16 = 0_0 {
        /// Port A Direction
        PADIR: 0..15 = struct PADIRField(u16);
    }
    /// Port A Resistor Enable
    rw PAREN @ 0x06: u16 = 0_0 {
        /// Port A Resistor Enable
        PAREN: 0..15 = struct PARENField(u16);
    }
    /// Port A Select 0
    rw PASEL0 @ 0x0a: u16 = 0_0 {
        /// Port A Select 0
        PASEL0: 0..15 = struct PASEL0Field(u16);
    }
    /// Port A Select 1
    rw PASEL1 @ 0x0c: u16 = 0_0 {
        /// Port A Select 1
        PASEL1: 0..15 = struct PASEL1Field(u16);
    }
    /// Port A Complement Select
    rw PASELC @ 0x16: u16 = 0_0 {
        /// Port A Complement Select
        PASELC: 0..15 = struct PASELCField(u16);
    }
    /// Port A Interrupt Edge Select
    rw PAIES @ 0x18: u16 = 0_0 {
        /// Port A Interrupt Edge Select
        PAIES: 0..15 = struct PAIESField(u16);
    }
    /// Port A Interrupt Enable
    rw PAIE @ 0x1a: u16 = 0_0 {
        /// Port A Interrupt Enable
        PAIE: 0..15 = struct PAIEField(u16);
    }
    /// Port A Interrupt Flag
    rw PAIFG @ 0x1c: u16 = 0_0 {
        /// Port A Interrupt Flag
        PAIFG: 0..15 = struct PAIFGField(u16);
    }
}
