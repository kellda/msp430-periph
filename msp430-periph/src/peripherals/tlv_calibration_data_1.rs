//! TLV Calibration Data

utils::periph! {
    /// TLV Calibration Data
    TLVCalibrationData;
    /// TLV CHECK SUM
    rw TLV_CHECKSUM @ 0x00: u16 = 0_0 {
        /// TLV CHECK SUM
        TLV_CHECKSUM: 0..15 = struct TLV_CHECKSUMField(u16);
    }
    /// TLV TAG_DCO30 TAG
    rw TLV_DCO_30_TAG @ 0x36: u8 = 0_0 {
        /// TLV TAG_DCO30 TAG
        TLV_DCO_30_TAG: 0..7 = struct TLV_DCO_30_TAGField(u8);
    }
    /// TLV TAG_DCO30 LEN
    rw TLV_DCO_30_LEN @ 0x37: u8 = 0_0 {
        /// TLV TAG_DCO30 LEN
        TLV_DCO_30_LEN: 0..7 = struct TLV_DCO_30_LENField(u8);
    }
    /// TLV ADC10_1 TAG
    rw TLV_ADC10_1_TAG @ 0x1a: u8 = 0_0 {
        /// TLV ADC10_1 TAG
        TLV_ADC10_1_TAG: 0..7 = struct TLV_ADC10_1_TAGField(u8);
    }
    /// TLV ADC10_1 LEN
    rw TLV_ADC10_1_LEN @ 0x1b: u8 = 0_0 {
        /// TLV ADC10_1 LEN
        TLV_ADC10_1_LEN: 0..7 = struct TLV_ADC10_1_LENField(u8);
    }
}
