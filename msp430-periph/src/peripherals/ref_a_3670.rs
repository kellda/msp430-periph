//! REF_A

utils::periph! {
    /// REF_A
    REF_A;
    /// REF Control Register 0
    rw REFCTL0 @ 0x00: u16 = 0_0 {
        /// Reference enable
        REFON: 0..0 = enum REFON {
            /// Disables reference if no other reference requests are pending
            REFON_0 = 0b0,
            /// Enables reference in static mode
            REFON_1 = 0b1,
        }
        /// Reference output buffer
        REFOUT: 1..1 = enum REFOUT {
            /// Reference output not available externally
            REFOUT_0 = 0b0,
            /// Reference output available externally. If ADC14REFBURST = 0, output is available continuously. If ADC14REFBURST = 1, output is available only during an ADC14 conversion.
            REFOUT_1 = 0b1,
        }
        /// Temperature sensor disabled
        REFTCOFF: 3..3 = enum REFTCOFF {
            /// Temperature sensor enabled
            REFTCOFF_0 = 0b0,
            /// Temperature sensor disabled to save power
            REFTCOFF_1 = 0b1,
        }
        /// Reference voltage level select
        REFVSEL: 4..5 = enum REFVSEL {
            /// 1.2 V available when reference requested or REFON = 1
            REFVSEL_0 = 0b00,
            /// 2.0 V available when reference requested or REFON = 1
            REFVSEL_1 = 0b01,
            /// 2.5 V available when reference requested or REFON = 1
            REFVSEL_2 = 0b10,
            /// 2.5 V available when reference requested or REFON = 1
            REFVSEL_3 = 0b11,
        }
        /// Reference generator one-time trigger
        REFGENOT: 6..6 = enum REFGENOT {
            /// No trigger
            REFGENOT_0 = 0b0,
            /// Generation of the reference voltage is started by writing 1 or by a hardware trigger
            REFGENOT_1 = 0b1,
        }
        /// Bandgap and bandgap buffer one-time trigger
        REFBGOT: 7..7 = enum REFBGOT {
            /// No trigger
            REFBGOT_0 = 0b0,
            /// Generation of the bandgap voltage is started by writing 1 or by a hardware trigger
            REFBGOT_1 = 0b1,
        }
        /// Reference generator active
        REFGENACT: 8..8 = enum REFGENACT {
            /// Reference generator not active
            REFGENACT_0 = 0b0,
            /// Reference generator active
            REFGENACT_1 = 0b1,
        }
        /// Reference bandgap active
        REFBGACT: 9..9 = enum REFBGACT {
            /// Reference bandgap buffer not active
            REFBGACT_0 = 0b0,
            /// Reference bandgap buffer active
            REFBGACT_1 = 0b1,
        }
        /// Reference generator busy
        REFGENBUSY: 10..10 = enum REFGENBUSY {
            /// Reference generator not busy
            REFGENBUSY_0 = 0b0,
            /// Reference generator busy
            REFGENBUSY_1 = 0b1,
        }
        /// Bandgap mode
        BGMODE: 11..11 = enum BGMODE {
            /// Static mode
            BGMODE_0 = 0b0,
            /// Sampled mode
            BGMODE_1 = 0b1,
        }
        /// Variable reference voltage ready status
        REFGENRDY: 12..12 = enum REFGENRDY {
            /// Reference voltage output is not ready to be used
            REFGENRDY_0 = 0b0,
            /// Reference voltage output is ready to be used
            REFGENRDY_1 = 0b1,
        }
        /// Buffered bandgap voltage ready status
        REFBGRDY: 13..13 = enum REFBGRDY {
            /// Buffered bandgap voltage is not ready to be used
            REFBGRDY_0 = 0b0,
            /// Buffered bandgap voltage is ready to be used
            REFBGRDY_1 = 0b1,
        }
    }
}
