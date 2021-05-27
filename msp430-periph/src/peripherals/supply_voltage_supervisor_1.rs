//! Supply Voltage Supervisor

utils::periph! {
    /// Supply Voltage Supervisor
    SupplyVoltageSupervisor;
    /// SVS Control
    rw CTL @ 0x00: u8 = 0_0 {
        /// SVS Flag
        SVSFG: 0 = struct SVSFG(bool);
        /// SVS output (read only)
        SVSOP: 1 = struct SVSOP(bool);
        /// Switches the SVS on/off
        SVSON: 2 = struct SVSON(bool);
        /// Enable POR Generation if Low Voltage
        PORON: 3 = struct PORON(bool);
        /// VLDON
        VLDON: 4 = struct VLDON(bool);
    }
}
