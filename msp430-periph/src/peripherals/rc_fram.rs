//! RC  RAM Control Module (FRAM)

utils::periph! {
    /// RC  RAM Control Module (FRAM)
    RC;
    /// Ram Controller Control Register
    rw RCCTL0 @ 0x00: u16 = 0_0 {
        /// RAM Controller RAM Sector 0 Off Bit: 0
        RCRS0OFF: 0..1 = enum RCRS0OFF {
            /// RAM Controller RAM Sector 0 Off : 0
            RCRS0OFF_0 = 0b00,
            /// RAM Controller RAM Sector 0 Off : 1
            RCRS0OFF_1 = 0b01,
            /// RAM Controller RAM Sector 0 Off : 2
            RCRS0OFF_2 = 0b10,
            /// RAM Controller RAM Sector 0 Off : 3
            RCRS0OFF_3 = 0b11,
        }
        /// RAM Controller RAM Sector 1 Off Bit: 0
        RCRS1OFF: 2..3 = enum RCRS1OFF {
            /// RAM Controller RAM Sector 1 Off : 0
            RCRS1OFF_0 = 0b00,
            /// RAM Controller RAM Sector 1 Off : 1
            RCRS1OFF_1 = 0b01,
            /// RAM Controller RAM Sector 1 Off : 2
            RCRS1OFF_2 = 0b10,
            /// RAM Controller RAM Sector 1 Off : 3
            RCRS1OFF_3 = 0b11,
        }
        /// RAM Controller RAM Sector 2 Off Bit: 0
        RCRS2OFF: 4..5 = enum RCRS2OFF {
            /// RAM Controller RAM Sector 2 Off : 0
            RCRS2OFF_0 = 0b00,
            /// RAM Controller RAM Sector 2 Off : 1
            RCRS2OFF_1 = 0b01,
            /// RAM Controller RAM Sector 2 Off : 2
            RCRS2OFF_2 = 0b10,
            /// RAM Controller RAM Sector 2 Off : 3
            RCRS2OFF_3 = 0b11,
        }
        /// RAM Controller RAM Sector 3 Off Bit: 0
        RCRS3OFF: 6..7 = enum RCRS3OFF {
            /// RAM Controller RAM Sector 3 Off : 0
            RCRS3OFF_0 = 0b00,
            /// RAM Controller RAM Sector 3 Off : 1
            RCRS3OFF_1 = 0b01,
            /// RAM Controller RAM Sector 3 Off : 2
            RCRS3OFF_2 = 0b10,
            /// RAM Controller RAM Sector 3 Off : 3
            RCRS3OFF_3 = 0b11,
        }
    }
}
