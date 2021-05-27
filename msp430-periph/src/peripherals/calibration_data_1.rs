//! Calibration Data

utils::periph! {
    /// Calibration Data
    CalibrationData;
    /// DCOCTL  Calibration Data for 16MHz
    rw DCO_16MHZ @ 0x00: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 16MHz
        DCO_16MHZ: 0..7 = struct DCO_16MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 16MHz
    rw BC1_16MHZ @ 0x01: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 16MHz
        BC1_16MHZ: 0..7 = struct BC1_16MHZField(u8);
    }
    /// DCOCTL  Calibration Data for 12MHz
    rw DCO_12MHZ @ 0x02: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 12MHz
        DCO_12MHZ: 0..7 = struct DCO_12MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 12MHz
    rw BC1_12MHZ @ 0x03: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 12MHz
        BC1_12MHZ: 0..7 = struct BC1_12MHZField(u8);
    }
    /// DCOCTL  Calibration Data for 8MHz
    rw DCO_8MHZ @ 0x04: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 8MHz
        DCO_8MHZ: 0..7 = struct DCO_8MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 8MHz
    rw BC1_8MHZ @ 0x05: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 8MHz
        BC1_8MHZ: 0..7 = struct BC1_8MHZField(u8);
    }
    /// DCOCTL  Calibration Data for 1MHz
    rw DCO_1MHZ @ 0x06: u8 = 0_0 {
        /// DCOCTL  Calibration Data for 1MHz
        DCO_1MHZ: 0..7 = struct DCO_1MHZField(u8);
    }
    /// BCSCTL1 Calibration Data for 1MHz
    rw BC1_1MHZ @ 0x07: u8 = 0_0 {
        /// BCSCTL1 Calibration Data for 1MHz
        BC1_1MHZ: 0..7 = struct BC1_1MHZField(u8);
    }
}
