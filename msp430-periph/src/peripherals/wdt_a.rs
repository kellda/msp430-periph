//! WDT_A

utils::periph! {
    /// WDT_A
    WDT_A;
    /// Watchdog Timer Control Register
    rw CTL @ 0x00: u16 = 0_0 {
        /// Watchdog timer interval select
        IS: 0..2 = enum IS {
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
        CNTCL: 3 = enum CNTCL {
            /// No action
            CNTCL_0 = 0b0,
            /// WDTCNT = 0000h
            CNTCL_1 = 0b1,
        }
        /// Watchdog timer mode select
        TMSEL: 4 = enum TMSEL {
            /// Watchdog mode
            TMSEL_0 = 0b0,
            /// Interval timer mode
            TMSEL_1 = 0b1,
        }
        /// Watchdog timer clock source select
        SSEL: 5..6 = enum SSEL {
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
        HOLD: 7 = enum HOLD {
            /// Watchdog timer is not stopped
            UNHOLD = 0b0,
            /// Watchdog timer is stopped
            HOLD = 0b1,
        }
        /// Watchdog timer password
        PW: 8..15 = struct PW(u16);
    }
}
