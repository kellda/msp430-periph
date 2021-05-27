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
        /// VLD0
        VLD0: 4 = struct VLD0(bool);
        /// VLD1
        VLD1: 5 = struct VLD1(bool);
        /// VLD2
        VLD2: 6 = struct VLD2(bool);
        /// VLD3
        VLD3: 7 = struct VLD3(bool);
    }
}
