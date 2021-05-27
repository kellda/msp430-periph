//! LCD_A

utils::periph! {
    /// LCD_A
    LCD_A;
    /// LCD_A Control Register
    rw CTL @ 0x00: u8 = 0_0 {
        /// LCDON
        ON: 0 = struct ON(bool);
        /// LCDSON
        SON: 2 = struct SON(bool);
        /// LCDMX0
        MX0: 3 = struct MX0(bool);
        /// LCDMX1
        MX1: 4 = struct MX1(bool);
        /// LCDFREQ
        FREQ: 5..7 = enum FREQ {
            /// LCD Freq: ACLK divided by 32
            FREQ_32 = 0b000,
            /// LCD Freq: ACLK divided by 64
            FREQ_64 = 0b001,
            /// LCD Freq: ACLK divided by 96
            FREQ_96 = 0b010,
            /// LCD Freq: ACLK divided by 128
            FREQ_128 = 0b011,
            /// LCD Freq: ACLK divided by 192
            FREQ_192 = 0b100,
            /// LCD Freq: ACLK divided by 256
            FREQ_256 = 0b101,
            /// LCD Freq: ACLK divided by 384
            FREQ_384 = 0b110,
            /// LCD Freq: ACLK divided by 512
            FREQ_512 = 0b111,
        }
    }
    /// LCD_A Port Control Register 0
    rw PCTL0 @ 0x1c: u8 = 0_0 {
        /// LCD Segment  0 to  3 Enable.
        S0: 0 = struct S0(bool);
        /// LCD Segment  4 to  7 Enable.
        S4: 1 = struct S4(bool);
        /// LCD Segment  8 to 11 Enable.
        S8: 2 = struct S8(bool);
        /// LCD Segment 12 to 15 Enable.
        S12: 3 = struct S12(bool);
        /// LCD Segment 16 to 19 Enable.
        S16: 4 = struct S16(bool);
        /// LCD Segment 20 to 23 Enable.
        S20: 5 = struct S20(bool);
        /// LCD Segment 24 to 27 Enable.
        S24: 6 = struct S24(bool);
        /// LCD Segment 28 to 31 Enable.
        S28: 7 = struct S28(bool);
    }
    /// LCD_A Port Control Register 1
    rw PCTL1 @ 0x1d: u8 = 0_0 {
        /// LCD Segment 32 to 35 Enable.
        S32: 0 = struct S32(bool);
        /// LCD Segment 36 to 39 Enable.
        S36: 1 = struct S36(bool);
    }
    /// LCD_A Voltage Control Register 0
    rw VCTL0 @ 0x1e: u8 = 0_0 {
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
        CPEN: 3 = struct CPEN(bool);
        /// Select external source for VLCD.
        VLCDEXT: 4 = struct VLCDEXT(bool);
        /// Selects external connections for LCD mid voltages.
        REXT: 5 = struct REXT(bool);
        /// Selects external connection for lowest LCD voltage.
        R03EXT: 6 = struct R03EXT(bool);
    }
    /// LCD_A Voltage Control Register 1
    rw VCTL1 @ 0x1f: u8 = 0_0 {
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
    rw M1 @ 0x01: u8 = 0_0 {
        /// LCD Memory 1
        M1: 0..7 = struct M1Field(u8);
    }
    /// LCD Memory 2
    rw M2 @ 0x02: u8 = 0_0 {
        /// LCD Memory 2
        M2: 0..7 = struct M2Field(u8);
    }
    /// LCD Memory 3
    rw M3 @ 0x03: u8 = 0_0 {
        /// LCD Memory 3
        M3: 0..7 = struct M3Field(u8);
    }
    /// LCD Memory 4
    rw M4 @ 0x04: u8 = 0_0 {
        /// LCD Memory 4
        M4: 0..7 = struct M4Field(u8);
    }
    /// LCD Memory 5
    rw M5 @ 0x05: u8 = 0_0 {
        /// LCD Memory 5
        M5: 0..7 = struct M5Field(u8);
    }
    /// LCD Memory 6
    rw M6 @ 0x06: u8 = 0_0 {
        /// LCD Memory 6
        M6: 0..7 = struct M6Field(u8);
    }
    /// LCD Memory 7
    rw M7 @ 0x07: u8 = 0_0 {
        /// LCD Memory 7
        M7: 0..7 = struct M7Field(u8);
    }
    /// LCD Memory 8
    rw M8 @ 0x08: u8 = 0_0 {
        /// LCD Memory 8
        M8: 0..7 = struct M8Field(u8);
    }
    /// LCD Memory 9
    rw M9 @ 0x09: u8 = 0_0 {
        /// LCD Memory 9
        M9: 0..7 = struct M9Field(u8);
    }
    /// LCD Memory 10
    rw M10 @ 0x0a: u8 = 0_0 {
        /// LCD Memory 10
        M10: 0..7 = struct M10Field(u8);
    }
    /// LCD Memory 11
    rw M11 @ 0x0b: u8 = 0_0 {
        /// LCD Memory 11
        M11: 0..7 = struct M11Field(u8);
    }
    /// LCD Memory 12
    rw M12 @ 0x0c: u8 = 0_0 {
        /// LCD Memory 12
        M12: 0..7 = struct M12Field(u8);
    }
    /// LCD Memory 13
    rw M13 @ 0x0d: u8 = 0_0 {
        /// LCD Memory 13
        M13: 0..7 = struct M13Field(u8);
    }
    /// LCD Memory 14
    rw M14 @ 0x0e: u8 = 0_0 {
        /// LCD Memory 14
        M14: 0..7 = struct M14Field(u8);
    }
    /// LCD Memory 15
    rw M15 @ 0x0f: u8 = 0_0 {
        /// LCD Memory 15
        M15: 0..7 = struct M15Field(u8);
    }
    /// LCD Memory 16
    rw M16 @ 0x10: u8 = 0_0 {
        /// LCD Memory 16
        M16: 0..7 = struct M16Field(u8);
    }
    /// LCD Memory 17
    rw M17 @ 0x11: u8 = 0_0 {
        /// LCD Memory 17
        M17: 0..7 = struct M17Field(u8);
    }
    /// LCD Memory 18
    rw M18 @ 0x12: u8 = 0_0 {
        /// LCD Memory 18
        M18: 0..7 = struct M18Field(u8);
    }
    /// LCD Memory 19
    rw M19 @ 0x13: u8 = 0_0 {
        /// LCD Memory 19
        M19: 0..7 = struct M19Field(u8);
    }
    /// LCD Memory 20
    rw M20 @ 0x14: u8 = 0_0 {
        /// LCD Memory 20
        M20: 0..7 = struct M20Field(u8);
    }
}
