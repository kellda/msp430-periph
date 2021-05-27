//! Shared Reference

utils::periph! {
    /// Shared Reference
    SharedReference;
    /// REF Shared Reference control register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// REF Reference On
        ON: 0 = struct ON(bool);
        /// REF Reference output Buffer On
        OUT: 1 = struct OUT(bool);
        /// REF Temp.Sensor off
        TCOFF: 3 = struct TCOFF(bool);
        /// REF Reference Voltage Level Select Bit:0
        VSEL: 4..5 = enum VSEL {
            /// REF Reference Voltage Level Select 1.2V
            VSEL_0 = 0b00,
            /// REF Reference Voltage Level Select 2.0V
            VSEL_1 = 0b01,
            /// REF Reference Voltage Level Select 2.5V
            VSEL_2 = 0b10,
            /// REF Reference Voltage Level Select 2.5V
            VSEL_3 = 0b11,
        }
        /// REF Reference generator one-time trigger
        GENOT: 6 = struct GENOT(bool);
        /// REF Bandgap and bandgap buffer one-time trigger
        BGOT: 7 = struct BGOT(bool);
        /// REF Reference generator active
        GENACT: 8 = struct GENACT(bool);
        /// REF Reference bandgap active
        BGACT: 9 = struct BGACT(bool);
        /// REF Reference generator busy
        GENBUSY: 10 = struct GENBUSY(bool);
        /// REF Bandgap mode
        BGMODE: 11 = struct BGMODE(bool);
        /// REF Reference generator ready
        GENRDY: 12 = struct GENRDY(bool);
        /// REF Reference bandgap ready
        BGRDY: 13 = struct BGRDY(bool);
    }
}
