//! System Clock

utils::periph! {
    /// System Clock
    SystemClock;
    /// DCO Clock Frequency Control
    rw DCOCTL @ 0x03: u8 = 0_0 {
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
    rw BCSCTL1 @ 0x04: u8 = 0_0 {
        /// Range Select Bit 0
        RSEL0: 0 = struct RSEL0(bool);
        /// Range Select Bit 1
        RSEL1: 1 = struct RSEL1(bool);
        /// Range Select Bit 2
        RSEL2: 2 = struct RSEL2(bool);
        /// Range Select Bit 3
        RSEL3: 3 = struct RSEL3(bool);
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
    rw BCSCTL2 @ 0x05: u8 = 0_0 {
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
    /// Basic Clock System Control 3
    rw BCSCTL3 @ 0x00: u8 = 0_0 {
        /// Low/high Frequency Oscillator Fault Flag
        LFXT1OF: 0 = struct LFXT1OF(bool);
        /// High frequency oscillator 2 fault flag
        XT2OF: 1 = struct XT2OF(bool);
        /// XIN/XOUT Cap 0
        XCAP: 2..3 = enum XCAP {
            /// XIN/XOUT Cap : 0 pF
            XCAP_0 = 0b00,
            /// XIN/XOUT Cap : 6 pF
            XCAP_1 = 0b01,
            /// XIN/XOUT Cap : 10 pF
            XCAP_2 = 0b10,
            /// XIN/XOUT Cap : 12.5 pF
            XCAP_3 = 0b11,
        }
        /// Mode 0 for LFXT1 (XTS = 0)
        LFXT1S: 4..5 = enum LFXT1S {
            /// Mode 0 for LFXT1 : Normal operation
            LFXT1S_0 = 0b00,
            /// Mode 1 for LFXT1 : Reserved
            LFXT1S_1 = 0b01,
            /// Mode 2 for LFXT1 : VLO
            LFXT1S_2 = 0b10,
            /// Mode 3 for LFXT1 : Digital input signal
            LFXT1S_3 = 0b11,
        }
        /// Mode 0 for XT2
        XT2S: 6..7 = enum XT2S {
            /// Mode 0 for XT2 : 0.4 - 1 MHz
            XT2S_0 = 0b00,
            /// Mode 1 for XT2 : 1 - 4 MHz
            XT2S_1 = 0b01,
            /// Mode 2 for XT2 : 2 - 16 MHz
            XT2S_2 = 0b10,
            /// Mode 3 for XT2 : Digital input signal
            XT2S_3 = 0b11,
        }
    }
}
