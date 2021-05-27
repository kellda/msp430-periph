//! RAMCTL

utils::periph! {
    /// RAMCTL
    RAMCTL;
    /// RAM Controller Control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// RAM controller RAM sector 0 off
        RS0OFF: 0..1 = enum RS0OFF {
            /// Contents of this RAM sector are retained in LPM3 and LPM4.
            RS0OFF_0 = 0b00,
            /// Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RS0OFF_1 = 0b01,
            /// Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RS0OFF_2 = 0b10,
        }
        /// RAM controller key. Always reads as 69h. Must be written as 5Ah; any other write is is ignored.
        KEY: 8..15 = struct KEY(u16);
        /// RAM controller RAM sector 0 off
        RS1OFF: 2..3 = enum RS1OFF {
            /// Contents of this RAM sector are retained in LPM3 and LPM4.
            RS0OFF_0 = 0b00,
            /// Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RS0OFF_1 = 0b01,
            /// Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RS0OFF_2 = 0b10,
        }
        /// RAM controller RAM sector 3 off
        RS3OFF: 6..7 = enum RS3OFF {
            /// Contents of this RAM sector are retained in LPM3 and LPM4.
            RS0OFF_0 = 0b00,
            /// Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RS0OFF_1 = 0b01,
            /// Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RS0OFF_2 = 0b10,
        }
        /// RAM controller RAM sector 0 off
        RS2OFF: 4..5 = enum RS2OFF {
            /// Contents of this RAM sector are retained in LPM3 and LPM4.
            RS0OFF_0 = 0b00,
            /// Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RS0OFF_1 = 0b01,
            /// Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RS0OFF_2 = 0b10,
        }
    }
}
