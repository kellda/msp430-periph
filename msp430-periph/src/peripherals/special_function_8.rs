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
        /// NMIIE
        NMIIE: 4 = struct NMIIE(bool);
        /// ACCVIE
        ACCVIE: 5 = struct ACCVIE(bool);
    }
    /// Interrupt Flag 1
    rw IFG1 @ 0x02: u8 = 0_0 {
        /// WDTIFG
        WDTIFG: 0 = struct WDTIFG(bool);
        /// OFIFG
        OFIFG: 1 = struct OFIFG(bool);
        /// NMIIFG
        NMIIFG: 4 = struct NMIIFG(bool);
    }
    /// Interrupt Enable 2
    rw IE2 @ 0x01: u8 = 0_0 {
        /// URXIE0
        URXIE0: 0 = struct URXIE0(bool);
        /// UTXIE0
        UTXIE0: 1 = struct UTXIE0(bool);
    }
    /// Interrupt Flag 2
    rw IFG2 @ 0x03: u8 = 0_0 {
        /// URXIFG0
        URXIFG0: 0 = struct URXIFG0(bool);
        /// UTXIFG0
        UTXIFG0: 1 = struct UTXIFG0(bool);
    }
    /// Module Enable 2
    rw ME2 @ 0x05: u8 = 0_0 {
        /// URXE0
        URXE0: 0 = struct URXE0(bool);
        /// UTXE0
        UTXE0: 1 = struct UTXE0(bool);
    }
}
