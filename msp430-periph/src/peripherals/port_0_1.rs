//! Port 0

utils::periph! {
    /// Port 0
    Port;
    /// Port Input
    rw PIN @ 0x00: u8 = 0_0 {
        /// PIN_0
        PIN_0: 0 = struct PIN_0(bool);
        /// PIN_1
        PIN_1: 1 = struct PIN_1(bool);
        /// PIN_2
        PIN_2: 2 = struct PIN_2(bool);
        /// PIN_3
        PIN_3: 3 = struct PIN_3(bool);
        /// PIN_4
        PIN_4: 4 = struct PIN_4(bool);
        /// PIN_5
        PIN_5: 5 = struct PIN_5(bool);
        /// PIN_6
        PIN_6: 6 = struct PIN_6(bool);
        /// PIN_7
        PIN_7: 7 = struct PIN_7(bool);
    }
    /// Port Output
    rw POUT @ 0x01: u8 = 0_0 {
        /// POUT_0
        POUT_0: 0 = struct POUT_0(bool);
        /// POUT_1
        POUT_1: 1 = struct POUT_1(bool);
        /// POUT_2
        POUT_2: 2 = struct POUT_2(bool);
        /// POUT_3
        POUT_3: 3 = struct POUT_3(bool);
        /// POUT_4
        POUT_4: 4 = struct POUT_4(bool);
        /// POUT_5
        POUT_5: 5 = struct POUT_5(bool);
        /// POUT_6
        POUT_6: 6 = struct POUT_6(bool);
        /// POUT_7
        POUT_7: 7 = struct POUT_7(bool);
    }
    /// Port Direction
    rw PDIR @ 0x02: u8 = 0_0 {
        /// PDIR_0
        PDIR_0: 0 = struct PDIR_0(bool);
        /// PDIR_1
        PDIR_1: 1 = struct PDIR_1(bool);
        /// PDIR_2
        PDIR_2: 2 = struct PDIR_2(bool);
        /// PDIR_3
        PDIR_3: 3 = struct PDIR_3(bool);
        /// PDIR_4
        PDIR_4: 4 = struct PDIR_4(bool);
        /// PDIR_5
        PDIR_5: 5 = struct PDIR_5(bool);
        /// PDIR_6
        PDIR_6: 6 = struct PDIR_6(bool);
        /// PDIR_7
        PDIR_7: 7 = struct PDIR_7(bool);
    }
    /// Port Interrupt Flag
    rw PIFG @ 0x03: u8 = 0_0 {
        /// PIFG_2
        PIFG_2: 2 = struct PIFG_2(bool);
        /// PIFG_3
        PIFG_3: 3 = struct PIFG_3(bool);
        /// PIFG_4
        PIFG_4: 4 = struct PIFG_4(bool);
        /// PIFG_5
        PIFG_5: 5 = struct PIFG_5(bool);
        /// PIFG_6
        PIFG_6: 6 = struct PIFG_6(bool);
        /// PIFG_7
        PIFG_7: 7 = struct PIFG_7(bool);
    }
    /// Port Interrupt Edge Select
    rw PIES @ 0x04: u8 = 0_0 {
        /// PIES_0
        PIES_0: 0 = struct PIES_0(bool);
        /// PIES_1
        PIES_1: 1 = struct PIES_1(bool);
        /// PIES_2
        PIES_2: 2 = struct PIES_2(bool);
        /// PIES_3
        PIES_3: 3 = struct PIES_3(bool);
        /// PIES_4
        PIES_4: 4 = struct PIES_4(bool);
        /// PIES_5
        PIES_5: 5 = struct PIES_5(bool);
        /// PIES_6
        PIES_6: 6 = struct PIES_6(bool);
        /// PIES_7
        PIES_7: 7 = struct PIES_7(bool);
    }
    /// Port Interrupt Enable
    rw PIE @ 0x05: u8 = 0_0 {
        /// PIE_2
        PIE_2: 2 = struct PIE_2(bool);
        /// PIE_3
        PIE_3: 3 = struct PIE_3(bool);
        /// PIE_4
        PIE_4: 4 = struct PIE_4(bool);
        /// PIE_5
        PIE_5: 5 = struct PIE_5(bool);
        /// PIE_6
        PIE_6: 6 = struct PIE_6(bool);
        /// PIE_7
        PIE_7: 7 = struct PIE_7(bool);
    }
}
