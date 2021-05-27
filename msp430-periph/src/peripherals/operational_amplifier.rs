//! Operational Amplifier

utils::periph! {
    /// Operational Amplifier
    OperationalAmplifier;
    /// OA Control register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// Amplifier mode selection
        M: 0 = struct M(bool);
        /// Rail-to-rail input enable
        RRI: 5 = struct RRI(bool);
        /// Rail-to-rail input ready status
        RRIRDY: 6 = struct RRIRDY(bool);
    }
    /// OA Positive Input Terminal Switches
    rw PSW @ 0x02: u16 = 0_0 {
        /// Positive input terminal switch 0 control
        PSW0: 0 = struct PSW0(bool);
        /// Positive input terminal switch 1 control
        PSW1: 1 = struct PSW1(bool);
        /// Positive input terminal switch 2 control
        PSW2: 2 = struct PSW2(bool);
        /// Positive input terminal switch 3 control
        PSW3: 3 = struct PSW3(bool);
    }
    /// OA Negative Input Terminal Switches
    rw NSW @ 0x04: u16 = 0_0 {
        /// Negative input terminal switch 0 control
        NSW0: 0 = struct NSW0(bool);
        /// Negative input terminal switch 1 control
        NSW1: 1 = struct NSW1(bool);
        /// Negative input terminal switch 2 control
        NSW2: 2 = struct NSW2(bool);
        /// Negative input terminal switch 3 control
        NSW3: 3 = struct NSW3(bool);
        /// Negative input terminal switch 4 control
        NSW4: 4 = struct NSW4(bool);
    }
    /// OA Ground Switches
    rw GSW @ 0x0e: u16 = 0_0 {
        /// Ground switch 0
        GSW0: 0 = struct GSW0(bool);
        /// Ground switch 1
        GSW1: 1 = struct GSW1(bool);
    }
}
