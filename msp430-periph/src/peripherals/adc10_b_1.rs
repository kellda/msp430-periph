//! ADC10_B

utils::periph! {
    /// ADC10_B
    ADC10_B;
    /// ADC10 Control 0
    rw ADC10CTL0 @ 0x00: u16 = 0_0 {
        /// ADC10 Start Conversion
        ADC10SC: 0 = struct ADC10SC(bool);
        /// ADC10 Enable Conversion
        ADC10ENC: 1 = struct ADC10ENC(bool);
        /// ADC10 On/enable
        ADC10ON: 4 = struct ADC10ON(bool);
        /// ADC10 Multiple SampleConversion
        ADC10MSC: 7 = struct ADC10MSC(bool);
        /// ADC10 Sample Hold Select Bit: 0
        ADC10SHT: 8..11 = enum ADC10SHT {
            /// ADC10 Sample Hold Select 0
            ADC10SHT_0 = 0b0000,
            /// ADC10 Sample Hold Select 1
            ADC10SHT_1 = 0b0001,
            /// ADC10 Sample Hold Select 2
            ADC10SHT_2 = 0b0010,
            /// ADC10 Sample Hold Select 3
            ADC10SHT_3 = 0b0011,
            /// ADC10 Sample Hold Select 4
            ADC10SHT_4 = 0b0100,
            /// ADC10 Sample Hold Select 5
            ADC10SHT_5 = 0b0101,
            /// ADC10 Sample Hold Select 6
            ADC10SHT_6 = 0b0110,
            /// ADC10 Sample Hold Select 7
            ADC10SHT_7 = 0b0111,
            /// ADC10 Sample Hold Select 8
            ADC10SHT_8 = 0b1000,
            /// ADC10 Sample Hold Select 9
            ADC10SHT_9 = 0b1001,
            /// ADC10 Sample Hold Select 10
            ADC10SHT_10 = 0b1010,
            /// ADC10 Sample Hold Select 11
            ADC10SHT_11 = 0b1011,
            /// ADC10 Sample Hold Select 12
            ADC10SHT_12 = 0b1100,
            /// ADC10 Sample Hold Select 13
            ADC10SHT_13 = 0b1101,
            /// ADC10 Sample Hold Select 14
            ADC10SHT_14 = 0b1110,
            /// ADC10 Sample Hold Select 15
            ADC10SHT_15 = 0b1111,
        }
    }
    /// ADC10 Control 1
    rw ADC10CTL1 @ 0x02: u16 = 0_0 {
        /// ADC10 Busy
        ADC10BUSY: 0 = struct ADC10BUSY(bool);
        /// ADC10 Conversion Sequence Select 0
        ADC10CONSEQ: 1..2 = enum ADC10CONSEQ {
            /// ADC10 Conversion Sequence Select: 0
            ADC10CONSEQ_0 = 0b00,
            /// ADC10 Conversion Sequence Select: 1
            ADC10CONSEQ_1 = 0b01,
            /// ADC10 Conversion Sequence Select: 2
            ADC10CONSEQ_2 = 0b10,
            /// ADC10 Conversion Sequence Select: 3
            ADC10CONSEQ_3 = 0b11,
        }
        /// ADC10 Clock Source Select 0
        ADC10SSEL: 3..4 = enum ADC10SSEL {
            /// ADC10 Clock Source Select: 0
            ADC10SSEL_0 = 0b00,
            /// ADC10 Clock Source Select: 1
            ADC10SSEL_1 = 0b01,
            /// ADC10 Clock Source Select: 2
            ADC10SSEL_2 = 0b10,
            /// ADC10 Clock Source Select: 3
            ADC10SSEL_3 = 0b11,
        }
        /// ADC10 Clock Divider Select 0
        ADC10DIV: 5..7 = enum ADC10DIV {
            /// ADC10 Clock Divider Select: 0
            ADC10DIV_0 = 0b000,
            /// ADC10 Clock Divider Select: 1
            ADC10DIV_1 = 0b001,
            /// ADC10 Clock Divider Select: 2
            ADC10DIV_2 = 0b010,
            /// ADC10 Clock Divider Select: 3
            ADC10DIV_3 = 0b011,
            /// ADC10 Clock Divider Select: 4
            ADC10DIV_4 = 0b100,
            /// ADC10 Clock Divider Select: 5
            ADC10DIV_5 = 0b101,
            /// ADC10 Clock Divider Select: 6
            ADC10DIV_6 = 0b110,
            /// ADC10 Clock Divider Select: 7
            ADC10DIV_7 = 0b111,
        }
        /// ADC10 Invert Sample Hold Signal
        ADC10ISSH: 8 = struct ADC10ISSH(bool);
        /// ADC10 Sample/Hold Pulse Mode
        ADC10SHP: 9 = struct ADC10SHP(bool);
        /// ADC10 Sample/Hold Source 0
        ADC10SHS: 10..11 = enum ADC10SHS {
            /// ADC10 Sample/Hold Source: 0
            ADC10SHS_0 = 0b00,
            /// ADC10 Sample/Hold Source: 1
            ADC10SHS_1 = 0b01,
            /// ADC10 Sample/Hold Source: 2
            ADC10SHS_2 = 0b10,
            /// ADC10 Sample/Hold Source: 3
            ADC10SHS_3 = 0b11,
        }
    }
    /// ADC10 Control 2
    rw ADC10CTL2 @ 0x04: u16 = 0_0 {
        /// ADC10 Reference Burst
        ADC10REFBURST: 0 = struct ADC10REFBURST(bool);
        /// ADC10 Sampling Rate
        ADC10SR: 2 = struct ADC10SR(bool);
        /// ADC10 Data Format
        ADC10DF: 3 = struct ADC10DF(bool);
        /// ADC10 Resolution Bit
        ADC10RES: 4 = struct ADC10RES(bool);
        /// ADC10 predivider Bit: 0
        ADC10PDIV: 8..9 = enum ADC10PDIV {
            /// ADC10 predivider /1
            ADC10PDIV_0 = 0b00,
            /// ADC10 predivider /2
            ADC10PDIV_1 = 0b01,
            /// ADC10 predivider /64
            ADC10PDIV_2 = 0b10,
            /// ADC10 predivider reserved
            ADC10PDIV_3 = 0b11,
        }
    }
    /// ADC10 Window Comparator High Threshold
    rw ADC10LO @ 0x06: u16 = 0_0 {
        /// ADC10 Window Comparator High Threshold
        ADC10LO: 0..15 = struct ADC10LOField(u16);
    }
    /// ADC10 Window Comparator High Threshold
    rw ADC10HI @ 0x08: u16 = 0_0 {
        /// ADC10 Window Comparator High Threshold
        ADC10HI: 0..15 = struct ADC10HIField(u16);
    }
    /// ADC10 Memory Control 0
    rw ADC10MCTL0 @ 0x0a: u16 = 0_0 {
        /// ADC10 Input Channel Select Bit 0
        ADC10INCH: 0..3 = enum ADC10INCH {
            /// ADC10 Input Channel 0
            ADC10INCH_0 = 0b0000,
            /// ADC10 Input Channel 1
            ADC10INCH_1 = 0b0001,
            /// ADC10 Input Channel 2
            ADC10INCH_2 = 0b0010,
            /// ADC10 Input Channel 3
            ADC10INCH_3 = 0b0011,
            /// ADC10 Input Channel 4
            ADC10INCH_4 = 0b0100,
            /// ADC10 Input Channel 5
            ADC10INCH_5 = 0b0101,
            /// ADC10 Input Channel 6
            ADC10INCH_6 = 0b0110,
            /// ADC10 Input Channel 7
            ADC10INCH_7 = 0b0111,
            /// ADC10 Input Channel 8
            ADC10INCH_8 = 0b1000,
            /// ADC10 Input Channel 9
            ADC10INCH_9 = 0b1001,
            /// ADC10 Input Channel 10
            ADC10INCH_10 = 0b1010,
            /// ADC10 Input Channel 11
            ADC10INCH_11 = 0b1011,
            /// ADC10 Input Channel 12
            ADC10INCH_12 = 0b1100,
            /// ADC10 Input Channel 13
            ADC10INCH_13 = 0b1101,
            /// ADC10 Input Channel 14
            ADC10INCH_14 = 0b1110,
            /// ADC10 Input Channel 15
            ADC10INCH_15 = 0b1111,
        }
        /// ADC10 Select Reference Bit 0
        ADC10SREF: 4..6 = enum ADC10SREF {
            /// ADC10 Select Reference 0
            ADC10SREF_0 = 0b000,
            /// ADC10 Select Reference 1
            ADC10SREF_1 = 0b001,
            /// ADC10 Select Reference 2
            ADC10SREF_2 = 0b010,
            /// ADC10 Select Reference 3
            ADC10SREF_3 = 0b011,
            /// ADC10 Select Reference 4
            ADC10SREF_4 = 0b100,
            /// ADC10 Select Reference 5
            ADC10SREF_5 = 0b101,
            /// ADC10 Select Reference 6
            ADC10SREF_6 = 0b110,
            /// ADC10 Select Reference 7
            ADC10SREF_7 = 0b111,
        }
    }
    /// ADC10 Conversion Memory 0
    rw ADC10MEM0 @ 0x12: u16 = 0_0 {
        /// ADC10 Conversion Memory 0
        ADC10MEM0: 0..15 = struct ADC10MEM0Field(u16);
    }
    /// ADC10 Interrupt Enable
    rw ADC10IE @ 0x1a: u16 = 0_0 {
        /// ADC10_A Interrupt enable
        ADC10IE0: 0 = struct ADC10IE0(bool);
        /// ADC10_A Interrupt enable for the inside of window of the Window comparator
        ADC10INIE: 1 = struct ADC10INIE(bool);
        /// ADC10_A Interrupt enable for lower threshold of the Window comparator
        ADC10LOIE: 2 = struct ADC10LOIE(bool);
        /// ADC10_A Interrupt enable for upper threshold of the Window comparator
        ADC10HIIE: 3 = struct ADC10HIIE(bool);
        /// ADC10_A ADC10MEM overflow Interrupt enable
        ADC10OVIE: 4 = struct ADC10OVIE(bool);
        /// ADC10_A conversion-time-overflow Interrupt enable
        ADC10TOVIE: 5 = struct ADC10TOVIE(bool);
    }
    /// ADC10 Interrupt Flag
    rw ADC10IFG @ 0x1c: u16 = 0_0 {
        /// ADC10_A Interrupt Flag
        ADC10IFG0: 0 = struct ADC10IFG0(bool);
        /// ADC10_A Interrupt Flag for the inside of window of the Window comparator
        ADC10INIFG: 1 = struct ADC10INIFG(bool);
        /// ADC10_A Interrupt Flag for lower threshold of the Window comparator
        ADC10LOIFG: 2 = struct ADC10LOIFG(bool);
        /// ADC10_A Interrupt Flag for upper threshold of the Window comparator
        ADC10HIIFG: 3 = struct ADC10HIIFG(bool);
        /// ADC10_A ADC10MEM overflow Interrupt Flag
        ADC10OVIFG: 4 = struct ADC10OVIFG(bool);
        /// ADC10_A conversion-time-overflow Interrupt Flag
        ADC10TOVIFG: 5 = struct ADC10TOVIFG(bool);
    }
    /// ADC10 Interrupt Vector Word
    rw ADC10IV @ 0x1e: u16 = 0_0 {
        /// ADC10 Interrupt Vector Word
        ADC10IV: 0..15 = struct ADC10IVField(u16);
    }
}
