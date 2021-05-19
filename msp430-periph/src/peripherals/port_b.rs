//! Port B

utils::periph! {
    /// Port B
    Port;
    /// Port Input
    rw PIN @ 0x00: u16 = 0_0 {
        /// P0
        PIN_P0: 0 = struct PIN_P0(bool);
        /// P1
        PIN_P1: 1 = struct PIN_P1(bool);
        /// P2
        PIN_P2: 2 = struct PIN_P2(bool);
        /// P3
        PIN_P3: 3 = struct PIN_P3(bool);
        /// P4
        PIN_P4: 4 = struct PIN_P4(bool);
        /// P5
        PIN_P5: 5 = struct PIN_P5(bool);
        /// P6
        PIN_P6: 6 = struct PIN_P6(bool);
        /// P7
        PIN_P7: 7 = struct PIN_P7(bool);
        /// P8
        PIN_P8: 8 = struct PIN_P8(bool);
        /// P9
        PIN_P9: 9 = struct PIN_P9(bool);
        /// P10
        PIN_P10: 10 = struct PIN_P10(bool);
        /// P11
        PIN_P11: 11 = struct PIN_P11(bool);
        /// P12
        PIN_P12: 12 = struct PIN_P12(bool);
        /// P13
        PIN_P13: 13 = struct PIN_P13(bool);
        /// P14
        PIN_P14: 14 = struct PIN_P14(bool);
        /// P15
        PIN_P15: 15 = struct PIN_P15(bool);
    }
    /// Port Output
    rw POUT @ 0x02: u16 = 0_0 {
        /// P0
        POUT_P0: 0 = struct POUT_P0(bool);
        /// P1
        POUT_P1: 1 = struct POUT_P1(bool);
        /// P2
        POUT_P2: 2 = struct POUT_P2(bool);
        /// P3
        POUT_P3: 3 = struct POUT_P3(bool);
        /// P4
        POUT_P4: 4 = struct POUT_P4(bool);
        /// P5
        POUT_P5: 5 = struct POUT_P5(bool);
        /// P6
        POUT_P6: 6 = struct POUT_P6(bool);
        /// P7
        POUT_P7: 7 = struct POUT_P7(bool);
        /// P8
        POUT_P8: 8 = struct POUT_P8(bool);
        /// P9
        POUT_P9: 9 = struct POUT_P9(bool);
        /// P10
        POUT_P10: 10 = struct POUT_P10(bool);
        /// P11
        POUT_P11: 11 = struct POUT_P11(bool);
        /// P12
        POUT_P12: 12 = struct POUT_P12(bool);
        /// P13
        POUT_P13: 13 = struct POUT_P13(bool);
        /// P14
        POUT_P14: 14 = struct POUT_P14(bool);
        /// P15
        POUT_P15: 15 = struct POUT_P15(bool);
    }
    /// Port Direction
    rw PDIR @ 0x04: u16 = 0_0 {
        /// P0
        PDIR_P0: 0 = struct PDIR_P0(bool);
        /// P1
        PDIR_P1: 1 = struct PDIR_P1(bool);
        /// P2
        PDIR_P2: 2 = struct PDIR_P2(bool);
        /// P3
        PDIR_P3: 3 = struct PDIR_P3(bool);
        /// P4
        PDIR_P4: 4 = struct PDIR_P4(bool);
        /// P5
        PDIR_P5: 5 = struct PDIR_P5(bool);
        /// P6
        PDIR_P6: 6 = struct PDIR_P6(bool);
        /// P7
        PDIR_P7: 7 = struct PDIR_P7(bool);
        /// P8
        PDIR_P8: 8 = struct PDIR_P8(bool);
        /// P9
        PDIR_P9: 9 = struct PDIR_P9(bool);
        /// P10
        PDIR_P10: 10 = struct PDIR_P10(bool);
        /// P11
        PDIR_P11: 11 = struct PDIR_P11(bool);
        /// P12
        PDIR_P12: 12 = struct PDIR_P12(bool);
        /// P13
        PDIR_P13: 13 = struct PDIR_P13(bool);
        /// P14
        PDIR_P14: 14 = struct PDIR_P14(bool);
        /// P15
        PDIR_P15: 15 = struct PDIR_P15(bool);
    }
    /// Port Selection
    rw PSEL @ 0x06: u16 = 0_0 {
        /// P0
        PSEL_P0: 0 = struct PSEL_P0(bool);
        /// P1
        PSEL_P1: 1 = struct PSEL_P1(bool);
        /// P2
        PSEL_P2: 2 = struct PSEL_P2(bool);
        /// P3
        PSEL_P3: 3 = struct PSEL_P3(bool);
        /// P4
        PSEL_P4: 4 = struct PSEL_P4(bool);
        /// P5
        PSEL_P5: 5 = struct PSEL_P5(bool);
        /// P6
        PSEL_P6: 6 = struct PSEL_P6(bool);
        /// P7
        PSEL_P7: 7 = struct PSEL_P7(bool);
        /// P8
        PSEL_P8: 8 = struct PSEL_P8(bool);
        /// P9
        PSEL_P9: 9 = struct PSEL_P9(bool);
        /// P10
        PSEL_P10: 10 = struct PSEL_P10(bool);
        /// P11
        PSEL_P11: 11 = struct PSEL_P11(bool);
        /// P12
        PSEL_P12: 12 = struct PSEL_P12(bool);
        /// P13
        PSEL_P13: 13 = struct PSEL_P13(bool);
        /// P14
        PSEL_P14: 14 = struct PSEL_P14(bool);
        /// P15
        PSEL_P15: 15 = struct PSEL_P15(bool);
    }
    /// Port Resistor Enable
    rw PREN @ 0x0e: u16 = 0_0 {
        /// P0
        PREN_P0: 0 = struct PREN_P0(bool);
        /// P1
        PREN_P1: 1 = struct PREN_P1(bool);
        /// P2
        PREN_P2: 2 = struct PREN_P2(bool);
        /// P3
        PREN_P3: 3 = struct PREN_P3(bool);
        /// P4
        PREN_P4: 4 = struct PREN_P4(bool);
        /// P5
        PREN_P5: 5 = struct PREN_P5(bool);
        /// P6
        PREN_P6: 6 = struct PREN_P6(bool);
        /// P7
        PREN_P7: 7 = struct PREN_P7(bool);
        /// P8
        PREN_P8: 8 = struct PREN_P8(bool);
        /// P9
        PREN_P9: 9 = struct PREN_P9(bool);
        /// P10
        PREN_P10: 10 = struct PREN_P10(bool);
        /// P11
        PREN_P11: 11 = struct PREN_P11(bool);
        /// P12
        PREN_P12: 12 = struct PREN_P12(bool);
        /// P13
        PREN_P13: 13 = struct PREN_P13(bool);
        /// P14
        PREN_P14: 14 = struct PREN_P14(bool);
        /// P15
        PREN_P15: 15 = struct PREN_P15(bool);
    }
}
