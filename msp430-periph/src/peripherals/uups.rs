//! UUPS

utils::periph! {
    /// UUPS
    UUPS;
    /// Interrupt Index Register
    r IIDX @ 0x00: u16 = 0_0 {
        /// UUPS Interrupt Vector Value.
        IIDX: 1..15 = enum IIDXField {
            /// No Interrupt pending
            IIDX_0 = 0b000000000000000,
            /// Interrupt Source: PTMOUT; Interrupt Priority: Highest
            IIDX_1 = 0b000000000000001,
            /// Interrupt Source: PREQIG
            IIDX_2 = 0b000000000000010,
            /// Interrupt Source: STPBYDB
            IIDX_3 = 0b000000000000011,
            /// Reserved; Interrupt Priority: Lowest
            IIDX_4 = 0b000000000000100,
        }
    }
    /// Masked Interrupt Status Register
    r MIS @ 0x02: u16 = 0_0 {
        /// UUPS Power Up Time Out Masked Interrupt Status bit.
        MIS_PTMOUT: 0 = enum MIS_PTMOUT {
            /// No interrupt pending
            PTMOUT_0 = 0b0,
            /// Interrupt pending
            PTMOUT_1 = 0b1,
        }
        /// UUPS Power Request Ignore Masked Interrupt Status bit.
        MIS_PREQIG: 1 = enum MIS_PREQIG {
            /// No interrupt pending
            PREQIG_0 = 0b0,
            /// Interrupt pending
            PREQIG_1 = 0b1,
        }
        /// USS has been interrupted by debug mode Masked Interrupt Status bit.
        MIS_STPBYDB: 2 = enum MIS_STPBYDB {
            /// No interrupt pending
            STPBYDB_0 = 0b0,
            /// Interrupt pending
            STPBYDB_1 = 0b1,
        }
    }
    /// Raw Interrupt Status Register
    r RIS @ 0x04: u16 = 0_0 {
        /// UUPS Power Up Timer Out Raw Interrupt Status bit.
        RIS_PTMOUT: 0 = enum RIS_PTMOUT {
            /// Time out during power up has not occurred
            PTMOUT_0 = 0b0,
            /// Time out during power up has occurred
            PTMOUT_1 = 0b1,
        }
        /// UUPS Power Request Ignore Masked Interrupt Status and Clear bit.
        RIS_PREQIG: 1 = enum RIS_PREQIG {
            /// No USS_PWRREQ signal has been ignored
            PREQIG_0 = 0b0,
            /// USS_PWRREQ signal has been ignored
            PREQIG_1 = 0b1,
        }
        /// USS has been interrupted by debug mode.
        RIS_STPBYDB: 2 = enum RIS_STPBYDB {
            /// USS has not been interrupted by debug halt mode.
            STPBYDB_0 = 0b0,
            /// USS has been interrupted by debug halt mode.
            STPBYDB_1 = 0b1,
        }
    }
    /// Interrupt Mask Register
    rw IMSC @ 0x06: u16 = 0_0 {
        /// UUPS Power Up Time Out Interrupt Mask bit.
        IMSC_PTMOUT: 0 = enum IMSC_PTMOUT {
            /// UUPS Power Up Time Out Interrupt is disabled.
            PTMOUT_0 = 0b0,
            /// UUPS Power Up Time Out Interrupt is enabled.
            PTMOUT_1 = 0b1,
        }
        /// Power Request Ignore Interrupt Mask bit.
        IMSC_PREQIG: 1 = enum IMSC_PREQIG {
            /// Power Request Ignore Interrupt is disabled.
            PREQIG_0 = 0b0,
            /// Power Request Ignore Interrupt is enabled.
            PREQIG_1 = 0b1,
        }
        /// USS has been interrupted by debug mode Interrupt Mask bit.
        IMSC_STPBYDB: 2 = enum IMSC_STPBYDB {
            /// STPBYDB Interrupt is disabled
            STPBYDB_0 = 0b0,
            /// STPBYDB Interrupt is enabled
            STPBYDB_1 = 0b1,
        }
    }
    /// Interrupt Clear Register.
    rw ICR @ 0x08: u16 = 0_0 {
        /// Power Request Ignored Interrupt Clear bit.
        ICR_PREQIG: 1 = struct ICR_PREQIG(bool);
        /// UUPS Power Up Time Out Interrupt Clear bit.
        ICR_PTMOUT: 0 = struct ICR_PTMOUT(bool);
        /// USS has been interrupted by debug mode Interrupt Clear bit.
        ICR_STPBYDB: 2 = struct ICR_STPBYDB(bool);
    }
    /// Interrupt Flag Set Register.
    rw ISR @ 0x0a: u16 = 0_0 {
        /// Power Request Ignored Interrupt Set bit.
        ISR_PREQIG: 1 = struct ISR_PREQIG(bool);
        /// UUPS Power Up Time Out Interrupt Set bit.
        ISR_PTMOUT: 0 = struct ISR_PTMOUT(bool);
        /// USS has been interrupted by debug mode Interrupt Set bit.
        ISR_STPBYDB: 2 = struct ISR_STPBYDB(bool);
    }
    /// UUPS Descriptor Register L.
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
    /// UUPS Descriptor Register H.
    rw DESCHI @ 0x0e: u16 = 0_0 {
        /// UUPS Descriptor Register H.
        DESCHI: 0..15 = struct DESCHIField(u16);
    }
    /// UUPS Control
    rw CTL @ 0x10: u16 = 0_0 {
        /// Turn on USS Power and  PLL
        USSPWRUP: 8 = enum USSPWRUP {
            /// No action
            USSPWRUP_0 = 0b0,
            /// Power up the USS module and generate the PSQ_START to the ASQ if CTL.ASQEN = 1. Note: This bit becomes invalid in debug mode.
            USSPWRUP_1 = 0b1,
        }
        /// Power Ready Output Event Select
        ASQEN: 11 = enum ASQEN {
            /// Do not generate the PSQ_START signal event to ASQ.
            ASQEN_0 = 0b0,
            /// Generate the PSQ_START signal event to the ASQ.
            ASQEN_1 = 0b1,
        }
        /// USS LDO is ready
        LDORDY: 0 = enum LDORDY {
            /// USS LDO is powered down or in transition mode
            LDORDY_0 = 0b0,
            /// USS LDO is powered on
            LDORDY_1 = 0b1,
        }
        /// Reserved_2
        LBHDEL: 12..13 = enum LBHDEL {
            /// no additional delay
            LBHDEL_0 = 0b00,
            /// additional hold off delay of ~100us (512 REFCLKs)
            LBHDEL_1 = 0b01,
            /// additional hold off delay of ~200us (1024 REFCLKs)
            LBHDEL_2 = 0b10,
            /// additional hold off delay of ~300us (1536 REFCLKs)
            LBHDEL_3 = 0b11,
        }
        /// USS Power Down
        USSPWRDN: 14 = enum USSPWRDN {
            /// No action
            USSPWRDN_0 = 0b0,
            /// Stop the current measurement and power off the USS module.
            USSPWRDN_1 = 0b1,
        }
        /// USS Power Up trigger source select.
        USSPWRUPSEL: 9..10 = enum USSPWRUPSEL {
            /// CTL.USSPWRUP bit
            USSPWRUPSEL_0 = 0b00,
            /// Ext. trigger (see the device specific datasheet)
            USSPWRUPSEL_1 = 0b01,
            /// Ext. trigger (see the device-specific data sheet)
            USSPWRUPSEL_2 = 0b10,
            /// Ext. trigger (see the device-specific data sheet)
            USSPWRUPSEL_3 = 0b11,
        }
        /// USS Measurement Stop
        USSSTOP: 15 = enum USSSTOP {
            /// No action
            USSSTOP_0 = 0b0,
            /// Stop the current measurement.
            USSSTOP_1 = 0b1,
        }
        /// USS Power Up trigger source select.
        UPSTATE: 1..2 = enum UPSTATE {
            /// USS is in OFF mode
            UPSTATE_0 = 0b00,
            /// USS is in STANDBY mode
            UPSTATE_1 = 0b01,
            /// USS power mode is in transition.
            UPSTATE_2 = 0b10,
            /// USS is in READY mode
            UPSTATE_3 = 0b11,
        }
        /// USS Busy bit.
        USS_BUSY: 3 = enum USS_BUSY {
            /// The USS module is not busy.
            USS_BUSY_0 = 0b0,
            /// The USS module is busy.
            USS_BUSY_1 = 0b1,
        }
        /// Software reset
        USSSWRST: 7 = enum USSSWRST {
            /// Disabled. USS (and sub modules) reset released for operation
            USSSWRST_0 = 0b0,
            /// Enabled. USS (and sub modules) logic held in reset state
            USSSWRST_1 = 0b1,
        }
    }
}
