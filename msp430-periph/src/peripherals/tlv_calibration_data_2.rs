//! TLV Calibration Data

utils::periph! {
    /// TLV Calibration Data
    TLVCalibrationData;
    /// TLV CHECK SUM
    rw CHECKSUM @ 0x00: u16 = 0_0 {
        /// TLV CHECK SUM
        CHECKSUM: 0..15 = struct CHECKSUMField(u16);
    }
    /// TLV TAG_DCO30 TAG
    rw DCO_30_TAG @ 0x36: u8 = 0_0 {
        /// TLV TAG_DCO30 TAG
        DCO_30_TAG: 0..7 = struct DCO_30_TAGField(u8);
    }
    /// TLV TAG_DCO30 LEN
    rw DCO_30_LEN @ 0x37: u8 = 0_0 {
        /// TLV TAG_DCO30 LEN
        DCO_30_LEN: 0..7 = struct DCO_30_LENField(u8);
    }
    /// TLV ADC12_1 TAG
    rw ADC12_1_TAG @ 0x1a: u8 = 0_0 {
        /// TLV ADC12_1 TAG
        ADC12_1_TAG: 0..7 = struct ADC12_1_TAGField(u8);
    }
    /// TLV ADC12_1 LEN
    rw ADC12_1_LEN @ 0x1b: u8 = 0_0 {
        /// TLV ADC12_1 LEN
        ADC12_1_LEN: 0..7 = struct ADC12_1_LENField(u8);
    }
}
