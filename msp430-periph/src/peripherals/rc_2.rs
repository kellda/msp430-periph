//! RC  RAM Control Module

utils::periph! {
    /// RC  RAM Control Module
    RC;
    /// Ram Controller Control Register
    rw RCCTL0 @ 0x00: u16 = 0_0 {
        /// RAM Controller RAM Sector 0 Off
        RCRS0OFF: 0 = struct RCRS0OFF(bool);
        /// RAM Controller RAM Sector 1 Off
        RCRS1OFF: 1 = struct RCRS1OFF(bool);
        /// RAM Controller RAM Sector 2 Off
        RCRS2OFF: 2 = struct RCRS2OFF(bool);
        /// RAM Controller RAM Sector 3 Off
        RCRS3OFF: 3 = struct RCRS3OFF(bool);
        /// RAM Controller RAM Sector 7 (USB) Off
        RCRS7OFF: 7 = struct RCRS7OFF(bool);
    }
}
