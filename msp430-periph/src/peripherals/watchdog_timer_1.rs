//! Watchdog Timer

utils::periph! {
    /// Watchdog Timer
    WatchdogTimer;
    /// Watchdog Timer Control
    rw CTL @ 0x00: u16 = 0_0 {
        /// WDTIS0
        IS0: 0 = struct IS0(bool);
        /// WDTIS1
        IS1: 1 = struct IS1(bool);
        /// WDTSSEL
        SSEL: 2 = struct SSEL(bool);
        /// WDTCNTCL
        CNTCL: 3 = struct CNTCL(bool);
        /// WDTTMSEL
        TMSEL: 4 = struct TMSEL(bool);
        /// WDTNMI
        NMI: 5 = struct NMI(bool);
        /// WDTNMIES
        NMIES: 6 = struct NMIES(bool);
        /// WDTHOLD
        HOLD: 7 = struct HOLD(bool);
    }
}
