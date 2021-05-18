//! Basic Timer

utils::periph! {
    /// Basic Timer
    BasicTimer;
    /// Basic Timer Control
    rw BTCTL @ 0x00: u8 = 0_0 {
        /// BTIP0
        BTIP0: 0 = struct BTIP0(bool);
        /// BTIP1
        BTIP1: 1 = struct BTIP1(bool);
        /// BTIP2
        BTIP2: 2 = struct BTIP2(bool);
        /// BTFRFQ0
        BTFRFQ0: 3 = struct BTFRFQ0(bool);
        /// BTFRFQ1
        BTFRFQ1: 4 = struct BTFRFQ1(bool);
        /// fCLK2 = ACLK:256
        BTDIV: 5 = struct BTDIV(bool);
        /// BT1 is held if this bit is set
        BTHOLD: 6 = struct BTHOLD(bool);
        /// fBT = fMCLK (main clock)
        BTSSEL: 7 = struct BTSSEL(bool);
    }
    /// Basic Timer Count 1
    rw BTCNT1 @ 0x06: u8 = 0_0 {
        /// Basic Timer Count 1
        BTCNT1: 0..7 = struct BTCNT1Field(u8);
    }
    /// Basic Timer Count 2
    rw BTCNT2 @ 0x07: u8 = 0_0 {
        /// Basic Timer Count 2
        BTCNT2: 0..7 = struct BTCNT2Field(u8);
    }
}
