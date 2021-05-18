//! Basic Timer / RTC

utils::periph! {
    /// Basic Timer / RTC
    BasicTimerRTC;
    /// Basic Timer Control
    rw BTCTL @ 0x00: u8 = 0_0 {
        /// BTIP0
        BTIP0: 0 = struct BTIP0(bool);
        /// BTIP1
        BTIP1: 1 = struct BTIP1(bool);
        /// BTIP2
        BTIP2: 2 = struct BTIP2(bool);
        /// fCLK2 = ACLK:256
        BTDIV: 5 = struct BTDIV(bool);
        /// BT1 is held if this bit is set
        BTHOLD: 6 = struct BTHOLD(bool);
        /// fBT = fMCLK (main clock)
        BTSSEL: 7 = struct BTSSEL(bool);
    }
    /// Real Time Clock Control
    rw RTCCTL @ 0x01: u8 = 0_0 {
        /// RTC Event Flag
        RTCFG: 0 = struct RTCFG(bool);
        /// RTC Interrupt Enable
        RTCIE: 1 = struct RTCIE(bool);
        /// RTC Time Event 1
        RTCTEV: 2..3 = enum RTCTEV {
            /// RTC Time Event: 0
            RTCTEV_0 = 0b00,
            /// RTC Time Event: 1
            RTCTEV_1 = 0b01,
            /// RTC Time Event: 2
            RTCTEV_2 = 0b10,
            /// RTC Time Event: 3
            RTCTEV_3 = 0b11,
        }
        /// RTC Mode 1
        RTCMODE: 4..5 = enum RTCMODE {
            /// RTC Mode: 0
            RTCMODE_0 = 0b00,
            /// RTC Mode: 1
            RTCMODE_1 = 0b01,
            /// RTC Mode: 2
            RTCMODE_2 = 0b10,
            /// RTC Mode: 3
            RTCMODE_3 = 0b11,
        }
        /// RTC Hold
        RTCHOLD: 6 = struct RTCHOLD(bool);
        /// RTC BCD Select
        RTCBCD: 7 = struct RTCBCD(bool);
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
        RTCDAY: 0..7 = struct RTCDAYField(u8);
    }
    /// Real Time Clock Month
    rw RTCMON @ 0x0d: u8 = 0_0 {
        /// Real Time Clock Month
        RTCMON: 0..7 = struct RTCMONField(u8);
    }
    /// Real Time Clock Year (Low Byte)
    rw RTCYEARL @ 0x0e: u8 = 0_0 {
        /// Real Time Clock Year (Low Byte)
        RTCYEARL: 0..7 = struct RTCYEARLField(u8);
    }
    /// Real Time Clock Year (High Byte)
    rw RTCYEARH @ 0x0f: u8 = 0_0 {
        /// Real Time Clock Year (High Byte)
        RTCYEARH: 0..7 = struct RTCYEARHField(u8);
    }
    /// Basic/Real Timer Control
    rw RTCTL @ 0x00: u16 = 0_0 {
        /// Basic/Real Timer Control
        RTCTL: 0..15 = struct RTCTLField(u16);
    }
    /// Real Time Clock Time 0
    rw RTCTIM0 @ 0x02: u16 = 0_0 {
        /// Real Time Clock Time 0
        RTCTIM0: 0..15 = struct RTCTIM0Field(u16);
    }
    /// Real Time Clock Time 1
    rw RTCTIM1 @ 0x04: u16 = 0_0 {
        /// Real Time Clock Time 1
        RTCTIM1: 0..15 = struct RTCTIM1Field(u16);
    }
    /// Basic Timer Count 1/2
    rw BTCNT12 @ 0x06: u16 = 0_0 {
        /// Basic Timer Count 1/2
        BTCNT12: 0..15 = struct BTCNT12Field(u16);
    }
    /// Real Time Clock Date
    rw RTCDATE @ 0x0c: u16 = 0_0 {
        /// Real Time Clock Date
        RTCDATE: 0..15 = struct RTCDATEField(u16);
    }
    /// Real Time Clock Year
    rw RTCYEAR @ 0x0e: u16 = 0_0 {
        /// Real Time Clock Year
        RTCYEAR: 0..15 = struct RTCYEARField(u16);
    }
}
