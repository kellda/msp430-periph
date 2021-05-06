//! HSPLL

utils::periph! {
    /// HSPLL
    HSPLL;
    /// Interrupt Index Register
    r HSPLLIIDX @ 0x00: u16 = 0_0 {
        /// HSPLL Interrupt Vector Value
        IIDX: 1..15 = enum IIDX {
            /// No Interrupt pending
            IIDX_0 = 0b000000000000000,
            /// Interrupt Source: PLLUNLOCK; Interrupt Priority: Highest
            IIDX_1 = 0b000000000000001,
            /// Reserved; Interrupt Priority: Lowest
            IIDX_2 = 0b000000000000010,
        }
    }
    /// Masked Interrupt Status Register.
    r HSPLLMIS @ 0x02: u16 = 0_0 {
        /// HSPLL Unlock Masked Interrupt Status bit
        HSPLLMIS_PLLUNLOCK: 0..0 = enum HSPLLMIS_PLLUNLOCK {
            /// No interrupt pending
            PLLUNLOCK_0 = 0b0,
            /// Interrupt pending
            PLLUNLOCK_1 = 0b1,
        }
    }
    /// Raw Interrupt Status Register
    r HSPLLRIS @ 0x04: u16 = 0_0 {
        /// PLL Unlock Raw Interrupt Status bit.
        HSPLLRIS_PLLUNLOCK: 0..0 = enum HSPLLRIS_PLLUNLOCK {
            /// PLL status has not been changed
            PLLUNLOCK_0 = 0b0,
            /// PLL status has been changed from Lock to Unlock
            PLLUNLOCK_1 = 0b1,
        }
    }
    /// Interrupt Mask Register
    rw HSPLLIMSC @ 0x06: u16 = 0_0 {
        /// PLL Unlock Interrupt Mask bit.
        HSPLLIMSC_PLLUNLOCK: 0..0 = enum HSPLLIMSC_PLLUNLOCK {
            /// PLL Unlock Interrupt is disabled
            PLLUNLOCK_0 = 0b0,
            /// PLL Unlock Interrupt is enabled
            PLLUNLOCK_1 = 0b1,
        }
    }
    /// Interrupt Flag Clear Register.
    rw HSPLLICR @ 0x08: u16 = 0_0 {
        /// PLL Unlock Interrupt Clear bit.
        HSPLLICR_PLLUNLOCK: 0 = struct HSPLLICR_PLLUNLOCK(bool);
    }
    /// Interrupt Flag Set Register.
    rw HSPLLISR @ 0x0a: u16 = 0_0 {
        /// PLL Unlock Interrupt Set bit.
        HSPLLISR_PLLUNLOCK: 0 = struct HSPLLISR_PLLUNLOCK(bool);
    }
    /// HSPLL Descriptor Register L.
    r HSPLLDESCLO @ 0x0c: u16 = 0_0 {
        /// Minor Revision
        MINREV: 0..3 = struct MINREV(u16);
        /// Instance Number within the device.
        INSTNUM: 8..11 = struct INSTNUM(u16);
        /// Major Revision
        MAJREV: 4..7 = struct MAJREV(u16);
        /// Feature Set for the module
        FEATUREVER: 12..15 = struct FEATUREVER(u16);
    }
    /// HSPLL Descriptor Register H.
    rw HSPLLDESCHI @ 0x0e: u16 = 0_0 {
        /// HSPLL Descriptor Register H.
        HSPLLDESCHI: 0..15 = struct HSPLLDESCHIField(u16);
    }
    /// HSPLL Control Register
    rw HSPLLCTL @ 0x10: u16 = 0_0 {
        /// PLL Multiplier
        PLLM: 10..15 = enum PLLM {
            /// PLLM_16
            PLLM_16 = 0b010000,
            /// PLLM_17
            PLLM_17 = 0b010001,
            /// PLLM_18
            PLLM_18 = 0b010010,
            /// PLLM_19
            PLLM_19 = 0b010011,
            /// PLLM_20
            PLLM_20 = 0b010100,
            /// PLLM_21
            PLLM_21 = 0b010101,
            /// PLLM_22
            PLLM_22 = 0b010110,
            /// PLLM_23
            PLLM_23 = 0b010111,
            /// PLLM_24
            PLLM_24 = 0b011000,
            /// PLLM_25
            PLLM_25 = 0b011001,
            /// PLLM_26
            PLLM_26 = 0b011010,
            /// PLLM_27
            PLLM_27 = 0b011011,
            /// PLLM_28
            PLLM_28 = 0b011100,
            /// PLLM_29
            PLLM_29 = 0b011101,
            /// PLLM_30
            PLLM_30 = 0b011110,
            /// PLLM_31
            PLLM_31 = 0b011111,
            /// PLLM_32
            PLLM_32 = 0b100000,
            /// PLLM_33
            PLLM_33 = 0b100001,
            /// PLLM_34
            PLLM_34 = 0b100010,
            /// PLLM_35
            PLLM_35 = 0b100011,
            /// PLLM_36
            PLLM_36 = 0b100100,
            /// PLLM_37
            PLLM_37 = 0b100101,
            /// PLLM_38
            PLLM_38 = 0b100110,
            /// PLLM_39
            PLLM_39 = 0b100111,
        }
        /// PLL Lock Status
        PLL_LOCK: 0..0 = enum PLL_LOCK {
            /// PLL is not running or not locked
            PLL_LOCK_0 = 0b0,
            /// PLL is locked
            PLL_LOCK_1 = 0b1,
        }
        /// PLL Input Frequency Selection.
        PLLINFREQ: 8..8 = enum PLLINFREQ {
            /// Input frequency is equal to 6MHz or lower than 6MHz
            PLLINFREQ_0 = 0b0,
            /// Input frequency is higher than 6MHz
            PLLINFREQ_1 = 0b1,
        }
    }
    /// USSXT Control Register
    rw HSPLLUSSXTLCTL @ 0x12: u16 = 0_0 {
        /// USSXT Enable.
        USSXTEN: 0..0 = enum USSXTEN {
            /// Disable USSXT Oscillator
            USSXTEN_0 = 0b0,
            /// Enable USSXT Oscillator
            USSXTEN_1 = 0b1,
        }
        /// USSXT Buffered Output OFF
        XTOUTOFF: 8..8 = enum XTOUTOFF {
            /// Enable USSXT buffered output
            XTOUTOFF_0 = 0b0,
            /// Disable USSXT buffered output. Default.
            XTOUTOFF_1 = 0b1,
        }
        /// Reserved
        OSCTYPE: 9..9 = enum OSCTYPE {
            /// Gating Counter Length: 4096. It is recommended to use this configuration for crystal resonators.  Note: the counter counts the oscillator clock, so total time can be calculated as Time = 4096 x 1/Oscillator Clock Frequency.
            XTAL = 0b0,
            /// Gating Counter Length: 512. It is recommended to use this configuration for ceramic resonators. Note: the counter counts the oscillator clock, so total time can be calculated as Time = 512x 1/Oscillator Clock Frequency.
            CERAMIC = 0b1,
        }
        /// Oscillator Status Bit.
        OSCSTATE: 1..1 = enum OSCSTATE {
            /// Oscillator is either not enabled or in the middle of start-up transition.
            OSCSTATE_0 = 0b0,
            /// Oscillator has started but is not stable yet. Wait for sufficient time for stabilization.
            OSCSTATE_1 = 0b1,
        }
    }
}
