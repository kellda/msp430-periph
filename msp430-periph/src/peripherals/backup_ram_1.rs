//! Backup RAM

utils::periph! {
    /// Backup RAM
    BackupRAM;
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
}
