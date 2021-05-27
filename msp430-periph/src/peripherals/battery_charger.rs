//! Battery Charger

utils::periph! {
    /// Battery Charger
    BatteryCharger;
    /// Battery Backup Control
    rw CTL @ 0x00: u16 = 0_0 {
        /// Lock backup sub-system
        LOCK: 0 = struct LOCK(bool);
        /// Manual switch to battery backup supply
        SW: 1 = struct SW(bool);
        /// Battery backup supply to ADC.
        ADC: 2 = struct ADC(bool);
        /// Disable backup supply switching.
        DIS: 3 = struct DIS(bool);
    }
    /// Battery Charger Control
    rw CHCTL @ 0x02: u16 = 0_0 {
        /// Charger enable
        EN: 0 = struct EN(bool);
        /// Charger charge current Bit 0
        C0: 1 = struct C0(bool);
        /// Charger charge current Bit 1
        C1: 2 = struct C1(bool);
        /// Charger end voltage Bit 0
        V0: 4 = struct V0(bool);
        /// Charger end voltage Bit 1
        V1: 5 = struct V1(bool);
    }
}
