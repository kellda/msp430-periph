//! Backup Memory

utils::periph! {
    /// Backup Memory
    BackupMemory;
    /// Battery Backup Memory 0
    rw BAKMEM0 @ 0x00: u16 = 0_0 {
        /// Battery Backup Memory 0
        BAKMEM0: 0..15 = struct BAKMEM0Field(u16);
    }
    /// Battery Backup Memory 1
    rw BAKMEM1 @ 0x02: u16 = 0_0 {
        /// Battery Backup Memory 1
        BAKMEM1: 0..15 = struct BAKMEM1Field(u16);
    }
    /// Battery Backup Memory 2
    rw BAKMEM2 @ 0x04: u16 = 0_0 {
        /// Battery Backup Memory 2
        BAKMEM2: 0..15 = struct BAKMEM2Field(u16);
    }
    /// Battery Backup Memory 3
    rw BAKMEM3 @ 0x06: u16 = 0_0 {
        /// Battery Backup Memory 3
        BAKMEM3: 0..15 = struct BAKMEM3Field(u16);
    }
    /// Battery Backup Memory 4
    rw BAKMEM4 @ 0x08: u16 = 0_0 {
        /// Battery Backup Memory 4
        BAKMEM4: 0..15 = struct BAKMEM4Field(u16);
    }
    /// Battery Backup Memory 5
    rw BAKMEM5 @ 0x0a: u16 = 0_0 {
        /// Battery Backup Memory 5
        BAKMEM5: 0..15 = struct BAKMEM5Field(u16);
    }
    /// Battery Backup Memory 6
    rw BAKMEM6 @ 0x0c: u16 = 0_0 {
        /// Battery Backup Memory 6
        BAKMEM6: 0..15 = struct BAKMEM6Field(u16);
    }
    /// Battery Backup Memory 7
    rw BAKMEM7 @ 0x0e: u16 = 0_0 {
        /// Battery Backup Memory 7
        BAKMEM7: 0..15 = struct BAKMEM7Field(u16);
    }
    /// Battery Backup Memory 8
    rw BAKMEM8 @ 0x10: u16 = 0_0 {
        /// Battery Backup Memory 8
        BAKMEM8: 0..15 = struct BAKMEM8Field(u16);
    }
    /// Battery Backup Memory 9
    rw BAKMEM9 @ 0x12: u16 = 0_0 {
        /// Battery Backup Memory 9
        BAKMEM9: 0..15 = struct BAKMEM9Field(u16);
    }
    /// Battery Backup Memory 10
    rw BAKMEM10 @ 0x14: u16 = 0_0 {
        /// Battery Backup Memory 10
        BAKMEM10: 0..15 = struct BAKMEM10Field(u16);
    }
    /// Battery Backup Memory 11
    rw BAKMEM11 @ 0x16: u16 = 0_0 {
        /// Battery Backup Memory 11
        BAKMEM11: 0..15 = struct BAKMEM11Field(u16);
    }
    /// Battery Backup Memory 12
    rw BAKMEM12 @ 0x18: u16 = 0_0 {
        /// Battery Backup Memory 12
        BAKMEM12: 0..15 = struct BAKMEM12Field(u16);
    }
    /// Battery Backup Memory 13
    rw BAKMEM13 @ 0x1a: u16 = 0_0 {
        /// Battery Backup Memory 13
        BAKMEM13: 0..15 = struct BAKMEM13Field(u16);
    }
    /// Battery Backup Memory 14
    rw BAKMEM14 @ 0x1c: u16 = 0_0 {
        /// Battery Backup Memory 14
        BAKMEM14: 0..15 = struct BAKMEM14Field(u16);
    }
    /// Battery Backup Memory 15
    rw BAKMEM15 @ 0x1e: u16 = 0_0 {
        /// Battery Backup Memory 15
        BAKMEM15: 0..15 = struct BAKMEM15Field(u16);
    }
}
