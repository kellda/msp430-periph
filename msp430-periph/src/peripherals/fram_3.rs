//! FRAM

utils::periph! {
    /// FRAM
    FRAM;
    /// FRAM Controller Control 0
    rw FRCTL0 @ 0x00: u16 = 0_0 {
        /// FRAM Wait state control Bit: 0
        NWAITS: 4..6 = enum NWAITS {
            /// FRAM Wait state control: 0
            NWAITS_0 = 0b000,
            /// FRAM Wait state control: 1
            NWAITS_1 = 0b001,
            /// FRAM Wait state control: 2
            NWAITS_2 = 0b010,
            /// FRAM Wait state control: 3
            NWAITS_3 = 0b011,
            /// FRAM Wait state control: 4
            NWAITS_4 = 0b100,
            /// FRAM Wait state control: 5
            NWAITS_5 = 0b101,
            /// FRAM Wait state control: 6
            NWAITS_6 = 0b110,
            /// FRAM Wait state control: 7
            NWAITS_7 = 0b111,
        }
    }
    /// General Control 0
    rw GCCTL0 @ 0x04: u16 = 0_0 {
        /// FRAM Enable FRAM auto power up after LPM
        FRLPMPWR: 1 = struct FRLPMPWR(bool);
        /// FRAM Power Control
        FRPWR: 2 = struct FRPWR(bool);
        /// RESERVED
        ACCTEIE: 3 = struct ACCTEIE(bool);
        /// Enable NMI event if correctable bit error detected
        CBDIE: 5 = struct CBDIE(bool);
        /// Enable NMI event if uncorrectable bit error detected
        UBDIE: 6 = struct UBDIE(bool);
        /// Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected
        UBDRSTEN: 7 = struct UBDRSTEN(bool);
    }
    /// General Control 1
    rw GCCTL1 @ 0x06: u16 = 0_0 {
        /// FRAM correctable bit error flag
        CBDIFG: 1 = struct CBDIFG(bool);
        /// FRAM uncorrectable bit error flag
        UBDIFG: 2 = struct UBDIFG(bool);
        /// Access time error flag
        ACCTEIFG: 3 = struct ACCTEIFG(bool);
    }
}
