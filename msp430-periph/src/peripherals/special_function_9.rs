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
        /// URXIE0
        URXIE0: 6 = struct URXIE0(bool);
        /// UTXIE0
        UTXIE0: 7 = struct UTXIE0(bool);
    }
    /// Interrupt Flag 1
    rw IFG1 @ 0x02: u8 = 0_0 {
        /// WDTIFG
        WDTIFG: 0 = struct WDTIFG(bool);
        /// OFIFG
        OFIFG: 1 = struct OFIFG(bool);
        /// NMIIFG
        NMIIFG: 4 = struct NMIIFG(bool);
        /// URXIFG0
        URXIFG0: 6 = struct URXIFG0(bool);
        /// UTXIFG0
        UTXIFG0: 7 = struct UTXIFG0(bool);
    }
    /// Module Enable 1
    rw ME1 @ 0x04: u8 = 0_0 {
        /// URXE0
        URXE0: 6 = struct URXE0(bool);
        /// UTXE0
        UTXE0: 7 = struct UTXE0(bool);
    }
    /// Interrupt Enable 2
    rw IE2 @ 0x01: u8 = 0_0 {
        /// URXIE1
        URXIE1: 4 = struct URXIE1(bool);
        /// UTXIE1
        UTXIE1: 5 = struct UTXIE1(bool);
    }
    /// Interrupt Flag 2
    rw IFG2 @ 0x03: u8 = 0_0 {
        /// URXIFG1
        URXIFG1: 4 = struct URXIFG1(bool);
        /// UTXIFG1
        UTXIFG1: 5 = struct UTXIFG1(bool);
    }
    /// Module Enable 2
    rw ME2 @ 0x05: u8 = 0_0 {
        /// URXE1
        URXE1: 4 = struct URXE1(bool);
        /// UTXE1
        UTXE1: 5 = struct UTXE1(bool);
    }
}
