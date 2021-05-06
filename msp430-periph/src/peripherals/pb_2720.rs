//! PB

utils::periph! {
    /// PB
    PB;
    /// Port B Input
    rw PBIN @ 0x00: u16 = 0_0 {
        /// Port B Input
        PBIN: 0..15 = struct PBINField(u16);
    }
    /// Port B Output
    rw PBOUT @ 0x02: u16 = 0_0 {
        /// Port B Output
        PBOUT: 0..15 = struct PBOUTField(u16);
    }
    /// Port B Direction
    rw PBDIR @ 0x04: u16 = 0_0 {
        /// Port B Direction
        PBDIR: 0..15 = struct PBDIRField(u16);
    }
    /// Port B Resistor Enable
    rw PBREN @ 0x06: u16 = 0_0 {
        /// Port B Resistor Enable
        PBREN: 0..15 = struct PBRENField(u16);
    }
    /// Port B Select 0
    rw PBSEL0 @ 0x0a: u16 = 0_0 {
        /// Port B Select 0
        PBSEL0: 0..15 = struct PBSEL0Field(u16);
    }
    /// Port B Select 1
    rw PBSEL1 @ 0x0c: u16 = 0_0 {
        /// Port B Select 1
        PBSEL1: 0..15 = struct PBSEL1Field(u16);
    }
    /// Port B Complement Select
    rw PBSELC @ 0x16: u16 = 0_0 {
        /// Port B Complement Select
        PBSELC: 0..15 = struct PBSELCField(u16);
    }
    /// Port B Interrupt Edge Select
    rw PBIES @ 0x18: u16 = 0_0 {
        /// Port B Interrupt Edge Select
        PBIES: 0..15 = struct PBIESField(u16);
    }
    /// Port B Interrupt Enable
    rw PBIE @ 0x1a: u16 = 0_0 {
        /// Port B Interrupt Enable
        PBIE: 0..15 = struct PBIEField(u16);
    }
    /// Port B Interrupt Flag
    rw PBIFG @ 0x1c: u16 = 0_0 {
        /// Port B Interrupt Flag
        PBIFG: 0..15 = struct PBIFGField(u16);
    }
}
