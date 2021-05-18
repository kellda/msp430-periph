//! PMM  Power Management System

utils::periph! {
    /// PMM  Power Management System
    PMM;
    /// PMM Control 0
    rw PMMCTL0 @ 0x00: u16 = 0_0 {
        /// PMM Software BOR
        PMMSWBOR: 2 = struct PMMSWBOR(bool);
        /// PMM Software POR
        PMMSWPOR: 3 = struct PMMSWPOR(bool);
        /// PMM Turn Regulator off
        PMMREGOFF: 4 = struct PMMREGOFF(bool);
        /// SVS low side enable
        SVSLE: 5 = struct SVSLE(bool);
        /// SVS high side enable
        SVSHE: 6 = struct SVSHE(bool);
    }
    /// PMM Interrupt Flag
    rw PMMIFG @ 0x0a: u16 = 0_0 {
        /// PMM Software BOR interrupt flag
        PMMBORIFG: 8 = struct PMMBORIFG(bool);
        /// PMM RESET pin interrupt flag
        PMMRSTIFG: 9 = struct PMMRSTIFG(bool);
        /// PMM Software POR interrupt flag
        PMMPORIFG: 10 = struct PMMPORIFG(bool);
        /// SVS high side interrupt flag
        SVSLIFG: 12 = struct SVSLIFG(bool);
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
