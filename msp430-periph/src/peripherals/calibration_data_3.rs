//! Calibration Data

utils::periph! {
    /// Calibration Data
    CalibrationData;
    /// DCOCTL  Calibration Data for 12MHz
    rw CALDCO_12MHZ @ 0x00: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 12MHz
        CALDCO_12MHZ: 0..7 = struct CALDCO_12MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 12MHz
    rw CALBC1_12MHZ @ 0x01: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 12MHz
        CALBC1_12MHZ: 0..7 = struct CALBC1_12MHZField(u8);
    }
    /// DCOCTL  Calibration Data for 8MHz
    rw CALDCO_8MHZ @ 0x02: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 8MHz
        CALDCO_8MHZ: 0..7 = struct CALDCO_8MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 8MHz
    rw CALBC1_8MHZ @ 0x03: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 8MHz
        CALBC1_8MHZ: 0..7 = struct CALBC1_8MHZField(u8);
    }
}
