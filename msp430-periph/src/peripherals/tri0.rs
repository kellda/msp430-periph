//! TRI0

utils::periph! {
    /// TRI0
    TRI0;
    /// TRI Control Register
    rw CTL @ 0x00: u16 = 0_0 {
        /// TRI enable.
        EN: 0 = enum EN {
            /// Disable the TRI module
            DISABLE = 0b0,
            /// Enable the TRI module
            ENABLE = 0b1,
        }
        /// TRI power mode select.
        PM: 1 = enum PM {
            /// High speed and high power
            PM_0 = 0b0,
            /// Low speed and low power
            PM_1 = 0b1,
        }
        /// TRI positive input select.
        PSEL: 6..7 = enum PSEL {
            /// External source (package terminal)
            PSEL_0 = 0b00,
            /// Device specific; refer to the device-specific sheet for details
            PSEL_1 = 0b01,
            /// Device specific; refer to the device-specific sheet for details
            PSEL_2 = 0b10,
            /// Device specific; refer to the device-specific sheet for details
            PSEL_3 = 0b11,
        }
    }
}
