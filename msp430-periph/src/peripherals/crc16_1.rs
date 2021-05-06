//! CRC16

utils::periph! {
    /// CRC16
    CRC16;
    /// CRC Data In Register
    rw CRCDI @ 0x00: u16 = 0_0 {
        /// CRC Data In Register
        CRCDI: 0..15 = struct CRCDIField(u16);
    }
    /// CRC Initialisation Register and Result Register
    rw CRCINIRES @ 0x04: u16 = 0_0 {
        /// CRC Initialisation Register and Result Register
        CRCINIRES: 0..15 = struct CRCINIRESField(u16);
    }
}
