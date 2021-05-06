//! FRAM

utils::periph! {
    /// FRAM
    FRAM;
    /// FRAM Controller Control 0
    rw FRCTL0 @ 0x00: u16 = 0_0 {
        /// FRAM Wait state Generator Precharge Time control Bit: 0
        NPRECHG: 0..2 = enum NPRECHG {
            /// FRAM Wait state Generator Precharge Time control: 0
            NPRECHG_0 = 0b000,
            /// FRAM Wait state Generator Precharge Time control: 1
            NPRECHG_1 = 0b001,
            /// FRAM Wait state Generator Precharge Time control: 2
            NPRECHG_2 = 0b010,
            /// FRAM Wait state Generator Precharge Time control: 3
            NPRECHG_3 = 0b011,
            /// FRAM Wait state Generator Precharge Time control: 4
            NPRECHG_4 = 0b100,
            /// FRAM Wait state Generator Precharge Time control: 5
            NPRECHG_5 = 0b101,
            /// FRAM Wait state Generator Precharge Time control: 6
            NPRECHG_6 = 0b110,
            /// FRAM Wait state Generator Precharge Time control: 7
            NPRECHG_7 = 0b111,
        }
        /// FRAM Disables the wait state generator
        NAUTO: 3 = struct NAUTO(bool);
        /// FRAM Wait state Generator Access Time control Bit: 0
        NACCESS: 4..6 = enum NACCESS {
            /// FRAM Wait state Generator Access Time control: 0
            NACCESS_0 = 0b000,
            /// FRAM Wait state Generator Access Time control: 1
            NACCESS_1 = 0b001,
            /// FRAM Wait state Generator Access Time control: 2
            NACCESS_2 = 0b010,
            /// FRAM Wait state Generator Access Time control: 3
            NACCESS_3 = 0b011,
            /// FRAM Wait state Generator Access Time control: 4
            NACCESS_4 = 0b100,
            /// FRAM Wait state Generator Access Time control: 5
            NACCESS_5 = 0b101,
            /// FRAM Wait state Generator Access Time control: 6
            NACCESS_6 = 0b110,
            /// FRAM Wait state Generator Access Time control: 7
            NACCESS_7 = 0b111,
        }
    }
    /// General Control 0
    rw GCCTL0 @ 0x04: u16 = 0_0 {
        /// FRAM is currently busy programming
        BUSY: 0 = struct BUSY(bool);
        /// Enable NMI event if Access time error occurs
        ACCTEIE: 3 = struct ACCTEIE(bool);
        /// Enable NMI event if Access Violation occurs
        ACCVIE: 4 = struct ACCVIE(bool);
        /// Enable NMI event if correctable bit error detected
        CBDIE: 5 = struct CBDIE(bool);
        /// Enable NMI event if uncorrectable bit error detected
        UBDIE: 6 = struct UBDIE(bool);
        /// Enable Power Up Clear on uncorrectable bit error
        UBDRSTEN: 7 = struct UBDRSTEN(bool);
    }
    /// General Control 1
    rw GCCTL1 @ 0x06: u16 = 0_0 {
        /// Access Violation Interrupt Flag
        ACCVIFG: 0 = struct ACCVIFG(bool);
        /// FRAM correctable bit error flag
        CBDIFG: 1 = struct CBDIFG(bool);
        /// FRAM uncorrectable bit error flag
        UBDIFG: 2 = struct UBDIFG(bool);
        /// Access time error flag
        ACCTEIFG: 3 = struct ACCTEIFG(bool);
    }
}
