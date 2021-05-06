//! SAPH_A

utils::periph! {
    /// SAPH_A
    SAPH_A;
    /// Interrupt Index
    r SAPH_AIIDX @ 0x00: u16 = 0_0 {
        /// This register provides the highest priority enabled interrupt index.
        IIDX: 1..3 = enum IIDX {
            /// no interrupts pending
            NONE = 0b000,
            /// This interrupt indicates that either WINHI interrupt or WINLO interrupt has occurred in SDHS.
            DATAERR = 0b001,
            /// This interrupt is valid when ASQ is activcve (auto mode). The interrupt indicates that the time counter in ASQ has reached to TIMEMARK_F (timeout).
            TMFTO = 0b010,
            /// This interrupt is valid when ASQ is activcve (auto mode). The interrupt occurs when ASQ completes all of the measurements programmed in ASCTL0.PNGCNT. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that all of the four measurements have been completed.
            SEQDN = 0b011,
            /// This interrupt is valid when ASQ is active (auto mode). The interrupt occurs when ASQ completes one measurement sequence. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that one measurement has been completed.
            PNGDN = 0b100,
        }
    }
    /// Masked Interrupt Satus
    r SAPH_AMIS @ 0x02: u16 = 0_0 {
        /// This interrupt indicates that either WINHI interrupt or WINLO interrupt has occurred in SDHS.
        SAPH_AMIS_DATAERR: 0 = struct SAPH_AMIS_DATAERR(bool);
        /// This bit indicates a TIMEMARK F (timeout) event has happened.
        SAPH_AMIS_TMFTO: 1 = struct SAPH_AMIS_TMFTO(bool);
        /// This interrupt is valid when ASQ is activcve (auto mode). The interrupt occurs when ASQ completes all of the measurements programmed in ASCTL0.PNGCNT. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that all of the four measurements have been completed.
        SAPH_AMIS_SEQDN: 2 = struct SAPH_AMIS_SEQDN(bool);
        /// This interrupt is valid when ASQ is active (auto mode). The interrupt occurs when ASQ completes one measurement sequence. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that one measurement has been completed.
        SAPH_AMIS_PNGDN: 3 = struct SAPH_AMIS_PNGDN(bool);
    }
    /// Raw Interrupt Status
    r SAPH_ARIS @ 0x04: u16 = 0_0 {
        /// This interrupt indicates that either WINHI interrupt or WINLO interrupt has occurred in SDHS.
        SAPH_ARIS_DATAERR: 0 = struct SAPH_ARIS_DATAERR(bool);
        /// This bit indicates a TIMEMARK F (timeout) event has happened.
        SAPH_ARIS_TMFTO: 1 = struct SAPH_ARIS_TMFTO(bool);
        /// This interrupt is valid when ASQ is activcve (auto mode). The interrupt occurs when ASQ completes all of the measurements programmed in ASCTL0.PNGCNT. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that all of the four measurements have been completed.
        SAPH_ARIS_SEQDN: 2 = struct SAPH_ARIS_SEQDN(bool);
        /// This interrupt is valid when ASQ is active (auto mode). The interrupt occurs when ASQ completes one measurement sequence. For example, when ASCTL0.PNGCNT = 3, total four measurements are performed. The interrupt indicates that one measurement has been completed.
        SAPH_ARIS_PNGDN: 3 = struct SAPH_ARIS_PNGDN(bool);
        /// This bit indicates a DMA access violation
        SAPH_ARIS_DAV: 14 = struct SAPH_ARIS_DAV(bool);
    }
    /// Interrupt Mask
    rw SAPH_AIMSC @ 0x06: u16 = 0_0 {
        /// This bit enables the DATAERR interrupt.
        SAPH_AIMSC_DATAERR: 0 = struct SAPH_AIMSC_DATAERR(bool);
        /// This bit enables the TIMEMARK F (timeout) interrupt.
        SAPH_AIMSC_TMFTO: 1 = struct SAPH_AIMSC_TMFTO(bool);
        /// This bit enables the SEQDN interrupt
        SAPH_AIMSC_SEQDN: 2 = struct SAPH_AIMSC_SEQDN(bool);
        /// This bit enables the PNGDN interrupt
        SAPH_AIMSC_PNGDN: 3 = struct SAPH_AIMSC_PNGDN(bool);
        /// This bit indicates a DMA access violation
        SAPH_AIMSC_DAV: 4 = struct SAPH_AIMSC_DAV(bool);
    }
    /// Interrupt Clear
    rw SAPH_AICR @ 0x08: u16 = 0_0 {
        /// Writing one this bit to clear the pending DATAERR interrupt.
        SAPH_AICR_DATAERR: 0 = struct SAPH_AICR_DATAERR(bool);
        /// Writing one this bit to clear the pending TIMEMARK F (timeout) interrupt
        SAPH_AICR_TMFTO: 1 = struct SAPH_AICR_TMFTO(bool);
        /// Writing one this bit to clear the pending SEQDN interrupt.
        SAPH_AICR_SEQDN: 2 = struct SAPH_AICR_SEQDN(bool);
        /// Writing one this bit to clear the pending PNGDN interrupt.
        SAPH_AICR_PNGDN: 3 = struct SAPH_AICR_PNGDN(bool);
        /// Writing one this bit to clear any previous DMA access violations
        SAPH_AICR_DAV: 14 = struct SAPH_AICR_DAV(bool);
    }
    /// Interrupt Set
    rw SAPH_AISR @ 0x0a: u16 = 0_0 {
        /// Writing one this bit generates a DATAERR interrupt by software.
        SAPH_AISR_DATAERR: 0 = struct SAPH_AISR_DATAERR(bool);
        /// Writing one this bit to generate a TIMEMARK F (timeout) interrupt by software.
        SAPH_AISR_TMFTO: 1 = struct SAPH_AISR_TMFTO(bool);
        /// Writing one this bit to generate a SEQDN interrupt by software.
        SAPH_AISR_SEQDN: 2 = struct SAPH_AISR_SEQDN(bool);
        /// Writing one this bit to generate a PNGDN interrupt by software.
        SAPH_AISR_PNGDN: 3 = struct SAPH_AISR_PNGDN(bool);
        /// Writing one this bit to set RIS.DAV by software.
        SAPH_AISR_DAV: 14 = struct SAPH_AISR_DAV(bool);
    }
    /// Module-Descriptor Low Word
    r SAPH_ADESCLO @ 0x0c: u16 = 0_0 {
        /// Minor Revision
        MINREV: 0..3 = struct MINREV(u16);
        /// Major Revision
        MAJREV: 4..7 = struct MAJREV(u16);
        /// Instance Number
        INSTNUM: 8..11 = struct INSTNUM(u16);
        /// Feature Version
        FEATUREVER: 12..15 = struct FEATUREVER(u16);
    }
    /// Module-Descriptor High Word
    rw SAPH_ADESCHI @ 0x0e: u16 = 0_0 {
        /// Module-Descriptor High Word
        SAPH_ADESCHI: 0..15 = struct SAPH_ADESCHIField(u16);
    }
    /// Key
    rw SAPH_AKEY @ 0x10: u16 = 0_0 {
        /// Key
        SAPH_AKEY: 0..15 = struct SAPH_AKEYField(u16);
    }
    /// Physical Interface Output Control #0
    rw SAPH_AOCTL0 @ 0x12: u16 = 0_0 {
        /// CH0_OUT Enable. When OSEL.PCH0SEL =0, this bit enables the output CH0 when set to 1. When OSEL.PCH0SEL != 0, this bit is invalid.
        CH0OE: 0..0 = enum CH0OE {
            /// Ch0 Output is HiZ
            CH0OE_0 = 0b0,
            /// CH0 Output is driving
            CH0OE_1 = 0b1,
        }
        /// CH1_OUT Enable. When OSEL.PCH1SEL =0, this bit enables the output CH1 when set to 1. When OSEL.PCH1SEL != 0, this bit is invalid.
        CH1OE: 1..1 = enum CH1OE {
            /// Ch1 Output is HiZ
            CH1OE_0 = 0b0,
            /// CH1 Output is driving
            CH1OE_1 = 0b1,
        }
        /// CH0_OUT Value. When OSEL.PCH0SEL =0 and OCTL0.CH0OE=1, this bit represents the logical value on the CH0 terminal.  0 = low 1 = high
        CH0OUT: 8..8 = enum CH0OUT {
            /// Ch0 is set to low signal
            CH0OUT_0 = 0b0,
            /// Ch0 is set to high signal
            CH0OUT_1 = 0b1,
        }
        /// CH1_OUT Value. When OSEL.PCH1SEL =0 and OCTL0.CH1OE=1, this bit represents the logical value on the CH1 terminal.  0 = low 1 = high
        CH1OUT: 9..9 = enum CH1OUT {
            /// Ch1 is set to low signal
            CH1OUT_0 = 0b0,
            /// Ch1 is set to high signal
            CH1OUT_1 = 0b1,
        }
    }
    /// Physical Interface Output Control #1
    rw SAPH_AOCTL1 @ 0x14: u16 = 0_0 {
        /// CH0 termination switch (SWG0) enable.  When OSEL.PCH0SEL =0, this bit controls the SWG0 switch.  0 = SWG0 is off.   1 = SWG0 is on. The CH0_OUT is disabled and connected to PVSS via SWG0 switch. No need to change OCTL0.CH0OE status.
        CH0TERM: 0..0 = enum CH0TERM {
            /// CH0 Output is defined by CH0OUT and CH0OE
            CH0TERM_0 = 0b0,
            /// CH0 Output is set low with termination strength
            CH0TERM_1 = 0b1,
        }
        /// CH1 termination switch (SWG1) enable.  When OSEL.PCH1SEL =0, this bit controls the SWG1 switch.  0 = SWG1 is off.   1 = SWG1 is on. The CH1_OUT is disabled and connected to PVSS via SWG1 switch. No need to change OCTL0.CH1OE status.
        CH1TERM: 1..1 = enum CH1TERM {
            /// CH1 Output is defined by CH1OUT and CH1OE
            CH1TERM_0 = 0b0,
            /// CH1 Output is set low with termination strength
            CH1TERM_1 = 0b1,
        }
        /// DRV0 (output driver on the CH0_OUT) full strength enable.  0 = The DRV0 output impedance is deterimed by CH0PUT and CH0PDT registers.  1 = The DRV0 has lowest output impedance.
        CH0FP: 8..8 = enum CH0FP {
            /// Ch0 Output is set to normal strength
            CH0FP_0 = 0b0,
            /// Ch0 Output is set to maximum strength
            CH0FP_1 = 0b1,
        }
        /// DRV1 (output driver on the CH1_OUT) full strength enable.  0 = The DRV1 output impedance is deterimed by CH1PUT and CH1PDT registers.  1 = The DRV1 has lowest output impedance.
        CH1FP: 9..9 = enum CH1FP {
            /// Ch1 Output is set to normal strength
            CH1FP_0 = 0b0,
            /// Ch1 Output is set to maximum strength
            CH1FP_1 = 0b1,
        }
    }
    /// Physical Interface Output Function Select
    rw SAPH_AOSEL @ 0x16: u16 = 0_0 {
        /// Output functional select for CH0_OUT.
        PCH0SEL: 0..1 = enum PCH0SEL {
            /// CH0_OUT is used as a GPO pin. It is controlled by OCTL0.CH0OUT and OCTL0.CH0OE.
            GPIO = 0b00,
            /// CH0_OUT is driven by the PPG.
            PPGSE = 0b01,
            /// CH0_OUT is driven by the PPG as differential output along with CH1_OUT. (CH0_OUT and CH1_OUT are alwasy opposite polarity)
            PCH0SEL_2 = 0b10,
            /// CH0_OUT is used as a GPO pin. It is controlled by OCTL0.CH0OUT and OCTL0.CH0OE.
            PCH0SEL_3 = 0b11,
        }
        /// Output functional select for CH0_OUT.
        PCH1SEL: 2..3 = enum PCH1SEL {
            /// CH1_OUT is used as a GPO pin. It is controlled by OCTL0.CH1OUT and OCTL0.CH1OE.
            GPIO = 0b00,
            /// CH1_OUT is driven by the PPG.
            PPGSE = 0b01,
            /// CH1_OUT is driven by the PPG as differential output along with CH0_OUT. (CH0_OUT and CH1_OUT are alwasy opposite polarity)
            PCH1SEL_2 = 0b10,
            /// CH1_OUT is used as a GPO pin. It is controlled by OCTL0.CH1OUT and OCTL0.CH1OE.
            PCH1SEL_3 = 0b11,
        }
    }
    /// Channel 0 Pull UpTrim Register
    rw SAPH_ACH0PUT @ 0x20: u16 = 0_0 {
        /// DRV0 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation.
        CH0PUT: 0..3 = struct CH0PUT(u16);
    }
    /// Channel 0 Pull DownTrim Register
    rw SAPH_ACH0PDT @ 0x22: u16 = 0_0 {
        /// DRV0 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation.
        CH0PDT: 0..3 = struct CH0PDT(u16);
    }
    /// Channel 0 Termination Trim
    rw SAPH_ACH0TT @ 0x24: u16 = 0_0 {
        /// SWG0 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation.
        CH0TT: 0..3 = struct CH0TT(u16);
    }
    /// Channel 1 Pull UpTrim
    rw SAPH_ACH1PUT @ 0x26: u16 = 0_0 {
        /// DRV1 pull up trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation.
        CH1PUT: 0..3 = struct CH1PUT(u16);
    }
    /// Channel 1 Pull DownTrim
    rw SAPH_ACH1PDT @ 0x28: u16 = 0_0 {
        /// DRV1 pull down trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation.
        CH1PDT: 0..3 = struct CH1PDT(u16);
    }
    /// Channel 1 Termination Trim
    rw SAPH_ACH1TT @ 0x2a: u16 = 0_0 {
        /// SWG1 trim register. Write access is allowed only when TACR.UNLOCK=1. For secure the trim value, it is recommended to keep TACR.UNLOCK=0 during normal operation.
        CH1TT: 0..3 = struct CH1TT(u16);
    }
    /// Mode Configuration Register
    rw SAPH_AMCNF @ 0x2c: u16 = 0_0 {
        /// LPBE, low power bias mode enable. This bit enables the low power bias operation mode. The selection of the operation mode shall only be changed while the PSQ is in OFF state (changes during other states of the PSQ causes corrupt measurement results and irregular triggers of sub modules ba ASQ)
        LPBE: 11..11 = enum LPBE {
            /// For manual bias mode and regular ASQ bias mode. In this configuration the user controls by the ASQBSW has full control over the TxBias and RxBias switches.
            LPBE_0 = 0b0,
            /// Low power bias mode. In this mode the ASQ uses the CHxEBSW and PGABSW as auxiliary values to achieve faster channel setting on reactive input loads. The ASQ has full controls over the bias switch multiplexer.
            LPBE_1 = 0b1,
        }
        /// Reserved for future use
        RSV1: 9..10 = struct RSV1(u16);
        /// This bit enables the charge pump of the input multiplexer.
        CPEO: 8..8 = enum CPEO {
            /// Charge pump is turned on by SDHS and ASQ related requests only.
            CPEO_0 = 0b0,
            /// Charge pump is turned on regardless of SDHS and ASQ related charge pump requests.
            CPEO_1 = 0b1,
        }
        /// Reserved for future use
        RSV0: 2..3 = struct RSV0(u16);
        /// These bits define the impedance of the buffers for RxBias and TxBias. While for resistive loads the lowest impedance shows the fastest settling; is this not the case for reactive loads.
        BIMP: 0..1 = enum BIMP {
            /// 200 Ohms buffer impedance for RxBias and TxBias
            BIMP_0 = 0b00,
            /// 600 Ohms buffer impedance for RxBias and TxBias
            BIMP_1 = 0b01,
            /// 1200 Ohms buffer impedance for RxBias and TxBias (default)
            BIMP_2 = 0b10,
            /// 2800 Ohms buffer impedance for RxBias and TxBias
            BIMP_3 = 0b11,
        }
    }
    /// Trim Access Control
    rw SAPH_ATACTL @ 0x2e: u16 = 0_0 {
        /// When UNLOCK = 1, the trim registers are allowed to be updated (CH0PUT, CH0PDT, CH0TT, CH1PUT, CH1PDT, and CH1TT).
        UNLOCK: 0 = struct UNLOCK(bool);
    }
    /// Physical Interface Input Control #0
    rw SAPH_AICTL0 @ 0x30: u16 = 0_0 {
        /// Input Multiplexer Channel Select
        MUXSEL: 0..3 = enum MUXSEL {
            /// Channel 0 is selected for input
            CH0IN = 0b0000,
            /// Channel 1 is selected for input
            CH1IN = 0b0001,
            /// reserved for future channels
            MUXSEL_2 = 0b0010,
            /// reserved for future channels
            MUXSEL_3 = 0b0011,
            /// reserved for future channels
            MUXSEL_4 = 0b0100,
            /// reserved for future channels
            MUXSEL_5 = 0b0101,
            /// reserved for future channels
            MUXSEL_6 = 0b0110,
            /// reserved for future channels
            MUXSEL_7 = 0b0111,
            /// no channel is selected
            MUXSEL_8 = 0b1000,
            /// no channel is selected
            MUXSEL_9 = 0b1001,
            /// no channel is selected
            MUXSEL_10 = 0b1010,
            /// no channel is selected
            MUXSEL_11 = 0b1011,
            /// no channel is selected
            MUXSEL_12 = 0b1100,
            /// no channel is selected
            MUXSEL_13 = 0b1101,
            /// no channel is selected
            MUXSEL_14 = 0b1110,
            /// no channel is selected
            MUXSEL_15 = 0b1111,
        }
        /// PGA dummy load enable on the deselected multiplexer inputs.
        DUMEN: 7..7 = enum DUMEN {
            /// PGA dummy input load is Hi-Z.
            DUMEN_0 = 0b0,
            /// PGA dummy input load matches the PGA input impedance.
            DUMEN_1 = 0b1,
        }
        /// Input Multiplexer Control source
        MUXCTL: 4..4 = enum MUXCTL {
            /// The input multiplexer is controlled by ICTL0.MUXSEL (register mode)
            MUXCTL_0 = 0b0,
            /// The input multiplexer is controlled by ASQ (auto mode)
            MUXCTL_1 = 0b1,
        }
        /// External PGA Bias Switch 0 control
        XPBSW0: 12..12 = enum XPBSW0 {
            /// External bias switch is open (no bias driven)
            XPBSW0_0 = 0b0,
            /// External  bias switch is closed (bias is driven)
            XPBSW0_1 = 0b1,
        }
        /// External PGA Bias Switch 1 control
        XPBSW1: 13..13 = enum XPBSW1 {
            /// External bias switch is open (no bias driven)
            XPBSW1_0 = 0b0,
            /// External  bias switch is closed (bias is driven)
            XPBSW1_1 = 0b1,
        }
        /// XPB0 Pin Function Enable
        XPB0FEN: 8..8 = enum XPB0FEN {
            /// XPB0 pin function disabled
            XPB0FEN_0 = 0b0,
            /// XPB0 pin function enabled
            XPB0FEN_1 = 0b1,
        }
        /// XPB1 Pin Function Enable
        XPB1FEN: 9..9 = enum XPB1FEN {
            /// XPB1 pin function disabled
            XPB1FEN_0 = 0b0,
            /// XPB1 pin function enabled
            XPB1FEN_1 = 0b1,
        }
    }
    /// Bias Control
    rw SAPH_ABCTL @ 0x34: u16 = 0_0 {
        /// Tx bias and Rx bias switches control source select
        ASQBSC: 0..0 = enum ASQBSC {
            /// Bias switches are controlled by BCTL.CH0EBSW, BCTL.CH1EBSW, BCTL.PGABSW bits (register mode).
            ASQBSC_0 = 0b0,
            /// Bias switches are controlled by ASQ (auto mode)
            ASQBSC_1 = 0b1,
        }
        /// Rx bias (PGA bias) switch control. Note that the channel to apply the Rx bias is determined by ICTL0.MUXSEL.
        PGABSW: 1..1 = enum PGABSW {
            /// Rx bias switch is open.
            PGABSW_0 = 0b0,
            /// Rx bias switch is closed (enabled).
            PGABSW_1 = 0b1,
        }
        /// Excitation bias (Rx bias) Voltage Select
        EXCBIAS: 4..5 = enum EXCBIAS {
            /// 0.2V nominal
            EXCBIAS_0 = 0b00,
            /// 0.3V nominal
            EXCBIAS_1 = 0b01,
            /// 0.4V nominal
            EXCBIAS_2 = 0b10,
            /// 0.6V nominal
            EXCBIAS_3 = 0b11,
        }
        /// PGA bias (Rx bias) Voltage Select
        PGABIAS: 6..7 = enum PGABIAS {
            /// 0.75V nominal
            PGABIAS_0 = 0b00,
            /// 0.8V nominal
            PGABIAS_1 = 0b01,
            /// 0.9V nominal
            PGABIAS_2 = 0b10,
            /// 0.95V nominal
            PGABIAS_3 = 0b11,
        }
        /// Channel 0 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS.
        CH0EBSW: 8..8 = enum CH0EBSW {
            /// Tx bias switch to CH0 is open.
            CH0EBSW_0 = 0b0,
            /// Tx bias switch to CH0 is closed (enabled).
            CH0EBSW_1 = 0b1,
        }
        /// Channel 1 Tx bias Switch Control. Note that the Tx bias voltage is determined by BCTL.EXCBIAS.
        CH1EBSW: 9..9 = enum CH1EBSW {
            /// Tx bias switch to CH1 is open.
            CH1EBSW_0 = 0b0,
            /// Tx bias switch to CH1 is closed (enabled).
            CH1EBSW_1 = 0b1,
        }
        /// Enable the power supply (Charge Pump) for the the input multiplexer during data acquisition.
        CPDA: 3..3 = enum CPDA {
            /// Turn off the charge pump during data acquisition.
            CPDA_0 = 0b0,
            /// Keep turning on the charge pump during data acquisition.
            CPDA_1 = 0b1,
        }
        /// Line input leakage compensation (LILC) enable.
        LILC: 2..2 = enum LILC {
            /// LICL is disabled.
            LILC_0 = 0b0,
            /// LICL is enabled.
            LILC_1 = 0b1,
        }
    }
    /// PPG Count
    rw SAPH_APGC @ 0x40: u16 = 0_0 {
        /// Excitation Pulse Count. This bit field defines the number of excitation pulses. Minimum value is zero.
        EPULS: 0..6 = struct EPULS(u16);
        /// Stop Pulse Count; This bit field defines the number of stop pulses. Minimum value is zero. Stop pulses have the inverted polarity of excitation pulses.
        SPULS: 8..11 = struct SPULS(u16);
        /// Pulse Polarity. This bit defines the polarity of the first excitation pulse.
        PPOL: 13..13 = enum PPOL {
            /// The excitation begins with logical high phase. The stop begines with logical low phase.
            PPOL_0 = 0b0,
            /// The excitation begins with logical low phase. The stop begines with logical high phase.
            PPOL_1 = 0b1,
        }
        /// PPG ouptut level during inactive. This bit affects the status of PPG output before and after excitations. Note that this bit is only valid when PGC.PHIZ =0.
        PLEV: 14..14 = enum PLEV {
            /// PPG output is low during inactive
            PLEV_0 = 0b0,
            /// PPG output is high during inactive
            PLEV_1 = 0b1,
        }
        /// Hi-Z enable to PPG output during inactive.
        PHIZ: 15..15 = enum PHIZ {
            /// PPG output during inactive is determined by PGC.PLEV bit.
            PHIZ_0 = 0b0,
            /// PPG output is in Hi-Z during inactive regardless of PGC.PLEV bit.
            PHIZ_1 = 0b1,
        }
    }
    /// Pulse Generator Low Period
    rw SAPH_APGLPER @ 0x42: u16 = 0_0 {
        /// Low phase period of PPG excitation pulses. This value defines the length of the low phase of the pulses. The minimum count is two regardless of the value set in this register.
        LPER: 0..7 = struct LPER(u16);
    }
    /// Pulse Generator High Period
    rw SAPH_APGHPER @ 0x44: u16 = 0_0 {
        /// High phase period of PPG excitation pulses. This value defines the length of the high phase of the pulses. The minimum count is two regardless of the value set in this register.
        HPER: 0..7 = struct HPER(u16);
    }
    /// PPG Control
    rw SAPH_APGCTL @ 0x46: u16 = 0_0 {
        /// PPG output channel select source.
        PGSEL: 0..0 = enum PGSEL {
            /// PPG output channel is selected by PGCTL.PPGCHSEL bit (register mode).
            PGSEL_0 = 0b0,
            /// PPG output channel is selected by ASQ (auto mode).
            PGSEL_1 = 0b1,
        }
        /// PPG output channel select when PGCTL.PGSEL = 0.
        PPGCHSEL: 1..1 = enum PPGCHSEL {
            /// CH0 is selected
            PPGCHSEL_0 = 0b0,
            /// CH1 is selected
            PPGCHSEL_1 = 0b1,
        }
        /// PPG Trigger source select.
        TRSEL: 4..5 = enum TRSEL {
            /// Writing 1 to PPGTRIG.PPGTRIG to trigger the PPG (start pulse generation).
            TRSEL_0 = 0b00,
            /// PPG trigger is controlled by the ASQ.
            TRSEL_1 = 0b01,
            /// Ext. Signal (See device specific datasheet)
            TRSEL_2 = 0b10,
            /// Ext. Signal (See device specific datasheet)
            TRSEL_3 = 0b11,
        }
        /// PPGEN PPG trigger enable. This bit can be used to indicates that the configuration of the pulse generator is complete. The PPG can only be started when this bit is set to 1 regardless of its trigger source. It is recommended to keep this bit zero while updating PPG registers.
        PPGEN: 9..9 = enum PPGEN {
            /// PPG trigger is disabled.
            PPGEN_0 = 0b0,
            /// PPG trigger is enabled.
            PPGEN_1 = 0b1,
        }
        /// Tone generation enable. The frequency of the test tone is determined by LPER and HPER. The test tone persists while PGCTL.TONE = 1 & PGCTL.STOP=0. Either writing 0 to PGCTL.TONE or writing 1  to PGCTL.STOP stops tone generation.
        TONE: 14..14 = enum TONE {
            /// Test tone generation is disabled.  Note: This bit is automatically cleared when writing '1' to PGCTL.STOP, and it stops test tone generation immediately.
            DISABLE = 0b0,
            /// Test tone generation is enabled.
            ENABLE = 0b1,
        }
        /// Writing one to this bit stopps the PPG to generate pulses. This bit is cleared automatically.
        PPGSTOP: 15 = struct PPGSTOP(bool);
        /// PPG pre scaler enable.
        PSCEN: 13..13 = enum PSCEN {
            /// Prescaler is disabled. PPG clock = PLL output clock.
            DISABLE = 0b0,
            /// Prescaler by four is enabled. PPG clock = 1/4 of the PLL output clock.
            ENABLE = 0b1,
        }
    }
    /// PPG Software Trigger
    rw SAPH_APPGTRIG @ 0x48: u16 = 0_0 {
        /// Writing '1' to this bit triggers the PPG to generate pulses when PGCTL.TRSEL =0.    Note: This bit is write only. Reading always returns with zero.
        PPGTRIG: 0 = struct PPGTRIG(bool);
    }
    /// Extended Pulse Control Register
    rw SAPH_AXPGCTL @ 0x4a: u16 = 0_0 {
        /// XPULS Extra Pulse Count; This bit field defines the count of extra excitation pulses excited. If set to zero no extra excitation pulses are generated. Directly after the start the regular excitation  phase is entered. Any other number will generate up the number of excitation pulses set by this field .
        XPULS: 0..6 = struct XPULS(u16);
        /// Phase Status of PPG
        XSTAT: 8..9 = enum XSTAT {
            /// PPG in Pause Phase
            XSTAT_0 = 0b00,
            /// PPG in Stop Phase
            XSTAT_1 = 0b01,
            /// PPG in Regular Excitation phase
            XSTAT_2 = 0b10,
            /// PPG in Extra Excitation Phase
            XSTAT_3 = 0b11,
        }
        /// Extended Mode Select
        XMOD: 12..13 = enum XMOD {
            /// Single tone generation (Pause-E-S-Pause)
            XMOD_0 = 0b00,
            /// reserved
            XMOD_1 = 0b01,
            /// Dual Tone Generation (Pause-X-E-S-Pause)
            XMOD_2 = 0b10,
            /// Dual Tone Loop (X-E-X-E triggers events)
            XMOD_3 = 0b11,
        }
        /// Event type. Selects the type of the event generated in XMOD=3
        ETY: 14..14 = enum ETY {
            /// Timer count event
            ETY_0 = 0b0,
            /// DMA trigger event
            ETY_1 = 0b1,
        }
    }
    /// Extra Pulse Low Period Register
    rw SAPH_AXPGLPER @ 0x4c: u16 = 0_0 {
        /// XLPER low phase period of the extra pulses. This value defines the length of the low phase of the extra pulses in units of high speed clocks. The minimum count is two regardless of the value set in this register.
        XLPER: 0..7 = struct XLPER(u16);
    }
    /// Extra Pulse High Period Register
    rw SAPH_AXPGHPER @ 0x4e: u16 = 0_0 {
        /// XHPER high phase period of the extra pulses. This value defines the length of the high phase of the pulses in units of high speed clocks. The minimum count is two  regardless of the value set in this register.
        XHPER: 0..7 = struct XHPER(u16);
    }
    /// A-SEQ control register 0
    rw SAPH_AASCTL0 @ 0x60: u16 = 0_0 {
        /// The total number of measurements to be performed.  0 = 1 measurement will be performed (Min) 1 = 2 measurements will be performed 2 = 3 measurements will be performed 3 = 4 measurements will be performed (Max) Note: This bit field is static, does not reflect the currently reamining measurement numbers.
        PNGCNT: 0..1 = struct PNGCNT(u16);
        /// This bit selects the channel to start with when ASQ contols the measuremnet sequences.
        ASQCHSEL: 4..4 = enum ASQCHSEL {
            /// CH0 is selected to start with.  If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =1, then the channel selection would be CH0 - CH1 - CH0 - CH1. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =0, then the channel selection would be CH0 - CH0 - CH0 - CH0.
            ASQCHSEL_0 = 0b0,
            /// CH1 is selected to start with.  If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =0, then the channel selection would be CH1 - CH1 - CH1 - CH1. If ASCTL0.PNGCNT = 3 and ASCTL1.CHTOG =1, then the channel selection would be CH1 - CH0 - CH1 - CH0.
            ASQCHSEL_1 = 0b1,
        }
        /// Stop the ASQ. Writing '1' to this bit stops the measurement sequence controlled by the ASQ.  This bit is self cleared.
        ASQSTOP: 7 = struct ASQSTOP(bool);
        /// ASQ Trigger Enable. This bit can be used to indicate that the configuration of the ASQ is complete. The ASQ can only be triggered when this bit is set to '1' regardless of its trigger source. It is recommended to keep this bit zero while updating ASQ registerds.
        ASQTEN: 9..9 = enum ASQTEN {
            /// ASQ trigger is disabled.
            ASQEN_0 = 0b0,
            /// ASQ trigger is enabled.
            ASQEN_1 = 0b1,
        }
        /// ASQ trigger select.
        TRIGSEL: 10..11 = enum TRIGSEL {
            /// Writing '1' to ASQTRIG.ASQTRIG
            SWTRIG = 0b00,
            /// The PSQ is selected to start the ASQ.
            PSQ = 0b01,
            /// Ext. Signal (See device specific datasheet)
            TIMER = 0b10,
            /// Ext. Signal (See device specific datasheet)
            TRIGSEL_3 = 0b11,
        }
        /// Stop ASQ when the DATAERR interrupt occurrs when the mesurements controlled by ASQ (auto mode). Note that it is not possible to resume the measurement from where it was stopped. When ASQ is triggered again, the measurement seqeunce starts from the beginning.
        ERABRT: 13..13 = enum ERABRT {
            /// Continue the measurements until completion regardless of the DATAERR interrupt.
            ERABRT_0 = 0b0,
            /// Stop the ASQ upon the DATAERR interrupt.
            ERABRT_1 = 0b1,
        }
    }
    /// A-SEQ control register 1
    rw SAPH_AASCTL1 @ 0x62: u16 = 0_0 {
        /// Channel toggle enable at each PNGDN interrupt.
        CHTOG: 0..0 = enum CHTOG {
            /// Channel toggle is disabled.
            CHTOG_0 = 0b0,
            /// Channel toggle is enabled at each PNGDN interrupt.
            CHTOG_1 = 0b1,
        }
        /// Read Only bit. This bit indicates the currently selected Tx channel.
        CHACT: 4 = struct CHACT(bool);
        /// Early Receive Bias Control. The Rx bias is applied at the TIMEMARK C, but when this bit is set to '1', the Rx bias is applied at the TIMEMARK A.
        EARLYRB: 7..7 = enum EARLYRB {
            /// Rx bias is applied to the Rx channel by the TIMEMARK C
            EARLYRB_0 = 0b0,
            /// Rx bias is applied to the Rx channel by the TIMEMARK A
            EARLYRB_1 = 0b1,
        }
        /// Enable the OFF request when ASQ completes all of the measurement sequences.
        ESOFF: 8..8 = enum ESOFF {
            /// OFF request is disabled. The ASQ does not send a request about USS power mode to the PSQ.
            ESOFF_0 = 0b0,
            /// OFF request is generated after sequence
            ESOFF_1 = 0b1,
        }
        /// ASQ can send a request sigal to the PSQ (Power Sequencer) of the USS module when the OFF request is received (See ASCTL1.ESOFF and the UUPS module).
        STDBY: 10..10 = enum STDBY {
            /// The ASQ sends a power down request to the PSQ (Power Sequencer) when the OFF request is received.
            PWROFF = 0b0,
            /// The ASQ sends a standby request to the PSQ (Power Sequencer) when the OFF request is received.
            STDBY = 0b1,
        }
        /// In general, if CH0 is selected for Tx, CH1 is selected for Rx. However, when this bit is set to '1', the Tx channel and Rx channel are identical. This bit can be used for debugging purpose.
        CHOWN: 11..11 = enum CHOWN {
            /// Tx channel and Rx channel are not the same (This is the typical configuration).
            CHOWN_0 = 0b0,
            /// Tx channel and Rx channel are identical.
            CHOWN_1 = 0b1,
        }
    }
    /// ASQ Software Trigger
    rw SAPH_AASQTRIG @ 0x64: u8 = 0_0 {
        /// Writing '1' to this bit trigger the ASQ when ASCTL0.TRIGSEL = 0. Note: This bit is write only. Reading always returns with zero.
        ASQTRIG: 0 = struct ASQTRIG(bool);
    }
    /// ASQ ping output polarity
    rw SAPH_AAPOL @ 0x66: u16 = 0_0 {
        /// Bit 0 defines the PPG pulse polarity for the first measurement.  Bit 1 defines the PPG pulse polarity for the second measurement.  Bit 2 defines the PPG pulse polarity for the third measurement.  Bit 3 defines the PPG pulse polarity for the fourth measurement.   0 = PPG output pulses starts with logical high polarity. 1 = PPG output pulses starts with logical low polarity.
        PCPOL: 0..3 = struct PCPOL(u16);
    }
    /// ASQ ping pause level
    rw SAPH_AAPLEV @ 0x68: u16 = 0_0 {
        /// Bit 0 defines the PPG output level at pause for the first measurement when PCPHIZ bit 0 = 0. Bit 1 defines the PPG output level at pause for the second measurement when PCPHIZ bit 1 = 0. Bit 2 defines the PPG output level at pause for the third measurement when PCPHIZ bit 2 = 0. Bit 3 defines the PPG output level at pause for the fourth measurement when PCPHIZ bit 3 = 0.   0 = Logical Low. 1 = Logical High.
        PCPLEV: 0..3 = struct PCPLEV(u16);
    }
    /// ASQ ping pause impedance
    rw SAPH_AAPHIZ @ 0x6a: u16 = 0_0 {
        /// Bit 0 defines the PPG output status at pause for the first measurement.  Bit 1 defines the PPG output status at pause for the second measurement.  Bit 2 defines the PPG output status at pause for the third measurement.  Bit 3 defines the PPG output status at pause for the fourth measurement.   0 = PPG ouput level is determined by APLEV.PCPLEV bits 1 = Hi-z. regardless of APLEV.PCPLEV bits
        PCPHIZ: 0..3 = struct PCPHIZ(u16);
    }
    /// A-SEQ start to 1st ping
    rw SAPH_AATM_A @ 0x6e: u16 = 0_0 {
        /// A-SEQ start to 1st ping
        SAPH_AATM_A: 0..15 = struct SAPH_AATM_AField(u16);
    }
    /// ASQ start to ADC arm
    rw SAPH_AATM_B @ 0x70: u16 = 0_0 {
        /// ASQ start to ADC arm
        SAPH_AATM_B: 0..15 = struct SAPH_AATM_BField(u16);
    }
    /// Count for the TIMEMARK C Event
    rw SAPH_AATM_C @ 0x72: u16 = 0_0 {
        /// Count for the TIMEMARK C Event
        SAPH_AATM_C: 0..15 = struct SAPH_AATM_CField(u16);
    }
    /// ASQ start to ADC trig
    rw SAPH_AATM_D @ 0x74: u16 = 0_0 {
        /// ASQ start to ADC trig
        SAPH_AATM_D: 0..15 = struct SAPH_AATM_DField(u16);
    }
    /// ASQ start to restart
    rw SAPH_AATM_E @ 0x76: u16 = 0_0 {
        /// ASQ start to restart
        SAPH_AATM_E: 0..15 = struct SAPH_AATM_EField(u16);
    }
    /// ASQ start to timeout
    rw SAPH_AATM_F @ 0x78: u16 = 0_0 {
        /// ASQ start to timeout
        SAPH_AATM_F: 0..15 = struct SAPH_AATM_FField(u16);
    }
    /// Time Base Control
    rw SAPH_ATBCTL @ 0x7a: u16 = 0_0 {
        /// The ASQ time counter clear. Writing '1' to this bit clears the the counter value. The counter must be stopped prior to be cleared. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero.
        TCLR: 0 = struct TCLR(bool);
        /// The ASQ time counter start. Writing '1' to this bit starts the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero.
        TSTART: 1 = struct TSTART(bool);
        /// The ASQ time counter stop. Writing '1' to this bit stops the counter. This bit is self cleared. TSTOP, TSTART, and TCLR bits are offerred for only debugging purpose. It is not recommend to use this bit while ASQ is active. Note: This bit is write only. Reading always returns with zero.
        TSTOP: 2 = struct TSTOP(bool);
        /// ASQ pre-scaler shift. The value written to the PSSV bits shifts the start point of the ASQ's pre-scaler. Note that the value only affects the first cycle of the pre-scaler.   0 = No shift 1 = The pre-scaler starts 1 clock later  2 = The pre-scaler starts 2 clocks later  ... 15 = The pre-scaler starts 15 clocks later
        PSSV: 4..7 = struct PSSV(u16);
    }
    /// Acquisition Timer Low Part
    rw SAPH_AATIMLO @ 0x7c: u16 = 0_0 {
        /// Acquisition Timer Low Part
        SAPH_AATIMLO: 0..15 = struct SAPH_AATIMLOField(u16);
    }
    /// Acquisition Timer High Part
    r SAPH_AATIMHI @ 0x7e: u16 = 0_0 {
        /// ASQ Timer Counter high part. The reading this register returns the counter value [19:16].
        ATIMHI: 0..3 = struct ATIMHI(u16);
    }
}
