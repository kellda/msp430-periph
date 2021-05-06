//! Port 9/10

utils::periph! {
    /// Port 9/10
    Port910;
    /// Port 9 Input
    rw P9IN @ 0x00: u8 = 0_0 {
        /// P9IN0
        P9IN0: 0 = struct P9IN0(bool);
        /// P9IN1
        P9IN1: 1 = struct P9IN1(bool);
        /// P9IN2
        P9IN2: 2 = struct P9IN2(bool);
        /// P9IN3
        P9IN3: 3 = struct P9IN3(bool);
        /// P9IN4
        P9IN4: 4 = struct P9IN4(bool);
        /// P9IN5
        P9IN5: 5 = struct P9IN5(bool);
        /// P9IN6
        P9IN6: 6 = struct P9IN6(bool);
        /// P9IN7
        P9IN7: 7 = struct P9IN7(bool);
    }
    /// Port 9 Output
    rw P9OUT @ 0x02: u8 = 0_0 {
        /// P9OUT0
        P9OUT0: 0 = struct P9OUT0(bool);
        /// P9OUT1
        P9OUT1: 1 = struct P9OUT1(bool);
        /// P9OUT2
        P9OUT2: 2 = struct P9OUT2(bool);
        /// P9OUT3
        P9OUT3: 3 = struct P9OUT3(bool);
        /// P9OUT4
        P9OUT4: 4 = struct P9OUT4(bool);
        /// P9OUT5
        P9OUT5: 5 = struct P9OUT5(bool);
        /// P9OUT6
        P9OUT6: 6 = struct P9OUT6(bool);
        /// P9OUT7
        P9OUT7: 7 = struct P9OUT7(bool);
    }
    /// Port 9 Direction
    rw P9DIR @ 0x04: u8 = 0_0 {
        /// P9DIR0
        P9DIR0: 0 = struct P9DIR0(bool);
        /// P9DIR1
        P9DIR1: 1 = struct P9DIR1(bool);
        /// P9DIR2
        P9DIR2: 2 = struct P9DIR2(bool);
        /// P9DIR3
        P9DIR3: 3 = struct P9DIR3(bool);
        /// P9DIR4
        P9DIR4: 4 = struct P9DIR4(bool);
        /// P9DIR5
        P9DIR5: 5 = struct P9DIR5(bool);
        /// P9DIR6
        P9DIR6: 6 = struct P9DIR6(bool);
        /// P9DIR7
        P9DIR7: 7 = struct P9DIR7(bool);
    }
    /// Port 9 Resistor Enable
    rw P9REN @ 0x06: u8 = 0_0 {
        /// P9REN0
        P9REN0: 0 = struct P9REN0(bool);
        /// P9REN1
        P9REN1: 1 = struct P9REN1(bool);
        /// P9REN2
        P9REN2: 2 = struct P9REN2(bool);
        /// P9REN3
        P9REN3: 3 = struct P9REN3(bool);
        /// P9REN4
        P9REN4: 4 = struct P9REN4(bool);
        /// P9REN5
        P9REN5: 5 = struct P9REN5(bool);
        /// P9REN6
        P9REN6: 6 = struct P9REN6(bool);
        /// P9REN7
        P9REN7: 7 = struct P9REN7(bool);
    }
    /// Port 9 Selection 0
    rw P9SEL0 @ 0x0a: u8 = 0_0 {
        /// P9SEL0_0
        P9SEL0_0: 0 = struct P9SEL0_0(bool);
        /// P9SEL0_1
        P9SEL0_1: 1 = struct P9SEL0_1(bool);
        /// P9SEL0_2
        P9SEL0_2: 2 = struct P9SEL0_2(bool);
        /// P9SEL0_3
        P9SEL0_3: 3 = struct P9SEL0_3(bool);
        /// P9SEL0_4
        P9SEL0_4: 4 = struct P9SEL0_4(bool);
        /// P9SEL0_5
        P9SEL0_5: 5 = struct P9SEL0_5(bool);
        /// P9SEL0_6
        P9SEL0_6: 6 = struct P9SEL0_6(bool);
        /// P9SEL0_7
        P9SEL0_7: 7 = struct P9SEL0_7(bool);
    }
    /// Port 9 Selection 1
    rw P9SEL1 @ 0x0c: u8 = 0_0 {
        /// P9SEL1_0
        P9SEL1_0: 0 = struct P9SEL1_0(bool);
        /// P9SEL1_1
        P9SEL1_1: 1 = struct P9SEL1_1(bool);
        /// P9SEL1_2
        P9SEL1_2: 2 = struct P9SEL1_2(bool);
        /// P9SEL1_3
        P9SEL1_3: 3 = struct P9SEL1_3(bool);
        /// P9SEL1_4
        P9SEL1_4: 4 = struct P9SEL1_4(bool);
        /// P9SEL1_5
        P9SEL1_5: 5 = struct P9SEL1_5(bool);
        /// P9SEL1_6
        P9SEL1_6: 6 = struct P9SEL1_6(bool);
        /// P9SEL1_7
        P9SEL1_7: 7 = struct P9SEL1_7(bool);
    }
    /// Port 9 Complement Selection
    rw P9SELC @ 0x16: u8 = 0_0 {
        /// P9SELC_0
        P9SELC_0: 0 = struct P9SELC_0(bool);
        /// P9SELC_1
        P9SELC_1: 1 = struct P9SELC_1(bool);
        /// P9SELC_2
        P9SELC_2: 2 = struct P9SELC_2(bool);
        /// P9SELC_3
        P9SELC_3: 3 = struct P9SELC_3(bool);
        /// P9SELC_4
        P9SELC_4: 4 = struct P9SELC_4(bool);
        /// P9SELC_5
        P9SELC_5: 5 = struct P9SELC_5(bool);
        /// P9SELC_6
        P9SELC_6: 6 = struct P9SELC_6(bool);
        /// P9SELC_7
        P9SELC_7: 7 = struct P9SELC_7(bool);
    }
    /// Port 10 Input
    rw P10IN @ 0x01: u8 = 0_0 {
        /// P10IN0
        P10IN0: 0 = struct P10IN0(bool);
        /// P10IN1
        P10IN1: 1 = struct P10IN1(bool);
        /// P10IN2
        P10IN2: 2 = struct P10IN2(bool);
        /// P10IN3
        P10IN3: 3 = struct P10IN3(bool);
        /// P10IN4
        P10IN4: 4 = struct P10IN4(bool);
        /// P10IN5
        P10IN5: 5 = struct P10IN5(bool);
        /// P10IN6
        P10IN6: 6 = struct P10IN6(bool);
        /// P10IN7
        P10IN7: 7 = struct P10IN7(bool);
    }
    /// Port 10 Output
    rw P10OUT @ 0x03: u8 = 0_0 {
        /// P10OUT0
        P10OUT0: 0 = struct P10OUT0(bool);
        /// P10OUT1
        P10OUT1: 1 = struct P10OUT1(bool);
        /// P10OUT2
        P10OUT2: 2 = struct P10OUT2(bool);
        /// P10OUT3
        P10OUT3: 3 = struct P10OUT3(bool);
        /// P10OUT4
        P10OUT4: 4 = struct P10OUT4(bool);
        /// P10OUT5
        P10OUT5: 5 = struct P10OUT5(bool);
        /// P10OUT6
        P10OUT6: 6 = struct P10OUT6(bool);
        /// P10OUT7
        P10OUT7: 7 = struct P10OUT7(bool);
    }
    /// Port 10 Direction
    rw P10DIR @ 0x05: u8 = 0_0 {
        /// P10DIR0
        P10DIR0: 0 = struct P10DIR0(bool);
        /// P10DIR1
        P10DIR1: 1 = struct P10DIR1(bool);
        /// P10DIR2
        P10DIR2: 2 = struct P10DIR2(bool);
        /// P10DIR3
        P10DIR3: 3 = struct P10DIR3(bool);
        /// P10DIR4
        P10DIR4: 4 = struct P10DIR4(bool);
        /// P10DIR5
        P10DIR5: 5 = struct P10DIR5(bool);
        /// P10DIR6
        P10DIR6: 6 = struct P10DIR6(bool);
        /// P10DIR7
        P10DIR7: 7 = struct P10DIR7(bool);
    }
    /// Port 10 Resistor Enable
    rw P10REN @ 0x07: u8 = 0_0 {
        /// P10REN0
        P10REN0: 0 = struct P10REN0(bool);
        /// P10REN1
        P10REN1: 1 = struct P10REN1(bool);
        /// P10REN2
        P10REN2: 2 = struct P10REN2(bool);
        /// P10REN3
        P10REN3: 3 = struct P10REN3(bool);
        /// P10REN4
        P10REN4: 4 = struct P10REN4(bool);
        /// P10REN5
        P10REN5: 5 = struct P10REN5(bool);
        /// P10REN6
        P10REN6: 6 = struct P10REN6(bool);
        /// P10REN7
        P10REN7: 7 = struct P10REN7(bool);
    }
    /// Port 10 Selection 0
    rw P10SEL0 @ 0x0b: u8 = 0_0 {
        /// P10SEL0_0
        P10SEL0_0: 0 = struct P10SEL0_0(bool);
        /// P10SEL0_1
        P10SEL0_1: 1 = struct P10SEL0_1(bool);
        /// P10SEL0_2
        P10SEL0_2: 2 = struct P10SEL0_2(bool);
        /// P10SEL0_3
        P10SEL0_3: 3 = struct P10SEL0_3(bool);
        /// P10SEL0_4
        P10SEL0_4: 4 = struct P10SEL0_4(bool);
        /// P10SEL0_5
        P10SEL0_5: 5 = struct P10SEL0_5(bool);
        /// P10SEL0_6
        P10SEL0_6: 6 = struct P10SEL0_6(bool);
        /// P10SEL0_7
        P10SEL0_7: 7 = struct P10SEL0_7(bool);
    }
    /// Port 10 Selection 1
    rw P10SEL1 @ 0x0d: u8 = 0_0 {
        /// P10SEL1_0
        P10SEL1_0: 0 = struct P10SEL1_0(bool);
        /// P10SEL1_1
        P10SEL1_1: 1 = struct P10SEL1_1(bool);
        /// P10SEL1_2
        P10SEL1_2: 2 = struct P10SEL1_2(bool);
        /// P10SEL1_3
        P10SEL1_3: 3 = struct P10SEL1_3(bool);
        /// P10SEL1_4
        P10SEL1_4: 4 = struct P10SEL1_4(bool);
        /// P10SEL1_5
        P10SEL1_5: 5 = struct P10SEL1_5(bool);
        /// P10SEL1_6
        P10SEL1_6: 6 = struct P10SEL1_6(bool);
        /// P10SEL1_7
        P10SEL1_7: 7 = struct P10SEL1_7(bool);
    }
    /// Port 10 Complement Selection
    rw P10SELC @ 0x17: u8 = 0_0 {
        /// P10SELC_0
        P10SELC_0: 0 = struct P10SELC_0(bool);
        /// P10SELC_1
        P10SELC_1: 1 = struct P10SELC_1(bool);
        /// P10SELC_2
        P10SELC_2: 2 = struct P10SELC_2(bool);
        /// P10SELC_3
        P10SELC_3: 3 = struct P10SELC_3(bool);
        /// P10SELC_4
        P10SELC_4: 4 = struct P10SELC_4(bool);
        /// P10SELC_5
        P10SELC_5: 5 = struct P10SELC_5(bool);
        /// P10SELC_6
        P10SELC_6: 6 = struct P10SELC_6(bool);
        /// P10SELC_7
        P10SELC_7: 7 = struct P10SELC_7(bool);
    }
}
