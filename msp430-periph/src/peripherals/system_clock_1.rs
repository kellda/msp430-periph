//! System Clock

utils::periph! {
    /// System Clock
    SystemClock;
    /// DCO Clock Frequency Control
    rw DCOCTL @ 0x00: u8 = 0_0 {
        /// Modulation Bit 0
        MOD0: 0 = struct MOD0(bool);
        /// Modulation Bit 1
        MOD1: 1 = struct MOD1(bool);
        /// Modulation Bit 2
        MOD2: 2 = struct MOD2(bool);
        /// Modulation Bit 3
        MOD3: 3 = struct MOD3(bool);
        /// Modulation Bit 4
        MOD4: 4 = struct MOD4(bool);
        /// DCO Select Bit 0
        DCO0: 5 = struct DCO0(bool);
        /// DCO Select Bit 1
        DCO1: 6 = struct DCO1(bool);
        /// DCO Select Bit 2
        DCO2: 7 = struct DCO2(bool);
    }
    /// Basic Clock System Control 1
    rw BCSCTL1 @ 0x01: u8 = 0_0 {
        /// Range Select Bit 0
        RSEL0: 0 = struct RSEL0(bool);
        /// Range Select Bit 1
        RSEL1: 1 = struct RSEL1(bool);
        /// Range Select Bit 2
        RSEL2: 2 = struct RSEL2(bool);
        /// XT5V should always be reset
        XT5V: 3 = struct XT5V(bool);
        /// ACLK Divider 0
        DIVA: 4..5 = enum DIVA {
            /// ACLK Divider 0: /1
            DIVA_0 = 0b00,
            /// ACLK Divider 1: /2
            DIVA_1 = 0b01,
            /// ACLK Divider 2: /4
            DIVA_2 = 0b10,
            /// ACLK Divider 3: /8
            DIVA_3 = 0b11,
        }
        /// LFXTCLK 0:Low Freq. / 1: High Freq.
        XTS: 6 = struct XTS(bool);
        /// Enable XT2CLK
        XT2OFF: 7 = struct XT2OFF(bool);
    }
    /// Basic Clock System Control 2
    rw BCSCTL2 @ 0x02: u8 = 0_0 {
        /// Enable External Resistor : 1
        DCOR: 0 = struct DCOR(bool);
        /// SMCLK Divider 0
        DIVS: 1..2 = enum DIVS {
            /// SMCLK Divider 0: /1
            DIVS_0 = 0b00,
            /// SMCLK Divider 1: /2
            DIVS_1 = 0b01,
            /// SMCLK Divider 2: /4
            DIVS_2 = 0b10,
            /// SMCLK Divider 3: /8
            DIVS_3 = 0b11,
        }
        /// SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK
        SELS: 3 = struct SELS(bool);
        /// MCLK Divider 0
        DIVM: 4..5 = enum DIVM {
            /// MCLK Divider 0: /1
            DIVM_0 = 0b00,
            /// MCLK Divider 1: /2
            DIVM_1 = 0b01,
            /// MCLK Divider 2: /4
            DIVM_2 = 0b10,
            /// MCLK Divider 3: /8
            DIVM_3 = 0b11,
        }
        /// MCLK Source Select 0
        SELM: 6..7 = enum SELM {
            /// MCLK Source Select 0: DCOCLK
            SELM_0 = 0b00,
            /// MCLK Source Select 1: DCOCLK
            SELM_1 = 0b01,
            /// MCLK Source Select 2: XT2CLK/LFXTCLK
            SELM_2 = 0b10,
            /// MCLK Source Select 3: LFXTCLK
            SELM_3 = 0b11,
        }
    }
}
