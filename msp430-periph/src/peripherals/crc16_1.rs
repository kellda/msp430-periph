//! CRC16

utils::periph! {
    /// CRC16
    CRC16;
    /// CRC Data In Register
    rw DI @ 0x00: u16 = 0_0 {
        /// CRC Data In Register
        DI: 0..15 = struct DIField(u16);
    }
    /// CRC Initialisation Register and Result Register
    rw INIRES @ 0x04: u16 = 0_0 {
        /// CRC Initialisation Register and Result Register
        INIRES: 0..15 = struct INIRESField(u16);
    }
}
