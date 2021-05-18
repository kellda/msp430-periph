//! P10

utils::periph! {
    /// P10
    P10;
    /// Port Input
    rw PIN @ 0x00: u8 = 0_0 {
        /// Port Input
        PIN: 0..7 = struct PINField(u8);
    }
    /// Port Output
    rw POUT @ 0x02: u8 = 0_0 {
        /// Port Output
        POUT: 0..7 = struct POUTField(u8);
    }
    /// Port Direction
    rw PDIR @ 0x04: u8 = 0_0 {
        /// Port Direction
        PDIR: 0..7 = struct PDIRField(u8);
    }
    /// Port Resistor Enable
    rw PREN @ 0x06: u8 = 0_0 {
        /// Port Resistor Enable
        PREN: 0..7 = struct PRENField(u8);
    }
    /// Port Select 0
    rw PSEL0 @ 0x0a: u8 = 0_0 {
        /// Port Select 0
        PSEL0: 0..7 = struct PSEL0Field(u8);
    }
    /// Port Select 1
    rw PSEL1 @ 0x0c: u8 = 0_0 {
        /// Port Select 1
        PSEL1: 0..7 = struct PSEL1Field(u8);
    }
    /// Port Complement Select
    rw PSELC @ 0x16: u8 = 0_0 {
        /// Port Complement Select
        PSELC: 0..7 = struct PSELCField(u8);
    }
    /// Port Interrupt Edge Select
    rw PIES @ 0x18: u8 = 0_0 {
        /// Port Interrupt Edge Select
        PIES: 0..7 = struct PIESField(u8);
    }
    /// Port Interrupt Enable
    rw PIE @ 0x1a: u8 = 0_0 {
        /// Port Interrupt Enable
        PIE: 0..7 = struct PIEField(u8);
    }
    /// Port Interrupt Flag
    rw PIFG @ 0x1c: u8 = 0_0 {
        /// Port Interrupt Flag
        PIFG: 0..7 = struct PIFGField(u8);
    }
}
