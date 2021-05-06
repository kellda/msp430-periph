//! MPU

utils::periph! {
    /// MPU
    MPU;
    /// MPU Control Register 0
    rw MPUCTL0 @ 0x00: u16 = 0_0 {
        /// MPU Enable
        MPUENA: 0 = struct MPUENA(bool);
        /// MPU Lock
        MPULOCK: 1 = struct MPULOCK(bool);
        /// MPU Enable NMI on Segment violation
        MPUSEGIE: 4 = struct MPUSEGIE(bool);
    }
    /// MPU Control Register 1
    rw MPUCTL1 @ 0x02: u16 = 0_0 {
        /// MPU Main Memory Segment 1 violation interupt flag
        MPUSEG1IFG: 0 = struct MPUSEG1IFG(bool);
        /// MPU Main Memory Segment 2 violation interupt flag
        MPUSEG2IFG: 1 = struct MPUSEG2IFG(bool);
        /// MPU Main Memory Segment 3 violation interupt flag
        MPUSEG3IFG: 2 = struct MPUSEG3IFG(bool);
        /// MPU Info Memory Segment violation interupt flag
        MPUSEGIIFG: 3 = struct MPUSEGIIFG(bool);
        /// MPU IP Memory Segment violation interupt flag
        MPUSEGIPIFG: 4 = struct MPUSEGIPIFG(bool);
    }
    /// MPU Segmentation Border 2 Register
    rw MPUSEGB2 @ 0x04: u16 = 0_0 {
        /// MPU Segment Border 2 Bit: 0
        MPUSEGB20: 0 = struct MPUSEGB20(bool);
        /// MPU Segment Border 2 Bit: 1
        MPUSEGB21: 1 = struct MPUSEGB21(bool);
        /// MPU Segment Border 2 Bit: 2
        MPUSEGB22: 2 = struct MPUSEGB22(bool);
        /// MPU Segment Border 2 Bit: 3
        MPUSEGB23: 3 = struct MPUSEGB23(bool);
        /// MPU Segment Border 2 Bit: 4
        MPUSEGB24: 4 = struct MPUSEGB24(bool);
        /// MPU Segment Border 2 Bit: 5
        MPUSEGB25: 5 = struct MPUSEGB25(bool);
        /// MPU Segment Border 2 Bit: 6
        MPUSEGB26: 6 = struct MPUSEGB26(bool);
        /// MPU Segment Border 2 Bit: 7
        MPUSEGB27: 7 = struct MPUSEGB27(bool);
        /// MPU Segment Border 2 Bit: 8
        MPUSEGB28: 8 = struct MPUSEGB28(bool);
        /// MPU Segment Border 2 Bit: 9
        MPUSEGB29: 9 = struct MPUSEGB29(bool);
        /// MPU Segment Border 2 Bit: 10
        MPUSEGB210: 10 = struct MPUSEGB210(bool);
        /// MPU Segment Border 2 Bit: 11
        MPUSEGB211: 11 = struct MPUSEGB211(bool);
        /// MPU Segment Border 2 Bit: 12
        MPUSEGB212: 12 = struct MPUSEGB212(bool);
        /// MPU Segment Border 2 Bit: 13
        MPUSEGB213: 13 = struct MPUSEGB213(bool);
        /// MPU Segment Border 2 Bit: 14
        MPUSEGB214: 14 = struct MPUSEGB214(bool);
        /// MPU Segment Border 2 Bit: 15
        MPUSEGB215: 15 = struct MPUSEGB215(bool);
    }
    /// MPU Segmentation Border 1 Register
    rw MPUSEGB1 @ 0x06: u16 = 0_0 {
        /// MPU Segment Border 1 Bit: 0
        MPUSEGB10: 0 = struct MPUSEGB10(bool);
        /// MPU Segment Border 1 Bit: 1
        MPUSEGB11: 1 = struct MPUSEGB11(bool);
        /// MPU Segment Border 1 Bit: 2
        MPUSEGB12: 2 = struct MPUSEGB12(bool);
        /// MPU Segment Border 1 Bit: 3
        MPUSEGB13: 3 = struct MPUSEGB13(bool);
        /// MPU Segment Border 1 Bit: 4
        MPUSEGB14: 4 = struct MPUSEGB14(bool);
        /// MPU Segment Border 1 Bit: 5
        MPUSEGB15: 5 = struct MPUSEGB15(bool);
        /// MPU Segment Border 1 Bit: 6
        MPUSEGB16: 6 = struct MPUSEGB16(bool);
        /// MPU Segment Border 1 Bit: 7
        MPUSEGB17: 7 = struct MPUSEGB17(bool);
        /// MPU Segment Border 1 Bit: 8
        MPUSEGB18: 8 = struct MPUSEGB18(bool);
        /// MPU Segment Border 1 Bit: 9
        MPUSEGB19: 9 = struct MPUSEGB19(bool);
        /// MPU Segment Border 1 Bit: 10
        MPUSEGB110: 10 = struct MPUSEGB110(bool);
        /// MPU Segment Border 1 Bit: 11
        MPUSEGB111: 11 = struct MPUSEGB111(bool);
        /// MPU Segment Border 1 Bit: 12
        MPUSEGB112: 12 = struct MPUSEGB112(bool);
        /// MPU Segment Border 1 Bit: 13
        MPUSEGB113: 13 = struct MPUSEGB113(bool);
        /// MPU Segment Border 1 Bit: 14
        MPUSEGB114: 14 = struct MPUSEGB114(bool);
        /// MPU Segment Border 1 Bit: 15
        MPUSEGB115: 15 = struct MPUSEGB115(bool);
    }
    /// MPU Access Management Register
    rw MPUSAM @ 0x08: u16 = 0_0 {
        /// MPU Main memory Segment 1 Read enable
        MPUSEG1RE: 0 = struct MPUSEG1RE(bool);
        /// MPU Main memory Segment 1 Write enable
        MPUSEG1WE: 1 = struct MPUSEG1WE(bool);
        /// MPU Main memory Segment 1 Execute enable
        MPUSEG1XE: 2 = struct MPUSEG1XE(bool);
        /// MPU Main memory Segment 1 Violation select
        MPUSEG1VS: 3 = struct MPUSEG1VS(bool);
        /// MPU Main memory Segment 2 Read enable
        MPUSEG2RE: 4 = struct MPUSEG2RE(bool);
        /// MPU Main memory Segment 2 Write enable
        MPUSEG2WE: 5 = struct MPUSEG2WE(bool);
        /// MPU Main memory Segment 2 Execute enable
        MPUSEG2XE: 6 = struct MPUSEG2XE(bool);
        /// MPU Main memory Segment 2 Violation select
        MPUSEG2VS: 7 = struct MPUSEG2VS(bool);
        /// MPU Main memory Segment 3 Read enable
        MPUSEG3RE: 8 = struct MPUSEG3RE(bool);
        /// MPU Main memory Segment 3 Write enable
        MPUSEG3WE: 9 = struct MPUSEG3WE(bool);
        /// MPU Main memory Segment 3 Execute enable
        MPUSEG3XE: 10 = struct MPUSEG3XE(bool);
        /// MPU Main memory Segment 3 Violation select
        MPUSEG3VS: 11 = struct MPUSEG3VS(bool);
        /// MPU Info memory Segment Read enable
        MPUSEGIRE: 12 = struct MPUSEGIRE(bool);
        /// MPU Info memory Segment Write enable
        MPUSEGIWE: 13 = struct MPUSEGIWE(bool);
        /// MPU Info memory Segment Execute enable
        MPUSEGIXE: 14 = struct MPUSEGIXE(bool);
        /// MPU Info memory Segment Violation select
        MPUSEGIVS: 15 = struct MPUSEGIVS(bool);
    }
    /// MPU IP Control 0 Register
    rw MPUIPC0 @ 0x0a: u16 = 0_0 {
        /// MPU MPU IP protection segment Violation Select
        MPUIPVS: 5 = struct MPUIPVS(bool);
        /// MPU MPU IP Protection Enable
        MPUIPENA: 6 = struct MPUIPENA(bool);
        /// MPU IP Protection Lock
        MPUIPLOCK: 7 = struct MPUIPLOCK(bool);
    }
    /// MPU IP Segment Border 2 Register
    rw MPUIPSEGB2 @ 0x0c: u16 = 0_0 {
        /// MPU IP Segment Border 2 Bit: 0
        MPUIPSEGB20: 0 = struct MPUIPSEGB20(bool);
        /// MPU IP Segment Border 2 Bit: 1
        MPUIPSEGB21: 1 = struct MPUIPSEGB21(bool);
        /// MPU IP Segment Border 2 Bit: 2
        MPUIPSEGB22: 2 = struct MPUIPSEGB22(bool);
        /// MPU IP Segment Border 2 Bit: 3
        MPUIPSEGB23: 3 = struct MPUIPSEGB23(bool);
        /// MPU IP Segment Border 2 Bit: 4
        MPUIPSEGB24: 4 = struct MPUIPSEGB24(bool);
        /// MPU IP Segment Border 2 Bit: 5
        MPUIPSEGB25: 5 = struct MPUIPSEGB25(bool);
        /// MPU IP Segment Border 2 Bit: 6
        MPUIPSEGB26: 6 = struct MPUIPSEGB26(bool);
        /// MPU IP Segment Border 2 Bit: 7
        MPUIPSEGB27: 7 = struct MPUIPSEGB27(bool);
        /// MPU IP Segment Border 2 Bit: 8
        MPUIPSEGB28: 8 = struct MPUIPSEGB28(bool);
        /// MPU IP Segment Border 2 Bit: 9
        MPUIPSEGB29: 9 = struct MPUIPSEGB29(bool);
        /// MPU IP Segment Border 2 Bit: 10
        MPUIPSEGB210: 10 = struct MPUIPSEGB210(bool);
        /// MPU IP Segment Border 2 Bit: 11
        MPUIPSEGB211: 11 = struct MPUIPSEGB211(bool);
        /// MPU IP Segment Border 2 Bit: 12
        MPUIPSEGB212: 12 = struct MPUIPSEGB212(bool);
        /// MPU IP Segment Border 2 Bit: 13
        MPUIPSEGB213: 13 = struct MPUIPSEGB213(bool);
        /// MPU IP Segment Border 2 Bit: 14
        MPUIPSEGB214: 14 = struct MPUIPSEGB214(bool);
        /// MPU IP Segment Border 2 Bit: 15
        MPUIPSEGB215: 15 = struct MPUIPSEGB215(bool);
    }
    /// MPU IP Segment Border 1 Register
    rw MPUIPSEGB1 @ 0x0e: u16 = 0_0 {
        /// MPU IP Segment Border 1 Bit: 0
        MPUIPSEGB10: 0 = struct MPUIPSEGB10(bool);
        /// MPU IP Segment Border 1 Bit: 1
        MPUIPSEGB11: 1 = struct MPUIPSEGB11(bool);
        /// MPU IP Segment Border 1 Bit: 2
        MPUIPSEGB12: 2 = struct MPUIPSEGB12(bool);
        /// MPU IP Segment Border 1 Bit: 3
        MPUIPSEGB13: 3 = struct MPUIPSEGB13(bool);
        /// MPU IP Segment Border 1 Bit: 4
        MPUIPSEGB14: 4 = struct MPUIPSEGB14(bool);
        /// MPU IP Segment Border 1 Bit: 5
        MPUIPSEGB15: 5 = struct MPUIPSEGB15(bool);
        /// MPU IP Segment Border 1 Bit: 6
        MPUIPSEGB16: 6 = struct MPUIPSEGB16(bool);
        /// MPU IP Segment Border 1 Bit: 7
        MPUIPSEGB17: 7 = struct MPUIPSEGB17(bool);
        /// MPU IP Segment Border 1 Bit: 8
        MPUIPSEGB18: 8 = struct MPUIPSEGB18(bool);
        /// MPU IP Segment Border 1 Bit: 9
        MPUIPSEGB19: 9 = struct MPUIPSEGB19(bool);
        /// MPU IP Segment Border 1 Bit: 10
        MPUIPSEGB110: 10 = struct MPUIPSEGB110(bool);
        /// MPU IP Segment Border 1 Bit: 11
        MPUIPSEGB111: 11 = struct MPUIPSEGB111(bool);
        /// MPU IP Segment Border 1 Bit: 12
        MPUIPSEGB112: 12 = struct MPUIPSEGB112(bool);
        /// MPU IP Segment Border 1 Bit: 13
        MPUIPSEGB113: 13 = struct MPUIPSEGB113(bool);
        /// MPU IP Segment Border 1 Bit: 14
        MPUIPSEGB114: 14 = struct MPUIPSEGB114(bool);
        /// MPU IP Segment Border 1 Bit: 15
        MPUIPSEGB115: 15 = struct MPUIPSEGB115(bool);
    }
}
