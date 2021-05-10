//! PMM

utils::periph! {
    /// PMM
    PMM;
    /// PMM control register 0
    rw PMMCTL0 @ 0x00: u16 = 0_0 {
        /// Software brownout reset.
        PMMSWBOR: 2 = enum PMMSWBOR {
            /// Normal operation
            PMMSWBOR_0 = 0b0,
            /// Set to 1 to trigger a BOR
            PMMSWBOR_1 = 0b1,
        }
        /// Software POR.
        PMMSWPOR: 3 = enum PMMSWPOR {
            /// Normal operation
            PMMSWPOR_0 = 0b0,
            /// Set to 1 to trigger a POR
            PMMSWPOR_1 = 0b1,
        }
        /// Regulator off
        PMMREGOFF: 4 = enum PMMREGOFF {
            /// Regulator remains on when going into LPM3 or LPM4
            PMMREGOFF_0 = 0b0,
            /// Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively.
            PMMREGOFF_1 = 0b1,
        }
        /// High-side SVS enable.
        SVSHE: 6 = enum SVSHE {
            /// High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1.
            SVSHE_0 = 0b0,
            /// SVSH is always enabled.
            SVSHE_1 = 0b1,
        }
        /// PMM password.
        PMMPW: 8..15 = struct PMMPW(u16);
    }
    /// PMM interrupt flag register
    rw PMMIFG @ 0x0a: u16 = 0_0 {
        /// PMM software brownout reset interrupt flag.
        PMMBORIFG: 8 = enum PMMBORIFG {
            /// Reset not due to PMMSWBOR
            PMMBORIFG_0 = 0b0,
            /// Reset due to PMMSWBOR
            PMMBORIFG_1 = 0b1,
        }
        /// PMM reset pin interrupt flag.
        PMMRSTIFG: 9 = enum PMMRSTIFG {
            /// Reset not due to reset pin
            PMMBORIFG_0 = 0b0,
            /// Reset due to reset pin
            PMMBORIFG_1 = 0b1,
        }
        /// PMM software POR interrupt flag.
        PMMPORIFG: 10 = enum PMMPORIFG {
            /// Reset not due to PMMSWPOR
            PMMBORIFG_0 = 0b0,
            /// Reset due to PMMSWPOR
            PMMBORIFG_1 = 0b1,
        }
        /// High-side SVS interrupt flag.
        SVSHIFG: 13 = enum SVSHIFG {
            /// Reset not due to SVSH
            SVSHIFG_0 = 0b0,
            /// Reset due to SVSH
            SVSHIFG_1 = 0b1,
        }
        /// LPMx.5 flag.
        PMMLPM5IFG: 15 = enum PMMLPM5IFG {
            /// Reset not due to wake-up from LPMx.5
            PMMLPM5IFG_0 = 0b0,
            /// Reset due to wake-up from LPMx.5
            PMMLPM5IFG_1 = 0b1,
        }
    }
    /// Power mode 5 control register 0
    rw PM5CTL0 @ 0x10: u16 = 0_0 {
        /// LPMx.5 Lock Bit
        LOCKLPM5: 0 = enum LOCKLPM5 {
            /// LPMx.5 configuration is not locked and defaults to its reset condition.
            LOCKLPM5_0 = 0b0,
            /// LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit.
            LOCKLPM5_1 = 0b1,
        }
    }
}
