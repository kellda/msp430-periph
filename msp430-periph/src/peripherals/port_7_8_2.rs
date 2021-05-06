//! Port 7/8

utils::periph! {
    /// Port 7/8
    Port78;
    /// Port 7 Input
    rw P7IN @ 0x24: u8 = 0_0 {
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
    rw P7OUT @ 0x26: u8 = 0_0 {
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
    rw P7DIR @ 0x28: u8 = 0_0 {
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
    rw P7SEL @ 0x2a: u8 = 0_0 {
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
    /// Port 7 Resistor Enable
    rw P7REN @ 0x00: u8 = 0_0 {
        /// P0
        P7REN_P0: 0 = struct P7REN_P0(bool);
        /// P1
        P7REN_P1: 1 = struct P7REN_P1(bool);
        /// P2
        P7REN_P2: 2 = struct P7REN_P2(bool);
        /// P3
        P7REN_P3: 3 = struct P7REN_P3(bool);
        /// P4
        P7REN_P4: 4 = struct P7REN_P4(bool);
        /// P5
        P7REN_P5: 5 = struct P7REN_P5(bool);
        /// P6
        P7REN_P6: 6 = struct P7REN_P6(bool);
        /// P7
        P7REN_P7: 7 = struct P7REN_P7(bool);
    }
    /// Port 8 Input
    rw P8IN @ 0x25: u8 = 0_0 {
        /// P0
        P8IN_P0: 0 = struct P8IN_P0(bool);
        /// P1
        P8IN_P1: 1 = struct P8IN_P1(bool);
        /// P2
        P8IN_P2: 2 = struct P8IN_P2(bool);
        /// P3
        P8IN_P3: 3 = struct P8IN_P3(bool);
        /// P4
        P8IN_P4: 4 = struct P8IN_P4(bool);
        /// P5
        P8IN_P5: 5 = struct P8IN_P5(bool);
        /// P6
        P8IN_P6: 6 = struct P8IN_P6(bool);
        /// P7
        P8IN_P7: 7 = struct P8IN_P7(bool);
    }
    /// Port 8 Output
    rw P8OUT @ 0x27: u8 = 0_0 {
        /// P0
        P8OUT_P0: 0 = struct P8OUT_P0(bool);
        /// P1
        P8OUT_P1: 1 = struct P8OUT_P1(bool);
        /// P2
        P8OUT_P2: 2 = struct P8OUT_P2(bool);
        /// P3
        P8OUT_P3: 3 = struct P8OUT_P3(bool);
        /// P4
        P8OUT_P4: 4 = struct P8OUT_P4(bool);
        /// P5
        P8OUT_P5: 5 = struct P8OUT_P5(bool);
        /// P6
        P8OUT_P6: 6 = struct P8OUT_P6(bool);
        /// P7
        P8OUT_P7: 7 = struct P8OUT_P7(bool);
    }
    /// Port 8 Direction
    rw P8DIR @ 0x29: u8 = 0_0 {
        /// P0
        P8DIR_P0: 0 = struct P8DIR_P0(bool);
        /// P1
        P8DIR_P1: 1 = struct P8DIR_P1(bool);
        /// P2
        P8DIR_P2: 2 = struct P8DIR_P2(bool);
        /// P3
        P8DIR_P3: 3 = struct P8DIR_P3(bool);
        /// P4
        P8DIR_P4: 4 = struct P8DIR_P4(bool);
        /// P5
        P8DIR_P5: 5 = struct P8DIR_P5(bool);
        /// P6
        P8DIR_P6: 6 = struct P8DIR_P6(bool);
        /// P7
        P8DIR_P7: 7 = struct P8DIR_P7(bool);
    }
    /// Port 8 Selection
    rw P8SEL @ 0x2b: u8 = 0_0 {
        /// P0
        P8SEL_P0: 0 = struct P8SEL_P0(bool);
        /// P1
        P8SEL_P1: 1 = struct P8SEL_P1(bool);
        /// P2
        P8SEL_P2: 2 = struct P8SEL_P2(bool);
        /// P3
        P8SEL_P3: 3 = struct P8SEL_P3(bool);
        /// P4
        P8SEL_P4: 4 = struct P8SEL_P4(bool);
        /// P5
        P8SEL_P5: 5 = struct P8SEL_P5(bool);
        /// P6
        P8SEL_P6: 6 = struct P8SEL_P6(bool);
        /// P7
        P8SEL_P7: 7 = struct P8SEL_P7(bool);
    }
    /// Port 8 Resistor Enable
    rw P8REN @ 0x01: u8 = 0_0 {
        /// P0
        P8REN_P0: 0 = struct P8REN_P0(bool);
        /// P1
        P8REN_P1: 1 = struct P8REN_P1(bool);
        /// P2
        P8REN_P2: 2 = struct P8REN_P2(bool);
        /// P3
        P8REN_P3: 3 = struct P8REN_P3(bool);
        /// P4
        P8REN_P4: 4 = struct P8REN_P4(bool);
        /// P5
        P8REN_P5: 5 = struct P8REN_P5(bool);
        /// P6
        P8REN_P6: 6 = struct P8REN_P6(bool);
        /// P7
        P8REN_P7: 7 = struct P8REN_P7(bool);
    }
    /// Port A Input
    rw PAIN @ 0x24: u16 = 0_0 {
        /// P0
        PAIN_P0: 0 = struct PAIN_P0(bool);
        /// P1
        PAIN_P1: 1 = struct PAIN_P1(bool);
        /// P2
        PAIN_P2: 2 = struct PAIN_P2(bool);
        /// P3
        PAIN_P3: 3 = struct PAIN_P3(bool);
        /// P4
        PAIN_P4: 4 = struct PAIN_P4(bool);
        /// P5
        PAIN_P5: 5 = struct PAIN_P5(bool);
        /// P6
        PAIN_P6: 6 = struct PAIN_P6(bool);
        /// P7
        PAIN_P7: 7 = struct PAIN_P7(bool);
        /// P8
        PAIN_P8: 8 = struct PAIN_P8(bool);
        /// P9
        PAIN_P9: 9 = struct PAIN_P9(bool);
        /// P10
        PAIN_P10: 10 = struct PAIN_P10(bool);
        /// P11
        PAIN_P11: 11 = struct PAIN_P11(bool);
        /// P12
        PAIN_P12: 12 = struct PAIN_P12(bool);
        /// P13
        PAIN_P13: 13 = struct PAIN_P13(bool);
        /// P14
        PAIN_P14: 14 = struct PAIN_P14(bool);
        /// P15
        PAIN_P15: 15 = struct PAIN_P15(bool);
    }
    /// Port A Output
    rw PAOUT @ 0x26: u16 = 0_0 {
        /// P0
        PAOUT_P0: 0 = struct PAOUT_P0(bool);
        /// P1
        PAOUT_P1: 1 = struct PAOUT_P1(bool);
        /// P2
        PAOUT_P2: 2 = struct PAOUT_P2(bool);
        /// P3
        PAOUT_P3: 3 = struct PAOUT_P3(bool);
        /// P4
        PAOUT_P4: 4 = struct PAOUT_P4(bool);
        /// P5
        PAOUT_P5: 5 = struct PAOUT_P5(bool);
        /// P6
        PAOUT_P6: 6 = struct PAOUT_P6(bool);
        /// P7
        PAOUT_P7: 7 = struct PAOUT_P7(bool);
        /// P8
        PAOUT_P8: 8 = struct PAOUT_P8(bool);
        /// P9
        PAOUT_P9: 9 = struct PAOUT_P9(bool);
        /// P10
        PAOUT_P10: 10 = struct PAOUT_P10(bool);
        /// P11
        PAOUT_P11: 11 = struct PAOUT_P11(bool);
        /// P12
        PAOUT_P12: 12 = struct PAOUT_P12(bool);
        /// P13
        PAOUT_P13: 13 = struct PAOUT_P13(bool);
        /// P14
        PAOUT_P14: 14 = struct PAOUT_P14(bool);
        /// P15
        PAOUT_P15: 15 = struct PAOUT_P15(bool);
    }
    /// Port A Direction
    rw PADIR @ 0x28: u16 = 0_0 {
        /// P0
        PADIR_P0: 0 = struct PADIR_P0(bool);
        /// P1
        PADIR_P1: 1 = struct PADIR_P1(bool);
        /// P2
        PADIR_P2: 2 = struct PADIR_P2(bool);
        /// P3
        PADIR_P3: 3 = struct PADIR_P3(bool);
        /// P4
        PADIR_P4: 4 = struct PADIR_P4(bool);
        /// P5
        PADIR_P5: 5 = struct PADIR_P5(bool);
        /// P6
        PADIR_P6: 6 = struct PADIR_P6(bool);
        /// P7
        PADIR_P7: 7 = struct PADIR_P7(bool);
        /// P8
        PADIR_P8: 8 = struct PADIR_P8(bool);
        /// P9
        PADIR_P9: 9 = struct PADIR_P9(bool);
        /// P10
        PADIR_P10: 10 = struct PADIR_P10(bool);
        /// P11
        PADIR_P11: 11 = struct PADIR_P11(bool);
        /// P12
        PADIR_P12: 12 = struct PADIR_P12(bool);
        /// P13
        PADIR_P13: 13 = struct PADIR_P13(bool);
        /// P14
        PADIR_P14: 14 = struct PADIR_P14(bool);
        /// P15
        PADIR_P15: 15 = struct PADIR_P15(bool);
    }
    /// Port A Selection
    rw PASEL @ 0x2a: u16 = 0_0 {
        /// P0
        PASEL_P0: 0 = struct PASEL_P0(bool);
        /// P1
        PASEL_P1: 1 = struct PASEL_P1(bool);
        /// P2
        PASEL_P2: 2 = struct PASEL_P2(bool);
        /// P3
        PASEL_P3: 3 = struct PASEL_P3(bool);
        /// P4
        PASEL_P4: 4 = struct PASEL_P4(bool);
        /// P5
        PASEL_P5: 5 = struct PASEL_P5(bool);
        /// P6
        PASEL_P6: 6 = struct PASEL_P6(bool);
        /// P7
        PASEL_P7: 7 = struct PASEL_P7(bool);
        /// P8
        PASEL_P8: 8 = struct PASEL_P8(bool);
        /// P9
        PASEL_P9: 9 = struct PASEL_P9(bool);
        /// P10
        PASEL_P10: 10 = struct PASEL_P10(bool);
        /// P11
        PASEL_P11: 11 = struct PASEL_P11(bool);
        /// P12
        PASEL_P12: 12 = struct PASEL_P12(bool);
        /// P13
        PASEL_P13: 13 = struct PASEL_P13(bool);
        /// P14
        PASEL_P14: 14 = struct PASEL_P14(bool);
        /// P15
        PASEL_P15: 15 = struct PASEL_P15(bool);
    }
    /// Port A Resistor Enable
    rw PAREN @ 0x00: u16 = 0_0 {
        /// P0
        PAREN_P0: 0 = struct PAREN_P0(bool);
        /// P1
        PAREN_P1: 1 = struct PAREN_P1(bool);
        /// P2
        PAREN_P2: 2 = struct PAREN_P2(bool);
        /// P3
        PAREN_P3: 3 = struct PAREN_P3(bool);
        /// P4
        PAREN_P4: 4 = struct PAREN_P4(bool);
        /// P5
        PAREN_P5: 5 = struct PAREN_P5(bool);
        /// P6
        PAREN_P6: 6 = struct PAREN_P6(bool);
        /// P7
        PAREN_P7: 7 = struct PAREN_P7(bool);
        /// P8
        PAREN_P8: 8 = struct PAREN_P8(bool);
        /// P9
        PAREN_P9: 9 = struct PAREN_P9(bool);
        /// P10
        PAREN_P10: 10 = struct PAREN_P10(bool);
        /// P11
        PAREN_P11: 11 = struct PAREN_P11(bool);
        /// P12
        PAREN_P12: 12 = struct PAREN_P12(bool);
        /// P13
        PAREN_P13: 13 = struct PAREN_P13(bool);
        /// P14
        PAREN_P14: 14 = struct PAREN_P14(bool);
        /// P15
        PAREN_P15: 15 = struct PAREN_P15(bool);
    }
}
