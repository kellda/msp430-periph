//! Shared Reference

utils::periph! {
    /// Shared Reference
    SharedReference;
    /// REF Shared Reference control register 0
    rw REFCTL0 @ 0x00: u16 = 0_0 {
        /// REF Reference On
        REFON: 0 = struct REFON(bool);
        /// REF Reference output Buffer On
        REFOUT: 1 = struct REFOUT(bool);
        /// REF Temp.Sensor off
        REFTCOFF: 3 = struct REFTCOFF(bool);
        /// REF Reference Voltage Level Select Bit:0
        REFVSEL: 4..5 = enum REFVSEL {
            /// REF Reference Voltage Level Select 1.2V
            REFVSEL_0 = 0b00,
            /// REF Reference Voltage Level Select 2.0V
            REFVSEL_1 = 0b01,
            /// REF Reference Voltage Level Select 2.5V
            REFVSEL_2 = 0b10,
            /// REF Reference Voltage Level Select 2.5V
            REFVSEL_3 = 0b11,
        }
        /// REF Reference generator one-time trigger
        REFGENOT: 6 = struct REFGENOT(bool);
        /// REF Bandgap and bandgap buffer one-time trigger
        REFBGOT: 7 = struct REFBGOT(bool);
        /// REF Reference generator active
        REFGENACT: 8 = struct REFGENACT(bool);
        /// REF Reference bandgap active
        REFBGACT: 9 = struct REFBGACT(bool);
        /// REF Reference generator busy
        REFGENBUSY: 10 = struct REFGENBUSY(bool);
        /// REF Bandgap mode
        BGMODE: 11 = struct BGMODE(bool);
        /// REF Reference generator ready
        REFGENRDY: 12 = struct REFGENRDY(bool);
        /// REF Reference bandgap ready
        REFBGRDY: 13 = struct REFBGRDY(bool);
    }
}
