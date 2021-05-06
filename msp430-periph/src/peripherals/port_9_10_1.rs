//! Port 9/10

utils::periph! {
    /// Port 9/10
    Port910;
    /// Port 9 Input
    rw P9IN @ 0x00: u8 = 0_0 {
        /// P0
        P9IN_P0: 0 = struct P9IN_P0(bool);
        /// P1
        P9IN_P1: 1 = struct P9IN_P1(bool);
        /// P2
        P9IN_P2: 2 = struct P9IN_P2(bool);
        /// P3
        P9IN_P3: 3 = struct P9IN_P3(bool);
        /// P4
        P9IN_P4: 4 = struct P9IN_P4(bool);
        /// P5
        P9IN_P5: 5 = struct P9IN_P5(bool);
        /// P6
        P9IN_P6: 6 = struct P9IN_P6(bool);
        /// P7
        P9IN_P7: 7 = struct P9IN_P7(bool);
    }
    /// Port 9 Output
    rw P9OUT @ 0x02: u8 = 0_0 {
        /// P0
        P9OUT_P0: 0 = struct P9OUT_P0(bool);
        /// P1
        P9OUT_P1: 1 = struct P9OUT_P1(bool);
        /// P2
        P9OUT_P2: 2 = struct P9OUT_P2(bool);
        /// P3
        P9OUT_P3: 3 = struct P9OUT_P3(bool);
        /// P4
        P9OUT_P4: 4 = struct P9OUT_P4(bool);
        /// P5
        P9OUT_P5: 5 = struct P9OUT_P5(bool);
        /// P6
        P9OUT_P6: 6 = struct P9OUT_P6(bool);
        /// P7
        P9OUT_P7: 7 = struct P9OUT_P7(bool);
    }
    /// Port 9 Direction
    rw P9DIR @ 0x04: u8 = 0_0 {
        /// P0
        P9DIR_P0: 0 = struct P9DIR_P0(bool);
        /// P1
        P9DIR_P1: 1 = struct P9DIR_P1(bool);
        /// P2
        P9DIR_P2: 2 = struct P9DIR_P2(bool);
        /// P3
        P9DIR_P3: 3 = struct P9DIR_P3(bool);
        /// P4
        P9DIR_P4: 4 = struct P9DIR_P4(bool);
        /// P5
        P9DIR_P5: 5 = struct P9DIR_P5(bool);
        /// P6
        P9DIR_P6: 6 = struct P9DIR_P6(bool);
        /// P7
        P9DIR_P7: 7 = struct P9DIR_P7(bool);
    }
    /// Port 9 Selection
    rw P9SEL @ 0x06: u8 = 0_0 {
        /// P0
        P9SEL_P0: 0 = struct P9SEL_P0(bool);
        /// P1
        P9SEL_P1: 1 = struct P9SEL_P1(bool);
        /// P2
        P9SEL_P2: 2 = struct P9SEL_P2(bool);
        /// P3
        P9SEL_P3: 3 = struct P9SEL_P3(bool);
        /// P4
        P9SEL_P4: 4 = struct P9SEL_P4(bool);
        /// P5
        P9SEL_P5: 5 = struct P9SEL_P5(bool);
        /// P6
        P9SEL_P6: 6 = struct P9SEL_P6(bool);
        /// P7
        P9SEL_P7: 7 = struct P9SEL_P7(bool);
    }
    /// Port 10 Input
    rw P10IN @ 0x01: u8 = 0_0 {
        /// P0
        P10IN_P0: 0 = struct P10IN_P0(bool);
        /// P1
        P10IN_P1: 1 = struct P10IN_P1(bool);
        /// P2
        P10IN_P2: 2 = struct P10IN_P2(bool);
        /// P3
        P10IN_P3: 3 = struct P10IN_P3(bool);
        /// P4
        P10IN_P4: 4 = struct P10IN_P4(bool);
        /// P5
        P10IN_P5: 5 = struct P10IN_P5(bool);
        /// P6
        P10IN_P6: 6 = struct P10IN_P6(bool);
        /// P7
        P10IN_P7: 7 = struct P10IN_P7(bool);
    }
    /// Port 10 Output
    rw P10OUT @ 0x03: u8 = 0_0 {
        /// P0
        P10OUT_P0: 0 = struct P10OUT_P0(bool);
        /// P1
        P10OUT_P1: 1 = struct P10OUT_P1(bool);
        /// P2
        P10OUT_P2: 2 = struct P10OUT_P2(bool);
        /// P3
        P10OUT_P3: 3 = struct P10OUT_P3(bool);
        /// P4
        P10OUT_P4: 4 = struct P10OUT_P4(bool);
        /// P5
        P10OUT_P5: 5 = struct P10OUT_P5(bool);
        /// P6
        P10OUT_P6: 6 = struct P10OUT_P6(bool);
        /// P7
        P10OUT_P7: 7 = struct P10OUT_P7(bool);
    }
    /// Port 10 Direction
    rw P10DIR @ 0x05: u8 = 0_0 {
        /// P0
        P10DIR_P0: 0 = struct P10DIR_P0(bool);
        /// P1
        P10DIR_P1: 1 = struct P10DIR_P1(bool);
        /// P2
        P10DIR_P2: 2 = struct P10DIR_P2(bool);
        /// P3
        P10DIR_P3: 3 = struct P10DIR_P3(bool);
        /// P4
        P10DIR_P4: 4 = struct P10DIR_P4(bool);
        /// P5
        P10DIR_P5: 5 = struct P10DIR_P5(bool);
        /// P6
        P10DIR_P6: 6 = struct P10DIR_P6(bool);
        /// P7
        P10DIR_P7: 7 = struct P10DIR_P7(bool);
    }
    /// Port 10 Selection
    rw P10SEL @ 0x07: u8 = 0_0 {
        /// P0
        P10SEL_P0: 0 = struct P10SEL_P0(bool);
        /// P1
        P10SEL_P1: 1 = struct P10SEL_P1(bool);
        /// P2
        P10SEL_P2: 2 = struct P10SEL_P2(bool);
        /// P3
        P10SEL_P3: 3 = struct P10SEL_P3(bool);
        /// P4
        P10SEL_P4: 4 = struct P10SEL_P4(bool);
        /// P5
        P10SEL_P5: 5 = struct P10SEL_P5(bool);
        /// P6
        P10SEL_P6: 6 = struct P10SEL_P6(bool);
        /// P7
        P10SEL_P7: 7 = struct P10SEL_P7(bool);
    }
    /// Port B Input
    rw PBIN @ 0x00: u16 = 0_0 {
        /// P0
        PBIN_P0: 0 = struct PBIN_P0(bool);
        /// P1
        PBIN_P1: 1 = struct PBIN_P1(bool);
        /// P2
        PBIN_P2: 2 = struct PBIN_P2(bool);
        /// P3
        PBIN_P3: 3 = struct PBIN_P3(bool);
        /// P4
        PBIN_P4: 4 = struct PBIN_P4(bool);
        /// P5
        PBIN_P5: 5 = struct PBIN_P5(bool);
        /// P6
        PBIN_P6: 6 = struct PBIN_P6(bool);
        /// P7
        PBIN_P7: 7 = struct PBIN_P7(bool);
        /// P8
        PBIN_P8: 8 = struct PBIN_P8(bool);
        /// P9
        PBIN_P9: 9 = struct PBIN_P9(bool);
        /// P10
        PBIN_P10: 10 = struct PBIN_P10(bool);
        /// P11
        PBIN_P11: 11 = struct PBIN_P11(bool);
        /// P12
        PBIN_P12: 12 = struct PBIN_P12(bool);
        /// P13
        PBIN_P13: 13 = struct PBIN_P13(bool);
        /// P14
        PBIN_P14: 14 = struct PBIN_P14(bool);
        /// P15
        PBIN_P15: 15 = struct PBIN_P15(bool);
    }
    /// Port B Output
    rw PBOUT @ 0x02: u16 = 0_0 {
        /// P0
        PBOUT_P0: 0 = struct PBOUT_P0(bool);
        /// P1
        PBOUT_P1: 1 = struct PBOUT_P1(bool);
        /// P2
        PBOUT_P2: 2 = struct PBOUT_P2(bool);
        /// P3
        PBOUT_P3: 3 = struct PBOUT_P3(bool);
        /// P4
        PBOUT_P4: 4 = struct PBOUT_P4(bool);
        /// P5
        PBOUT_P5: 5 = struct PBOUT_P5(bool);
        /// P6
        PBOUT_P6: 6 = struct PBOUT_P6(bool);
        /// P7
        PBOUT_P7: 7 = struct PBOUT_P7(bool);
        /// P8
        PBOUT_P8: 8 = struct PBOUT_P8(bool);
        /// P9
        PBOUT_P9: 9 = struct PBOUT_P9(bool);
        /// P10
        PBOUT_P10: 10 = struct PBOUT_P10(bool);
        /// P11
        PBOUT_P11: 11 = struct PBOUT_P11(bool);
        /// P12
        PBOUT_P12: 12 = struct PBOUT_P12(bool);
        /// P13
        PBOUT_P13: 13 = struct PBOUT_P13(bool);
        /// P14
        PBOUT_P14: 14 = struct PBOUT_P14(bool);
        /// P15
        PBOUT_P15: 15 = struct PBOUT_P15(bool);
    }
    /// Port B Direction
    rw PBDIR @ 0x04: u16 = 0_0 {
        /// P0
        PBDIR_P0: 0 = struct PBDIR_P0(bool);
        /// P1
        PBDIR_P1: 1 = struct PBDIR_P1(bool);
        /// P2
        PBDIR_P2: 2 = struct PBDIR_P2(bool);
        /// P3
        PBDIR_P3: 3 = struct PBDIR_P3(bool);
        /// P4
        PBDIR_P4: 4 = struct PBDIR_P4(bool);
        /// P5
        PBDIR_P5: 5 = struct PBDIR_P5(bool);
        /// P6
        PBDIR_P6: 6 = struct PBDIR_P6(bool);
        /// P7
        PBDIR_P7: 7 = struct PBDIR_P7(bool);
        /// P8
        PBDIR_P8: 8 = struct PBDIR_P8(bool);
        /// P9
        PBDIR_P9: 9 = struct PBDIR_P9(bool);
        /// P10
        PBDIR_P10: 10 = struct PBDIR_P10(bool);
        /// P11
        PBDIR_P11: 11 = struct PBDIR_P11(bool);
        /// P12
        PBDIR_P12: 12 = struct PBDIR_P12(bool);
        /// P13
        PBDIR_P13: 13 = struct PBDIR_P13(bool);
        /// P14
        PBDIR_P14: 14 = struct PBDIR_P14(bool);
        /// P15
        PBDIR_P15: 15 = struct PBDIR_P15(bool);
    }
    /// Port B Selection
    rw PBSEL @ 0x06: u16 = 0_0 {
        /// P0
        PBSEL_P0: 0 = struct PBSEL_P0(bool);
        /// P1
        PBSEL_P1: 1 = struct PBSEL_P1(bool);
        /// P2
        PBSEL_P2: 2 = struct PBSEL_P2(bool);
        /// P3
        PBSEL_P3: 3 = struct PBSEL_P3(bool);
        /// P4
        PBSEL_P4: 4 = struct PBSEL_P4(bool);
        /// P5
        PBSEL_P5: 5 = struct PBSEL_P5(bool);
        /// P6
        PBSEL_P6: 6 = struct PBSEL_P6(bool);
        /// P7
        PBSEL_P7: 7 = struct PBSEL_P7(bool);
        /// P8
        PBSEL_P8: 8 = struct PBSEL_P8(bool);
        /// P9
        PBSEL_P9: 9 = struct PBSEL_P9(bool);
        /// P10
        PBSEL_P10: 10 = struct PBSEL_P10(bool);
        /// P11
        PBSEL_P11: 11 = struct PBSEL_P11(bool);
        /// P12
        PBSEL_P12: 12 = struct PBSEL_P12(bool);
        /// P13
        PBSEL_P13: 13 = struct PBSEL_P13(bool);
        /// P14
        PBSEL_P14: 14 = struct PBSEL_P14(bool);
        /// P15
        PBSEL_P15: 15 = struct PBSEL_P15(bool);
    }
}
