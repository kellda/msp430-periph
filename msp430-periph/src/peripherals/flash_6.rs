//! Flash

utils::periph! {
    /// Flash
    Flash;
    /// FLASH Control 1
    rw CTL1 @ 0x00: u16 = 0_0 {
        /// Enable bit for Flash segment erase
        ERASE: 1 = struct ERASE(bool);
        /// Enable bit for Flash mass erase
        MERAS: 2 = struct MERAS(bool);
        /// Smart Write enable
        SWRT: 5 = struct SWRT(bool);
        /// Enable bit for Flash write
        WRT: 6 = struct WRT(bool);
        /// Enable bit for Flash segment write
        BLKWRT: 7 = struct BLKWRT(bool);
    }
    /// FLASH Control 3
    rw CTL3 @ 0x04: u16 = 0_0 {
        /// Flash busy: 1
        BUSY: 0 = struct BUSY(bool);
        /// Flash Key violation flag
        KEYV: 1 = struct KEYV(bool);
        /// Flash Access violation flag
        ACCVIFG: 2 = struct ACCVIFG(bool);
        /// Wait flag for segment write
        WAIT: 3 = struct WAIT(bool);
        /// Lock bit: 1 - Flash is locked (read only)
        LOCK: 4 = struct LOCK(bool);
        /// Flash Emergency Exit
        EMEX: 5 = struct EMEX(bool);
        /// Segment A Lock bit: read = 1 - Segment is locked (read only)
        LOCKA: 6 = struct LOCKA(bool);
    }
    /// FLASH Control 4
    rw CTL4 @ 0x06: u16 = 0_0 {
        /// Voltage Changed during Program Error Flag
        VPE: 0 = struct VPE(bool);
        /// Marginal read 0 mode.
        MGR0: 4 = struct MGR0(bool);
        /// Marginal read 1 mode.
        MGR1: 5 = struct MGR1(bool);
        /// Lock INFO Memory bit: read = 1 - Segment is locked (read only)
        LOCKINFO: 7 = struct LOCKINFO(bool);
    }
}
