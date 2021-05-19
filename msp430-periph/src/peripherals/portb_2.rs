//! Port x

utils::periph! {
    /// Port x
    Port;
    /// Port Input
    rw PIN @ 0x00: u8 = 0_0 {
        /// PIN0
        PIN0: 0 = struct PIN0(bool);
        /// PIN1
        PIN1: 1 = struct PIN1(bool);
        /// PIN2
        PIN2: 2 = struct PIN2(bool);
        /// PIN3
        PIN3: 3 = struct PIN3(bool);
        /// PIN4
        PIN4: 4 = struct PIN4(bool);
        /// PIN5
        PIN5: 5 = struct PIN5(bool);
        /// PIN6
        PIN6: 6 = struct PIN6(bool);
        /// PIN7
        PIN7: 7 = struct PIN7(bool);
    }
    /// Port Output
    rw POUT @ 0x02: u8 = 0_0 {
        /// POUT0
        POUT0: 0 = struct POUT0(bool);
        /// POUT1
        POUT1: 1 = struct POUT1(bool);
        /// POUT2
        POUT2: 2 = struct POUT2(bool);
        /// POUT3
        POUT3: 3 = struct POUT3(bool);
        /// POUT4
        POUT4: 4 = struct POUT4(bool);
        /// POUT5
        POUT5: 5 = struct POUT5(bool);
        /// POUT6
        POUT6: 6 = struct POUT6(bool);
        /// POUT7
        POUT7: 7 = struct POUT7(bool);
    }
    /// Port Direction
    rw PDIR @ 0x04: u8 = 0_0 {
        /// PDIR0
        PDIR0: 0 = struct PDIR0(bool);
        /// PDIR1
        PDIR1: 1 = struct PDIR1(bool);
        /// PDIR2
        PDIR2: 2 = struct PDIR2(bool);
        /// PDIR3
        PDIR3: 3 = struct PDIR3(bool);
        /// PDIR4
        PDIR4: 4 = struct PDIR4(bool);
        /// PDIR5
        PDIR5: 5 = struct PDIR5(bool);
        /// PDIR6
        PDIR6: 6 = struct PDIR6(bool);
        /// PDIR7
        PDIR7: 7 = struct PDIR7(bool);
    }
    /// Port Resistor Enable
    rw PREN @ 0x06: u8 = 0_0 {
        /// PREN0
        PREN0: 0 = struct PREN0(bool);
        /// PREN1
        PREN1: 1 = struct PREN1(bool);
        /// PREN2
        PREN2: 2 = struct PREN2(bool);
        /// PREN3
        PREN3: 3 = struct PREN3(bool);
        /// PREN4
        PREN4: 4 = struct PREN4(bool);
        /// PREN5
        PREN5: 5 = struct PREN5(bool);
        /// PREN6
        PREN6: 6 = struct PREN6(bool);
        /// PREN7
        PREN7: 7 = struct PREN7(bool);
    }
    /// Port Drive Strenght
    rw PDS @ 0x08: u8 = 0_0 {
        /// PDS0
        PDS0: 0 = struct PDS0(bool);
        /// PDS1
        PDS1: 1 = struct PDS1(bool);
        /// PDS2
        PDS2: 2 = struct PDS2(bool);
        /// PDS3
        PDS3: 3 = struct PDS3(bool);
        /// PDS4
        PDS4: 4 = struct PDS4(bool);
        /// PDS5
        PDS5: 5 = struct PDS5(bool);
        /// PDS6
        PDS6: 6 = struct PDS6(bool);
        /// PDS7
        PDS7: 7 = struct PDS7(bool);
    }
    /// Port Selection 0
    rw PSEL0 @ 0x0a: u8 = 0_0 {
        /// PSEL0_0
        PSEL0_0: 0 = struct PDSEL0_0(bool);
        /// PSEL0_1
        PSEL0_1: 1 = struct PDSEL0_1(bool);
        /// PSEL0_2
        PSEL0_2: 2 = struct PDSEL0_2(bool);
        /// PSEL0_3
        PSEL0_3: 3 = struct PDSEL0_3(bool);
        /// PSEL0_4
        PSEL0_4: 4 = struct PDSEL0_4(bool);
        /// PSEL0_5
        PSEL0_5: 5 = struct PDSEL0_5(bool);
        /// PSEL0_6
        PSEL0_6: 6 = struct PDSEL0_6(bool);
        /// PSEL0_7
        PSEL0_7: 7 = struct PDSEL0_7(bool);
    }
    /// Port Selection 1
    rw PSEL1 @ 0x0c: u8 = 0_0 {
        /// PSEL1_0
        PSEL1_0: 0 = struct PDSEL1_0(bool);
        /// PSEL1_1
        PSEL1_1: 1 = struct PDSEL1_1(bool);
        /// PSEL1_2
        PSEL1_2: 2 = struct PDSEL1_2(bool);
        /// PSEL1_3
        PSEL1_3: 3 = struct PDSEL1_3(bool);
        /// PSEL1_4
        PSEL1_4: 4 = struct PDSEL1_4(bool);
        /// PSEL1_5
        PSEL1_5: 5 = struct PDSEL1_5(bool);
        /// PSEL1_6
        PSEL1_6: 6 = struct PDSEL1_6(bool);
        /// PSEL1_7
        PSEL1_7: 7 = struct PDSEL1_7(bool);
    }
}
