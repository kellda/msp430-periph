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
        /// UCA0RXIE
        UCA0RXIE: 0 = struct UCA0RXIE(bool);
        /// UCA0TXIE
        UCA0TXIE: 1 = struct UCA0TXIE(bool);
        /// UCB0RXIE
        UCB0RXIE: 2 = struct UCB0RXIE(bool);
        /// UCB0TXIE
        UCB0TXIE: 3 = struct UCB0TXIE(bool);
        /// URXIE1
        URXIE1: 4 = struct URXIE1(bool);
        /// UTXIE1
        UTXIE1: 5 = struct UTXIE1(bool);
        /// BTIE
        BTIE: 7 = struct BTIE(bool);
    }
    /// Interrupt Flag 2
    rw IFG2 @ 0x03: u8 = 0_0 {
        /// UCA0RXIFG
        UCA0RXIFG: 0 = struct UCA0RXIFG(bool);
        /// UCA0TXIFG
        UCA0TXIFG: 1 = struct UCA0TXIFG(bool);
        /// UCB0RXIFG
        UCB0RXIFG: 2 = struct UCB0RXIFG(bool);
        /// UCB0TXIFG
        UCB0TXIFG: 3 = struct UCB0TXIFG(bool);
        /// URXIFG1
        URXIFG1: 4 = struct URXIFG1(bool);
        /// UTXIFG1
        UTXIFG1: 5 = struct UTXIFG1(bool);
        /// BTIFG
        BTIFG: 7 = struct BTIFG(bool);
    }
    /// Module Enable 2
    rw ME2 @ 0x05: u8 = 0_0 {
        /// URXE1
        URXE1: 4 = struct URXE1(bool);
        /// UTXE1
        UTXE1: 5 = struct UTXE1(bool);
    }
}
