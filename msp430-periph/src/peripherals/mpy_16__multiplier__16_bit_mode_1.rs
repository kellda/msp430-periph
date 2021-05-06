//! MPY 16  Multiplier  16 Bit Mode

utils::periph! {
    /// MPY 16  Multiplier  16 Bit Mode
    MPY16Multiplier16BitMode;
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
    /// MPY32 Control Register 0
    rw MPY32CTL0 @ 0x2c: u16 = 0_0 {
        /// Carry of the multiplier
        MPYC: 0 = struct MPYC(bool);
        /// Fractional mode
        MPYFRAC: 2 = struct MPYFRAC(bool);
        /// Saturation mode
        MPYSAT: 3 = struct MPYSAT(bool);
        /// Multiplier mode Bit:0
        MPYM: 4..5 = enum MPYM {
            /// Multiplier mode: MPY
            MPYM_0 = 0b00,
            /// Multiplier mode: MPYS
            MPYM_1 = 0b01,
            /// Multiplier mode: MAC
            MPYM_2 = 0b10,
            /// Multiplier mode: MACS
            MPYM_3 = 0b11,
        }
        /// Bit-width of operand 1 0:16Bit / 1:32Bit
        OP1_32: 6 = struct OP1_32(bool);
        /// Bit-width of operand 2 0:16Bit / 1:32Bit
        OP2_32: 7 = struct OP2_32(bool);
        /// Delayed write enable
        MPYDLYWRTEN: 8 = struct MPYDLYWRTEN(bool);
        /// Delayed write mode
        MPYDLY32: 9 = struct MPYDLY32(bool);
    }
}
