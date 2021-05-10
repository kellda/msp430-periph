//! CS

utils::periph! {
    /// CS
    CS;
    /// Clock System Control 0
    rw CSCTL0 @ 0x00: u16 = 0_0 {
        /// DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation.
        DCO: 0..8 = struct DCO(u16);
        /// Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased.
        MOD_: 9..13 = struct MOD_(u16);
    }
    /// Clock System Control 1
    rw CSCTL1 @ 0x02: u16 = 0_0 {
        /// Modulation. This bit enables/disables the modulation.
        DISMOD: 0 = enum DISMOD {
            /// Modulation enabled
            DISMOD_0 = 0b0,
            /// Modulation disabled
            DISMOD_1 = 0b1,
        }
        /// DCO Range Select
        DCORSEL: 1..3 = enum DCORSEL {
            /// 1 MHz
            DCORSEL_0 = 0b000,
            /// 2 MHz
            DCORSEL_1 = 0b001,
            /// 4 MHz
            DCORSEL_2 = 0b010,
            /// 8 MHz
            DCORSEL_3 = 0b011,
            /// 12 MHz
            DCORSEL_4 = 0b100,
            /// 16 MHz
            DCORSEL_5 = 0b101,
            /// 20 MHz(Only avaliable in 24MHz clock system)
            DCORSEL_6 = 0b110,
            /// 24 MHz(Only avaliable in 24MHz clock system)
            DCORSEL_7 = 0b111,
        }
        /// DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code.
        DCOFTRIM: 4..6 = struct DCOFTRIM(u16);
        /// DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture.
        DCOFTRIMEN: 7 = enum DCOFTRIMEN {
            /// Disable frequency trim
            DCOFTRIMEN_0 = 0b0,
            /// Enable frequency trim
            DCOFTRIMEN_1 = 0b1,
        }
    }
    /// Clock System Control 2
    rw CSCTL2 @ 0x04: u16 = 0_0 {
        /// Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1.
        FLLN: 0..9 = struct FLLN(u16);
        /// FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits.
        FLLD: 12..14 = enum FLLD {
            /// fDCOCLK / 1
            _1 = 0b000,
            /// fDCOCLK / 2
            _2 = 0b001,
            /// fDCOCLK / 4
            _4 = 0b010,
            /// fDCOCLK / 8
            _8 = 0b011,
            /// fDCOCLK / 16
            _16 = 0b100,
            /// fDCOCLK / 32
            _32 = 0b101,
            /// fDCOCLK / 40(Only avaliable in 24MHz clock system)
            FLLD_6 = 0b110,
            /// fDCOCLK / 48(Only avaliable in 24MHz clock system)
            FLLD_7 = 0b111,
        }
    }
    /// Clock System Control 3
    rw CSCTL3 @ 0x06: u16 = 0_0 {
        /// FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1
        FLLREFDIV: 0..2 = enum FLLREFDIV {
            /// fFLLREFCLK / 1
            _1 = 0b000,
            /// fFLLREFCLK / 32
            _32 = 0b001,
            /// fFLLREFCLK / 64
            _64 = 0b010,
            /// fFLLREFCLK / 128
            _128 = 0b011,
            /// fFLLREFCLK / 256
            _256 = 0b100,
            /// fFLLREFCLK / 512
            _512 = 0b101,
            /// fFLLREFCLK / 640 (only available in 24MHz clock system)
            FLLREFDIV_6 = 0b110,
            /// fFLLREFCLK / 768(only available in 24MHz clock system)
            FLLREFDIV_7 = 0b111,
        }
        /// FLL reference select. These bits select the FLL reference clock source.
        SELREF: 4..5 = enum SELREF {
            /// XT1CLK
            XT1CLK = 0b00,
            /// REFOCLK
            REFOCLK = 0b01,
            /// served for future use
            SELREF_2 = 0b10,
            /// served for future use
            SELREF_3 = 0b11,
        }
        /// REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set.
        REFOLP: 7 = enum REFOLP {
            /// REFO Low Power Disabled (High Power Mode)
            REFOLP_0 = 0b0,
            /// REFO Low Power Enabled
            REFOLP_1 = 0b1,
        }
    }
    /// Clock System Control 4
    rw CSCTL4 @ 0x08: u16 = 0_0 {
        /// Selects the MCLK and SMCLK source
        SELMS: 0..2 = enum SELMS {
            /// DCOCLKDIV
            DCOCLKDIV = 0b000,
            /// REFOCLK
            REFOCLK = 0b001,
            /// XT1CLK
            XT1CLK = 0b010,
            /// VLOCLK
            VLOCLK = 0b011,
            /// Reserved for future use
            SELMS_4 = 0b100,
            /// Reserved for future use
            SELMS_5 = 0b101,
            /// Reserved for future use
            SELMS_6 = 0b110,
            /// Reserved for future use
            SELMS_7 = 0b111,
        }
        /// Selects the ACLK source
        SELA: 8..9 = enum SELA {
            /// XT1CLK with divider (must be no more than 40 kHz)
            XT1CLK = 0b00,
            /// REFO (internal 32-kHz clock source)
            REFOCLK = 0b01,
            /// VLO (internal 10-kHz clock source)
            VLOCLK = 0b10,
            /// Reserved
            RESERVED = 0b11,
        }
    }
    /// Clock System Control 5
    rw CSCTL5 @ 0x0a: u16 = 0_0 {
        /// MCLK source divider
        DIVM: 0..2 = enum DIVM {
            /// /1
            _1 = 0b000,
            /// /2
            _2 = 0b001,
            /// /4
            _4 = 0b010,
            /// /8
            _8 = 0b011,
            /// /16
            _16 = 0b100,
            /// /32
            _32 = 0b101,
            /// /64
            _64 = 0b110,
            /// /128
            _128 = 0b111,
        }
        /// SMCLK source divider. SMCLK directly derives from MCLK. SMCLK frequency is the combination of DIVM and DIVS out of selected clock source.
        DIVS: 4..5 = enum DIVS {
            /// /1
            _1 = 0b00,
            /// /2
            _2 = 0b01,
            /// /4
            _4 = 0b10,
            /// /8
            _8 = 0b11,
        }
        /// SMCLK off. This bit turns off SMCLK clock
        SMCLKOFF: 8 = enum SMCLKOFF {
            /// SMCLK on
            SMCLKOFF_0 = 0b0,
            /// SMCLK off
            SMCLKOFF_1 = 0b1,
        }
        /// VLO automatic off enable. This bit turns off VLO, if VLO is not used.
        VLOAUTOOFF: 12 = enum VLOAUTOOFF {
            /// VLO always on
            VLOAUTOOFF_0 = 0b0,
            /// VLO automatically turned off if not used(default)
            VLOAUTOOFF_1 = 0b1,
        }
    }
    /// Clock System Control 6
    rw CSCTL6 @ 0x0c: u16 = 0_0 {
        /// XT1 automatic off enable. This bit allows XT1 turned turns off when it is not used
        XT1AUTOOFF: 0 = enum XT1AUTOOFF {
            /// XT1 is on if XT1 is selected by the port selection and XT1 is not in bypass mode of operation.
            XT1AUTOOFF_0 = 0b0,
            /// XT1 is off if it is not used as a source for ACLK, MCLK, or SMCLK or is not used as a reference source required for FLL operation.
            XT1AUTOOFF_1 = 0b1,
        }
        /// Automatic Gain Control (AGC) disable.
        XT1AGCOFF: 1 = enum XT1AGCOFF {
            /// AGC on
            XT1AGCOFF_0 = 0b0,
            /// AGC off
            XT1AGCOFF_1 = 0b1,
        }
        /// The XT1 High-frequency selection. These bits must be set to appropriate frequency for crystal or bypass modes of operation.
        XT1HFFREQ: 2..3 = enum XT1HFFREQ {
            /// 1 to 4 MHz
            XT1HFFREQ_0 = 0b00,
            /// 4 MHz to 6 MHz
            XT1HFFREQ_1 = 0b01,
            /// 6 MHz to 16 MHz
            XT1HFFREQ_2 = 0b10,
            /// 16 MHz to 24 MHz
            XT1HFFREQ_3 = 0b11,
        }
        /// XT1 bypass select
        XT1BYPASS: 4 = enum XT1BYPASS {
            /// XT1 source internally
            XT1BYPASS_0 = 0b0,
            /// XT1 sources externally from pin
            XT1BYPASS_1 = 0b1,
        }
        /// XT1 mode select
        XTS: 5 = enum XTS {
            /// Low-frequency mode.
            XTS_0 = 0b0,
            /// High-frequency mode.
            XTS_1 = 0b1,
        }
        /// The XT1 oscillator current can be adjusted to its drive needs. Initially, it starts with the highest supply current for reliable and quick startup. If needed, user software can reduce the drive strength. The configuration of these bits is retained during LPM3.5 until LOCKLPM5 is cleared, but not the register bits itself; therefore, reconfiguration after wake-up from LPM3.5 before clearing LOCKLPM5 is required.
        XT1DRIVE: 6..7 = enum XT1DRIVE {
            /// Lowest drive strength and current consumption
            XT1DRIVE_0 = 0b00,
            /// Lower drive strength and current consumption
            XT1DRIVE_1 = 0b01,
            /// Higher drive strength and current consumption
            XT1DRIVE_2 = 0b10,
            /// Highest drive strength and current consumption
            XT1DRIVE_3 = 0b11,
        }
        /// ACLK source divider.
        DIVA: 8..11 = enum DIVA {
            /// /1
            _1 = 0b0000,
            /// /16
            _16 = 0b0001,
            /// /32
            _32 = 0b0010,
            /// /64
            _64 = 0b0011,
            /// /128
            _128 = 0b0100,
            /// /256
            _256 = 0b0101,
            /// /384
            _384 = 0b0110,
            /// /512
            _512 = 0b0111,
            /// /768(Only available in 24MHz clock system, 24 MHz preference)
            _768 = 0b1000,
            /// /1024(Only available in 24MHz clock system, 24 MHz preference)
            _1024 = 0b1001,
            /// /108(Only available in 24MHz clock system, 24 MHz preference)
            _108 = 0b1010,
            /// 338(Only available in 24MHz clock system, 24 MHz preference)
            _338 = 0b1011,
            /// 414(Only available in 24MHz clock system, 24 MHz preference)
            _414 = 0b1100,
            /// 640(Only available in 24MHz clock system, 24 MHz preference)
            _640 = 0b1101,
            /// Reserved
            DIVA_14 = 0b1110,
            /// Reserved
            DIVA_15 = 0b1111,
        }
        /// The XT1 oscillator fault detection off
        XT1FAULTOFF: 13 = enum XT1FAULTOFF {
            /// Enabling XT1 fault to switch ACLK to REFO
            XT1FAULTOFF_0 = 0b0,
            /// Disabling XT1 fault to switch ACLK to REFO
            XT1FAULTOFF_1 = 0b1,
        }
    }
    /// Clock System Control Register 7
    rw CSCTL7 @ 0x0e: u16 = 0_0 {
        /// REFO ready flag. This bit reflects the REFO readiness whent REFO is good for operation (such as FLL reference)
        REFOREADY: 2 = enum REFOREADY {
            /// REFO unstable
            REFOREADY_0 = 0b0,
            /// REFO ready to go
            REFOREADY_1 = 0b1,
        }
        /// DCO fault flag. If this bit is set, the OFIFG flag is also set. The DCOFFG bit is set if DCO = {0} or DCO = {511}. DCOFFG can be cleared by software. If the DCO fault condition still remains, DCOFFG is set. As long as DCOFFG is set, FLLUNLOCK shows the DCOERROR condition.
        DCOFFG: 0 = enum DCOFFG {
            /// No fault condition occurred after the last reset.
            DCOFFG_0 = 0b0,
            /// DCO fault. A DCO fault occurred after the last reset.
            DCOFFG_1 = 0b1,
        }
        /// T1 oscillator fault flag. If this bit is set, the OFIFG flag is also set. XT1OFFG is set if a XT1 fault condition exists. XT1OFFG can be cleared by software. If the XT1 fault condition still remains, XT1OFFG is set.
        XT1OFFG: 1 = enum XT1OFFG {
            /// No fault condition occurred after the last reset.
            XT1OFFG_0 = 0b0,
            /// XT1 fault. An XT1 fault occurred after the last reset.
            XT1OFFG_1 = 0b1,
        }
        /// FLL unlock interrupt flag. This flag is set when FLLUNLOCK bits equal 10b (DCO too fast). If FLLULPUC is also set, a PUC is triggered when FLLUIFG is set.
        FLLULIFG: 4 = enum FLLULIFG {
            /// FLLUNLOCK bits not equal to 10b
            FLLULIFG_0 = 0b0,
            /// FLLUNLOCK bits equal to 10b
            FLLULIFG_1 = 0b1,
        }
        /// Enable start counter for XT1.
        ENSTFCNT1: 6 = enum ENSTFCNT1 {
            /// Startup fault counter disabled. Counter is cleared..
            ENSTFCNT1_0 = 0b0,
            /// Startup fault counter enabled.
            ENSTFCNT1_1 = 0b1,
        }
        /// Unlock. These bits indicate the current FLL unlock condition. These bits are both set as long as the DCOFFG flag is set.
        FLLUNLOCK: 8..9 = enum FLLUNLOCK {
            /// FLL is locked. No unlock condition currently active.
            FLLUNLOCK_0 = 0b00,
            /// DCOCLK is currently too slow.
            FLLUNLOCK_1 = 0b01,
            /// DCOCLK is currently too fast.
            FLLUNLOCK_2 = 0b10,
            /// DCOERROR. DCO out of range.
            FLLUNLOCK_3 = 0b11,
        }
        /// Unlock history bits. These bits indicate the FLL unlock condition history. As soon as any unlock condition happens, the respective bits are set and remain set until cleared by software by writing 0 to it or by a POR.
        FLLUNLOCKHIS: 10..11 = enum FLLUNLOCKHIS {
            /// FLL is locked. No unlock situation has been detected since the last reset of these bits.
            FLLUNLOCKHIS_0 = 0b00,
            /// DCOCLK has been too slow since the bits were cleared.
            FLLUNLOCKHIS_1 = 0b01,
            /// DCOCLK has been too fast since the bits were cleared.
            FLLUNLOCKHIS_2 = 0b10,
            /// DCOCLK has been both too fast and too slow since the bits were cleared.
            FLLUNLOCKHIS_3 = 0b11,
        }
        /// FLL unlock PUC enable. If the FLLULPUC bit is set, a reset (PUC) is triggered if FLLULIFG is set. FLLULIFG indicates when FLLUNLOCK bits equal 10 (too fast). FLLULPUC is automatically cleared upon servicing the event. If FLLULPUC is cleared (0), no PUC can be triggered by FLLULIFG.
        FLLULPUC: 12 = struct FLLULPUC(bool);
        /// Warning enable. If this bit is set, an interrupt is generated based on the FLLUNLOCKHIS bits. If FLLUNLOCKHIS is not equal to 00, an OFIFG is generated.
        FLLWARNEN: 13 = enum FLLWARNEN {
            /// FLLUNLOCKHIS status cannot set OFIFG.
            FLLWARNEN_0 = 0b0,
            /// FLLUNLOCKHIS status can set OFIFG.
            FLLWARNEN_1 = 0b1,
        }
    }
    /// Clock System Control Register 8
    rw CSCTL8 @ 0x10: u16 = 0_0 {
        /// ACLK clock request enable. Setting this enables conditional module requests for ACLK
        ACLKREQEN: 0 = enum ACLKREQEN {
            /// ACLK conditional requests are disabled.
            ACLKREQEN_0 = 0b0,
            /// ACLK conditional requests are enabled.
            ACLKREQEN_1 = 0b1,
        }
        /// MCLK clock request enable. Setting this enables conditional module requests for MCLK
        MCLKREQEN: 1 = enum MCLKREQEN {
            /// MCLK conditional requests are disabled.
            MCLKREQEN_0 = 0b0,
            /// MCLK conditional requests are enabled.
            MCLKREQEN_1 = 0b1,
        }
        /// SMCLK clock request enable. Setting this enables conditional module requests for SMCLK
        SMCLKREQEN: 2 = enum SMCLKREQEN {
            /// SMCLK conditional requests are disabled.
            SMCLKREQEN_0 = 0b0,
            /// SMCLK conditional requests are enabled.
            SMCLKREQEN_1 = 0b1,
        }
        /// MODOSC clock request enable. Setting this enables conditional module requests for MODOSC.
        MODOSCREQEN: 3 = enum MODOSCREQEN {
            /// MODOSC conditional requests are disabled.
            MODOSCREQEN_0 = 0b0,
            /// MODOSC conditional requests are enabled.
            MODOSCREQEN_1 = 0b1,
        }
    }
}
