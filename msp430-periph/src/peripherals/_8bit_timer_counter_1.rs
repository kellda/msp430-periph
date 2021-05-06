//! 8BIT TIMER/COUNTER

utils::periph! {
    /// 8BIT TIMER/COUNTER
    _8BITTIMERCOUNTER;
    /// Timer/Counter Control
    rw TCCTL @ 0x00: u8 = 0_0 {
        /// TCRXD
        TCRXD: 0 = struct TCRXD(bool);
        /// TCTXD
        TCTXD: 1 = struct TCTXD(bool);
        /// TCRXACT
        TCRXACT: 2 = struct TCRXACT(bool);
        /// TCENCNT
        TCENCNT: 3 = struct TCENCNT(bool);
        /// TCTXE
        TCTXE: 4 = struct TCTXE(bool);
        /// TCISCTL
        TCISCTL: 5 = struct TCISCTL(bool);
        /// TCSSEL0
        TCSSEL0: 6 = struct TCSSEL0(bool);
        /// TCSSEL1
        TCSSEL1: 7 = struct TCSSEL1(bool);
    }
    /// Timer/Counter Preload
    rw TCPLD @ 0x01: u8 = 0_0 {
        /// Timer/Counter Preload
        TCPLD: 0..7 = struct TCPLDField(u8);
    }
    /// Timer/Counter Data
    rw TCDAT @ 0x02: u8 = 0_0 {
        /// Timer/Counter Data
        TCDAT: 0..7 = struct TCDATField(u8);
    }
}
