//! REF_A

utils::periph! {
    /// REF_A
    REF_A;
    /// REF Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// Reference enable
        ON: 0 = enum ON {
            /// Disables reference if no other reference requests are pending
            ON_0 = 0b0,
            /// Enables reference in static mode
            ON_1 = 0b1,
        }
        /// Reference output buffer
        OUT: 1 = enum OUT {
            /// Reference output not available externally
            OUT_0 = 0b0,
            /// Reference output available externally. If ADC14REFBURST = 0, output is available continuously. If ADC14REFBURST = 1, output is available only during an ADC14 conversion.
            OUT_1 = 0b1,
        }
        /// Temperature sensor disabled
        TCOFF: 3 = enum TCOFF {
            /// Temperature sensor enabled
            TCOFF_0 = 0b0,
            /// Temperature sensor disabled to save power
            TCOFF_1 = 0b1,
        }
        /// Reference voltage level select
        VSEL: 4..5 = enum VSEL {
            /// 1.2 V available when reference requested or REFON = 1
            VSEL_0 = 0b00,
            /// 2.0 V available when reference requested or REFON = 1
            VSEL_1 = 0b01,
            /// 2.5 V available when reference requested or REFON = 1
            VSEL_2 = 0b10,
            /// 2.5 V available when reference requested or REFON = 1
            VSEL_3 = 0b11,
        }
        /// Reference generator one-time trigger
        GENOT: 6 = enum GENOT {
            /// No trigger
            GENOT_0 = 0b0,
            /// Generation of the reference voltage is started by writing 1 or by a hardware trigger
            GENOT_1 = 0b1,
        }
        /// Bandgap and bandgap buffer one-time trigger
        BGOT: 7 = enum BGOT {
            /// No trigger
            BGOT_0 = 0b0,
            /// Generation of the bandgap voltage is started by writing 1 or by a hardware trigger
            BGOT_1 = 0b1,
        }
        /// Reference generator active
        GENACT: 8 = enum GENACT {
            /// Reference generator not active
            GENACT_0 = 0b0,
            /// Reference generator active
            GENACT_1 = 0b1,
        }
        /// Reference bandgap active
        BGACT: 9 = enum BGACT {
            /// Reference bandgap buffer not active
            BGACT_0 = 0b0,
            /// Reference bandgap buffer active
            BGACT_1 = 0b1,
        }
        /// Reference generator busy
        GENBUSY: 10 = enum GENBUSY {
            /// Reference generator not busy
            GENBUSY_0 = 0b0,
            /// Reference generator busy
            GENBUSY_1 = 0b1,
        }
        /// Bandgap mode
        BGMODE: 11 = enum BGMODE {
            /// Static mode
            BGMODE_0 = 0b0,
            /// Sampled mode
            BGMODE_1 = 0b1,
        }
        /// Variable reference voltage ready status
        GENRDY: 12 = enum GENRDY {
            /// Reference voltage output is not ready to be used
            GENRDY_0 = 0b0,
            /// Reference voltage output is ready to be used
            GENRDY_1 = 0b1,
        }
        /// Buffered bandgap voltage ready status
        BGRDY: 13 = enum BGRDY {
            /// Buffered bandgap voltage is not ready to be used
            BGRDY_0 = 0b0,
            /// Buffered bandgap voltage is ready to be used
            BGRDY_1 = 0b1,
        }
    }
}
