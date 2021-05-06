//! System Clock FLLPLUS

utils::periph! {
    /// System Clock FLLPLUS
    SystemClockFLLPLUS;
    /// System Clock Frequency Integrator 0
    rw SCFI0 @ 0x00: u8 = 0_0 {
        /// fDCOCLK =   1.4-12MHz
        FN_2: 2 = struct FN_2(bool);
        /// fDCOCLK =   2.2-17Mhz
        FN_3: 3 = struct FN_3(bool);
        /// fDCOCLK =   3.2-25Mhz
        FN_4: 4 = struct FN_4(bool);
        /// fDCOCLK =     5-40Mhz
        FN_8: 5 = struct FN_8(bool);
        /// Loop Divider Bit : 0
        FLLD: 6..7 = enum FLLD {
            /// Multiply Selected Loop Freq. By 1
            FLLD_1 = 0b00,
            /// Multiply Selected Loop Freq. By 2
            FLLD_2 = 0b01,
            /// Multiply Selected Loop Freq. By 4
            FLLD_4 = 0b10,
            /// Multiply Selected Loop Freq. By 8
            FLLD_8 = 0b11,
        }
    }
    /// System Clock Frequency Integrator 1
    rw SCFI1 @ 0x01: u8 = 0_0 {
        /// System Clock Frequency Integrator 1
        SCFI1: 0..7 = struct SCFI1Field(u8);
    }
    /// System Clock Frequency Control
    rw SCFQCTL @ 0x02: u8 = 0_0 {
        /// System Clock Frequency Control
        SCFQCTL: 0..7 = struct SCFQCTLField(u8);
    }
    /// FLL+ Control 0
    rw FLL_CTL0 @ 0x03: u8 = 0_0 {
        /// DCO Fault Flag
        DCOF: 0 = struct DCOF(bool);
        /// Low Frequency Oscillator Fault Flag
        LFOF: 1 = struct LFOF(bool);
        /// High Frequency Oscillator 1 Fault Flag
        XT1OF: 2 = struct XT1OF(bool);
        /// High Frequency Oscillator 2 Fault Flag
        XT2OF: 3 = struct XT2OF(bool);
        /// XIN/XOUT Cap 0
        OSCCAP: 4..5 = enum OSCCAP {
            /// XIN Cap = XOUT Cap = 0pf
            OSCCAP_0 = 0b00,
            /// XIN Cap = XOUT Cap = 10pf
            OSCCAP_1 = 0b01,
            /// XIN Cap = XOUT Cap = 14pf
            OSCCAP_2 = 0b10,
            /// XIN Cap = XOUT Cap = 18pf
            OSCCAP_3 = 0b11,
        }
        /// 1: Selects high-freq. oscillator
        XTS_FLL: 6 = struct XTS_FLL(bool);
        /// DCO+ Enable
        DCOPLUS: 7 = struct DCOPLUS(bool);
    }
    /// FLL+ Control 1
    rw FLL_CTL1 @ 0x04: u8 = 0_0 {
        /// FLL+ Divide Px.x/ACLK 0
        FLL_DIV: 0..1 = enum FLL_DIV {
            /// FLL+ Divide Px.x/ACLK By 1
            FLL_DIV_1 = 0b00,
            /// FLL+ Divide Px.x/ACLK By 2
            FLL_DIV_2 = 0b01,
            /// FLL+ Divide Px.x/ACLK By 4
            FLL_DIV_4 = 0b10,
            /// FLL+ Divide Px.x/ACLK By 8
            FLL_DIV_8 = 0b11,
        }
        /// Peripheral Module Clock Source (0: DCO
        SELS: 2 = struct SELS(bool);
        /// MCLK Source Select 0
        SELM0: 3 = struct SELM0(bool);
        /// MCLK Source Select 1
        SELM1: 4 = struct SELM1(bool);
        /// High Frequency Oscillator 2 (XT2) disable
        XT2OFF: 5 = struct XT2OFF(bool);
        /// Peripheral Module Clock (SMCLK) disable
        SMCLKOFF: 6 = struct SMCLKOFF(bool);
    }
}
