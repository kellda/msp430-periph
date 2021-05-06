//! MPU

utils::periph! {
    /// MPU
    MPU;
    /// MPU Control Register 0
    rw MPUCTL0 @ 0x00: u16 = 0_0 {
        /// MPU Enable
        MPUENA: 0 = struct MPUENA(bool);
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
    }
    /// MPU Segmentation Register
    rw MPUSEG @ 0x04: u16 = 0_0 {
        /// MPU Segment Border 1 Bit: 0
        MPUSB10: 0 = struct MPUSB10(bool);
        /// MPU Segment Border 1 Bit: 1
        MPUSB11: 1 = struct MPUSB11(bool);
        /// MPU Segment Border 1 Bit: 2
        MPUSB12: 2 = struct MPUSB12(bool);
        /// MPU Segment Border 1 Bit: 3
        MPUSB13: 3 = struct MPUSB13(bool);
        /// MPU Segment Border 1 Bit: 4
        MPUSB14: 4 = struct MPUSB14(bool);
        /// MPU Segment Border 2 Bit: 0
        MPUSB20: 8 = struct MPUSB20(bool);
        /// MPU Segment Border 2 Bit: 1
        MPUSB21: 9 = struct MPUSB21(bool);
        /// MPU Segment Border 2 Bit: 2
        MPUSB22: 10 = struct MPUSB22(bool);
        /// MPU Segment Border 2 Bit: 3
        MPUSB23: 11 = struct MPUSB23(bool);
        /// MPU Segment Border 2 Bit: 4
        MPUSB24: 12 = struct MPUSB24(bool);
    }
    /// MPU Access Management Register
    rw MPUSAM @ 0x06: u16 = 0_0 {
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
}
