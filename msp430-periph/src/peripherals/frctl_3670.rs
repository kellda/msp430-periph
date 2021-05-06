//! FRCTL

utils::periph! {
    /// FRCTL
    FRCTL;
    /// FRAM Controller Control Register 0
    rw FRCTL0 @ 0x00: u16 = 0_0 {
        /// Wait state numbers
        NWAITS: 4..6 = enum NWAITS {
            /// FRAM wait states: 0
            NWAITS_0 = 0b000,
            /// FRAM wait states: 1
            NWAITS_1 = 0b001,
            /// FRAM wait states: 2
            NWAITS_2 = 0b010,
            /// FRAM wait states: 3
            NWAITS_3 = 0b011,
            /// FRAM wait states: 4
            NWAITS_4 = 0b100,
            /// FRAM wait states: 5
            NWAITS_5 = 0b101,
            /// FRAM wait states: 6
            NWAITS_6 = 0b110,
            /// FRAM wait states: 7
            NWAITS_7 = 0b111,
        }
        /// FRCTLPW password
        FRCTLPW: 8..15 = struct FRCTLPW(u16);
    }
    /// General Control Register 0
    rw GCCTL0 @ 0x04: u16 = 0_0 {
        /// Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)
        UBDRSTEN: 7..7 = enum UBDRSTEN {
            /// PUC not initiated on uncorrectable bit error detection flag.
            UBDRSTEN_0 = 0b0,
            /// PUC initiated on uncorrectable bit error detection flag. Generates vector in SYSRSTIV. Clear the UBDIE bit.
            UBDRSTEN_1 = 0b1,
        }
        /// Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)
        UBDIE: 6..6 = enum UBDIE {
            /// Disable NMI for the uncorrectable bit error detection flag (UBDIFG).
            UBDIE_0 = 0b0,
            /// Enable NMI for the uncorrectable bit error detection flag (UBDIFG). Generates vector in SYSSNIV. Clear the UBDRSTEN bit.
            UBDIE_1 = 0b1,
        }
        /// Enable NMI event for the correctable bit error detection flag (CBDIFG)
        CBDIE: 5..5 = enum CBDIE {
            /// Disable NMI for the correctable bit error detection flag (CBDIFG).
            CBDIE_0 = 0b0,
            /// Disable NMI for the correctable bit error detection flag (CBDIFG). Generates vector in SYSSNIV.
            CBDIE_1 = 0b1,
        }
        /// FRAM Memory Power Control Request
        FRPWR: 2..2 = enum FRPWR {
            /// Enable INACTIVE mode.
            FRPWR_0 = 0b0,
            /// Enable ACTIVE mode.
            FRPWR_1 = 0b1,
        }
        /// Enables FRAM auto power up after LPM
        FRLPMPWR: 1..1 = enum FRLPMPWR {
            /// FRAM startup is delayed to the first FRAM access after exit from LPM
            FRLPMPWR_0 = 0b0,
            /// FRAM is powered up immediately on exit from LPM
            FRLPMPWR_1 = 0b1,
        }
    }
    /// General Control Register 1
    rw GCCTL1 @ 0x06: u16 = 0_0 {
        /// Access time error flag
        ACCTEIFG: 3..3 = enum ACCTEIFG {
            /// No interrupt pending.
            ACCTEIFG_0 = 0b0,
            /// Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt.
            ACCTEIFG_1 = 0b1,
        }
        /// FRAM uncorrectable bit error detection flag
        UBDIFG: 2..2 = enum UBDIFG {
            /// No interrupt pending.
            UBDIFG_0 = 0b0,
            /// Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt.
            UBDIFG_1 = 0b1,
        }
        /// FRAM correctable bit error detection flag
        CBDIFG: 1..1 = enum CBDIFG {
            /// No interrupt is pending
            CBDIFG_0 = 0b0,
            /// Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV if it is the highest pending interrupt.
            CBDIFG_1 = 0b1,
        }
    }
}
