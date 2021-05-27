//! ADC12

utils::periph! {
    /// ADC12
    ADC12;
    /// ADC12+ Control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// ADC12 Start Conversion
        SC: 0 = struct SC(bool);
        /// ADC12 Enable Conversion
        ENC: 1 = struct ENC(bool);
        /// ADC12 Timer Overflow interrupt enable
        TOVIE: 2 = struct TOVIE(bool);
        /// ADC12 Overflow interrupt enable
        OVIE: 3 = struct OVIE(bool);
        /// ADC12 On/enable
        ON: 4 = struct ON(bool);
        /// ADC12 Reference on
        REFON: 5 = struct REFON(bool);
        /// ADC12 Ref 0:1.5V / 1:2.5V
        REF2_5V: 6 = struct REF2_5V(bool);
        /// ADC12 Multiple SampleConversion
        MSC: 7 = struct MSC(bool);
        /// ADC12 Sample Hold 0 Select Bit: 0
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
        /// ADC12 Sample Hold 1 Select Bit: 0
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
    /// ADC12+ Control 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// ADC12 Busy
        BUSY: 0 = struct BUSY(bool);
        /// ADC12 Conversion Sequence Select Bit: 0
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
        /// ADC12 Clock Source Select Bit: 0
        SSEL: 3..4 = enum SSEL {
            /// ADC12 Clock Source Select: 0
            SSEL_0 = 0b00,
            /// ADC12 Clock Source Select: 1
            SSEL_1 = 0b01,
            /// ADC12 Clock Source Select: 2
            SSEL_2 = 0b10,
            /// ADC12 Clock Source Select: 3
            SSEL_3 = 0b11,
        }
        /// ADC12 Clock Divider Select Bit: 0
        DIV: 5..7 = enum DIV {
            /// ADC12 Clock Divider Select: 0
            DIV_0 = 0b000,
            /// ADC12 Clock Divider Select: 1
            DIV_1 = 0b001,
            /// ADC12 Clock Divider Select: 2
            DIV_2 = 0b010,
            /// ADC12 Clock Divider Select: 3
            DIV_3 = 0b011,
            /// ADC12 Clock Divider Select: 4
            DIV_4 = 0b100,
            /// ADC12 Clock Divider Select: 5
            DIV_5 = 0b101,
            /// ADC12 Clock Divider Select: 6
            DIV_6 = 0b110,
            /// ADC12 Clock Divider Select: 7
            DIV_7 = 0b111,
        }
        /// ADC12 Invert Sample Hold Signal
        ISSH: 8 = struct ISSH(bool);
        /// ADC12 Sample/Hold Pulse Mode
        SHP: 9 = struct SHP(bool);
        /// ADC12 Sample/Hold Source Bit: 0
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
        /// ADC12 Conversion Start Address Bit: 0
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
    /// ADC12+ Control 2
    rw CTL2 @ 0x04: u16 = 0_0 {
        /// ADC12+ Reference Burst
        REFBURST: 0 = struct REFBURST(bool);
        /// ADC12+ Reference Out
        REFOUT: 1 = struct REFOUT(bool);
        /// ADC12+ Sampling Rate
        SR: 2 = struct SR(bool);
        /// ADC12+ Data Format
        DF: 3 = struct DF(bool);
        /// ADC12+ Resolution Bit: 0
        RES: 4..5 = enum RES {
            /// ADC12+ Resolution : 8 Bit
            RES_0 = 0b00,
            /// ADC12+ Resolution : 10 Bit
            RES_1 = 0b01,
            /// ADC12+ Resolution : 12 Bit
            RES_2 = 0b10,
            /// ADC12+ Resolution : reserved
            RES_3 = 0b11,
        }
        /// ADC12+ Temperature Sensor Off
        TCOFF: 7 = struct TCOFF(bool);
        /// ADC12+ predivider 0:/1   1:/4
        PDIV: 8 = struct PDIV(bool);
    }
    /// ADC12+ Interrupt Flag
    rw IFG @ 0x0a: u16 = 0_0 {
        /// ADC12 Memory 0 Interrupt Flag
        IFG0: 0 = struct IFG0(bool);
        /// ADC12 Memory 1 Interrupt Flag
        IFG1: 1 = struct IFG1(bool);
        /// ADC12 Memory 2 Interrupt Flag
        IFG2: 2 = struct IFG2(bool);
        /// ADC12 Memory 3 Interrupt Flag
        IFG3: 3 = struct IFG3(bool);
        /// ADC12 Memory 4 Interrupt Flag
        IFG4: 4 = struct IFG4(bool);
        /// ADC12 Memory 5 Interrupt Flag
        IFG5: 5 = struct IFG5(bool);
        /// ADC12 Memory 6 Interrupt Flag
        IFG6: 6 = struct IFG6(bool);
        /// ADC12 Memory 7 Interrupt Flag
        IFG7: 7 = struct IFG7(bool);
        /// ADC12 Memory 8 Interrupt Flag
        IFG8: 8 = struct IFG8(bool);
        /// ADC12 Memory 9 Interrupt Flag
        IFG9: 9 = struct IFG9(bool);
        /// ADC12 Memory 10 Interrupt Flag
        IFG10: 10 = struct IFG10(bool);
        /// ADC12 Memory 11 Interrupt Flag
        IFG11: 11 = struct IFG11(bool);
        /// ADC12 Memory 12 Interrupt Flag
        IFG12: 12 = struct IFG12(bool);
        /// ADC12 Memory 13 Interrupt Flag
        IFG13: 13 = struct IFG13(bool);
        /// ADC12 Memory 14 Interrupt Flag
        IFG14: 14 = struct IFG14(bool);
        /// ADC12 Memory 15 Interrupt Flag
        IFG15: 15 = struct IFG15(bool);
    }
    /// ADC12+ Interrupt Enable
    rw IE @ 0x0c: u16 = 0_0 {
        /// ADC12 Memory 0 Interrupt Enable
        IE0: 0 = struct IE0(bool);
        /// ADC12 Memory 1 Interrupt Enable
        IE1: 1 = struct IE1(bool);
        /// ADC12 Memory 2 Interrupt Enable
        IE2: 2 = struct IE2(bool);
        /// ADC12 Memory 3 Interrupt Enable
        IE3: 3 = struct IE3(bool);
        /// ADC12 Memory 4 Interrupt Enable
        IE4: 4 = struct IE4(bool);
        /// ADC12 Memory 5 Interrupt Enable
        IE5: 5 = struct IE5(bool);
        /// ADC12 Memory 6 Interrupt Enable
        IE6: 6 = struct IE6(bool);
        /// ADC12 Memory 7 Interrupt Enable
        IE7: 7 = struct IE7(bool);
        /// ADC12 Memory 8 Interrupt Enable
        IE8: 8 = struct IE8(bool);
        /// ADC12 Memory 9 Interrupt Enable
        IE9: 9 = struct IE9(bool);
        /// ADC12 Memory 10 Interrupt Enable
        IE10: 10 = struct IE10(bool);
        /// ADC12 Memory 11 Interrupt Enable
        IE11: 11 = struct IE11(bool);
        /// ADC12 Memory 12 Interrupt Enable
        IE12: 12 = struct IE12(bool);
        /// ADC12 Memory 13 Interrupt Enable
        IE13: 13 = struct IE13(bool);
        /// ADC12 Memory 14 Interrupt Enable
        IE14: 14 = struct IE14(bool);
        /// ADC12 Memory 15 Interrupt Enable
        IE15: 15 = struct IE15(bool);
    }
    /// ADC12+ Interrupt Vector Word
    rw IV @ 0x0e: u16 = 0_0 {
        /// ADC12+ Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
    /// ADC12 Conversion Memory 0
    rw MEM0 @ 0x20: u16 = 0_0 {
        /// ADC12 Conversion Memory 0
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// ADC12 Conversion Memory 1
    rw MEM1 @ 0x22: u16 = 0_0 {
        /// ADC12 Conversion Memory 1
        MEM1: 0..15 = struct MEM1Field(u16);
    }
    /// ADC12 Conversion Memory 2
    rw MEM2 @ 0x24: u16 = 0_0 {
        /// ADC12 Conversion Memory 2
        MEM2: 0..15 = struct MEM2Field(u16);
    }
    /// ADC12 Conversion Memory 3
    rw MEM3 @ 0x26: u16 = 0_0 {
        /// ADC12 Conversion Memory 3
        MEM3: 0..15 = struct MEM3Field(u16);
    }
    /// ADC12 Conversion Memory 4
    rw MEM4 @ 0x28: u16 = 0_0 {
        /// ADC12 Conversion Memory 4
        MEM4: 0..15 = struct MEM4Field(u16);
    }
    /// ADC12 Conversion Memory 5
    rw MEM5 @ 0x2a: u16 = 0_0 {
        /// ADC12 Conversion Memory 5
        MEM5: 0..15 = struct MEM5Field(u16);
    }
    /// ADC12 Conversion Memory 6
    rw MEM6 @ 0x2c: u16 = 0_0 {
        /// ADC12 Conversion Memory 6
        MEM6: 0..15 = struct MEM6Field(u16);
    }
    /// ADC12 Conversion Memory 7
    rw MEM7 @ 0x2e: u16 = 0_0 {
        /// ADC12 Conversion Memory 7
        MEM7: 0..15 = struct MEM7Field(u16);
    }
    /// ADC12 Conversion Memory 8
    rw MEM8 @ 0x30: u16 = 0_0 {
        /// ADC12 Conversion Memory 8
        MEM8: 0..15 = struct MEM8Field(u16);
    }
    /// ADC12 Conversion Memory 9
    rw MEM9 @ 0x32: u16 = 0_0 {
        /// ADC12 Conversion Memory 9
        MEM9: 0..15 = struct MEM9Field(u16);
    }
    /// ADC12 Conversion Memory 10
    rw MEM10 @ 0x34: u16 = 0_0 {
        /// ADC12 Conversion Memory 10
        MEM10: 0..15 = struct MEM10Field(u16);
    }
    /// ADC12 Conversion Memory 11
    rw MEM11 @ 0x36: u16 = 0_0 {
        /// ADC12 Conversion Memory 11
        MEM11: 0..15 = struct MEM11Field(u16);
    }
    /// ADC12 Conversion Memory 12
    rw MEM12 @ 0x38: u16 = 0_0 {
        /// ADC12 Conversion Memory 12
        MEM12: 0..15 = struct MEM12Field(u16);
    }
    /// ADC12 Conversion Memory 13
    rw MEM13 @ 0x3a: u16 = 0_0 {
        /// ADC12 Conversion Memory 13
        MEM13: 0..15 = struct MEM13Field(u16);
    }
    /// ADC12 Conversion Memory 14
    rw MEM14 @ 0x3c: u16 = 0_0 {
        /// ADC12 Conversion Memory 14
        MEM14: 0..15 = struct MEM14Field(u16);
    }
    /// ADC12 Conversion Memory 15
    rw MEM15 @ 0x3e: u16 = 0_0 {
        /// ADC12 Conversion Memory 15
        MEM15: 0..15 = struct MEM15Field(u16);
    }
    /// ADC12 Memory Control 0
    rw MCTL0 @ 0x10: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL0_INCH: 0..3 = enum MCTL0_INCH {
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
        MCTL0_SREF: 4..6 = enum MCTL0_SREF {
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
        MCTL0_EOS: 7 = struct MCTL0_EOS(bool);
    }
    /// ADC12 Memory Control 1
    rw MCTL1 @ 0x11: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL1_INCH: 0..3 = enum MCTL1_INCH {
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
        MCTL1_SREF: 4..6 = enum MCTL1_SREF {
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
        MCTL1_EOS: 7 = struct MCTL1_EOS(bool);
    }
    /// ADC12 Memory Control 2
    rw MCTL2 @ 0x12: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL2_INCH: 0..3 = enum MCTL2_INCH {
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
        MCTL2_SREF: 4..6 = enum MCTL2_SREF {
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
        MCTL2_EOS: 7 = struct MCTL2_EOS(bool);
    }
    /// ADC12 Memory Control 3
    rw MCTL3 @ 0x13: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL3_INCH: 0..3 = enum MCTL3_INCH {
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
        MCTL3_SREF: 4..6 = enum MCTL3_SREF {
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
        MCTL3_EOS: 7 = struct MCTL3_EOS(bool);
    }
    /// ADC12 Memory Control 4
    rw MCTL4 @ 0x14: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL4_INCH: 0..3 = enum MCTL4_INCH {
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
        MCTL4_SREF: 4..6 = enum MCTL4_SREF {
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
        MCTL4_EOS: 7 = struct MCTL4_EOS(bool);
    }
    /// ADC12 Memory Control 5
    rw MCTL5 @ 0x15: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL5_INCH: 0..3 = enum MCTL5_INCH {
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
        MCTL5_SREF: 4..6 = enum MCTL5_SREF {
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
        MCTL5_EOS: 7 = struct MCTL5_EOS(bool);
    }
    /// ADC12 Memory Control 6
    rw MCTL6 @ 0x16: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL6_INCH: 0..3 = enum MCTL6_INCH {
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
        MCTL6_SREF: 4..6 = enum MCTL6_SREF {
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
        MCTL6_EOS: 7 = struct MCTL6_EOS(bool);
    }
    /// ADC12 Memory Control 7
    rw MCTL7 @ 0x17: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL7_INCH: 0..3 = enum MCTL7_INCH {
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
        MCTL7_SREF: 4..6 = enum MCTL7_SREF {
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
        MCTL7_EOS: 7 = struct MCTL7_EOS(bool);
    }
    /// ADC12 Memory Control 8
    rw MCTL8 @ 0x18: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL8_INCH: 0..3 = enum MCTL8_INCH {
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
        MCTL8_SREF: 4..6 = enum MCTL8_SREF {
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
        MCTL8_EOS: 7 = struct MCTL8_EOS(bool);
    }
    /// ADC12 Memory Control 9
    rw MCTL9 @ 0x19: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL9_INCH: 0..3 = enum MCTL9_INCH {
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
        MCTL9_SREF: 4..6 = enum MCTL9_SREF {
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
        MCTL9_EOS: 7 = struct MCTL9_EOS(bool);
    }
    /// ADC12 Memory Control 10
    rw MCTL10 @ 0x1a: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL10_INCH: 0..3 = enum MCTL10_INCH {
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
        MCTL10_SREF: 4..6 = enum MCTL10_SREF {
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
        MCTL10_EOS: 7 = struct MCTL10_EOS(bool);
    }
    /// ADC12 Memory Control 11
    rw MCTL11 @ 0x1b: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL11_INCH: 0..3 = enum MCTL11_INCH {
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
        MCTL11_SREF: 4..6 = enum MCTL11_SREF {
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
        MCTL11_EOS: 7 = struct MCTL11_EOS(bool);
    }
    /// ADC12 Memory Control 12
    rw MCTL12 @ 0x1c: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL12_INCH: 0..3 = enum MCTL12_INCH {
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
        MCTL12_SREF: 4..6 = enum MCTL12_SREF {
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
        MCTL12_EOS: 7 = struct MCTL12_EOS(bool);
    }
    /// ADC12 Memory Control 13
    rw MCTL13 @ 0x1d: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL13_INCH: 0..3 = enum MCTL13_INCH {
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
        MCTL13_SREF: 4..6 = enum MCTL13_SREF {
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
        MCTL13_EOS: 7 = struct MCTL13_EOS(bool);
    }
    /// ADC12 Memory Control 14
    rw MCTL14 @ 0x1e: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL14_INCH: 0..3 = enum MCTL14_INCH {
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
        MCTL14_SREF: 4..6 = enum MCTL14_SREF {
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
        MCTL14_EOS: 7 = struct MCTL14_EOS(bool);
    }
    /// ADC12 Memory Control 15
    rw MCTL15 @ 0x1f: u8 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL15_INCH: 0..3 = enum MCTL15_INCH {
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
        MCTL15_SREF: 4..6 = enum MCTL15_SREF {
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
        MCTL15_EOS: 7 = struct MCTL15_EOS(bool);
    }
}
