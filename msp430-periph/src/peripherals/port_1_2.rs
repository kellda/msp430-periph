//! Port 1

utils::periph! {
    /// Port 1
    Port;
    /// Port Input
    rw PIN @ 0x00: u8 = 0_0 {
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
    rw POUT @ 0x01: u8 = 0_0 {
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
    rw PDIR @ 0x02: u8 = 0_0 {
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
    /// Port Interrupt Flag
    rw PIFG @ 0x03: u8 = 0_0 {
        /// P0
        PIFG_P0: 0 = struct PIFG_P0(bool);
        /// P1
        PIFG_P1: 1 = struct PIFG_P1(bool);
        /// P2
        PIFG_P2: 2 = struct PIFG_P2(bool);
        /// P3
        PIFG_P3: 3 = struct PIFG_P3(bool);
        /// P4
        PIFG_P4: 4 = struct PIFG_P4(bool);
        /// P5
        PIFG_P5: 5 = struct PIFG_P5(bool);
        /// P6
        PIFG_P6: 6 = struct PIFG_P6(bool);
        /// P7
        PIFG_P7: 7 = struct PIFG_P7(bool);
    }
    /// Port Interrupt Edge Select
    rw PIES @ 0x04: u8 = 0_0 {
        /// P0
        PIES_P0: 0 = struct PIES_P0(bool);
        /// P1
        PIES_P1: 1 = struct PIES_P1(bool);
        /// P2
        PIES_P2: 2 = struct PIES_P2(bool);
        /// P3
        PIES_P3: 3 = struct PIES_P3(bool);
        /// P4
        PIES_P4: 4 = struct PIES_P4(bool);
        /// P5
        PIES_P5: 5 = struct PIES_P5(bool);
        /// P6
        PIES_P6: 6 = struct PIES_P6(bool);
        /// P7
        PIES_P7: 7 = struct PIES_P7(bool);
    }
    /// Port Interrupt Enable
    rw PIE @ 0x05: u8 = 0_0 {
        /// P0
        PIE_P0: 0 = struct PIE_P0(bool);
        /// P1
        PIE_P1: 1 = struct PIE_P1(bool);
        /// P2
        PIE_P2: 2 = struct PIE_P2(bool);
        /// P3
        PIE_P3: 3 = struct PIE_P3(bool);
        /// P4
        PIE_P4: 4 = struct PIE_P4(bool);
        /// P5
        PIE_P5: 5 = struct PIE_P5(bool);
        /// P6
        PIE_P6: 6 = struct PIE_P6(bool);
        /// P7
        PIE_P7: 7 = struct PIE_P7(bool);
    }
    /// Port Selection
    rw PSEL @ 0x06: u8 = 0_0 {
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
    /// Port Selection 2
    rw PSEL2 @ 0x37: u8 = 0_0 {
        /// P0
        PSEL2_P0: 0 = struct PSEL2_P0(bool);
        /// P1
        PSEL2_P1: 1 = struct PSEL2_P1(bool);
        /// P2
        PSEL2_P2: 2 = struct PSEL2_P2(bool);
        /// P3
        PSEL2_P3: 3 = struct PSEL2_P3(bool);
        /// P4
        PSEL2_P4: 4 = struct PSEL2_P4(bool);
        /// P5
        PSEL2_P5: 5 = struct PSEL2_P5(bool);
        /// P6
        PSEL2_P6: 6 = struct PSEL2_P6(bool);
        /// P7
        PSEL2_P7: 7 = struct PSEL2_P7(bool);
    }
}
