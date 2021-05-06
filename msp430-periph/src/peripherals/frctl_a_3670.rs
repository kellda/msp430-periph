//! FRCTL_A

utils::periph! {
    /// FRCTL_A
    FRCTL_A;
    /// FRAM Controller A Control Register 0
    rw FRCTL0 @ 0x00: u16 = 0_0 {
        /// Wait state numbers
        NWAITS: 4..7 = enum NWAITS {
            /// FRAM wait states: 0
            NWAITS_0 = 0b0000,
            /// FRAM wait states: 1
            NWAITS_1 = 0b0001,
            /// FRAM wait states: 2
            NWAITS_2 = 0b0010,
            /// FRAM wait states: 3
            NWAITS_3 = 0b0011,
            /// FRAM wait states: 4
            NWAITS_4 = 0b0100,
            /// FRAM wait states: 5
            NWAITS_5 = 0b0101,
            /// FRAM wait states: 6
            NWAITS_6 = 0b0110,
            /// FRAM wait states: 7
            NWAITS_7 = 0b0111,
            /// FRAM wait states: 8
            NWAITS_8 = 0b1000,
            /// FRAM wait states: 9
            NWAITS_9 = 0b1001,
            /// FRAM wait states: 10
            NWAITS_10 = 0b1010,
            /// FRAM wait states: 11
            NWAITS_11 = 0b1011,
            /// FRAM wait states: 12
            NWAITS_12 = 0b1100,
            /// FRAM wait states: 13
            NWAITS_13 = 0b1101,
            /// FRAM wait states: 14
            NWAITS_14 = 0b1110,
            /// FRAM wait states: 15
            NWAITS_15 = 0b1111,
        }
        /// FRCTLPW password
        FRCTLPW: 8..15 = struct FRCTLPW(u16);
        /// Enable automatic Wait State Mode
        AUTO: 3..3 = enum AUTO {
            /// User Wait State Mode. The NWAITS[3:0] is used for the FRAM wait state.
            AUTO_0 = 0b0,
            /// Auto mode. The NWAITS[3:0] is ignored. Wait states are generated automatically by the internal FRAM controller state machine.
            AUTO_1 = 0b1,
        }
        /// Write Protection Enable
        WPROT: 0..0 = enum WPROT {
            /// Disable Write Protection. Write to FRAM memory is allowed.
            WPROT_0 = 0b0,
            /// Enable Write Protection. Write to FRAM memory is not allowed. In case a write access is attempted, the WPIFG (Write Protection Flag) bit will be set.
            WPROT_1 = 0b1,
        }
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
        /// Enable NMI event for the Write Protection Detection flag (WPIFG)
        WPIE: 4..4 = enum WPIE {
            /// Disable NMI for the Write Protection Detection flag (WPIFG).
            WPIE_0 = 0b0,
            /// Enable NMI for the Write Protection Detection flag (WPIFG). Generates vector in SYSSNIV.
            WPIE_1 = 0b1,
        }
        /// Enable NMI event for the Access time error flag (ACCTEIFG)
        ACCTEIE: 3..3 = enum ACCTEIE {
            /// Disable NMI for the Access time error flag (ACCTEIFG).
            ACCTEIE_0 = 0b0,
            /// Enable NMI for the Access time error flag (ACCTEIFG). Generates vector in SYSSNIV.
            ACCTEIE_1 = 0b1,
        }
        /// FRAM Memory Power Control Request
        FRPWR: 2..2 = enum FRPWR {
            /// Enable INACTIVE mode.
            FRPWR_0 = 0b0,
            /// Enable ACTIVE mode.
            FRPWR_1 = 0b1,
        }
    }
    /// General Control Register 1
    rw GCCTL1 @ 0x06: u16 = 0_0 {
        /// Write Protection Detection flag
        WPIFG: 4..4 = enum WPIFG {
            /// No interrupt pending.
            WPIFG_0 = 0b0,
            /// Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt.
            WPIFG_1 = 0b1,
        }
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
