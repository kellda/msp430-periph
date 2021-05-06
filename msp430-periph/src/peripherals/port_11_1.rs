//! Port 11

utils::periph! {
    /// Port 11
    Port11;
    /// Port 11 Input
    rw P11IN @ 0x00: u8 = 0_0 {
        /// P11IN0
        P11IN0: 0 = struct P11IN0(bool);
        /// P11IN1
        P11IN1: 1 = struct P11IN1(bool);
        /// P11IN2
        P11IN2: 2 = struct P11IN2(bool);
        /// P11IN3
        P11IN3: 3 = struct P11IN3(bool);
        /// P11IN4
        P11IN4: 4 = struct P11IN4(bool);
        /// P11IN5
        P11IN5: 5 = struct P11IN5(bool);
        /// P11IN6
        P11IN6: 6 = struct P11IN6(bool);
        /// P11IN7
        P11IN7: 7 = struct P11IN7(bool);
    }
    /// Port 11 Output
    rw P11OUT @ 0x02: u8 = 0_0 {
        /// P11OUT0
        P11OUT0: 0 = struct P11OUT0(bool);
        /// P11OUT1
        P11OUT1: 1 = struct P11OUT1(bool);
        /// P11OUT2
        P11OUT2: 2 = struct P11OUT2(bool);
        /// P11OUT3
        P11OUT3: 3 = struct P11OUT3(bool);
        /// P11OUT4
        P11OUT4: 4 = struct P11OUT4(bool);
        /// P11OUT5
        P11OUT5: 5 = struct P11OUT5(bool);
        /// P11OUT6
        P11OUT6: 6 = struct P11OUT6(bool);
        /// P11OUT7
        P11OUT7: 7 = struct P11OUT7(bool);
    }
    /// Port 11 Direction
    rw P11DIR @ 0x04: u8 = 0_0 {
        /// P11DIR0
        P11DIR0: 0 = struct P11DIR0(bool);
        /// P11DIR1
        P11DIR1: 1 = struct P11DIR1(bool);
        /// P11DIR2
        P11DIR2: 2 = struct P11DIR2(bool);
        /// P11DIR3
        P11DIR3: 3 = struct P11DIR3(bool);
        /// P11DIR4
        P11DIR4: 4 = struct P11DIR4(bool);
        /// P11DIR5
        P11DIR5: 5 = struct P11DIR5(bool);
        /// P11DIR6
        P11DIR6: 6 = struct P11DIR6(bool);
        /// P11DIR7
        P11DIR7: 7 = struct P11DIR7(bool);
    }
    /// Port 11 Resistor Enable
    rw P11REN @ 0x06: u8 = 0_0 {
        /// P11REN0
        P11REN0: 0 = struct P11REN0(bool);
        /// P11REN1
        P11REN1: 1 = struct P11REN1(bool);
        /// P11REN2
        P11REN2: 2 = struct P11REN2(bool);
        /// P11REN3
        P11REN3: 3 = struct P11REN3(bool);
        /// P11REN4
        P11REN4: 4 = struct P11REN4(bool);
        /// P11REN5
        P11REN5: 5 = struct P11REN5(bool);
        /// P11REN6
        P11REN6: 6 = struct P11REN6(bool);
        /// P11REN7
        P11REN7: 7 = struct P11REN7(bool);
    }
    /// Port 11 Drive Strenght
    rw P11DS @ 0x08: u8 = 0_0 {
        /// P11DS0
        P11DS0: 0 = struct P11DS0(bool);
        /// P11DS1
        P11DS1: 1 = struct P11DS1(bool);
        /// P11DS2
        P11DS2: 2 = struct P11DS2(bool);
        /// P11DS3
        P11DS3: 3 = struct P11DS3(bool);
        /// P11DS4
        P11DS4: 4 = struct P11DS4(bool);
        /// P11DS5
        P11DS5: 5 = struct P11DS5(bool);
        /// P11DS6
        P11DS6: 6 = struct P11DS6(bool);
        /// P11DS7
        P11DS7: 7 = struct P11DS7(bool);
    }
    /// Port 11 Selection
    rw P11SEL @ 0x0a: u8 = 0_0 {
        /// P11SEL0
        P11SEL0: 0 = struct P11SEL0(bool);
        /// P11SEL1
        P11SEL1: 1 = struct P11SEL1(bool);
        /// P11SEL2
        P11SEL2: 2 = struct P11SEL2(bool);
        /// P11SEL3
        P11SEL3: 3 = struct P11SEL3(bool);
        /// P11SEL4
        P11SEL4: 4 = struct P11SEL4(bool);
        /// P11SEL5
        P11SEL5: 5 = struct P11SEL5(bool);
        /// P11SEL6
        P11SEL6: 6 = struct P11SEL6(bool);
        /// P11SEL7
        P11SEL7: 7 = struct P11SEL7(bool);
    }
}
