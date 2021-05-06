//! System Clock

utils::periph! {
    /// System Clock
    SystemClock;
    /// System Clock Frequency Integrator 0
    rw SCFI0 @ 0x00: u8 = 0_0 {
        /// FN_2
        FN_2: 2 = struct FN_2(bool);
        /// FN_3
        FN_3: 3 = struct FN_3(bool);
        /// FN_4
        FN_4: 4 = struct FN_4(bool);
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
    /// Crystal Buffer Control *** WRITE-ONLY ***
    rw CBCTL @ 0x03: u8 = 0_0 {
        /// CBE
        CBE: 0 = struct CBE(bool);
        /// CBSEL0
        CBSEL0: 1 = struct CBSEL0(bool);
        /// CBSEL1
        CBSEL1: 2 = struct CBSEL1(bool);
    }
}
