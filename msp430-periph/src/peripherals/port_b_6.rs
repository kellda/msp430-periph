//! Port B

utils::periph! {
    /// Port B
    PortB;
    /// Port B Input
    rw PBIN @ 0x00: u16 = 0_0 {
        /// PBIN0
        PBIN0: 0 = struct PBIN0(bool);
        /// PBIN1
        PBIN1: 1 = struct PBIN1(bool);
        /// PBIN2
        PBIN2: 2 = struct PBIN2(bool);
        /// PBIN3
        PBIN3: 3 = struct PBIN3(bool);
        /// PBIN4
        PBIN4: 4 = struct PBIN4(bool);
        /// PBIN5
        PBIN5: 5 = struct PBIN5(bool);
        /// PBIN6
        PBIN6: 6 = struct PBIN6(bool);
        /// PBIN7
        PBIN7: 7 = struct PBIN7(bool);
        /// PBIN8
        PBIN8: 8 = struct PBIN8(bool);
        /// PBIN9
        PBIN9: 9 = struct PBIN9(bool);
        /// PBIN10
        PBIN10: 10 = struct PBIN10(bool);
        /// PBIN11
        PBIN11: 11 = struct PBIN11(bool);
        /// PBIN12
        PBIN12: 12 = struct PBIN12(bool);
        /// PBIN13
        PBIN13: 13 = struct PBIN13(bool);
        /// PBIN14
        PBIN14: 14 = struct PBIN14(bool);
        /// PBIN15
        PBIN15: 15 = struct PBIN15(bool);
    }
    /// Port B Output
    rw PBOUT @ 0x02: u16 = 0_0 {
        /// PBOUT0
        PBOUT0: 0 = struct PBOUT0(bool);
        /// PBOUT1
        PBOUT1: 1 = struct PBOUT1(bool);
        /// PBOUT2
        PBOUT2: 2 = struct PBOUT2(bool);
        /// PBOUT3
        PBOUT3: 3 = struct PBOUT3(bool);
        /// PBOUT4
        PBOUT4: 4 = struct PBOUT4(bool);
        /// PBOUT5
        PBOUT5: 5 = struct PBOUT5(bool);
        /// PBOUT6
        PBOUT6: 6 = struct PBOUT6(bool);
        /// PBOUT7
        PBOUT7: 7 = struct PBOUT7(bool);
        /// PBOUT8
        PBOUT8: 8 = struct PBOUT8(bool);
        /// PBOUT9
        PBOUT9: 9 = struct PBOUT9(bool);
        /// PBOUT10
        PBOUT10: 10 = struct PBOUT10(bool);
        /// PBOUT11
        PBOUT11: 11 = struct PBOUT11(bool);
        /// PBOUT12
        PBOUT12: 12 = struct PBOUT12(bool);
        /// PBOUT13
        PBOUT13: 13 = struct PBOUT13(bool);
        /// PBOUT14
        PBOUT14: 14 = struct PBOUT14(bool);
        /// PBOUT15
        PBOUT15: 15 = struct PBOUT15(bool);
    }
    /// Port B Direction
    rw PBDIR @ 0x04: u16 = 0_0 {
        /// PBDIR0
        PBDIR0: 0 = struct PBDIR0(bool);
        /// PBDIR1
        PBDIR1: 1 = struct PBDIR1(bool);
        /// PBDIR2
        PBDIR2: 2 = struct PBDIR2(bool);
        /// PBDIR3
        PBDIR3: 3 = struct PBDIR3(bool);
        /// PBDIR4
        PBDIR4: 4 = struct PBDIR4(bool);
        /// PBDIR5
        PBDIR5: 5 = struct PBDIR5(bool);
        /// PBDIR6
        PBDIR6: 6 = struct PBDIR6(bool);
        /// PBDIR7
        PBDIR7: 7 = struct PBDIR7(bool);
        /// PBDIR8
        PBDIR8: 8 = struct PBDIR8(bool);
        /// PBDIR9
        PBDIR9: 9 = struct PBDIR9(bool);
        /// PBDIR10
        PBDIR10: 10 = struct PBDIR10(bool);
        /// PBDIR11
        PBDIR11: 11 = struct PBDIR11(bool);
        /// PBDIR12
        PBDIR12: 12 = struct PBDIR12(bool);
        /// PBDIR13
        PBDIR13: 13 = struct PBDIR13(bool);
        /// PBDIR14
        PBDIR14: 14 = struct PBDIR14(bool);
        /// PBDIR15
        PBDIR15: 15 = struct PBDIR15(bool);
    }
    /// Port B Resistor Enable
    rw PBREN @ 0x06: u16 = 0_0 {
        /// PBREN0
        PBREN0: 0 = struct PBREN0(bool);
        /// PBREN1
        PBREN1: 1 = struct PBREN1(bool);
        /// PBREN2
        PBREN2: 2 = struct PBREN2(bool);
        /// PBREN3
        PBREN3: 3 = struct PBREN3(bool);
        /// PBREN4
        PBREN4: 4 = struct PBREN4(bool);
        /// PBREN5
        PBREN5: 5 = struct PBREN5(bool);
        /// PBREN6
        PBREN6: 6 = struct PBREN6(bool);
        /// PBREN7
        PBREN7: 7 = struct PBREN7(bool);
        /// PBREN8
        PBREN8: 8 = struct PBREN8(bool);
        /// PBREN9
        PBREN9: 9 = struct PBREN9(bool);
        /// PBREN10
        PBREN10: 10 = struct PBREN10(bool);
        /// PBREN11
        PBREN11: 11 = struct PBREN11(bool);
        /// PBREN12
        PBREN12: 12 = struct PBREN12(bool);
        /// PBREN13
        PBREN13: 13 = struct PBREN13(bool);
        /// PBREN14
        PBREN14: 14 = struct PBREN14(bool);
        /// PBREN15
        PBREN15: 15 = struct PBREN15(bool);
    }
    /// Port B Selection 0
    rw PBSEL0 @ 0x0a: u16 = 0_0 {
        /// PBSEL0_0
        PBSEL0_0: 0 = struct PBSEL0_0(bool);
        /// PBSEL0_1
        PBSEL0_1: 1 = struct PBSEL0_1(bool);
        /// PBSEL0_2
        PBSEL0_2: 2 = struct PBSEL0_2(bool);
        /// PBSEL0_3
        PBSEL0_3: 3 = struct PBSEL0_3(bool);
        /// PBSEL0_4
        PBSEL0_4: 4 = struct PBSEL0_4(bool);
        /// PBSEL0_5
        PBSEL0_5: 5 = struct PBSEL0_5(bool);
        /// PBSEL0_6
        PBSEL0_6: 6 = struct PBSEL0_6(bool);
        /// PBSEL0_7
        PBSEL0_7: 7 = struct PBSEL0_7(bool);
        /// PBSEL0_8
        PBSEL0_8: 8 = struct PBSEL0_8(bool);
        /// PBSEL0_9
        PBSEL0_9: 9 = struct PBSEL0_9(bool);
        /// PBSEL0_10
        PBSEL0_10: 10 = struct PBSEL0_10(bool);
        /// PBSEL0_11
        PBSEL0_11: 11 = struct PBSEL0_11(bool);
        /// PBSEL0_12
        PBSEL0_12: 12 = struct PBSEL0_12(bool);
        /// PBSEL0_13
        PBSEL0_13: 13 = struct PBSEL0_13(bool);
        /// PBSEL0_14
        PBSEL0_14: 14 = struct PBSEL0_14(bool);
        /// PBSEL0_15
        PBSEL0_15: 15 = struct PBSEL0_15(bool);
    }
    /// Port B Selection 1
    rw PBSEL1 @ 0x0c: u16 = 0_0 {
        /// PBSEL1_0
        PBSEL1_0: 0 = struct PBSEL1_0(bool);
        /// PBSEL1_1
        PBSEL1_1: 1 = struct PBSEL1_1(bool);
        /// PBSEL1_2
        PBSEL1_2: 2 = struct PBSEL1_2(bool);
        /// PBSEL1_3
        PBSEL1_3: 3 = struct PBSEL1_3(bool);
        /// PBSEL1_4
        PBSEL1_4: 4 = struct PBSEL1_4(bool);
        /// PBSEL1_5
        PBSEL1_5: 5 = struct PBSEL1_5(bool);
        /// PBSEL1_6
        PBSEL1_6: 6 = struct PBSEL1_6(bool);
        /// PBSEL1_7
        PBSEL1_7: 7 = struct PBSEL1_7(bool);
        /// PBSEL1_8
        PBSEL1_8: 8 = struct PBSEL1_8(bool);
        /// PBSEL1_9
        PBSEL1_9: 9 = struct PBSEL1_9(bool);
        /// PBSEL1_10
        PBSEL1_10: 10 = struct PBSEL1_10(bool);
        /// PBSEL1_11
        PBSEL1_11: 11 = struct PBSEL1_11(bool);
        /// PBSEL1_12
        PBSEL1_12: 12 = struct PBSEL1_12(bool);
        /// PBSEL1_13
        PBSEL1_13: 13 = struct PBSEL1_13(bool);
        /// PBSEL1_14
        PBSEL1_14: 14 = struct PBSEL1_14(bool);
        /// PBSEL1_15
        PBSEL1_15: 15 = struct PBSEL1_15(bool);
    }
}
