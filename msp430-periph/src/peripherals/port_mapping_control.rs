//! Port Mapping Control

utils::periph! {
    /// Port Mapping Control
    PortMappingControl;
    /// Port Mapping Key register
    rw KEYID @ 0x00: u16 = 0_0 {
        /// Port Mapping Key register
        KEYID: 0..15 = struct KEYIDField(u16);
    }
    /// Port Mapping control register
    rw CTL @ 0x02: u16 = 0_0 {
        /// Port Mapping Lock bit. Read only
        LOCKED: 0 = struct LOCKED(bool);
        /// Port Mapping re-configuration control bit
        RECFG: 1 = struct RECFG(bool);
    }
}
