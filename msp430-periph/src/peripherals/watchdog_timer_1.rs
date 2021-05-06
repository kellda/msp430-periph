//! Watchdog Timer

utils::periph! {
    /// Watchdog Timer
    WatchdogTimer;
    /// Watchdog Timer Control
    rw WDTCTL @ 0x00: u16 = 0_0 {
        /// WDTIS0
        WDTIS0: 0 = struct WDTIS0(bool);
        /// WDTIS1
        WDTIS1: 1 = struct WDTIS1(bool);
        /// WDTSSEL
        WDTSSEL: 2 = struct WDTSSEL(bool);
        /// WDTCNTCL
        WDTCNTCL: 3 = struct WDTCNTCL(bool);
        /// WDTTMSEL
        WDTTMSEL: 4 = struct WDTTMSEL(bool);
        /// WDTNMI
        WDTNMI: 5 = struct WDTNMI(bool);
        /// WDTNMIES
        WDTNMIES: 6 = struct WDTNMIES(bool);
        /// WDTHOLD
        WDTHOLD: 7 = struct WDTHOLD(bool);
    }
}
