//! LCD

utils::periph! {
    /// LCD
    LCD;
    /// LCD Control
    rw LCDCTL @ 0x00: u8 = 0_0 {
        /// LCDON
        LCDON: 0 = struct LCDON(bool);
        /// LCDLOWR
        LCDLOWR: 1 = struct LCDLOWR(bool);
        /// LCDSON
        LCDSON: 2 = struct LCDSON(bool);
        /// LCDMX0
        LCDMX0: 3 = struct LCDMX0(bool);
        /// LCDMX1
        LCDMX1: 4 = struct LCDMX1(bool);
        /// LCDP0
        LCDP0: 5 = struct LCDP0(bool);
        /// LCDP1
        LCDP1: 6 = struct LCDP1(bool);
        /// LCDP2
        LCDP2: 7 = struct LCDP2(bool);
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
}
