//! Port x

utils::periph! {
    /// Port x
    Port;
    /// Port Input
    rw IN_ @ 0x00: u16 = 0_0 {
        /// PIN0
        IN0: 0 = struct IN0(bool);
        /// PIN1
        IN1: 1 = struct IN1(bool);
        /// PIN2
        IN2: 2 = struct IN2(bool);
        /// PIN3
        IN3: 3 = struct IN3(bool);
        /// PIN4
        IN4: 4 = struct IN4(bool);
        /// PIN5
        IN5: 5 = struct IN5(bool);
        /// PIN6
        IN6: 6 = struct IN6(bool);
        /// PIN7
        IN7: 7 = struct IN7(bool);
        /// PIN8
        IN8: 8 = struct IN8(bool);
        /// PIN9
        IN9: 9 = struct IN9(bool);
        /// PIN10
        IN10: 10 = struct IN10(bool);
        /// PIN11
        IN11: 11 = struct IN11(bool);
        /// PIN12
        IN12: 12 = struct IN12(bool);
        /// PIN13
        IN13: 13 = struct IN13(bool);
        /// PIN14
        IN14: 14 = struct IN14(bool);
        /// PIN15
        IN15: 15 = struct IN15(bool);
    }
    /// Port Output
    rw OUT @ 0x02: u16 = 0_0 {
        /// POUT0
        OUT0: 0 = struct OUT0(bool);
        /// POUT1
        OUT1: 1 = struct OUT1(bool);
        /// POUT2
        OUT2: 2 = struct OUT2(bool);
        /// POUT3
        OUT3: 3 = struct OUT3(bool);
        /// POUT4
        OUT4: 4 = struct OUT4(bool);
        /// POUT5
        OUT5: 5 = struct OUT5(bool);
        /// POUT6
        OUT6: 6 = struct OUT6(bool);
        /// POUT7
        OUT7: 7 = struct OUT7(bool);
        /// POUT8
        OUT8: 8 = struct OUT8(bool);
        /// POUT9
        OUT9: 9 = struct OUT9(bool);
        /// POUT10
        OUT10: 10 = struct OUT10(bool);
        /// POUT11
        OUT11: 11 = struct OUT11(bool);
        /// POUT12
        OUT12: 12 = struct OUT12(bool);
        /// POUT13
        OUT13: 13 = struct OUT13(bool);
        /// POUT14
        OUT14: 14 = struct OUT14(bool);
        /// POUT15
        OUT15: 15 = struct OUT15(bool);
    }
    /// Port Direction
    rw DIR @ 0x04: u16 = 0_0 {
        /// PDIR0
        DIR0: 0 = struct DIR0(bool);
        /// PDIR1
        DIR1: 1 = struct DIR1(bool);
        /// PDIR2
        DIR2: 2 = struct DIR2(bool);
        /// PDIR3
        DIR3: 3 = struct DIR3(bool);
        /// PDIR4
        DIR4: 4 = struct DIR4(bool);
        /// PDIR5
        DIR5: 5 = struct DIR5(bool);
        /// PDIR6
        DIR6: 6 = struct DIR6(bool);
        /// PDIR7
        DIR7: 7 = struct DIR7(bool);
        /// PDIR8
        DIR8: 8 = struct DIR8(bool);
        /// PDIR9
        DIR9: 9 = struct DIR9(bool);
        /// PDIR10
        DIR10: 10 = struct DIR10(bool);
        /// PDIR11
        DIR11: 11 = struct DIR11(bool);
        /// PDIR12
        DIR12: 12 = struct DIR12(bool);
        /// PDIR13
        DIR13: 13 = struct DIR13(bool);
        /// PDIR14
        DIR14: 14 = struct DIR14(bool);
        /// PDIR15
        DIR15: 15 = struct DIR15(bool);
    }
    /// Port Resistor Enable
    rw REN @ 0x06: u16 = 0_0 {
        /// PREN0
        REN0: 0 = struct REN0(bool);
        /// PREN1
        REN1: 1 = struct REN1(bool);
        /// PREN2
        REN2: 2 = struct REN2(bool);
        /// PREN3
        REN3: 3 = struct REN3(bool);
        /// PREN4
        REN4: 4 = struct REN4(bool);
        /// PREN5
        REN5: 5 = struct REN5(bool);
        /// PREN6
        REN6: 6 = struct REN6(bool);
        /// PREN7
        REN7: 7 = struct REN7(bool);
        /// PREN8
        REN8: 8 = struct REN8(bool);
        /// PREN9
        REN9: 9 = struct REN9(bool);
        /// PREN10
        REN10: 10 = struct REN10(bool);
        /// PREN11
        REN11: 11 = struct REN11(bool);
        /// PREN12
        REN12: 12 = struct REN12(bool);
        /// PREN13
        REN13: 13 = struct REN13(bool);
        /// PREN14
        REN14: 14 = struct REN14(bool);
        /// PREN15
        REN15: 15 = struct REN15(bool);
    }
    /// Port Selection 0
    rw SEL0 @ 0x0a: u16 = 0_0 {
        /// PSEL0_0
        SEL0_0: 0 = struct DSEL0_0(bool);
        /// PSEL0_1
        SEL0_1: 1 = struct DSEL0_1(bool);
        /// PSEL0_2
        SEL0_2: 2 = struct DSEL0_2(bool);
        /// PSEL0_3
        SEL0_3: 3 = struct DSEL0_3(bool);
        /// PSEL0_4
        SEL0_4: 4 = struct DSEL0_4(bool);
        /// PSEL0_5
        SEL0_5: 5 = struct DSEL0_5(bool);
        /// PSEL0_6
        SEL0_6: 6 = struct DSEL0_6(bool);
        /// PSEL0_7
        SEL0_7: 7 = struct DSEL0_7(bool);
        /// PSEL0_8
        SEL0_8: 8 = struct DSEL0_8(bool);
        /// PSEL0_9
        SEL0_9: 9 = struct DSEL0_9(bool);
        /// PSEL0_10
        SEL0_10: 10 = struct DSEL0_10(bool);
        /// PSEL0_11
        SEL0_11: 11 = struct DSEL0_11(bool);
        /// PSEL0_12
        SEL0_12: 12 = struct DSEL0_12(bool);
        /// PSEL0_13
        SEL0_13: 13 = struct DSEL0_13(bool);
        /// PSEL0_14
        SEL0_14: 14 = struct DSEL0_14(bool);
        /// PSEL0_15
        SEL0_15: 15 = struct DSEL0_15(bool);
    }
}
