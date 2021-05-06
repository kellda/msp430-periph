//! SAC0

utils::periph! {
    /// SAC0
    SAC0;
    /// SAC OA Control Register
    rw SAC0OA @ 0x00: u16 = 0_0 {
        /// SAC OA Positive input source selection
        PSEL: 0..1 = enum PSEL {
            /// External source selected
            PSEL_0 = 0b00,
            /// 12-bit reference DAC source selected
            PSEL_1 = 0b01,
            /// Pair OA source selected
            PSEL_2 = 0b10,
        }
        /// SAC Positive input MUX control.
        PMUXEN: 3..3 = enum PMUXEN {
            /// All positive input sources are disconnected to OA positive port
            PMUXEN_0 = 0b0,
            /// All positive input sources are connected to OA positive port
            PMUXEN_1 = 0b1,
        }
        /// SAC OA Negative input source selection
        NSEL: 4..5 = enum NSEL {
            /// External source selected
            NSEL_0 = 0b00,
            /// PGA source selected
            NSEL_1 = 0b01,
            /// Device Specific
            NSEL_2 = 0b10,
        }
        /// SAC Negative input MUX controL
        NMUXEN: 7..7 = enum NMUXEN {
            /// All negative input sources are disconnected to OA negative port
            NMUXEN_0 = 0b0,
            /// All negative input sources are connected to OA negative port
            NMUXEN_1 = 0b1,
        }
        /// SAC OA Enable selection
        OAEN: 8..8 = enum OAEN {
            /// SAC OA is disabled, then the SAC OA output high impedance
            OAEN_0 = 0b0,
            /// SAC OA is enabled, normal mode
            OAEN_1 = 0b1,
        }
        /// SAC OA power mode selection
        OAPM: 9..9 = enum OAPM {
            /// High speed and high power
            OAPM_0 = 0b0,
            /// Llow speed and low power
            OAPM_1 = 0b1,
        }
        /// SAC Enable selection
        SACEN: 10..10 = enum SACEN {
            /// SAC all modules are disabled, then the SAC output high impedance
            SACEN_0 = 0b0,
            /// SAC all modules are enabled, normal mode
            SACEN_1 = 0b1,
        }
    }
    /// SAC PGA Control Register
    rw SAC0PGA @ 0x02: u16 = 0_0 {
        /// SAC PGA Mode Selection
        MSEL: 0..1 = enum MSEL {
            /// Inverting PGA mode (external pad IN- is selected)
            MSEL_0 = 0b00,
            /// Buffer mode (floating is selected )
            MSEL_1 = 0b01,
            /// Non-inverting mode
            MSEL_2 = 0b10,
            /// Cascade OA Inverting mode
            MSEL_3 = 0b11,
        }
        /// SAC PGA Gain configuration
        GAIN: 4..6 = struct GAIN(u16);
    }
    /// SAC DAC Control Register
    rw SAC0DAC @ 0x04: u16 = 0_0 {
        /// SAC DAC enable
        DACEN: 0..0 = enum DACEN {
            /// Disabled
            DACEN_0 = 0b0,
            /// Enabled
            DACEN_1 = 0b1,
        }
        /// SAC DAC interrupt enable
        DACIE: 1..1 = enum DACIE {
            /// Disabled
            DACIE_0 = 0b0,
            /// Enabled
            DACIE_1 = 0b1,
        }
        /// SAC DAC DMA request enable
        DACDMAE: 2..2 = enum DACDMAE {
            /// DMA request disabled
            DACDMAE_0 = 0b0,
            /// DMA request enabled
            DACDMAE_1 = 0b1,
        }
        /// SAC DAC load select. Selects the load trigger for the DAC latch.
        DACLSEL: 8..9 = enum DACLSEL {
            /// DAC latch loads when DACDAT written
            DACLSEL_0 = 0b00,
            /// Device specific 0. DAC always loads data from DACDAT at the positive edge of this signal
            DACLSEL_2 = 0b10,
            /// Device specific 1. DAC always loads data from DACDAT at the positive edge of this signal
            DACLSEL_3 = 0b11,
        }
        /// SAC DAC select reference voltage
        DACSREF: 12..12 = enum DACSREF {
            /// AVCC
            DACSREF_0 = 0b0,
            /// Alternative reference
            DACSREF_1 = 0b1,
        }
    }
    /// SAC DAC Data Register
    rw SAC0DAT @ 0x06: u16 = 0_0 {
        /// SAC DAC data in unsigned format.
        DACData: 0..11 = struct DACData(u16);
    }
    /// SAC DAC Status Register
    rw SAC0DACSTS @ 0x08: u16 = 0_0 {
        /// SAC DAC data update flag
        DACIFG: 0 = struct DACIFG(bool);
    }
    /// SAC Interrupt Vector Register
    r SAC0IV @ 0x0a: u16 = 0_0 {
        /// SAC Interrupt Vector Register
        SACIV0: 0..15 = enum SACIV0 {
            /// No interrupt pending
            SACIV_0 = 0b0000000000000000,
            /// S&H completed interrupt flag (Highest priority)
            SACIV_2 = 0b0000000000000010,
            /// DAC channel update interrupt flag
            SACIV_4 = 0b0000000000000100,
        }
    }
    /// SAC S&H Circuitry Register
    rw SAC0SHC @ 0x0c: u16 = 0_0 {
        /// SAC S&H interrupt enable.
        SHIE: 0..0 = enum SHIE {
            /// S&H interrupt disabled
            SHIE_0 = 0b0,
            /// S&H interrupt enabled
            SHIE_1 = 0b1,
        }
        /// SAC Counter Clock source selection
        SHCS: 1 = struct SHCS(bool);
        /// SAC S&H mode select.
        SHMD: 2..3 = enum SHMD {
            /// S&H interrupt enabled
            SHMD_0 = 0b00,
            /// S&H Channel 1 enabled
            SHMD_1 = 0b01,
            /// S&H Channel 2 enabled
            SHMD_2 = 0b10,
            /// S&H Channel 1 and 2 enabled
            SHMD_3 = 0b11,
        }
        /// SAC S&H Trigger source select
        SHTRG: 4..5 = enum SHTRG {
            /// External source is selected
            SHTRG_0 = 0b00,
            /// Internal Timer is selected
            SHTRG_1 = 0b01,
            /// Device Specific
            SHTRG_2 = 0b10,
            /// Bypass.
            SHTRG_3 = 0b11,
        }
        /// SAC S&H Trigger source Edge select.
        EDGSEL: 6..7 = enum EDGSEL {
            /// Rising edge is selected
            EDGSEL_0 = 0b00,
            /// Falling edge is selected
            EDGSEL_1 = 0b01,
            /// Either rising or falling edge is selected
            EDGSEL_2 = 0b10,
        }
        /// SAC S&H Counter
        SHCNT: 8..13 = struct SHCNT(u16);
        /// SAC S&H internal connector switch control
        SHINCON: 15..15 = enum SHINCON {
            /// Internal connector switch is off
            SHINCON_0 = 0b0,
            /// Internal connector switch is on
            SHINCON_1 = 0b1,
        }
    }
    /// SAC S&H Status Register
    rw SAC0SHSTS @ 0x0e: u16 = 0_0 {
        /// SAC S&H completed flag
        SHIFG: 0..0 = enum SHIFG {
            /// No S&H interrupt
            SHIFG_0 = 0b0,
            /// S&H interrupt present
            SHIFG_1 = 0b1,
        }
    }
}
