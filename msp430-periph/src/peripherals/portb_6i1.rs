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
    /// Port Interrupt Edge Select
    rw PIES @ 0x18: u8 = 0_0 {
        /// PIES0
        PIES0: 0 = struct PIES0(bool);
        /// PIES1
        PIES1: 1 = struct PIES1(bool);
        /// PIES2
        PIES2: 2 = struct PIES2(bool);
        /// PIES3
        PIES3: 3 = struct PIES3(bool);
        /// PIES4
        PIES4: 4 = struct PIES4(bool);
        /// PIES5
        PIES5: 5 = struct PIES5(bool);
        /// PIES6
        PIES6: 6 = struct PIES6(bool);
        /// PIES7
        PIES7: 7 = struct PIES7(bool);
    }
    /// Port Interrupt Enable
    rw PIE @ 0x1a: u8 = 0_0 {
        /// PIE0
        PIE0: 0 = struct PIE0(bool);
        /// PIE1
        PIE1: 1 = struct PIE1(bool);
        /// PIE2
        PIE2: 2 = struct PIE2(bool);
        /// PIE3
        PIE3: 3 = struct PIE3(bool);
        /// PIE4
        PIE4: 4 = struct PIE4(bool);
        /// PIE5
        PIE5: 5 = struct PIE5(bool);
        /// PIE6
        PIE6: 6 = struct PIE6(bool);
        /// PIE7
        PIE7: 7 = struct PIE7(bool);
    }
    /// Port Interrupt Flag
    rw PIFG @ 0x1c: u8 = 0_0 {
        /// PIFG0
        PIFG0: 0 = struct PIFG0(bool);
        /// PIFG1
        PIFG1: 1 = struct PIFG1(bool);
        /// PIFG2
        PIFG2: 2 = struct PIFG2(bool);
        /// PIFG3
        PIFG3: 3 = struct PIFG3(bool);
        /// PIFG4
        PIFG4: 4 = struct PIFG4(bool);
        /// PIFG5
        PIFG5: 5 = struct PIFG5(bool);
        /// PIFG6
        PIFG6: 6 = struct PIFG6(bool);
        /// PIFG7
        PIFG7: 7 = struct PIFG7(bool);
    }
    /// Port Interrupt Vector Word
    rw PIV @ 0x0e: u16 = 0_0 {
        /// Port Interrupt Vector Word
        PIV: 0..15 = struct PIVField(u16);
    }
}
