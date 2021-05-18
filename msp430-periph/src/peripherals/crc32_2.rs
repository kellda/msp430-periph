//! CRC32

utils::periph! {
    /// CRC32
    CRC32;
    /// CRC32 Data Input Word 0
    rw CRC32DIW0 @ 0x00: u16 = 0_0 {
        /// CRC32 Data Input Word 0
        CRC32DIW0: 0..15 = struct CRC32DIW0Field(u16);
    }
    /// CRC32 Data Input Word 1
    rw CRC32DIW1 @ 0x02: u16 = 0_0 {
        /// CRC32 Data Input Word 1
        CRC32DIW1: 0..15 = struct CRC32DIW1Field(u16);
    }
    /// CRC32 Data In Reverse Word 1
    rw CRC32DIRBW1 @ 0x04: u16 = 0_0 {
        /// CRC32 Data In Reverse Word 1
        CRC32DIRBW1: 0..15 = struct CRC32DIRBW1Field(u16);
    }
    /// CRC32 Data In Reverse Word 0
    rw CRC32DIRBW0 @ 0x06: u16 = 0_0 {
        /// CRC32 Data In Reverse Word 0
        CRC32DIRBW0: 0..15 = struct CRC32DIRBW0Field(u16);
    }
    /// CRC32 Initialization and Result Word 0
    rw CRC32INIRESW0 @ 0x08: u16 = 0_0 {
        /// CRC32 Initialization and Result Word 0
        CRC32INIRESW0: 0..15 = struct CRC32INIRESW0Field(u16);
    }
    /// CRC32 Initialization and Result Word 1
    rw CRC32INIRESW1 @ 0x0a: u16 = 0_0 {
        /// CRC32 Initialization and Result Word 1
        CRC32INIRESW1: 0..15 = struct CRC32INIRESW1Field(u16);
    }
    /// CRC32 Result Reverse Word 1
    rw CRC32RESRW1 @ 0x0c: u16 = 0_0 {
        /// CRC32 Result Reverse Word 1
        CRC32RESRW1: 0..15 = struct CRC32RESRW1Field(u16);
    }
    /// CRC32 Result Reverse Word 0
    rw CRC32RESRW0 @ 0x0e: u16 = 0_0 {
        /// CRC32 Result Reverse Word 0
        CRC32RESRW0: 0..15 = struct CRC32RESRW0Field(u16);
    }
    /// CRC16 Data Input
    rw CRC16DIW0 @ 0x10: u16 = 0_0 {
        /// CRC16 Data Input
        CRC16DIW0: 0..15 = struct CRC16DIW0Field(u16);
    }
    /// CRC16 Data In Reverse
    rw CRC16DIRBW0 @ 0x16: u16 = 0_0 {
        /// CRC16 Data In Reverse
        CRC16DIRBW0: 0..15 = struct CRC16DIRBW0Field(u16);
    }
    /// CRC16 Init and Result
    rw CRC16INIRESW0 @ 0x18: u16 = 0_0 {
        /// CRC16 Init and Result
        CRC16INIRESW0: 0..15 = struct CRC16INIRESW0Field(u16);
    }
    /// CRC16 Result Reverse
    rw CRC16RESRW0 @ 0x1e: u16 = 0_0 {
        /// CRC16 Result Reverse
        CRC16RESRW0: 0..15 = struct CRC16RESRW0Field(u16);
    }
}
