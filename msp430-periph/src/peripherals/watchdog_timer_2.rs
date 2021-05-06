//! Watchdog Timer

utils::periph! {
    /// Watchdog Timer
    WatchdogTimer;
    /// Watchdog Timer Control
    rw WDTCTL @ 0x00: u16 = 0_0 {
        /// WDT - Timer Interval Select 0
        WDTIS: 0..2 = enum WDTIS {
            /// WDT - Timer Interval Select: /2G
            WDTIS_0 = 0b000,
            /// WDT - Timer Interval Select: /128M
            WDTIS_1 = 0b001,
            /// WDT - Timer Interval Select: /8192k
            WDTIS_2 = 0b010,
            /// WDT - Timer Interval Select: /512k
            WDTIS_3 = 0b011,
            /// WDT - Timer Interval Select: /32k
            WDTIS_4 = 0b100,
            /// WDT - Timer Interval Select: /8192
            WDTIS_5 = 0b101,
            /// WDT - Timer Interval Select: /512
            WDTIS_6 = 0b110,
            /// WDT - Timer Interval Select: /64
            WDTIS_7 = 0b111,
        }
        /// WDT - Timer Clear
        WDTCNTCL: 3 = struct WDTCNTCL(bool);
        /// WDT - Timer Mode Select
        WDTTMSEL: 4 = struct WDTTMSEL(bool);
        /// WDT - Timer Clock Source Select 0
        WDTSSEL: 5..6 = enum WDTSSEL {
            /// WDT - Timer Clock Source Select: SMCLK
            WDTSSEL_0 = 0b00,
            /// WDT - Timer Clock Source Select: ACLK
            WDTSSEL_1 = 0b01,
            /// WDT - Timer Clock Source Select: VLO_CLK
            WDTSSEL_2 = 0b10,
            /// WDT - Timer Clock Source Select: reserved
            WDTSSEL_3 = 0b11,
        }
        /// WDT - Timer hold
        WDTHOLD: 7 = struct WDTHOLD(bool);
    }
}
