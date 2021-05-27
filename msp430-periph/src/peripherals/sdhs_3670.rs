//! SDHS

utils::periph! {
    /// SDHS
    SDHS;
    /// Interrupt Index Register
    r IIDX @ 0x00: u16 = 0_0 {
        /// SDHS Interrupt Vector Value.
        IIDX: 1..15 = enum IIDXField {
            /// No Interrupt pending.
            IIDX_0 = 0b000000000000000,
            /// Interrupt Source: RIS.OVF; Interrupt Priority: Highest
            IIDX_1 = 0b000000000000001,
            /// Interrupt Source: RIS.ACQDONE
            IIDX_2 = 0b000000000000010,
            /// Interrupt Source: RIS.SSTRG
            IIDX_3 = 0b000000000000011,
            /// Interrupt Source: RIS.DTRDY
            IIDX_4 = 0b000000000000100,
            /// Interrupt Source: RIS.WINHI
            IIDX_5 = 0b000000000000101,
            /// Interrupt Source: RIS.WINLO
            IIDX_6 = 0b000000000000110,
            /// Reserved; Interrupt
            IIDX_7 = 0b000000000000111,
            /// Reserved; Interrupt Priority: Lowest
            IIDX_8 = 0b000000000001000,
        }
    }
    /// Masked Interrupt Status and Clear Register
    r MIS @ 0x02: u16 = 0_0 {
        /// SDHS Data Ready Masked Interrupt Status bit.
        MIS_DTRDY: 3 = enum MIS_DTRDY {
            /// No interrupt pending
            DTRDY_0 = 0b0,
            /// Interrupt pending
            DTRDY_1 = 0b1,
        }
        /// SDHS Start Conversion Trigger Masked Interrupt Status bit.
        MIS_SSTRG: 2 = enum MIS_SSTRG {
            /// No interrupt pending
            SSTRG_0 = 0b0,
            /// Interrupt pending
            SSTRG_1 = 0b1,
        }
        /// SDHS Data Overflow Masked Interrupt Status bit.
        MIS_OVF: 0 = enum MIS_OVF {
            /// No interrupt pending
            OVF_0 = 0b0,
            /// Interrupt pending
            OVF_1 = 0b1,
        }
        /// Acquisition Done Masked Interrupt Status bit.
        MIS_ACQDONE: 1 = enum MIS_ACQDONE {
            /// No interrupt pending
            ACQDONE_0 = 0b0,
            /// Interrupt pending
            ACQDONE_1 = 0b1,
        }
        /// SDHS Window High Masked Interrupt Status bit.
        MIS_WINHI: 4 = enum MIS_WINHI {
            /// No interrupt pending
            WINHI_0 = 0b0,
            /// Interrupt pending
            WINHI_1 = 0b1,
        }
        /// SDHS Window Low Masked Interrupt Status and Clear bit.
        MIS_WINLO: 5 = enum MIS_WINLO {
            /// No interrupt pending
            WINLO_0 = 0b0,
            /// Interrupt pending
            WINLO_1 = 0b1,
        }
    }
    /// Raw Interrupt Status Register
    r RIS @ 0x04: u16 = 0_0 {
        /// SDHS Data Ready Raw Interrupt Status bit.
        RIS_DTRDY: 3 = enum RIS_DTRDY {
            /// No DTRDY event
            DTRDY_0 = 0b0,
            /// The data buffer has become empty.
            DTRDY_1 = 0b1,
        }
        /// SDHS Start Conversion Trigger Raw Interrupt Status bit.
        RIS_SSTRG: 2 = enum RIS_SSTRG {
            /// No SSTRG event
            SSTRG_0 = 0b0,
            /// Converson Start signal has been asserted
            SSTRG_1 = 0b1,
        }
        /// SDHS Data Overflow Raw Interrupt Status bit.
        RIS_OVF: 0 = enum RIS_OVF {
            /// No OVF event
            OVF_0 = 0b0,
            /// When DTC is enabled (CTL2.DTCOFF = 0), DTC has dropped at least one sample. This indicates that the system clock needs to be increased.  When DTC is disabled (CTL2.DTCOFF = 1),  At least one new sample has been overwritten to SDHSDT register before the previous value is read.
            OVF_1 = 0b1,
        }
        /// Acquisition Done Raw Interrupt Status bit
        RIS_ACQDONE: 1 = enum RIS_ACQDONE {
            /// No ACQDONE event
            ACQDONE_0 = 0b0,
            /// Data conversion has been finished (either complete or incomplete).
            ACQDONE_1 = 0b1,
        }
        /// SDHS Window High Raw Interrupt Status bit.
        RIS_WINHI: 4 = enum RIS_WINHI {
            /// No WINHI event
            WINHI_0 = 0b0,
            /// The output data value is higher than the value in the WINHITH register
            WINHI_1 = 0b1,
        }
        /// SDHS Window Low Raw Interrupt Status bit.
        RIS_WINLO: 5 = enum RIS_WINLO {
            /// No new data is lower than the value in the WINLOTH register
            WINLO_0 = 0b0,
            /// New data is low than the value in the WINLOTH register
            WINLO_1 = 0b1,
        }
        /// Incomplete Stop Status bit.
        RIS_ISTOP: 15 = enum RIS_ISTOP {
            /// No ISTOP event
            ISTOP_0 = 0b0,
            /// Conversion has been interrupted and stopped before completing the number of samples defined in CTL2.SAMPSZ.
            ISTOP_1 = 0b1,
        }
    }
    /// Interrupt Mask Register
    rw IMSC @ 0x06: u16 = 0_0 {
        /// SDHS Data Ready Interrupt Mask bit.
        IMSC_DTRDY: 3 = enum IMSC_DTRDY {
            /// Interrupt is disabled
            DTRDY_0 = 0b0,
            /// Interrupt is enabled
            DTRDY_1 = 0b1,
        }
        /// SDHS Start Conversion Trigger Interrupt Mask bit.
        IMSC_SSTRG: 2 = enum IMSC_SSTRG {
            /// Interrupt is disabled
            SSTRG_0 = 0b0,
            /// Interrupt is enabled
            SSTRG_1 = 0b1,
        }
        /// SDHS Data Overflow Interrupt Mask bit.
        IMSC_OVF: 0 = enum IMSC_OVF {
            /// Interrupt is disabled
            OVF_0 = 0b0,
            /// Interrupt is enabled
            OVF_1 = 0b1,
        }
        /// Acquisition Done  Interrupt Mask bit.
        IMSC_ACQDONE: 1 = enum IMSC_ACQDONE {
            /// Interrupt is disabled
            ACQDONE_0 = 0b0,
            /// Interrupt is enabled
            ACQDONE_1 = 0b1,
        }
        /// SDHS Window High Interrupt Mask bit.
        IMSC_WINHI: 4 = enum IMSC_WINHI {
            /// Interrupt is disabled
            WINHI_0 = 0b0,
            /// Interrupt is enabled
            WINHI_1 = 0b1,
        }
        /// SDHS Window Low Interrupt Mask bit.
        IMSC_WINLO: 5 = enum IMSC_WINLO {
            /// Interrupt is disabled
            WINLO_0 = 0b0,
            /// Interrupt is enabled
            WINLO_1 = 0b1,
        }
    }
    /// Interrupt Clear Register.
    rw ICR @ 0x08: u16 = 0_0 {
        /// SDHS Start Conversion Trigger Interrupt Clear bit.
        ICR_SSTRG: 2 = struct ICR_SSTRG(bool);
        /// SDHS Data Overflow Interrupt Clear bit.
        ICR_OVF: 0 = struct ICR_OVF(bool);
        /// Acquisition Done  Interrupt Clear bit.
        ICR_ACQDONE: 1 = struct ICR_ACQDONE(bool);
        /// SDHS Window High Interrupt Clear bit.
        ICR_WINHI: 4 = struct ICR_WINHI(bool);
        /// SDHS Window Low Interrupt Clear bit.
        ICR_WINLO: 5 = struct ICR_WINLO(bool);
        /// Incomplete Stop Interrupt Clear bit.
        ICR_ISTOP: 15 = struct ICR_ISTOP(bool);
        /// SDHS Data Ready Interrupt Clear bit.
        ICR_DTRDY: 3 = struct ICR_DTRDY(bool);
    }
    /// Interrupt Set Register.
    rw ISR @ 0x0a: u16 = 0_0 {
        /// SDHS Data Ready Interrupt Set bit.
        ISR_DTRDY: 3 = struct ISR_DTRDY(bool);
        /// SDHS Start Conversion Trigger Interrupt Set bit.
        ISR_SSTRG: 2 = struct ISR_SSTRG(bool);
        /// SDHS Data Overflow Interrupt Set bit.
        ISR_OVF: 0 = struct ISR_OVF(bool);
        /// Acquisition Done Interrupt Set bit.
        ISR_ACQDONE: 1 = struct ISR_ACQDONE(bool);
        /// SDHS Window High Interrupt Set bit.
        ISR_WINHI: 4 = struct ISR_WINHI(bool);
        /// SDHS Window Low Interrupt Set bit.
        ISR_WINLO: 5 = struct ISR_WINLO(bool);
        /// Incomplete Stop Interrupt Set bit.
        ISR_ISTOP: 15 = struct ISR_ISTOP(bool);
    }
    /// SDHS Descriptor Register L.
    r DESCLO @ 0x0c: u16 = 0_0 {
        /// Minor Revision
        MINREV: 0..3 = struct MINREV(u16);
        /// Instance Number within the device.
        INSTNUM: 8..11 = struct INSTNUM(u16);
        /// Major Revision
        MAJREV: 4..7 = struct MAJREV(u16);
        /// Feature Set for the module
        FEATUREVER: 12..15 = struct FEATUREVER(u16);
    }
    /// SDHS Descriptor Register H.
    rw DESCHI @ 0x0e: u16 = 0_0 {
        /// SDHS Descriptor Register H.
        DESCHI: 0..15 = struct DESCHIField(u16);
    }
    /// SDHS Control Register 0
    rw CTL0 @ 0x10: u16 = 0_0 {
        /// Data alignment
        DALGN: 7 = enum DALGN {
            /// Right-aligned.
            DALGN_0 = 0b0,
            /// Left-aligned.
            DALGN_1 = 0b1,
        }
        /// Data format
        DFMSEL: 8..9 = enum DFMSEL {
            /// 2's complement
            DFMSEL_0 = 0b00,
            /// Offset binary
            DFMSEL_1 = 0b01,
            /// Reserved (defaults to 0, 2s complement)
            DFMSEL_2 = 0b10,
            /// Reserved (defaults to 0, 2s complement)
            DFMSEL_3 = 0b11,
        }
        /// DTRDY Interrupt delay select.  This regiser can be used to discard up to 7 samples after conversion start.  Note that the skipped samples will be lost.
        INTDLY: 1..3 = enum INTDLY {
            /// No dealy
            INTDLY_0 = 0b000,
            /// 1 sample delay, 2nd sample is the first interrupt
            INTDLY_1 = 0b001,
            /// 2 samples delay, 3rd sample is the first interrupt
            INTDLY_2 = 0b010,
            /// 3 samples delay, 4rd sample is the first interrupt
            INTDLY_3 = 0b011,
            /// 4 samples delay, 5th sample is the first interrupt
            INTDLY_4 = 0b100,
            /// 5 samples delay, 6th sample is the first interrupt
            INTDLY_5 = 0b101,
            /// 6 samples delay, 7th sample is the first interrupt
            INTDLY_6 = 0b110,
            /// 7 samples delay, 8th sample is the first interrupt
            INTDLY_7 = 0b111,
        }
        /// SDHS Auto Sample Start Disable
        AUTOSSDIS: 0 = enum AUTOSSDIS {
            /// Auto Sample start enabled. SDHS is powered up when the SHDS_PWR_UP applied, then data conversion is automatically started once the SDHS is fully powered up.
            AUTOSSDIS_0 = 0b0,
            /// Auto Sample start disabled.  (This configuration must be used when the ASQ controls the measurement sequences) - SHDS_PWR_UP signal to turns on the SDHS - CONVERSION_START signal to start data convesion
            AUTOSSDIS_1 = 0b1,
        }
        /// SDHS trigger source select.
        TRGSRC: 15 = enum TRGSRC {
            /// Register control mode:  - CTL4.SDHSON is the source of the SHDS_PWR_UP/DOWN signal - CTL5.SSTART is the source of the CONVERSION_START/STOP signal
            TRGSRC_0 = 0b0,
            /// ASQ control mode: The SDHS is controlled by the ASQ. - ASQ_ACQARM signal from the ASQ is the source of the SHDS_PWR_UP/DOWN signal - ASQ_ACQTRIG signal from the ASQ is the source of the CONVERSION_START/STOP signal
            TRGSRC_1 = 0b1,
        }
        /// Output Bit Resolution
        OBR: 10..11 = enum OBR {
            /// 12-bit
            OBR_0 = 0b00,
            /// 13-bit
            OBR_1 = 0b01,
            /// 14-bit
            OBR_2 = 0b10,
            /// Reserved (default: 12-bit)
            OBR_3 = 0b11,
        }
        /// MSB Shift
        SHIFT: 12..13 = enum SHIFT {
            /// No Shift, MSB.
            SHIFT_0 = 0b00,
            /// MSB - 1 (Shift left by 1 from filter out). If OBR = 2, then this configuration is invalid. No shift is performed.
            SHIFT_1 = 0b01,
            /// MSB -2 (Shift left by 2 from filter out). If OBR = 1, then this configuration is invalid. No shift is performed.
            SHIFT_2 = 0b10,
            /// Reserved (No shift)
            SHIFT_3 = 0b11,
        }
    }
    /// SDHS Control Register 1
    rw CTL1 @ 0x12: u16 = 0_0 {
        /// Over Sampling Rate.
        OSR: 0..3 = enum OSR {
            /// 10
            OSR_0 = 0b0000,
            /// 20
            OSR_1 = 0b0001,
            /// 40
            OSR_2 = 0b0010,
            /// 80
            OSR_3 = 0b0011,
            /// 160
            OSR_4 = 0b0100,
        }
    }
    /// SDHS Control Register 2
    rw CTL2 @ 0x14: u16 = 0_0 {
        /// Disable sampling size counting.
        SMPCTLOFF: 10 = enum SMPCTLOFF {
            /// Total sampling size is determined by SMPSZ bits. The SDHS automatically stops data conversion.
            SMPCTLOFF_0 = 0b0,
            /// SMPSZ bits are ignored. Conversion does not stop until the trigger source selected by TRGSRC bits is deasserted.
            SMPCTLOFF_1 = 0b1,
        }
        /// Total Sample Size.
        SMPSZ: 0..9 = struct SMPSZ(u16);
        /// Data Transfer Controller (DTC) Off
        DTCOFF: 15 = enum DTCOFF {
            /// DTC enabled. The DTC automatically transfers the data from the SDHSDT register to the address specified in the DTCDA register.
            DTCOFF_0 = 0b0,
            /// DTC disabled. The data in the SDHSDT register must be read by CPU, otherwise the overflow interrupt flag (RIS.OVF) will eventually be asserted.
            DTCOFF_1 = 0b1,
        }
        /// Window Comparator Enable
        WINCMPEN: 14 = enum WINCMPEN {
            /// Window Comparator is disabled
            WINCMPEN_0 = 0b0,
            /// Window Comparator is enabled
            WINCMPEN_1 = 0b1,
        }
    }
    /// SDHS Control Register 3
    rw CTL3 @ 0x16: u16 = 0_0 {
        /// SDHS Trigger Enable bit
        TRIGEN: 0 = enum TRIGEN {
            /// SDHS Trigger is disabled. Once this bit is de-asserted, CTL0,  CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers are unlocked (allowed to be modified).
            TRIGEN_0 = 0b0,
            /// SDHS Trigger is enabled. Once this bit is asserted, CTL0,  CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers are locked (not allowed to be modified).
            TRIGEN_1 = 0b1,
        }
    }
    /// SDHS Control Register 4
    rw CTL4 @ 0x18: u16 = 0_0 {
        /// SDHS Power-up
        ON: 0 = enum ON {
            /// Power down the SDHS module
            ON_0 = 0b0,
            /// Power on the SDHS module
            ON_1 = 0b1,
        }
    }
    /// SDHS Control Register 5
    rw CTL5 @ 0x1a: u16 = 0_0 {
        /// Start of conversion.
        SSTART: 0 = enum SSTART {
            /// Stop conversion
            SSTART_0 = 0b0,
            /// Start conversion
            SSTART_1 = 0b1,
        }
        /// Start of conversion.
        SDHS_LOCK: 8 = enum SDHS_LOCK {
            /// CTL3 register is unlocked.
            SDHS_LOCK_0 = 0b0,
            /// CTL3 register is locked as well as  CTL0,  CTL1, CTL2, CTL7,WINHITH, WINLOTH, and DTCDA registers. Only read is allowed.
            SDHS_LOCK_1 = 0b1,
        }
    }
    /// SDHS Control Register 6
    rw CTL6 @ 0x1c: u16 = 0_0 {
        /// PGA Gain Control bits
        PGA_GAIN: 0..5 = struct PGA_GAIN(u16);
    }
    /// SDHS Control Register 7
    rw CTL7 @ 0x1e: u16 = 0_0 {
        /// SDHS Modulator Optimization bits.
        MODOPTI: 0..4 = struct MODOPTI(u16);
    }
    /// SDHS Data Converstion Register
    rw DT @ 0x22: u16 = 0_0 {
        /// SDHS Data Converstion Register
        DT: 0..15 = struct DTField(u16);
    }
    /// SDHS Window Comparator High Threshold Register.
    rw WINHITH @ 0x24: u16 = 0_0 {
        /// SDHS Window Comparator High Threshold Register.
        WINHITH: 0..15 = struct WINHITHField(u16);
    }
    /// SDHS Window Comparator Low Threshold Register.
    rw WINLOTH @ 0x26: u16 = 0_0 {
        /// SDHS Window Comparator Low Threshold Register.
        WINLOTH: 0..15 = struct WINLOTHField(u16);
    }
    /// DTC destination address register
    rw DTCDA @ 0x28: u16 = 0_0 {
        /// DTC destination address.
        DTCDA: 0..14 = struct DTCDAField(u16);
    }
}
