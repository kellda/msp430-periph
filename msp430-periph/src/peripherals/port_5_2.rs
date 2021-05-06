//! Port 5

utils::periph! {
    /// Port 5
    Port5;
    /// Port 5 Input
    rw P5IN @ 0x00: u8 = 0_0 {
        /// P5IN0
        P5IN0: 0 = struct P5IN0(bool);
        /// P5IN1
        P5IN1: 1 = struct P5IN1(bool);
        /// P5IN2
        P5IN2: 2 = struct P5IN2(bool);
        /// P5IN3
        P5IN3: 3 = struct P5IN3(bool);
        /// P5IN4
        P5IN4: 4 = struct P5IN4(bool);
        /// P5IN5
        P5IN5: 5 = struct P5IN5(bool);
        /// P5IN6
        P5IN6: 6 = struct P5IN6(bool);
        /// P5IN7
        P5IN7: 7 = struct P5IN7(bool);
    }
    /// Port 5 Output
    rw P5OUT @ 0x02: u8 = 0_0 {
        /// P5OUT0
        P5OUT0: 0 = struct P5OUT0(bool);
        /// P5OUT1
        P5OUT1: 1 = struct P5OUT1(bool);
        /// P5OUT2
        P5OUT2: 2 = struct P5OUT2(bool);
        /// P5OUT3
        P5OUT3: 3 = struct P5OUT3(bool);
        /// P5OUT4
        P5OUT4: 4 = struct P5OUT4(bool);
        /// P5OUT5
        P5OUT5: 5 = struct P5OUT5(bool);
        /// P5OUT6
        P5OUT6: 6 = struct P5OUT6(bool);
        /// P5OUT7
        P5OUT7: 7 = struct P5OUT7(bool);
    }
    /// Port 5 Direction
    rw P5DIR @ 0x04: u8 = 0_0 {
        /// P5DIR0
        P5DIR0: 0 = struct P5DIR0(bool);
        /// P5DIR1
        P5DIR1: 1 = struct P5DIR1(bool);
        /// P5DIR2
        P5DIR2: 2 = struct P5DIR2(bool);
        /// P5DIR3
        P5DIR3: 3 = struct P5DIR3(bool);
        /// P5DIR4
        P5DIR4: 4 = struct P5DIR4(bool);
        /// P5DIR5
        P5DIR5: 5 = struct P5DIR5(bool);
        /// P5DIR6
        P5DIR6: 6 = struct P5DIR6(bool);
        /// P5DIR7
        P5DIR7: 7 = struct P5DIR7(bool);
    }
    /// Port 5 Resistor Enable
    rw P5REN @ 0x06: u8 = 0_0 {
        /// P5REN0
        P5REN0: 0 = struct P5REN0(bool);
        /// P5REN1
        P5REN1: 1 = struct P5REN1(bool);
        /// P5REN2
        P5REN2: 2 = struct P5REN2(bool);
        /// P5REN3
        P5REN3: 3 = struct P5REN3(bool);
        /// P5REN4
        P5REN4: 4 = struct P5REN4(bool);
        /// P5REN5
        P5REN5: 5 = struct P5REN5(bool);
        /// P5REN6
        P5REN6: 6 = struct P5REN6(bool);
        /// P5REN7
        P5REN7: 7 = struct P5REN7(bool);
    }
    /// Port 5 Drive Strenght
    rw P5DS @ 0x08: u8 = 0_0 {
        /// P5DS0
        P5DS0: 0 = struct P5DS0(bool);
        /// P5DS1
        P5DS1: 1 = struct P5DS1(bool);
        /// P5DS2
        P5DS2: 2 = struct P5DS2(bool);
        /// P5DS3
        P5DS3: 3 = struct P5DS3(bool);
        /// P5DS4
        P5DS4: 4 = struct P5DS4(bool);
        /// P5DS5
        P5DS5: 5 = struct P5DS5(bool);
        /// P5DS6
        P5DS6: 6 = struct P5DS6(bool);
        /// P5DS7
        P5DS7: 7 = struct P5DS7(bool);
    }
    /// Port 5 Selection
    rw P5SEL @ 0x0a: u8 = 0_0 {
        /// P5SEL0
        P5SEL0: 0 = struct P5SEL0(bool);
        /// P5SEL1
        P5SEL1: 1 = struct P5SEL1(bool);
        /// P5SEL2
        P5SEL2: 2 = struct P5SEL2(bool);
        /// P5SEL3
        P5SEL3: 3 = struct P5SEL3(bool);
        /// P5SEL4
        P5SEL4: 4 = struct P5SEL4(bool);
        /// P5SEL5
        P5SEL5: 5 = struct P5SEL5(bool);
        /// P5SEL6
        P5SEL6: 6 = struct P5SEL6(bool);
        /// P5SEL7
        P5SEL7: 7 = struct P5SEL7(bool);
    }
}
