//! Port A/B

utils::periph! {
    /// Port A/B
    Port;
    /// Port Input
    rw IN_ @ 0x00: u16 = 0_0 {
        /// P0
        IN_P0: 0 = struct IN_P0(bool);
        /// P1
        IN_P1: 1 = struct IN_P1(bool);
        /// P2
        IN_P2: 2 = struct IN_P2(bool);
        /// P3
        IN_P3: 3 = struct IN_P3(bool);
        /// P4
        IN_P4: 4 = struct IN_P4(bool);
        /// P5
        IN_P5: 5 = struct IN_P5(bool);
        /// P6
        IN_P6: 6 = struct IN_P6(bool);
        /// P7
        IN_P7: 7 = struct IN_P7(bool);
        /// P8
        IN_P8: 8 = struct IN_P8(bool);
        /// P9
        IN_P9: 9 = struct IN_P9(bool);
        /// P10
        IN_P10: 10 = struct IN_P10(bool);
        /// P11
        IN_P11: 11 = struct IN_P11(bool);
        /// P12
        IN_P12: 12 = struct IN_P12(bool);
        /// P13
        IN_P13: 13 = struct IN_P13(bool);
        /// P14
        IN_P14: 14 = struct IN_P14(bool);
        /// P15
        IN_P15: 15 = struct IN_P15(bool);
    }
    /// Port Output
    rw OUT @ 0x02: u16 = 0_0 {
        /// P0
        OUT_P0: 0 = struct OUT_P0(bool);
        /// P1
        OUT_P1: 1 = struct OUT_P1(bool);
        /// P2
        OUT_P2: 2 = struct OUT_P2(bool);
        /// P3
        OUT_P3: 3 = struct OUT_P3(bool);
        /// P4
        OUT_P4: 4 = struct OUT_P4(bool);
        /// P5
        OUT_P5: 5 = struct OUT_P5(bool);
        /// P6
        OUT_P6: 6 = struct OUT_P6(bool);
        /// P7
        OUT_P7: 7 = struct OUT_P7(bool);
        /// P8
        OUT_P8: 8 = struct OUT_P8(bool);
        /// P9
        OUT_P9: 9 = struct OUT_P9(bool);
        /// P10
        OUT_P10: 10 = struct OUT_P10(bool);
        /// P11
        OUT_P11: 11 = struct OUT_P11(bool);
        /// P12
        OUT_P12: 12 = struct OUT_P12(bool);
        /// P13
        OUT_P13: 13 = struct OUT_P13(bool);
        /// P14
        OUT_P14: 14 = struct OUT_P14(bool);
        /// P15
        OUT_P15: 15 = struct OUT_P15(bool);
    }
    /// Port Direction
    rw DIR @ 0x04: u16 = 0_0 {
        /// P0
        DIR_P0: 0 = struct DIR_P0(bool);
        /// P1
        DIR_P1: 1 = struct DIR_P1(bool);
        /// P2
        DIR_P2: 2 = struct DIR_P2(bool);
        /// P3
        DIR_P3: 3 = struct DIR_P3(bool);
        /// P4
        DIR_P4: 4 = struct DIR_P4(bool);
        /// P5
        DIR_P5: 5 = struct DIR_P5(bool);
        /// P6
        DIR_P6: 6 = struct DIR_P6(bool);
        /// P7
        DIR_P7: 7 = struct DIR_P7(bool);
        /// P8
        DIR_P8: 8 = struct DIR_P8(bool);
        /// P9
        DIR_P9: 9 = struct DIR_P9(bool);
        /// P10
        DIR_P10: 10 = struct DIR_P10(bool);
        /// P11
        DIR_P11: 11 = struct DIR_P11(bool);
        /// P12
        DIR_P12: 12 = struct DIR_P12(bool);
        /// P13
        DIR_P13: 13 = struct DIR_P13(bool);
        /// P14
        DIR_P14: 14 = struct DIR_P14(bool);
        /// P15
        DIR_P15: 15 = struct DIR_P15(bool);
    }
    /// Port Selection
    rw SEL @ 0x06: u16 = 0_0 {
        /// P0
        SEL_P0: 0 = struct SEL_P0(bool);
        /// P1
        SEL_P1: 1 = struct SEL_P1(bool);
        /// P2
        SEL_P2: 2 = struct SEL_P2(bool);
        /// P3
        SEL_P3: 3 = struct SEL_P3(bool);
        /// P4
        SEL_P4: 4 = struct SEL_P4(bool);
        /// P5
        SEL_P5: 5 = struct SEL_P5(bool);
        /// P6
        SEL_P6: 6 = struct SEL_P6(bool);
        /// P7
        SEL_P7: 7 = struct SEL_P7(bool);
        /// P8
        SEL_P8: 8 = struct SEL_P8(bool);
        /// P9
        SEL_P9: 9 = struct SEL_P9(bool);
        /// P10
        SEL_P10: 10 = struct SEL_P10(bool);
        /// P11
        SEL_P11: 11 = struct SEL_P11(bool);
        /// P12
        SEL_P12: 12 = struct SEL_P12(bool);
        /// P13
        SEL_P13: 13 = struct SEL_P13(bool);
        /// P14
        SEL_P14: 14 = struct SEL_P14(bool);
        /// P15
        SEL_P15: 15 = struct SEL_P15(bool);
    }
}
