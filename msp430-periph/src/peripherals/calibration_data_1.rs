//! Calibration Data

utils::periph! {
    /// Calibration Data
    CalibrationData;
    /// DCOCTL  Calibration Data for 16MHz
    rw CALDCO_16MHZ @ 0x00: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 16MHz
        CALDCO_16MHZ: 0..7 = struct CALDCO_16MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 16MHz
    rw CALBC1_16MHZ @ 0x01: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 16MHz
        CALBC1_16MHZ: 0..7 = struct CALBC1_16MHZField(u8);
    }
    /// DCOCTL  Calibration Data for 12MHz
    rw CALDCO_12MHZ @ 0x02: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 12MHz
        CALDCO_12MHZ: 0..7 = struct CALDCO_12MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 12MHz
    rw CALBC1_12MHZ @ 0x03: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 12MHz
        CALBC1_12MHZ: 0..7 = struct CALBC1_12MHZField(u8);
    }
    /// DCOCTL  Calibration Data for 8MHz
    rw CALDCO_8MHZ @ 0x04: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 8MHz
        CALDCO_8MHZ: 0..7 = struct CALDCO_8MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 8MHz
    rw CALBC1_8MHZ @ 0x05: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 8MHz
        CALBC1_8MHZ: 0..7 = struct CALBC1_8MHZField(u8);
    }
    /// DCOCTL  Calibration Data for 1MHz
    rw CALDCO_1MHZ @ 0x06: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 1MHz
        CALDCO_1MHZ: 0..7 = struct CALDCO_1MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 1MHz
    rw CALBC1_1MHZ @ 0x07: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 1MHz
        CALBC1_1MHZ: 0..7 = struct CALBC1_1MHZField(u8);
    }
}
