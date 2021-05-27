//! UCS  Unified System Clock

utils::periph! {
    /// UCS  Unified System Clock
    UCS;
    /// UCS Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// Modulation Bit Counter Bit : 0
        MOD0: 3 = struct MOD0(bool);
        /// Modulation Bit Counter Bit : 1
        MOD1: 4 = struct MOD1(bool);
        /// Modulation Bit Counter Bit : 2
        MOD2: 5 = struct MOD2(bool);
        /// Modulation Bit Counter Bit : 3
        MOD3: 6 = struct MOD3(bool);
        /// Modulation Bit Counter Bit : 4
        MOD4: 7 = struct MOD4(bool);
        /// DCO TAP Bit : 0
        DCO0: 8 = struct DCO0(bool);
        /// DCO TAP Bit : 1
        DCO1: 9 = struct DCO1(bool);
        /// DCO TAP Bit : 2
        DCO2: 10 = struct DCO2(bool);
        /// DCO TAP Bit : 3
        DCO3: 11 = struct DCO3(bool);
        /// DCO TAP Bit : 4
        DCO4: 12 = struct DCO4(bool);
    }
    /// UCS Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// Disable Modulation
        DISMOD: 0 = struct DISMOD(bool);
        /// DCO Freq. Range Select Bit : 0
        DCORSEL: 4..6 = enum DCORSEL {
            /// DCO RSEL 0
            DCORSEL_0 = 0b000,
            /// DCO RSEL 1
            DCORSEL_1 = 0b001,
            /// DCO RSEL 2
            DCORSEL_2 = 0b010,
            /// DCO RSEL 3
            DCORSEL_3 = 0b011,
            /// DCO RSEL 4
            DCORSEL_4 = 0b100,
            /// DCO RSEL 5
            DCORSEL_5 = 0b101,
            /// DCO RSEL 6
            DCORSEL_6 = 0b110,
            /// DCO RSEL 7
            DCORSEL_7 = 0b111,
        }
    }
    /// UCS Control Register 2
    rw CTL2 @ 0x04: u16 = 0_0 {
        /// FLL Multipier Bit : 0
        FLLN0: 0 = struct FLLN0(bool);
        /// FLL Multipier Bit : 1
        FLLN1: 1 = struct FLLN1(bool);
        /// FLL Multipier Bit : 2
        FLLN2: 2 = struct FLLN2(bool);
        /// FLL Multipier Bit : 3
        FLLN3: 3 = struct FLLN3(bool);
        /// FLL Multipier Bit : 4
        FLLN4: 4 = struct FLLN4(bool);
        /// FLL Multipier Bit : 5
        FLLN5: 5 = struct FLLN5(bool);
        /// FLL Multipier Bit : 6
        FLLN6: 6 = struct FLLN6(bool);
        /// FLL Multipier Bit : 7
        FLLN7: 7 = struct FLLN7(bool);
        /// FLL Multipier Bit : 8
        FLLN8: 8 = struct FLLN8(bool);
        /// FLL Multipier Bit : 9
        FLLN9: 9 = struct FLLN9(bool);
        /// Loop Divider Bit : 0
        FLLD: 12..14 = enum FLLD {
            /// Multiply Selected Loop Freq. 1
            FLLD_0 = 0b000,
            /// Multiply Selected Loop Freq. 2
            FLLD_1 = 0b001,
            /// Multiply Selected Loop Freq. 4
            FLLD_2 = 0b010,
            /// Multiply Selected Loop Freq. 8
            FLLD_3 = 0b011,
            /// Multiply Selected Loop Freq. 16
            FLLD_4 = 0b100,
            /// Multiply Selected Loop Freq. 32
            FLLD_5 = 0b101,
            /// Multiply Selected Loop Freq. 32
            FLLD_6 = 0b110,
            /// Multiply Selected Loop Freq. 32
            FLLD_7 = 0b111,
        }
    }
    /// UCS Control Register 3
    rw CTL3 @ 0x06: u16 = 0_0 {
        /// Reference Divider Bit : 0
        FLLREFDIV: 0..2 = enum FLLREFDIV {
            /// Reference Divider: f(LFCLK)/1
            FLLREFDIV_0 = 0b000,
            /// Reference Divider: f(LFCLK)/2
            FLLREFDIV_1 = 0b001,
            /// Reference Divider: f(LFCLK)/4
            FLLREFDIV_2 = 0b010,
            /// Reference Divider: f(LFCLK)/8
            FLLREFDIV_3 = 0b011,
            /// Reference Divider: f(LFCLK)/12
            FLLREFDIV_4 = 0b100,
            /// Reference Divider: f(LFCLK)/16
            FLLREFDIV_5 = 0b101,
            /// Reference Divider: f(LFCLK)/16
            FLLREFDIV_6 = 0b110,
            /// Reference Divider: f(LFCLK)/16
            FLLREFDIV_7 = 0b111,
        }
        /// FLL Reference Clock Select Bit : 0
        SELREF: 4..6 = enum SELREF {
            /// FLL Reference Clock Select 0
            SELREF_0 = 0b000,
            /// FLL Reference Clock Select 1
            SELREF_1 = 0b001,
            /// FLL Reference Clock Select 2
            SELREF_2 = 0b010,
            /// FLL Reference Clock Select 3
            SELREF_3 = 0b011,
            /// FLL Reference Clock Select 4
            SELREF_4 = 0b100,
            /// FLL Reference Clock Select 5
            SELREF_5 = 0b101,
            /// FLL Reference Clock Select 6
            SELREF_6 = 0b110,
            /// FLL Reference Clock Select 7
            SELREF_7 = 0b111,
        }
    }
    /// UCS Control Register 4
    rw CTL4 @ 0x08: u16 = 0_0 {
        /// MCLK Source Select Bit: 0
        SELM: 0..2 = enum SELM {
            /// MCLK Source Select 0
            SELM_0 = 0b000,
            /// MCLK Source Select 1
            SELM_1 = 0b001,
            /// MCLK Source Select 2
            SELM_2 = 0b010,
            /// MCLK Source Select 3
            SELM_3 = 0b011,
            /// MCLK Source Select 4
            SELM_4 = 0b100,
            /// MCLK Source Select 5
            SELM_5 = 0b101,
            /// MCLK Source Select 6
            SELM_6 = 0b110,
            /// MCLK Source Select 7
            SELM_7 = 0b111,
        }
        /// SMCLK Source Select Bit: 0
        SELS: 4..6 = enum SELS {
            /// SMCLK Source Select 0
            SELS_0 = 0b000,
            /// SMCLK Source Select 1
            SELS_1 = 0b001,
            /// SMCLK Source Select 2
            SELS_2 = 0b010,
            /// SMCLK Source Select 3
            SELS_3 = 0b011,
            /// SMCLK Source Select 4
            SELS_4 = 0b100,
            /// SMCLK Source Select 5
            SELS_5 = 0b101,
            /// SMCLK Source Select 6
            SELS_6 = 0b110,
            /// SMCLK Source Select 7
            SELS_7 = 0b111,
        }
        /// ACLK Source Select Bit: 0
        SELA: 8..10 = enum SELA {
            /// ACLK Source Select 0
            SELA_0 = 0b000,
            /// ACLK Source Select 1
            SELA_1 = 0b001,
            /// ACLK Source Select 2
            SELA_2 = 0b010,
            /// ACLK Source Select 3
            SELA_3 = 0b011,
            /// ACLK Source Select 4
            SELA_4 = 0b100,
            /// ACLK Source Select 5
            SELA_5 = 0b101,
            /// ACLK Source Select 6
            SELA_6 = 0b110,
            /// ACLK Source Select 7
            SELA_7 = 0b111,
        }
    }
    /// UCS Control Register 5
    rw CTL5 @ 0x0a: u16 = 0_0 {
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
            /// MCLK Source Divider 6
            DIVM_6 = 0b110,
            /// MCLK Source Divider 7
            DIVM_7 = 0b111,
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
            /// SMCLK Source Divider 6
            DIVS_6 = 0b110,
            /// SMCLK Source Divider 7
            DIVS_7 = 0b111,
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
            /// ACLK Source Divider 6
            DIVA_6 = 0b110,
            /// ACLK Source Divider 7
            DIVA_7 = 0b111,
        }
        /// ACLK from Pin Divider Bit: 0
        DIVPA: 12..14 = enum DIVPA {
            /// ACLK from Pin Source Divider 0
            DIVPA_0 = 0b000,
            /// ACLK from Pin Source Divider 1
            DIVPA_1 = 0b001,
            /// ACLK from Pin Source Divider 2
            DIVPA_2 = 0b010,
            /// ACLK from Pin Source Divider 3
            DIVPA_3 = 0b011,
            /// ACLK from Pin Source Divider 4
            DIVPA_4 = 0b100,
            /// ACLK from Pin Source Divider 5
            DIVPA_5 = 0b101,
            /// ACLK from Pin Source Divider 6
            DIVPA_6 = 0b110,
            /// ACLK from Pin Source Divider 7
            DIVPA_7 = 0b111,
        }
    }
    /// UCS Control Register 6
    rw CTL6 @ 0x0c: u16 = 0_0 {
        /// High Frequency Oscillator 1 (XT1) disable
        XT1OFF: 0 = struct XT1OFF(bool);
        /// SMCLK Off
        SMCLKOFF: 1 = struct SMCLKOFF(bool);
        /// XIN/XOUT Cap Bit: 0
        XCAP: 2..3 = enum XCAP {
            /// XIN/XOUT Cap 0
            XCAP_0 = 0b00,
            /// XIN/XOUT Cap 1
            XCAP_1 = 0b01,
            /// XIN/XOUT Cap 2
            XCAP_2 = 0b10,
            /// XIN/XOUT Cap 3
            XCAP_3 = 0b11,
        }
        /// XT1 bypass mode : 0: internal 1:sourced from external pin
        XT1BYPASS: 4 = struct XT1BYPASS(bool);
        /// 1: Selects high-freq. oscillator
        XTS: 5 = struct XTS(bool);
        /// XT1 Drive Level mode Bit 0
        XT1DRIVE: 6..7 = enum XT1DRIVE {
            /// XT1 Drive Level mode: 0
            XT1DRIVE_0 = 0b00,
            /// XT1 Drive Level mode: 1
            XT1DRIVE_1 = 0b01,
            /// XT1 Drive Level mode: 2
            XT1DRIVE_2 = 0b10,
            /// XT1 Drive Level mode: 3
            XT1DRIVE_3 = 0b11,
        }
        /// High Frequency Oscillator 2 (XT2) disable
        XT2OFF: 8 = struct XT2OFF(bool);
    }
    /// UCS Control Register 7
    rw CTL7 @ 0x0e: u16 = 0_0 {
        /// DCO Fault Flag
        DCOFFG: 0 = struct DCOFFG(bool);
        /// XT1 Low Frequency Oscillator Fault Flag
        XT1LFOFFG: 1 = struct XT1LFOFFG(bool);
        /// XT1 High Frequency Oscillator 1 Fault Flag
        XT1HFOFFG: 2 = struct XT1HFOFFG(bool);
        /// High Frequency Oscillator 2 Fault Flag
        XT2OFFG: 3 = struct XT2OFFG(bool);
    }
    /// UCS Control Register 8
    rw CTL8 @ 0x10: u16 = 0_0 {
        /// ACLK Clock Request Enable
        ACLKREQEN: 0 = struct ACLKREQEN(bool);
        /// MCLK Clock Request Enable
        MCLKREQEN: 1 = struct MCLKREQEN(bool);
        /// SMCLK Clock Request Enable
        SMCLKREQEN: 2 = struct SMCLKREQEN(bool);
        /// MODOSC Clock Request Enable
        MODOSCREQEN: 3 = struct MODOSCREQEN(bool);
    }
}
