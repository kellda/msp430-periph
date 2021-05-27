//! Basic Timer / RTC

utils::periph! {
    /// Basic Timer / RTC
    BasicTimerRTC;
    /// Basic Timer Control
    rw BTCTL @ 0x00: u8 = 0_0 {
        /// BTIP0
        IP0: 0 = struct IP0(bool);
        /// BTIP1
        IP1: 1 = struct IP1(bool);
        /// BTIP2
        IP2: 2 = struct IP2(bool);
        /// fCLK2 = ACLK:256
        DIV: 5 = struct DIV(bool);
        /// BT1 is held if this bit is set
        BTHOLD: 6 = struct BTHOLD(bool);
        /// fBT = fMCLK (main clock)
        SSEL: 7 = struct SSEL(bool);
    }
    /// Real Time Clock Control
    rw RTCCTL @ 0x01: u8 = 0_0 {
        /// RTC Event Flag
        FG: 0 = struct FG(bool);
        /// RTC Interrupt Enable
        IE: 1 = struct IE(bool);
        /// RTC Time Event 1
        TEV: 2..3 = enum TEV {
            /// RTC Time Event: 0
            TEV_0 = 0b00,
            /// RTC Time Event: 1
            TEV_1 = 0b01,
            /// RTC Time Event: 2
            TEV_2 = 0b10,
            /// RTC Time Event: 3
            TEV_3 = 0b11,
        }
        /// RTC Mode 1
        MODE: 4..5 = enum MODE {
            /// RTC Mode: 0
            MODE_0 = 0b00,
            /// RTC Mode: 1
            MODE_1 = 0b01,
            /// RTC Mode: 2
            MODE_2 = 0b10,
            /// RTC Mode: 3
            MODE_3 = 0b11,
        }
        /// RTC Hold
        RTCHOLD: 6 = struct RTCHOLD(bool);
        /// RTC BCD Select
        BCD: 7 = struct BCD(bool);
    }
    /// Real Time Counter 1
    rw RTCNT1 @ 0x02: u8 = 0_0 {
        /// Real Time Counter 1
        RTCNT1: 0..7 = struct RTCNT1Field(u8);
    }
    /// Real Time Counter 2
    rw RTCNT2 @ 0x03: u8 = 0_0 {
        /// Real Time Counter 2
        RTCNT2: 0..7 = struct RTCNT2Field(u8);
    }
    /// Real Time Counter 3
    rw RTCNT3 @ 0x04: u8 = 0_0 {
        /// Real Time Counter 3
        RTCNT3: 0..7 = struct RTCNT3Field(u8);
    }
    /// Real Time Counter 4
    rw RTCNT4 @ 0x05: u8 = 0_0 {
        /// Real Time Counter 4
        RTCNT4: 0..7 = struct RTCNT4Field(u8);
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
    /// Real Time Clock Day
    rw RTCDAY @ 0x0c: u8 = 0_0 {
        /// Real Time Clock Day
        DAY: 0..7 = struct DAYField(u8);
    }
    /// Real Time Clock Month
    rw RTCMON @ 0x0d: u8 = 0_0 {
        /// Real Time Clock Month
        MON: 0..7 = struct MONField(u8);
    }
    /// Real Time Clock Year (Low Byte)
    rw RTCYEARL @ 0x0e: u8 = 0_0 {
        /// Real Time Clock Year (Low Byte)
        YEARL: 0..7 = struct YEARLField(u8);
    }
    /// Real Time Clock Year (High Byte)
    rw RTCYEARH @ 0x0f: u8 = 0_0 {
        /// Real Time Clock Year (High Byte)
        YEARH: 0..7 = struct YEARHField(u8);
    }
    /// Basic/Real Timer Control
    rw RTCTL @ 0x00: u16 = 0_0 {
        /// Basic/Real Timer Control
        CTL: 0..15 = struct CTLField(u16);
    }
    /// Real Time Clock Time 0
    rw RTCTIM0 @ 0x02: u16 = 0_0 {
        /// Real Time Clock Time 0
        TIM0: 0..15 = struct TIM0Field(u16);
    }
    /// Real Time Clock Time 1
    rw RTCTIM1 @ 0x04: u16 = 0_0 {
        /// Real Time Clock Time 1
        TIM1: 0..15 = struct TIM1Field(u16);
    }
    /// Basic Timer Count 1/2
    rw BTCNT12 @ 0x06: u16 = 0_0 {
        /// Basic Timer Count 1/2
        BTCNT12: 0..15 = struct BTCNT12Field(u16);
    }
    /// Real Time Clock Date
    rw RTCDATE @ 0x0c: u16 = 0_0 {
        /// Real Time Clock Date
        DATE: 0..15 = struct DATEField(u16);
    }
    /// Real Time Clock Year
    rw RTCYEAR @ 0x0e: u16 = 0_0 {
        /// Real Time Clock Year
        YEAR: 0..15 = struct YEARField(u16);
    }
}
