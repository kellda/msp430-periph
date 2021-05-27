//! PMM

utils::periph! {
    /// PMM
    PMM;
    /// PMM control register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// Software brownout reset.
        SWBOR: 2 = enum SWBOR {
            /// Normal operation
            SWBOR_0 = 0b0,
            /// Set to 1 to trigger a BOR
            SWBOR_1 = 0b1,
        }
        /// Software POR.
        SWPOR: 3 = enum SWPOR {
            /// Normal operation
            SWPOR_0 = 0b0,
            /// Set to 1 to trigger a POR
            SWPOR_1 = 0b1,
        }
        /// Regulator off
        REGOFF: 4 = enum REGOFF {
            /// Regulator remains on when going into LPM3 or LPM4
            REGOFF_0 = 0b0,
            /// Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively.
            REGOFF_1 = 0b1,
        }
        /// High-side SVS enable.
        SVSHE: 6 = enum SVSHE {
            /// High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1.
            SVSHE_0 = 0b0,
            /// SVSH is always enabled.
            SVSHE_1 = 0b1,
        }
        /// PMM password.
        PW: 8..15 = struct PW(u16);
    }
    /// PMM interrupt flag register
    rw IFG @ 0x0a: u16 = 0_0 {
        /// PMM software brownout reset interrupt flag.
        BORIFG: 8 = enum BORIFG {
            /// Reset not due to PMMSWBOR
            BORIFG_0 = 0b0,
            /// Reset due to PMMSWBOR
            BORIFG_1 = 0b1,
        }
        /// PMM reset pin interrupt flag.
        RSTIFG: 9 = enum RSTIFG {
            /// Reset not due to reset pin
            BORIFG_0 = 0b0,
            /// Reset due to reset pin
            BORIFG_1 = 0b1,
        }
        /// PMM software POR interrupt flag.
        PORIFG: 10 = enum PORIFG {
            /// Reset not due to PMMSWPOR
            BORIFG_0 = 0b0,
            /// Reset due to PMMSWPOR
            BORIFG_1 = 0b1,
        }
        /// High-side SVS interrupt flag.
        SVSHIFG: 13 = enum SVSHIFG {
            /// Reset not due to SVSH
            SVSHIFG_0 = 0b0,
            /// Reset due to SVSH
            SVSHIFG_1 = 0b1,
        }
        /// LPMx.5 flag.
        LPM5IFG: 15 = enum LPM5IFG {
            /// Reset not due to wake-up from LPMx.5
            LPM5IFG_0 = 0b0,
            /// Reset due to wake-up from LPMx.5
            LPM5IFG_1 = 0b1,
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
