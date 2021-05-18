//! Battery Charger

utils::periph! {
    /// Battery Charger
    BatteryCharger;
    /// Battery Backup Control
    rw BAKCTL @ 0x00: u16 = 0_0 {
        /// Lock backup sub-system
        LOCKBAK: 0 = struct LOCKBAK(bool);
        /// Manual switch to battery backup supply
        BAKSW: 1 = struct BAKSW(bool);
        /// Battery backup supply to ADC.
        BAKADC: 2 = struct BAKADC(bool);
        /// Disable backup supply switching.
        BAKDIS: 3 = struct BAKDIS(bool);
    }
    /// Battery Charger Control
    rw BAKCHCTL @ 0x02: u16 = 0_0 {
        /// Charger enable
        CHEN: 0 = struct CHEN(bool);
        /// Charger charge current Bit 0
        CHC0: 1 = struct CHC0(bool);
        /// Charger charge current Bit 1
        CHC1: 2 = struct CHC1(bool);
        /// Charger end voltage Bit 0
        CHV0: 4 = struct CHV0(bool);
        /// Charger end voltage Bit 1
        CHV1: 5 = struct CHV1(bool);
    }
}
