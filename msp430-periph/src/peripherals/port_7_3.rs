//! Port 7

utils::periph! {
    /// Port 7
    Port7;
    /// Port 7 Input
    rw P7IN @ 0x00: u8 = 0_0 {
        /// P7IN0
        P7IN0: 0 = struct P7IN0(bool);
        /// P7IN1
        P7IN1: 1 = struct P7IN1(bool);
        /// P7IN2
        P7IN2: 2 = struct P7IN2(bool);
        /// P7IN3
        P7IN3: 3 = struct P7IN3(bool);
        /// P7IN4
        P7IN4: 4 = struct P7IN4(bool);
        /// P7IN5
        P7IN5: 5 = struct P7IN5(bool);
        /// P7IN6
        P7IN6: 6 = struct P7IN6(bool);
        /// P7IN7
        P7IN7: 7 = struct P7IN7(bool);
    }
    /// Port 7 Output
    rw P7OUT @ 0x02: u8 = 0_0 {
        /// P7OUT0
        P7OUT0: 0 = struct P7OUT0(bool);
        /// P7OUT1
        P7OUT1: 1 = struct P7OUT1(bool);
        /// P7OUT2
        P7OUT2: 2 = struct P7OUT2(bool);
        /// P7OUT3
        P7OUT3: 3 = struct P7OUT3(bool);
        /// P7OUT4
        P7OUT4: 4 = struct P7OUT4(bool);
        /// P7OUT5
        P7OUT5: 5 = struct P7OUT5(bool);
        /// P7OUT6
        P7OUT6: 6 = struct P7OUT6(bool);
        /// P7OUT7
        P7OUT7: 7 = struct P7OUT7(bool);
    }
    /// Port 7 Direction
    rw P7DIR @ 0x04: u8 = 0_0 {
        /// P7DIR0
        P7DIR0: 0 = struct P7DIR0(bool);
        /// P7DIR1
        P7DIR1: 1 = struct P7DIR1(bool);
        /// P7DIR2
        P7DIR2: 2 = struct P7DIR2(bool);
        /// P7DIR3
        P7DIR3: 3 = struct P7DIR3(bool);
        /// P7DIR4
        P7DIR4: 4 = struct P7DIR4(bool);
        /// P7DIR5
        P7DIR5: 5 = struct P7DIR5(bool);
        /// P7DIR6
        P7DIR6: 6 = struct P7DIR6(bool);
        /// P7DIR7
        P7DIR7: 7 = struct P7DIR7(bool);
    }
    /// Port 7 Resistor Enable
    rw P7REN @ 0x06: u8 = 0_0 {
        /// P7REN0
        P7REN0: 0 = struct P7REN0(bool);
        /// P7REN1
        P7REN1: 1 = struct P7REN1(bool);
        /// P7REN2
        P7REN2: 2 = struct P7REN2(bool);
        /// P7REN3
        P7REN3: 3 = struct P7REN3(bool);
        /// P7REN4
        P7REN4: 4 = struct P7REN4(bool);
        /// P7REN5
        P7REN5: 5 = struct P7REN5(bool);
        /// P7REN6
        P7REN6: 6 = struct P7REN6(bool);
        /// P7REN7
        P7REN7: 7 = struct P7REN7(bool);
    }
    /// Port 7 Selection0
    rw P7SEL0 @ 0x0a: u8 = 0_0 {
        /// P7SEL0_0
        P7SEL0_0: 0 = struct P7SEL0_0(bool);
        /// P7SEL0_1
        P7SEL0_1: 1 = struct P7SEL0_1(bool);
        /// P7SEL0_2
        P7SEL0_2: 2 = struct P7SEL0_2(bool);
        /// P7SEL0_3
        P7SEL0_3: 3 = struct P7SEL0_3(bool);
        /// P7SEL0_4
        P7SEL0_4: 4 = struct P7SEL0_4(bool);
        /// P7SEL0_5
        P7SEL0_5: 5 = struct P7SEL0_5(bool);
        /// P7SEL0_6
        P7SEL0_6: 6 = struct P7SEL0_6(bool);
        /// P7SEL0_7
        P7SEL0_7: 7 = struct P7SEL0_7(bool);
    }
    /// Port 7 Selection1
    rw P7SEL1 @ 0x0c: u8 = 0_0 {
        /// P7SEL1_0
        P7SEL1_0: 0 = struct P7SEL1_0(bool);
        /// P7SEL1_1
        P7SEL1_1: 1 = struct P7SEL1_1(bool);
        /// P7SEL1_2
        P7SEL1_2: 2 = struct P7SEL1_2(bool);
        /// P7SEL1_3
        P7SEL1_3: 3 = struct P7SEL1_3(bool);
        /// P7SEL1_4
        P7SEL1_4: 4 = struct P7SEL1_4(bool);
        /// P7SEL1_5
        P7SEL1_5: 5 = struct P7SEL1_5(bool);
        /// P7SEL1_6
        P7SEL1_6: 6 = struct P7SEL1_6(bool);
        /// P7SEL1_7
        P7SEL1_7: 7 = struct P7SEL1_7(bool);
    }
    /// Port 7 Complement Selection
    rw P7SELC @ 0x16: u8 = 0_0 {
        /// P7SELC_0
        P7SELC_0: 0 = struct P7SELC_0(bool);
        /// P7SELC_1
        P7SELC_1: 1 = struct P7SELC_1(bool);
        /// P7SELC_2
        P7SELC_2: 2 = struct P7SELC_2(bool);
        /// P7SELC_3
        P7SELC_3: 3 = struct P7SELC_3(bool);
        /// P7SELC_4
        P7SELC_4: 4 = struct P7SELC_4(bool);
        /// P7SELC_5
        P7SELC_5: 5 = struct P7SELC_5(bool);
        /// P7SELC_6
        P7SELC_6: 6 = struct P7SELC_6(bool);
        /// P7SELC_7
        P7SELC_7: 7 = struct P7SELC_7(bool);
    }
}
