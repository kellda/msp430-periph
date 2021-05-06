//! Port 7

utils::periph! {
    /// Port 7
    Port7;
    /// Port 7 Input
    rw P7IN @ 0x00: u8 = 0_0 {
        /// P0
        P7IN_P0: 0 = struct P7IN_P0(bool);
        /// P1
        P7IN_P1: 1 = struct P7IN_P1(bool);
        /// P2
        P7IN_P2: 2 = struct P7IN_P2(bool);
        /// P3
        P7IN_P3: 3 = struct P7IN_P3(bool);
        /// P4
        P7IN_P4: 4 = struct P7IN_P4(bool);
        /// P5
        P7IN_P5: 5 = struct P7IN_P5(bool);
        /// P6
        P7IN_P6: 6 = struct P7IN_P6(bool);
        /// P7
        P7IN_P7: 7 = struct P7IN_P7(bool);
    }
    /// Port 7 Output
    rw P7OUT @ 0x01: u8 = 0_0 {
        /// P0
        P7OUT_P0: 0 = struct P7OUT_P0(bool);
        /// P1
        P7OUT_P1: 1 = struct P7OUT_P1(bool);
        /// P2
        P7OUT_P2: 2 = struct P7OUT_P2(bool);
        /// P3
        P7OUT_P3: 3 = struct P7OUT_P3(bool);
        /// P4
        P7OUT_P4: 4 = struct P7OUT_P4(bool);
        /// P5
        P7OUT_P5: 5 = struct P7OUT_P5(bool);
        /// P6
        P7OUT_P6: 6 = struct P7OUT_P6(bool);
        /// P7
        P7OUT_P7: 7 = struct P7OUT_P7(bool);
    }
    /// Port 7 Direction
    rw P7DIR @ 0x02: u8 = 0_0 {
        /// P0
        P7DIR_P0: 0 = struct P7DIR_P0(bool);
        /// P1
        P7DIR_P1: 1 = struct P7DIR_P1(bool);
        /// P2
        P7DIR_P2: 2 = struct P7DIR_P2(bool);
        /// P3
        P7DIR_P3: 3 = struct P7DIR_P3(bool);
        /// P4
        P7DIR_P4: 4 = struct P7DIR_P4(bool);
        /// P5
        P7DIR_P5: 5 = struct P7DIR_P5(bool);
        /// P6
        P7DIR_P6: 6 = struct P7DIR_P6(bool);
        /// P7
        P7DIR_P7: 7 = struct P7DIR_P7(bool);
    }
    /// Port 7 Selection
    rw P7SEL @ 0x03: u8 = 0_0 {
        /// P0
        P7SEL_P0: 0 = struct P7SEL_P0(bool);
        /// P1
        P7SEL_P1: 1 = struct P7SEL_P1(bool);
        /// P2
        P7SEL_P2: 2 = struct P7SEL_P2(bool);
        /// P3
        P7SEL_P3: 3 = struct P7SEL_P3(bool);
        /// P4
        P7SEL_P4: 4 = struct P7SEL_P4(bool);
        /// P5
        P7SEL_P5: 5 = struct P7SEL_P5(bool);
        /// P6
        P7SEL_P6: 6 = struct P7SEL_P6(bool);
        /// P7
        P7SEL_P7: 7 = struct P7SEL_P7(bool);
    }
}
