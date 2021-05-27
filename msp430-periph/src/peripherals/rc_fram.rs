//! RC  RAM Control Module (FRAM)

utils::periph! {
    /// RC  RAM Control Module (FRAM)
    RC;
    /// Ram Controller Control Register
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// RAM Controller RAM Sector 0 Off Bit: 0
        RS0OFF: 0..1 = enum RS0OFF {
            /// RAM Controller RAM Sector 0 Off : 0
            RS0OFF_0 = 0b00,
            /// RAM Controller RAM Sector 0 Off : 1
            RS0OFF_1 = 0b01,
            /// RAM Controller RAM Sector 0 Off : 2
            RS0OFF_2 = 0b10,
            /// RAM Controller RAM Sector 0 Off : 3
            RS0OFF_3 = 0b11,
        }
        /// RAM Controller RAM Sector 1 Off Bit: 0
        RS1OFF: 2..3 = enum RS1OFF {
            /// RAM Controller RAM Sector 1 Off : 0
            RS1OFF_0 = 0b00,
            /// RAM Controller RAM Sector 1 Off : 1
            RS1OFF_1 = 0b01,
            /// RAM Controller RAM Sector 1 Off : 2
            RS1OFF_2 = 0b10,
            /// RAM Controller RAM Sector 1 Off : 3
            RS1OFF_3 = 0b11,
        }
        /// RAM Controller RAM Sector 2 Off Bit: 0
        RS2OFF: 4..5 = enum RS2OFF {
            /// RAM Controller RAM Sector 2 Off : 0
            RS2OFF_0 = 0b00,
            /// RAM Controller RAM Sector 2 Off : 1
            RS2OFF_1 = 0b01,
            /// RAM Controller RAM Sector 2 Off : 2
            RS2OFF_2 = 0b10,
            /// RAM Controller RAM Sector 2 Off : 3
            RS2OFF_3 = 0b11,
        }
        /// RAM Controller RAM Sector 3 Off Bit: 0
        RS3OFF: 6..7 = enum RS3OFF {
            /// RAM Controller RAM Sector 3 Off : 0
            RS3OFF_0 = 0b00,
            /// RAM Controller RAM Sector 3 Off : 1
            RS3OFF_1 = 0b01,
            /// RAM Controller RAM Sector 3 Off : 2
            RS3OFF_2 = 0b10,
            /// RAM Controller RAM Sector 3 Off : 3
            RS3OFF_3 = 0b11,
        }
    }
}
