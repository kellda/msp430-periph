//! MPY32

utils::periph! {
    /// MPY32
    MPY32;
    /// 16-bit operand one  multiply
    rw MPY @ 0x00: u16 = 0_0 {
        /// 16-bit operand one  multiply
        MPY: 0..15 = struct MPYField(u16);
    }
    /// 16-bit operand one  signed multiply
    rw MPYS @ 0x02: u16 = 0_0 {
        /// 16-bit operand one  signed multiply
        MPYS: 0..15 = struct MPYSField(u16);
    }
    /// 16-bit operand one  multiply accumulate
    rw MAC @ 0x04: u16 = 0_0 {
        /// 16-bit operand one  multiply accumulate
        MAC: 0..15 = struct MACField(u16);
    }
    /// 16-bit operand one  signed multiply accumulate
    rw MACS @ 0x06: u16 = 0_0 {
        /// 16-bit operand one  signed multiply accumulate
        MACS: 0..15 = struct MACSField(u16);
    }
    /// 16-bit operand two
    rw OP2 @ 0x08: u16 = 0_0 {
        /// 16-bit operand two
        OP2: 0..15 = struct OP2Field(u16);
    }
    /// 16x16-bit result low word
    rw RESLO @ 0x0a: u16 = 0_0 {
        /// 16x16-bit result low word
        RESLO: 0..15 = struct RESLOField(u16);
    }
    /// 16x16-bit result high word
    rw RESHI @ 0x0c: u16 = 0_0 {
        /// 16x16-bit result high word
        RESHI: 0..15 = struct RESHIField(u16);
    }
    /// 16x16-bit sum extension register
    rw SUMEXT @ 0x0e: u16 = 0_0 {
        /// 16x16-bit sum extension register
        SUMEXT: 0..15 = struct SUMEXTField(u16);
    }
    /// 32-bit operand 1  multiply  low word
    rw MPY32L @ 0x10: u16 = 0_0 {
        /// 32-bit operand 1  multiply  low word
        MPY32L: 0..15 = struct MPY32LField(u16);
    }
    /// 32-bit operand 1  multiply  high word
    rw MPY32H @ 0x12: u16 = 0_0 {
        /// 32-bit operand 1  multiply  high word
        MPY32H: 0..15 = struct MPY32HField(u16);
    }
    /// 32-bit operand 1  signed multiply  low word
    rw MPYS32L @ 0x14: u16 = 0_0 {
        /// 32-bit operand 1  signed multiply  low word
        MPYS32L: 0..15 = struct MPYS32LField(u16);
    }
    /// 32-bit operand 1  signed multiply  high word
    rw MPYS32H @ 0x16: u16 = 0_0 {
        /// 32-bit operand 1  signed multiply  high word
        MPYS32H: 0..15 = struct MPYS32HField(u16);
    }
    /// 32-bit operand 1  multiply accumulate  low word
    rw MAC32L @ 0x18: u16 = 0_0 {
        /// 32-bit operand 1  multiply accumulate  low word
        MAC32L: 0..15 = struct MAC32LField(u16);
    }
    /// 32-bit operand 1  multiply accumulate  high word
    rw MAC32H @ 0x1a: u16 = 0_0 {
        /// 32-bit operand 1  multiply accumulate  high word
        MAC32H: 0..15 = struct MAC32HField(u16);
    }
    /// 32-bit operand 1  signed multiply accumulate  low word
    rw MACS32L @ 0x1c: u16 = 0_0 {
        /// 32-bit operand 1  signed multiply accumulate  low word
        MACS32L: 0..15 = struct MACS32LField(u16);
    }
    /// 32-bit operand 1  signed multiply accumulate  high word
    rw MACS32H @ 0x1e: u16 = 0_0 {
        /// 32-bit operand 1  signed multiply accumulate  high word
        MACS32H: 8..15 = struct MACS32HField(u16);
    }
    /// 32-bit operand 2  low word
    rw OP2L @ 0x20: u16 = 0_0 {
        /// 32-bit operand 2  low word
        OP2L: 0..15 = struct OP2LField(u16);
    }
    /// 32-bit operand 2  high word
    rw OP2H @ 0x22: u16 = 0_0 {
        /// 32-bit operand 2  high word
        OP2H: 0..15 = struct OP2HField(u16);
    }
    /// 32x32-bit result 0  least significant word
    rw RES0 @ 0x24: u16 = 0_0 {
        /// 32x32-bit result 0  least significant word
        RES0: 0..15 = struct RES0Field(u16);
    }
    /// 32x32-bit result 1
    rw RES1 @ 0x26: u16 = 0_0 {
        /// 32x32-bit result 1
        RES1: 0..15 = struct RES1Field(u16);
    }
    /// 32x32-bit result 2
    rw RES2 @ 0x28: u16 = 0_0 {
        /// 32x32-bit result 2
        RES2: 0..15 = struct RES2Field(u16);
    }
    /// 32x32-bit result 3  most significant word
    rw RES3 @ 0x2a: u16 = 0_0 {
        /// 32x32-bit result 3  most significant word
        RES3: 0..15 = struct RES3Field(u16);
    }
    /// MPY32 control register 0
    rw MPY32CTL0 @ 0x2c: u16 = 0_0 {
        /// Delayed write mode.
        MPYDLY32: 9..9 = enum MPYDLY32 {
            /// Writes are delayed until 64-bit result (RES0 to RES3) is available.
            MPYDLY32_0 = 0b0,
            /// Writes are delayed until 32-bit result (RES0 to RES1) is available. 8 MPYDLYWRTEN
            MPYDLY32_1 = 0b1,
        }
        /// Delayed write enable.
        MPYDLYWRTEN: 8..8 = enum MPYDLYWRTEN {
            /// Writes are not delayed.
            MPYDLYWRTEN_0 = 0b0,
            /// Writes are delayed.
            MPYDLYWRTEN_1 = 0b1,
        }
        /// Multiplier bit width of operand 2
        MPYOP2_32: 7..7 = enum MPYOP2_32 {
            /// 16 bits.
            _16 = 0b0,
            /// 32 bits.
            _32 = 0b1,
        }
        /// Multiplier bit width of operand 1
        MPYOP1_32: 6..6 = enum MPYOP1_32 {
            /// 16 bits.
            _16 = 0b0,
            /// 32 bits.
            _32 = 0b1,
        }
        /// Multiplier mode
        MPYM: 4..5 = enum MPYM {
            /// MPY  Multiply
            MPY = 0b00,
            /// MPYS  Signed multiply
            MPYS = 0b01,
            /// MAC  Multiply accumulate
            MAC = 0b10,
            /// MACS  Signed multiply accumulate
            MACS = 0b11,
        }
        /// Saturation mode
        MPYSAT: 3..3 = enum MPYSAT {
            /// Saturation mode disabled.
            DISABLE = 0b0,
            /// Saturation mode enabled.
            ENABLE = 0b1,
        }
        /// Fractional mode.
        MPYFRAC: 2..2 = enum MPYFRAC {
            /// Fractional mode disabled.
            DISABLE = 0b0,
            /// Fractional mode enabled.
            ENABLE = 0b1,
        }
        /// Carry of the multiplier
        MPYC: 0..0 = enum MPYC {
            /// No carry for result.
            MPYC_0 = 0b0,
            /// Result has a carry.
            MPYC_1 = 0b1,
        }
    }
}
