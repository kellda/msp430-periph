//! ADC12

utils::periph! {
    /// ADC12
    ADC12;
    /// ADC12 Control 0
    rw ADC12CTL0 @ 0x120: u16 = 0_0 {
        /// ADC12 Start Conversion
        ADC12SC: 0 = struct ADC12SC(bool);
        /// ADC12 Enable Conversion
        ENC: 1 = struct ENC(bool);
        /// ADC12 Timer Overflow interrupt enable
        ADC12TOVIE: 2 = struct ADC12TOVIE(bool);
        /// ADC12 Overflow interrupt enable
        ADC12OVIE: 3 = struct ADC12OVIE(bool);
        /// ADC12 On/enable
        ADC12ON: 4 = struct ADC12ON(bool);
        /// ADC12 Reference on
        REFON: 5 = struct REFON(bool);
        /// ADC12 Ref 0:1.5V / 1:2.5V
        REF2_5V: 6 = struct REF2_5V(bool);
        /// ADC12 Multiple SampleConversion
        MSC: 7 = struct MSC(bool);
        /// ADC12 Sample Hold 0 Select 0
        SHT0: 8..11 = enum SHT0 {
            /// ADC12 Sample Hold 0 Select Bit: 0
            SHT0_0 = 0b0000,
            /// ADC12 Sample Hold 0 Select Bit: 1
            SHT0_1 = 0b0001,
            /// ADC12 Sample Hold 0 Select Bit: 2
            SHT0_2 = 0b0010,
            /// ADC12 Sample Hold 0 Select Bit: 3
            SHT0_3 = 0b0011,
            /// ADC12 Sample Hold 0 Select Bit: 4
            SHT0_4 = 0b0100,
            /// ADC12 Sample Hold 0 Select Bit: 5
            SHT0_5 = 0b0101,
            /// ADC12 Sample Hold 0 Select Bit: 6
            SHT0_6 = 0b0110,
            /// ADC12 Sample Hold 0 Select Bit: 7
            SHT0_7 = 0b0111,
            /// ADC12 Sample Hold 0 Select Bit: 8
            SHT0_8 = 0b1000,
            /// ADC12 Sample Hold 0 Select Bit: 9
            SHT0_9 = 0b1001,
            /// ADC12 Sample Hold 0 Select Bit: 10
            SHT0_10 = 0b1010,
            /// ADC12 Sample Hold 0 Select Bit: 11
            SHT0_11 = 0b1011,
            /// ADC12 Sample Hold 0 Select Bit: 12
            SHT0_12 = 0b1100,
            /// ADC12 Sample Hold 0 Select Bit: 13
            SHT0_13 = 0b1101,
            /// ADC12 Sample Hold 0 Select Bit: 14
            SHT0_14 = 0b1110,
            /// ADC12 Sample Hold 0 Select Bit: 15
            SHT0_15 = 0b1111,
        }
        /// ADC12 Sample Hold 0 Select 0
        SHT1: 12..15 = enum SHT1 {
            /// ADC12 Sample Hold 1 Select Bit: 0
            SHT1_0 = 0b0000,
            /// ADC12 Sample Hold 1 Select Bit: 1
            SHT1_1 = 0b0001,
            /// ADC12 Sample Hold 1 Select Bit: 2
            SHT1_2 = 0b0010,
            /// ADC12 Sample Hold 1 Select Bit: 3
            SHT1_3 = 0b0011,
            /// ADC12 Sample Hold 1 Select Bit: 4
            SHT1_4 = 0b0100,
            /// ADC12 Sample Hold 1 Select Bit: 5
            SHT1_5 = 0b0101,
            /// ADC12 Sample Hold 1 Select Bit: 6
            SHT1_6 = 0b0110,
            /// ADC12 Sample Hold 1 Select Bit: 7
            SHT1_7 = 0b0111,
            /// ADC12 Sample Hold 1 Select Bit: 8
            SHT1_8 = 0b1000,
            /// ADC12 Sample Hold 1 Select Bit: 9
            SHT1_9 = 0b1001,
            /// ADC12 Sample Hold 1 Select Bit: 10
            SHT1_10 = 0b1010,
            /// ADC12 Sample Hold 1 Select Bit: 11
            SHT1_11 = 0b1011,
            /// ADC12 Sample Hold 1 Select Bit: 12
            SHT1_12 = 0b1100,
            /// ADC12 Sample Hold 1 Select Bit: 13
            SHT1_13 = 0b1101,
            /// ADC12 Sample Hold 1 Select Bit: 14
            SHT1_14 = 0b1110,
            /// ADC12 Sample Hold 1 Select Bit: 15
            SHT1_15 = 0b1111,
        }
    }
    /// ADC12 Control 1
    rw ADC12CTL1 @ 0x122: u16 = 0_0 {
        /// ADC12 Busy
        ADC12BUSY: 0 = struct ADC12BUSY(bool);
        /// ADC12 Conversion Sequence Select 0
        CONSEQ: 1..2 = enum CONSEQ {
            /// ADC12 Conversion Sequence Select: 0
            CONSEQ_0 = 0b00,
            /// ADC12 Conversion Sequence Select: 1
            CONSEQ_1 = 0b01,
            /// ADC12 Conversion Sequence Select: 2
            CONSEQ_2 = 0b10,
            /// ADC12 Conversion Sequence Select: 3
            CONSEQ_3 = 0b11,
        }
        /// ADC12 Clock Source Select 0
        ADC12SSEL: 3..4 = enum ADC12SSEL {
            /// ADC12 Clock Source Select: 0
            ADC12SSEL_0 = 0b00,
            /// ADC12 Clock Source Select: 1
            ADC12SSEL_1 = 0b01,
            /// ADC12 Clock Source Select: 2
            ADC12SSEL_2 = 0b10,
            /// ADC12 Clock Source Select: 3
            ADC12SSEL_3 = 0b11,
        }
        /// ADC12 Clock Divider Select 0
        ADC12DIV: 5..7 = enum ADC12DIV {
            /// ADC12 Clock Divider Select: 0
            ADC12DIV_0 = 0b000,
            /// ADC12 Clock Divider Select: 1
            ADC12DIV_1 = 0b001,
            /// ADC12 Clock Divider Select: 2
            ADC12DIV_2 = 0b010,
            /// ADC12 Clock Divider Select: 3
            ADC12DIV_3 = 0b011,
            /// ADC12 Clock Divider Select: 4
            ADC12DIV_4 = 0b100,
            /// ADC12 Clock Divider Select: 5
            ADC12DIV_5 = 0b101,
            /// ADC12 Clock Divider Select: 6
            ADC12DIV_6 = 0b110,
            /// ADC12 Clock Divider Select: 7
            ADC12DIV_7 = 0b111,
        }
        /// ADC12 Invert Sample Hold Signal
        ISSH: 8 = struct ISSH(bool);
        /// ADC12 Sample/Hold Pulse Mode
        SHP: 9 = struct SHP(bool);
        /// ADC12 Sample/Hold Source 0
        SHS: 10..11 = enum SHS {
            /// ADC12 Sample/Hold Source: 0
            SHS_0 = 0b00,
            /// ADC12 Sample/Hold Source: 1
            SHS_1 = 0b01,
            /// ADC12 Sample/Hold Source: 2
            SHS_2 = 0b10,
            /// ADC12 Sample/Hold Source: 3
            SHS_3 = 0b11,
        }
        /// ADC12 Conversion Start Address 0
        CSTARTADD: 12..15 = enum CSTARTADD {
            /// ADC12 Conversion Start Address: 0
            CSTARTADD_0 = 0b0000,
            /// ADC12 Conversion Start Address: 1
            CSTARTADD_1 = 0b0001,
            /// ADC12 Conversion Start Address: 2
            CSTARTADD_2 = 0b0010,
            /// ADC12 Conversion Start Address: 3
            CSTARTADD_3 = 0b0011,
            /// ADC12 Conversion Start Address: 4
            CSTARTADD_4 = 0b0100,
            /// ADC12 Conversion Start Address: 5
            CSTARTADD_5 = 0b0101,
            /// ADC12 Conversion Start Address: 6
            CSTARTADD_6 = 0b0110,
            /// ADC12 Conversion Start Address: 7
            CSTARTADD_7 = 0b0111,
            /// ADC12 Conversion Start Address: 8
            CSTARTADD_8 = 0b1000,
            /// ADC12 Conversion Start Address: 9
            CSTARTADD_9 = 0b1001,
            /// ADC12 Conversion Start Address: 10
            CSTARTADD_10 = 0b1010,
            /// ADC12 Conversion Start Address: 11
            CSTARTADD_11 = 0b1011,
            /// ADC12 Conversion Start Address: 12
            CSTARTADD_12 = 0b1100,
            /// ADC12 Conversion Start Address: 13
            CSTARTADD_13 = 0b1101,
            /// ADC12 Conversion Start Address: 14
            CSTARTADD_14 = 0b1110,
            /// ADC12 Conversion Start Address: 15
            CSTARTADD_15 = 0b1111,
        }
    }
    /// ADC12 Interrupt Flag
    rw ADC12IFG @ 0x124: u16 = 0_0 {
        /// ADC12 Interrupt Flag
        ADC12IFG: 0..15 = struct ADC12IFGField(u16);
    }
    /// ADC12 Interrupt Enable
    rw ADC12IE @ 0x126: u16 = 0_0 {
        /// ADC12 Interrupt Enable
        ADC12IE: 0..15 = struct ADC12IEField(u16);
    }
    /// ADC12 Interrupt Vector Word
    rw ADC12IV @ 0x128: u16 = 0_0 {
        /// ADC12 Interrupt Vector Word
        ADC12IV: 0..15 = struct ADC12IVField(u16);
    }
    /// ADC12 Conversion Memory 0
    rw ADC12MEM0 @ 0xc0: u16 = 0_0 {
        /// ADC12 Conversion Memory 0
        ADC12MEM0: 0..15 = struct ADC12MEM0Field(u16);
    }
    /// ADC12 Conversion Memory 1
    rw ADC12MEM1 @ 0xc2: u16 = 0_0 {
        /// ADC12 Conversion Memory 1
        ADC12MEM1: 0..15 = struct ADC12MEM1Field(u16);
    }
    /// ADC12 Conversion Memory 2
    rw ADC12MEM2 @ 0xc4: u16 = 0_0 {
        /// ADC12 Conversion Memory 2
        ADC12MEM2: 0..15 = struct ADC12MEM2Field(u16);
    }
    /// ADC12 Conversion Memory 3
    rw ADC12MEM3 @ 0xc6: u16 = 0_0 {
        /// ADC12 Conversion Memory 3
        ADC12MEM3: 0..15 = struct ADC12MEM3Field(u16);
    }
    /// ADC12 Conversion Memory 4
    rw ADC12MEM4 @ 0xc8: u16 = 0_0 {
        /// ADC12 Conversion Memory 4
        ADC12MEM4: 0..15 = struct ADC12MEM4Field(u16);
    }
    /// ADC12 Conversion Memory 5
    rw ADC12MEM5 @ 0xca: u16 = 0_0 {
        /// ADC12 Conversion Memory 5
        ADC12MEM5: 0..15 = struct ADC12MEM5Field(u16);
    }
    /// ADC12 Conversion Memory 6
    rw ADC12MEM6 @ 0xcc: u16 = 0_0 {
        /// ADC12 Conversion Memory 6
        ADC12MEM6: 0..15 = struct ADC12MEM6Field(u16);
    }
    /// ADC12 Conversion Memory 7
    rw ADC12MEM7 @ 0xce: u16 = 0_0 {
        /// ADC12 Conversion Memory 7
        ADC12MEM7: 0..15 = struct ADC12MEM7Field(u16);
    }
    /// ADC12 Conversion Memory 8
    rw ADC12MEM8 @ 0xd0: u16 = 0_0 {
        /// ADC12 Conversion Memory 8
        ADC12MEM8: 0..15 = struct ADC12MEM8Field(u16);
    }
    /// ADC12 Conversion Memory 9
    rw ADC12MEM9 @ 0xd2: u16 = 0_0 {
        /// ADC12 Conversion Memory 9
        ADC12MEM9: 0..15 = struct ADC12MEM9Field(u16);
    }
    /// ADC12 Conversion Memory 10
    rw ADC12MEM10 @ 0xd4: u16 = 0_0 {
        /// ADC12 Conversion Memory 10
        ADC12MEM10: 0..15 = struct ADC12MEM10Field(u16);
    }
    /// ADC12 Conversion Memory 11
    rw ADC12MEM11 @ 0xd6: u16 = 0_0 {
        /// ADC12 Conversion Memory 11
        ADC12MEM11: 0..15 = struct ADC12MEM11Field(u16);
    }
    /// ADC12 Conversion Memory 12
    rw ADC12MEM12 @ 0xd8: u16 = 0_0 {
        /// ADC12 Conversion Memory 12
        ADC12MEM12: 0..15 = struct ADC12MEM12Field(u16);
    }
    /// ADC12 Conversion Memory 13
    rw ADC12MEM13 @ 0xda: u16 = 0_0 {
        /// ADC12 Conversion Memory 13
        ADC12MEM13: 0..15 = struct ADC12MEM13Field(u16);
    }
    /// ADC12 Conversion Memory 14
    rw ADC12MEM14 @ 0xdc: u16 = 0_0 {
        /// ADC12 Conversion Memory 14
        ADC12MEM14: 0..15 = struct ADC12MEM14Field(u16);
    }
    /// ADC12 Conversion Memory 15
    rw ADC12MEM15 @ 0xde: u16 = 0_0 {
        /// ADC12 Conversion Memory 15
        ADC12MEM15: 0..15 = struct ADC12MEM15Field(u16);
    }
    /// ADC12 Memory Control 0
    rw ADC12MCTL0 @ 0x00: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL0_INCH: 0..3 = enum ADC12MCTL0_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL0_SREF: 4..6 = enum ADC12MCTL0_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL0_EOS: 7 = struct ADC12MCTL0_EOS(bool);
    }
    /// ADC12 Memory Control 1
    rw ADC12MCTL1 @ 0x01: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL1_INCH: 0..3 = enum ADC12MCTL1_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL1_SREF: 4..6 = enum ADC12MCTL1_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL1_EOS: 7 = struct ADC12MCTL1_EOS(bool);
    }
    /// ADC12 Memory Control 2
    rw ADC12MCTL2 @ 0x02: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL2_INCH: 0..3 = enum ADC12MCTL2_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL2_SREF: 4..6 = enum ADC12MCTL2_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL2_EOS: 7 = struct ADC12MCTL2_EOS(bool);
    }
    /// ADC12 Memory Control 3
    rw ADC12MCTL3 @ 0x03: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL3_INCH: 0..3 = enum ADC12MCTL3_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL3_SREF: 4..6 = enum ADC12MCTL3_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL3_EOS: 7 = struct ADC12MCTL3_EOS(bool);
    }
    /// ADC12 Memory Control 4
    rw ADC12MCTL4 @ 0x04: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL4_INCH: 0..3 = enum ADC12MCTL4_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL4_SREF: 4..6 = enum ADC12MCTL4_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL4_EOS: 7 = struct ADC12MCTL4_EOS(bool);
    }
    /// ADC12 Memory Control 5
    rw ADC12MCTL5 @ 0x05: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL5_INCH: 0..3 = enum ADC12MCTL5_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL5_SREF: 4..6 = enum ADC12MCTL5_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL5_EOS: 7 = struct ADC12MCTL5_EOS(bool);
    }
    /// ADC12 Memory Control 6
    rw ADC12MCTL6 @ 0x06: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL6_INCH: 0..3 = enum ADC12MCTL6_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL6_SREF: 4..6 = enum ADC12MCTL6_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL6_EOS: 7 = struct ADC12MCTL6_EOS(bool);
    }
    /// ADC12 Memory Control 7
    rw ADC12MCTL7 @ 0x07: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL7_INCH: 0..3 = enum ADC12MCTL7_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL7_SREF: 4..6 = enum ADC12MCTL7_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL7_EOS: 7 = struct ADC12MCTL7_EOS(bool);
    }
    /// ADC12 Memory Control 8
    rw ADC12MCTL8 @ 0x08: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL8_INCH: 0..3 = enum ADC12MCTL8_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL8_SREF: 4..6 = enum ADC12MCTL8_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL8_EOS: 7 = struct ADC12MCTL8_EOS(bool);
    }
    /// ADC12 Memory Control 9
    rw ADC12MCTL9 @ 0x09: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL9_INCH: 0..3 = enum ADC12MCTL9_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL9_SREF: 4..6 = enum ADC12MCTL9_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL9_EOS: 7 = struct ADC12MCTL9_EOS(bool);
    }
    /// ADC12 Memory Control 10
    rw ADC12MCTL10 @ 0x0a: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL10_INCH: 0..3 = enum ADC12MCTL10_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL10_SREF: 4..6 = enum ADC12MCTL10_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL10_EOS: 7 = struct ADC12MCTL10_EOS(bool);
    }
    /// ADC12 Memory Control 11
    rw ADC12MCTL11 @ 0x0b: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL11_INCH: 0..3 = enum ADC12MCTL11_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL11_SREF: 4..6 = enum ADC12MCTL11_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL11_EOS: 7 = struct ADC12MCTL11_EOS(bool);
    }
    /// ADC12 Memory Control 12
    rw ADC12MCTL12 @ 0x0c: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL12_INCH: 0..3 = enum ADC12MCTL12_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL12_SREF: 4..6 = enum ADC12MCTL12_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL12_EOS: 7 = struct ADC12MCTL12_EOS(bool);
    }
    /// ADC12 Memory Control 13
    rw ADC12MCTL13 @ 0x0d: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL13_INCH: 0..3 = enum ADC12MCTL13_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL13_SREF: 4..6 = enum ADC12MCTL13_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL13_EOS: 7 = struct ADC12MCTL13_EOS(bool);
    }
    /// ADC12 Memory Control 14
    rw ADC12MCTL14 @ 0x0e: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL14_INCH: 0..3 = enum ADC12MCTL14_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL14_SREF: 4..6 = enum ADC12MCTL14_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL14_EOS: 7 = struct ADC12MCTL14_EOS(bool);
    }
    /// ADC12 Memory Control 15
    rw ADC12MCTL15 @ 0x0f: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL15_INCH: 0..3 = enum ADC12MCTL15_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL15_SREF: 4..6 = enum ADC12MCTL15_SREF {
            /// ADC12 Select Reference 0
            SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL15_EOS: 7 = struct ADC12MCTL15_EOS(bool);
    }
}
