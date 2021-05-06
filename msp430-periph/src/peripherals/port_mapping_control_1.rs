//! Port Mapping Control

utils::periph! {
    /// Port Mapping Control
    PortMappingControl;
    /// Port Mapping Key register
    rw PMAPKEYID @ 0x00: u16 = 0_0 {
        /// Port Mapping Key register
        PMAPKEYID: 0..15 = struct PMAPKEYIDField(u16);
    }
    /// Port Mapping control register
    rw PMAPCTL @ 0x02: u16 = 0_0 {
        /// Port Mapping Lock bit. Read only
        PMAPLOCKED: 0 = struct PMAPLOCKED(bool);
        /// Port Mapping re-configuration control bit
        PMAPRECFG: 1 = struct PMAPRECFG(bool);
    }
}
