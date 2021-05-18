//! CCS  Compact System Clock

utils::periph! {
    /// CCS  Compact System Clock
    CCS;
    /// CCS Control Register 0
    rw CCSCTL0 @ 0x00: u16 = 0_0 {
        /// CCS Control Register 0
        CCSCTL0: 0..15 = struct CCSCTL0Field(u16);
    }
    /// CCS Control Register 1
    rw CCSCTL1 @ 0x02: u16 = 0_0 {
        /// Clock division for CLKIN / X-OSC
        DIVCLK: 0 = struct DIVCLK(bool);
    }
    /// CCS Control Register 2
    rw CCSCTL2 @ 0x04: u16 = 0_0 {
        /// Frequency trimming of the HF-OSC Bit: 0
        FSEL0: 0 = struct FSEL0(bool);
        /// Frequency trimming of the HF-OSC Bit: 1
        FSEL1: 1 = struct FSEL1(bool);
        /// Frequency trimming of the HF-OSC Bit: 2
        FSEL2: 2 = struct FSEL2(bool);
        /// Frequency trimming of the HF-OSC Bit: 3
        FSEL3: 3 = struct FSEL3(bool);
        /// Frequency trimming of the HF-OSC Bit: 4
        FSEL4: 4 = struct FSEL4(bool);
        /// Frequency trimming of the HF-OSC Bit: 5
        FSEL5: 5 = struct FSEL5(bool);
        /// Frequency trimming of the HF-OSC Bit: 6
        FSEL6: 6 = struct FSEL6(bool);
    }
    /// CCS Control Register 4
    rw CCSCTL4 @ 0x08: u16 = 0_0 {
        /// MCLK Source Select Bit: 0
        SELM: 0..1 = enum SELM {
            /// MCLK Source Select 0
            SELM_0 = 0b00,
            /// MCLK Source Select 1
            SELM_1 = 0b01,
            /// MCLK Source Select 2
            SELM_2 = 0b10,
            /// MCLK Source Select 3
            SELM_3 = 0b11,
        }
        /// SMCLK Source Select Bit: 0
        SELS: 4..5 = enum SELS {
            /// SMCLK Source Select 0
            SELS_0 = 0b00,
            /// SMCLK Source Select 1
            SELS_1 = 0b01,
            /// SMCLK Source Select 2
            SELS_2 = 0b10,
            /// SMCLK Source Select 3
            SELS_3 = 0b11,
        }
        /// ACLK Source Select Bit: 0
        SELA: 8..9 = enum SELA {
            /// ACLK Source Select 0
            SELA_0 = 0b00,
            /// ACLK Source Select 1
            SELA_1 = 0b01,
            /// ACLK Source Select 2
            SELA_2 = 0b10,
            /// ACLK Source Select 3
            SELA_3 = 0b11,
        }
    }
    /// CCS Control Register 5
    rw CCSCTL5 @ 0x0a: u16 = 0_0 {
        /// MCLK Divider Bit: 0
        DIVM: 0..2 = enum DIVM {
            /// MCLK Source Divider 0
            DIVM_0 = 0b000,
            /// MCLK Source Divider 1
            DIVM_1 = 0b001,
            /// MCLK Source Divider 2
            DIVM_2 = 0b010,
            /// MCLK Source Divider 3
            DIVM_3 = 0b011,
            /// MCLK Source Divider 4
            DIVM_4 = 0b100,
            /// MCLK Source Divider 5
            DIVM_5 = 0b101,
        }
        /// SMCLK Divider Bit: 0
        DIVS: 4..6 = enum DIVS {
            /// SMCLK Source Divider 0
            DIVS_0 = 0b000,
            /// SMCLK Source Divider 1
            DIVS_1 = 0b001,
            /// SMCLK Source Divider 2
            DIVS_2 = 0b010,
            /// SMCLK Source Divider 3
            DIVS_3 = 0b011,
            /// SMCLK Source Divider 4
            DIVS_4 = 0b100,
            /// SMCLK Source Divider 5
            DIVS_5 = 0b101,
        }
        /// ACLK Divider Bit: 0
        DIVA: 8..10 = enum DIVA {
            /// ACLK Source Divider 0
            DIVA_0 = 0b000,
            /// ACLK Source Divider 1
            DIVA_1 = 0b001,
            /// ACLK Source Divider 2
            DIVA_2 = 0b010,
            /// ACLK Source Divider 3
            DIVA_3 = 0b011,
            /// ACLK Source Divider 4
            DIVA_4 = 0b100,
            /// ACLK Source Divider 5
            DIVA_5 = 0b101,
        }
    }
    /// CCS Control Register 6
    rw CCSCTL6 @ 0x0c: u16 = 0_0 {
        /// Disable XT oscillator
        XTOFF: 0 = struct XTOFF(bool);
    }
    /// CCS Control Register 7
    rw CCSCTL7 @ 0x0e: u16 = 0_0 {
        /// X-tal Oscillator Fault Flag
        XOFFG: 0 = struct XOFFG(bool);
        /// High Frequency Oscillator Fault Flag
        HFOFFG: 1 = struct HFOFFG(bool);
    }
    /// CCS Control Register 8
    rw CCSCTL8 @ 0x10: u16 = 0_0 {
        /// ACLK Clock Request Enable
        ACLKREQEN: 0 = struct ACLKREQEN(bool);
        /// MCLK Clock Request Enable
        MCLKREQEN: 1 = struct MCLKREQEN(bool);
        /// SMCLK Clock Request Enable
        SMCLKREQEN: 2 = struct SMCLKREQEN(bool);
    }
}
