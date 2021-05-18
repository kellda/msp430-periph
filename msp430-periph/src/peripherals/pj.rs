//! PJ

utils::periph! {
    /// PJ
    PJ;
    /// Port Input
    rw PIN @ 0x00: u16 = 0_0 {
        /// Port Input
        PIN: 0..15 = struct PINField(u16);
    }
    /// Port Output
    rw POUT @ 0x02: u16 = 0_0 {
        /// Port Output
        POUT: 0..15 = struct POUTField(u16);
    }
    /// Port Direction
    rw PDIR @ 0x04: u16 = 0_0 {
        /// Port Direction
        PDIR: 0..15 = struct PDIRField(u16);
    }
    /// Port Resistor Enable
    rw PREN @ 0x06: u16 = 0_0 {
        /// Port Resistor Enable
        PREN: 0..15 = struct PRENField(u16);
    }
    /// Port Select 0
    rw PSEL0 @ 0x0a: u16 = 0_0 {
        /// Port Select 0
        PSEL0: 0..15 = struct PSEL0Field(u16);
    }
    /// Port Select 1
    rw PSEL1 @ 0x0c: u16 = 0_0 {
        /// Port Select 1
        PSEL1: 0..15 = struct PSEL1Field(u16);
    }
    /// Port Complement Select
    rw PSELC @ 0x16: u16 = 0_0 {
        /// Port Complement Select
        PSELC: 0..15 = struct PSELCField(u16);
    }
}
