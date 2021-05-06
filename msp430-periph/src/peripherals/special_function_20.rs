//! Special Function

utils::periph! {
    /// Special Function
    SpecialFunction;
    /// Interrupt Enable 1
    rw IE1 @ 0x00: u8 = 0_0 {
        /// Watchdog Interrupt Enable
        WDTIE: 0 = struct WDTIE(bool);
        /// Osc. Fault  Interrupt Enable
        OFIE: 1 = struct OFIE(bool);
        /// NMI Interrupt Enable
        NMIIE: 4 = struct NMIIE(bool);
        /// Flash Access Violation Interrupt Enable
        ACCVIE: 5 = struct ACCVIE(bool);
    }
    /// Interrupt Flag 1
    rw IFG1 @ 0x02: u8 = 0_0 {
        /// Watchdog Interrupt Flag
        WDTIFG: 0 = struct WDTIFG(bool);
        /// Osc. Fault Interrupt Flag
        OFIFG: 1 = struct OFIFG(bool);
        /// Brown Out Reset Interrupt Flag
        BORIFG: 2 = struct BORIFG(bool);
        /// Reset Interrupt Flag
        RSTIFG: 3 = struct RSTIFG(bool);
        /// NMI Interrupt Flag
        NMIIFG: 4 = struct NMIIFG(bool);
    }
    /// JTAG Disable Register
    rw SYSJTAGDIS @ 0x1fe: u16 = 0_0 {
        /// JTAG Disable Register
        SYSJTAGDIS: 0..15 = struct SYSJTAGDISField(u16);
    }
}
