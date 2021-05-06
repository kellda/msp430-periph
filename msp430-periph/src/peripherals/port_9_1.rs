//! Port 9

utils::periph! {
    /// Port 9
    Port9;
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
    /// Port 9 Drive Strenght
    rw P9DS @ 0x08: u8 = 0_0 {
        /// P9DS0
        P9DS0: 0 = struct P9DS0(bool);
        /// P9DS1
        P9DS1: 1 = struct P9DS1(bool);
        /// P9DS2
        P9DS2: 2 = struct P9DS2(bool);
        /// P9DS3
        P9DS3: 3 = struct P9DS3(bool);
        /// P9DS4
        P9DS4: 4 = struct P9DS4(bool);
        /// P9DS5
        P9DS5: 5 = struct P9DS5(bool);
        /// P9DS6
        P9DS6: 6 = struct P9DS6(bool);
        /// P9DS7
        P9DS7: 7 = struct P9DS7(bool);
    }
    /// Port 9 Selection
    rw P9SEL @ 0x0a: u8 = 0_0 {
        /// P9SEL0
        P9SEL0: 0 = struct P9SEL0(bool);
        /// P9SEL1
        P9SEL1: 1 = struct P9SEL1(bool);
        /// P9SEL2
        P9SEL2: 2 = struct P9SEL2(bool);
        /// P9SEL3
        P9SEL3: 3 = struct P9SEL3(bool);
        /// P9SEL4
        P9SEL4: 4 = struct P9SEL4(bool);
        /// P9SEL5
        P9SEL5: 5 = struct P9SEL5(bool);
        /// P9SEL6
        P9SEL6: 6 = struct P9SEL6(bool);
        /// P9SEL7
        P9SEL7: 7 = struct P9SEL7(bool);
    }
}
