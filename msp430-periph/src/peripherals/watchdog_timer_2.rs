//! Watchdog Timer

utils::periph! {
    /// Watchdog Timer
    WatchdogTimer;
    /// Watchdog Timer Control
    rw CTL @ 0x00: u16 = 0_0 {
        /// WDT - Timer Interval Select 0
        IS: 0..2 = enum IS {
            /// WDT - Timer Interval Select: /2G
            IS_0 = 0b000,
            /// WDT - Timer Interval Select: /128M
            IS_1 = 0b001,
            /// WDT - Timer Interval Select: /8192k
            IS_2 = 0b010,
            /// WDT - Timer Interval Select: /512k
            IS_3 = 0b011,
            /// WDT - Timer Interval Select: /32k
            IS_4 = 0b100,
            /// WDT - Timer Interval Select: /8192
            IS_5 = 0b101,
            /// WDT - Timer Interval Select: /512
            IS_6 = 0b110,
            /// WDT - Timer Interval Select: /64
            IS_7 = 0b111,
        }
        /// WDT - Timer Clear
        CNTCL: 3 = struct CNTCL(bool);
        /// WDT - Timer Mode Select
        TMSEL: 4 = struct TMSEL(bool);
        /// WDT - Timer Clock Source Select 0
        SSEL: 5..6 = enum SSEL {
            /// WDT - Timer Clock Source Select: SMCLK
            SSEL_0 = 0b00,
            /// WDT - Timer Clock Source Select: ACLK
            SSEL_1 = 0b01,
            /// WDT - Timer Clock Source Select: VLO_CLK
            SSEL_2 = 0b10,
            /// WDT - Timer Clock Source Select: reserved
            SSEL_3 = 0b11,
        }
        /// WDT - Timer hold
        HOLD: 7 = struct HOLD(bool);
    }
}
