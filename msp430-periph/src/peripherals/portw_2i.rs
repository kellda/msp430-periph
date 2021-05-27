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
    /// Port Drive Strenght
    rw DS @ 0x08: u16 = 0_0 {
        /// PDS0
        DS0: 0 = struct DS0(bool);
        /// PDS1
        DS1: 1 = struct DS1(bool);
        /// PDS2
        DS2: 2 = struct DS2(bool);
        /// PDS3
        DS3: 3 = struct DS3(bool);
        /// PDS4
        DS4: 4 = struct DS4(bool);
        /// PDS5
        DS5: 5 = struct DS5(bool);
        /// PDS6
        DS6: 6 = struct DS6(bool);
        /// PDS7
        DS7: 7 = struct DS7(bool);
        /// PDS8
        DS8: 8 = struct DS8(bool);
        /// PDS9
        DS9: 9 = struct DS9(bool);
        /// PDS10
        DS10: 10 = struct DS10(bool);
        /// PDS11
        DS11: 11 = struct DS11(bool);
        /// PDS12
        DS12: 12 = struct DS12(bool);
        /// PDS13
        DS13: 13 = struct DS13(bool);
        /// PDS14
        DS14: 14 = struct DS14(bool);
        /// PDS15
        DS15: 15 = struct DS15(bool);
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
    /// Port Selection 1
    rw SEL1 @ 0x0c: u16 = 0_0 {
        /// PSEL1_0
        SEL1_0: 0 = struct DSEL1_0(bool);
        /// PSEL1_1
        SEL1_1: 1 = struct DSEL1_1(bool);
        /// PSEL1_2
        SEL1_2: 2 = struct DSEL1_2(bool);
        /// PSEL1_3
        SEL1_3: 3 = struct DSEL1_3(bool);
        /// PSEL1_4
        SEL1_4: 4 = struct DSEL1_4(bool);
        /// PSEL1_5
        SEL1_5: 5 = struct DSEL1_5(bool);
        /// PSEL1_6
        SEL1_6: 6 = struct DSEL1_6(bool);
        /// PSEL1_7
        SEL1_7: 7 = struct DSEL1_7(bool);
        /// PSEL1_8
        SEL1_8: 8 = struct DSEL1_8(bool);
        /// PSEL1_9
        SEL1_9: 9 = struct DSEL1_9(bool);
        /// PSEL1_10
        SEL1_10: 10 = struct DSEL1_10(bool);
        /// PSEL1_11
        SEL1_11: 11 = struct DSEL1_11(bool);
        /// PSEL1_12
        SEL1_12: 12 = struct DSEL1_12(bool);
        /// PSEL1_13
        SEL1_13: 13 = struct DSEL1_13(bool);
        /// PSEL1_14
        SEL1_14: 14 = struct DSEL1_14(bool);
        /// PSEL1_15
        SEL1_15: 15 = struct DSEL1_15(bool);
    }
    /// Port Interrupt Edge Select
    rw IES @ 0x18: u16 = 0_0 {
        /// PIES0
        IES0: 0 = struct IES0(bool);
        /// PIES1
        IES1: 1 = struct IES1(bool);
        /// PIES2
        IES2: 2 = struct IES2(bool);
        /// PIES3
        IES3: 3 = struct IES3(bool);
        /// PIES4
        IES4: 4 = struct IES4(bool);
        /// PIES5
        IES5: 5 = struct IES5(bool);
        /// PIES6
        IES6: 6 = struct IES6(bool);
        /// PIES7
        IES7: 7 = struct IES7(bool);
        /// PIES8
        IES8: 8 = struct IES8(bool);
        /// PIES9
        IES9: 9 = struct IES9(bool);
        /// PIES10
        IES10: 10 = struct IES10(bool);
        /// PIES11
        IES11: 11 = struct IES11(bool);
        /// PIES12
        IES12: 12 = struct IES12(bool);
        /// PIES13
        IES13: 13 = struct IES13(bool);
        /// PIES14
        IES14: 14 = struct IES14(bool);
        /// PIES15
        IES15: 15 = struct IES15(bool);
    }
    /// Port Interrupt Enable
    rw IE @ 0x1a: u16 = 0_0 {
        /// PIE0
        IE0: 0 = struct IE0(bool);
        /// PIE1
        IE1: 1 = struct IE1(bool);
        /// PIE2
        IE2: 2 = struct IE2(bool);
        /// PIE3
        IE3: 3 = struct IE3(bool);
        /// PIE4
        IE4: 4 = struct IE4(bool);
        /// PIE5
        IE5: 5 = struct IE5(bool);
        /// PIE6
        IE6: 6 = struct IE6(bool);
        /// PIE7
        IE7: 7 = struct IE7(bool);
        /// PIE8
        IE8: 8 = struct IE8(bool);
        /// PIE9
        IE9: 9 = struct IE9(bool);
        /// PIE10
        IE10: 10 = struct IE10(bool);
        /// PIE11
        IE11: 11 = struct IE11(bool);
        /// PIE12
        IE12: 12 = struct IE12(bool);
        /// PIE13
        IE13: 13 = struct IE13(bool);
        /// PIE14
        IE14: 14 = struct IE14(bool);
        /// PIE15
        IE15: 15 = struct IE15(bool);
    }
    /// Port Interrupt Flag
    rw IFG @ 0x1c: u16 = 0_0 {
        /// PIFG0
        IFG0: 0 = struct IFG0(bool);
        /// PIFG1
        IFG1: 1 = struct IFG1(bool);
        /// PIFG2
        IFG2: 2 = struct IFG2(bool);
        /// PIFG3
        IFG3: 3 = struct IFG3(bool);
        /// PIFG4
        IFG4: 4 = struct IFG4(bool);
        /// PIFG5
        IFG5: 5 = struct IFG5(bool);
        /// PIFG6
        IFG6: 6 = struct IFG6(bool);
        /// PIFG7
        IFG7: 7 = struct IFG7(bool);
        /// PIFG8
        IFG8: 8 = struct IFG8(bool);
        /// PIFG9
        IFG9: 9 = struct IFG9(bool);
        /// PIFG10
        IFG10: 10 = struct IFG10(bool);
        /// PIFG11
        IFG11: 11 = struct IFG11(bool);
        /// PIFG12
        IFG12: 12 = struct IFG12(bool);
        /// PIFG13
        IFG13: 13 = struct IFG13(bool);
        /// PIFG14
        IFG14: 14 = struct IFG14(bool);
        /// PIFG15
        IFG15: 15 = struct IFG15(bool);
    }
}
