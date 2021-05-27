//! Capacitive Touch IO

utils::periph! {
    /// Capacitive Touch IO
    Capacitive_Touch_IO;
    /// Capacitive_Touch_IO control register
    rw CTL @ 0x00: u16 = 0_0 {
        /// CapTouchIO Pin Select Bit: 0
        PISEL0: 1 = struct PISEL0(bool);
        /// CapTouchIO Pin Select Bit: 1
        PISEL1: 2 = struct PISEL1(bool);
        /// CapTouchIO Pin Select Bit: 2
        PISEL2: 3 = struct PISEL2(bool);
        /// CapTouchIO Port Select Bit: 0
        POSEL0: 4 = struct POSEL0(bool);
        /// CapTouchIO Port Select Bit: 1
        POSEL1: 5 = struct POSEL1(bool);
        /// CapTouchIO Port Select Bit: 2
        POSEL2: 6 = struct POSEL2(bool);
        /// CapTouchIO Port Select Bit: 3
        POSEL3: 7 = struct POSEL3(bool);
        /// CapTouchIO Enable
        EN: 8 = struct EN(bool);
        /// CapTouchIO state
        CAPTIO: 9 = struct CAPTIO(bool);
    }
}
