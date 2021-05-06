//! Port D

utils::periph! {
    /// Port D
    PortD;
    /// Port D Input
    rw PDIN @ 0x00: u16 = 0_0 {
        /// PDIN0
        PDIN0: 0 = struct PDIN0(bool);
        /// PDIN1
        PDIN1: 1 = struct PDIN1(bool);
        /// PDIN2
        PDIN2: 2 = struct PDIN2(bool);
        /// PDIN3
        PDIN3: 3 = struct PDIN3(bool);
        /// PDIN4
        PDIN4: 4 = struct PDIN4(bool);
        /// PDIN5
        PDIN5: 5 = struct PDIN5(bool);
        /// PDIN6
        PDIN6: 6 = struct PDIN6(bool);
        /// PDIN7
        PDIN7: 7 = struct PDIN7(bool);
        /// PDIN8
        PDIN8: 8 = struct PDIN8(bool);
        /// PDIN9
        PDIN9: 9 = struct PDIN9(bool);
        /// PDIN10
        PDIN10: 10 = struct PDIN10(bool);
        /// PDIN11
        PDIN11: 11 = struct PDIN11(bool);
        /// PDIN12
        PDIN12: 12 = struct PDIN12(bool);
        /// PDIN13
        PDIN13: 13 = struct PDIN13(bool);
        /// PDIN14
        PDIN14: 14 = struct PDIN14(bool);
        /// PDIN15
        PDIN15: 15 = struct PDIN15(bool);
    }
    /// Port D Output
    rw PDOUT @ 0x02: u16 = 0_0 {
        /// PDOUT0
        PDOUT0: 0 = struct PDOUT0(bool);
        /// PDOUT1
        PDOUT1: 1 = struct PDOUT1(bool);
        /// PDOUT2
        PDOUT2: 2 = struct PDOUT2(bool);
        /// PDOUT3
        PDOUT3: 3 = struct PDOUT3(bool);
        /// PDOUT4
        PDOUT4: 4 = struct PDOUT4(bool);
        /// PDOUT5
        PDOUT5: 5 = struct PDOUT5(bool);
        /// PDOUT6
        PDOUT6: 6 = struct PDOUT6(bool);
        /// PDOUT7
        PDOUT7: 7 = struct PDOUT7(bool);
        /// PDOUT8
        PDOUT8: 8 = struct PDOUT8(bool);
        /// PDOUT9
        PDOUT9: 9 = struct PDOUT9(bool);
        /// PDOUT10
        PDOUT10: 10 = struct PDOUT10(bool);
        /// PDOUT11
        PDOUT11: 11 = struct PDOUT11(bool);
        /// PDOUT12
        PDOUT12: 12 = struct PDOUT12(bool);
        /// PDOUT13
        PDOUT13: 13 = struct PDOUT13(bool);
        /// PDOUT14
        PDOUT14: 14 = struct PDOUT14(bool);
        /// PDOUT15
        PDOUT15: 15 = struct PDOUT15(bool);
    }
    /// Port D Direction
    rw PDDIR @ 0x04: u16 = 0_0 {
        /// PDDIR0
        PDDIR0: 0 = struct PDDIR0(bool);
        /// PDDIR1
        PDDIR1: 1 = struct PDDIR1(bool);
        /// PDDIR2
        PDDIR2: 2 = struct PDDIR2(bool);
        /// PDDIR3
        PDDIR3: 3 = struct PDDIR3(bool);
        /// PDDIR4
        PDDIR4: 4 = struct PDDIR4(bool);
        /// PDDIR5
        PDDIR5: 5 = struct PDDIR5(bool);
        /// PDDIR6
        PDDIR6: 6 = struct PDDIR6(bool);
        /// PDDIR7
        PDDIR7: 7 = struct PDDIR7(bool);
        /// PDDIR8
        PDDIR8: 8 = struct PDDIR8(bool);
        /// PDDIR9
        PDDIR9: 9 = struct PDDIR9(bool);
        /// PDDIR10
        PDDIR10: 10 = struct PDDIR10(bool);
        /// PDDIR11
        PDDIR11: 11 = struct PDDIR11(bool);
        /// PDDIR12
        PDDIR12: 12 = struct PDDIR12(bool);
        /// PDDIR13
        PDDIR13: 13 = struct PDDIR13(bool);
        /// PDDIR14
        PDDIR14: 14 = struct PDDIR14(bool);
        /// PDDIR15
        PDDIR15: 15 = struct PDDIR15(bool);
    }
    /// Port D Resistor Enable
    rw PDREN @ 0x06: u16 = 0_0 {
        /// PDREN0
        PDREN0: 0 = struct PDREN0(bool);
        /// PDREN1
        PDREN1: 1 = struct PDREN1(bool);
        /// PDREN2
        PDREN2: 2 = struct PDREN2(bool);
        /// PDREN3
        PDREN3: 3 = struct PDREN3(bool);
        /// PDREN4
        PDREN4: 4 = struct PDREN4(bool);
        /// PDREN5
        PDREN5: 5 = struct PDREN5(bool);
        /// PDREN6
        PDREN6: 6 = struct PDREN6(bool);
        /// PDREN7
        PDREN7: 7 = struct PDREN7(bool);
        /// PDREN8
        PDREN8: 8 = struct PDREN8(bool);
        /// PDREN9
        PDREN9: 9 = struct PDREN9(bool);
        /// PDREN10
        PDREN10: 10 = struct PDREN10(bool);
        /// PDREN11
        PDREN11: 11 = struct PDREN11(bool);
        /// PDREN12
        PDREN12: 12 = struct PDREN12(bool);
        /// PDREN13
        PDREN13: 13 = struct PDREN13(bool);
        /// PDREN14
        PDREN14: 14 = struct PDREN14(bool);
        /// PDREN15
        PDREN15: 15 = struct PDREN15(bool);
    }
    /// Port D Selection 0
    rw PDSEL0 @ 0x0a: u16 = 0_0 {
        /// PDSEL0_0
        PDSEL0_0: 0 = struct PDSEL0_0(bool);
        /// PDSEL0_1
        PDSEL0_1: 1 = struct PDSEL0_1(bool);
        /// PDSEL0_2
        PDSEL0_2: 2 = struct PDSEL0_2(bool);
        /// PDSEL0_3
        PDSEL0_3: 3 = struct PDSEL0_3(bool);
        /// PDSEL0_4
        PDSEL0_4: 4 = struct PDSEL0_4(bool);
        /// PDSEL0_5
        PDSEL0_5: 5 = struct PDSEL0_5(bool);
        /// PDSEL0_6
        PDSEL0_6: 6 = struct PDSEL0_6(bool);
        /// PDSEL0_7
        PDSEL0_7: 7 = struct PDSEL0_7(bool);
        /// PDSEL0_8
        PDSEL0_8: 8 = struct PDSEL0_8(bool);
        /// PDSEL0_9
        PDSEL0_9: 9 = struct PDSEL0_9(bool);
        /// PDSEL0_10
        PDSEL0_10: 10 = struct PDSEL0_10(bool);
        /// PDSEL0_11
        PDSEL0_11: 11 = struct PDSEL0_11(bool);
        /// PDSEL0_12
        PDSEL0_12: 12 = struct PDSEL0_12(bool);
        /// PDSEL0_13
        PDSEL0_13: 13 = struct PDSEL0_13(bool);
        /// PDSEL0_14
        PDSEL0_14: 14 = struct PDSEL0_14(bool);
        /// PDSEL0_15
        PDSEL0_15: 15 = struct PDSEL0_15(bool);
    }
    /// Port D Selection 1
    rw PDSEL1 @ 0x0c: u16 = 0_0 {
        /// PDSEL1_0
        PDSEL1_0: 0 = struct PDSEL1_0(bool);
        /// PDSEL1_1
        PDSEL1_1: 1 = struct PDSEL1_1(bool);
        /// PDSEL1_2
        PDSEL1_2: 2 = struct PDSEL1_2(bool);
        /// PDSEL1_3
        PDSEL1_3: 3 = struct PDSEL1_3(bool);
        /// PDSEL1_4
        PDSEL1_4: 4 = struct PDSEL1_4(bool);
        /// PDSEL1_5
        PDSEL1_5: 5 = struct PDSEL1_5(bool);
        /// PDSEL1_6
        PDSEL1_6: 6 = struct PDSEL1_6(bool);
        /// PDSEL1_7
        PDSEL1_7: 7 = struct PDSEL1_7(bool);
        /// PDSEL1_8
        PDSEL1_8: 8 = struct PDSEL1_8(bool);
        /// PDSEL1_9
        PDSEL1_9: 9 = struct PDSEL1_9(bool);
        /// PDSEL1_10
        PDSEL1_10: 10 = struct PDSEL1_10(bool);
        /// PDSEL1_11
        PDSEL1_11: 11 = struct PDSEL1_11(bool);
        /// PDSEL1_12
        PDSEL1_12: 12 = struct PDSEL1_12(bool);
        /// PDSEL1_13
        PDSEL1_13: 13 = struct PDSEL1_13(bool);
        /// PDSEL1_14
        PDSEL1_14: 14 = struct PDSEL1_14(bool);
        /// PDSEL1_15
        PDSEL1_15: 15 = struct PDSEL1_15(bool);
    }
    /// Port D Complement Selection
    rw PDSELC @ 0x16: u16 = 0_0 {
        /// Port D Complement Selection
        PDSELC: 0..15 = struct PDSELCField(u16);
    }
}
