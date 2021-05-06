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
        DCOFSEL: 1..2 = enum DCOFSEL {
            /// DCO frequency select: 0
            DCOFSEL_0 = 0b00,
            /// DCO frequency select: 1
            DCOFSEL_1 = 0b01,
            /// DCO frequency select: 2
            DCOFSEL_2 = 0b10,
            /// DCO frequency select: 3
            DCOFSEL_3 = 0b11,
        }
        /// DCO range select.
        DCORSEL: 7 = struct DCORSEL(bool);
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
        /// High Frequency Oscillator 1 (XT1) disable
        XT1OFF: 0 = struct XT1OFF(bool);
        /// SMCLK Off
        SMCLKOFF: 1 = struct SMCLKOFF(bool);
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
        /// XT2 bypass mode : 0: internal 1:sourced from external pin
        XT2BYPASS: 12 = struct XT2BYPASS(bool);
        /// XT2 Drive Level mode Bit 0
        XT2DRIVE: 14..15 = enum XT2DRIVE {
            /// XT2 Drive Level mode: 0
            XT2DRIVE_0 = 0b00,
            /// XT2 Drive Level mode: 1
            XT2DRIVE_1 = 0b01,
            /// XT2 Drive Level mode: 2
            XT2DRIVE_2 = 0b10,
            /// XT2 Drive Level mode: 3
            XT2DRIVE_3 = 0b11,
        }
    }
    /// CS Control Register 5
    rw CSCTL5 @ 0x0a: u16 = 0_0 {
        /// XT1 Low Frequency Oscillator Fault Flag
        XT1OFFG: 0 = struct XT1OFFG(bool);
        /// High Frequency Oscillator 2 Fault Flag
        XT2OFFG: 1 = struct XT2OFFG(bool);
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
