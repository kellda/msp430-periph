//! Port J

utils::periph! {
    /// Port J
    PortJ;
    /// Port J Input
    rw PJIN @ 0x00: u16 = 0_0 {
        /// PJIN0
        PJIN0: 0 = struct PJIN0(bool);
        /// PJIN1
        PJIN1: 1 = struct PJIN1(bool);
        /// PJIN2
        PJIN2: 2 = struct PJIN2(bool);
        /// PJIN3
        PJIN3: 3 = struct PJIN3(bool);
        /// PJIN4
        PJIN4: 4 = struct PJIN4(bool);
        /// PJIN5
        PJIN5: 5 = struct PJIN5(bool);
    }
    /// Port J Output
    rw PJOUT @ 0x02: u16 = 0_0 {
        /// PJOUT0
        PJOUT0: 0 = struct PJOUT0(bool);
        /// PJOUT1
        PJOUT1: 1 = struct PJOUT1(bool);
        /// PJOUT2
        PJOUT2: 2 = struct PJOUT2(bool);
        /// PJOUT3
        PJOUT3: 3 = struct PJOUT3(bool);
        /// PJOUT4
        PJOUT4: 4 = struct PJOUT4(bool);
        /// PJOUT5
        PJOUT5: 5 = struct PJOUT5(bool);
    }
    /// Port J Direction
    rw PJDIR @ 0x04: u16 = 0_0 {
        /// PJDIR0
        PJDIR0: 0 = struct PJDIR0(bool);
        /// PJDIR1
        PJDIR1: 1 = struct PJDIR1(bool);
        /// PJDIR2
        PJDIR2: 2 = struct PJDIR2(bool);
        /// PJDIR3
        PJDIR3: 3 = struct PJDIR3(bool);
        /// PJDIR4
        PJDIR4: 4 = struct PJDIR4(bool);
        /// PJDIR5
        PJDIR5: 5 = struct PJDIR5(bool);
    }
    /// Port J Resistor Enable
    rw PJREN @ 0x06: u16 = 0_0 {
        /// PJREN0
        PJREN0: 0 = struct PJREN0(bool);
        /// PJREN1
        PJREN1: 1 = struct PJREN1(bool);
        /// PJREN2
        PJREN2: 2 = struct PJREN2(bool);
        /// PJREN3
        PJREN3: 3 = struct PJREN3(bool);
        /// PJREN4
        PJREN4: 4 = struct PJREN4(bool);
        /// PJREN5
        PJREN5: 5 = struct PJREN5(bool);
    }
    /// Port J Selection 0
    rw PJSEL0 @ 0x0a: u16 = 0_0 {
        /// PJSEL0_0
        PJSEL0_0: 0 = struct PJSEL0_0(bool);
        /// PJSEL0_1
        PJSEL0_1: 1 = struct PJSEL0_1(bool);
        /// PJSEL0_2
        PJSEL0_2: 2 = struct PJSEL0_2(bool);
        /// PJSEL0_3
        PJSEL0_3: 3 = struct PJSEL0_3(bool);
        /// PJSEL0_4
        PJSEL0_4: 4 = struct PJSEL0_4(bool);
        /// PJSEL0_5
        PJSEL0_5: 5 = struct PJSEL0_5(bool);
    }
    /// Port J Selection 1
    rw PJSEL1 @ 0x0c: u16 = 0_0 {
        /// PJSEL1_0
        PJSEL1_0: 0 = struct PJSEL1_0(bool);
        /// PJSEL1_1
        PJSEL1_1: 1 = struct PJSEL1_1(bool);
        /// PJSEL1_2
        PJSEL1_2: 2 = struct PJSEL1_2(bool);
        /// PJSEL1_3
        PJSEL1_3: 3 = struct PJSEL1_3(bool);
        /// PJSEL1_4
        PJSEL1_4: 4 = struct PJSEL1_4(bool);
        /// PJSEL1_5
        PJSEL1_5: 5 = struct PJSEL1_5(bool);
    }
    /// Port J Complement Selection
    rw PJSELC @ 0x16: u16 = 0_0 {
        /// Port J Complement Selection
        PJSELC: 0..15 = struct PJSELCField(u16);
    }
}
