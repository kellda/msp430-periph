//! Port E

utils::periph! {
    /// Port E
    PortE;
    /// Port E Input
    rw PEIN @ 0x00: u16 = 0_0 {
        /// PEIN0
        PEIN0: 0 = struct PEIN0(bool);
        /// PEIN1
        PEIN1: 1 = struct PEIN1(bool);
        /// PEIN2
        PEIN2: 2 = struct PEIN2(bool);
        /// PEIN3
        PEIN3: 3 = struct PEIN3(bool);
        /// PEIN4
        PEIN4: 4 = struct PEIN4(bool);
        /// PEIN5
        PEIN5: 5 = struct PEIN5(bool);
        /// PEIN6
        PEIN6: 6 = struct PEIN6(bool);
        /// PEIN7
        PEIN7: 7 = struct PEIN7(bool);
        /// PEIN8
        PEIN8: 8 = struct PEIN8(bool);
        /// PEIN9
        PEIN9: 9 = struct PEIN9(bool);
        /// PEIN10
        PEIN10: 10 = struct PEIN10(bool);
        /// PEIN11
        PEIN11: 11 = struct PEIN11(bool);
        /// PEIN12
        PEIN12: 12 = struct PEIN12(bool);
        /// PEIN13
        PEIN13: 13 = struct PEIN13(bool);
        /// PEIN14
        PEIN14: 14 = struct PEIN14(bool);
        /// PEIN15
        PEIN15: 15 = struct PEIN15(bool);
    }
    /// Port E Output
    rw PEOUT @ 0x02: u16 = 0_0 {
        /// PEOUT0
        PEOUT0: 0 = struct PEOUT0(bool);
        /// PEOUT1
        PEOUT1: 1 = struct PEOUT1(bool);
        /// PEOUT2
        PEOUT2: 2 = struct PEOUT2(bool);
        /// PEOUT3
        PEOUT3: 3 = struct PEOUT3(bool);
        /// PEOUT4
        PEOUT4: 4 = struct PEOUT4(bool);
        /// PEOUT5
        PEOUT5: 5 = struct PEOUT5(bool);
        /// PEOUT6
        PEOUT6: 6 = struct PEOUT6(bool);
        /// PEOUT7
        PEOUT7: 7 = struct PEOUT7(bool);
        /// PEOUT8
        PEOUT8: 8 = struct PEOUT8(bool);
        /// PEOUT9
        PEOUT9: 9 = struct PEOUT9(bool);
        /// PEOUT10
        PEOUT10: 10 = struct PEOUT10(bool);
        /// PEOUT11
        PEOUT11: 11 = struct PEOUT11(bool);
        /// PEOUT12
        PEOUT12: 12 = struct PEOUT12(bool);
        /// PEOUT13
        PEOUT13: 13 = struct PEOUT13(bool);
        /// PEOUT14
        PEOUT14: 14 = struct PEOUT14(bool);
        /// PEOUT15
        PEOUT15: 15 = struct PEOUT15(bool);
    }
    /// Port E Direction
    rw PEDIR @ 0x04: u16 = 0_0 {
        /// PEDIR0
        PEDIR0: 0 = struct PEDIR0(bool);
        /// PEDIR1
        PEDIR1: 1 = struct PEDIR1(bool);
        /// PEDIR2
        PEDIR2: 2 = struct PEDIR2(bool);
        /// PEDIR3
        PEDIR3: 3 = struct PEDIR3(bool);
        /// PEDIR4
        PEDIR4: 4 = struct PEDIR4(bool);
        /// PEDIR5
        PEDIR5: 5 = struct PEDIR5(bool);
        /// PEDIR6
        PEDIR6: 6 = struct PEDIR6(bool);
        /// PEDIR7
        PEDIR7: 7 = struct PEDIR7(bool);
        /// PEDIR8
        PEDIR8: 8 = struct PEDIR8(bool);
        /// PEDIR9
        PEDIR9: 9 = struct PEDIR9(bool);
        /// PEDIR10
        PEDIR10: 10 = struct PEDIR10(bool);
        /// PEDIR11
        PEDIR11: 11 = struct PEDIR11(bool);
        /// PEDIR12
        PEDIR12: 12 = struct PEDIR12(bool);
        /// PEDIR13
        PEDIR13: 13 = struct PEDIR13(bool);
        /// PEDIR14
        PEDIR14: 14 = struct PEDIR14(bool);
        /// PEDIR15
        PEDIR15: 15 = struct PEDIR15(bool);
    }
    /// Port E Resistor Enable
    rw PEREN @ 0x06: u16 = 0_0 {
        /// PEREN0
        PEREN0: 0 = struct PEREN0(bool);
        /// PEREN1
        PEREN1: 1 = struct PEREN1(bool);
        /// PEREN2
        PEREN2: 2 = struct PEREN2(bool);
        /// PEREN3
        PEREN3: 3 = struct PEREN3(bool);
        /// PEREN4
        PEREN4: 4 = struct PEREN4(bool);
        /// PEREN5
        PEREN5: 5 = struct PEREN5(bool);
        /// PEREN6
        PEREN6: 6 = struct PEREN6(bool);
        /// PEREN7
        PEREN7: 7 = struct PEREN7(bool);
        /// PEREN8
        PEREN8: 8 = struct PEREN8(bool);
        /// PEREN9
        PEREN9: 9 = struct PEREN9(bool);
        /// PEREN10
        PEREN10: 10 = struct PEREN10(bool);
        /// PEREN11
        PEREN11: 11 = struct PEREN11(bool);
        /// PEREN12
        PEREN12: 12 = struct PEREN12(bool);
        /// PEREN13
        PEREN13: 13 = struct PEREN13(bool);
        /// PEREN14
        PEREN14: 14 = struct PEREN14(bool);
        /// PEREN15
        PEREN15: 15 = struct PEREN15(bool);
    }
    /// Port E Selection 0
    rw PESEL0 @ 0x0a: u16 = 0_0 {
        /// PESEL0_0
        PESEL0_0: 0 = struct PESEL0_0(bool);
        /// PESEL0_1
        PESEL0_1: 1 = struct PESEL0_1(bool);
        /// PESEL0_2
        PESEL0_2: 2 = struct PESEL0_2(bool);
        /// PESEL0_3
        PESEL0_3: 3 = struct PESEL0_3(bool);
        /// PESEL0_4
        PESEL0_4: 4 = struct PESEL0_4(bool);
        /// PESEL0_5
        PESEL0_5: 5 = struct PESEL0_5(bool);
        /// PESEL0_6
        PESEL0_6: 6 = struct PESEL0_6(bool);
        /// PESEL0_7
        PESEL0_7: 7 = struct PESEL0_7(bool);
        /// PESEL0_8
        PESEL0_8: 8 = struct PESEL0_8(bool);
        /// PESEL0_9
        PESEL0_9: 9 = struct PESEL0_9(bool);
        /// PESEL0_10
        PESEL0_10: 10 = struct PESEL0_10(bool);
        /// PESEL0_11
        PESEL0_11: 11 = struct PESEL0_11(bool);
        /// PESEL0_12
        PESEL0_12: 12 = struct PESEL0_12(bool);
        /// PESEL0_13
        PESEL0_13: 13 = struct PESEL0_13(bool);
        /// PESEL0_14
        PESEL0_14: 14 = struct PESEL0_14(bool);
        /// PESEL0_15
        PESEL0_15: 15 = struct PESEL0_15(bool);
    }
    /// Port E Selection 1
    rw PESEL1 @ 0x0c: u16 = 0_0 {
        /// PESEL1_0
        PESEL1_0: 0 = struct PESEL1_0(bool);
        /// PESEL1_1
        PESEL1_1: 1 = struct PESEL1_1(bool);
        /// PESEL1_2
        PESEL1_2: 2 = struct PESEL1_2(bool);
        /// PESEL1_3
        PESEL1_3: 3 = struct PESEL1_3(bool);
        /// PESEL1_4
        PESEL1_4: 4 = struct PESEL1_4(bool);
        /// PESEL1_5
        PESEL1_5: 5 = struct PESEL1_5(bool);
        /// PESEL1_6
        PESEL1_6: 6 = struct PESEL1_6(bool);
        /// PESEL1_7
        PESEL1_7: 7 = struct PESEL1_7(bool);
        /// PESEL1_8
        PESEL1_8: 8 = struct PESEL1_8(bool);
        /// PESEL1_9
        PESEL1_9: 9 = struct PESEL1_9(bool);
        /// PESEL1_10
        PESEL1_10: 10 = struct PESEL1_10(bool);
        /// PESEL1_11
        PESEL1_11: 11 = struct PESEL1_11(bool);
        /// PESEL1_12
        PESEL1_12: 12 = struct PESEL1_12(bool);
        /// PESEL1_13
        PESEL1_13: 13 = struct PESEL1_13(bool);
        /// PESEL1_14
        PESEL1_14: 14 = struct PESEL1_14(bool);
        /// PESEL1_15
        PESEL1_15: 15 = struct PESEL1_15(bool);
    }
    /// Port E Complement Selection
    rw PESELC @ 0x16: u16 = 0_0 {
        /// PESELC_0
        PESELC_0: 0 = struct PESELC_0(bool);
        /// PESELC_1
        PESELC_1: 1 = struct PESELC_1(bool);
        /// PESELC_2
        PESELC_2: 2 = struct PESELC_2(bool);
        /// PESELC_3
        PESELC_3: 3 = struct PESELC_3(bool);
        /// PESELC_4
        PESELC_4: 4 = struct PESELC_4(bool);
        /// PESELC_5
        PESELC_5: 5 = struct PESELC_5(bool);
        /// PESELC_6
        PESELC_6: 6 = struct PESELC_6(bool);
        /// PESELC_7
        PESELC_7: 7 = struct PESELC_7(bool);
        /// PESELC_8
        PESELC_8: 8 = struct PESELC_8(bool);
        /// PESELC_9
        PESELC_9: 9 = struct PESELC_9(bool);
        /// PESELC_10
        PESELC_10: 10 = struct PESELC_10(bool);
        /// PESELC_11
        PESELC_11: 11 = struct PESELC_11(bool);
        /// PESELC_12
        PESELC_12: 12 = struct PESELC_12(bool);
        /// PESELC_13
        PESELC_13: 13 = struct PESELC_13(bool);
        /// PESELC_14
        PESELC_14: 14 = struct PESELC_14(bool);
        /// PESELC_15
        PESELC_15: 15 = struct PESELC_15(bool);
    }
}
