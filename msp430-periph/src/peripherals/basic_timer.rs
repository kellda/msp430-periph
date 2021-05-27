//! Basic Timer

utils::periph! {
    /// Basic Timer
    BasicTimer;
    /// Basic Timer Control
    rw CTL @ 0x00: u8 = 0_0 {
        /// BTIP0
        IP0: 0 = struct IP0(bool);
        /// BTIP1
        IP1: 1 = struct IP1(bool);
        /// BTIP2
        IP2: 2 = struct IP2(bool);
        /// BTFRFQ0
        FRFQ0: 3 = struct FRFQ0(bool);
        /// BTFRFQ1
        FRFQ1: 4 = struct FRFQ1(bool);
        /// fCLK2 = ACLK:256
        DIV: 5 = struct DIV(bool);
        /// BT1 is held if this bit is set
        HOLD: 6 = struct HOLD(bool);
        /// fBT = fMCLK (main clock)
        SSEL: 7 = struct SSEL(bool);
    }
    /// Basic Timer Count 1
    rw CNT1 @ 0x06: u8 = 0_0 {
        /// Basic Timer Count 1
        CNT1: 0..7 = struct CNT1Field(u8);
    }
    /// Basic Timer Count 2
    rw CNT2 @ 0x07: u8 = 0_0 {
        /// Basic Timer Count 2
        CNT2: 0..7 = struct CNT2Field(u8);
    }
}
