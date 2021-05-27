//! LCD

utils::periph! {
    /// LCD
    LCD;
    /// LCD Control
    rw CTL @ 0x00: u8 = 0_0 {
        /// LCDON
        ON: 0 = struct ON(bool);
        /// LCDSON
        SON: 2 = struct SON(bool);
        /// LCDMX0
        MX0: 3 = struct MX0(bool);
        /// LCDMX1
        MX1: 4 = struct MX1(bool);
        /// LCDP0
        P0: 5 = struct P0(bool);
        /// LCDP1
        P1: 6 = struct P1(bool);
        /// LCDP2
        P2: 7 = struct P2(bool);
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
