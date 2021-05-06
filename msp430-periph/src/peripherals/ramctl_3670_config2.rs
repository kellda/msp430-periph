//! RAMCTL

utils::periph! {
    /// RAMCTL
    RAMCTL;
    /// RAM Controller Control 0
    rw RCCTL0 @ 0x00: u16 = 0_0 {
        /// RAM controller RAM sector 0 off
        RCRS0OFF: 0..1 = enum RCRS0OFF {
            /// Contents of this RAM sector are retained in LPM3 and LPM4.
            RCRS0OFF_0 = 0b00,
            /// Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RCRS0OFF_1 = 0b01,
            /// Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RCRS0OFF_2 = 0b10,
        }
        /// RAM controller key. Always reads as 69h. Must be written as 5Ah; any other write is is ignored.
        RCKEY: 8..15 = struct RCKEY(u16);
        /// RAM controller RAM sector 1 off
        RCRS1OFF: 2..3 = enum RCRS1OFF {
            /// Contents of this RAM sector are retained in LPM3 and LPM4.
            RCRS0OFF_0 = 0b00,
            /// Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RCRS0OFF_1 = 0b01,
            /// Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RCRS0OFF_2 = 0b10,
        }
        /// RAM controller RAM sector 3 off
        RCRS3OFF: 6..7 = enum RCRS3OFF {
            /// Contents of this RAM sector are retained in LPM3 and LPM4.
            RCRS0OFF_0 = 0b00,
            /// Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RCRS0OFF_1 = 0b01,
            /// Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RCRS0OFF_2 = 0b10,
        }
        /// RAM controller RAM sector 2 off
        RCRS2OFF: 4..5 = enum RCRS2OFF {
            /// Contents of this RAM sector are retained in LPM3 and LPM4.
            RCRS0OFF_0 = 0b00,
            /// Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RCRS0OFF_1 = 0b01,
            /// Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector.
            RCRS0OFF_2 = 0b10,
        }
    }
    /// RAM Controller Control 1
    rw RCCTL1 @ 0x02: u16 = 0_0 {
        /// DACCESS Bus Error NMI enable
        DACCESSIE: 8..8 = enum DACCESSIE {
            /// Disable NMI for DACCESS Interrupt
            DACCESSIE_0 = 0b0,
            /// Enable NMI for DACCESS Interrupt
            DACCESSIE_1 = 0b1,
        }
        /// DACCESS Interrupt Flag
        DACCESSIFG: 0..0 = enum DACCESSIFG {
            /// DACCESS Interrupt is not pending
            DACCESSIFG_0 = 0b0,
            /// DACCESS Interrupt is pending.
            DACCESSIFG_1 = 0b1,
        }
    }
}
