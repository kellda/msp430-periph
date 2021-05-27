//! Port x

utils::periph! {
    /// Port x
    Port;
    /// Port Input
    rw IN_ @ 0x00: u8 = 0_0 {
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
    }
    /// Port Output
    rw OUT @ 0x02: u8 = 0_0 {
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
    }
    /// Port Direction
    rw DIR @ 0x04: u8 = 0_0 {
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
    }
    /// Port Resistor Enable
    rw REN @ 0x06: u8 = 0_0 {
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
    }
    /// Port Selection 0
    rw SEL0 @ 0x0a: u8 = 0_0 {
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
    }
    /// Port Selection 1
    rw SEL1 @ 0x0c: u8 = 0_0 {
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
    }
    /// Port Interrupt Edge Select
    rw IES @ 0x18: u8 = 0_0 {
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
    }
    /// Port Interrupt Enable
    rw IE @ 0x1a: u8 = 0_0 {
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
    }
    /// Port Interrupt Flag
    rw IFG @ 0x1c: u8 = 0_0 {
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
    }
    /// Port Interrupt Vector Word
    rw IV @ 0x0e: u16 = 0_0 {
        /// Port Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
}
