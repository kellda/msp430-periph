//! Port 6

utils::periph! {
    /// Port 6
    Port;
    /// Port Input
    rw PIN @ 0x21: u8 = 0_0 {
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
    }
    /// Port Output
    rw POUT @ 0x22: u8 = 0_0 {
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
    }
    /// Port Direction
    rw PDIR @ 0x23: u8 = 0_0 {
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
    }
    /// Port Selection
    rw PSEL @ 0x24: u8 = 0_0 {
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
    }
    /// Port Resistor Enable
    rw PREN @ 0x00: u8 = 0_0 {
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
    }
}
