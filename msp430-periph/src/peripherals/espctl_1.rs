//! ESPCTL

utils::periph! {
    /// ESPCTL
    ESPCTLPeriph;
    /// ESP430 Control Register
    rw ESPCTL @ 0x00: u16 = 0_0 {
        /// ESP430 Module enable
        ESPEN: 0 = struct ESPEN(bool);
        /// ESP430 Module suspend
        ESPSUSP: 1 = struct ESPSUSP(bool);
        /// NOT supported by current ESP430 Software
        IREQ: 2 = struct IREQ(bool);
    }
    /// Mailbox Control Register
    rw MBCTL @ 0x02: u16 = 0_0 {
        /// Incoming Mail 0 Interrupt Flag
        IN0IFG: 0 = struct IN0IFG(bool);
        /// Incoming Mail 1 Interrupt Flag
        IN1IFG: 1 = struct IN1IFG(bool);
        /// Outgoing Mail 0 Flag
        OUT0FG: 2 = struct OUT0FG(bool);
        /// Outgoing Mail 1 Flag
        OUT1FG: 3 = struct OUT1FG(bool);
        /// Incoming Mail 0 Interrupt Enable
        IN0IE: 4 = struct IN0IE(bool);
        /// Incoming Mail 1 Interrupt Enable
        IN1IE: 5 = struct IN1IE(bool);
        /// Switch off automatic clear of IN0IFG
        CLR0OFF: 6 = struct CLR0OFF(bool);
        /// Switch off automatic clear of IN1IFG
        CLR1OFF: 7 = struct CLR1OFF(bool);
        /// Outgoing Mail 0 Interrupt Flag
        OUT0IFG: 8 = struct OUT0IFG(bool);
        /// Outgoing Mail 1 Interrupt Flag
        OUT1IFG: 9 = struct OUT1IFG(bool);
        /// Outgoing Mail 0 Interrupt Enable
        OUT0IE: 10 = struct OUT0IE(bool);
        /// Outgoing Mail 1 Interrupt Enable
        OUT1IE: 11 = struct OUT1IE(bool);
    }
    /// Incoming Mailbox 0 Register
    rw MBIN0 @ 0x04: u16 = 0_0 {
        /// Incoming Mailbox 0 Register
        MBIN0: 0..15 = struct MBIN0Field(u16);
    }
    /// Incoming Mailbox 1 Register
    rw MBIN1 @ 0x06: u16 = 0_0 {
        /// Incoming Mailbox 1 Register
        MBIN1: 0..15 = struct MBIN1Field(u16);
    }
    /// Outgoing Mailbox 0 Register
    rw MBOUT0 @ 0x08: u16 = 0_0 {
        /// Outgoing Mailbox 0 Register
        MBOUT0: 0..15 = struct MBOUT0Field(u16);
    }
    /// Outgoing Mailbox 1 Register
    rw MBOUT1 @ 0x0a: u16 = 0_0 {
        /// Outgoing Mailbox 1 Register
        MBOUT1: 0..15 = struct MBOUT1Field(u16);
    }
    /// ESP430 Return Value 0
    rw ESP430_STAT0 @ 0x70: u16 = 0_0 {
        /// ESP430 Return Value 0
        ESP430_STAT0: 0..15 = struct ESP430_STAT0Field(u16);
    }
    /// ESP430 Return Value 1
    rw ESP430_STAT1 @ 0x72: u16 = 0_0 {
        /// ESP430 Return Value 1
        ESP430_STAT1: 0..15 = struct ESP430_STAT1Field(u16);
    }
    /// ESP430 Return Value 2
    rw WAVEFSV1 @ 0x74: u16 = 0_0 {
        /// ESP430 Return Value 2
        WAVEFSV1: 0..15 = struct WAVEFSV1Field(u16);
    }
    /// ESP430 Return Value 3
    rw RET3 @ 0x76: u16 = 0_0 {
        /// ESP430 Return Value 3
        RET3: 0..15 = struct RET3Field(u16);
    }
    /// ESP430 Return Value 4
    rw RET4 @ 0x78: u16 = 0_0 {
        /// ESP430 Return Value 4
        RET4: 0..15 = struct RET4Field(u16);
    }
    /// ESP430 Return Value 5
    rw WAVEFSI1 @ 0x7a: u16 = 0_0 {
        /// ESP430 Return Value 5
        WAVEFSI1: 0..15 = struct WAVEFSI1Field(u16);
    }
    /// ESP430 Return Value 6
    rw WAVEFSI2 @ 0x7c: u16 = 0_0 {
        /// ESP430 Return Value 6
        WAVEFSI2: 0..15 = struct WAVEFSI2Field(u16);
    }
    /// ESP430 Return Value 7
    rw RET7 @ 0x7e: u16 = 0_0 {
        /// ESP430 Return Value 7
        RET7: 0..15 = struct RET7Field(u16);
    }
    /// ESP430 Return Value 8
    rw ACTENERGY1_LO @ 0x80: u16 = 0_0 {
        /// ESP430 Return Value 8
        ACTENERGY1_LO: 0..15 = struct ACTENERGY1_LOField(u16);
    }
    /// ESP430 Return Value 9
    rw ACTENERGY1_HI @ 0x82: u16 = 0_0 {
        /// ESP430 Return Value 9
        ACTENERGY1_HI: 0..15 = struct ACTENERGY1_HIField(u16);
    }
    /// ESP430 Return Value 10
    rw ACTENERGY2_LO @ 0x84: u16 = 0_0 {
        /// ESP430 Return Value 10
        ACTENERGY2_LO: 0..15 = struct ACTENERGY2_LOField(u16);
    }
    /// ESP430 Return Value 11
    rw ACTENERGY2_HI @ 0x86: u16 = 0_0 {
        /// ESP430 Return Value 11
        ACTENERGY2_HI: 0..15 = struct ACTENERGY2_HIField(u16);
    }
    /// ESP430 Return Value 12
    rw REACTENERGY_LO @ 0x88: u16 = 0_0 {
        /// ESP430 Return Value 12
        REACTENERGY_LO: 0..15 = struct REACTENERGY_LOField(u16);
    }
    /// ESP430 Return Value 13
    rw REACTENERGY_HI @ 0x8a: u16 = 0_0 {
        /// ESP430 Return Value 13
        REACTENERGY_HI: 0..15 = struct REACTENERGY_HIField(u16);
    }
    /// ESP430 Return Value 14
    rw APPENERGY_LO @ 0x8c: u16 = 0_0 {
        /// ESP430 Return Value 14
        APPENERGY_LO: 0..15 = struct APPENERGY_LOField(u16);
    }
    /// ESP430 Return Value 15
    rw APPENERGY_HI @ 0x8e: u16 = 0_0 {
        /// ESP430 Return Value 15
        APPENERGY_HI: 0..15 = struct APPENERGY_HIField(u16);
    }
    /// ESP430 Return Value 16
    rw ACTENSPER1_LO @ 0x90: u16 = 0_0 {
        /// ESP430 Return Value 16
        ACTENSPER1_LO: 0..15 = struct ACTENSPER1_LOField(u16);
    }
    /// ESP430 Return Value 17
    rw ACTENSPER1_HI @ 0x92: u16 = 0_0 {
        /// ESP430 Return Value 17
        ACTENSPER1_HI: 0..15 = struct ACTENSPER1_HIField(u16);
    }
    /// ESP430 Return Value 18
    rw ACTENSPER2_LO @ 0x94: u16 = 0_0 {
        /// ESP430 Return Value 18
        ACTENSPER2_LO: 0..15 = struct ACTENSPER2_LOField(u16);
    }
    /// ESP430 Return Value 19
    rw ACTENSPER2_HI @ 0x96: u16 = 0_0 {
        /// ESP430 Return Value 19
        ACTENSPER2_HI: 0..15 = struct ACTENSPER2_HIField(u16);
    }
    /// ESP430 Return Value 20
    rw POWERFCT @ 0x98: u16 = 0_0 {
        /// ESP430 Return Value 20
        POWERFCT: 0..15 = struct POWERFCTField(u16);
    }
    /// ESP430 Return Value 21
    rw CAPIND @ 0x9a: u16 = 0_0 {
        /// ESP430 Return Value 21
        CAPIND: 0..15 = struct CAPINDField(u16);
    }
    /// ESP430 Return Value 22
    rw MAINSPERIOD @ 0x9c: u16 = 0_0 {
        /// ESP430 Return Value 22
        MAINSPERIOD: 0..15 = struct MAINSPERIODField(u16);
    }
    /// ESP430 Return Value 23
    rw V1RMS @ 0x9e: u16 = 0_0 {
        /// ESP430 Return Value 23
        V1RMS: 0..15 = struct V1RMSField(u16);
    }
    /// ESP430 Return Value 24
    rw IRMS_LO @ 0xa0: u16 = 0_0 {
        /// ESP430 Return Value 24
        IRMS_LO: 0..15 = struct IRMS_LOField(u16);
    }
    /// ESP430 Return Value 25
    rw IRMS_HI @ 0xa2: u16 = 0_0 {
        /// ESP430 Return Value 25
        IRMS_HI: 0..15 = struct IRMS_HIField(u16);
    }
    /// ESP430 Return Value 26
    rw VPEAK @ 0xa4: u16 = 0_0 {
        /// ESP430 Return Value 26
        VPEAK: 0..15 = struct VPEAKField(u16);
    }
    /// ESP430 Return Value 27
    rw IPEAK @ 0xa6: u16 = 0_0 {
        /// ESP430 Return Value 27
        IPEAK: 0..15 = struct IPEAKField(u16);
    }
    /// ESP430 Return Value 28
    rw LINECYCLCNT_LO @ 0xa8: u16 = 0_0 {
        /// ESP430 Return Value 28
        LINECYCLCNT_LO: 0..15 = struct LINECYCLCNT_LOField(u16);
    }
    /// ESP430 Return Value 29
    rw LINECYCLCNT_HI @ 0xaa: u16 = 0_0 {
        /// ESP430 Return Value 29
        LINECYCLCNT_HI: 0..15 = struct LINECYCLCNT_HIField(u16);
    }
    /// ESP430 Return Value 30
    rw NMBMEAS_LO @ 0xac: u16 = 0_0 {
        /// ESP430 Return Value 30
        NMBMEAS_LO: 0..15 = struct NMBMEAS_LOField(u16);
    }
    /// ESP430 Return Value 31
    rw NMBMEAS_HI @ 0xae: u16 = 0_0 {
        /// ESP430 Return Value 31
        NMBMEAS_HI: 0..15 = struct NMBMEAS_HIField(u16);
    }
}
