//! Port 0

utils::periph! {
    /// Port 0
    Port0;
    /// Port 0 Input
    rw P0IN @ 0x00: u8 = 0_0 {
        /// P0IN_0
        P0IN_0: 0 = struct P0IN_0(bool);
        /// P0IN_1
        P0IN_1: 1 = struct P0IN_1(bool);
        /// P0IN_2
        P0IN_2: 2 = struct P0IN_2(bool);
        /// P0IN_3
        P0IN_3: 3 = struct P0IN_3(bool);
        /// P0IN_4
        P0IN_4: 4 = struct P0IN_4(bool);
        /// P0IN_5
        P0IN_5: 5 = struct P0IN_5(bool);
        /// P0IN_6
        P0IN_6: 6 = struct P0IN_6(bool);
        /// P0IN_7
        P0IN_7: 7 = struct P0IN_7(bool);
    }
    /// Port 0 Output
    rw P0OUT @ 0x01: u8 = 0_0 {
        /// P0OUT_0
        P0OUT_0: 0 = struct P0OUT_0(bool);
        /// P0OUT_1
        P0OUT_1: 1 = struct P0OUT_1(bool);
        /// P0OUT_2
        P0OUT_2: 2 = struct P0OUT_2(bool);
        /// P0OUT_3
        P0OUT_3: 3 = struct P0OUT_3(bool);
        /// P0OUT_4
        P0OUT_4: 4 = struct P0OUT_4(bool);
        /// P0OUT_5
        P0OUT_5: 5 = struct P0OUT_5(bool);
        /// P0OUT_6
        P0OUT_6: 6 = struct P0OUT_6(bool);
        /// P0OUT_7
        P0OUT_7: 7 = struct P0OUT_7(bool);
    }
    /// Port 0 Direction
    rw P0DIR @ 0x02: u8 = 0_0 {
        /// P0DIR_0
        P0DIR_0: 0 = struct P0DIR_0(bool);
        /// P0DIR_1
        P0DIR_1: 1 = struct P0DIR_1(bool);
        /// P0DIR_2
        P0DIR_2: 2 = struct P0DIR_2(bool);
        /// P0DIR_3
        P0DIR_3: 3 = struct P0DIR_3(bool);
        /// P0DIR_4
        P0DIR_4: 4 = struct P0DIR_4(bool);
        /// P0DIR_5
        P0DIR_5: 5 = struct P0DIR_5(bool);
        /// P0DIR_6
        P0DIR_6: 6 = struct P0DIR_6(bool);
        /// P0DIR_7
        P0DIR_7: 7 = struct P0DIR_7(bool);
    }
    /// Port 0 Interrupt Flag
    rw P0IFG @ 0x03: u8 = 0_0 {
        /// P0IFG_2
        P0IFG_2: 2 = struct P0IFG_2(bool);
        /// P0IFG_3
        P0IFG_3: 3 = struct P0IFG_3(bool);
        /// P0IFG_4
        P0IFG_4: 4 = struct P0IFG_4(bool);
        /// P0IFG_5
        P0IFG_5: 5 = struct P0IFG_5(bool);
        /// P0IFG_6
        P0IFG_6: 6 = struct P0IFG_6(bool);
        /// P0IFG_7
        P0IFG_7: 7 = struct P0IFG_7(bool);
    }
    /// Port 0 Interrupt Edge Select
    rw P0IES @ 0x04: u8 = 0_0 {
        /// P0IES_0
        P0IES_0: 0 = struct P0IES_0(bool);
        /// P0IES_1
        P0IES_1: 1 = struct P0IES_1(bool);
        /// P0IES_2
        P0IES_2: 2 = struct P0IES_2(bool);
        /// P0IES_3
        P0IES_3: 3 = struct P0IES_3(bool);
        /// P0IES_4
        P0IES_4: 4 = struct P0IES_4(bool);
        /// P0IES_5
        P0IES_5: 5 = struct P0IES_5(bool);
        /// P0IES_6
        P0IES_6: 6 = struct P0IES_6(bool);
        /// P0IES_7
        P0IES_7: 7 = struct P0IES_7(bool);
    }
    /// Port 0 Interrupt Enable
    rw P0IE @ 0x05: u8 = 0_0 {
        /// P0IE_2
        P0IE_2: 2 = struct P0IE_2(bool);
        /// P0IE_3
        P0IE_3: 3 = struct P0IE_3(bool);
        /// P0IE_4
        P0IE_4: 4 = struct P0IE_4(bool);
        /// P0IE_5
        P0IE_5: 5 = struct P0IE_5(bool);
        /// P0IE_6
        P0IE_6: 6 = struct P0IE_6(bool);
        /// P0IE_7
        P0IE_7: 7 = struct P0IE_7(bool);
    }
}
