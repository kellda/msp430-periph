//! PC

utils::periph! {
    /// PC
    PC;
    /// Port C Input
    rw PCIN @ 0x00: u16 = 0_0 {
        /// Port C Input
        PCIN: 0..15 = struct PCINField(u16);
    }
    /// Port C Output
    rw PCOUT @ 0x02: u16 = 0_0 {
        /// Port C Output
        PCOUT: 0..15 = struct PCOUTField(u16);
    }
    /// Port C Direction
    rw PCDIR @ 0x04: u16 = 0_0 {
        /// Port C Direction
        PCDIR: 0..15 = struct PCDIRField(u16);
    }
    /// Port C Resistor Enable
    rw PCREN @ 0x06: u16 = 0_0 {
        /// Port C Resistor Enable
        PCREN: 0..15 = struct PCRENField(u16);
    }
    /// Port C Select 0
    rw PCSEL0 @ 0x0a: u16 = 0_0 {
        /// Port C Select 0
        PCSEL0: 0..15 = struct PCSEL0Field(u16);
    }
    /// Port C Select 1
    rw PCSEL1 @ 0x0c: u16 = 0_0 {
        /// Port C Select 1
        PCSEL1: 0..15 = struct PCSEL1Field(u16);
    }
    /// Port C Complement Select
    rw PCSELC @ 0x16: u16 = 0_0 {
        /// Port C Complement Select
        PCSELC: 0..15 = struct PCSELCField(u16);
    }
    /// Port C Interrupt Edge Select
    rw PCIES @ 0x18: u16 = 0_0 {
        /// Port C Interrupt Edge Select
        PCIES: 0..15 = struct PCIESField(u16);
    }
    /// Port C Interrupt Enable
    rw PCIE @ 0x1a: u16 = 0_0 {
        /// Port C Interrupt Enable
        PCIE: 0..15 = struct PCIEField(u16);
    }
    /// Port C Interrupt Flag
    rw PCIFG @ 0x1c: u16 = 0_0 {
        /// Port C Interrupt Flag
        PCIFG: 0..15 = struct PCIFGField(u16);
    }
}
