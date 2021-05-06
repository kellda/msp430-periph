//! Port 3

utils::periph! {
    /// Port 3
    Port3;
    /// Port 3 Input
    rw P3IN @ 0x00: u8 = 0_0 {
        /// P3IN0
        P3IN0: 0 = struct P3IN0(bool);
        /// P3IN1
        P3IN1: 1 = struct P3IN1(bool);
        /// P3IN2
        P3IN2: 2 = struct P3IN2(bool);
        /// P3IN3
        P3IN3: 3 = struct P3IN3(bool);
        /// P3IN4
        P3IN4: 4 = struct P3IN4(bool);
        /// P3IN5
        P3IN5: 5 = struct P3IN5(bool);
        /// P3IN6
        P3IN6: 6 = struct P3IN6(bool);
        /// P3IN7
        P3IN7: 7 = struct P3IN7(bool);
    }
    /// Port 3 Output
    rw P3OUT @ 0x02: u8 = 0_0 {
        /// P3OUT0
        P3OUT0: 0 = struct P3OUT0(bool);
        /// P3OUT1
        P3OUT1: 1 = struct P3OUT1(bool);
        /// P3OUT2
        P3OUT2: 2 = struct P3OUT2(bool);
        /// P3OUT3
        P3OUT3: 3 = struct P3OUT3(bool);
        /// P3OUT4
        P3OUT4: 4 = struct P3OUT4(bool);
        /// P3OUT5
        P3OUT5: 5 = struct P3OUT5(bool);
        /// P3OUT6
        P3OUT6: 6 = struct P3OUT6(bool);
        /// P3OUT7
        P3OUT7: 7 = struct P3OUT7(bool);
    }
    /// Port 3 Direction
    rw P3DIR @ 0x04: u8 = 0_0 {
        /// P3DIR0
        P3DIR0: 0 = struct P3DIR0(bool);
        /// P3DIR1
        P3DIR1: 1 = struct P3DIR1(bool);
        /// P3DIR2
        P3DIR2: 2 = struct P3DIR2(bool);
        /// P3DIR3
        P3DIR3: 3 = struct P3DIR3(bool);
        /// P3DIR4
        P3DIR4: 4 = struct P3DIR4(bool);
        /// P3DIR5
        P3DIR5: 5 = struct P3DIR5(bool);
        /// P3DIR6
        P3DIR6: 6 = struct P3DIR6(bool);
        /// P3DIR7
        P3DIR7: 7 = struct P3DIR7(bool);
    }
    /// Port 3 Resistor Enable
    rw P3REN @ 0x06: u8 = 0_0 {
        /// P3REN0
        P3REN0: 0 = struct P3REN0(bool);
        /// P3REN1
        P3REN1: 1 = struct P3REN1(bool);
        /// P3REN2
        P3REN2: 2 = struct P3REN2(bool);
        /// P3REN3
        P3REN3: 3 = struct P3REN3(bool);
        /// P3REN4
        P3REN4: 4 = struct P3REN4(bool);
        /// P3REN5
        P3REN5: 5 = struct P3REN5(bool);
        /// P3REN6
        P3REN6: 6 = struct P3REN6(bool);
        /// P3REN7
        P3REN7: 7 = struct P3REN7(bool);
    }
    /// Port 3 Selection0
    rw P3SEL0 @ 0x0a: u8 = 0_0 {
        /// P3SEL0_0
        P3SEL0_0: 0 = struct P3SEL0_0(bool);
        /// P3SEL0_1
        P3SEL0_1: 1 = struct P3SEL0_1(bool);
        /// P3SEL0_2
        P3SEL0_2: 2 = struct P3SEL0_2(bool);
        /// P3SEL0_3
        P3SEL0_3: 3 = struct P3SEL0_3(bool);
        /// P3SEL0_4
        P3SEL0_4: 4 = struct P3SEL0_4(bool);
        /// P3SEL0_5
        P3SEL0_5: 5 = struct P3SEL0_5(bool);
        /// P3SEL0_6
        P3SEL0_6: 6 = struct P3SEL0_6(bool);
        /// P3SEL0_7
        P3SEL0_7: 7 = struct P3SEL0_7(bool);
    }
    /// Port 3 Selection1
    rw P3SEL1 @ 0x0c: u8 = 0_0 {
        /// P3SEL1_0
        P3SEL1_0: 0 = struct P3SEL1_0(bool);
        /// P3SEL1_1
        P3SEL1_1: 1 = struct P3SEL1_1(bool);
        /// P3SEL1_2
        P3SEL1_2: 2 = struct P3SEL1_2(bool);
        /// P3SEL1_3
        P3SEL1_3: 3 = struct P3SEL1_3(bool);
        /// P3SEL1_4
        P3SEL1_4: 4 = struct P3SEL1_4(bool);
        /// P3SEL1_5
        P3SEL1_5: 5 = struct P3SEL1_5(bool);
        /// P3SEL1_6
        P3SEL1_6: 6 = struct P3SEL1_6(bool);
        /// P3SEL1_7
        P3SEL1_7: 7 = struct P3SEL1_7(bool);
    }
}
