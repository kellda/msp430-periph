//! RC  RAM Control Module

utils::periph! {
    /// RC  RAM Control Module
    RC;
    /// Ram Controller Control Register
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// RAM Controller RAM Sector 0 Off
        RS0OFF: 0 = struct RS0OFF(bool);
        /// RAM Controller RAM Sector 1 Off
        RS1OFF: 1 = struct RS1OFF(bool);
        /// RAM Controller RAM Sector 2 Off
        RS2OFF: 2 = struct RS2OFF(bool);
        /// RAM Controller RAM Sector 3 Off
        RS3OFF: 3 = struct RS3OFF(bool);
        /// RAM Controller RAM Sector 7 (USB) Off
        RS7OFF: 7 = struct RS7OFF(bool);
    }
}
