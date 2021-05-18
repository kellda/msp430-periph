//! Port Mapping

utils::periph! {
    /// Port Mapping
    PortMapping;
    /// Port Px.0/1 mapping register
    rw PMAP01 @ 0x00: u16 = 0_0 {
        /// PMAP01_0
        PMAP01_0: 0 = struct PMAP01_0(bool);
        /// PMAP01_1
        PMAP01_1: 1 = struct PMAP01_1(bool);
        /// PMAP01_2
        PMAP01_2: 2 = struct PMAP01_2(bool);
        /// PMAP01_3
        PMAP01_3: 3 = struct PMAP01_3(bool);
        /// PMAP01_4
        PMAP01_4: 4 = struct PMAP01_4(bool);
        /// PMAP01_5
        PMAP01_5: 5 = struct PMAP01_5(bool);
        /// PMAP01_6
        PMAP01_6: 6 = struct PMAP01_6(bool);
        /// PMAP01_7
        PMAP01_7: 7 = struct PMAP01_7(bool);
        /// PMAP01_8
        PMAP01_8: 8 = struct PMAP01_8(bool);
        /// PMAP01_9
        PMAP01_9: 9 = struct PMAP01_9(bool);
        /// PMAP01_10
        PMAP01_10: 10 = struct PMAP01_10(bool);
        /// PMAP01_11
        PMAP01_11: 11 = struct PMAP01_11(bool);
        /// PMAP01_12
        PMAP01_12: 12 = struct PMAP01_12(bool);
        /// PMAP01_13
        PMAP01_13: 13 = struct PMAP01_13(bool);
        /// PMAP01_14
        PMAP01_14: 14 = struct PMAP01_14(bool);
        /// PMAP01_15
        PMAP01_15: 15 = struct PMAP01_15(bool);
    }
    /// Port Px.2/3 mapping register
    rw PMAP23 @ 0x02: u16 = 0_0 {
        /// PMAP23_0
        PMAP23_0: 0 = struct PMAP23_0(bool);
        /// PMAP23_1
        PMAP23_1: 1 = struct PMAP23_1(bool);
        /// PMAP23_2
        PMAP23_2: 2 = struct PMAP23_2(bool);
        /// PMAP23_3
        PMAP23_3: 3 = struct PMAP23_3(bool);
        /// PMAP23_4
        PMAP23_4: 4 = struct PMAP23_4(bool);
        /// PMAP23_5
        PMAP23_5: 5 = struct PMAP23_5(bool);
        /// PMAP23_6
        PMAP23_6: 6 = struct PMAP23_6(bool);
        /// PMAP23_7
        PMAP23_7: 7 = struct PMAP23_7(bool);
        /// PMAP23_8
        PMAP23_8: 8 = struct PMAP23_8(bool);
        /// PMAP23_9
        PMAP23_9: 9 = struct PMAP23_9(bool);
        /// PMAP23_10
        PMAP23_10: 10 = struct PMAP23_10(bool);
        /// PMAP23_11
        PMAP23_11: 11 = struct PMAP23_11(bool);
        /// PMAP23_12
        PMAP23_12: 12 = struct PMAP23_12(bool);
        /// PMAP23_13
        PMAP23_13: 13 = struct PMAP23_13(bool);
        /// PMAP23_14
        PMAP23_14: 14 = struct PMAP23_14(bool);
        /// PMAP23_15
        PMAP23_15: 15 = struct PMAP23_15(bool);
    }
    /// Port Px.4/5 mapping register
    rw PMAP45 @ 0x04: u16 = 0_0 {
        /// PMAP45_0
        PMAP45_0: 0 = struct PMAP45_0(bool);
        /// PMAP45_1
        PMAP45_1: 1 = struct PMAP45_1(bool);
        /// PMAP45_2
        PMAP45_2: 2 = struct PMAP45_2(bool);
        /// PMAP45_3
        PMAP45_3: 3 = struct PMAP45_3(bool);
        /// PMAP45_4
        PMAP45_4: 4 = struct PMAP45_4(bool);
        /// PMAP45_5
        PMAP45_5: 5 = struct PMAP45_5(bool);
        /// PMAP45_6
        PMAP45_6: 6 = struct PMAP45_6(bool);
        /// PMAP45_7
        PMAP45_7: 7 = struct PMAP45_7(bool);
        /// PMAP45_8
        PMAP45_8: 8 = struct PMAP45_8(bool);
        /// PMAP45_9
        PMAP45_9: 9 = struct PMAP45_9(bool);
        /// PMAP45_10
        PMAP45_10: 10 = struct PMAP45_10(bool);
        /// PMAP45_11
        PMAP45_11: 11 = struct PMAP45_11(bool);
        /// PMAP45_12
        PMAP45_12: 12 = struct PMAP45_12(bool);
        /// PMAP45_13
        PMAP45_13: 13 = struct PMAP45_13(bool);
        /// PMAP45_14
        PMAP45_14: 14 = struct PMAP45_14(bool);
        /// PMAP45_15
        PMAP45_15: 15 = struct PMAP45_15(bool);
    }
    /// Port Px.6/7 mapping register
    rw PxMAP67 @ 0x06: u16 = 0_0 {
        /// PMAP67_0
        PMAP67_0: 0 = struct PMAP67_0(bool);
        /// PMAP67_1
        PMAP67_1: 1 = struct PMAP67_1(bool);
        /// PMAP67_2
        PMAP67_2: 2 = struct PMAP67_2(bool);
        /// PMAP67_3
        PMAP67_3: 3 = struct PMAP67_3(bool);
        /// PMAP67_4
        PMAP67_4: 4 = struct PMAP67_4(bool);
        /// PMAP67_5
        PMAP67_5: 5 = struct PMAP67_5(bool);
        /// PMAP67_6
        PMAP67_6: 6 = struct PMAP67_6(bool);
        /// PMAP67_7
        PMAP67_7: 7 = struct PMAP67_7(bool);
        /// PMAP67_8
        PMAP67_8: 8 = struct PMAP67_8(bool);
        /// PMAP67_9
        PMAP67_9: 9 = struct PMAP67_9(bool);
        /// PMAP67_10
        PMAP67_10: 10 = struct PMAP67_10(bool);
        /// PMAP67_11
        PMAP67_11: 11 = struct PMAP67_11(bool);
        /// PMAP67_12
        PMAP67_12: 12 = struct PMAP67_12(bool);
        /// PMAP67_13
        PMAP67_13: 13 = struct PMAP67_13(bool);
        /// PMAP67_14
        PMAP67_14: 14 = struct PMAP67_14(bool);
        /// PMAP67_15
        PMAP67_15: 15 = struct PMAP67_15(bool);
    }
    /// Port Px.0 mapping register
    rw PMAP0 @ 0x00: u8 = 0_0 {
        /// PMAP0
        PMAP0_PMAP0: 0 = struct PMAP0_PMAP0(bool);
        /// PMAP1
        PMAP0_PMAP1: 1 = struct PMAP0_PMAP1(bool);
        /// PMAP2
        PMAP0_PMAP2: 2 = struct PMAP0_PMAP2(bool);
        /// PMAP3
        PMAP0_PMAP3: 3 = struct PMAP0_PMAP3(bool);
        /// PMAP4
        PMAP0_PMAP4: 4 = struct PMAP0_PMAP4(bool);
        /// PMAP5
        PMAP0_PMAP5: 5 = struct PMAP0_PMAP5(bool);
        /// PMAP6
        PMAP0_PMAP6: 6 = struct PMAP0_PMAP6(bool);
        /// PMAP7
        PMAP0_PMAP7: 7 = struct PMAP0_PMAP7(bool);
    }
    /// Port Px.1 mapping register
    rw PMAP1 @ 0x01: u8 = 0_0 {
        /// PMAP0
        PMAP1_PMAP0: 0 = struct PMAP1_PMAP0(bool);
        /// PMAP1
        PMAP1_PMAP1: 1 = struct PMAP1_PMAP1(bool);
        /// PMAP2
        PMAP1_PMAP2: 2 = struct PMAP1_PMAP2(bool);
        /// PMAP3
        PMAP1_PMAP3: 3 = struct PMAP1_PMAP3(bool);
        /// PMAP4
        PMAP1_PMAP4: 4 = struct PMAP1_PMAP4(bool);
        /// PMAP5
        PMAP1_PMAP5: 5 = struct PMAP1_PMAP5(bool);
        /// PMAP6
        PMAP1_PMAP6: 6 = struct PMAP1_PMAP6(bool);
        /// PMAP7
        PMAP1_PMAP7: 7 = struct PMAP1_PMAP7(bool);
    }
    /// Port Px.2 mapping register
    rw PMAP2 @ 0x02: u8 = 0_0 {
        /// PMAP0
        PMAP2_PMAP0: 0 = struct PMAP2_PMAP0(bool);
        /// PMAP
        PMAP2_PMAP1: 1 = struct PMAP2_PMAP1(bool);
        /// PMAP2
        PMAP2_PMAP2: 2 = struct PMAP2_PMAP2(bool);
        /// PMAP3
        PMAP2_PMAP3: 3 = struct PMAP2_PMAP3(bool);
        /// PMAP4
        PMAP2_PMAP4: 4 = struct PMAP2_PMAP4(bool);
        /// PMAP5
        PMAP2_PMAP5: 5 = struct PMAP2_PMAP5(bool);
        /// PMAP6
        PMAP2_PMAP6: 6 = struct PMAP2_PMAP6(bool);
        /// PMAP7
        PMAP2_PMAP7: 7 = struct PMAP2_PMAP7(bool);
    }
    /// Port Px.3 mapping register
    rw PMAP3 @ 0x03: u8 = 0_0 {
        /// PMAP0
        PMAP3_PMAP0: 0 = struct PMAP3_PMAP0(bool);
        /// PMAP1
        PMAP3_PMAP1: 1 = struct PMAP3_PMAP1(bool);
        /// PMAP2
        PMAP3_PMAP2: 2 = struct PMAP3_PMAP2(bool);
        /// PMAP3
        PMAP3_PMAP3: 3 = struct PMAP3_PMAP3(bool);
        /// PMAP4
        PMAP3_PMAP4: 4 = struct PMAP3_PMAP4(bool);
        /// PMAP5
        PMAP3_PMAP5: 5 = struct PMAP3_PMAP5(bool);
        /// PMAP6
        PMAP3_PMAP6: 6 = struct PMAP3_PMAP6(bool);
        /// PMAP7
        PMAP3_PMAP7: 7 = struct PMAP3_PMAP7(bool);
    }
    /// Port Px.4 mapping register
    rw PMAP4 @ 0x04: u8 = 0_0 {
        /// PMAP0
        PMAP4_PMAP0: 0 = struct PMAP4_PMAP0(bool);
        /// PMAP1
        PMAP4_PMAP1: 1 = struct PMAP4_PMAP1(bool);
        /// PMAP2
        PMAP4_PMAP2: 2 = struct PMAP4_PMAP2(bool);
        /// PMAP3
        PMAP4_PMAP3: 3 = struct PMAP4_PMAP3(bool);
        /// PMAP4
        PMAP4_PMAP4: 4 = struct PMAP4_PMAP4(bool);
        /// PMAP5
        PMAP4_PMAP5: 5 = struct PMAP4_PMAP5(bool);
        /// PMAP6
        PMAP4_PMAP6: 6 = struct PMAP4_PMAP6(bool);
        /// PMAP7
        PMAP4_PMAP7: 7 = struct PMAP4_PMAP7(bool);
    }
    /// Port Px.5 mapping register
    rw PMAP5 @ 0x05: u8 = 0_0 {
        /// PMAP0
        PMAP5_PMAP0: 0 = struct PMAP5_PMAP0(bool);
        /// PMAP1
        PMAP5_PMAP1: 1 = struct PMAP5_PMAP1(bool);
        /// PMAP2
        PMAP5_PMAP2: 2 = struct PMAP5_PMAP2(bool);
        /// PMAP3
        PMAP5_PMAP3: 3 = struct PMAP5_PMAP3(bool);
        /// PMAP4
        PMAP5_PMAP4: 4 = struct PMAP5_PMAP4(bool);
        /// PMAP5
        PMAP5_PMAP5: 5 = struct PMAP5_PMAP5(bool);
        /// PMAP6
        PMAP5_PMAP6: 6 = struct PMAP5_PMAP6(bool);
        /// PMAP7
        PMAP5_PMAP7: 7 = struct PMAP5_PMAP7(bool);
    }
    /// Port Px.6 mapping register
    rw PMAP6 @ 0x06: u8 = 0_0 {
        /// PMAP0
        PMAP6_PMAP0: 0 = struct PMAP6_PMAP0(bool);
        /// PMAP1
        PMAP6_PMAP1: 1 = struct PMAP6_PMAP1(bool);
        /// PMAP2
        PMAP6_PMAP2: 2 = struct PMAP6_PMAP2(bool);
        /// PMAP3
        PMAP6_PMAP3: 3 = struct PMAP6_PMAP3(bool);
        /// PMAP4
        PMAP6_PMAP4: 4 = struct PMAP6_PMAP4(bool);
        /// PMAP5
        PMAP6_PMAP5: 5 = struct PMAP6_PMAP5(bool);
        /// PMAP6
        PMAP6_PMAP6: 6 = struct PMAP6_PMAP6(bool);
        /// PMAP7
        PMAP6_PMAP7: 7 = struct PMAP6_PMAP7(bool);
    }
    /// Port Px.7 mapping register
    rw PMAP7 @ 0x07: u8 = 0_0 {
        /// PMAP0
        PMAP7_PMAP0: 0 = struct PMAP7_PMAP0(bool);
        /// PMAP1
        PMAP7_PMAP1: 1 = struct PMAP7_PMAP1(bool);
        /// PMAP2
        PMAP7_PMAP2: 2 = struct PMAP7_PMAP2(bool);
        /// PMAP3
        PMAP7_PMAP3: 3 = struct PMAP7_PMAP3(bool);
        /// PMAP4
        PMAP7_PMAP4: 4 = struct PMAP7_PMAP4(bool);
        /// PMAP5
        PMAP7_PMAP5: 5 = struct PMAP7_PMAP5(bool);
        /// PMAP6
        PMAP7_PMAP6: 6 = struct PMAP7_PMAP6(bool);
        /// PMAP7
        PMAP7_PMAP7: 7 = struct PMAP7_PMAP7(bool);
    }
}
