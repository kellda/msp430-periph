//! MPU

utils::periph! {
    /// MPU
    MPU;
    /// MPU Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// MPU Enable
        ENA: 0 = struct ENA(bool);
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
    }
    /// MPU Segmentation Register
    rw SEG @ 0x04: u16 = 0_0 {
        /// MPU Segment Border 1 Bit: 0
        SB10: 0 = struct SB10(bool);
        /// MPU Segment Border 1 Bit: 1
        SB11: 1 = struct SB11(bool);
        /// MPU Segment Border 1 Bit: 2
        SB12: 2 = struct SB12(bool);
        /// MPU Segment Border 1 Bit: 3
        SB13: 3 = struct SB13(bool);
        /// MPU Segment Border 1 Bit: 4
        SB14: 4 = struct SB14(bool);
        /// MPU Segment Border 2 Bit: 0
        SB20: 8 = struct SB20(bool);
        /// MPU Segment Border 2 Bit: 1
        SB21: 9 = struct SB21(bool);
        /// MPU Segment Border 2 Bit: 2
        SB22: 10 = struct SB22(bool);
        /// MPU Segment Border 2 Bit: 3
        SB23: 11 = struct SB23(bool);
        /// MPU Segment Border 2 Bit: 4
        SB24: 12 = struct SB24(bool);
    }
    /// MPU Access Management Register
    rw SAM @ 0x06: u16 = 0_0 {
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
}
