//! LCD_A

utils::periph! {
    /// LCD_A
    LCD_A;
    /// LCD_A Control Register
    rw LCDACTL @ 0x00: u8 = 0_0 {
        /// LCDON
        LCDON: 0 = struct LCDON(bool);
        /// LCDSON
        LCDSON: 2 = struct LCDSON(bool);
        /// LCDMX0
        LCDMX0: 3 = struct LCDMX0(bool);
        /// LCDMX1
        LCDMX1: 4 = struct LCDMX1(bool);
        /// LCDFREQ
        LCDFREQ: 5..7 = enum LCDFREQ {
            /// LCD Freq: ACLK divided by 32
            LCDFREQ_32 = 0b000,
            /// LCD Freq: ACLK divided by 64
            LCDFREQ_64 = 0b001,
            /// LCD Freq: ACLK divided by 96
            LCDFREQ_96 = 0b010,
            /// LCD Freq: ACLK divided by 128
            LCDFREQ_128 = 0b011,
            /// LCD Freq: ACLK divided by 192
            LCDFREQ_192 = 0b100,
            /// LCD Freq: ACLK divided by 256
            LCDFREQ_256 = 0b101,
            /// LCD Freq: ACLK divided by 384
            LCDFREQ_384 = 0b110,
            /// LCD Freq: ACLK divided by 512
            LCDFREQ_512 = 0b111,
        }
    }
    /// LCD_A Port Control Register 0
    rw LCDAPCTL0 @ 0x1c: u8 = 0_0 {
        /// LCD Segment  0 to  3 Enable.
        LCDS0: 0 = struct LCDS0(bool);
        /// LCD Segment  4 to  7 Enable.
        LCDS4: 1 = struct LCDS4(bool);
        /// LCD Segment  8 to 11 Enable.
        LCDS8: 2 = struct LCDS8(bool);
        /// LCD Segment 12 to 15 Enable.
        LCDS12: 3 = struct LCDS12(bool);
        /// LCD Segment 16 to 19 Enable.
        LCDS16: 4 = struct LCDS16(bool);
        /// LCD Segment 20 to 23 Enable.
        LCDS20: 5 = struct LCDS20(bool);
        /// LCD Segment 24 to 27 Enable.
        LCDS24: 6 = struct LCDS24(bool);
        /// LCD Segment 28 to 31 Enable.
        LCDS28: 7 = struct LCDS28(bool);
    }
    /// LCD_A Port Control Register 1
    rw LCDAPCTL1 @ 0x1d: u8 = 0_0 {
        /// LCD Segment 32 to 35 Enable.
        LCDS32: 0 = struct LCDS32(bool);
        /// LCD Segment 36 to 39 Enable.
        LCDS36: 1 = struct LCDS36(bool);
    }
    /// LCD_A Voltage Control Register 0
    rw LCDAVCTL0 @ 0x1e: u8 = 0_0 {
        /// Selects 1/2 bias.
        LCD2B: 0 = struct LCD2B(bool);
        /// Selects reference voltage for regulated charge pump: 0
        VLCDREF: 1..2 = enum VLCDREF {
            /// Internal
            VLCDREF_0 = 0b00,
            /// External
            VLCDREF_1 = 0b01,
            /// Reserved
            VLCDREF_2 = 0b10,
            /// Reserved
            VLCDREF_3 = 0b11,
        }
        /// LCD Voltage Charge Pump Enable.
        LCDCPEN: 3 = struct LCDCPEN(bool);
        /// Select external source for VLCD.
        VLCDEXT: 4 = struct VLCDEXT(bool);
        /// Selects external connections for LCD mid voltages.
        LCDREXT: 5 = struct LCDREXT(bool);
        /// Selects external connection for lowest LCD voltage.
        LCDR03EXT: 6 = struct LCDR03EXT(bool);
    }
    /// LCD_A Voltage Control Register 1
    rw LCDAVCTL1 @ 0x1f: u8 = 0_0 {
        /// VLCD select: 0
        VLCD: 1..4 = enum VLCD {
            /// Charge pump disabled
            VLCD_0 = 0b0000,
            /// VLCD = 2.60V
            VLCD_1 = 0b0001,
            /// VLCD = 2.66V
            VLCD_2 = 0b0010,
            /// VLCD = 2.72V
            VLCD_3 = 0b0011,
            /// VLCD = 2.78V
            VLCD_4 = 0b0100,
            /// VLCD = 2.84V
            VLCD_5 = 0b0101,
            /// VLCD = 2.90V
            VLCD_6 = 0b0110,
            /// VLCD = 2.96V
            VLCD_7 = 0b0111,
            /// VLCD = 3.02V
            VLCD_8 = 0b1000,
            /// VLCD = 3.08V
            VLCD_9 = 0b1001,
            /// VLCD = 3.14V
            VLCD_10 = 0b1010,
            /// VLCD = 3.20V
            VLCD_11 = 0b1011,
            /// VLCD = 3.26V
            VLCD_12 = 0b1100,
            /// VLCD = 3.32V
            VLCD_13 = 0b1101,
            /// VLCD = 3.38V
            VLCD_14 = 0b1110,
            /// VLCD = 3.44V
            VLCD_15 = 0b1111,
        }
    }
    /// LCD Memory 1
    rw LCDM1 @ 0x01: u8 = 0_0 {
        /// LCD Memory 1
        LCDM1: 0..7 = struct LCDM1Field(u8);
    }
    /// LCD Memory 2
    rw LCDM2 @ 0x02: u8 = 0_0 {
        /// LCD Memory 2
        LCDM2: 0..7 = struct LCDM2Field(u8);
    }
    /// LCD Memory 3
    rw LCDM3 @ 0x03: u8 = 0_0 {
        /// LCD Memory 3
        LCDM3: 0..7 = struct LCDM3Field(u8);
    }
    /// LCD Memory 4
    rw LCDM4 @ 0x04: u8 = 0_0 {
        /// LCD Memory 4
        LCDM4: 0..7 = struct LCDM4Field(u8);
    }
    /// LCD Memory 5
    rw LCDM5 @ 0x05: u8 = 0_0 {
        /// LCD Memory 5
        LCDM5: 0..7 = struct LCDM5Field(u8);
    }
    /// LCD Memory 6
    rw LCDM6 @ 0x06: u8 = 0_0 {
        /// LCD Memory 6
        LCDM6: 0..7 = struct LCDM6Field(u8);
    }
    /// LCD Memory 7
    rw LCDM7 @ 0x07: u8 = 0_0 {
        /// LCD Memory 7
        LCDM7: 0..7 = struct LCDM7Field(u8);
    }
    /// LCD Memory 8
    rw LCDM8 @ 0x08: u8 = 0_0 {
        /// LCD Memory 8
        LCDM8: 0..7 = struct LCDM8Field(u8);
    }
    /// LCD Memory 9
    rw LCDM9 @ 0x09: u8 = 0_0 {
        /// LCD Memory 9
        LCDM9: 0..7 = struct LCDM9Field(u8);
    }
    /// LCD Memory 10
    rw LCDM10 @ 0x0a: u8 = 0_0 {
        /// LCD Memory 10
        LCDM10: 0..7 = struct LCDM10Field(u8);
    }
    /// LCD Memory 11
    rw LCDM11 @ 0x0b: u8 = 0_0 {
        /// LCD Memory 11
        LCDM11: 0..7 = struct LCDM11Field(u8);
    }
    /// LCD Memory 12
    rw LCDM12 @ 0x0c: u8 = 0_0 {
        /// LCD Memory 12
        LCDM12: 0..7 = struct LCDM12Field(u8);
    }
    /// LCD Memory 13
    rw LCDM13 @ 0x0d: u8 = 0_0 {
        /// LCD Memory 13
        LCDM13: 0..7 = struct LCDM13Field(u8);
    }
    /// LCD Memory 14
    rw LCDM14 @ 0x0e: u8 = 0_0 {
        /// LCD Memory 14
        LCDM14: 0..7 = struct LCDM14Field(u8);
    }
    /// LCD Memory 15
    rw LCDM15 @ 0x0f: u8 = 0_0 {
        /// LCD Memory 15
        LCDM15: 0..7 = struct LCDM15Field(u8);
    }
    /// LCD Memory 16
    rw LCDM16 @ 0x10: u8 = 0_0 {
        /// LCD Memory 16
        LCDM16: 0..7 = struct LCDM16Field(u8);
    }
    /// LCD Memory 17
    rw LCDM17 @ 0x11: u8 = 0_0 {
        /// LCD Memory 17
        LCDM17: 0..7 = struct LCDM17Field(u8);
    }
    /// LCD Memory 18
    rw LCDM18 @ 0x12: u8 = 0_0 {
        /// LCD Memory 18
        LCDM18: 0..7 = struct LCDM18Field(u8);
    }
    /// LCD Memory 19
    rw LCDM19 @ 0x13: u8 = 0_0 {
        /// LCD Memory 19
        LCDM19: 0..7 = struct LCDM19Field(u8);
    }
    /// LCD Memory 20
    rw LCDM20 @ 0x14: u8 = 0_0 {
        /// LCD Memory 20
        LCDM20: 0..7 = struct LCDM20Field(u8);
    }
}
