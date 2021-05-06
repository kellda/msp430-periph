//! Port 5/6

utils::periph! {
    /// Port 5/6
    Port56;
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
    /// Port 6 Input
    rw P6IN @ 0x01: u8 = 0_0 {
        /// P6IN0
        P6IN0: 0 = struct P6IN0(bool);
        /// P6IN1
        P6IN1: 1 = struct P6IN1(bool);
        /// P6IN2
        P6IN2: 2 = struct P6IN2(bool);
        /// P6IN3
        P6IN3: 3 = struct P6IN3(bool);
        /// P6IN4
        P6IN4: 4 = struct P6IN4(bool);
        /// P6IN5
        P6IN5: 5 = struct P6IN5(bool);
        /// P6IN6
        P6IN6: 6 = struct P6IN6(bool);
        /// P6IN7
        P6IN7: 7 = struct P6IN7(bool);
    }
    /// Port 6 Output
    rw P6OUT @ 0x03: u8 = 0_0 {
        /// P6OUT0
        P6OUT0: 0 = struct P6OUT0(bool);
        /// P6OUT1
        P6OUT1: 1 = struct P6OUT1(bool);
        /// P6OUT2
        P6OUT2: 2 = struct P6OUT2(bool);
        /// P6OUT3
        P6OUT3: 3 = struct P6OUT3(bool);
        /// P6OUT4
        P6OUT4: 4 = struct P6OUT4(bool);
        /// P6OUT5
        P6OUT5: 5 = struct P6OUT5(bool);
        /// P6OUT6
        P6OUT6: 6 = struct P6OUT6(bool);
        /// P6OUT7
        P6OUT7: 7 = struct P6OUT7(bool);
    }
    /// Port 6 Direction
    rw P6DIR @ 0x05: u8 = 0_0 {
        /// P6DIR0
        P6DIR0: 0 = struct P6DIR0(bool);
        /// P6DIR1
        P6DIR1: 1 = struct P6DIR1(bool);
        /// P6DIR2
        P6DIR2: 2 = struct P6DIR2(bool);
        /// P6DIR3
        P6DIR3: 3 = struct P6DIR3(bool);
        /// P6DIR4
        P6DIR4: 4 = struct P6DIR4(bool);
        /// P6DIR5
        P6DIR5: 5 = struct P6DIR5(bool);
        /// P6DIR6
        P6DIR6: 6 = struct P6DIR6(bool);
        /// P6DIR7
        P6DIR7: 7 = struct P6DIR7(bool);
    }
    /// Port 6 Resistor Enable
    rw P6REN @ 0x07: u8 = 0_0 {
        /// P6REN0
        P6REN0: 0 = struct P6REN0(bool);
        /// P6REN1
        P6REN1: 1 = struct P6REN1(bool);
        /// P6REN2
        P6REN2: 2 = struct P6REN2(bool);
        /// P6REN3
        P6REN3: 3 = struct P6REN3(bool);
        /// P6REN4
        P6REN4: 4 = struct P6REN4(bool);
        /// P6REN5
        P6REN5: 5 = struct P6REN5(bool);
        /// P6REN6
        P6REN6: 6 = struct P6REN6(bool);
        /// P6REN7
        P6REN7: 7 = struct P6REN7(bool);
    }
    /// Port 6 Drive Strenght
    rw P6DS @ 0x09: u8 = 0_0 {
        /// P6DS0
        P6DS0: 0 = struct P6DS0(bool);
        /// P6DS1
        P6DS1: 1 = struct P6DS1(bool);
        /// P6DS2
        P6DS2: 2 = struct P6DS2(bool);
        /// P6DS3
        P6DS3: 3 = struct P6DS3(bool);
        /// P6DS4
        P6DS4: 4 = struct P6DS4(bool);
        /// P6DS5
        P6DS5: 5 = struct P6DS5(bool);
        /// P6DS6
        P6DS6: 6 = struct P6DS6(bool);
        /// P6DS7
        P6DS7: 7 = struct P6DS7(bool);
    }
    /// Port 6 Selection
    rw P6SEL @ 0x0b: u8 = 0_0 {
        /// P6SEL0
        P6SEL0: 0 = struct P6SEL0(bool);
        /// P6SEL1
        P6SEL1: 1 = struct P6SEL1(bool);
        /// P6SEL2
        P6SEL2: 2 = struct P6SEL2(bool);
        /// P6SEL3
        P6SEL3: 3 = struct P6SEL3(bool);
        /// P6SEL4
        P6SEL4: 4 = struct P6SEL4(bool);
        /// P6SEL5
        P6SEL5: 5 = struct P6SEL5(bool);
        /// P6SEL6
        P6SEL6: 6 = struct P6SEL6(bool);
        /// P6SEL7
        P6SEL7: 7 = struct P6SEL7(bool);
    }
}
