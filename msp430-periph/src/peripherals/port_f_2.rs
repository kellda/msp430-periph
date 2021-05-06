//! Port F

utils::periph! {
    /// Port F
    PortF;
    /// Port F Input
    rw PFIN @ 0x00: u16 = 0_0 {
        /// PFIN0
        PFIN0: 0 = struct PFIN0(bool);
        /// PFIN1
        PFIN1: 1 = struct PFIN1(bool);
        /// PFIN2
        PFIN2: 2 = struct PFIN2(bool);
        /// PFIN3
        PFIN3: 3 = struct PFIN3(bool);
        /// PFIN4
        PFIN4: 4 = struct PFIN4(bool);
        /// PFIN5
        PFIN5: 5 = struct PFIN5(bool);
        /// PFIN6
        PFIN6: 6 = struct PFIN6(bool);
        /// PFIN7
        PFIN7: 7 = struct PFIN7(bool);
        /// PFIN8
        PFIN8: 8 = struct PFIN8(bool);
        /// PFIN9
        PFIN9: 9 = struct PFIN9(bool);
        /// PFIN10
        PFIN10: 10 = struct PFIN10(bool);
        /// PFIN11
        PFIN11: 11 = struct PFIN11(bool);
        /// PFIN12
        PFIN12: 12 = struct PFIN12(bool);
        /// PFIN13
        PFIN13: 13 = struct PFIN13(bool);
        /// PFIN14
        PFIN14: 14 = struct PFIN14(bool);
        /// PFIN15
        PFIN15: 15 = struct PFIN15(bool);
    }
    /// Port F Output
    rw PFOUT @ 0x02: u16 = 0_0 {
        /// PFOUT0
        PFOUT0: 0 = struct PFOUT0(bool);
        /// PFOUT1
        PFOUT1: 1 = struct PFOUT1(bool);
        /// PFOUT2
        PFOUT2: 2 = struct PFOUT2(bool);
        /// PFOUT3
        PFOUT3: 3 = struct PFOUT3(bool);
        /// PFOUT4
        PFOUT4: 4 = struct PFOUT4(bool);
        /// PFOUT5
        PFOUT5: 5 = struct PFOUT5(bool);
        /// PFOUT6
        PFOUT6: 6 = struct PFOUT6(bool);
        /// PFOUT7
        PFOUT7: 7 = struct PFOUT7(bool);
        /// PFOUT8
        PFOUT8: 8 = struct PFOUT8(bool);
        /// PFOUT9
        PFOUT9: 9 = struct PFOUT9(bool);
        /// PFOUT10
        PFOUT10: 10 = struct PFOUT10(bool);
        /// PFOUT11
        PFOUT11: 11 = struct PFOUT11(bool);
        /// PFOUT12
        PFOUT12: 12 = struct PFOUT12(bool);
        /// PFOUT13
        PFOUT13: 13 = struct PFOUT13(bool);
        /// PFOUT14
        PFOUT14: 14 = struct PFOUT14(bool);
        /// PFOUT15
        PFOUT15: 15 = struct PFOUT15(bool);
    }
    /// Port F Direction
    rw PFDIR @ 0x04: u16 = 0_0 {
        /// PFDIR0
        PFDIR0: 0 = struct PFDIR0(bool);
        /// PFDIR1
        PFDIR1: 1 = struct PFDIR1(bool);
        /// PFDIR2
        PFDIR2: 2 = struct PFDIR2(bool);
        /// PFDIR3
        PFDIR3: 3 = struct PFDIR3(bool);
        /// PFDIR4
        PFDIR4: 4 = struct PFDIR4(bool);
        /// PFDIR5
        PFDIR5: 5 = struct PFDIR5(bool);
        /// PFDIR6
        PFDIR6: 6 = struct PFDIR6(bool);
        /// PFDIR7
        PFDIR7: 7 = struct PFDIR7(bool);
        /// PFDIR8
        PFDIR8: 8 = struct PFDIR8(bool);
        /// PFDIR9
        PFDIR9: 9 = struct PFDIR9(bool);
        /// PFDIR10
        PFDIR10: 10 = struct PFDIR10(bool);
        /// PFDIR11
        PFDIR11: 11 = struct PFDIR11(bool);
        /// PFDIR12
        PFDIR12: 12 = struct PFDIR12(bool);
        /// PFDIR13
        PFDIR13: 13 = struct PFDIR13(bool);
        /// PFDIR14
        PFDIR14: 14 = struct PFDIR14(bool);
        /// PFDIR15
        PFDIR15: 15 = struct PFDIR15(bool);
    }
    /// Port F Resistor Enable
    rw PFREN @ 0x06: u16 = 0_0 {
        /// PFREN0
        PFREN0: 0 = struct PFREN0(bool);
        /// PFREN1
        PFREN1: 1 = struct PFREN1(bool);
        /// PFREN2
        PFREN2: 2 = struct PFREN2(bool);
        /// PFREN3
        PFREN3: 3 = struct PFREN3(bool);
        /// PFREN4
        PFREN4: 4 = struct PFREN4(bool);
        /// PFREN5
        PFREN5: 5 = struct PFREN5(bool);
        /// PFREN6
        PFREN6: 6 = struct PFREN6(bool);
        /// PFREN7
        PFREN7: 7 = struct PFREN7(bool);
        /// PFREN8
        PFREN8: 8 = struct PFREN8(bool);
        /// PFREN9
        PFREN9: 9 = struct PFREN9(bool);
        /// PFREN10
        PFREN10: 10 = struct PFREN10(bool);
        /// PFREN11
        PFREN11: 11 = struct PFREN11(bool);
        /// PFREN12
        PFREN12: 12 = struct PFREN12(bool);
        /// PFREN13
        PFREN13: 13 = struct PFREN13(bool);
        /// PFREN14
        PFREN14: 14 = struct PFREN14(bool);
        /// PFREN15
        PFREN15: 15 = struct PFREN15(bool);
    }
    /// Port F Drive Strenght
    rw PFDS @ 0x08: u16 = 0_0 {
        /// PFDS0
        PFDS0: 0 = struct PFDS0(bool);
        /// PFDS1
        PFDS1: 1 = struct PFDS1(bool);
        /// PFDS2
        PFDS2: 2 = struct PFDS2(bool);
        /// PFDS3
        PFDS3: 3 = struct PFDS3(bool);
        /// PFDS4
        PFDS4: 4 = struct PFDS4(bool);
        /// PFDS5
        PFDS5: 5 = struct PFDS5(bool);
        /// PFDS6
        PFDS6: 6 = struct PFDS6(bool);
        /// PFDS7
        PFDS7: 7 = struct PFDS7(bool);
        /// PFDS8
        PFDS8: 8 = struct PFDS8(bool);
        /// PFDS9
        PFDS9: 9 = struct PFDS9(bool);
        /// PFDS10
        PFDS10: 10 = struct PFDS10(bool);
        /// PFDS11
        PFDS11: 11 = struct PFDS11(bool);
        /// PFDS12
        PFDS12: 12 = struct PFDS12(bool);
        /// PFDS13
        PFDS13: 13 = struct PFDS13(bool);
        /// PFDS14
        PFDS14: 14 = struct PFDS14(bool);
        /// PFDS15
        PFDS15: 15 = struct PFDS15(bool);
    }
    /// Port F Selection 0
    rw PFSEL0 @ 0x0a: u16 = 0_0 {
        /// PFSEL0_0
        PFSEL0_0: 0 = struct PFSEL0_0(bool);
        /// PFSEL0_1
        PFSEL0_1: 1 = struct PFSEL0_1(bool);
        /// PFSEL0_2
        PFSEL0_2: 2 = struct PFSEL0_2(bool);
        /// PFSEL0_3
        PFSEL0_3: 3 = struct PFSEL0_3(bool);
        /// PFSEL0_4
        PFSEL0_4: 4 = struct PFSEL0_4(bool);
        /// PFSEL0_5
        PFSEL0_5: 5 = struct PFSEL0_5(bool);
        /// PFSEL0_6
        PFSEL0_6: 6 = struct PFSEL0_6(bool);
        /// PFSEL0_7
        PFSEL0_7: 7 = struct PFSEL0_7(bool);
        /// PFSEL0_8
        PFSEL0_8: 8 = struct PFSEL0_8(bool);
        /// PFSEL0_9
        PFSEL0_9: 9 = struct PFSEL0_9(bool);
        /// PFSEL0_10
        PFSEL0_10: 10 = struct PFSEL0_10(bool);
        /// PFSEL0_11
        PFSEL0_11: 11 = struct PFSEL0_11(bool);
        /// PFSEL0_12
        PFSEL0_12: 12 = struct PFSEL0_12(bool);
        /// PFSEL0_13
        PFSEL0_13: 13 = struct PFSEL0_13(bool);
        /// PFSEL0_14
        PFSEL0_14: 14 = struct PFSEL0_14(bool);
        /// PFSEL0_15
        PFSEL0_15: 15 = struct PFSEL0_15(bool);
    }
}
