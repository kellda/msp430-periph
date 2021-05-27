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
    /// Port Drive Strenght
    rw DS @ 0x08: u8 = 0_0 {
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
}
