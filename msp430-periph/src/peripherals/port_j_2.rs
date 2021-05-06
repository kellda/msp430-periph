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
        /// PJIN6
        PJIN6: 6 = struct PJIN6(bool);
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
        /// PJOUT6
        PJOUT6: 6 = struct PJOUT6(bool);
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
        /// PJDIR6
        PJDIR6: 6 = struct PJDIR6(bool);
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
        /// PJREN6
        PJREN6: 6 = struct PJREN6(bool);
    }
    /// Port J Drive Strenght
    rw PJDS @ 0x08: u16 = 0_0 {
        /// PJDS0
        PJDS0: 0 = struct PJDS0(bool);
        /// PJDS1
        PJDS1: 1 = struct PJDS1(bool);
        /// PJDS2
        PJDS2: 2 = struct PJDS2(bool);
        /// PJDS3
        PJDS3: 3 = struct PJDS3(bool);
        /// PJDS4
        PJDS4: 4 = struct PJDS4(bool);
        /// PJDS5
        PJDS5: 5 = struct PJDS5(bool);
        /// PJDS6
        PJDS6: 6 = struct PJDS6(bool);
    }
    /// Port J Selection
    rw PJSEL @ 0x0a: u16 = 0_0 {
        /// PJSEL0
        PJSEL0: 0 = struct PJSEL0(bool);
        /// PJSEL1
        PJSEL1: 1 = struct PJSEL1(bool);
        /// PJSEL2
        PJSEL2: 2 = struct PJSEL2(bool);
        /// PJSEL3
        PJSEL3: 3 = struct PJSEL3(bool);
        /// PJSEL4
        PJSEL4: 4 = struct PJSEL4(bool);
        /// PJSEL5
        PJSEL5: 5 = struct PJSEL5(bool);
        /// PJSEL6
        PJSEL6: 6 = struct PJSEL6(bool);
    }
}
