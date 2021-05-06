//! TRI0

utils::periph! {
    /// TRI0
    TRI0;
    /// TRI Control Register
    rw TRI0CTL @ 0x00: u16 = 0_0 {
        /// TRI enable.
        TRIEN: 0..0 = enum TRIEN {
            /// Disable the TRI module
            DISABLE = 0b0,
            /// Enable the TRI module
            ENABLE = 0b1,
        }
        /// TRI power mode select.
        TRIPM: 1..1 = enum TRIPM {
            /// High speed and high power
            TRIPM_0 = 0b0,
            /// Low speed and low power
            TRIPM_1 = 0b1,
        }
        /// TRI positive input select.
        TRIPSEL: 6..7 = enum TRIPSEL {
            /// External source (package terminal)
            TRIPSEL_0 = 0b00,
            /// Device specific; refer to the device-specific sheet for details
            TRIPSEL_1 = 0b01,
            /// Device specific; refer to the device-specific sheet for details
            TRIPSEL_2 = 0b10,
            /// Device specific; refer to the device-specific sheet for details
            TRIPSEL_3 = 0b11,
        }
    }
}
