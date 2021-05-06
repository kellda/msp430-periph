//! EPROM

utils::periph! {
    /// EPROM
    EPROM;
    /// EPROM Control
    rw EPCTL @ 0x00: u8 = 0_0 {
        /// EPEXE
        EPEXE: 0 = struct EPEXE(bool);
        /// EPVPPS
        EPVPPS: 1 = struct EPVPPS(bool);
    }
}
