//! Port 7/8

utils::periph! {
    /// Port 7/8
    Port78;
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
    /// Port 7 Selection 0
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
    /// Port 8 Input
    rw P8IN @ 0x01: u8 = 0_0 {
        /// P8IN0
        P8IN0: 0 = struct P8IN0(bool);
        /// P8IN1
        P8IN1: 1 = struct P8IN1(bool);
        /// P8IN2
        P8IN2: 2 = struct P8IN2(bool);
        /// P8IN3
        P8IN3: 3 = struct P8IN3(bool);
        /// P8IN4
        P8IN4: 4 = struct P8IN4(bool);
        /// P8IN5
        P8IN5: 5 = struct P8IN5(bool);
        /// P8IN6
        P8IN6: 6 = struct P8IN6(bool);
        /// P8IN7
        P8IN7: 7 = struct P8IN7(bool);
    }
    /// Port 8 Output
    rw P8OUT @ 0x03: u8 = 0_0 {
        /// P8OUT0
        P8OUT0: 0 = struct P8OUT0(bool);
        /// P8OUT1
        P8OUT1: 1 = struct P8OUT1(bool);
        /// P8OUT2
        P8OUT2: 2 = struct P8OUT2(bool);
        /// P8OUT3
        P8OUT3: 3 = struct P8OUT3(bool);
        /// P8OUT4
        P8OUT4: 4 = struct P8OUT4(bool);
        /// P8OUT5
        P8OUT5: 5 = struct P8OUT5(bool);
        /// P8OUT6
        P8OUT6: 6 = struct P8OUT6(bool);
        /// P8OUT7
        P8OUT7: 7 = struct P8OUT7(bool);
    }
    /// Port 8 Direction
    rw P8DIR @ 0x05: u8 = 0_0 {
        /// P8DIR0
        P8DIR0: 0 = struct P8DIR0(bool);
        /// P8DIR1
        P8DIR1: 1 = struct P8DIR1(bool);
        /// P8DIR2
        P8DIR2: 2 = struct P8DIR2(bool);
        /// P8DIR3
        P8DIR3: 3 = struct P8DIR3(bool);
        /// P8DIR4
        P8DIR4: 4 = struct P8DIR4(bool);
        /// P8DIR5
        P8DIR5: 5 = struct P8DIR5(bool);
        /// P8DIR6
        P8DIR6: 6 = struct P8DIR6(bool);
        /// P8DIR7
        P8DIR7: 7 = struct P8DIR7(bool);
    }
    /// Port 8 Resistor Enable
    rw P8REN @ 0x07: u8 = 0_0 {
        /// P8REN0
        P8REN0: 0 = struct P8REN0(bool);
        /// P8REN1
        P8REN1: 1 = struct P8REN1(bool);
        /// P8REN2
        P8REN2: 2 = struct P8REN2(bool);
        /// P8REN3
        P8REN3: 3 = struct P8REN3(bool);
        /// P8REN4
        P8REN4: 4 = struct P8REN4(bool);
        /// P8REN5
        P8REN5: 5 = struct P8REN5(bool);
        /// P8REN6
        P8REN6: 6 = struct P8REN6(bool);
        /// P8REN7
        P8REN7: 7 = struct P8REN7(bool);
    }
    /// Port 8 Selection 0
    rw P8SEL0 @ 0x0b: u8 = 0_0 {
        /// P8SEL0_0
        P8SEL0_0: 0 = struct P8SEL0_0(bool);
        /// P8SEL0_1
        P8SEL0_1: 1 = struct P8SEL0_1(bool);
        /// P8SEL0_2
        P8SEL0_2: 2 = struct P8SEL0_2(bool);
        /// P8SEL0_3
        P8SEL0_3: 3 = struct P8SEL0_3(bool);
        /// P8SEL0_4
        P8SEL0_4: 4 = struct P8SEL0_4(bool);
        /// P8SEL0_5
        P8SEL0_5: 5 = struct P8SEL0_5(bool);
        /// P8SEL0_6
        P8SEL0_6: 6 = struct P8SEL0_6(bool);
        /// P8SEL0_7
        P8SEL0_7: 7 = struct P8SEL0_7(bool);
    }
}
