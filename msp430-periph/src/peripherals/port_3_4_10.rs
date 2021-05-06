//! Port 3/4

utils::periph! {
    /// Port 3/4
    Port34;
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
    /// Port 3 Selection 0
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
    /// Port 4 Input
    rw P4IN @ 0x01: u8 = 0_0 {
        /// P4IN0
        P4IN0: 0 = struct P4IN0(bool);
        /// P4IN1
        P4IN1: 1 = struct P4IN1(bool);
        /// P4IN2
        P4IN2: 2 = struct P4IN2(bool);
        /// P4IN3
        P4IN3: 3 = struct P4IN3(bool);
        /// P4IN4
        P4IN4: 4 = struct P4IN4(bool);
        /// P4IN5
        P4IN5: 5 = struct P4IN5(bool);
        /// P4IN6
        P4IN6: 6 = struct P4IN6(bool);
        /// P4IN7
        P4IN7: 7 = struct P4IN7(bool);
    }
    /// Port 4 Output
    rw P4OUT @ 0x03: u8 = 0_0 {
        /// P4OUT0
        P4OUT0: 0 = struct P4OUT0(bool);
        /// P4OUT1
        P4OUT1: 1 = struct P4OUT1(bool);
        /// P4OUT2
        P4OUT2: 2 = struct P4OUT2(bool);
        /// P4OUT3
        P4OUT3: 3 = struct P4OUT3(bool);
        /// P4OUT4
        P4OUT4: 4 = struct P4OUT4(bool);
        /// P4OUT5
        P4OUT5: 5 = struct P4OUT5(bool);
        /// P4OUT6
        P4OUT6: 6 = struct P4OUT6(bool);
        /// P4OUT7
        P4OUT7: 7 = struct P4OUT7(bool);
    }
    /// Port 4 Direction
    rw P4DIR @ 0x05: u8 = 0_0 {
        /// P4DIR0
        P4DIR0: 0 = struct P4DIR0(bool);
        /// P4DIR1
        P4DIR1: 1 = struct P4DIR1(bool);
        /// P4DIR2
        P4DIR2: 2 = struct P4DIR2(bool);
        /// P4DIR3
        P4DIR3: 3 = struct P4DIR3(bool);
        /// P4DIR4
        P4DIR4: 4 = struct P4DIR4(bool);
        /// P4DIR5
        P4DIR5: 5 = struct P4DIR5(bool);
        /// P4DIR6
        P4DIR6: 6 = struct P4DIR6(bool);
        /// P4DIR7
        P4DIR7: 7 = struct P4DIR7(bool);
    }
    /// Port 4 Resistor Enable
    rw P4REN @ 0x07: u8 = 0_0 {
        /// P4REN0
        P4REN0: 0 = struct P4REN0(bool);
        /// P4REN1
        P4REN1: 1 = struct P4REN1(bool);
        /// P4REN2
        P4REN2: 2 = struct P4REN2(bool);
        /// P4REN3
        P4REN3: 3 = struct P4REN3(bool);
        /// P4REN4
        P4REN4: 4 = struct P4REN4(bool);
        /// P4REN5
        P4REN5: 5 = struct P4REN5(bool);
        /// P4REN6
        P4REN6: 6 = struct P4REN6(bool);
        /// P4REN7
        P4REN7: 7 = struct P4REN7(bool);
    }
    /// Port 4 Selection 0
    rw P4SEL0 @ 0x0b: u8 = 0_0 {
        /// P4SEL0_0
        P4SEL0_0: 0 = struct P4SEL0_0(bool);
        /// P4SEL0_1
        P4SEL0_1: 1 = struct P4SEL0_1(bool);
        /// P4SEL0_2
        P4SEL0_2: 2 = struct P4SEL0_2(bool);
        /// P4SEL0_3
        P4SEL0_3: 3 = struct P4SEL0_3(bool);
        /// P4SEL0_4
        P4SEL0_4: 4 = struct P4SEL0_4(bool);
        /// P4SEL0_5
        P4SEL0_5: 5 = struct P4SEL0_5(bool);
        /// P4SEL0_6
        P4SEL0_6: 6 = struct P4SEL0_6(bool);
        /// P4SEL0_7
        P4SEL0_7: 7 = struct P4SEL0_7(bool);
    }
}
