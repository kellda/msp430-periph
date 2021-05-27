//! EPROM

utils::periph! {
    /// EPROM
    EPROM;
    /// EPROM Control
    rw CTL @ 0x00: u8 = 0_0 {
        /// EPEXE
        EXE: 0 = struct EXE(bool);
        /// EPVPPS
        VPPS: 1 = struct VPPS(bool);
    }
}
