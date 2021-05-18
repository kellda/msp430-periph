//! Capacitive Touch IO

utils::periph! {
    /// Capacitive Touch IO
    Capacitive_Touch_IO;
    /// Capacitive_Touch_IO control register
    rw CAPTIOCTL @ 0x00: u16 = 0_0 {
        /// CapTouchIO Pin Select Bit: 0
        CAPTIOPISEL0: 1 = struct CAPTIOPISEL0(bool);
        /// CapTouchIO Pin Select Bit: 1
        CAPTIOPISEL1: 2 = struct CAPTIOPISEL1(bool);
        /// CapTouchIO Pin Select Bit: 2
        CAPTIOPISEL2: 3 = struct CAPTIOPISEL2(bool);
        /// CapTouchIO Port Select Bit: 0
        CAPTIOPOSEL0: 4 = struct CAPTIOPOSEL0(bool);
        /// CapTouchIO Port Select Bit: 1
        CAPTIOPOSEL1: 5 = struct CAPTIOPOSEL1(bool);
        /// CapTouchIO Port Select Bit: 2
        CAPTIOPOSEL2: 6 = struct CAPTIOPOSEL2(bool);
        /// CapTouchIO Port Select Bit: 3
        CAPTIOPOSEL3: 7 = struct CAPTIOPOSEL3(bool);
        /// CapTouchIO Enable
        CAPTIOEN: 8 = struct CAPTIOEN(bool);
        /// CapTouchIO state
        CAPTIO: 9 = struct CAPTIO(bool);
    }
}
