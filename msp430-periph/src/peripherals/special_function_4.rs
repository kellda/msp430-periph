//! Special Function

utils::periph! {
    /// Special Function
    SpecialFunction;
    /// Interrupt Enable 1
    rw IE1 @ 0x00: u8 = 0_0 {
        /// WDTIE
        WDTIE: 0 = struct WDTIE(bool);
        /// OFIE
        OFIE: 1 = struct OFIE(bool);
        /// P0IE_0
        P0IE_0: 2 = struct P0IE_0(bool);
        /// P0IE_1
        P0IE_1: 3 = struct P0IE_1(bool);
    }
    /// Interrupt Flag 1
    rw IFG1 @ 0x02: u8 = 0_0 {
        /// WDTIFG
        WDTIFG: 0 = struct WDTIFG(bool);
        /// OFIFG
        OFIFG: 1 = struct OFIFG(bool);
        /// P0IFG_0
        P0IFG_0: 2 = struct P0IFG_0(bool);
        /// P0IFG_1
        P0IFG_1: 3 = struct P0IFG_1(bool);
        /// NMIIFG
        NMIIFG: 4 = struct NMIIFG(bool);
    }
    /// Interrupt Enable 2
    rw IE2 @ 0x01: u8 = 0_0 {
        /// ADIE
        ADIE: 2 = struct ADIE(bool);
        /// TPIE
        TPIE: 3 = struct TPIE(bool);
        /// BTIE
        BTIE: 7 = struct BTIE(bool);
    }
    /// Interrupt Flag 2
    rw IFG2 @ 0x03: u8 = 0_0 {
        /// ADIFG
        ADIFG: 2 = struct ADIFG(bool);
        /// BTIFG
        BTIFG: 7 = struct BTIFG(bool);
    }
}
