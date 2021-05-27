//! 8-Bit Timer/Counter

utils::periph! {
    /// 8-Bit Timer/Counter
    TC;
    /// Timer/Counter Control
    rw CTL @ 0x00: u8 = 0_0 {
        /// RXD
        RXD: 0 = struct RXD(bool);
        /// TXD
        TXD: 1 = struct TXD(bool);
        /// RXACT
        RXACT: 2 = struct RXACT(bool);
        /// ENCNT
        ENCNT: 3 = struct ENCNT(bool);
        /// TXE
        TXE: 4 = struct TXE(bool);
        /// ISCTL
        ISCTL: 5 = struct ISCTL(bool);
        /// SSEL0
        SSEL0: 6 = struct SSEL0(bool);
        /// SSEL1
        SSEL1: 7 = struct SSEL1(bool);
    }
    /// Timer/Counter Preload
    rw PLD @ 0x01: u8 = 0_0 {
        /// Timer/Counter Preload
        PLD: 0..7 = struct PLDField(u8);
    }
    /// Timer/Counter Data
    rw DAT @ 0x02: u8 = 0_0 {
        /// Timer/Counter Data
        DAT: 0..7 = struct DATField(u8);
    }
}
