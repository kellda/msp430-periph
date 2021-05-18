//! Multiplier

utils::periph! {
    /// Multiplier
    Multiplier;
    /// Multiply Unsigned/Operand 1
    rw MPY @ 0x00: u16 = 0_0 {
        /// Multiply Unsigned/Operand 1
        MPY: 0..15 = struct MPYField(u16);
    }
    /// Multiply Signed/Operand 1
    rw MPYS @ 0x02: u16 = 0_0 {
        /// Multiply Signed/Operand 1
        MPYS: 0..15 = struct MPYSField(u16);
    }
    /// Multiply Unsigned and Accumulate/Operand 1
    rw MAC @ 0x04: u16 = 0_0 {
        /// Multiply Unsigned and Accumulate/Operand 1
        MAC: 0..15 = struct MACField(u16);
    }
    /// Multiply Signed and Accumulate/Operand 1
    rw MACS @ 0x06: u16 = 0_0 {
        /// Multiply Signed and Accumulate/Operand 1
        MACS: 0..15 = struct MACSField(u16);
    }
    /// Operand 2
    rw OP2 @ 0x08: u16 = 0_0 {
        /// Operand 2
        OP2: 0..15 = struct OP2Field(u16);
    }
    /// Result Low Word
    rw RESLO @ 0x0a: u16 = 0_0 {
        /// Result Low Word
        RESLO: 0..15 = struct RESLOField(u16);
    }
    /// Result High Word
    rw RESHI @ 0x0c: u16 = 0_0 {
        /// Result High Word
        RESHI: 0..15 = struct RESHIField(u16);
    }
    /// Sum Extend
    rw SUMEXT @ 0x0e: u16 = 0_0 {
        /// Sum Extend
        SUMEXT: 0..15 = struct SUMEXTField(u16);
    }
}
