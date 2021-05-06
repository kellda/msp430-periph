//! CRC16

utils::periph! {
    /// CRC16
    CRC16;
    /// CRC Data In Register
    rw CRCDI @ 0x00: u16 = 0_0 {
        /// CRC Data In Register
        CRCDI: 0..15 = struct CRCDIField(u16);
    }
    /// CRC data in reverse byte Register
    rw CRCDIRB @ 0x02: u16 = 0_0 {
        /// CRC data in reverse byte Register
        CRCDIRB: 0..15 = struct CRCDIRBField(u16);
    }
    /// CRC Initialisation Register and Result Register
    rw CRCINIRES @ 0x04: u16 = 0_0 {
        /// CRC Initialisation Register and Result Register
        CRCINIRES: 0..15 = struct CRCINIRESField(u16);
    }
    /// CRC reverse result Register
    rw CRCRESR @ 0x06: u16 = 0_0 {
        /// CRC reverse result Register
        CRCRESR: 0..15 = struct CRCRESRField(u16);
    }
}
