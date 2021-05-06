//! Port 1/2

utils::periph! {
    /// Port 1/2
    Port12;
    /// Port 1 Input
    rw P1IN @ 0x00: u8 = 0_0 {
        /// P0
        P1IN_P0: 0 = struct P1IN_P0(bool);
        /// P1
        P1IN_P1: 1 = struct P1IN_P1(bool);
        /// P2
        P1IN_P2: 2 = struct P1IN_P2(bool);
        /// P3
        P1IN_P3: 3 = struct P1IN_P3(bool);
        /// P4
        P1IN_P4: 4 = struct P1IN_P4(bool);
        /// P5
        P1IN_P5: 5 = struct P1IN_P5(bool);
        /// P6
        P1IN_P6: 6 = struct P1IN_P6(bool);
        /// P7
        P1IN_P7: 7 = struct P1IN_P7(bool);
    }
    /// Port 1 Output
    rw P1OUT @ 0x01: u8 = 0_0 {
        /// P0
        P1OUT_P0: 0 = struct P1OUT_P0(bool);
        /// P1
        P1OUT_P1: 1 = struct P1OUT_P1(bool);
        /// P2
        P1OUT_P2: 2 = struct P1OUT_P2(bool);
        /// P3
        P1OUT_P3: 3 = struct P1OUT_P3(bool);
        /// P4
        P1OUT_P4: 4 = struct P1OUT_P4(bool);
        /// P5
        P1OUT_P5: 5 = struct P1OUT_P5(bool);
        /// P6
        P1OUT_P6: 6 = struct P1OUT_P6(bool);
        /// P7
        P1OUT_P7: 7 = struct P1OUT_P7(bool);
    }
    /// Port 1 Direction
    rw P1DIR @ 0x02: u8 = 0_0 {
        /// P0
        P1DIR_P0: 0 = struct P1DIR_P0(bool);
        /// P1
        P1DIR_P1: 1 = struct P1DIR_P1(bool);
        /// P2
        P1DIR_P2: 2 = struct P1DIR_P2(bool);
        /// P3
        P1DIR_P3: 3 = struct P1DIR_P3(bool);
        /// P4
        P1DIR_P4: 4 = struct P1DIR_P4(bool);
        /// P5
        P1DIR_P5: 5 = struct P1DIR_P5(bool);
        /// P6
        P1DIR_P6: 6 = struct P1DIR_P6(bool);
        /// P7
        P1DIR_P7: 7 = struct P1DIR_P7(bool);
    }
    /// Port 1 Interrupt Flag
    rw P1IFG @ 0x03: u8 = 0_0 {
        /// P0
        P1IFG_P0: 0 = struct P1IFG_P0(bool);
        /// P1
        P1IFG_P1: 1 = struct P1IFG_P1(bool);
        /// P2
        P1IFG_P2: 2 = struct P1IFG_P2(bool);
        /// P3
        P1IFG_P3: 3 = struct P1IFG_P3(bool);
        /// P4
        P1IFG_P4: 4 = struct P1IFG_P4(bool);
        /// P5
        P1IFG_P5: 5 = struct P1IFG_P5(bool);
        /// P6
        P1IFG_P6: 6 = struct P1IFG_P6(bool);
        /// P7
        P1IFG_P7: 7 = struct P1IFG_P7(bool);
    }
    /// Port 1 Interrupt Edge Select
    rw P1IES @ 0x04: u8 = 0_0 {
        /// P0
        P1IES_P0: 0 = struct P1IES_P0(bool);
        /// P1
        P1IES_P1: 1 = struct P1IES_P1(bool);
        /// P2
        P1IES_P2: 2 = struct P1IES_P2(bool);
        /// P3
        P1IES_P3: 3 = struct P1IES_P3(bool);
        /// P4
        P1IES_P4: 4 = struct P1IES_P4(bool);
        /// P5
        P1IES_P5: 5 = struct P1IES_P5(bool);
        /// P6
        P1IES_P6: 6 = struct P1IES_P6(bool);
        /// P7
        P1IES_P7: 7 = struct P1IES_P7(bool);
    }
    /// Port 1 Interrupt Enable
    rw P1IE @ 0x05: u8 = 0_0 {
        /// P0
        P1IE_P0: 0 = struct P1IE_P0(bool);
        /// P1
        P1IE_P1: 1 = struct P1IE_P1(bool);
        /// P2
        P1IE_P2: 2 = struct P1IE_P2(bool);
        /// P3
        P1IE_P3: 3 = struct P1IE_P3(bool);
        /// P4
        P1IE_P4: 4 = struct P1IE_P4(bool);
        /// P5
        P1IE_P5: 5 = struct P1IE_P5(bool);
        /// P6
        P1IE_P6: 6 = struct P1IE_P6(bool);
        /// P7
        P1IE_P7: 7 = struct P1IE_P7(bool);
    }
    /// Port 1 Selection
    rw P1SEL @ 0x06: u8 = 0_0 {
        /// P0
        P1SEL_P0: 0 = struct P1SEL_P0(bool);
        /// P1
        P1SEL_P1: 1 = struct P1SEL_P1(bool);
        /// P2
        P1SEL_P2: 2 = struct P1SEL_P2(bool);
        /// P3
        P1SEL_P3: 3 = struct P1SEL_P3(bool);
        /// P4
        P1SEL_P4: 4 = struct P1SEL_P4(bool);
        /// P5
        P1SEL_P5: 5 = struct P1SEL_P5(bool);
        /// P6
        P1SEL_P6: 6 = struct P1SEL_P6(bool);
        /// P7
        P1SEL_P7: 7 = struct P1SEL_P7(bool);
    }
    /// Port 1 Selection 2
    rw P1SEL2 @ 0x21: u8 = 0_0 {
        /// P0
        P1SEL2_P0: 0 = struct P1SEL2_P0(bool);
        /// P1
        P1SEL2_P1: 1 = struct P1SEL2_P1(bool);
        /// P2
        P1SEL2_P2: 2 = struct P1SEL2_P2(bool);
        /// P3
        P1SEL2_P3: 3 = struct P1SEL2_P3(bool);
        /// P4
        P1SEL2_P4: 4 = struct P1SEL2_P4(bool);
        /// P5
        P1SEL2_P5: 5 = struct P1SEL2_P5(bool);
        /// P6
        P1SEL2_P6: 6 = struct P1SEL2_P6(bool);
        /// P7
        P1SEL2_P7: 7 = struct P1SEL2_P7(bool);
    }
    /// Port 1 Resistor Enable
    rw P1REN @ 0x07: u8 = 0_0 {
        /// P0
        P1REN_P0: 0 = struct P1REN_P0(bool);
        /// P1
        P1REN_P1: 1 = struct P1REN_P1(bool);
        /// P2
        P1REN_P2: 2 = struct P1REN_P2(bool);
        /// P3
        P1REN_P3: 3 = struct P1REN_P3(bool);
        /// P4
        P1REN_P4: 4 = struct P1REN_P4(bool);
        /// P5
        P1REN_P5: 5 = struct P1REN_P5(bool);
        /// P6
        P1REN_P6: 6 = struct P1REN_P6(bool);
        /// P7
        P1REN_P7: 7 = struct P1REN_P7(bool);
    }
    /// Port 2 Input
    rw P2IN @ 0x08: u8 = 0_0 {
        /// P0
        P2IN_P0: 0 = struct P2IN_P0(bool);
        /// P1
        P2IN_P1: 1 = struct P2IN_P1(bool);
        /// P2
        P2IN_P2: 2 = struct P2IN_P2(bool);
        /// P3
        P2IN_P3: 3 = struct P2IN_P3(bool);
        /// P4
        P2IN_P4: 4 = struct P2IN_P4(bool);
        /// P5
        P2IN_P5: 5 = struct P2IN_P5(bool);
        /// P6
        P2IN_P6: 6 = struct P2IN_P6(bool);
        /// P7
        P2IN_P7: 7 = struct P2IN_P7(bool);
    }
    /// Port 2 Output
    rw P2OUT @ 0x09: u8 = 0_0 {
        /// P0
        P2OUT_P0: 0 = struct P2OUT_P0(bool);
        /// P1
        P2OUT_P1: 1 = struct P2OUT_P1(bool);
        /// P2
        P2OUT_P2: 2 = struct P2OUT_P2(bool);
        /// P3
        P2OUT_P3: 3 = struct P2OUT_P3(bool);
        /// P4
        P2OUT_P4: 4 = struct P2OUT_P4(bool);
        /// P5
        P2OUT_P5: 5 = struct P2OUT_P5(bool);
        /// P6
        P2OUT_P6: 6 = struct P2OUT_P6(bool);
        /// P7
        P2OUT_P7: 7 = struct P2OUT_P7(bool);
    }
    /// Port 2 Direction
    rw P2DIR @ 0x0a: u8 = 0_0 {
        /// P0
        P2DIR_P0: 0 = struct P2DIR_P0(bool);
        /// P1
        P2DIR_P1: 1 = struct P2DIR_P1(bool);
        /// P2
        P2DIR_P2: 2 = struct P2DIR_P2(bool);
        /// P3
        P2DIR_P3: 3 = struct P2DIR_P3(bool);
        /// P4
        P2DIR_P4: 4 = struct P2DIR_P4(bool);
        /// P5
        P2DIR_P5: 5 = struct P2DIR_P5(bool);
        /// P6
        P2DIR_P6: 6 = struct P2DIR_P6(bool);
        /// P7
        P2DIR_P7: 7 = struct P2DIR_P7(bool);
    }
    /// Port 2 Interrupt Flag
    rw P2IFG @ 0x0b: u8 = 0_0 {
        /// P0
        P2IFG_P0: 0 = struct P2IFG_P0(bool);
        /// P1
        P2IFG_P1: 1 = struct P2IFG_P1(bool);
        /// P2
        P2IFG_P2: 2 = struct P2IFG_P2(bool);
        /// P3
        P2IFG_P3: 3 = struct P2IFG_P3(bool);
        /// P4
        P2IFG_P4: 4 = struct P2IFG_P4(bool);
        /// P5
        P2IFG_P5: 5 = struct P2IFG_P5(bool);
        /// P6
        P2IFG_P6: 6 = struct P2IFG_P6(bool);
        /// P7
        P2IFG_P7: 7 = struct P2IFG_P7(bool);
    }
    /// Port 2 Interrupt Edge Select
    rw P2IES @ 0x0c: u8 = 0_0 {
        /// P0
        P2IES_P0: 0 = struct P2IES_P0(bool);
        /// P1
        P2IES_P1: 1 = struct P2IES_P1(bool);
        /// P2
        P2IES_P2: 2 = struct P2IES_P2(bool);
        /// P3
        P2IES_P3: 3 = struct P2IES_P3(bool);
        /// P4
        P2IES_P4: 4 = struct P2IES_P4(bool);
        /// P5
        P2IES_P5: 5 = struct P2IES_P5(bool);
        /// P6
        P2IES_P6: 6 = struct P2IES_P6(bool);
        /// P7
        P2IES_P7: 7 = struct P2IES_P7(bool);
    }
    /// Port 2 Interrupt Enable
    rw P2IE @ 0x0d: u8 = 0_0 {
        /// P0
        P2IE_P0: 0 = struct P2IE_P0(bool);
        /// P1
        P2IE_P1: 1 = struct P2IE_P1(bool);
        /// P2
        P2IE_P2: 2 = struct P2IE_P2(bool);
        /// P3
        P2IE_P3: 3 = struct P2IE_P3(bool);
        /// P4
        P2IE_P4: 4 = struct P2IE_P4(bool);
        /// P5
        P2IE_P5: 5 = struct P2IE_P5(bool);
        /// P6
        P2IE_P6: 6 = struct P2IE_P6(bool);
        /// P7
        P2IE_P7: 7 = struct P2IE_P7(bool);
    }
    /// Port 2 Selection
    rw P2SEL @ 0x0e: u8 = 0_0 {
        /// P0
        P2SEL_P0: 0 = struct P2SEL_P0(bool);
        /// P1
        P2SEL_P1: 1 = struct P2SEL_P1(bool);
        /// P2
        P2SEL_P2: 2 = struct P2SEL_P2(bool);
        /// P3
        P2SEL_P3: 3 = struct P2SEL_P3(bool);
        /// P4
        P2SEL_P4: 4 = struct P2SEL_P4(bool);
        /// P5
        P2SEL_P5: 5 = struct P2SEL_P5(bool);
        /// P6
        P2SEL_P6: 6 = struct P2SEL_P6(bool);
        /// P7
        P2SEL_P7: 7 = struct P2SEL_P7(bool);
    }
    /// Port 2 Selection 2
    rw P2SEL2 @ 0x22: u8 = 0_0 {
        /// P0
        P2SEL2_P0: 0 = struct P2SEL2_P0(bool);
        /// P1
        P2SEL2_P1: 1 = struct P2SEL2_P1(bool);
        /// P2
        P2SEL2_P2: 2 = struct P2SEL2_P2(bool);
        /// P3
        P2SEL2_P3: 3 = struct P2SEL2_P3(bool);
        /// P4
        P2SEL2_P4: 4 = struct P2SEL2_P4(bool);
        /// P5
        P2SEL2_P5: 5 = struct P2SEL2_P5(bool);
        /// P6
        P2SEL2_P6: 6 = struct P2SEL2_P6(bool);
        /// P7
        P2SEL2_P7: 7 = struct P2SEL2_P7(bool);
    }
    /// Port 2 Resistor Enable
    rw P2REN @ 0x0f: u8 = 0_0 {
        /// P0
        P2REN_P0: 0 = struct P2REN_P0(bool);
        /// P1
        P2REN_P1: 1 = struct P2REN_P1(bool);
        /// P2
        P2REN_P2: 2 = struct P2REN_P2(bool);
        /// P3
        P2REN_P3: 3 = struct P2REN_P3(bool);
        /// P4
        P2REN_P4: 4 = struct P2REN_P4(bool);
        /// P5
        P2REN_P5: 5 = struct P2REN_P5(bool);
        /// P6
        P2REN_P6: 6 = struct P2REN_P6(bool);
        /// P7
        P2REN_P7: 7 = struct P2REN_P7(bool);
    }
}
