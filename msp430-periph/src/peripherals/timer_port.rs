//! Timer/Port

utils::periph! {
    /// Timer/Port
    TimerPort;
    /// Timer/Port Control
    rw TPCTL @ 0x00: u8 = 0_0 {
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
        TPSSEL0: 6 = struct TPSSEL0(bool);
        /// TPSSEL1
        TPSSEL1: 7 = struct TPSSEL1(bool);
    }
    /// Timer/Port Counter 1
    rw TPCNT1 @ 0x01: u8 = 0_0 {
        /// Timer/Port Counter 1
        TPCNT1: 0..7 = struct TPCNT1Field(u8);
    }
    /// Timer/Port Counter 2
    rw TPCNT2 @ 0x02: u8 = 0_0 {
        /// Timer/Port Counter 2
        TPCNT2: 0..7 = struct TPCNT2Field(u8);
    }
    /// Timer/Port Data
    rw TPD @ 0x03: u8 = 0_0 {
        /// TPD_0
        TPD_0: 0 = struct TPD_0(bool);
        /// TPD_1
        TPD_1: 1 = struct TPD_1(bool);
        /// TPD_2
        TPD_2: 2 = struct TPD_2(bool);
        /// TPD_3
        TPD_3: 3 = struct TPD_3(bool);
        /// TPD_4
        TPD_4: 4 = struct TPD_4(bool);
        /// TPD_5
        TPD_5: 5 = struct TPD_5(bool);
        /// CPON
        CPON: 6 = struct CPON(bool);
        /// B16
        B16: 7 = struct B16(bool);
    }
    /// Timer/Port Enable
    rw TPE @ 0x04: u8 = 0_0 {
        /// TPE_0
        TPE_0: 0 = struct TPE_0(bool);
        /// TPE_1
        TPE_1: 1 = struct TPE_1(bool);
        /// TPE_2
        TPE_2: 2 = struct TPE_2(bool);
        /// TPE_3
        TPE_3: 3 = struct TPE_3(bool);
        /// TPE_4
        TPE_4: 4 = struct TPE_4(bool);
        /// TPE_5
        TPE_5: 5 = struct TPE_5(bool);
        /// TPSSEL2
        TPSSEL2: 6 = struct TPSSEL2(bool);
        /// TPSSEL3
        TPSSEL3: 7 = struct TPSSEL3(bool);
    }
}
