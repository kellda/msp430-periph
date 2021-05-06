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
    /// Port A Resistor Enable
    rw PAREN @ 0x06: u16 = 0_0 {
        /// PAREN0
        PAREN0: 0 = struct PAREN0(bool);
        /// PAREN1
        PAREN1: 1 = struct PAREN1(bool);
        /// PAREN2
        PAREN2: 2 = struct PAREN2(bool);
        /// PAREN3
        PAREN3: 3 = struct PAREN3(bool);
        /// PAREN4
        PAREN4: 4 = struct PAREN4(bool);
        /// PAREN5
        PAREN5: 5 = struct PAREN5(bool);
        /// PAREN6
        PAREN6: 6 = struct PAREN6(bool);
        /// PAREN7
        PAREN7: 7 = struct PAREN7(bool);
        /// PAREN8
        PAREN8: 8 = struct PAREN8(bool);
        /// PAREN9
        PAREN9: 9 = struct PAREN9(bool);
        /// PAREN10
        PAREN10: 10 = struct PAREN10(bool);
        /// PAREN11
        PAREN11: 11 = struct PAREN11(bool);
        /// PAREN12
        PAREN12: 12 = struct PAREN12(bool);
        /// PAREN13
        PAREN13: 13 = struct PAREN13(bool);
        /// PAREN14
        PAREN14: 14 = struct PAREN14(bool);
        /// PAREN15
        PAREN15: 15 = struct PAREN15(bool);
    }
    /// Port A Drive Strenght
    rw PADS @ 0x08: u16 = 0_0 {
        /// PADS0
        PADS0: 0 = struct PADS0(bool);
        /// PADS1
        PADS1: 1 = struct PADS1(bool);
        /// PADS2
        PADS2: 2 = struct PADS2(bool);
        /// PADS3
        PADS3: 3 = struct PADS3(bool);
        /// PADS4
        PADS4: 4 = struct PADS4(bool);
        /// PADS5
        PADS5: 5 = struct PADS5(bool);
        /// PADS6
        PADS6: 6 = struct PADS6(bool);
        /// PADS7
        PADS7: 7 = struct PADS7(bool);
        /// PADS8
        PADS8: 8 = struct PADS8(bool);
        /// PADS9
        PADS9: 9 = struct PADS9(bool);
        /// PADS10
        PADS10: 10 = struct PADS10(bool);
        /// PADS11
        PADS11: 11 = struct PADS11(bool);
        /// PADS12
        PADS12: 12 = struct PADS12(bool);
        /// PADS13
        PADS13: 13 = struct PADS13(bool);
        /// PADS14
        PADS14: 14 = struct PADS14(bool);
        /// PADS15
        PADS15: 15 = struct PADS15(bool);
    }
    /// Port A Selection
    rw PASEL @ 0x0a: u16 = 0_0 {
        /// PASEL0
        PASEL0: 0 = struct PASEL0(bool);
        /// PASEL1
        PASEL1: 1 = struct PASEL1(bool);
        /// PASEL2
        PASEL2: 2 = struct PASEL2(bool);
        /// PASEL3
        PASEL3: 3 = struct PASEL3(bool);
        /// PASEL4
        PASEL4: 4 = struct PASEL4(bool);
        /// PASEL5
        PASEL5: 5 = struct PASEL5(bool);
        /// PASEL6
        PASEL6: 6 = struct PASEL6(bool);
        /// PASEL7
        PASEL7: 7 = struct PASEL7(bool);
        /// PASEL8
        PASEL8: 8 = struct PASEL8(bool);
        /// PASEL9
        PASEL9: 9 = struct PASEL9(bool);
        /// PASEL10
        PASEL10: 10 = struct PASEL10(bool);
        /// PASEL11
        PASEL11: 11 = struct PASEL11(bool);
        /// PASEL12
        PASEL12: 12 = struct PASEL12(bool);
        /// PASEL13
        PASEL13: 13 = struct PASEL13(bool);
        /// PASEL14
        PASEL14: 14 = struct PASEL14(bool);
        /// PASEL15
        PASEL15: 15 = struct PASEL15(bool);
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
