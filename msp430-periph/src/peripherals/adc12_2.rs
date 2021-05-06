//! ADC12

utils::periph! {
    /// ADC12
    ADC12;
    /// ADC12+ Control 0
    rw ADC12CTL0 @ 0x00: u16 = 0_0 {
        /// ADC12 Start Conversion
        ADC12SC: 0 = struct ADC12SC(bool);
        /// ADC12 Enable Conversion
        ADC12ENC: 1 = struct ADC12ENC(bool);
        /// ADC12 Timer Overflow interrupt enable
        ADC12TOVIE: 2 = struct ADC12TOVIE(bool);
        /// ADC12 Overflow interrupt enable
        ADC12OVIE: 3 = struct ADC12OVIE(bool);
        /// ADC12 On/enable
        ADC12ON: 4 = struct ADC12ON(bool);
        /// ADC12 Reference on
        ADC12REFON: 5 = struct ADC12REFON(bool);
        /// ADC12 Ref 0:1.5V / 1:2.5V
        ADC12REF2_5V: 6 = struct ADC12REF2_5V(bool);
        /// ADC12 Multiple SampleConversion
        ADC12MSC: 7 = struct ADC12MSC(bool);
        /// ADC12 Sample Hold 0 Select Bit: 0
        ADC12SHT0: 8..11 = enum ADC12SHT0 {
            /// ADC12 Sample Hold 0 Select Bit: 0
            ADC12SHT0_0 = 0b0000,
            /// ADC12 Sample Hold 0 Select Bit: 1
            ADC12SHT0_1 = 0b0001,
            /// ADC12 Sample Hold 0 Select Bit: 2
            ADC12SHT0_2 = 0b0010,
            /// ADC12 Sample Hold 0 Select Bit: 3
            ADC12SHT0_3 = 0b0011,
            /// ADC12 Sample Hold 0 Select Bit: 4
            ADC12SHT0_4 = 0b0100,
            /// ADC12 Sample Hold 0 Select Bit: 5
            ADC12SHT0_5 = 0b0101,
            /// ADC12 Sample Hold 0 Select Bit: 6
            ADC12SHT0_6 = 0b0110,
            /// ADC12 Sample Hold 0 Select Bit: 7
            ADC12SHT0_7 = 0b0111,
            /// ADC12 Sample Hold 0 Select Bit: 8
            ADC12SHT0_8 = 0b1000,
            /// ADC12 Sample Hold 0 Select Bit: 9
            ADC12SHT0_9 = 0b1001,
            /// ADC12 Sample Hold 0 Select Bit: 10
            ADC12SHT0_10 = 0b1010,
            /// ADC12 Sample Hold 0 Select Bit: 11
            ADC12SHT0_11 = 0b1011,
            /// ADC12 Sample Hold 0 Select Bit: 12
            ADC12SHT0_12 = 0b1100,
            /// ADC12 Sample Hold 0 Select Bit: 13
            ADC12SHT0_13 = 0b1101,
            /// ADC12 Sample Hold 0 Select Bit: 14
            ADC12SHT0_14 = 0b1110,
            /// ADC12 Sample Hold 0 Select Bit: 15
            ADC12SHT0_15 = 0b1111,
        }
        /// ADC12 Sample Hold 1 Select Bit: 0
        ADC12SHT1: 12..15 = enum ADC12SHT1 {
            /// ADC12 Sample Hold 1 Select Bit: 0
            ADC12SHT1_0 = 0b0000,
            /// ADC12 Sample Hold 1 Select Bit: 1
            ADC12SHT1_1 = 0b0001,
            /// ADC12 Sample Hold 1 Select Bit: 2
            ADC12SHT1_2 = 0b0010,
            /// ADC12 Sample Hold 1 Select Bit: 3
            ADC12SHT1_3 = 0b0011,
            /// ADC12 Sample Hold 1 Select Bit: 4
            ADC12SHT1_4 = 0b0100,
            /// ADC12 Sample Hold 1 Select Bit: 5
            ADC12SHT1_5 = 0b0101,
            /// ADC12 Sample Hold 1 Select Bit: 6
            ADC12SHT1_6 = 0b0110,
            /// ADC12 Sample Hold 1 Select Bit: 7
            ADC12SHT1_7 = 0b0111,
            /// ADC12 Sample Hold 1 Select Bit: 8
            ADC12SHT1_8 = 0b1000,
            /// ADC12 Sample Hold 1 Select Bit: 9
            ADC12SHT1_9 = 0b1001,
            /// ADC12 Sample Hold 1 Select Bit: 10
            ADC12SHT1_10 = 0b1010,
            /// ADC12 Sample Hold 1 Select Bit: 11
            ADC12SHT1_11 = 0b1011,
            /// ADC12 Sample Hold 1 Select Bit: 12
            ADC12SHT1_12 = 0b1100,
            /// ADC12 Sample Hold 1 Select Bit: 13
            ADC12SHT1_13 = 0b1101,
            /// ADC12 Sample Hold 1 Select Bit: 14
            ADC12SHT1_14 = 0b1110,
            /// ADC12 Sample Hold 1 Select Bit: 15
            ADC12SHT1_15 = 0b1111,
        }
    }
    /// ADC12+ Control 1
    rw ADC12CTL1 @ 0x02: u16 = 0_0 {
        /// ADC12 Busy
        ADC12BUSY: 0 = struct ADC12BUSY(bool);
        /// ADC12 Conversion Sequence Select Bit: 0
        ADC12CONSEQ: 1..2 = enum ADC12CONSEQ {
            /// ADC12 Conversion Sequence Select: 0
            ADC12CONSEQ_0 = 0b00,
            /// ADC12 Conversion Sequence Select: 1
            ADC12CONSEQ_1 = 0b01,
            /// ADC12 Conversion Sequence Select: 2
            ADC12CONSEQ_2 = 0b10,
            /// ADC12 Conversion Sequence Select: 3
            ADC12CONSEQ_3 = 0b11,
        }
        /// ADC12 Clock Source Select Bit: 0
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
        /// ADC12 Clock Divider Select Bit: 0
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
        ADC12ISSH: 8 = struct ADC12ISSH(bool);
        /// ADC12 Sample/Hold Pulse Mode
        ADC12SHP: 9 = struct ADC12SHP(bool);
        /// ADC12 Sample/Hold Source Bit: 0
        ADC12SHS: 10..11 = enum ADC12SHS {
            /// ADC12 Sample/Hold Source: 0
            ADC12SHS_0 = 0b00,
            /// ADC12 Sample/Hold Source: 1
            ADC12SHS_1 = 0b01,
            /// ADC12 Sample/Hold Source: 2
            ADC12SHS_2 = 0b10,
            /// ADC12 Sample/Hold Source: 3
            ADC12SHS_3 = 0b11,
        }
        /// ADC12 Conversion Start Address Bit: 0
        ADC12CSTARTADD: 12..15 = enum ADC12CSTARTADD {
            /// ADC12 Conversion Start Address: 0
            ADC12CSTARTADD_0 = 0b0000,
            /// ADC12 Conversion Start Address: 1
            ADC12CSTARTADD_1 = 0b0001,
            /// ADC12 Conversion Start Address: 2
            ADC12CSTARTADD_2 = 0b0010,
            /// ADC12 Conversion Start Address: 3
            ADC12CSTARTADD_3 = 0b0011,
            /// ADC12 Conversion Start Address: 4
            ADC12CSTARTADD_4 = 0b0100,
            /// ADC12 Conversion Start Address: 5
            ADC12CSTARTADD_5 = 0b0101,
            /// ADC12 Conversion Start Address: 6
            ADC12CSTARTADD_6 = 0b0110,
            /// ADC12 Conversion Start Address: 7
            ADC12CSTARTADD_7 = 0b0111,
            /// ADC12 Conversion Start Address: 8
            ADC12CSTARTADD_8 = 0b1000,
            /// ADC12 Conversion Start Address: 9
            ADC12CSTARTADD_9 = 0b1001,
            /// ADC12 Conversion Start Address: 10
            ADC12CSTARTADD_10 = 0b1010,
            /// ADC12 Conversion Start Address: 11
            ADC12CSTARTADD_11 = 0b1011,
            /// ADC12 Conversion Start Address: 12
            ADC12CSTARTADD_12 = 0b1100,
            /// ADC12 Conversion Start Address: 13
            ADC12CSTARTADD_13 = 0b1101,
            /// ADC12 Conversion Start Address: 14
            ADC12CSTARTADD_14 = 0b1110,
            /// ADC12 Conversion Start Address: 15
            ADC12CSTARTADD_15 = 0b1111,
        }
    }
    /// ADC12+ Control 2
    rw ADC12CTL2 @ 0x04: u16 = 0_0 {
        /// ADC12+ Reference Burst
        ADC12REFBURST: 0 = struct ADC12REFBURST(bool);
        /// ADC12+ Reference Out
        ADC12REFOUT: 1 = struct ADC12REFOUT(bool);
        /// ADC12+ Sampling Rate
        ADC12SR: 2 = struct ADC12SR(bool);
        /// ADC12+ Data Format
        ADC12DF: 3 = struct ADC12DF(bool);
        /// ADC12+ Resolution Bit: 0
        ADC12RES: 4..5 = enum ADC12RES {
            /// ADC12+ Resolution : 8 Bit
            ADC12RES_0 = 0b00,
            /// ADC12+ Resolution : 10 Bit
            ADC12RES_1 = 0b01,
            /// ADC12+ Resolution : 12 Bit
            ADC12RES_2 = 0b10,
            /// ADC12+ Resolution : reserved
            ADC12RES_3 = 0b11,
        }
        /// ADC12+ Temperature Sensor Off
        ADC12TCOFF: 7 = struct ADC12TCOFF(bool);
        /// ADC12+ predivider 0:/1   1:/4
        ADC12PDIV: 8 = struct ADC12PDIV(bool);
    }
    /// ADC12+ Interrupt Flag
    rw ADC12IFG @ 0x0a: u16 = 0_0 {
        /// ADC12 Memory 0 Interrupt Flag
        ADC12IFG0: 0 = struct ADC12IFG0(bool);
        /// ADC12 Memory 1 Interrupt Flag
        ADC12IFG1: 1 = struct ADC12IFG1(bool);
        /// ADC12 Memory 2 Interrupt Flag
        ADC12IFG2: 2 = struct ADC12IFG2(bool);
        /// ADC12 Memory 3 Interrupt Flag
        ADC12IFG3: 3 = struct ADC12IFG3(bool);
        /// ADC12 Memory 4 Interrupt Flag
        ADC12IFG4: 4 = struct ADC12IFG4(bool);
        /// ADC12 Memory 5 Interrupt Flag
        ADC12IFG5: 5 = struct ADC12IFG5(bool);
        /// ADC12 Memory 6 Interrupt Flag
        ADC12IFG6: 6 = struct ADC12IFG6(bool);
        /// ADC12 Memory 7 Interrupt Flag
        ADC12IFG7: 7 = struct ADC12IFG7(bool);
        /// ADC12 Memory 8 Interrupt Flag
        ADC12IFG8: 8 = struct ADC12IFG8(bool);
        /// ADC12 Memory 9 Interrupt Flag
        ADC12IFG9: 9 = struct ADC12IFG9(bool);
        /// ADC12 Memory 10 Interrupt Flag
        ADC12IFG10: 10 = struct ADC12IFG10(bool);
        /// ADC12 Memory 11 Interrupt Flag
        ADC12IFG11: 11 = struct ADC12IFG11(bool);
        /// ADC12 Memory 12 Interrupt Flag
        ADC12IFG12: 12 = struct ADC12IFG12(bool);
        /// ADC12 Memory 13 Interrupt Flag
        ADC12IFG13: 13 = struct ADC12IFG13(bool);
        /// ADC12 Memory 14 Interrupt Flag
        ADC12IFG14: 14 = struct ADC12IFG14(bool);
        /// ADC12 Memory 15 Interrupt Flag
        ADC12IFG15: 15 = struct ADC12IFG15(bool);
    }
    /// ADC12+ Interrupt Enable
    rw ADC12IE @ 0x0c: u16 = 0_0 {
        /// ADC12 Memory 0 Interrupt Enable
        ADC12IE0: 0 = struct ADC12IE0(bool);
        /// ADC12 Memory 1 Interrupt Enable
        ADC12IE1: 1 = struct ADC12IE1(bool);
        /// ADC12 Memory 2 Interrupt Enable
        ADC12IE2: 2 = struct ADC12IE2(bool);
        /// ADC12 Memory 3 Interrupt Enable
        ADC12IE3: 3 = struct ADC12IE3(bool);
        /// ADC12 Memory 4 Interrupt Enable
        ADC12IE4: 4 = struct ADC12IE4(bool);
        /// ADC12 Memory 5 Interrupt Enable
        ADC12IE5: 5 = struct ADC12IE5(bool);
        /// ADC12 Memory 6 Interrupt Enable
        ADC12IE6: 6 = struct ADC12IE6(bool);
        /// ADC12 Memory 7 Interrupt Enable
        ADC12IE7: 7 = struct ADC12IE7(bool);
        /// ADC12 Memory 8 Interrupt Enable
        ADC12IE8: 8 = struct ADC12IE8(bool);
        /// ADC12 Memory 9 Interrupt Enable
        ADC12IE9: 9 = struct ADC12IE9(bool);
        /// ADC12 Memory 10 Interrupt Enable
        ADC12IE10: 10 = struct ADC12IE10(bool);
        /// ADC12 Memory 11 Interrupt Enable
        ADC12IE11: 11 = struct ADC12IE11(bool);
        /// ADC12 Memory 12 Interrupt Enable
        ADC12IE12: 12 = struct ADC12IE12(bool);
        /// ADC12 Memory 13 Interrupt Enable
        ADC12IE13: 13 = struct ADC12IE13(bool);
        /// ADC12 Memory 14 Interrupt Enable
        ADC12IE14: 14 = struct ADC12IE14(bool);
        /// ADC12 Memory 15 Interrupt Enable
        ADC12IE15: 15 = struct ADC12IE15(bool);
    }
    /// ADC12+ Interrupt Vector Word
    rw ADC12IV @ 0x0e: u16 = 0_0 {
        /// ADC12+ Interrupt Vector Word
        ADC12IV: 0..15 = struct ADC12IVField(u16);
    }
    /// ADC12 Conversion Memory 0
    rw ADC12MEM0 @ 0x20: u16 = 0_0 {
        /// ADC12 Conversion Memory 0
        ADC12MEM0: 0..15 = struct ADC12MEM0Field(u16);
    }
    /// ADC12 Conversion Memory 1
    rw ADC12MEM1 @ 0x22: u16 = 0_0 {
        /// ADC12 Conversion Memory 1
        ADC12MEM1: 0..15 = struct ADC12MEM1Field(u16);
    }
    /// ADC12 Conversion Memory 2
    rw ADC12MEM2 @ 0x24: u16 = 0_0 {
        /// ADC12 Conversion Memory 2
        ADC12MEM2: 0..15 = struct ADC12MEM2Field(u16);
    }
    /// ADC12 Conversion Memory 3
    rw ADC12MEM3 @ 0x26: u16 = 0_0 {
        /// ADC12 Conversion Memory 3
        ADC12MEM3: 0..15 = struct ADC12MEM3Field(u16);
    }
    /// ADC12 Conversion Memory 4
    rw ADC12MEM4 @ 0x28: u16 = 0_0 {
        /// ADC12 Conversion Memory 4
        ADC12MEM4: 0..15 = struct ADC12MEM4Field(u16);
    }
    /// ADC12 Conversion Memory 5
    rw ADC12MEM5 @ 0x2a: u16 = 0_0 {
        /// ADC12 Conversion Memory 5
        ADC12MEM5: 0..15 = struct ADC12MEM5Field(u16);
    }
    /// ADC12 Conversion Memory 6
    rw ADC12MEM6 @ 0x2c: u16 = 0_0 {
        /// ADC12 Conversion Memory 6
        ADC12MEM6: 0..15 = struct ADC12MEM6Field(u16);
    }
    /// ADC12 Conversion Memory 7
    rw ADC12MEM7 @ 0x2e: u16 = 0_0 {
        /// ADC12 Conversion Memory 7
        ADC12MEM7: 0..15 = struct ADC12MEM7Field(u16);
    }
    /// ADC12 Conversion Memory 8
    rw ADC12MEM8 @ 0x30: u16 = 0_0 {
        /// ADC12 Conversion Memory 8
        ADC12MEM8: 0..15 = struct ADC12MEM8Field(u16);
    }
    /// ADC12 Conversion Memory 9
    rw ADC12MEM9 @ 0x32: u16 = 0_0 {
        /// ADC12 Conversion Memory 9
        ADC12MEM9: 0..15 = struct ADC12MEM9Field(u16);
    }
    /// ADC12 Conversion Memory 10
    rw ADC12MEM10 @ 0x34: u16 = 0_0 {
        /// ADC12 Conversion Memory 10
        ADC12MEM10: 0..15 = struct ADC12MEM10Field(u16);
    }
    /// ADC12 Conversion Memory 11
    rw ADC12MEM11 @ 0x36: u16 = 0_0 {
        /// ADC12 Conversion Memory 11
        ADC12MEM11: 0..15 = struct ADC12MEM11Field(u16);
    }
    /// ADC12 Conversion Memory 12
    rw ADC12MEM12 @ 0x38: u16 = 0_0 {
        /// ADC12 Conversion Memory 12
        ADC12MEM12: 0..15 = struct ADC12MEM12Field(u16);
    }
    /// ADC12 Conversion Memory 13
    rw ADC12MEM13 @ 0x3a: u16 = 0_0 {
        /// ADC12 Conversion Memory 13
        ADC12MEM13: 0..15 = struct ADC12MEM13Field(u16);
    }
    /// ADC12 Conversion Memory 14
    rw ADC12MEM14 @ 0x3c: u16 = 0_0 {
        /// ADC12 Conversion Memory 14
        ADC12MEM14: 0..15 = struct ADC12MEM14Field(u16);
    }
    /// ADC12 Conversion Memory 15
    rw ADC12MEM15 @ 0x3e: u16 = 0_0 {
        /// ADC12 Conversion Memory 15
        ADC12MEM15: 0..15 = struct ADC12MEM15Field(u16);
    }
    /// ADC12 Memory Control 0
    rw ADC12MCTL0 @ 0x10: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL0_ADC12INCH: 0..3 = enum ADC12MCTL0_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL0_ADC12SREF: 4..6 = enum ADC12MCTL0_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL0_ADC12EOS: 7 = struct ADC12MCTL0_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 1
    rw ADC12MCTL1 @ 0x11: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL1_ADC12INCH: 0..3 = enum ADC12MCTL1_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL1_ADC12SREF: 4..6 = enum ADC12MCTL1_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL1_ADC12EOS: 7 = struct ADC12MCTL1_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 2
    rw ADC12MCTL2 @ 0x12: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL2_ADC12INCH: 0..3 = enum ADC12MCTL2_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL2_ADC12SREF: 4..6 = enum ADC12MCTL2_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL2_ADC12EOS: 7 = struct ADC12MCTL2_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 3
    rw ADC12MCTL3 @ 0x13: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL3_ADC12INCH: 0..3 = enum ADC12MCTL3_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL3_ADC12SREF: 4..6 = enum ADC12MCTL3_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL3_ADC12EOS: 7 = struct ADC12MCTL3_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 4
    rw ADC12MCTL4 @ 0x14: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL4_ADC12INCH: 0..3 = enum ADC12MCTL4_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL4_ADC12SREF: 4..6 = enum ADC12MCTL4_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL4_ADC12EOS: 7 = struct ADC12MCTL4_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 5
    rw ADC12MCTL5 @ 0x15: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL5_ADC12INCH: 0..3 = enum ADC12MCTL5_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL5_ADC12SREF: 4..6 = enum ADC12MCTL5_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL5_ADC12EOS: 7 = struct ADC12MCTL5_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 6
    rw ADC12MCTL6 @ 0x16: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL6_ADC12INCH: 0..3 = enum ADC12MCTL6_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL6_ADC12SREF: 4..6 = enum ADC12MCTL6_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL6_ADC12EOS: 7 = struct ADC12MCTL6_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 7
    rw ADC12MCTL7 @ 0x17: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL7_ADC12INCH: 0..3 = enum ADC12MCTL7_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL7_ADC12SREF: 4..6 = enum ADC12MCTL7_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL7_ADC12EOS: 7 = struct ADC12MCTL7_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 8
    rw ADC12MCTL8 @ 0x18: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL8_ADC12INCH: 0..3 = enum ADC12MCTL8_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL8_ADC12SREF: 4..6 = enum ADC12MCTL8_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL8_ADC12EOS: 7 = struct ADC12MCTL8_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 9
    rw ADC12MCTL9 @ 0x19: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL9_ADC12INCH: 0..3 = enum ADC12MCTL9_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL9_ADC12SREF: 4..6 = enum ADC12MCTL9_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL9_ADC12EOS: 7 = struct ADC12MCTL9_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 10
    rw ADC12MCTL10 @ 0x1a: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL10_ADC12INCH: 0..3 = enum ADC12MCTL10_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL10_ADC12SREF: 4..6 = enum ADC12MCTL10_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL10_ADC12EOS: 7 = struct ADC12MCTL10_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 11
    rw ADC12MCTL11 @ 0x1b: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL11_ADC12INCH: 0..3 = enum ADC12MCTL11_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL11_ADC12SREF: 4..6 = enum ADC12MCTL11_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL11_ADC12EOS: 7 = struct ADC12MCTL11_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 12
    rw ADC12MCTL12 @ 0x1c: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL12_ADC12INCH: 0..3 = enum ADC12MCTL12_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL12_ADC12SREF: 4..6 = enum ADC12MCTL12_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL12_ADC12EOS: 7 = struct ADC12MCTL12_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 13
    rw ADC12MCTL13 @ 0x1d: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL13_ADC12INCH: 0..3 = enum ADC12MCTL13_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL13_ADC12SREF: 4..6 = enum ADC12MCTL13_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL13_ADC12EOS: 7 = struct ADC12MCTL13_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 14
    rw ADC12MCTL14 @ 0x1e: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL14_ADC12INCH: 0..3 = enum ADC12MCTL14_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL14_ADC12SREF: 4..6 = enum ADC12MCTL14_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL14_ADC12EOS: 7 = struct ADC12MCTL14_ADC12EOS(bool);
    }
    /// ADC12 Memory Control 15
    rw ADC12MCTL15 @ 0x1f: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL15_ADC12INCH: 0..3 = enum ADC12MCTL15_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b0000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b0001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b0010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b0011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b0100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b0101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b0110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b0111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b1000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b1001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b1010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b1011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b1100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b1101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b1110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b1111,
        }
        /// ADC12 Select Reference Bit 0
        ADC12MCTL15_ADC12SREF: 4..6 = enum ADC12MCTL15_ADC12SREF {
            /// ADC12 Select Reference 0
            ADC12SREF_0 = 0b000,
            /// ADC12 Select Reference 1
            ADC12SREF_1 = 0b001,
            /// ADC12 Select Reference 2
            ADC12SREF_2 = 0b010,
            /// ADC12 Select Reference 3
            ADC12SREF_3 = 0b011,
            /// ADC12 Select Reference 4
            ADC12SREF_4 = 0b100,
            /// ADC12 Select Reference 5
            ADC12SREF_5 = 0b101,
            /// ADC12 Select Reference 6
            ADC12SREF_6 = 0b110,
            /// ADC12 Select Reference 7
            ADC12SREF_7 = 0b111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL15_ADC12EOS: 7 = struct ADC12MCTL15_ADC12EOS(bool);
    }
}
