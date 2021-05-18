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
        /// SVS high side enable
        SVSHE: 6 = struct SVSHE(bool);
    }
    /// PMM Control 1
    rw PMMCTL1 @ 0x02: u16 = 0_0 {
        /// PMM Control 1
        PMMCTL1: 0..15 = struct PMMCTL1Field(u16);
    }
    /// PMM Control 2
    rw PMMCTL2 @ 0x04: u16 = 0_0 {
        /// Internal Reference Enable
        INTREFEN: 0 = struct INTREFEN(bool);
        /// External Reference output Enable
        EXTREFEN: 1 = struct EXTREFEN(bool);
        /// Temperature Sensor Enable
        TSENSOREN: 3 = struct TSENSOREN(bool);
        /// REF Reference generator active
        REFGENACT: 8 = struct REFGENACT(bool);
        /// REF Reference bandgap active
        REFBGACT: 9 = struct REFBGACT(bool);
        /// REF Bandgap mode
        BGMODE: 11 = struct BGMODE(bool);
        /// REF Reference generator ready
        REFGENRDY: 12 = struct REFGENRDY(bool);
        /// REF Reference bandgap ready
        REFBGRDY: 13 = struct REFBGRDY(bool);
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
    /// PMM Interrupt Enable
    rw PMMIE @ 0x0e: u16 = 0_0 {
        /// PMM Interrupt Enable
        PMMIE: 0..15 = struct PMMIEField(u16);
    }
    /// PMM Power Mode 5 Control Register 0
    rw PM5CTL0 @ 0x10: u16 = 0_0 {
        /// Lock I/O pin configuration upon entry/exit to/from LPM5
        LOCKLPM5: 0 = struct LOCKLPM5(bool);
        /// LPMx.5 switch dis/connected
        LPM5SW: 4 = struct LPM5SW(bool);
        /// Manual mode for LPM3.5 switch
        LPM5SM: 5 = struct LPM5SM(bool);
    }
}
