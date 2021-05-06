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
        /// Power On Interrupt Flag
        PORIFG: 2 = struct PORIFG(bool);
        /// Reset Interrupt Flag
        RSTIFG: 3 = struct RSTIFG(bool);
        /// NMI Interrupt Flag
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
    }
}
