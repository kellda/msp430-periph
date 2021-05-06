//! Port 1/2

utils::periph! {
    /// Port 1/2
    Port12;
    /// Port 1 Input
    rw P1IN @ 0x00: u8 = 0_0 {
        /// P1IN0
        P1IN0: 0 = struct P1IN0(bool);
        /// P1IN1
        P1IN1: 1 = struct P1IN1(bool);
        /// P1IN2
        P1IN2: 2 = struct P1IN2(bool);
        /// P1IN3
        P1IN3: 3 = struct P1IN3(bool);
        /// P1IN4
        P1IN4: 4 = struct P1IN4(bool);
        /// P1IN5
        P1IN5: 5 = struct P1IN5(bool);
        /// P1IN6
        P1IN6: 6 = struct P1IN6(bool);
        /// P1IN7
        P1IN7: 7 = struct P1IN7(bool);
    }
    /// Port 1 Output
    rw P1OUT @ 0x02: u8 = 0_0 {
        /// P1OUT0
        P1OUT0: 0 = struct P1OUT0(bool);
        /// P1OUT1
        P1OUT1: 1 = struct P1OUT1(bool);
        /// P1OUT2
        P1OUT2: 2 = struct P1OUT2(bool);
        /// P1OUT3
        P1OUT3: 3 = struct P1OUT3(bool);
        /// P1OUT4
        P1OUT4: 4 = struct P1OUT4(bool);
        /// P1OUT5
        P1OUT5: 5 = struct P1OUT5(bool);
        /// P1OUT6
        P1OUT6: 6 = struct P1OUT6(bool);
        /// P1OUT7
        P1OUT7: 7 = struct P1OUT7(bool);
    }
    /// Port 1 Direction
    rw P1DIR @ 0x04: u8 = 0_0 {
        /// P1DIR0
        P1DIR0: 0 = struct P1DIR0(bool);
        /// P1DIR1
        P1DIR1: 1 = struct P1DIR1(bool);
        /// P1DIR2
        P1DIR2: 2 = struct P1DIR2(bool);
        /// P1DIR3
        P1DIR3: 3 = struct P1DIR3(bool);
        /// P1DIR4
        P1DIR4: 4 = struct P1DIR4(bool);
        /// P1DIR5
        P1DIR5: 5 = struct P1DIR5(bool);
        /// P1DIR6
        P1DIR6: 6 = struct P1DIR6(bool);
        /// P1DIR7
        P1DIR7: 7 = struct P1DIR7(bool);
    }
    /// Port 1 Resistor Enable
    rw P1REN @ 0x06: u8 = 0_0 {
        /// P1REN0
        P1REN0: 0 = struct P1REN0(bool);
        /// P1REN1
        P1REN1: 1 = struct P1REN1(bool);
        /// P1REN2
        P1REN2: 2 = struct P1REN2(bool);
        /// P1REN3
        P1REN3: 3 = struct P1REN3(bool);
        /// P1REN4
        P1REN4: 4 = struct P1REN4(bool);
        /// P1REN5
        P1REN5: 5 = struct P1REN5(bool);
        /// P1REN6
        P1REN6: 6 = struct P1REN6(bool);
        /// P1REN7
        P1REN7: 7 = struct P1REN7(bool);
    }
    /// Port 1 Drive Strenght
    rw P1DS @ 0x08: u8 = 0_0 {
        /// P1DS0
        P1DS0: 0 = struct P1DS0(bool);
        /// P1DS1
        P1DS1: 1 = struct P1DS1(bool);
        /// P1DS2
        P1DS2: 2 = struct P1DS2(bool);
        /// P1DS3
        P1DS3: 3 = struct P1DS3(bool);
        /// P1DS4
        P1DS4: 4 = struct P1DS4(bool);
        /// P1DS5
        P1DS5: 5 = struct P1DS5(bool);
        /// P1DS6
        P1DS6: 6 = struct P1DS6(bool);
        /// P1DS7
        P1DS7: 7 = struct P1DS7(bool);
    }
    /// Port 1 Selection
    rw P1SEL @ 0x0a: u8 = 0_0 {
        /// P1SEL0
        P1SEL0: 0 = struct P1SEL0(bool);
        /// P1SEL1
        P1SEL1: 1 = struct P1SEL1(bool);
        /// P1SEL2
        P1SEL2: 2 = struct P1SEL2(bool);
        /// P1SEL3
        P1SEL3: 3 = struct P1SEL3(bool);
        /// P1SEL4
        P1SEL4: 4 = struct P1SEL4(bool);
        /// P1SEL5
        P1SEL5: 5 = struct P1SEL5(bool);
        /// P1SEL6
        P1SEL6: 6 = struct P1SEL6(bool);
        /// P1SEL7
        P1SEL7: 7 = struct P1SEL7(bool);
    }
    /// Port 1 Interrupt Vector Word
    rw P1IV @ 0x0e: u16 = 0_0 {
        /// Port 1 Interrupt Vector Word
        P1IV: 0..15 = struct P1IVField(u16);
    }
    /// Port 1 Interrupt Edge Select
    rw P1IES @ 0x18: u8 = 0_0 {
        /// P1IES0
        P1IES0: 0 = struct P1IES0(bool);
        /// P1IES1
        P1IES1: 1 = struct P1IES1(bool);
        /// P1IES2
        P1IES2: 2 = struct P1IES2(bool);
        /// P1IES3
        P1IES3: 3 = struct P1IES3(bool);
        /// P1IES4
        P1IES4: 4 = struct P1IES4(bool);
        /// P1IES5
        P1IES5: 5 = struct P1IES5(bool);
        /// P1IES6
        P1IES6: 6 = struct P1IES6(bool);
        /// P1IES7
        P1IES7: 7 = struct P1IES7(bool);
    }
    /// Port 1 Interrupt Enable
    rw P1IE @ 0x1a: u8 = 0_0 {
        /// P1IE0
        P1IE0: 0 = struct P1IE0(bool);
        /// P1IE1
        P1IE1: 1 = struct P1IE1(bool);
        /// P1IE2
        P1IE2: 2 = struct P1IE2(bool);
        /// P1IE3
        P1IE3: 3 = struct P1IE3(bool);
        /// P1IE4
        P1IE4: 4 = struct P1IE4(bool);
        /// P1IE5
        P1IE5: 5 = struct P1IE5(bool);
        /// P1IE6
        P1IE6: 6 = struct P1IE6(bool);
        /// P1IE7
        P1IE7: 7 = struct P1IE7(bool);
    }
    /// Port 1 Interrupt Flag
    rw P1IFG @ 0x1c: u8 = 0_0 {
        /// P1IFG0
        P1IFG0: 0 = struct P1IFG0(bool);
        /// P1IFG1
        P1IFG1: 1 = struct P1IFG1(bool);
        /// P1IFG2
        P1IFG2: 2 = struct P1IFG2(bool);
        /// P1IFG3
        P1IFG3: 3 = struct P1IFG3(bool);
        /// P1IFG4
        P1IFG4: 4 = struct P1IFG4(bool);
        /// P1IFG5
        P1IFG5: 5 = struct P1IFG5(bool);
        /// P1IFG6
        P1IFG6: 6 = struct P1IFG6(bool);
        /// P1IFG7
        P1IFG7: 7 = struct P1IFG7(bool);
    }
    /// Port 2 Input
    rw P2IN @ 0x01: u8 = 0_0 {
        /// P2IN0
        P2IN0: 0 = struct P2IN0(bool);
        /// P2IN1
        P2IN1: 1 = struct P2IN1(bool);
        /// P2IN2
        P2IN2: 2 = struct P2IN2(bool);
        /// P2IN3
        P2IN3: 3 = struct P2IN3(bool);
        /// P2IN4
        P2IN4: 4 = struct P2IN4(bool);
        /// P2IN5
        P2IN5: 5 = struct P2IN5(bool);
        /// P2IN6
        P2IN6: 6 = struct P2IN6(bool);
        /// P2IN7
        P2IN7: 7 = struct P2IN7(bool);
    }
    /// Port 2 Output
    rw P2OUT @ 0x03: u8 = 0_0 {
        /// P2OUT0
        P2OUT0: 0 = struct P2OUT0(bool);
        /// P2OUT1
        P2OUT1: 1 = struct P2OUT1(bool);
        /// P2OUT2
        P2OUT2: 2 = struct P2OUT2(bool);
        /// P2OUT3
        P2OUT3: 3 = struct P2OUT3(bool);
        /// P2OUT4
        P2OUT4: 4 = struct P2OUT4(bool);
        /// P2OUT5
        P2OUT5: 5 = struct P2OUT5(bool);
        /// P2OUT6
        P2OUT6: 6 = struct P2OUT6(bool);
        /// P2OUT7
        P2OUT7: 7 = struct P2OUT7(bool);
    }
    /// Port 2 Direction
    rw P2DIR @ 0x05: u8 = 0_0 {
        /// P2DIR0
        P2DIR0: 0 = struct P2DIR0(bool);
        /// P2DIR1
        P2DIR1: 1 = struct P2DIR1(bool);
        /// P2DIR2
        P2DIR2: 2 = struct P2DIR2(bool);
        /// P2DIR3
        P2DIR3: 3 = struct P2DIR3(bool);
        /// P2DIR4
        P2DIR4: 4 = struct P2DIR4(bool);
        /// P2DIR5
        P2DIR5: 5 = struct P2DIR5(bool);
        /// P2DIR6
        P2DIR6: 6 = struct P2DIR6(bool);
        /// P2DIR7
        P2DIR7: 7 = struct P2DIR7(bool);
    }
    /// Port 2 Resistor Enable
    rw P2REN @ 0x07: u8 = 0_0 {
        /// P2REN0
        P2REN0: 0 = struct P2REN0(bool);
        /// P2REN1
        P2REN1: 1 = struct P2REN1(bool);
        /// P2REN2
        P2REN2: 2 = struct P2REN2(bool);
        /// P2REN3
        P2REN3: 3 = struct P2REN3(bool);
        /// P2REN4
        P2REN4: 4 = struct P2REN4(bool);
        /// P2REN5
        P2REN5: 5 = struct P2REN5(bool);
        /// P2REN6
        P2REN6: 6 = struct P2REN6(bool);
        /// P2REN7
        P2REN7: 7 = struct P2REN7(bool);
    }
    /// Port 2 Drive Strenght
    rw P2DS @ 0x09: u8 = 0_0 {
        /// P2DS0
        P2DS0: 0 = struct P2DS0(bool);
        /// P2DS1
        P2DS1: 1 = struct P2DS1(bool);
        /// P2DS2
        P2DS2: 2 = struct P2DS2(bool);
        /// P2DS3
        P2DS3: 3 = struct P2DS3(bool);
        /// P2DS4
        P2DS4: 4 = struct P2DS4(bool);
        /// P2DS5
        P2DS5: 5 = struct P2DS5(bool);
        /// P2DS6
        P2DS6: 6 = struct P2DS6(bool);
        /// P2DS7
        P2DS7: 7 = struct P2DS7(bool);
    }
    /// Port 2 Selection
    rw P2SEL @ 0x0b: u8 = 0_0 {
        /// P2SEL0
        P2SEL0: 0 = struct P2SEL0(bool);
        /// P2SEL1
        P2SEL1: 1 = struct P2SEL1(bool);
        /// P2SEL2
        P2SEL2: 2 = struct P2SEL2(bool);
        /// P2SEL3
        P2SEL3: 3 = struct P2SEL3(bool);
        /// P2SEL4
        P2SEL4: 4 = struct P2SEL4(bool);
        /// P2SEL5
        P2SEL5: 5 = struct P2SEL5(bool);
        /// P2SEL6
        P2SEL6: 6 = struct P2SEL6(bool);
        /// P2SEL7
        P2SEL7: 7 = struct P2SEL7(bool);
    }
    /// Port 2 Interrupt Vector Word
    rw P2IV @ 0x1e: u16 = 0_0 {
        /// Port 2 Interrupt Vector Word
        P2IV: 0..15 = struct P2IVField(u16);
    }
    /// Port 2 Interrupt Edge Select
    rw P2IES @ 0x19: u8 = 0_0 {
        /// P2IES0
        P2IES0: 0 = struct P2IES0(bool);
        /// P2IES1
        P2IES1: 1 = struct P2IES1(bool);
        /// P2IES2
        P2IES2: 2 = struct P2IES2(bool);
        /// P2IES3
        P2IES3: 3 = struct P2IES3(bool);
        /// P2IES4
        P2IES4: 4 = struct P2IES4(bool);
        /// P2IES5
        P2IES5: 5 = struct P2IES5(bool);
        /// P2IES6
        P2IES6: 6 = struct P2IES6(bool);
        /// P2IES7
        P2IES7: 7 = struct P2IES7(bool);
    }
    /// Port 2 Interrupt Enable
    rw P2IE @ 0x1b: u8 = 0_0 {
        /// P2IE0
        P2IE0: 0 = struct P2IE0(bool);
        /// P2IE1
        P2IE1: 1 = struct P2IE1(bool);
        /// P2IE2
        P2IE2: 2 = struct P2IE2(bool);
        /// P2IE3
        P2IE3: 3 = struct P2IE3(bool);
        /// P2IE4
        P2IE4: 4 = struct P2IE4(bool);
        /// P2IE5
        P2IE5: 5 = struct P2IE5(bool);
        /// P2IE6
        P2IE6: 6 = struct P2IE6(bool);
        /// P2IE7
        P2IE7: 7 = struct P2IE7(bool);
    }
    /// Port 2 Interrupt Flag
    rw P2IFG @ 0x1d: u8 = 0_0 {
        /// P2IFG0
        P2IFG0: 0 = struct P2IFG0(bool);
        /// P2IFG1
        P2IFG1: 1 = struct P2IFG1(bool);
        /// P2IFG2
        P2IFG2: 2 = struct P2IFG2(bool);
        /// P2IFG3
        P2IFG3: 3 = struct P2IFG3(bool);
        /// P2IFG4
        P2IFG4: 4 = struct P2IFG4(bool);
        /// P2IFG5
        P2IFG5: 5 = struct P2IFG5(bool);
        /// P2IFG6
        P2IFG6: 6 = struct P2IFG6(bool);
        /// P2IFG7
        P2IFG7: 7 = struct P2IFG7(bool);
    }
}
