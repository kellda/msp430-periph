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
    /// Port Selection 1
    rw PSEL1 @ 0x0c: u16 = 0_0 {
        /// PSEL1_0
        PSEL1_0: 0 = struct PDSEL1_0(bool);
        /// PSEL1_1
        PSEL1_1: 1 = struct PDSEL1_1(bool);
        /// PSEL1_2
        PSEL1_2: 2 = struct PDSEL1_2(bool);
        /// PSEL1_3
        PSEL1_3: 3 = struct PDSEL1_3(bool);
        /// PSEL1_4
        PSEL1_4: 4 = struct PDSEL1_4(bool);
        /// PSEL1_5
        PSEL1_5: 5 = struct PDSEL1_5(bool);
        /// PSEL1_6
        PSEL1_6: 6 = struct PDSEL1_6(bool);
        /// PSEL1_7
        PSEL1_7: 7 = struct PDSEL1_7(bool);
        /// PSEL1_8
        PSEL1_8: 8 = struct PDSEL1_8(bool);
        /// PSEL1_9
        PSEL1_9: 9 = struct PDSEL1_9(bool);
        /// PSEL1_10
        PSEL1_10: 10 = struct PDSEL1_10(bool);
        /// PSEL1_11
        PSEL1_11: 11 = struct PDSEL1_11(bool);
        /// PSEL1_12
        PSEL1_12: 12 = struct PDSEL1_12(bool);
        /// PSEL1_13
        PSEL1_13: 13 = struct PDSEL1_13(bool);
        /// PSEL1_14
        PSEL1_14: 14 = struct PDSEL1_14(bool);
        /// PSEL1_15
        PSEL1_15: 15 = struct PDSEL1_15(bool);
    }
    /// Port Interrupt Edge Select
    rw PIES @ 0x18: u16 = 0_0 {
        /// PIES0
        PIES0: 0 = struct PIES0(bool);
        /// PIES1
        PIES1: 1 = struct PIES1(bool);
        /// PIES2
        PIES2: 2 = struct PIES2(bool);
        /// PIES3
        PIES3: 3 = struct PIES3(bool);
        /// PIES4
        PIES4: 4 = struct PIES4(bool);
        /// PIES5
        PIES5: 5 = struct PIES5(bool);
        /// PIES6
        PIES6: 6 = struct PIES6(bool);
        /// PIES7
        PIES7: 7 = struct PIES7(bool);
        /// PIES8
        PIES8: 8 = struct PIES8(bool);
        /// PIES9
        PIES9: 9 = struct PIES9(bool);
        /// PIES10
        PIES10: 10 = struct PIES10(bool);
        /// PIES11
        PIES11: 11 = struct PIES11(bool);
        /// PIES12
        PIES12: 12 = struct PIES12(bool);
        /// PIES13
        PIES13: 13 = struct PIES13(bool);
        /// PIES14
        PIES14: 14 = struct PIES14(bool);
        /// PIES15
        PIES15: 15 = struct PIES15(bool);
    }
    /// Port Interrupt Enable
    rw PIE @ 0x1a: u16 = 0_0 {
        /// PIE0
        PIE0: 0 = struct PIE0(bool);
        /// PIE1
        PIE1: 1 = struct PIE1(bool);
        /// PIE2
        PIE2: 2 = struct PIE2(bool);
        /// PIE3
        PIE3: 3 = struct PIE3(bool);
        /// PIE4
        PIE4: 4 = struct PIE4(bool);
        /// PIE5
        PIE5: 5 = struct PIE5(bool);
        /// PIE6
        PIE6: 6 = struct PIE6(bool);
        /// PIE7
        PIE7: 7 = struct PIE7(bool);
        /// PIE8
        PIE8: 8 = struct PIE8(bool);
        /// PIE9
        PIE9: 9 = struct PIE9(bool);
        /// PIE10
        PIE10: 10 = struct PIE10(bool);
        /// PIE11
        PIE11: 11 = struct PIE11(bool);
        /// PIE12
        PIE12: 12 = struct PIE12(bool);
        /// PIE13
        PIE13: 13 = struct PIE13(bool);
        /// PIE14
        PIE14: 14 = struct PIE14(bool);
        /// PIE15
        PIE15: 15 = struct PIE15(bool);
    }
    /// Port Interrupt Flag
    rw PIFG @ 0x1c: u16 = 0_0 {
        /// PIFG0
        PIFG0: 0 = struct PIFG0(bool);
        /// PIFG1
        PIFG1: 1 = struct PIFG1(bool);
        /// PIFG2
        PIFG2: 2 = struct PIFG2(bool);
        /// PIFG3
        PIFG3: 3 = struct PIFG3(bool);
        /// PIFG4
        PIFG4: 4 = struct PIFG4(bool);
        /// PIFG5
        PIFG5: 5 = struct PIFG5(bool);
        /// PIFG6
        PIFG6: 6 = struct PIFG6(bool);
        /// PIFG7
        PIFG7: 7 = struct PIFG7(bool);
        /// PIFG8
        PIFG8: 8 = struct PIFG8(bool);
        /// PIFG9
        PIFG9: 9 = struct PIFG9(bool);
        /// PIFG10
        PIFG10: 10 = struct PIFG10(bool);
        /// PIFG11
        PIFG11: 11 = struct PIFG11(bool);
        /// PIFG12
        PIFG12: 12 = struct PIFG12(bool);
        /// PIFG13
        PIFG13: 13 = struct PIFG13(bool);
        /// PIFG14
        PIFG14: 14 = struct PIFG14(bool);
        /// PIFG15
        PIFG15: 15 = struct PIFG15(bool);
    }
}
