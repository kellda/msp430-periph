//! Port J

utils::periph! {
    /// Port J
    Port;
    /// Port Input
    rw IN_ @ 0x00: u16 = 0_0 {
        /// PIN0
        IN0: 0 = struct IN0(bool);
        /// PIN1
        IN1: 1 = struct IN1(bool);
        /// PIN2
        IN2: 2 = struct IN2(bool);
        /// PIN3
        IN3: 3 = struct IN3(bool);
    }
    /// Port Output
    rw OUT @ 0x02: u16 = 0_0 {
        /// POUT0
        OUT0: 0 = struct OUT0(bool);
        /// POUT1
        OUT1: 1 = struct OUT1(bool);
        /// POUT2
        OUT2: 2 = struct OUT2(bool);
        /// POUT3
        OUT3: 3 = struct OUT3(bool);
    }
    /// Port Direction
    rw DIR @ 0x04: u16 = 0_0 {
        /// PDIR0
        DIR0: 0 = struct DIR0(bool);
        /// PDIR1
        DIR1: 1 = struct DIR1(bool);
        /// PDIR2
        DIR2: 2 = struct DIR2(bool);
        /// PDIR3
        DIR3: 3 = struct DIR3(bool);
    }
    /// Port Resistor Enable
    rw REN @ 0x06: u16 = 0_0 {
        /// PREN0
        REN0: 0 = struct REN0(bool);
        /// PREN1
        REN1: 1 = struct REN1(bool);
        /// PREN2
        REN2: 2 = struct REN2(bool);
        /// PREN3
        REN3: 3 = struct REN3(bool);
    }
    /// Portrive Strenght
    rw DS @ 0x08: u16 = 0_0 {
        /// PDS0
        DS0: 0 = struct S0(bool);
        /// PDS1
        DS1: 1 = struct S1(bool);
        /// PDS2
        DS2: 2 = struct S2(bool);
        /// PDS3
        DS3: 3 = struct S3(bool);
    }
}
