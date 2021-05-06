//! Calibration Data

utils::periph! {
    /// Calibration Data
    CalibrationData;
    /// DCOCTL  Calibration Data for 1MHz
    rw CALDCO_1MHZ @ 0x00: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 1MHz
        CALDCO_1MHZ: 0..7 = struct CALDCO_1MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 1MHz
    rw CALBC1_1MHZ @ 0x01: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 1MHz
        CALBC1_1MHZ: 0..7 = struct CALBC1_1MHZField(u8);
    }
}
