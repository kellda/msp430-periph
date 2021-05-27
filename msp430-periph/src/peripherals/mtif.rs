//! MTIF

utils::periph! {
    /// MTIF
    MTIF;
    /// Pulse Generator Configuration Register
    rw PGCNF @ 0x00: u16 = 0_0 {
        /// PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords.
        PGCNF_PGPW: 8..15 = struct PGCNF_PGPW(u16);
        /// PG pulse grid frequency select. This value determines at which time grid pulses are generated. The pulse generator frame frequency is an 1/256th of this (PGEN has to be one to perform a change).
        PGFS: 4..6 = enum PGFS {
            /// Pulse grid frequency is set to 8 Hz (nominal)
            PGFS_0 = 0b000,
            /// Pulse grid frequency is set to 16 Hz (nominal)
            PGFS_1 = 0b001,
            /// Pulse grid frequency is set to 32 Hz (nominal)
            PGFS_2 = 0b010,
            /// Pulse grid frequency is set to 64 Hz (nominal)
            PGFS_3 = 0b011,
            /// Pulse grid frequency is set to 128 Hz (nominal)
            PGFS_4 = 0b100,
            /// Pulse grid frequency is set to 256 Hz (nominal)
            PGFS_5 = 0b101,
            /// Pulse grid frequency is set to 512 Hz (nominal)
            PGFS_6 = 0b110,
            /// Pulse grid frequency is set to 1024 Hz (nominal) default
            PGFS_7 = 0b111,
        }
        /// PG pulse counter clear. This bit allows to clear the pulse generator (PGEN has to be set to one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. PCEN =0 and LFXTOFF=1 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back
        PGCLR: 2 = struct PGCLR(bool);
        /// PG sub module enable. This bit enables the PG sub module when set to one
        PGEN: 0 = struct PGEN(bool);
    }
    /// Pulse Generator Value Register
    rw PGKVAL @ 0x02: u16 = 0_0 {
        /// Pulse Count Number. This register value determines how many pulses are generated withing 256 periods of the pulse grid frequency(with password protection as in PGCNF). PGEN has to be one to perform a change.
        KVAL: 0..6 = struct KVAL(u16);
        /// PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords.
        PGKVAL_PGPW: 8..15 = struct PGKVAL_PGPW(u16);
    }
    /// Pulse Generator Control Register
    rw PGCTL @ 0x04: u16 = 0_0 {
        /// Pulse K-Count Update Request (with password protection as in PGCNF). The update of KVAL occurs during the frequency grid slot 0xff (e.g. in the last 4ms of a second with a pulse grid frequency of 256Hz)
        PKUR: 0 = struct PKUR(bool);
        /// PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords.
        PGCTL_PGPW: 8..15 = struct PGCTL_PGPW(u16);
        /// Pulse Grid Frequency Update Request (with password protection as in PGCNF). The update of PGFS occurs during the frequency grid slot 0xff (e.g. in the last 4ms of an second with an pulse grid frequency of 256Hz)
        PGUR: 1 = struct PGUR(bool);
    }
    /// Pulse Generator Status Register
    rw PGSR @ 0x06: u16 = 0_0 {
        /// Pulse K-Count Update Acknowledge. This acknowledges a PCUR directly after the K-values has been updated.
        PKUA: 0 = struct PKUA(bool);
        /// Pulse Grid Frequency Update Acknowledge. This acknowledges a PGUR directly after the PGFS has been updated.
        PGUA: 1 = struct PGUA(bool);
    }
    /// Pulse Counter Configuration Register
    rw PCCNF @ 0x08: u16 = 0_0 {
        /// Pulse counter password. Always reads as 0x96. Must be written as 0xA5 for register changes to be effective. This password differs from the pin configuration and pulse generator passwords
        PCPW: 8..15 = struct PCPW(u16);
        /// PC sub module enable. This bit enables the PC sub module when set to one
        PCEN: 0 = struct PCEN(bool);
        /// Pulse counter clear. This bit allows to clear the pulse counter when set to one (PCEN has to be one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. LFXTOFF=1 and PCEN=0 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back
        PCCLR: 2 = struct PCCLR(bool);
    }
    /// Pulse Counter Value Register
    rw PCR @ 0x0a: u16 = 0_0 {
        /// Pulse Counter Value Register
        PCR: 0..15 = struct PCRField(u16);
    }
    /// Pulse Counter Control Register
    rw PCCTL @ 0x0c: u16 = 0_0 {
        /// Pulse Counter Read Request. Set this to request an update of PCR read register from the actual counter.
        PCRR: 0 = struct PCRR(bool);
    }
    /// Pulse Counter Status Register
    rw PCSR @ 0x0e: u16 = 0_0 {
        /// Pulse counter overflow. This bit indicates an overflow of the pulse counter when its value changes since the last read request procedure. It is basically the 17th bit of the counter
        PCOFL: 1 = struct PCOFL(bool);
        /// Pulse counter read acknowledge. This acknowledges the update of the PCR register as response to the PCRR read request. Note!: A read request is being latched. LFXTOFF=1 and PCEN=0 will prevent that.The read will then be performed and acknowledged after the clock is reenabled.
        PCRA: 0 = struct PCRA(bool);
    }
    /// Measurement Test Port Control Register
    rw TPCTL @ 0x10: u16 = 0_0 {
        /// Test port password. Always reads as 0x0F. Must be written as 0xC3 for register changes to be effective.This password differs from the pulse generator and pulse counter passwords
        TPPW: 8..15 = struct TPPW(u16);
        /// Test port input select for pulse counter. This value determines the source for the pulse counter.
        TPISEL: 2 = enum TPISEL {
            /// The pulse generator is used as input
            TPISEL_0 = 0b0,
            /// The test port input terminal is selected as input
            TPISEL_1 = 0b1,
        }
        /// Test port input enable. This bit allows to enable the test input port
        TPIE: 1 = struct TPIE(bool);
        /// Test port output enable. This bit allows to enable the test pulse output when set to one
        TPOE: 0 = struct TPOE(bool);
        /// Test port terminal enable activation. This value determines if the testport output is enabled solely by software or by software and hardware.
        ACTIVATE: 3 = enum ACTIVATE {
            /// The test port output is enabled solely by TPOE (enabled if TPOE=1)
            ACTIVATE_0 = 0b0,
            /// The testport output requires both TPOE to be high and the MTPE pin to be high to be enabled
            ACTIVATE_1 = 0b1,
        }
    }
}
