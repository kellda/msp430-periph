//! MPY 32  Multiplier  32 Bit Mode

utils::periph! {
    /// MPY 32  Multiplier  32 Bit Mode
    MPY32;
    /// 32-bit operand 1 - multiply - low word
    rw MPY32L @ 0x00: u16 = 0_0 {
        /// 32-bit operand 1 - multiply - low word
        MPY32L: 0..15 = struct MPY32LField(u16);
    }
    /// 32-bit operand 1 - multiply - high word
    rw MPY32H @ 0x02: u16 = 0_0 {
        /// 32-bit operand 1 - multiply - high word
        MPY32H: 0..15 = struct MPY32HField(u16);
    }
    /// 32-bit operand 1 - signed multiply - low word
    rw MPYS32L @ 0x04: u16 = 0_0 {
        /// 32-bit operand 1 - signed multiply - low word
        MPYS32L: 0..15 = struct MPYS32LField(u16);
    }
    /// 32-bit operand 1 - signed multiply - high word
    rw MPYS32H @ 0x06: u16 = 0_0 {
        /// 32-bit operand 1 - signed multiply - high word
        MPYS32H: 0..15 = struct MPYS32HField(u16);
    }
    /// 32-bit operand 1 - multiply accumulate - low word
    rw MAC32L @ 0x08: u16 = 0_0 {
        /// 32-bit operand 1 - multiply accumulate - low word
        MAC32L: 0..15 = struct MAC32LField(u16);
    }
    /// 32-bit operand 1 - multiply accumulate - high word
    rw MAC32H @ 0x0a: u16 = 0_0 {
        /// 32-bit operand 1 - multiply accumulate - high word
        MAC32H: 0..15 = struct MAC32HField(u16);
    }
    /// 32-bit operand 1 - signed multiply accumulate - low word
    rw MACS32L @ 0x0c: u16 = 0_0 {
        /// 32-bit operand 1 - signed multiply accumulate - low word
        MACS32L: 0..15 = struct MACS32LField(u16);
    }
    /// 32-bit operand 1 - signed multiply accumulate - high word
    rw MACS32H @ 0x0e: u16 = 0_0 {
        /// 32-bit operand 1 - signed multiply accumulate - high word
        MACS32H: 0..15 = struct MACS32HField(u16);
    }
    /// 32-bit operand 2 - low word
    rw OP2L @ 0x10: u16 = 0_0 {
        /// 32-bit operand 2 - low word
        OP2L: 0..15 = struct OP2LField(u16);
    }
    /// 32-bit operand 2 - high word
    rw OP2H @ 0x12: u16 = 0_0 {
        /// 32-bit operand 2 - high word
        OP2H: 0..15 = struct OP2HField(u16);
    }
    /// 32x32-bit result 0 - least significant word
    rw RES0 @ 0x14: u16 = 0_0 {
        /// 32x32-bit result 0 - least significant word
        RES0: 0..15 = struct RES0Field(u16);
    }
    /// 32x32-bit result 1
    rw RES1 @ 0x16: u16 = 0_0 {
        /// 32x32-bit result 1
        RES1: 0..15 = struct RES1Field(u16);
    }
    /// 32x32-bit result 2
    rw RES2 @ 0x18: u16 = 0_0 {
        /// 32x32-bit result 2
        RES2: 0..15 = struct RES2Field(u16);
    }
    /// 32x32-bit result 3 - most significant word
    rw RES3 @ 0x1a: u16 = 0_0 {
        /// 32x32-bit result 3 - most significant word
        RES3: 0..15 = struct RES3Field(u16);
    }
}
