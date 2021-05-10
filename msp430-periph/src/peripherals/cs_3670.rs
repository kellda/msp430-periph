//! CS

utils::periph! {
    /// CS
    CS;
    /// Clock System Control 0
    rw CSCTL0 @ 0x00: u16 = 0_0 {
        /// CSKEY password
        CSKEY: 8..15 = struct CSKEY(u16);
    }
    /// Clock System Control 1
    rw CSCTL1 @ 0x02: u16 = 0_0 {
        /// DCO frequency select
        DCOFSEL: 1..3 = enum DCOFSEL {
            /// If DCORSEL = 0: 1 MHz; If DCORSEL = 1: 1 MHz
            DCOFSEL_0 = 0b000,
            /// If DCORSEL = 0: 2.67 MHz; If DCORSEL = 1: 5.33 MHz
            DCOFSEL_1 = 0b001,
            /// If DCORSEL = 0: 3.33 MHz; If DCORSEL = 1: 6.67 MHz
            DCOFSEL_2 = 0b010,
            /// If DCORSEL = 0: 4 MHz; If DCORSEL = 1: 8 MHz
            DCOFSEL_3 = 0b011,
            /// If DCORSEL = 0: 5.33 MHz; If DCORSEL = 1: 16 MHz
            DCOFSEL_4 = 0b100,
            /// If DCORSEL = 0: 6.67 MHz; If DCORSEL = 1: 21 MHz
            DCOFSEL_5 = 0b101,
            /// If DCORSEL = 0: 8 MHz; If DCORSEL = 1: 24 MHz
            DCOFSEL_6 = 0b110,
            /// If DCORSEL = 0: Reserved. Defaults to 8. It is not recommended to use this setting; If DCORSEL = 1: Reserved. Defaults to 24. It is not recommended to use this setting
            DCOFSEL_7 = 0b111,
        }
        /// DCO range select
        DCORSEL: 6 = struct DCORSEL(bool);
    }
    /// Clock System Control 2
    rw CSCTL2 @ 0x04: u16 = 0_0 {
        /// Selects the MCLK source
        SELM: 0..2 = enum SELM {
            /// LFXTCLK when LFXT available, otherwise VLOCLK
            LFXTCLK = 0b000,
            /// VLOCLK
            VLOCLK = 0b001,
            /// LFMODCLK
            LFMODCLK = 0b010,
            /// DCOCLK
            DCOCLK = 0b011,
            /// MODCLK
            MODCLK = 0b100,
            /// HFXTCLK when HFXT available, otherwise DCOCLK
            HFXTCLK = 0b101,
        }
        /// Selects the SMCLK source
        SELS: 4..6 = enum SELS {
            /// LFXTCLK when LFXT available, otherwise VLOCLK.
            LFXTCLK = 0b000,
            /// VLOCLK
            VLOCLK = 0b001,
            /// LFMODCLK
            LFMODCLK = 0b010,
            /// DCOCLK
            DCOCLK = 0b011,
            /// MODCLK
            MODCLK = 0b100,
            /// HFXTCLK when HFXT available, otherwise DCOCLK.
            HFXTCLK = 0b101,
        }
        /// Selects the ACLK source
        SELA: 8..10 = enum SELA {
            /// LFXTCLK when LFXT available, otherwise VLOCLK.
            LFXTCLK = 0b000,
            /// VLOCLK
            VLOCLK = 0b001,
            /// LFMODCLK
            LFMODCLK = 0b010,
        }
    }
    /// Clock System Control 3
    rw CSCTL3 @ 0x06: u16 = 0_0 {
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
        }
        /// SMCLK source divider
        DIVS: 4..6 = enum DIVS {
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
        }
        /// ACLK source divider
        DIVA: 8..10 = enum DIVA {
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
        }
    }
    /// Clock System Control 4
    rw CSCTL4 @ 0x08: u16 = 0_0 {
        /// LFXT off
        LFXTOFF: 0 = enum LFXTOFF {
            /// LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation
            LFXTOFF_0 = 0b0,
            /// LFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK
            LFXTOFF_1 = 0b1,
        }
        /// SMCLK off
        SMCLKOFF: 1 = enum SMCLKOFF {
            /// SMCLK on
            SMCLKOFF_0 = 0b0,
            /// SMCLK off
            SMCLKOFF_1 = 0b1,
        }
        /// VLO off
        VLOOFF: 3 = enum VLOOFF {
            /// VLO is on
            VLOOFF_0 = 0b0,
            /// VLO is off if it is not used as a source for ACLK, MCLK, or SMCLK or if not used as a source for the RTC in LPM3.5
            VLOOFF_1 = 0b1,
        }
        /// LFXT bypass select
        LFXTBYPASS: 4 = enum LFXTBYPASS {
            /// LFXT sourced from external crystal
            LFXTBYPASS_0 = 0b0,
            /// LFXT sourced from external clock signal
            LFXTBYPASS_1 = 0b1,
        }
        /// LFXT oscillator current
        LFXTDRIVE: 6..7 = enum LFXTDRIVE {
            /// Lowest drive strength and current consumption LFXT oscillator
            LFXTDRIVE_0 = 0b00,
            /// Increased drive strength LFXT oscillator
            LFXTDRIVE_1 = 0b01,
            /// Increased drive strength LFXT oscillator
            LFXTDRIVE_2 = 0b10,
            /// Maximum drive strength and maximum current consumption LFXT oscillator
            LFXTDRIVE_3 = 0b11,
        }
        /// Turns off the HFXT oscillator
        HFXTOFF: 8 = enum HFXTOFF {
            /// HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation
            HFXTOFF_0 = 0b0,
            /// HFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK
            HFXTOFF_1 = 0b1,
        }
        /// HFXT frequency selection
        HFFREQ: 10..11 = enum HFFREQ {
            /// 0 to 4 MHz
            HFFREQ_0 = 0b00,
            /// Greater than 4 MHz to 8 MHz
            HFFREQ_1 = 0b01,
            /// Greater than 8 MHz to 16 MHz
            HFFREQ_2 = 0b10,
            /// Greater than 16 MHz to 24 MHz
            HFFREQ_3 = 0b11,
        }
        /// HFXT bypass select
        HFXTBYPASS: 12 = enum HFXTBYPASS {
            /// HFXT sourced from external crystal
            HFXTBYPASS_0 = 0b0,
            /// HFXT sourced from external clock signal
            HFXTBYPASS_1 = 0b1,
        }
        /// HFXT oscillator current
        HFXTDRIVE: 14..15 = enum HFXTDRIVE {
            /// Lowest current consumption
            HFXTDRIVE_0 = 0b00,
            /// Increased drive strength HFXT oscillator
            HFXTDRIVE_1 = 0b01,
            /// Increased drive strength HFXT oscillator
            HFXTDRIVE_2 = 0b10,
            /// Maximum drive strength HFXT oscillator
            HFXTDRIVE_3 = 0b11,
        }
    }
    /// Clock System Control 5
    rw CSCTL5 @ 0x0a: u16 = 0_0 {
        /// LFXT oscillator fault flag
        LFXTOFFG: 0 = enum LFXTOFFG {
            /// No fault condition occurred after the last reset
            LFXTOFFG_0 = 0b0,
            /// LFXT fault; an LFXT fault occurred after the last reset
            LFXTOFFG_1 = 0b1,
        }
        /// HFXT oscillator fault flag
        HFXTOFFG: 1 = enum HFXTOFFG {
            /// No fault condition occurred after the last reset
            HFXTOFFG_0 = 0b0,
            /// HFXT fault; an HFXT fault occurred after the last reset
            HFXTOFFG_1 = 0b1,
        }
        /// Enable start counter for LFXT
        ENSTFCNT1: 6 = enum ENSTFCNT1 {
            /// Startup fault counter disabled. Counter is cleared.
            DISABLE = 0b0,
            /// Startup fault counter enabled
            ENABLE = 0b1,
        }
        /// Enable start counter for HFXT
        ENSTFCNT2: 7 = enum ENSTFCNT2 {
            /// Startup fault counter disabled. Counter is cleared.
            DISABLE = 0b0,
            /// Startup fault counter enabled
            ENABLE = 0b1,
        }
    }
    /// Clock System Control 6
    rw CSCTL6 @ 0x0c: u16 = 0_0 {
        /// ACLK clock request enable
        ACLKREQEN: 0 = enum ACLKREQEN {
            /// ACLK conditional requests are disabled
            DISABLE = 0b0,
            /// ACLK conditional requests are enabled
            ENABLE = 0b1,
        }
        /// MCLK clock request enable
        MCLKREQEN: 1 = enum MCLKREQEN {
            /// MCLK conditional requests are disabled
            DISABLE = 0b0,
            /// MCLK conditional requests are enabled
            ENABLE = 0b1,
        }
        /// SMCLK clock request enable
        SMCLKREQEN: 2 = enum SMCLKREQEN {
            /// SMCLK conditional requests are disabled
            DISABLE = 0b0,
            /// SMCLK conditional requests are enabled
            ENABLE = 0b1,
        }
        /// MODCLK clock request enable
        MODCLKREQEN: 3 = enum MODCLKREQEN {
            /// MODCLK conditional requests are disabled
            DISABLE = 0b0,
            /// MODCLK conditional requests are enabled
            ENABLE = 0b1,
        }
    }
}
