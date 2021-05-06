//! CS  Clock System

utils::periph! {
    /// CS  Clock System
    CSClockSystem;
    /// CS Control Register 0
    rw CSCTL0 @ 0x00: u16 = 0_0 {
        /// CS Control Register 0
        CSCTL0: 0..15 = struct CSCTL0Field(u16);
    }
    /// CS Control Register 1
    rw CSCTL1 @ 0x02: u16 = 0_0 {
        /// DCO frequency select Bit: 0
        DCOFSEL: 1..3 = enum DCOFSEL {
            /// DCO frequency select: 0
            DCOFSEL_0 = 0b000,
            /// DCO frequency select: 1
            DCOFSEL_1 = 0b001,
            /// DCO frequency select: 2
            DCOFSEL_2 = 0b010,
            /// DCO frequency select: 3
            DCOFSEL_3 = 0b011,
            /// DCO frequency select: 4
            DCOFSEL_4 = 0b100,
            /// DCO frequency select: 5
            DCOFSEL_5 = 0b101,
            /// DCO frequency select: 6
            DCOFSEL_6 = 0b110,
            /// DCO frequency select: 7
            DCOFSEL_7 = 0b111,
        }
        /// DCO range select.
        DCORSEL: 6 = struct DCORSEL(bool);
    }
    /// CS Control Register 2
    rw CSCTL2 @ 0x04: u16 = 0_0 {
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
    /// CS Control Register 3
    rw CSCTL3 @ 0x06: u16 = 0_0 {
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
    /// CS Control Register 4
    rw CSCTL4 @ 0x08: u16 = 0_0 {
        /// Low Frequency Oscillator (LFXT) disable
        LFXTOFF: 0 = struct LFXTOFF(bool);
        /// SMCLK Off
        SMCLKOFF: 1 = struct SMCLKOFF(bool);
        /// VLO Off
        VLOOFF: 3 = struct VLOOFF(bool);
        /// LFXT bypass mode : 0: internal 1:sourced from external pin
        LFXTBYPASS: 4 = struct LFXTBYPASS(bool);
        /// LFXT Drive Level mode Bit 0
        LFXTDRIVE: 6..7 = enum LFXTDRIVE {
            /// LFXT Drive Level mode: 0
            LFXTDRIVE_0 = 0b00,
            /// LFXT Drive Level mode: 1
            LFXTDRIVE_1 = 0b01,
            /// LFXT Drive Level mode: 2
            LFXTDRIVE_2 = 0b10,
            /// LFXT Drive Level mode: 3
            LFXTDRIVE_3 = 0b11,
        }
        /// High Frequency Oscillator disable
        HFXTOFF: 8 = struct HFXTOFF(bool);
        /// HFXT frequency selection Bit 1
        HFFREQ: 10..11 = enum HFFREQ {
            /// HFXT frequency selection: 0
            HFFREQ_0 = 0b00,
            /// HFXT frequency selection: 1
            HFFREQ_1 = 0b01,
            /// HFXT frequency selection: 2
            HFFREQ_2 = 0b10,
            /// HFXT frequency selection: 3
            HFFREQ_3 = 0b11,
        }
        /// HFXT bypass mode : 0: internal 1:sourced from external pin
        HFXTBYPASS: 12 = struct HFXTBYPASS(bool);
        /// HFXT Drive Level mode Bit 0
        HFXTDRIVE: 14..15 = enum HFXTDRIVE {
            /// HFXT Drive Level mode: 0
            HFXTDRIVE_0 = 0b00,
            /// HFXT Drive Level mode: 1
            HFXTDRIVE_1 = 0b01,
            /// HFXT Drive Level mode: 2
            HFXTDRIVE_2 = 0b10,
            /// HFXT Drive Level mode: 3
            HFXTDRIVE_3 = 0b11,
        }
    }
    /// CS Control Register 5
    rw CSCTL5 @ 0x0a: u16 = 0_0 {
        /// LFXT Low Frequency Oscillator Fault Flag
        LFXTOFFG: 0 = struct LFXTOFFG(bool);
        /// HFXT High Frequency Oscillator Fault Flag
        HFXTOFFG: 1 = struct HFXTOFFG(bool);
        /// Enable start counter for XT1
        ENSTFCNT1: 6 = struct ENSTFCNT1(bool);
        /// Enable start counter for XT2
        ENSTFCNT2: 7 = struct ENSTFCNT2(bool);
    }
    /// CS Control Register 6
    rw CSCTL6 @ 0x0c: u16 = 0_0 {
        /// ACLK Clock Request Enable
        ACLKREQEN: 0 = struct ACLKREQEN(bool);
        /// MCLK Clock Request Enable
        MCLKREQEN: 1 = struct MCLKREQEN(bool);
        /// SMCLK Clock Request Enable
        SMCLKREQEN: 2 = struct SMCLKREQEN(bool);
        /// MODOSC Clock Request Enable
        MODCLKREQEN: 3 = struct MODCLKREQEN(bool);
    }
}
