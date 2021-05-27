//! Calibration Data

utils::periph! {
    /// Calibration Data
    CalibrationData;
    /// DCOCTL  Calibration Data for 1MHz
    rw DCO_1MHZ @ 0x00: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 1MHz
        DCO_1MHZ: 0..7 = struct DCO_1MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 1MHz
    rw BC1_1MHZ @ 0x01: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 1MHz
        BC1_1MHZ: 0..7 = struct BC1_1MHZField(u8);
    }
}
