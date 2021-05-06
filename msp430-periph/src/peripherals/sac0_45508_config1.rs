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
}
