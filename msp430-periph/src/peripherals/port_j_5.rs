//! Port J

utils::periph! {
    /// Port J
    Port;
    /// Port Input
    rw PIN @ 0x00: u16 = 0_0 {
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
    rw POUT @ 0x02: u16 = 0_0 {
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
    rw PDIR @ 0x04: u16 = 0_0 {
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
    rw PREN @ 0x06: u16 = 0_0 {
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
    /// Port Selection 0
    rw PSEL0 @ 0x0a: u16 = 0_0 {
        /// PSEL0_0
        PSEL0_0: 0 = struct PSEL0_0(bool);
        /// PSEL0_1
        PSEL0_1: 1 = struct PSEL0_1(bool);
        /// PSEL0_2
        PSEL0_2: 2 = struct PSEL0_2(bool);
        /// PSEL0_3
        PSEL0_3: 3 = struct PSEL0_3(bool);
        /// PSEL0_4
        PSEL0_4: 4 = struct PSEL0_4(bool);
        /// PSEL0_5
        PSEL0_5: 5 = struct PSEL0_5(bool);
        /// PSEL0_6
        PSEL0_6: 6 = struct PSEL0_6(bool);
        /// PSEL0_7
        PSEL0_7: 7 = struct PSEL0_7(bool);
    }
    /// Port Selection 1
    rw PSEL1 @ 0x0c: u16 = 0_0 {
        /// PSEL1_0
        PSEL1_0: 0 = struct PSEL1_0(bool);
        /// PSEL1_1
        PSEL1_1: 1 = struct PSEL1_1(bool);
        /// PSEL1_2
        PSEL1_2: 2 = struct PSEL1_2(bool);
        /// PSEL1_3
        PSEL1_3: 3 = struct PSEL1_3(bool);
        /// PSEL1_4
        PSEL1_4: 4 = struct PSEL1_4(bool);
        /// PSEL1_5
        PSEL1_5: 5 = struct PSEL1_5(bool);
        /// PSEL1_6
        PSEL1_6: 6 = struct PSEL1_6(bool);
        /// PSEL1_7
        PSEL1_7: 7 = struct PSEL1_7(bool);
    }
    /// Port Complement Selection
    rw PSELC @ 0x16: u16 = 0_0 {
        /// PSELC_0
        PSELC_0: 0 = struct PSELC_0(bool);
        /// PSELC_1
        PSELC_1: 1 = struct PSELC_1(bool);
        /// PSELC_2
        PSELC_2: 2 = struct PSELC_2(bool);
        /// PSELC_3
        PSELC_3: 3 = struct PSELC_3(bool);
        /// PSELC_4
        PSELC_4: 4 = struct PSELC_4(bool);
        /// PSELC_5
        PSELC_5: 5 = struct PSELC_5(bool);
        /// PSELC_6
        PSELC_6: 6 = struct PSELC_6(bool);
        /// PSELC_7
        PSELC_7: 7 = struct PSELC_7(bool);
    }
}
