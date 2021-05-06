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
    /// Port 3 Drive Strenght
    rw P3DS @ 0x08: u8 = 0_0 {
        /// P3DS0
        P3DS0: 0 = struct P3DS0(bool);
        /// P3DS1
        P3DS1: 1 = struct P3DS1(bool);
        /// P3DS2
        P3DS2: 2 = struct P3DS2(bool);
        /// P3DS3
        P3DS3: 3 = struct P3DS3(bool);
        /// P3DS4
        P3DS4: 4 = struct P3DS4(bool);
        /// P3DS5
        P3DS5: 5 = struct P3DS5(bool);
        /// P3DS6
        P3DS6: 6 = struct P3DS6(bool);
        /// P3DS7
        P3DS7: 7 = struct P3DS7(bool);
    }
    /// Port 3 Selection
    rw P3SEL @ 0x0a: u8 = 0_0 {
        /// P3SEL0
        P3SEL0: 0 = struct P3SEL0(bool);
        /// P3SEL1
        P3SEL1: 1 = struct P3SEL1(bool);
        /// P3SEL2
        P3SEL2: 2 = struct P3SEL2(bool);
        /// P3SEL3
        P3SEL3: 3 = struct P3SEL3(bool);
        /// P3SEL4
        P3SEL4: 4 = struct P3SEL4(bool);
        /// P3SEL5
        P3SEL5: 5 = struct P3SEL5(bool);
        /// P3SEL6
        P3SEL6: 6 = struct P3SEL6(bool);
        /// P3SEL7
        P3SEL7: 7 = struct P3SEL7(bool);
    }
}
