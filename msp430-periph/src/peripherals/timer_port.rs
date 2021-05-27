//! Timer/Port

utils::periph! {
    /// Timer/Port
    TimerPort;
    /// Timer/Port Control
    rw CTL @ 0x00: u8 = 0_0 {
        /// EN1FG
        EN1FG: 0 = struct EN1FG(bool);
        /// RC1FG
        RC1FG: 1 = struct RC1FG(bool);
        /// RC2FG
        RC2FG: 2 = struct RC2FG(bool);
        /// EN1
        EN1: 3 = struct EN1(bool);
        /// ENA
        ENA: 4 = struct ENA(bool);
        /// ENB
        ENB: 5 = struct ENB(bool);
        /// TPSSEL0
        SSEL0: 6 = struct SSEL0(bool);
        /// TPSSEL1
        SSEL1: 7 = struct SSEL1(bool);
    }
    /// Timer/Port Counter 1
    rw CNT1 @ 0x01: u8 = 0_0 {
        /// Timer/Port Counter 1
        CNT1: 0..7 = struct CNT1Field(u8);
    }
    /// Timer/Port Counter 2
    rw CNT2 @ 0x02: u8 = 0_0 {
        /// Timer/Port Counter 2
        CNT2: 0..7 = struct CNT2Field(u8);
    }
    /// Timer/Port Data
    rw D @ 0x03: u8 = 0_0 {
        /// TPD_0
        D_0: 0 = struct D_0(bool);
        /// D_1
        D_1: 1 = struct D_1(bool);
        /// TPD_2
        D_2: 2 = struct D_2(bool);
        /// TPD_3
        D_3: 3 = struct D_3(bool);
        /// TPD_4
        D_4: 4 = struct D_4(bool);
        /// TPD_5
        D_5: 5 = struct D_5(bool);
        /// CPON
        CPON: 6 = struct CPON(bool);
        /// B16
        B16: 7 = struct B16(bool);
    }
    /// Timer/Port Enable
    rw E @ 0x04: u8 = 0_0 {
        /// TPE_0
        E_0: 0 = struct E_0(bool);
        /// TPE_1
        E_1: 1 = struct E_1(bool);
        /// TPE_2
        E_2: 2 = struct E_2(bool);
        /// TPE_3
        E_3: 3 = struct E_3(bool);
        /// TPE_4
        E_4: 4 = struct E_4(bool);
        /// TPE_5
        E_5: 5 = struct E_5(bool);
        /// TPSSEL2
        SSEL2: 6 = struct SSEL2(bool);
        /// TPSSEL3
        SSEL3: 7 = struct SSEL3(bool);
    }
}
