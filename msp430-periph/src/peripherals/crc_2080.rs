//! CRC

utils::periph! {
    /// CRC
    CRC;
    /// CRC Data In
    rw CRCDI @ 0x00: u16 = 0_0 {
        /// CRC Data In
        CRCDI: 0..15 = struct CRCDIField(u16);
    }
    /// CRC Data In Reverse Byte
    rw CRCDIRB @ 0x02: u16 = 0_0 {
        /// CRC Data In Reverse Byte
        CRCDIRB: 0..15 = struct CRCDIRBField(u16);
    }
    /// CRC Initialization and Result
    rw CRCINIRES @ 0x04: u16 = 0_0 {
        /// CRC Initialization and Result
        CRCINIRES: 0..15 = struct CRCINIRESField(u16);
    }
    /// CRC Result Reverse
    rw CRCRESR @ 0x06: u16 = 0_0 {
        /// CRC Result Reverse
        CRCRESR: 0..15 = struct CRCRESRField(u16);
    }
}
