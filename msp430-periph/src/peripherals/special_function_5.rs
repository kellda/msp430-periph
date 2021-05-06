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
        /// URXIE
        URXIE: 0 = struct URXIE(bool);
        /// UTXIE
        UTXIE: 1 = struct UTXIE(bool);
        /// TPIE
        TPIE: 3 = struct TPIE(bool);
        /// BTIE
        BTIE: 7 = struct BTIE(bool);
    }
    /// Interrupt Flag 2
    rw IFG2 @ 0x03: u8 = 0_0 {
        /// URXIFG
        URXIFG: 0 = struct URXIFG(bool);
        /// UTXIFG
        UTXIFG: 1 = struct UTXIFG(bool);
        /// BTIFG
        BTIFG: 7 = struct BTIFG(bool);
    }
    /// Module Enable 2
    rw ME2 @ 0x05: u8 = 0_0 {
        /// URXE
        URXE: 0 = struct URXE(bool);
        /// USPIE
        USPIE: 0 = struct USPIE(bool);
        /// UTXE
        UTXE: 1 = struct UTXE(bool);
    }
}
