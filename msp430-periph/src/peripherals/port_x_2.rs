//! Port 7/8/9/10

utils::periph! {
    /// Port 7/8/9/10
    Port;
    /// Port Input
    rw IN_ @ 0x00: u8 = 0_0 {
        /// P0
        IN_P0: 0 = struct IN_P0(bool);
        /// P1
        IN_P1: 1 = struct IN_P1(bool);
        /// P2
        IN_P2: 2 = struct IN_P2(bool);
        /// P3
        IN_P3: 3 = struct IN_P3(bool);
        /// P4
        IN_P4: 4 = struct IN_P4(bool);
        /// P5
        IN_P5: 5 = struct IN_P5(bool);
        /// P6
        IN_P6: 6 = struct IN_P6(bool);
        /// P7
        IN_P7: 7 = struct IN_P7(bool);
    }
    /// Port Output
    rw OUT @ 0x02: u8 = 0_0 {
        /// P0
        OUT_P0: 0 = struct OUT_P0(bool);
        /// P1
        OUT_P1: 1 = struct OUT_P1(bool);
        /// P2
        OUT_P2: 2 = struct OUT_P2(bool);
        /// P3
        OUT_P3: 3 = struct OUT_P3(bool);
        /// P4
        OUT_P4: 4 = struct OUT_P4(bool);
        /// P5
        OUT_P5: 5 = struct OUT_P5(bool);
        /// P6
        OUT_P6: 6 = struct OUT_P6(bool);
        /// P7
        OUT_P7: 7 = struct OUT_P7(bool);
    }
    /// Port Direction
    rw DIR @ 0x04: u8 = 0_0 {
        /// P0
        DIR_P0: 0 = struct DIR_P0(bool);
        /// P1
        DIR_P1: 1 = struct DIR_P1(bool);
        /// P2
        DIR_P2: 2 = struct DIR_P2(bool);
        /// P3
        DIR_P3: 3 = struct DIR_P3(bool);
        /// P4
        DIR_P4: 4 = struct DIR_P4(bool);
        /// P5
        DIR_P5: 5 = struct DIR_P5(bool);
        /// P6
        DIR_P6: 6 = struct DIR_P6(bool);
        /// P7
        DIR_P7: 7 = struct DIR_P7(bool);
    }
    /// Port Selection
    rw SEL @ 0x06: u8 = 0_0 {
        /// P0
        SEL_P0: 0 = struct SEL_P0(bool);
        /// P1
        SEL_P1: 1 = struct SEL_P1(bool);
        /// P2
        SEL_P2: 2 = struct SEL_P2(bool);
        /// P3
        SEL_P3: 3 = struct SEL_P3(bool);
        /// P4
        SEL_P4: 4 = struct SEL_P4(bool);
        /// P5
        SEL_P5: 5 = struct SEL_P5(bool);
        /// P6
        SEL_P6: 6 = struct SEL_P6(bool);
        /// P7
        SEL_P7: 7 = struct SEL_P7(bool);
    }
}
