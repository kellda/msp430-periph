//! ADC10_AB

utils::periph! {
    /// ADC10_AB
    ADC10_AB;
    /// ADC10 Control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// ADC10 Start Conversion
        SC: 0 = struct SC(bool);
        /// ADC10 Enable Conversion
        ENC: 1 = struct ENC(bool);
        /// ADC10 On/enable
        ON: 4 = struct ON(bool);
        /// ADC10 Multiple SampleConversion
        MSC: 7 = struct MSC(bool);
        /// ADC10 Sample Hold Select Bit: 0
        SHT: 8..11 = enum SHT {
            /// ADC10 Sample Hold Select 0
            SHT_0 = 0b0000,
            /// ADC10 Sample Hold Select 1
            SHT_1 = 0b0001,
            /// ADC10 Sample Hold Select 2
            SHT_2 = 0b0010,
            /// ADC10 Sample Hold Select 3
            SHT_3 = 0b0011,
            /// ADC10 Sample Hold Select 4
            SHT_4 = 0b0100,
            /// ADC10 Sample Hold Select 5
            SHT_5 = 0b0101,
            /// ADC10 Sample Hold Select 6
            SHT_6 = 0b0110,
            /// ADC10 Sample Hold Select 7
            SHT_7 = 0b0111,
            /// ADC10 Sample Hold Select 8
            SHT_8 = 0b1000,
            /// ADC10 Sample Hold Select 9
            SHT_9 = 0b1001,
            /// ADC10 Sample Hold Select 10
            SHT_10 = 0b1010,
            /// ADC10 Sample Hold Select 11
            SHT_11 = 0b1011,
            /// ADC10 Sample Hold Select 12
            SHT_12 = 0b1100,
            /// ADC10 Sample Hold Select 13
            SHT_13 = 0b1101,
            /// ADC10 Sample Hold Select 14
            SHT_14 = 0b1110,
            /// ADC10 Sample Hold Select 15
            SHT_15 = 0b1111,
        }
    }
    /// ADC10 Control 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// ADC10 Busy
        BUSY: 0 = struct BUSY(bool);
        /// ADC10 Conversion Sequence Select 0
        CONSEQ: 1..2 = enum CONSEQ {
            /// ADC10 Conversion Sequence Select: 0
            CONSEQ_0 = 0b00,
            /// ADC10 Conversion Sequence Select: 1
            CONSEQ_1 = 0b01,
            /// ADC10 Conversion Sequence Select: 2
            CONSEQ_2 = 0b10,
            /// ADC10 Conversion Sequence Select: 3
            CONSEQ_3 = 0b11,
        }
        /// ADC10 Clock Source Select 0
        SSEL: 3..4 = enum SSEL {
            /// ADC10 Clock Source Select: 0
            SSEL_0 = 0b00,
            /// ADC10 Clock Source Select: 1
            SSEL_1 = 0b01,
            /// ADC10 Clock Source Select: 2
            SSEL_2 = 0b10,
            /// ADC10 Clock Source Select: 3
            SSEL_3 = 0b11,
        }
        /// ADC10 Clock Divider Select 0
        DIV: 5..7 = enum DIV {
            /// ADC10 Clock Divider Select: 0
            DIV_0 = 0b000,
            /// ADC10 Clock Divider Select: 1
            DIV_1 = 0b001,
            /// ADC10 Clock Divider Select: 2
            DIV_2 = 0b010,
            /// ADC10 Clock Divider Select: 3
            DIV_3 = 0b011,
            /// ADC10 Clock Divider Select: 4
            DIV_4 = 0b100,
            /// ADC10 Clock Divider Select: 5
            DIV_5 = 0b101,
            /// ADC10 Clock Divider Select: 6
            DIV_6 = 0b110,
            /// ADC10 Clock Divider Select: 7
            DIV_7 = 0b111,
        }
        /// ADC10 Invert Sample Hold Signal
        ISSH: 8 = struct ISSH(bool);
        /// ADC10 Sample/Hold Pulse Mode
        SHP: 9 = struct SHP(bool);
        /// ADC10 Sample/Hold Source 0
        SHS: 10..11 = enum SHS {
            /// ADC10 Sample/Hold Source: 0
            SHS_0 = 0b00,
            /// ADC10 Sample/Hold Source: 1
            SHS_1 = 0b01,
            /// ADC10 Sample/Hold Source: 2
            SHS_2 = 0b10,
            /// ADC10 Sample/Hold Source: 3
            SHS_3 = 0b11,
        }
    }
    /// ADC10 Control 2
    rw CTL2 @ 0x04: u16 = 0_0 {
        /// ADC10 Reference Burst
        REFBURST: 0 = struct REFBURST(bool);
        /// ADC10 Sampling Rate
        SR: 2 = struct SR(bool);
        /// ADC10 Data Format
        DF: 3 = struct DF(bool);
        /// ADC10 Resolution Bit
        RES: 4 = struct RES(bool);
        /// ADC10 predivider Bit: 0
        PDIV: 8..9 = enum PDIV {
            /// ADC10 predivider /1
            PDIV_0 = 0b00,
            /// ADC10 predivider /2
            PDIV_1 = 0b01,
            /// ADC10 predivider /64
            PDIV_2 = 0b10,
            /// ADC10 predivider reserved
            PDIV_3 = 0b11,
        }
    }
    /// ADC10 Window Comparator High Threshold
    rw LO @ 0x06: u16 = 0_0 {
        /// ADC10 Window Comparator High Threshold
        LO: 0..15 = struct LOField(u16);
    }
    /// ADC10 Window Comparator High Threshold
    rw HI @ 0x08: u16 = 0_0 {
        /// ADC10 Window Comparator High Threshold
        HI: 0..15 = struct HIField(u16);
    }
    /// ADC10 Memory Control 0
    rw MCTL0 @ 0x0a: u16 = 0_0 {
        /// ADC10 Input Channel Select Bit 0
        INCH: 0..3 = enum INCH {
            /// ADC10 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC10 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC10 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC10 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC10 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC10 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC10 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC10 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC10 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC10 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC10 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC10 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC10 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC10 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC10 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC10 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC10 Select Reference Bit 0
        SREF: 4..6 = enum SREF {
            /// ADC10 Select Reference 0
            SREF_0 = 0b000,
            /// ADC10 Select Reference 1
            SREF_1 = 0b001,
            /// ADC10 Select Reference 2
            SREF_2 = 0b010,
            /// ADC10 Select Reference 3
            SREF_3 = 0b011,
            /// ADC10 Select Reference 4
            SREF_4 = 0b100,
            /// ADC10 Select Reference 5
            SREF_5 = 0b101,
            /// ADC10 Select Reference 6
            SREF_6 = 0b110,
            /// ADC10 Select Reference 7
            SREF_7 = 0b111,
        }
    }
    /// ADC10 Conversion Memory 0
    rw MEM0 @ 0x12: u16 = 0_0 {
        /// ADC10 Conversion Memory 0
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// ADC10 Interrupt Enable
    rw IE @ 0x1a: u16 = 0_0 {
        /// ADC10_A Interrupt enable
        IE0: 0 = struct IE0(bool);
        /// ADC10_A Interrupt enable for the inside of window of the Window comparator
        INIE: 1 = struct INIE(bool);
        /// ADC10_A Interrupt enable for lower threshold of the Window comparator
        LOIE: 2 = struct LOIE(bool);
        /// ADC10_A Interrupt enable for upper threshold of the Window comparator
        HIIE: 3 = struct HIIE(bool);
        /// ADC10_A ADC10MEM overflow Interrupt enable
        OVIE: 4 = struct OVIE(bool);
        /// ADC10_A conversion-time-overflow Interrupt enable
        TOVIE: 5 = struct TOVIE(bool);
    }
    /// ADC10 Interrupt Flag
    rw IFG @ 0x1c: u16 = 0_0 {
        /// ADC10_A Interrupt Flag
        IFG0: 0 = struct IFG0(bool);
        /// ADC10_A Interrupt Flag for the inside of window of the Window comparator
        INIFG: 1 = struct INIFG(bool);
        /// ADC10_A Interrupt Flag for lower threshold of the Window comparator
        LOIFG: 2 = struct LOIFG(bool);
        /// ADC10_A Interrupt Flag for upper threshold of the Window comparator
        HIIFG: 3 = struct HIIFG(bool);
        /// ADC10_A ADC10MEM overflow Interrupt Flag
        OVIFG: 4 = struct OVIFG(bool);
        /// ADC10_A conversion-time-overflow Interrupt Flag
        TOVIFG: 5 = struct TOVIFG(bool);
    }
    /// ADC10 Interrupt Vector Word
    rw IV @ 0x1e: u16 = 0_0 {
        /// ADC10 Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
}
