//! CS  Clock System

utils::periph! {
    /// CS  Clock System
    CSClockSystem;
    /// CS Control Register 0
    rw CSCTL0 @ 0x00: u16 = 0_0 {
        /// DCO TAP Bit : 0
        DCO0: 0 = struct DCO0(bool);
        /// DCO TAP Bit : 1
        DCO1: 1 = struct DCO1(bool);
        /// DCO TAP Bit : 2
        DCO2: 2 = struct DCO2(bool);
        /// DCO TAP Bit : 3
        DCO3: 3 = struct DCO3(bool);
        /// DCO TAP Bit : 4
        DCO4: 4 = struct DCO4(bool);
        /// DCO TAP Bit : 5
        DCO5: 5 = struct DCO5(bool);
        /// DCO TAP Bit : 6
        DCO6: 6 = struct DCO6(bool);
        /// DCO TAP Bit : 7
        DCO7: 7 = struct DCO7(bool);
        /// DCO TAP Bit : 8
        DCO8: 8 = struct DCO8(bool);
        /// Modulation Bit Counter Bit : 0
        MOD0: 9 = struct MOD0(bool);
        /// Modulation Bit Counter Bit : 1
        MOD1: 10 = struct MOD1(bool);
        /// Modulation Bit Counter Bit : 2
        MOD2: 11 = struct MOD2(bool);
        /// Modulation Bit Counter Bit : 3
        MOD3: 12 = struct MOD3(bool);
        /// Modulation Bit Counter Bit : 4
        MOD4: 13 = struct MOD4(bool);
    }
    /// CS Control Register 1
    rw CSCTL1 @ 0x02: u16 = 0_0 {
        /// Disable Modulation
        DISMOD: 0 = struct DISMOD(bool);
        /// DCO frequency range select Bit: 0
        DCORSEL: 1..3 = enum DCORSEL {
            /// DCO frequency range select: 0
            DCORSEL_0 = 0b000,
            /// DCO frequency range select: 1
            DCORSEL_1 = 0b001,
            /// DCO frequency range select: 2
            DCORSEL_2 = 0b010,
            /// DCO frequency range select: 3
            DCORSEL_3 = 0b011,
            /// DCO frequency range select: 4
            DCORSEL_4 = 0b100,
            /// DCO frequency range select: 5
            DCORSEL_5 = 0b101,
            /// DCO frequency range select: 6
            DCORSEL_6 = 0b110,
            /// DCO frequency range select: 7
            DCORSEL_7 = 0b111,
        }
        /// DCO frequency trim. Bit: 0
        DCOFTRIM: 4..6 = enum DCOFTRIM {
            /// DCO frequency trim: 0
            DCOFTRIM_0 = 0b000,
            /// DCO frequency trim: 1
            DCOFTRIM_1 = 0b001,
            /// DCO frequency trim: 2
            DCOFTRIM_2 = 0b010,
            /// DCO frequency trim: 3
            DCOFTRIM_3 = 0b011,
            /// DCO frequency trim: 4
            DCOFTRIM_4 = 0b100,
            /// DCO frequency trim: 5
            DCOFTRIM_5 = 0b101,
            /// DCO frequency trim: 6
            DCOFTRIM_6 = 0b110,
            /// DCO frequency trim: 7
            DCOFTRIM_7 = 0b111,
        }
        /// DCO frequency trim enable
        DCOFTRIMEN: 7 = struct DCOFTRIMEN(bool);
    }
    /// CS Control Register 2
    rw CSCTL2 @ 0x04: u16 = 0_0 {
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
            /// Multiply Selected Loop Freq. By 1
            FLLD_0 = 0b000,
            /// Multiply Selected Loop Freq. By 2
            FLLD_1 = 0b001,
            /// Multiply Selected Loop Freq. By 4
            FLLD_2 = 0b010,
            /// Multiply Selected Loop Freq. By 8
            FLLD_3 = 0b011,
            /// Multiply Selected Loop Freq. By 16
            FLLD_4 = 0b100,
            /// Multiply Selected Loop Freq. By 32
            FLLD_5 = 0b101,
            /// Reserved
            FLLD_6 = 0b110,
            /// Reserved
            FLLD_7 = 0b111,
        }
    }
    /// CS Control Register 3
    rw CSCTL3 @ 0x06: u16 = 0_0 {
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
        SELREF: 4..5 = enum SELREF {
            /// FLL Reference Clock Select 0
            SELREF_0 = 0b00,
            /// FLL Reference Clock Select 1
            SELREF_1 = 0b01,
            /// FLL Reference Clock Select 2
            SELREF_2 = 0b10,
            /// FLL Reference Clock Select 3
            SELREF_3 = 0b11,
        }
    }
    /// CS Control Register 4
    rw CSCTL4 @ 0x08: u16 = 0_0 {
        /// MCLK and SMCLK Source Select Bit: 0
        SELMS: 0..2 = enum SELMS {
            /// MCLK and SMCLK Source Select 0
            SELMS_0 = 0b000,
            /// MCLK and SMCLK Source Select 1
            SELMS_1 = 0b001,
            /// MCLK and SMCLK Source Select 2
            SELMS_2 = 0b010,
            /// MCLK and SMCLK Source Select 3
            SELMS_3 = 0b011,
            /// MCLK and SMCLK Source Select 4
            SELMS_4 = 0b100,
            /// MCLK and SMCLK Source Select 5
            SELMS_5 = 0b101,
            /// MCLK and SMCLK Source Select 6
            SELMS_6 = 0b110,
            /// MCLK and SMCLK Source Select 7
            SELMS_7 = 0b111,
        }
        /// ACLK Source Select Bit: 0
        SELA: 8 = struct SELA(bool);
    }
    /// CS Control Register 5
    rw CSCTL5 @ 0x0a: u16 = 0_0 {
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
        DIVS: 4..5 = enum DIVS {
            /// SMCLK Source Divider 0
            DIVS_0 = 0b00,
            /// SMCLK Source Divider 1
            DIVS_1 = 0b01,
            /// SMCLK Source Divider 2
            DIVS_2 = 0b10,
            /// SMCLK Source Divider 3
            DIVS_3 = 0b11,
        }
        /// SMCLK off
        SMCLKOFF: 8 = struct SMCLKOFF(bool);
        /// VLO automatic off enable
        VLOAUTOOFF: 12 = struct VLOAUTOOFF(bool);
    }
    /// CS Control Register 6
    rw CSCTL6 @ 0x0c: u16 = 0_0 {
        /// XT1 automatic off enable
        XT1AUTOOFF: 0 = struct XT1AUTOOFF(bool);
        /// XT1 Automatic Gain Control (AGC) disable
        XT1AGCOFF: 1 = struct XT1AGCOFF(bool);
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
    }
    /// CS Control Register 7
    rw CSCTL7 @ 0x0e: u16 = 0_0 {
        /// DCO fault flag
        DCOFFG: 0 = struct DCOFFG(bool);
        /// XT1 Low Frequency Oscillator Fault Flag
        XT1OFFG: 1 = struct XT1OFFG(bool);
        /// FLL unlock interrupt flag
        FLLULIFG: 4 = struct FLLULIFG(bool);
        /// Enable start counter for XT1
        ENSTFCNT1: 6 = struct ENSTFCNT1(bool);
        /// FLL unlock condition Bit: 0
        FLLUNLOCK: 8..9 = enum FLLUNLOCK {
            /// FLL unlock condition: 0
            FLLUNLOCK_0 = 0b00,
            /// FLL unlock condition: 1
            FLLUNLOCK_1 = 0b01,
            /// FLL unlock condition: 2
            FLLUNLOCK_2 = 0b10,
            /// FLL unlock condition: 3
            FLLUNLOCK_3 = 0b11,
        }
        /// Unlock history Bit: 0
        FLLUNLOCKHIS: 10..11 = enum FLLUNLOCKHIS {
            /// Unlock history: 0
            FLLUNLOCKHIS_0 = 0b00,
            /// Unlock history: 1
            FLLUNLOCKHIS_1 = 0b01,
            /// Unlock history: 2
            FLLUNLOCKHIS_2 = 0b10,
            /// Unlock history: 3
            FLLUNLOCKHIS_3 = 0b11,
        }
        /// FLL unlock PUC enable
        FLLULPUC: 12 = struct FLLULPUC(bool);
        /// Warning enable
        FLLWARNEN: 13 = struct FLLWARNEN(bool);
    }
    /// CS Control Register 8
    rw CSCTL8 @ 0x10: u16 = 0_0 {
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
