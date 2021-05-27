//! MPU

utils::periph! {
    /// MPU
    MPU;
    /// MPU Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// MPU Enable
        ENA: 0 = struct ENA(bool);
        /// MPU Lock
        LOCK: 1 = struct LOCK(bool);
        /// MPU Enable NMI on Segment violation
        SEGIE: 4 = struct SEGIE(bool);
    }
    /// MPU Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// MPU Main Memory Segment 1 violation interupt flag
        SEG1IFG: 0 = struct SEG1IFG(bool);
        /// MPU Main Memory Segment 2 violation interupt flag
        SEG2IFG: 1 = struct SEG2IFG(bool);
        /// MPU Main Memory Segment 3 violation interupt flag
        SEG3IFG: 2 = struct SEG3IFG(bool);
        /// MPU Info Memory Segment violation interupt flag
        SEGIIFG: 3 = struct SEGIIFG(bool);
        /// MPU IP Memory Segment violation interupt flag
        SEGIPIFG: 4 = struct SEGIPIFG(bool);
    }
    /// MPU Segmentation Border 2 Register
    rw SEGB2 @ 0x04: u16 = 0_0 {
        /// MPU Segment Border 2 Bit: 0
        SEGB20: 0 = struct SEGB20(bool);
        /// MPU Segment Border 2 Bit: 1
        SEGB21: 1 = struct SEGB21(bool);
        /// MPU Segment Border 2 Bit: 2
        SEGB22: 2 = struct SEGB22(bool);
        /// MPU Segment Border 2 Bit: 3
        SEGB23: 3 = struct SEGB23(bool);
        /// MPU Segment Border 2 Bit: 4
        SEGB24: 4 = struct SEGB24(bool);
        /// MPU Segment Border 2 Bit: 5
        SEGB25: 5 = struct SEGB25(bool);
        /// MPU Segment Border 2 Bit: 6
        SEGB26: 6 = struct SEGB26(bool);
        /// MPU Segment Border 2 Bit: 7
        SEGB27: 7 = struct SEGB27(bool);
        /// MPU Segment Border 2 Bit: 8
        SEGB28: 8 = struct SEGB28(bool);
        /// MPU Segment Border 2 Bit: 9
        SEGB29: 9 = struct SEGB29(bool);
        /// MPU Segment Border 2 Bit: 10
        SEGB210: 10 = struct SEGB210(bool);
        /// MPU Segment Border 2 Bit: 11
        SEGB211: 11 = struct SEGB211(bool);
        /// MPU Segment Border 2 Bit: 12
        SEGB212: 12 = struct SEGB212(bool);
        /// MPU Segment Border 2 Bit: 13
        SEGB213: 13 = struct SEGB213(bool);
        /// MPU Segment Border 2 Bit: 14
        SEGB214: 14 = struct SEGB214(bool);
        /// MPU Segment Border 2 Bit: 15
        SEGB215: 15 = struct SEGB215(bool);
    }
    /// MPU Segmentation Border 1 Register
    rw SEGB1 @ 0x06: u16 = 0_0 {
        /// MPU Segment Border 1 Bit: 0
        SEGB10: 0 = struct SEGB10(bool);
        /// MPU Segment Border 1 Bit: 1
        SEGB11: 1 = struct SEGB11(bool);
        /// MPU Segment Border 1 Bit: 2
        SEGB12: 2 = struct SEGB12(bool);
        /// MPU Segment Border 1 Bit: 3
        SEGB13: 3 = struct SEGB13(bool);
        /// MPU Segment Border 1 Bit: 4
        SEGB14: 4 = struct SEGB14(bool);
        /// MPU Segment Border 1 Bit: 5
        SEGB15: 5 = struct SEGB15(bool);
        /// MPU Segment Border 1 Bit: 6
        SEGB16: 6 = struct SEGB16(bool);
        /// MPU Segment Border 1 Bit: 7
        SEGB17: 7 = struct SEGB17(bool);
        /// MPU Segment Border 1 Bit: 8
        SEGB18: 8 = struct SEGB18(bool);
        /// MPU Segment Border 1 Bit: 9
        SEGB19: 9 = struct SEGB19(bool);
        /// MPU Segment Border 1 Bit: 10
        SEGB110: 10 = struct SEGB110(bool);
        /// MPU Segment Border 1 Bit: 11
        SEGB111: 11 = struct SEGB111(bool);
        /// MPU Segment Border 1 Bit: 12
        SEGB112: 12 = struct SEGB112(bool);
        /// MPU Segment Border 1 Bit: 13
        SEGB113: 13 = struct SEGB113(bool);
        /// MPU Segment Border 1 Bit: 14
        SEGB114: 14 = struct SEGB114(bool);
        /// MPU Segment Border 1 Bit: 15
        SEGB115: 15 = struct SEGB115(bool);
    }
    /// MPU Access Management Register
    rw SAM @ 0x08: u16 = 0_0 {
        /// MPU Main memory Segment 1 Read enable
        SEG1RE: 0 = struct SEG1RE(bool);
        /// MPU Main memory Segment 1 Write enable
        SEG1WE: 1 = struct SEG1WE(bool);
        /// MPU Main memory Segment 1 Execute enable
        SEG1XE: 2 = struct SEG1XE(bool);
        /// MPU Main memory Segment 1 Violation select
        SEG1VS: 3 = struct SEG1VS(bool);
        /// MPU Main memory Segment 2 Read enable
        SEG2RE: 4 = struct SEG2RE(bool);
        /// MPU Main memory Segment 2 Write enable
        SEG2WE: 5 = struct SEG2WE(bool);
        /// MPU Main memory Segment 2 Execute enable
        SEG2XE: 6 = struct SEG2XE(bool);
        /// MPU Main memory Segment 2 Violation select
        SEG2VS: 7 = struct SEG2VS(bool);
        /// MPU Main memory Segment 3 Read enable
        SEG3RE: 8 = struct SEG3RE(bool);
        /// MPU Main memory Segment 3 Write enable
        SEG3WE: 9 = struct SEG3WE(bool);
        /// MPU Main memory Segment 3 Execute enable
        SEG3XE: 10 = struct SEG3XE(bool);
        /// MPU Main memory Segment 3 Violation select
        SEG3VS: 11 = struct SEG3VS(bool);
        /// MPU Info memory Segment Read enable
        SEGIRE: 12 = struct SEGIRE(bool);
        /// MPU Info memory Segment Write enable
        SEGIWE: 13 = struct SEGIWE(bool);
        /// MPU Info memory Segment Execute enable
        SEGIXE: 14 = struct SEGIXE(bool);
        /// MPU Info memory Segment Violation select
        SEGIVS: 15 = struct SEGIVS(bool);
    }
    /// MPU IP Control 0 Register
    rw IPC0 @ 0x0a: u16 = 0_0 {
        /// MPU MPU IP protection segment Violation Select
        IPVS: 5 = struct IPVS(bool);
        /// MPU MPU IP Protection Enable
        IPENA: 6 = struct IPENA(bool);
        /// MPU IP Protection Lock
        IPLOCK: 7 = struct IPLOCK(bool);
    }
    /// MPU IP Segment Border 2 Register
    rw IPSEGB2 @ 0x0c: u16 = 0_0 {
        /// MPU IP Segment Border 2 Bit: 0
        IPSEGB20: 0 = struct IPSEGB20(bool);
        /// MPU IP Segment Border 2 Bit: 1
        IPSEGB21: 1 = struct IPSEGB21(bool);
        /// MPU IP Segment Border 2 Bit: 2
        IPSEGB22: 2 = struct IPSEGB22(bool);
        /// MPU IP Segment Border 2 Bit: 3
        IPSEGB23: 3 = struct IPSEGB23(bool);
        /// MPU IP Segment Border 2 Bit: 4
        IPSEGB24: 4 = struct IPSEGB24(bool);
        /// MPU IP Segment Border 2 Bit: 5
        IPSEGB25: 5 = struct IPSEGB25(bool);
        /// MPU IP Segment Border 2 Bit: 6
        IPSEGB26: 6 = struct IPSEGB26(bool);
        /// MPU IP Segment Border 2 Bit: 7
        IPSEGB27: 7 = struct IPSEGB27(bool);
        /// MPU IP Segment Border 2 Bit: 8
        IPSEGB28: 8 = struct IPSEGB28(bool);
        /// MPU IP Segment Border 2 Bit: 9
        IPSEGB29: 9 = struct IPSEGB29(bool);
        /// MPU IP Segment Border 2 Bit: 10
        IPSEGB210: 10 = struct IPSEGB210(bool);
        /// MPU IP Segment Border 2 Bit: 11
        IPSEGB211: 11 = struct IPSEGB211(bool);
        /// MPU IP Segment Border 2 Bit: 12
        IPSEGB212: 12 = struct IPSEGB212(bool);
        /// MPU IP Segment Border 2 Bit: 13
        IPSEGB213: 13 = struct IPSEGB213(bool);
        /// MPU IP Segment Border 2 Bit: 14
        IPSEGB214: 14 = struct IPSEGB214(bool);
        /// MPU IP Segment Border 2 Bit: 15
        IPSEGB215: 15 = struct IPSEGB215(bool);
    }
    /// MPU IP Segment Border 1 Register
    rw IPSEGB1 @ 0x0e: u16 = 0_0 {
        /// MPU IP Segment Border 1 Bit: 0
        IPSEGB10: 0 = struct IPSEGB10(bool);
        /// MPU IP Segment Border 1 Bit: 1
        IPSEGB11: 1 = struct IPSEGB11(bool);
        /// MPU IP Segment Border 1 Bit: 2
        IPSEGB12: 2 = struct IPSEGB12(bool);
        /// MPU IP Segment Border 1 Bit: 3
        IPSEGB13: 3 = struct IPSEGB13(bool);
        /// MPU IP Segment Border 1 Bit: 4
        IPSEGB14: 4 = struct IPSEGB14(bool);
        /// MPU IP Segment Border 1 Bit: 5
        IPSEGB15: 5 = struct IPSEGB15(bool);
        /// MPU IP Segment Border 1 Bit: 6
        IPSEGB16: 6 = struct IPSEGB16(bool);
        /// MPU IP Segment Border 1 Bit: 7
        IPSEGB17: 7 = struct IPSEGB17(bool);
        /// MPU IP Segment Border 1 Bit: 8
        IPSEGB18: 8 = struct IPSEGB18(bool);
        /// MPU IP Segment Border 1 Bit: 9
        IPSEGB19: 9 = struct IPSEGB19(bool);
        /// MPU IP Segment Border 1 Bit: 10
        IPSEGB110: 10 = struct IPSEGB110(bool);
        /// MPU IP Segment Border 1 Bit: 11
        IPSEGB111: 11 = struct IPSEGB111(bool);
        /// MPU IP Segment Border 1 Bit: 12
        IPSEGB112: 12 = struct IPSEGB112(bool);
        /// MPU IP Segment Border 1 Bit: 13
        IPSEGB113: 13 = struct IPSEGB113(bool);
        /// MPU IP Segment Border 1 Bit: 14
        IPSEGB114: 14 = struct IPSEGB114(bool);
        /// MPU IP Segment Border 1 Bit: 15
        IPSEGB115: 15 = struct IPSEGB115(bool);
    }
}
