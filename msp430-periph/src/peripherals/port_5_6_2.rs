//! Port 5/6

utils::periph! {
    /// Port 5/6
    Port56;
    /// Port 5 Input
    rw P5IN @ 0x1e: u8 = 0_0 {
        /// P0
        P5IN_P0: 0 = struct P5IN_P0(bool);
        /// P1
        P5IN_P1: 1 = struct P5IN_P1(bool);
        /// P2
        P5IN_P2: 2 = struct P5IN_P2(bool);
        /// P3
        P5IN_P3: 3 = struct P5IN_P3(bool);
        /// P4
        P5IN_P4: 4 = struct P5IN_P4(bool);
        /// P5
        P5IN_P5: 5 = struct P5IN_P5(bool);
        /// P6
        P5IN_P6: 6 = struct P5IN_P6(bool);
        /// P7
        P5IN_P7: 7 = struct P5IN_P7(bool);
    }
    /// Port 5 Output
    rw P5OUT @ 0x1f: u8 = 0_0 {
        /// P0
        P5OUT_P0: 0 = struct P5OUT_P0(bool);
        /// P1
        P5OUT_P1: 1 = struct P5OUT_P1(bool);
        /// P2
        P5OUT_P2: 2 = struct P5OUT_P2(bool);
        /// P3
        P5OUT_P3: 3 = struct P5OUT_P3(bool);
        /// P4
        P5OUT_P4: 4 = struct P5OUT_P4(bool);
        /// P5
        P5OUT_P5: 5 = struct P5OUT_P5(bool);
        /// P6
        P5OUT_P6: 6 = struct P5OUT_P6(bool);
        /// P7
        P5OUT_P7: 7 = struct P5OUT_P7(bool);
    }
    /// Port 5 Direction
    rw P5DIR @ 0x20: u8 = 0_0 {
        /// P0
        P5DIR_P0: 0 = struct P5DIR_P0(bool);
        /// P1
        P5DIR_P1: 1 = struct P5DIR_P1(bool);
        /// P2
        P5DIR_P2: 2 = struct P5DIR_P2(bool);
        /// P3
        P5DIR_P3: 3 = struct P5DIR_P3(bool);
        /// P4
        P5DIR_P4: 4 = struct P5DIR_P4(bool);
        /// P5
        P5DIR_P5: 5 = struct P5DIR_P5(bool);
        /// P6
        P5DIR_P6: 6 = struct P5DIR_P6(bool);
        /// P7
        P5DIR_P7: 7 = struct P5DIR_P7(bool);
    }
    /// Port 5 Selection
    rw P5SEL @ 0x21: u8 = 0_0 {
        /// P0
        P5SEL_P0: 0 = struct P5SEL_P0(bool);
        /// P1
        P5SEL_P1: 1 = struct P5SEL_P1(bool);
        /// P2
        P5SEL_P2: 2 = struct P5SEL_P2(bool);
        /// P3
        P5SEL_P3: 3 = struct P5SEL_P3(bool);
        /// P4
        P5SEL_P4: 4 = struct P5SEL_P4(bool);
        /// P5
        P5SEL_P5: 5 = struct P5SEL_P5(bool);
        /// P6
        P5SEL_P6: 6 = struct P5SEL_P6(bool);
        /// P7
        P5SEL_P7: 7 = struct P5SEL_P7(bool);
    }
    /// Port 5 Resistor Enable
    rw P5REN @ 0x00: u8 = 0_0 {
        /// P0
        P5REN_P0: 0 = struct P5REN_P0(bool);
        /// P1
        P5REN_P1: 1 = struct P5REN_P1(bool);
        /// P2
        P5REN_P2: 2 = struct P5REN_P2(bool);
        /// P3
        P5REN_P3: 3 = struct P5REN_P3(bool);
        /// P4
        P5REN_P4: 4 = struct P5REN_P4(bool);
        /// P5
        P5REN_P5: 5 = struct P5REN_P5(bool);
        /// P6
        P5REN_P6: 6 = struct P5REN_P6(bool);
        /// P7
        P5REN_P7: 7 = struct P5REN_P7(bool);
    }
    /// Port 6 Input
    rw P6IN @ 0x22: u8 = 0_0 {
        /// P0
        P6IN_P0: 0 = struct P6IN_P0(bool);
        /// P1
        P6IN_P1: 1 = struct P6IN_P1(bool);
        /// P2
        P6IN_P2: 2 = struct P6IN_P2(bool);
        /// P3
        P6IN_P3: 3 = struct P6IN_P3(bool);
        /// P4
        P6IN_P4: 4 = struct P6IN_P4(bool);
        /// P5
        P6IN_P5: 5 = struct P6IN_P5(bool);
        /// P6
        P6IN_P6: 6 = struct P6IN_P6(bool);
        /// P7
        P6IN_P7: 7 = struct P6IN_P7(bool);
    }
    /// Port 6 Output
    rw P6OUT @ 0x23: u8 = 0_0 {
        /// P0
        P6OUT_P0: 0 = struct P6OUT_P0(bool);
        /// P1
        P6OUT_P1: 1 = struct P6OUT_P1(bool);
        /// P2
        P6OUT_P2: 2 = struct P6OUT_P2(bool);
        /// P3
        P6OUT_P3: 3 = struct P6OUT_P3(bool);
        /// P4
        P6OUT_P4: 4 = struct P6OUT_P4(bool);
        /// P5
        P6OUT_P5: 5 = struct P6OUT_P5(bool);
        /// P6
        P6OUT_P6: 6 = struct P6OUT_P6(bool);
        /// P7
        P6OUT_P7: 7 = struct P6OUT_P7(bool);
    }
    /// Port 6 Direction
    rw P6DIR @ 0x24: u8 = 0_0 {
        /// P0
        P6DIR_P0: 0 = struct P6DIR_P0(bool);
        /// P1
        P6DIR_P1: 1 = struct P6DIR_P1(bool);
        /// P2
        P6DIR_P2: 2 = struct P6DIR_P2(bool);
        /// P3
        P6DIR_P3: 3 = struct P6DIR_P3(bool);
        /// P4
        P6DIR_P4: 4 = struct P6DIR_P4(bool);
        /// P5
        P6DIR_P5: 5 = struct P6DIR_P5(bool);
        /// P6
        P6DIR_P6: 6 = struct P6DIR_P6(bool);
        /// P7
        P6DIR_P7: 7 = struct P6DIR_P7(bool);
    }
    /// Port 6 Selection
    rw P6SEL @ 0x25: u8 = 0_0 {
        /// P0
        P6SEL_P0: 0 = struct P6SEL_P0(bool);
        /// P1
        P6SEL_P1: 1 = struct P6SEL_P1(bool);
        /// P2
        P6SEL_P2: 2 = struct P6SEL_P2(bool);
        /// P3
        P6SEL_P3: 3 = struct P6SEL_P3(bool);
        /// P4
        P6SEL_P4: 4 = struct P6SEL_P4(bool);
        /// P5
        P6SEL_P5: 5 = struct P6SEL_P5(bool);
        /// P6
        P6SEL_P6: 6 = struct P6SEL_P6(bool);
        /// P7
        P6SEL_P7: 7 = struct P6SEL_P7(bool);
    }
    /// Port 6 Resistor Enable
    rw P6REN @ 0x01: u8 = 0_0 {
        /// P0
        P6REN_P0: 0 = struct P6REN_P0(bool);
        /// P1
        P6REN_P1: 1 = struct P6REN_P1(bool);
        /// P2
        P6REN_P2: 2 = struct P6REN_P2(bool);
        /// P3
        P6REN_P3: 3 = struct P6REN_P3(bool);
        /// P4
        P6REN_P4: 4 = struct P6REN_P4(bool);
        /// P5
        P6REN_P5: 5 = struct P6REN_P5(bool);
        /// P6
        P6REN_P6: 6 = struct P6REN_P6(bool);
        /// P7
        P6REN_P7: 7 = struct P6REN_P7(bool);
    }
}
