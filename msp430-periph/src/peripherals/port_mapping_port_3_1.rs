//! Port Mapping Port 3

utils::periph! {
    /// Port Mapping Port 3
    PortMappingPort3;
    /// Port P3.0/1 mapping register
    rw P3MAP01 @ 0x00: u16 = 0_0 {
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
    /// Port P3.2/3 mapping register
    rw P3MAP23 @ 0x02: u16 = 0_0 {
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
    /// Port P3.4/5 mapping register
    rw P3MAP45 @ 0x04: u16 = 0_0 {
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
    /// Port P3.6/7 mapping register
    rw P3MAP67 @ 0x06: u16 = 0_0 {
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
    /// Port P3.0 mapping register
    rw P3MAP0 @ 0x00: u8 = 0_0 {
        /// PMAP0
        P3MAP0_PMAP0: 0 = struct P3MAP0_PMAP0(bool);
        /// PMAP1
        P3MAP0_PMAP1: 1 = struct P3MAP0_PMAP1(bool);
        /// PMAP2
        P3MAP0_PMAP2: 2 = struct P3MAP0_PMAP2(bool);
        /// PMAP3
        P3MAP0_PMAP3: 3 = struct P3MAP0_PMAP3(bool);
        /// PMAP4
        P3MAP0_PMAP4: 4 = struct P3MAP0_PMAP4(bool);
        /// PMAP5
        P3MAP0_PMAP5: 5 = struct P3MAP0_PMAP5(bool);
        /// PMAP6
        P3MAP0_PMAP6: 6 = struct P3MAP0_PMAP6(bool);
        /// PMAP7
        P3MAP0_PMAP7: 7 = struct P3MAP0_PMAP7(bool);
    }
    /// Port P3.1 mapping register
    rw P3MAP1 @ 0x01: u8 = 0_0 {
        /// PMAP0
        P3MAP1_PMAP0: 0 = struct P3MAP1_PMAP0(bool);
        /// PMAP1
        P3MAP1_PMAP1: 1 = struct P3MAP1_PMAP1(bool);
        /// PMAP2
        P3MAP1_PMAP2: 2 = struct P3MAP1_PMAP2(bool);
        /// PMAP3
        P3MAP1_PMAP3: 3 = struct P3MAP1_PMAP3(bool);
        /// PMAP4
        P3MAP1_PMAP4: 4 = struct P3MAP1_PMAP4(bool);
        /// PMAP5
        P3MAP1_PMAP5: 5 = struct P3MAP1_PMAP5(bool);
        /// PMAP6
        P3MAP1_PMAP6: 6 = struct P3MAP1_PMAP6(bool);
        /// PMAP7
        P3MAP1_PMAP7: 7 = struct P3MAP1_PMAP7(bool);
    }
    /// Port P3.2 mapping register
    rw P3MAP2 @ 0x02: u8 = 0_0 {
        /// PMAP0
        P3MAP2_PMAP0: 0 = struct P3MAP2_PMAP0(bool);
        /// PMAP1
        P3MAP2_PMAP1: 1 = struct P3MAP2_PMAP1(bool);
        /// PMAP2
        P3MAP2_PMAP2: 2 = struct P3MAP2_PMAP2(bool);
        /// PMAP3
        P3MAP2_PMAP3: 3 = struct P3MAP2_PMAP3(bool);
        /// PMAP4
        P3MAP2_PMAP4: 4 = struct P3MAP2_PMAP4(bool);
        /// PMAP5
        P3MAP2_PMAP5: 5 = struct P3MAP2_PMAP5(bool);
        /// PMAP6
        P3MAP2_PMAP6: 6 = struct P3MAP2_PMAP6(bool);
        /// PMAP7
        P3MAP2_PMAP7: 7 = struct P3MAP2_PMAP7(bool);
    }
    /// Port P3.3 mapping register
    rw P3MAP3 @ 0x03: u8 = 0_0 {
        /// PMAP0
        P3MAP3_PMAP0: 0 = struct P3MAP3_PMAP0(bool);
        /// PMAP1
        P3MAP3_PMAP1: 1 = struct P3MAP3_PMAP1(bool);
        /// PMAP2
        P3MAP3_PMAP2: 2 = struct P3MAP3_PMAP2(bool);
        /// PMAP3
        P3MAP3_PMAP3: 3 = struct P3MAP3_PMAP3(bool);
        /// PMAP4
        P3MAP3_PMAP4: 4 = struct P3MAP3_PMAP4(bool);
        /// PMAP5
        P3MAP3_PMAP5: 5 = struct P3MAP3_PMAP5(bool);
        /// PMAP6
        P3MAP3_PMAP6: 6 = struct P3MAP3_PMAP6(bool);
        /// PMAP7
        P3MAP3_PMAP7: 7 = struct P3MAP3_PMAP7(bool);
    }
    /// Port P3.4 mapping register
    rw P3MAP4 @ 0x04: u8 = 0_0 {
        /// PMAP0
        P3MAP4_PMAP0: 0 = struct P3MAP4_PMAP0(bool);
        /// PMAP1
        P3MAP4_PMAP1: 1 = struct P3MAP4_PMAP1(bool);
        /// PMAP2
        P3MAP4_PMAP2: 2 = struct P3MAP4_PMAP2(bool);
        /// PMAP3
        P3MAP4_PMAP3: 3 = struct P3MAP4_PMAP3(bool);
        /// PMAP4
        P3MAP4_PMAP4: 4 = struct P3MAP4_PMAP4(bool);
        /// PMAP5
        P3MAP4_PMAP5: 5 = struct P3MAP4_PMAP5(bool);
        /// PMAP6
        P3MAP4_PMAP6: 6 = struct P3MAP4_PMAP6(bool);
        /// PMAP7
        P3MAP4_PMAP7: 7 = struct P3MAP4_PMAP7(bool);
    }
    /// Port P3.5 mapping register
    rw P3MAP5 @ 0x05: u8 = 0_0 {
        /// PMAP0
        P3MAP5_PMAP0: 0 = struct P3MAP5_PMAP0(bool);
        /// PMAP1
        P3MAP5_PMAP1: 1 = struct P3MAP5_PMAP1(bool);
        /// PMAP2
        P3MAP5_PMAP2: 2 = struct P3MAP5_PMAP2(bool);
        /// PMAP3
        P3MAP5_PMAP3: 3 = struct P3MAP5_PMAP3(bool);
        /// PMAP4
        P3MAP5_PMAP4: 4 = struct P3MAP5_PMAP4(bool);
        /// PMAP5
        P3MAP5_PMAP5: 5 = struct P3MAP5_PMAP5(bool);
        /// PMAP6
        P3MAP5_PMAP6: 6 = struct P3MAP5_PMAP6(bool);
        /// PMAP7
        P3MAP5_PMAP7: 7 = struct P3MAP5_PMAP7(bool);
    }
    /// Port P3.6 mapping register
    rw P3MAP6 @ 0x06: u8 = 0_0 {
        /// PMAP0
        P3MAP6_PMAP0: 0 = struct P3MAP6_PMAP0(bool);
        /// PMAP1
        P3MAP6_PMAP1: 1 = struct P3MAP6_PMAP1(bool);
        /// PMAP2
        P3MAP6_PMAP2: 2 = struct P3MAP6_PMAP2(bool);
        /// PMAP3
        P3MAP6_PMAP3: 3 = struct P3MAP6_PMAP3(bool);
        /// PMAP4
        P3MAP6_PMAP4: 4 = struct P3MAP6_PMAP4(bool);
        /// PMAP5
        P3MAP6_PMAP5: 5 = struct P3MAP6_PMAP5(bool);
        /// PMAP6
        P3MAP6_PMAP6: 6 = struct P3MAP6_PMAP6(bool);
        /// PMAP7
        P3MAP6_PMAP7: 7 = struct P3MAP6_PMAP7(bool);
    }
    /// Port P3.7 mapping register
    rw P3MAP7 @ 0x07: u8 = 0_0 {
        /// PMAP0
        P3MAP7_PMAP0: 0 = struct P3MAP7_PMAP0(bool);
        /// PMAP1
        P3MAP7_PMAP1: 1 = struct P3MAP7_PMAP1(bool);
        /// PMAP2
        P3MAP7_PMAP2: 2 = struct P3MAP7_PMAP2(bool);
        /// PMAP3
        P3MAP7_PMAP3: 3 = struct P3MAP7_PMAP3(bool);
        /// PMAP4
        P3MAP7_PMAP4: 4 = struct P3MAP7_PMAP4(bool);
        /// PMAP5
        P3MAP7_PMAP5: 5 = struct P3MAP7_PMAP5(bool);
        /// PMAP6
        P3MAP7_PMAP6: 6 = struct P3MAP7_PMAP6(bool);
        /// PMAP7
        P3MAP7_PMAP7: 7 = struct P3MAP7_PMAP7(bool);
    }
}
