//! PD

utils::periph! {
    /// PD
    PD;
    /// Port D Input
    rw PDIN @ 0x00: u16 = 0_0 {
        /// Port D Input
        PDIN: 0..15 = struct PDINField(u16);
    }
    /// Port D Output
    rw PDOUT @ 0x02: u16 = 0_0 {
        /// Port D Output
        PDOUT: 0..15 = struct PDOUTField(u16);
    }
    /// Port D Direction
    rw PDDIR @ 0x04: u16 = 0_0 {
        /// Port D Direction
        PDDIR: 0..15 = struct PDDIRField(u16);
    }
    /// Port D Resistor Enable
    rw PDREN @ 0x06: u16 = 0_0 {
        /// Port D Resistor Enable
        PDREN: 0..15 = struct PDRENField(u16);
    }
    /// Port D Select 0
    rw PDSEL0 @ 0x0a: u16 = 0_0 {
        /// Port D Select 0
        PDSEL0: 0..15 = struct PDSEL0Field(u16);
    }
    /// Port D Select 1
    rw PDSEL1 @ 0x0c: u16 = 0_0 {
        /// Port D Select 1
        PDSEL1: 0..15 = struct PDSEL1Field(u16);
    }
    /// Port D Complement Select
    rw PDSELC @ 0x16: u16 = 0_0 {
        /// Port D Complement Select
        PDSELC: 0..15 = struct PDSELCField(u16);
    }
    /// Port D Interrupt Edge Select
    rw PDIES @ 0x18: u16 = 0_0 {
        /// Port D Interrupt Edge Select
        PDIES: 0..15 = struct PDIESField(u16);
    }
    /// Port D Interrupt Enable
    rw PDIE @ 0x1a: u16 = 0_0 {
        /// Port D Interrupt Enable
        PDIE: 0..15 = struct PDIEField(u16);
    }
    /// Port D Interrupt Flag
    rw PDIFG @ 0x1c: u16 = 0_0 {
        /// Port D Interrupt Flag
        PDIFG: 0..15 = struct PDIFGField(u16);
    }
}
