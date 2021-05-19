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
    }
    /// Portrive Strenght
    rw PDS @ 0x08: u16 = 0_0 {
        /// PDS0
        PDS0: 0 = struct PS0(bool);
        /// PDS1
        PDS1: 1 = struct PS1(bool);
        /// PDS2
        PDS2: 2 = struct PS2(bool);
        /// PDS3
        PDS3: 3 = struct PS3(bool);
    }
    /// Port Selection
    rw PSEL0 @ 0x0a: u16 = 0_0 {
        /// PSEL0
        PSEL0_0: 0 = struct PSEL0_0(bool);
        /// PSEL1
        PSEL0_1: 1 = struct PSEL0_1(bool);
        /// PSEL2
        PSEL0_2: 2 = struct PSEL0_2(bool);
        /// PSEL3
        PSEL0_3: 3 = struct PSEL0_3(bool);
    }
}
