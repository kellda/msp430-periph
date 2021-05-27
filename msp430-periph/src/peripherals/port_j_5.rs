//! Port J

utils::periph! {
    /// Port J
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
    }
    /// Port Selection 0
    rw SEL0 @ 0x0a: u16 = 0_0 {
        /// PSEL0_0
        SEL0_0: 0 = struct SEL0_0(bool);
        /// PSEL0_1
        SEL0_1: 1 = struct SEL0_1(bool);
        /// PSEL0_2
        SEL0_2: 2 = struct SEL0_2(bool);
        /// PSEL0_3
        SEL0_3: 3 = struct SEL0_3(bool);
        /// PSEL0_4
        SEL0_4: 4 = struct SEL0_4(bool);
        /// PSEL0_5
        SEL0_5: 5 = struct SEL0_5(bool);
        /// PSEL0_6
        SEL0_6: 6 = struct SEL0_6(bool);
        /// PSEL0_7
        SEL0_7: 7 = struct SEL0_7(bool);
    }
    /// Port Selection 1
    rw SEL1 @ 0x0c: u16 = 0_0 {
        /// PSEL1_0
        SEL1_0: 0 = struct SEL1_0(bool);
        /// PSEL1_1
        SEL1_1: 1 = struct SEL1_1(bool);
        /// PSEL1_2
        SEL1_2: 2 = struct SEL1_2(bool);
        /// PSEL1_3
        SEL1_3: 3 = struct SEL1_3(bool);
        /// PSEL1_4
        SEL1_4: 4 = struct SEL1_4(bool);
        /// PSEL1_5
        SEL1_5: 5 = struct SEL1_5(bool);
        /// PSEL1_6
        SEL1_6: 6 = struct SEL1_6(bool);
        /// PSEL1_7
        SEL1_7: 7 = struct SEL1_7(bool);
    }
    /// Port Complement Selection
    rw SELC @ 0x16: u16 = 0_0 {
        /// PSELC_0
        SELC_0: 0 = struct SELC_0(bool);
        /// PSELC_1
        SELC_1: 1 = struct SELC_1(bool);
        /// PSELC_2
        SELC_2: 2 = struct SELC_2(bool);
        /// PSELC_3
        SELC_3: 3 = struct SELC_3(bool);
        /// PSELC_4
        SELC_4: 4 = struct SELC_4(bool);
        /// PSELC_5
        SELC_5: 5 = struct SELC_5(bool);
        /// PSELC_6
        SELC_6: 6 = struct SELC_6(bool);
        /// PSELC_7
        SELC_7: 7 = struct SELC_7(bool);
    }
}
