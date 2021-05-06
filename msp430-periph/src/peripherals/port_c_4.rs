//! Port C

utils::periph! {
    /// Port C
    PortC;
    /// Port C Input
    rw PCIN @ 0x00: u16 = 0_0 {
        /// PCIN0
        PCIN0: 0 = struct PCIN0(bool);
        /// PCIN1
        PCIN1: 1 = struct PCIN1(bool);
        /// PCIN2
        PCIN2: 2 = struct PCIN2(bool);
        /// PCIN3
        PCIN3: 3 = struct PCIN3(bool);
        /// PCIN4
        PCIN4: 4 = struct PCIN4(bool);
        /// PCIN5
        PCIN5: 5 = struct PCIN5(bool);
        /// PCIN6
        PCIN6: 6 = struct PCIN6(bool);
        /// PCIN7
        PCIN7: 7 = struct PCIN7(bool);
        /// PCIN8
        PCIN8: 8 = struct PCIN8(bool);
        /// PCIN9
        PCIN9: 9 = struct PCIN9(bool);
        /// PCIN10
        PCIN10: 10 = struct PCIN10(bool);
        /// PCIN11
        PCIN11: 11 = struct PCIN11(bool);
        /// PCIN12
        PCIN12: 12 = struct PCIN12(bool);
        /// PCIN13
        PCIN13: 13 = struct PCIN13(bool);
        /// PCIN14
        PCIN14: 14 = struct PCIN14(bool);
        /// PCIN15
        PCIN15: 15 = struct PCIN15(bool);
    }
    /// Port C Output
    rw PCOUT @ 0x02: u16 = 0_0 {
        /// PCOUT0
        PCOUT0: 0 = struct PCOUT0(bool);
        /// PCOUT1
        PCOUT1: 1 = struct PCOUT1(bool);
        /// PCOUT2
        PCOUT2: 2 = struct PCOUT2(bool);
        /// PCOUT3
        PCOUT3: 3 = struct PCOUT3(bool);
        /// PCOUT4
        PCOUT4: 4 = struct PCOUT4(bool);
        /// PCOUT5
        PCOUT5: 5 = struct PCOUT5(bool);
        /// PCOUT6
        PCOUT6: 6 = struct PCOUT6(bool);
        /// PCOUT7
        PCOUT7: 7 = struct PCOUT7(bool);
        /// PCOUT8
        PCOUT8: 8 = struct PCOUT8(bool);
        /// PCOUT9
        PCOUT9: 9 = struct PCOUT9(bool);
        /// PCOUT10
        PCOUT10: 10 = struct PCOUT10(bool);
        /// PCOUT11
        PCOUT11: 11 = struct PCOUT11(bool);
        /// PCOUT12
        PCOUT12: 12 = struct PCOUT12(bool);
        /// PCOUT13
        PCOUT13: 13 = struct PCOUT13(bool);
        /// PCOUT14
        PCOUT14: 14 = struct PCOUT14(bool);
        /// PCOUT15
        PCOUT15: 15 = struct PCOUT15(bool);
    }
    /// Port C Direction
    rw PCDIR @ 0x04: u16 = 0_0 {
        /// PCDIR0
        PCDIR0: 0 = struct PCDIR0(bool);
        /// PCDIR1
        PCDIR1: 1 = struct PCDIR1(bool);
        /// PCDIR2
        PCDIR2: 2 = struct PCDIR2(bool);
        /// PCDIR3
        PCDIR3: 3 = struct PCDIR3(bool);
        /// PCDIR4
        PCDIR4: 4 = struct PCDIR4(bool);
        /// PCDIR5
        PCDIR5: 5 = struct PCDIR5(bool);
        /// PCDIR6
        PCDIR6: 6 = struct PCDIR6(bool);
        /// PCDIR7
        PCDIR7: 7 = struct PCDIR7(bool);
        /// PCDIR8
        PCDIR8: 8 = struct PCDIR8(bool);
        /// PCDIR9
        PCDIR9: 9 = struct PCDIR9(bool);
        /// PCDIR10
        PCDIR10: 10 = struct PCDIR10(bool);
        /// PCDIR11
        PCDIR11: 11 = struct PCDIR11(bool);
        /// PCDIR12
        PCDIR12: 12 = struct PCDIR12(bool);
        /// PCDIR13
        PCDIR13: 13 = struct PCDIR13(bool);
        /// PCDIR14
        PCDIR14: 14 = struct PCDIR14(bool);
        /// PCDIR15
        PCDIR15: 15 = struct PCDIR15(bool);
    }
    /// Port C Resistor Enable
    rw PCREN @ 0x06: u16 = 0_0 {
        /// PCREN0
        PCREN0: 0 = struct PCREN0(bool);
        /// PCREN1
        PCREN1: 1 = struct PCREN1(bool);
        /// PCREN2
        PCREN2: 2 = struct PCREN2(bool);
        /// PCREN3
        PCREN3: 3 = struct PCREN3(bool);
        /// PCREN4
        PCREN4: 4 = struct PCREN4(bool);
        /// PCREN5
        PCREN5: 5 = struct PCREN5(bool);
        /// PCREN6
        PCREN6: 6 = struct PCREN6(bool);
        /// PCREN7
        PCREN7: 7 = struct PCREN7(bool);
        /// PCREN8
        PCREN8: 8 = struct PCREN8(bool);
        /// PCREN9
        PCREN9: 9 = struct PCREN9(bool);
        /// PCREN10
        PCREN10: 10 = struct PCREN10(bool);
        /// PCREN11
        PCREN11: 11 = struct PCREN11(bool);
        /// PCREN12
        PCREN12: 12 = struct PCREN12(bool);
        /// PCREN13
        PCREN13: 13 = struct PCREN13(bool);
        /// PCREN14
        PCREN14: 14 = struct PCREN14(bool);
        /// PCREN15
        PCREN15: 15 = struct PCREN15(bool);
    }
    /// Port C Selection 0
    rw PCSEL0 @ 0x0a: u16 = 0_0 {
        /// PCSEL0_0
        PCSEL0_0: 0 = struct PCSEL0_0(bool);
        /// PCSEL0_1
        PCSEL0_1: 1 = struct PCSEL0_1(bool);
        /// PCSEL0_2
        PCSEL0_2: 2 = struct PCSEL0_2(bool);
        /// PCSEL0_3
        PCSEL0_3: 3 = struct PCSEL0_3(bool);
        /// PCSEL0_4
        PCSEL0_4: 4 = struct PCSEL0_4(bool);
        /// PCSEL0_5
        PCSEL0_5: 5 = struct PCSEL0_5(bool);
        /// PCSEL0_6
        PCSEL0_6: 6 = struct PCSEL0_6(bool);
        /// PCSEL0_7
        PCSEL0_7: 7 = struct PCSEL0_7(bool);
        /// PCSEL0_8
        PCSEL0_8: 8 = struct PCSEL0_8(bool);
        /// PCSEL0_9
        PCSEL0_9: 9 = struct PCSEL0_9(bool);
        /// PCSEL0_10
        PCSEL0_10: 10 = struct PCSEL0_10(bool);
        /// PCSEL0_11
        PCSEL0_11: 11 = struct PCSEL0_11(bool);
        /// PCSEL0_12
        PCSEL0_12: 12 = struct PCSEL0_12(bool);
        /// PCSEL0_13
        PCSEL0_13: 13 = struct PCSEL0_13(bool);
        /// PCSEL0_14
        PCSEL0_14: 14 = struct PCSEL0_14(bool);
        /// PCSEL0_15
        PCSEL0_15: 15 = struct PCSEL0_15(bool);
    }
}
