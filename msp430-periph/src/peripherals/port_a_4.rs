//! Port A

utils::periph! {
    /// Port A
    PortA;
    /// Port A Input
    rw PAIN @ 0x00: u16 = 0_0 {
        /// PAIN0
        PAIN0: 0 = struct PAIN0(bool);
        /// PAIN1
        PAIN1: 1 = struct PAIN1(bool);
        /// PAIN2
        PAIN2: 2 = struct PAIN2(bool);
        /// PAIN3
        PAIN3: 3 = struct PAIN3(bool);
        /// PAIN4
        PAIN4: 4 = struct PAIN4(bool);
        /// PAIN5
        PAIN5: 5 = struct PAIN5(bool);
        /// PAIN6
        PAIN6: 6 = struct PAIN6(bool);
        /// PAIN7
        PAIN7: 7 = struct PAIN7(bool);
        /// PAIN8
        PAIN8: 8 = struct PAIN8(bool);
        /// PAIN9
        PAIN9: 9 = struct PAIN9(bool);
        /// PAIN10
        PAIN10: 10 = struct PAIN10(bool);
        /// PAIN11
        PAIN11: 11 = struct PAIN11(bool);
        /// PAIN12
        PAIN12: 12 = struct PAIN12(bool);
        /// PAIN13
        PAIN13: 13 = struct PAIN13(bool);
        /// PAIN14
        PAIN14: 14 = struct PAIN14(bool);
        /// PAIN15
        PAIN15: 15 = struct PAIN15(bool);
    }
    /// Port A Output
    rw PAOUT @ 0x02: u16 = 0_0 {
        /// PAOUT0
        PAOUT0: 0 = struct PAOUT0(bool);
        /// PAOUT1
        PAOUT1: 1 = struct PAOUT1(bool);
        /// PAOUT2
        PAOUT2: 2 = struct PAOUT2(bool);
        /// PAOUT3
        PAOUT3: 3 = struct PAOUT3(bool);
        /// PAOUT4
        PAOUT4: 4 = struct PAOUT4(bool);
        /// PAOUT5
        PAOUT5: 5 = struct PAOUT5(bool);
        /// PAOUT6
        PAOUT6: 6 = struct PAOUT6(bool);
        /// PAOUT7
        PAOUT7: 7 = struct PAOUT7(bool);
        /// PAOUT8
        PAOUT8: 8 = struct PAOUT8(bool);
        /// PAOUT9
        PAOUT9: 9 = struct PAOUT9(bool);
        /// PAOUT10
        PAOUT10: 10 = struct PAOUT10(bool);
        /// PAOUT11
        PAOUT11: 11 = struct PAOUT11(bool);
        /// PAOUT12
        PAOUT12: 12 = struct PAOUT12(bool);
        /// PAOUT13
        PAOUT13: 13 = struct PAOUT13(bool);
        /// PAOUT14
        PAOUT14: 14 = struct PAOUT14(bool);
        /// PAOUT15
        PAOUT15: 15 = struct PAOUT15(bool);
    }
    /// Port A Direction
    rw PADIR @ 0x04: u16 = 0_0 {
        /// PADIR0
        PADIR0: 0 = struct PADIR0(bool);
        /// PADIR1
        PADIR1: 1 = struct PADIR1(bool);
        /// PADIR2
        PADIR2: 2 = struct PADIR2(bool);
        /// PADIR3
        PADIR3: 3 = struct PADIR3(bool);
        /// PADIR4
        PADIR4: 4 = struct PADIR4(bool);
        /// PADIR5
        PADIR5: 5 = struct PADIR5(bool);
        /// PADIR6
        PADIR6: 6 = struct PADIR6(bool);
        /// PADIR7
        PADIR7: 7 = struct PADIR7(bool);
        /// PADIR8
        PADIR8: 8 = struct PADIR8(bool);
        /// PADIR9
        PADIR9: 9 = struct PADIR9(bool);
        /// PADIR10
        PADIR10: 10 = struct PADIR10(bool);
        /// PADIR11
        PADIR11: 11 = struct PADIR11(bool);
        /// PADIR12
        PADIR12: 12 = struct PADIR12(bool);
        /// PADIR13
        PADIR13: 13 = struct PADIR13(bool);
        /// PADIR14
        PADIR14: 14 = struct PADIR14(bool);
        /// PADIR15
        PADIR15: 15 = struct PADIR15(bool);
    }
    /// Port A Selection 0
    rw PASEL0 @ 0x0a: u16 = 0_0 {
        /// PASEL0_0
        PASEL0_0: 0 = struct PASEL0_0(bool);
        /// PASEL0_1
        PASEL0_1: 1 = struct PASEL0_1(bool);
        /// PASEL0_2
        PASEL0_2: 2 = struct PASEL0_2(bool);
        /// PASEL0_3
        PASEL0_3: 3 = struct PASEL0_3(bool);
        /// PASEL0_4
        PASEL0_4: 4 = struct PASEL0_4(bool);
        /// PASEL0_5
        PASEL0_5: 5 = struct PASEL0_5(bool);
        /// PASEL0_6
        PASEL0_6: 6 = struct PASEL0_6(bool);
        /// PASEL0_7
        PASEL0_7: 7 = struct PASEL0_7(bool);
        /// PASEL0_8
        PASEL0_8: 8 = struct PASEL0_8(bool);
        /// PASEL0_9
        PASEL0_9: 9 = struct PASEL0_9(bool);
        /// PASEL0_10
        PASEL0_10: 10 = struct PASEL0_10(bool);
        /// PASEL0_11
        PASEL0_11: 11 = struct PASEL0_11(bool);
        /// PASEL0_12
        PASEL0_12: 12 = struct PASEL0_12(bool);
        /// PASEL0_13
        PASEL0_13: 13 = struct PASEL0_13(bool);
        /// PASEL0_14
        PASEL0_14: 14 = struct PASEL0_14(bool);
        /// PASEL0_15
        PASEL0_15: 15 = struct PASEL0_15(bool);
    }
    /// Port A Selection 1
    rw PASEL1 @ 0x0c: u16 = 0_0 {
        /// PASEL1_0
        PASEL1_0: 0 = struct PASEL1_0(bool);
        /// PASEL1_1
        PASEL1_1: 1 = struct PASEL1_1(bool);
        /// PASEL1_2
        PASEL1_2: 2 = struct PASEL1_2(bool);
        /// PASEL1_3
        PASEL1_3: 3 = struct PASEL1_3(bool);
        /// PASEL1_4
        PASEL1_4: 4 = struct PASEL1_4(bool);
        /// PASEL1_5
        PASEL1_5: 5 = struct PASEL1_5(bool);
        /// PASEL1_6
        PASEL1_6: 6 = struct PASEL1_6(bool);
        /// PASEL1_7
        PASEL1_7: 7 = struct PASEL1_7(bool);
        /// PASEL1_8
        PASEL1_8: 8 = struct PASEL1_8(bool);
        /// PASEL1_9
        PASEL1_9: 9 = struct PASEL1_9(bool);
        /// PASEL1_10
        PASEL1_10: 10 = struct PASEL1_10(bool);
        /// PASEL1_11
        PASEL1_11: 11 = struct PASEL1_11(bool);
        /// PASEL1_12
        PASEL1_12: 12 = struct PASEL1_12(bool);
        /// PASEL1_13
        PASEL1_13: 13 = struct PASEL1_13(bool);
        /// PASEL1_14
        PASEL1_14: 14 = struct PASEL1_14(bool);
        /// PASEL1_15
        PASEL1_15: 15 = struct PASEL1_15(bool);
    }
    /// Port A Interrupt Edge Select
    rw PAIES @ 0x18: u16 = 0_0 {
        /// PAIES0
        PAIES0: 0 = struct PAIES0(bool);
        /// PAIES1
        PAIES1: 1 = struct PAIES1(bool);
        /// PAIES2
        PAIES2: 2 = struct PAIES2(bool);
        /// PAIES3
        PAIES3: 3 = struct PAIES3(bool);
        /// PAIES4
        PAIES4: 4 = struct PAIES4(bool);
        /// PAIES5
        PAIES5: 5 = struct PAIES5(bool);
        /// PAIES6
        PAIES6: 6 = struct PAIES6(bool);
        /// PAIES7
        PAIES7: 7 = struct PAIES7(bool);
        /// PAIES8
        PAIES8: 8 = struct PAIES8(bool);
        /// PAIES9
        PAIES9: 9 = struct PAIES9(bool);
        /// PAIES10
        PAIES10: 10 = struct PAIES10(bool);
        /// PAIES11
        PAIES11: 11 = struct PAIES11(bool);
        /// PAIES12
        PAIES12: 12 = struct PAIES12(bool);
        /// PAIES13
        PAIES13: 13 = struct PAIES13(bool);
        /// PAIES14
        PAIES14: 14 = struct PAIES14(bool);
        /// PAIES15
        PAIES15: 15 = struct PAIES15(bool);
    }
    /// Port A Interrupt Enable
    rw PAIE @ 0x1a: u16 = 0_0 {
        /// PAIE0
        PAIE0: 0 = struct PAIE0(bool);
        /// PAIE1
        PAIE1: 1 = struct PAIE1(bool);
        /// PAIE2
        PAIE2: 2 = struct PAIE2(bool);
        /// PAIE3
        PAIE3: 3 = struct PAIE3(bool);
        /// PAIE4
        PAIE4: 4 = struct PAIE4(bool);
        /// PAIE5
        PAIE5: 5 = struct PAIE5(bool);
        /// PAIE6
        PAIE6: 6 = struct PAIE6(bool);
        /// PAIE7
        PAIE7: 7 = struct PAIE7(bool);
        /// PAIE8
        PAIE8: 8 = struct PAIE8(bool);
        /// PAIE9
        PAIE9: 9 = struct PAIE9(bool);
        /// PAIE10
        PAIE10: 10 = struct PAIE10(bool);
        /// PAIE11
        PAIE11: 11 = struct PAIE11(bool);
        /// PAIE12
        PAIE12: 12 = struct PAIE12(bool);
        /// PAIE13
        PAIE13: 13 = struct PAIE13(bool);
        /// PAIE14
        PAIE14: 14 = struct PAIE14(bool);
        /// PAIE15
        PAIE15: 15 = struct PAIE15(bool);
    }
    /// Port A Interrupt Flag
    rw PAIFG @ 0x1c: u16 = 0_0 {
        /// PAIFG0
        PAIFG0: 0 = struct PAIFG0(bool);
        /// PAIFG1
        PAIFG1: 1 = struct PAIFG1(bool);
        /// PAIFG2
        PAIFG2: 2 = struct PAIFG2(bool);
        /// PAIFG3
        PAIFG3: 3 = struct PAIFG3(bool);
        /// PAIFG4
        PAIFG4: 4 = struct PAIFG4(bool);
        /// PAIFG5
        PAIFG5: 5 = struct PAIFG5(bool);
        /// PAIFG6
        PAIFG6: 6 = struct PAIFG6(bool);
        /// PAIFG7
        PAIFG7: 7 = struct PAIFG7(bool);
        /// PAIFG8
        PAIFG8: 8 = struct PAIFG8(bool);
        /// PAIFG9
        PAIFG9: 9 = struct PAIFG9(bool);
        /// PAIFG10
        PAIFG10: 10 = struct PAIFG10(bool);
        /// PAIFG11
        PAIFG11: 11 = struct PAIFG11(bool);
        /// PAIFG12
        PAIFG12: 12 = struct PAIFG12(bool);
        /// PAIFG13
        PAIFG13: 13 = struct PAIFG13(bool);
        /// PAIFG14
        PAIFG14: 14 = struct PAIFG14(bool);
        /// PAIFG15
        PAIFG15: 15 = struct PAIFG15(bool);
    }
}
