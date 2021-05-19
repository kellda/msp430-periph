//! Port x

utils::periph! {
    /// Port x
    Port;
    /// Port Input
    rw PIN @ 0x00: u16 = 0_0 {
        /// PIN0
        PIN0: 0 = struct PIN0(bool);
        /// PIN1
        PIN1: 1 = struct PIN1(bool);
        /// PIN2
        PIN2: 2 = struct PIN2(bool);
        /// PIN3
        PIN3: 3 = struct PIN3(bool);
        /// PIN4
        PIN4: 4 = struct PIN4(bool);
        /// PIN5
        PIN5: 5 = struct PIN5(bool);
        /// PIN6
        PIN6: 6 = struct PIN6(bool);
        /// PIN7
        PIN7: 7 = struct PIN7(bool);
        /// PIN8
        PIN8: 8 = struct PIN8(bool);
        /// PIN9
        PIN9: 9 = struct PIN9(bool);
        /// PIN10
        PIN10: 10 = struct PIN10(bool);
        /// PIN11
        PIN11: 11 = struct PIN11(bool);
        /// PIN12
        PIN12: 12 = struct PIN12(bool);
        /// PIN13
        PIN13: 13 = struct PIN13(bool);
        /// PIN14
        PIN14: 14 = struct PIN14(bool);
        /// PIN15
        PIN15: 15 = struct PIN15(bool);
    }
    /// Port Output
    rw POUT @ 0x02: u16 = 0_0 {
        /// POUT0
        POUT0: 0 = struct POUT0(bool);
        /// POUT1
        POUT1: 1 = struct POUT1(bool);
        /// POUT2
        POUT2: 2 = struct POUT2(bool);
        /// POUT3
        POUT3: 3 = struct POUT3(bool);
        /// POUT4
        POUT4: 4 = struct POUT4(bool);
        /// POUT5
        POUT5: 5 = struct POUT5(bool);
        /// POUT6
        POUT6: 6 = struct POUT6(bool);
        /// POUT7
        POUT7: 7 = struct POUT7(bool);
        /// POUT8
        POUT8: 8 = struct POUT8(bool);
        /// POUT9
        POUT9: 9 = struct POUT9(bool);
        /// POUT10
        POUT10: 10 = struct POUT10(bool);
        /// POUT11
        POUT11: 11 = struct POUT11(bool);
        /// POUT12
        POUT12: 12 = struct POUT12(bool);
        /// POUT13
        POUT13: 13 = struct POUT13(bool);
        /// POUT14
        POUT14: 14 = struct POUT14(bool);
        /// POUT15
        POUT15: 15 = struct POUT15(bool);
    }
    /// Port Direction
    rw PDIR @ 0x04: u16 = 0_0 {
        /// PDIR0
        PDIR0: 0 = struct PDIR0(bool);
        /// PDIR1
        PDIR1: 1 = struct PDIR1(bool);
        /// PDIR2
        PDIR2: 2 = struct PDIR2(bool);
        /// PDIR3
        PDIR3: 3 = struct PDIR3(bool);
        /// PDIR4
        PDIR4: 4 = struct PDIR4(bool);
        /// PDIR5
        PDIR5: 5 = struct PDIR5(bool);
        /// PDIR6
        PDIR6: 6 = struct PDIR6(bool);
        /// PDIR7
        PDIR7: 7 = struct PDIR7(bool);
        /// PDIR8
        PDIR8: 8 = struct PDIR8(bool);
        /// PDIR9
        PDIR9: 9 = struct PDIR9(bool);
        /// PDIR10
        PDIR10: 10 = struct PDIR10(bool);
        /// PDIR11
        PDIR11: 11 = struct PDIR11(bool);
        /// PDIR12
        PDIR12: 12 = struct PDIR12(bool);
        /// PDIR13
        PDIR13: 13 = struct PDIR13(bool);
        /// PDIR14
        PDIR14: 14 = struct PDIR14(bool);
        /// PDIR15
        PDIR15: 15 = struct PDIR15(bool);
    }
    /// Port Resistor Enable
    rw PREN @ 0x06: u16 = 0_0 {
        /// PREN0
        PREN0: 0 = struct PREN0(bool);
        /// PREN1
        PREN1: 1 = struct PREN1(bool);
        /// PREN2
        PREN2: 2 = struct PREN2(bool);
        /// PREN3
        PREN3: 3 = struct PREN3(bool);
        /// PREN4
        PREN4: 4 = struct PREN4(bool);
        /// PREN5
        PREN5: 5 = struct PREN5(bool);
        /// PREN6
        PREN6: 6 = struct PREN6(bool);
        /// PREN7
        PREN7: 7 = struct PREN7(bool);
        /// PREN8
        PREN8: 8 = struct PREN8(bool);
        /// PREN9
        PREN9: 9 = struct PREN9(bool);
        /// PREN10
        PREN10: 10 = struct PREN10(bool);
        /// PREN11
        PREN11: 11 = struct PREN11(bool);
        /// PREN12
        PREN12: 12 = struct PREN12(bool);
        /// PREN13
        PREN13: 13 = struct PREN13(bool);
        /// PREN14
        PREN14: 14 = struct PREN14(bool);
        /// PREN15
        PREN15: 15 = struct PREN15(bool);
    }
    /// Port Selection 0
    rw PSEL0 @ 0x0a: u16 = 0_0 {
        /// PSEL0_0
        PSEL0_0: 0 = struct PDSEL0_0(bool);
        /// PSEL0_1
        PSEL0_1: 1 = struct PDSEL0_1(bool);
        /// PSEL0_2
        PSEL0_2: 2 = struct PDSEL0_2(bool);
        /// PSEL0_3
        PSEL0_3: 3 = struct PDSEL0_3(bool);
        /// PSEL0_4
        PSEL0_4: 4 = struct PDSEL0_4(bool);
        /// PSEL0_5
        PSEL0_5: 5 = struct PDSEL0_5(bool);
        /// PSEL0_6
        PSEL0_6: 6 = struct PDSEL0_6(bool);
        /// PSEL0_7
        PSEL0_7: 7 = struct PDSEL0_7(bool);
        /// PSEL0_8
        PSEL0_8: 8 = struct PDSEL0_8(bool);
        /// PSEL0_9
        PSEL0_9: 9 = struct PDSEL0_9(bool);
        /// PSEL0_10
        PSEL0_10: 10 = struct PDSEL0_10(bool);
        /// PSEL0_11
        PSEL0_11: 11 = struct PDSEL0_11(bool);
        /// PSEL0_12
        PSEL0_12: 12 = struct PDSEL0_12(bool);
        /// PSEL0_13
        PSEL0_13: 13 = struct PDSEL0_13(bool);
        /// PSEL0_14
        PSEL0_14: 14 = struct PDSEL0_14(bool);
        /// PSEL0_15
        PSEL0_15: 15 = struct PDSEL0_15(bool);
    }
}
