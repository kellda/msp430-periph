//! CRC16

utils::periph! {
    /// CRC16
    CRC16;
    /// CRC Data In Register
    rw DI @ 0x00: u16 = 0_0 {
        /// CRC Data In Register
        DI: 0..15 = struct DIField(u16);
    }
    /// CRC data in reverse byte Register
    rw DIRB @ 0x02: u16 = 0_0 {
        /// CRC data in reverse byte Register
        DIRB: 0..15 = struct DIRBField(u16);
    }
    /// CRC Initialisation Register and Result Register
    rw INIRES @ 0x04: u16 = 0_0 {
        /// CRC Initialisation Register and Result Register
        INIRES: 0..15 = struct INIRESField(u16);
    }
    /// CRC reverse result Register
    rw RESR @ 0x06: u16 = 0_0 {
        /// CRC reverse result Register
        RESR: 0..15 = struct RESRField(u16);
    }
}
