//! WDT_A

utils::periph! {
    /// WDT_A
    WDT_A;
    /// Watchdog Timer Control Register
    rw WDTCTL @ 0x00: u16 = 0_0 {
        /// Watchdog timer interval select
        WDTIS: 0..2 = enum WDTIS {
            /// Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)
            _2G = 0b000,
            /// Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)
            _128M = 0b001,
            /// Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)
            _8192K = 0b010,
            /// Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)
            _512K = 0b011,
            /// Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)
            _32K = 0b100,
            /// Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)
            _8192 = 0b101,
            /// Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)
            _512 = 0b110,
            /// Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)
            _64 = 0b111,
        }
        /// Watchdog timer counter clear
        WDTCNTCL: 3 = enum WDTCNTCL {
            /// No action
            WDTCNTCL_0 = 0b0,
            /// WDTCNT = 0000h
            WDTCNTCL_1 = 0b1,
        }
        /// Watchdog timer mode select
        WDTTMSEL: 4 = enum WDTTMSEL {
            /// Watchdog mode
            WDTTMSEL_0 = 0b0,
            /// Interval timer mode
            WDTTMSEL_1 = 0b1,
        }
        /// Watchdog timer clock source select
        WDTSSEL: 5..6 = enum WDTSSEL {
            /// SMCLK
            SMCLK = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// VLOCLK
            VLOCLK = 0b10,
            /// BCLK
            BCLK = 0b11,
        }
        /// Watchdog timer hold
        WDTHOLD: 7 = enum WDTHOLD {
            /// Watchdog timer is not stopped
            UNHOLD = 0b0,
            /// Watchdog timer is stopped
            HOLD = 0b1,
        }
        /// Watchdog timer password
        WDTPW: 8..15 = struct WDTPW(u16);
    }
}
