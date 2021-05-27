//! Port 0

utils::periph! {
    /// Port 0
    Port;
    /// Port Input
    rw IN_ @ 0x00: u8 = 0_0 {
        /// PIN_0
        IN_0: 0 = struct IN_0(bool);
        /// PIN_1
        IN_1: 1 = struct IN_1(bool);
        /// PIN_2
        IN_2: 2 = struct IN_2(bool);
        /// PIN_3
        IN_3: 3 = struct IN_3(bool);
        /// PIN_4
        IN_4: 4 = struct IN_4(bool);
        /// PIN_5
        IN_5: 5 = struct IN_5(bool);
        /// PIN_6
        IN_6: 6 = struct IN_6(bool);
        /// PIN_7
        IN_7: 7 = struct IN_7(bool);
    }
    /// Port Output
    rw OUT @ 0x01: u8 = 0_0 {
        /// POUT_0
        OUT_0: 0 = struct OUT_0(bool);
        /// POUT_1
        OUT_1: 1 = struct OUT_1(bool);
        /// POUT_2
        OUT_2: 2 = struct OUT_2(bool);
        /// POUT_3
        OUT_3: 3 = struct OUT_3(bool);
        /// POUT_4
        OUT_4: 4 = struct OUT_4(bool);
        /// POUT_5
        OUT_5: 5 = struct OUT_5(bool);
        /// POUT_6
        OUT_6: 6 = struct OUT_6(bool);
        /// POUT_7
        OUT_7: 7 = struct OUT_7(bool);
    }
    /// Port Direction
    rw DIR @ 0x02: u8 = 0_0 {
        /// PDIR_0
        DIR_0: 0 = struct DIR_0(bool);
        /// PDIR_1
        DIR_1: 1 = struct DIR_1(bool);
        /// PDIR_2
        DIR_2: 2 = struct DIR_2(bool);
        /// PDIR_3
        DIR_3: 3 = struct DIR_3(bool);
        /// PDIR_4
        DIR_4: 4 = struct DIR_4(bool);
        /// PDIR_5
        DIR_5: 5 = struct DIR_5(bool);
        /// PDIR_6
        DIR_6: 6 = struct DIR_6(bool);
        /// PDIR_7
        DIR_7: 7 = struct DIR_7(bool);
    }
    /// Port Interrupt Flag
    rw IFG @ 0x03: u8 = 0_0 {
        /// PIFG_2
        IFG_2: 2 = struct IFG_2(bool);
        /// PIFG_3
        IFG_3: 3 = struct IFG_3(bool);
        /// PIFG_4
        IFG_4: 4 = struct IFG_4(bool);
        /// PIFG_5
        IFG_5: 5 = struct IFG_5(bool);
        /// PIFG_6
        IFG_6: 6 = struct IFG_6(bool);
        /// PIFG_7
        IFG_7: 7 = struct IFG_7(bool);
    }
    /// Port Interrupt Edge Select
    rw IES @ 0x04: u8 = 0_0 {
        /// PIES_0
        IES_0: 0 = struct IES_0(bool);
        /// PIES_1
        IES_1: 1 = struct IES_1(bool);
        /// PIES_2
        IES_2: 2 = struct IES_2(bool);
        /// PIES_3
        IES_3: 3 = struct IES_3(bool);
        /// PIES_4
        IES_4: 4 = struct IES_4(bool);
        /// PIES_5
        IES_5: 5 = struct IES_5(bool);
        /// PIES_6
        IES_6: 6 = struct IES_6(bool);
        /// PIES_7
        IES_7: 7 = struct IES_7(bool);
    }
    /// Port Interrupt Enable
    rw IE @ 0x05: u8 = 0_0 {
        /// PIE_2
        IE_2: 2 = struct IE_2(bool);
        /// PIE_3
        IE_3: 3 = struct IE_3(bool);
        /// PIE_4
        IE_4: 4 = struct IE_4(bool);
        /// PIE_5
        IE_5: 5 = struct IE_5(bool);
        /// PIE_6
        IE_6: 6 = struct IE_6(bool);
        /// PIE_7
        IE_7: 7 = struct IE_7(bool);
    }
}
