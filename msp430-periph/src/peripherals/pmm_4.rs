//! PMM  Power Management System

utils::periph! {
    /// PMM  Power Management System
    PMM;
    /// PMM Control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// PMM Software BOR
        SWBOR: 2 = struct SWBOR(bool);
        /// PMM Software POR
        SWPOR: 3 = struct SWPOR(bool);
        /// PMM Turn Regulator off
        REGOFF: 4 = struct REGOFF(bool);
        /// SVS high side enable
        SVSHE: 6 = struct SVSHE(bool);
        /// PMM Low-Power Reset Enable
        LPRST: 7 = struct LPRST(bool);
    }
    /// PMM Interrupt Flag
    rw PMMIFG @ 0x0a: u16 = 0_0 {
        /// PMM Software BOR interrupt flag
        PMMBORIFG: 8 = struct PMMBORIFG(bool);
        /// PMM RESET pin interrupt flag
        PMMRSTIFG: 9 = struct PMMRSTIFG(bool);
        /// PMM Software POR interrupt flag
        PMMPORIFG: 10 = struct PMMPORIFG(bool);
        /// SVS low side interrupt flag
        SVSHIFG: 13 = struct SVSHIFG(bool);
        /// LPM5 indication Flag
        PMMLPM5IFG: 15 = struct PMMLPM5IFG(bool);
    }
    /// PMM Power Mode 5 Control Register 0
    rw PM5CTL0 @ 0x10: u16 = 0_0 {
        /// Lock I/O pin configuration upon entry/exit to/from LPM5
        LOCKLPM5: 0 = struct LOCKLPM5(bool);
    }
}
