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
            /// REF Reference Voltage Level Select 1.5V
            REFVSEL_0 = 0b00,
            /// REF Reference Voltage Level Select 2.0V
            REFVSEL_1 = 0b01,
            /// REF Reference Voltage Level Select 2.5V
            REFVSEL_2 = 0b10,
            /// REF Reference Voltage Level Select 2.5V
            REFVSEL_3 = 0b11,
        }
        /// REF Master Control
        REFMSTR: 7 = struct REFMSTR(bool);
        /// REF Reference generator active
        REFGENACT: 8 = struct REFGENACT(bool);
        /// REF Reference bandgap active
        REFBGACT: 9 = struct REFBGACT(bool);
        /// REF Reference generator busy
        REFGENBUSY: 10 = struct REFGENBUSY(bool);
        /// REF Bandgap mode
        BGMODE: 11 = struct BGMODE(bool);
    }
}
