//! Multiplier  32 Bit Mode

utils::periph! {
    /// Multiplier  32 Bit Mode
    Multiplier32BitMode;
    /// 32-bit operand 1 - multiply - low word (Byte Access)
    rw MPY32L_B @ 0x00: u8 = 0_0 {
        /// 32-bit operand 1 - multiply - low word (Byte Access)
        MPY32L_B: 0..7 = struct MPY32L_BField(u8);
    }
    /// 32-bit operand 1 - multiply - high word (Byte Access)
    rw MPY32H_B @ 0x02: u8 = 0_0 {
        /// 32-bit operand 1 - multiply - high word (Byte Access)
        MPY32H_B: 0..7 = struct MPY32H_BField(u8);
    }
    /// 32-bit operand 1 - signed multiply - low word (Byte Access)
    rw MPYS32L_B @ 0x04: u8 = 0_0 {
        /// 32-bit operand 1 - signed multiply - low word (Byte Access)
        MPYS32L_B: 0..7 = struct MPYS32L_BField(u8);
    }
    /// 32-bit operand 1 - signed multiply - high word (Byte Access)
    rw MPYS32H_B @ 0x06: u8 = 0_0 {
        /// 32-bit operand 1 - signed multiply - high word (Byte Access)
        MPYS32H_B: 0..7 = struct MPYS32H_BField(u8);
    }
    /// 32-bit operand 1 - multiply accumulate - low word (Byte Access)
    rw MAC32L_B @ 0x08: u8 = 0_0 {
        /// 32-bit operand 1 - multiply accumulate - low word (Byte Access)
        MAC32L_B: 0..7 = struct MAC32L_BField(u8);
    }
    /// 32-bit operand 1 - multiply accumulate - high word (Byte Access)
    rw MAC32H_B @ 0x0a: u8 = 0_0 {
        /// 32-bit operand 1 - multiply accumulate - high word (Byte Access)
        MAC32H_B: 0..7 = struct MAC32H_BField(u8);
    }
    /// 32-bit operand 1 - signed multiply accumulate - low word (Byte Access)
    rw MACS32L_B @ 0x0c: u8 = 0_0 {
        /// 32-bit operand 1 - signed multiply accumulate - low word (Byte Access)
        MACS32L_B: 0..7 = struct MACS32L_BField(u8);
    }
    /// 32-bit operand 1 - signed multiply accumulate - high word (Byte Access)
    rw MACS32H_B @ 0x0e: u8 = 0_0 {
        /// 32-bit operand 1 - signed multiply accumulate - high word (Byte Access)
        MACS32H_B: 0..7 = struct MACS32H_BField(u8);
    }
    /// 32-bit operand 2 - low word (Byte Access)
    rw OP2L_B @ 0x10: u8 = 0_0 {
        /// 32-bit operand 2 - low word (Byte Access)
        OP2L_B: 0..7 = struct OP2L_BField(u8);
    }
    /// 32-bit operand 2 - high word (Byte Access)
    rw OP2H_B @ 0x12: u8 = 0_0 {
        /// 32-bit operand 2 - high word (Byte Access)
        OP2H_B: 0..7 = struct OP2H_BField(u8);
    }
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
    /// MPY32 Control Register 0
    rw MPY32CTL0 @ 0x1c: u16 = 0_0 {
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
