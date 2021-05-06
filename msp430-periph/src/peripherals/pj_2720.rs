//! PJ

utils::periph! {
    /// PJ
    PJ;
    /// Port J Input
    rw PJIN @ 0x00: u16 = 0_0 {
        /// Port J Input
        PJIN: 0..15 = struct PJINField(u16);
    }
    /// Port J Output
    rw PJOUT @ 0x02: u16 = 0_0 {
        /// Port J Output
        PJOUT: 0..15 = struct PJOUTField(u16);
    }
    /// Port J Direction
    rw PJDIR @ 0x04: u16 = 0_0 {
        /// Port J Direction
        PJDIR: 0..15 = struct PJDIRField(u16);
    }
    /// Port J Resistor Enable
    rw PJREN @ 0x06: u16 = 0_0 {
        /// Port J Resistor Enable
        PJREN: 0..15 = struct PJRENField(u16);
    }
    /// Port J Select 0
    rw PJSEL0 @ 0x0a: u16 = 0_0 {
        /// Port J Select 0
        PJSEL0: 0..15 = struct PJSEL0Field(u16);
    }
    /// Port J Select 1
    rw PJSEL1 @ 0x0c: u16 = 0_0 {
        /// Port J Select 1
        PJSEL1: 0..15 = struct PJSEL1Field(u16);
    }
    /// Port J Complement Select
    rw PJSELC @ 0x16: u16 = 0_0 {
        /// Port J Complement Select
        PJSELC: 0..15 = struct PJSELCField(u16);
    }
}
