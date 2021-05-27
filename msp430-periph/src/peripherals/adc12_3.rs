//! ADC12

utils::periph! {
    /// ADC12
    ADC12;
    /// ADC12 B Control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// ADC12 Start Conversion
        SC: 0 = struct SC(bool);
        /// ADC12 Enable Conversion
        ENC: 1 = struct ENC(bool);
        /// ADC12 On/enable
        ON: 4 = struct ON(bool);
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
    /// ADC12 B Control 1
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
        SHS: 10..12 = enum SHS {
            /// ADC12 Sample/Hold Source: 0
            SHS_0 = 0b000,
            /// ADC12 Sample/Hold Source: 1
            SHS_1 = 0b001,
            /// ADC12 Sample/Hold Source: 2
            SHS_2 = 0b010,
            /// ADC12 Sample/Hold Source: 3
            SHS_3 = 0b011,
            /// ADC12 Sample/Hold Source: 4
            SHS_4 = 0b100,
            /// ADC12 Sample/Hold Source: 5
            SHS_5 = 0b101,
            /// ADC12 Sample/Hold Source: 6
            SHS_6 = 0b110,
            /// ADC12 Sample/Hold Source: 7
            SHS_7 = 0b111,
        }
        /// ADC12 Predivider Bit: 0
        PDIV: 13..14 = enum PDIV {
            /// ADC12 Clock predivider Select 0
            PDIV_0 = 0b00,
            /// ADC12 Clock predivider Select 1
            PDIV_1 = 0b01,
            /// ADC12 Clock predivider Select 2
            PDIV_2 = 0b10,
            /// ADC12 Clock predivider Select 3
            PDIV_3 = 0b11,
        }
    }
    /// ADC12 B Control 2
    rw CTL2 @ 0x04: u16 = 0_0 {
        /// ADC12 Power Mode
        PWRMD: 0 = struct PWRMD(bool);
        /// ADC12 Data Format
        DF: 3 = struct DF(bool);
        /// ADC12 Resolution Bit: 0
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
    }
    /// ADC12 B Control 3
    rw CTL3 @ 0x06: u16 = 0_0 {
        /// ADC12 Conversion Start Address Bit: 0
        CSTARTADD: 0..4 = enum CSTARTADD {
            /// ADC12 Conversion Start Address: 0
            CSTARTADD_0 = 0b00000,
            /// ADC12 Conversion Start Address: 1
            CSTARTADD_1 = 0b00001,
            /// ADC12 Conversion Start Address: 2
            CSTARTADD_2 = 0b00010,
            /// ADC12 Conversion Start Address: 3
            CSTARTADD_3 = 0b00011,
            /// ADC12 Conversion Start Address: 4
            CSTARTADD_4 = 0b00100,
            /// ADC12 Conversion Start Address: 5
            CSTARTADD_5 = 0b00101,
            /// ADC12 Conversion Start Address: 6
            CSTARTADD_6 = 0b00110,
            /// ADC12 Conversion Start Address: 7
            CSTARTADD_7 = 0b00111,
            /// ADC12 Conversion Start Address: 8
            CSTARTADD_8 = 0b01000,
            /// ADC12 Conversion Start Address: 9
            CSTARTADD_9 = 0b01001,
            /// ADC12 Conversion Start Address: 10
            CSTARTADD_10 = 0b01010,
            /// ADC12 Conversion Start Address: 11
            CSTARTADD_11 = 0b01011,
            /// ADC12 Conversion Start Address: 12
            CSTARTADD_12 = 0b01100,
            /// ADC12 Conversion Start Address: 13
            CSTARTADD_13 = 0b01101,
            /// ADC12 Conversion Start Address: 14
            CSTARTADD_14 = 0b01110,
            /// ADC12 Conversion Start Address: 15
            CSTARTADD_15 = 0b01111,
            /// ADC12 Conversion Start Address: 16
            CSTARTADD_16 = 0b10000,
            /// ADC12 Conversion Start Address: 17
            CSTARTADD_17 = 0b10001,
            /// ADC12 Conversion Start Address: 18
            CSTARTADD_18 = 0b10010,
            /// ADC12 Conversion Start Address: 19
            CSTARTADD_19 = 0b10011,
            /// ADC12 Conversion Start Address: 20
            CSTARTADD_20 = 0b10100,
            /// ADC12 Conversion Start Address: 21
            CSTARTADD_21 = 0b10101,
            /// ADC12 Conversion Start Address: 22
            CSTARTADD_22 = 0b10110,
            /// ADC12 Conversion Start Address: 23
            CSTARTADD_23 = 0b10111,
            /// ADC12 Conversion Start Address: 24
            CSTARTADD_24 = 0b11000,
            /// ADC12 Conversion Start Address: 25
            CSTARTADD_25 = 0b11001,
            /// ADC12 Conversion Start Address: 26
            CSTARTADD_26 = 0b11010,
            /// ADC12 Conversion Start Address: 27
            CSTARTADD_27 = 0b11011,
            /// ADC12 Conversion Start Address: 28
            CSTARTADD_28 = 0b11100,
            /// ADC12 Conversion Start Address: 29
            CSTARTADD_29 = 0b11101,
            /// ADC12 Conversion Start Address: 30
            CSTARTADD_30 = 0b11110,
            /// ADC12 Conversion Start Address: 31
            CSTARTADD_31 = 0b11111,
        }
        /// ADC12 Internal AVCC/2 select
        BATMAP: 6 = struct BATMAP(bool);
        /// ADC12 Internal TempSensor select
        TCMAP: 7 = struct TCMAP(bool);
        /// ADC12 Internal Channel 0 select
        ICH0MAP: 8 = struct ICH0MAP(bool);
        /// ADC12 Internal Channel 1 select
        ICH1MAP: 9 = struct ICH1MAP(bool);
        /// ADC12 Internal Channel 2 select
        ICH2MAP: 10 = struct ICH2MAP(bool);
        /// ADC12 Internal Channel 3 select
        ICH3MAP: 11 = struct ICH3MAP(bool);
    }
    /// ADC12 B Window Comparator High Threshold
    rw LO @ 0x08: u16 = 0_0 {
        /// ADC12 B Window Comparator High Threshold
        LO: 0..15 = struct LOField(u16);
    }
    /// ADC12 B Window Comparator High Threshold
    rw HI @ 0x0a: u16 = 0_0 {
        /// ADC12 B Window Comparator High Threshold
        HI: 0..15 = struct HIField(u16);
    }
    /// ADC12 B Interrupt Flag 0
    rw IFGR0 @ 0x0c: u16 = 0_0 {
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
    /// ADC12 B Interrupt Flag 1
    rw IFGR1 @ 0x0e: u16 = 0_0 {
        /// ADC12 Memory 16 Interrupt Flag
        IFG16: 0 = struct IFG16(bool);
        /// ADC12 Memory 17 Interrupt Flag
        IFG17: 1 = struct IFG17(bool);
        /// ADC12 Memory 18 Interrupt Flag
        IFG18: 2 = struct IFG18(bool);
        /// ADC12 Memory 19 Interrupt Flag
        IFG19: 3 = struct IFG19(bool);
        /// ADC12 Memory 20 Interrupt Flag
        IFG20: 4 = struct IFG20(bool);
        /// ADC12 Memory 21 Interrupt Flag
        IFG21: 5 = struct IFG21(bool);
        /// ADC12 Memory 22 Interrupt Flag
        IFG22: 6 = struct IFG22(bool);
        /// ADC12 Memory 23 Interrupt Flag
        IFG23: 7 = struct IFG23(bool);
        /// ADC12 Memory 24 Interrupt Flag
        IFG24: 8 = struct IFG24(bool);
        /// ADC12 Memory 25 Interrupt Flag
        IFG25: 9 = struct IFG25(bool);
        /// ADC12 Memory 26 Interrupt Flag
        IFG26: 10 = struct IFG26(bool);
        /// ADC12 Memory 27 Interrupt Flag
        IFG27: 11 = struct IFG27(bool);
        /// ADC12 Memory 28 Interrupt Flag
        IFG28: 12 = struct IFG28(bool);
        /// ADC12 Memory 29 Interrupt Flag
        IFG29: 13 = struct IFG29(bool);
        /// ADC12 Memory 30 Interrupt Flag
        IFG30: 14 = struct IFG30(bool);
        /// ADC12 Memory 31 Interrupt Flag
        IFG31: 15 = struct IFG31(bool);
    }
    /// ADC12 B Interrupt Flag 2
    rw IFGR2 @ 0x10: u16 = 0_0 {
        /// ADC12 Interrupt Flag for the inside of window of the Window comparator
        INIFG: 1 = struct INIFG(bool);
        /// ADC12 Interrupt Flag for lower threshold of the Window comparator
        LOIFG: 2 = struct LOIFG(bool);
        /// ADC12 Interrupt Flag for upper threshold of the Window comparator
        HIIFG: 3 = struct HIIFG(bool);
        /// ADC12 ADC12MEMx Overflow interrupt Flag
        OVIFG: 4 = struct OVIFG(bool);
        /// ADC12 Timer Overflow interrupt Flag
        TOVIFG: 5 = struct TOVIFG(bool);
        /// ADC12 local buffered reference ready interrupt Flag
        RDYIFG: 6 = struct RDYIFG(bool);
    }
    /// ADC12 B Interrupt Enable 0
    rw IER0 @ 0x12: u16 = 0_0 {
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
    /// ADC12 B Interrupt Enable 1
    rw IER1 @ 0x14: u16 = 0_0 {
        /// ADC12 Memory 16 Interrupt Enable
        IE16: 0 = struct IE16(bool);
        /// ADC12 Memory 17 Interrupt Enable
        IE17: 1 = struct IE17(bool);
        /// ADC12 Memory 18 Interrupt Enable
        IE18: 2 = struct IE18(bool);
        /// ADC12 Memory 19 Interrupt Enable
        IE19: 3 = struct IE19(bool);
        /// ADC12 Memory 20 Interrupt Enable
        IE20: 4 = struct IE20(bool);
        /// ADC12 Memory 21 Interrupt Enable
        IE21: 5 = struct IE21(bool);
        /// ADC12 Memory 22 Interrupt Enable
        IE22: 6 = struct IE22(bool);
        /// ADC12 Memory 23 Interrupt Enable
        IE23: 7 = struct IE23(bool);
        /// ADC12 Memory 24 Interrupt Enable
        IE24: 8 = struct IE24(bool);
        /// ADC12 Memory 25 Interrupt Enable
        IE25: 9 = struct IE25(bool);
        /// ADC12 Memory 26 Interrupt Enable
        IE26: 10 = struct IE26(bool);
        /// ADC12 Memory 27 Interrupt Enable
        IE27: 11 = struct IE27(bool);
        /// ADC12 Memory 28 Interrupt Enable
        IE28: 12 = struct IE28(bool);
        /// ADC12 Memory 29 Interrupt Enable
        IE29: 13 = struct IE29(bool);
        /// ADC12 Memory 30 Interrupt Enable
        IE30: 14 = struct IE30(bool);
        /// ADC12 Memory 31 Interrupt Enable
        IE31: 15 = struct IE31(bool);
    }
    /// ADC12 B Interrupt Enable 2
    rw IER2 @ 0x16: u16 = 0_0 {
        /// ADC12 Interrupt enable for the inside of window of the Window comparator
        INIE: 1 = struct INIE(bool);
        /// ADC12 Interrupt enable for lower threshold of the Window comparator
        LOIE: 2 = struct LOIE(bool);
        /// ADC12 Interrupt enable for upper threshold of the Window comparator
        HIIE: 3 = struct HIIE(bool);
        /// ADC12 ADC12MEMx Overflow interrupt enable
        OVIE: 4 = struct OVIE(bool);
        /// ADC12 Timer Overflow interrupt enable
        TOVIE: 5 = struct TOVIE(bool);
        /// ADC12 local buffered reference ready interrupt enable
        RDYIE: 6 = struct RDYIE(bool);
    }
    /// ADC12 B Interrupt Vector Word
    rw IV @ 0x18: u16 = 0_0 {
        /// ADC12 B Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
    /// ADC12 Memory Control 0
    rw MCTL0 @ 0x20: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL0_INCH: 0..4 = enum MCTL0_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL0_EOS: 7 = struct MCTL0_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL0_VRSEL: 8..11 = enum MCTL0_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL0_DIF: 13 = struct MCTL0_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL0_WINC: 14 = struct MCTL0_WINC(bool);
    }
    /// ADC12 Memory Control 1
    rw MCTL1 @ 0x22: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL1_INCH: 0..4 = enum MCTL1_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL1_EOS: 7 = struct MCTL1_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL1_VRSEL: 8..11 = enum MCTL1_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL1_DIF: 13 = struct MCTL1_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL1_WINC: 14 = struct MCTL1_WINC(bool);
    }
    /// ADC12 Memory Control 2
    rw MCTL2 @ 0x24: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL2_INCH: 0..4 = enum MCTL2_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL2_EOS: 7 = struct MCTL2_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL2_VRSEL: 8..11 = enum MCTL2_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL2_DIF: 13 = struct MCTL2_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL2_WINC: 14 = struct MCTL2_WINC(bool);
    }
    /// ADC12 Memory Control 3
    rw MCTL3 @ 0x26: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL3_INCH: 0..4 = enum MCTL3_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL3_EOS: 7 = struct MCTL3_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL3_VRSEL: 8..11 = enum MCTL3_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL3_DIF: 13 = struct MCTL3_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL3_WINC: 14 = struct MCTL3_WINC(bool);
    }
    /// ADC12 Memory Control 4
    rw MCTL4 @ 0x28: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL4_INCH: 0..4 = enum MCTL4_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL4_EOS: 7 = struct MCTL4_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL4_VRSEL: 8..11 = enum MCTL4_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL4_DIF: 13 = struct MCTL4_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL4_WINC: 14 = struct MCTL4_WINC(bool);
    }
    /// ADC12 Memory Control 5
    rw MCTL5 @ 0x2a: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL5_INCH: 0..4 = enum MCTL5_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL5_EOS: 7 = struct MCTL5_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL5_VRSEL: 8..11 = enum MCTL5_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL5_DIF: 13 = struct MCTL5_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL5_WINC: 14 = struct MCTL5_WINC(bool);
    }
    /// ADC12 Memory Control 6
    rw MCTL6 @ 0x2c: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL6_INCH: 0..4 = enum MCTL6_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL6_EOS: 7 = struct MCTL6_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL6_VRSEL: 8..11 = enum MCTL6_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL6_DIF: 13 = struct MCTL6_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL6_WINC: 14 = struct MCTL6_WINC(bool);
    }
    /// ADC12 Memory Control 7
    rw MCTL7 @ 0x2e: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL7_INCH: 0..4 = enum MCTL7_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL7_EOS: 7 = struct MCTL7_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL7_VRSEL: 8..11 = enum MCTL7_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL7_DIF: 13 = struct MCTL7_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL7_WINC: 14 = struct MCTL7_WINC(bool);
    }
    /// ADC12 Memory Control 8
    rw MCTL8 @ 0x30: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL8_INCH: 0..4 = enum MCTL8_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL8_EOS: 7 = struct MCTL8_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL8_VRSEL: 8..11 = enum MCTL8_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL8_DIF: 13 = struct MCTL8_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL8_WINC: 14 = struct MCTL8_WINC(bool);
    }
    /// ADC12 Memory Control 9
    rw MCTL9 @ 0x32: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL9_INCH: 0..4 = enum MCTL9_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL9_EOS: 7 = struct MCTL9_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL9_VRSEL: 8..11 = enum MCTL9_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL9_DIF: 13 = struct MCTL9_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL9_WINC: 14 = struct MCTL9_WINC(bool);
    }
    /// ADC12 Memory Control 10
    rw MCTL10 @ 0x34: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL10_INCH: 0..4 = enum MCTL10_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL10_EOS: 7 = struct MCTL10_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL10_VRSEL: 8..11 = enum MCTL10_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL10_DIF: 13 = struct MCTL10_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL10_WINC: 14 = struct MCTL10_WINC(bool);
    }
    /// ADC12 Memory Control 11
    rw MCTL11 @ 0x36: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL11_INCH: 0..4 = enum MCTL11_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL11_EOS: 7 = struct MCTL11_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL11_VRSEL: 8..11 = enum MCTL11_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL11_DIF: 13 = struct MCTL11_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL11_WINC: 14 = struct MCTL11_WINC(bool);
    }
    /// ADC12 Memory Control 12
    rw MCTL12 @ 0x38: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL12_INCH: 0..4 = enum MCTL12_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL12_EOS: 7 = struct MCTL12_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL12_VRSEL: 8..11 = enum MCTL12_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL12_DIF: 13 = struct MCTL12_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL12_WINC: 14 = struct MCTL12_WINC(bool);
    }
    /// ADC12 Memory Control 13
    rw MCTL13 @ 0x3a: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL13_INCH: 0..4 = enum MCTL13_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL13_EOS: 7 = struct MCTL13_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL13_VRSEL: 8..11 = enum MCTL13_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL13_DIF: 13 = struct MCTL13_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL13_WINC: 14 = struct MCTL13_WINC(bool);
    }
    /// ADC12 Memory Control 14
    rw MCTL14 @ 0x3c: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL14_INCH: 0..4 = enum MCTL14_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL14_EOS: 7 = struct MCTL14_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL14_VRSEL: 8..11 = enum MCTL14_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL14_DIF: 13 = struct MCTL14_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL14_WINC: 14 = struct MCTL14_WINC(bool);
    }
    /// ADC12 Memory Control 15
    rw MCTL15 @ 0x3e: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL15_INCH: 0..4 = enum MCTL15_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL15_EOS: 7 = struct MCTL15_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL15_VRSEL: 8..11 = enum MCTL15_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL15_DIF: 13 = struct MCTL15_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL15_WINC: 14 = struct MCTL15_WINC(bool);
    }
    /// ADC12 Memory Control 16
    rw MCTL16 @ 0x40: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL16_INCH: 0..4 = enum MCTL16_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL16_EOS: 7 = struct MCTL16_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL16_VRSEL: 8..11 = enum MCTL16_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL16_DIF: 13 = struct MCTL16_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL16_WINC: 14 = struct MCTL16_WINC(bool);
    }
    /// ADC12 Memory Control 17
    rw MCTL17 @ 0x42: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL17_INCH: 0..4 = enum MCTL17_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL17_EOS: 7 = struct MCTL17_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL17_VRSEL: 8..11 = enum MCTL17_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL17_DIF: 13 = struct MCTL17_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL17_WINC: 14 = struct MCTL17_WINC(bool);
    }
    /// ADC12 Memory Control 18
    rw MCTL18 @ 0x44: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL18_INCH: 0..4 = enum MCTL18_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL18_EOS: 7 = struct MCTL18_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL18_VRSEL: 8..11 = enum MCTL18_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL18_DIF: 13 = struct MCTL18_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL18_WINC: 14 = struct MCTL18_WINC(bool);
    }
    /// ADC12 Memory Control 19
    rw MCTL19 @ 0x46: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL19_INCH: 0..4 = enum MCTL19_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL19_EOS: 7 = struct MCTL19_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL19_VRSEL: 8..11 = enum MCTL19_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL19_DIF: 13 = struct MCTL19_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL19_WINC: 14 = struct MCTL19_WINC(bool);
    }
    /// ADC12 Memory Control 20
    rw MCTL20 @ 0x48: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL20_INCH: 0..4 = enum MCTL20_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL20_EOS: 7 = struct MCTL20_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL20_VRSEL: 8..11 = enum MCTL20_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL20_DIF: 13 = struct MCTL20_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL20_WINC: 14 = struct MCTL20_WINC(bool);
    }
    /// ADC12 Memory Control 21
    rw MCTL21 @ 0x4a: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL21_INCH: 0..4 = enum MCTL21_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL21_EOS: 7 = struct MCTL21_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL21_VRSEL: 8..11 = enum MCTL21_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL21_DIF: 13 = struct MCTL21_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL21_WINC: 14 = struct MCTL21_WINC(bool);
    }
    /// ADC12 Memory Control 22
    rw MCTL22 @ 0x4c: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL22_INCH: 0..4 = enum MCTL22_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL22_EOS: 7 = struct MCTL22_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL22_VRSEL: 8..11 = enum MCTL22_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL22_DIF: 13 = struct MCTL22_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL22_WINC: 14 = struct MCTL22_WINC(bool);
    }
    /// ADC12 Memory Control 23
    rw MCTL23 @ 0x4e: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL23_INCH: 0..4 = enum MCTL23_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL23_EOS: 7 = struct MCTL23_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL23_VRSEL: 8..11 = enum MCTL23_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL23_DIF: 13 = struct MCTL23_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL23_WINC: 14 = struct MCTL23_WINC(bool);
    }
    /// ADC12 Memory Control 24
    rw MCTL24 @ 0x50: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL24_INCH: 0..4 = enum MCTL24_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL24_EOS: 7 = struct MCTL24_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL24_VRSEL: 8..11 = enum MCTL24_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL24_DIF: 13 = struct MCTL24_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL24_WINC: 14 = struct MCTL24_WINC(bool);
    }
    /// ADC12 Memory Control 25
    rw MCTL25 @ 0x52: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL25_INCH: 0..4 = enum MCTL25_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL25_EOS: 7 = struct MCTL25_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL25_VRSEL: 8..11 = enum MCTL25_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL25_DIF: 13 = struct MCTL25_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL25_WINC: 14 = struct MCTL25_WINC(bool);
    }
    /// ADC12 Memory Control 26
    rw MCTL26 @ 0x54: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL26_INCH: 0..4 = enum MCTL26_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL26_EOS: 7 = struct MCTL26_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL26_VRSEL: 8..11 = enum MCTL26_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL26_DIF: 13 = struct MCTL26_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL26_WINC: 14 = struct MCTL26_WINC(bool);
    }
    /// ADC12 Memory Control 27
    rw MCTL27 @ 0x56: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL27_INCH: 0..4 = enum MCTL27_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL27_EOS: 7 = struct MCTL27_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL27_VRSEL: 8..11 = enum MCTL27_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL27_DIF: 13 = struct MCTL27_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL27_WINC: 14 = struct MCTL27_WINC(bool);
    }
    /// ADC12 Memory Control 28
    rw MCTL28 @ 0x58: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL28_INCH: 0..4 = enum MCTL28_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL28_EOS: 7 = struct MCTL28_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL28_VRSEL: 8..11 = enum MCTL28_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL28_DIF: 13 = struct MCTL28_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL28_WINC: 14 = struct MCTL28_WINC(bool);
    }
    /// ADC12 Memory Control 29
    rw MCTL29 @ 0x5a: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL29_INCH: 0..4 = enum MCTL29_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL29_EOS: 7 = struct MCTL29_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL29_VRSEL: 8..11 = enum MCTL29_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL29_DIF: 13 = struct MCTL29_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL29_WINC: 14 = struct MCTL29_WINC(bool);
    }
    /// ADC12 Memory Control 30
    rw MCTL30 @ 0x5c: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL30_INCH: 0..4 = enum MCTL30_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL30_EOS: 7 = struct MCTL30_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL30_VRSEL: 8..11 = enum MCTL30_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL30_DIF: 13 = struct MCTL30_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL30_WINC: 14 = struct MCTL30_WINC(bool);
    }
    /// ADC12 Memory Control 31
    rw MCTL31 @ 0x5e: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        MCTL31_INCH: 0..4 = enum MCTL31_INCH {
            /// ADC12 Input Channel 0
            INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        MCTL31_EOS: 7 = struct MCTL31_EOS(bool);
        /// ADC12 VR Select Bit 0
        MCTL31_VRSEL: 8..11 = enum MCTL31_VRSEL {
            /// ADC12 Select Reference 0
            VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        MCTL31_DIF: 13 = struct MCTL31_DIF(bool);
        /// ADC12 Comparator window enable
        MCTL31_WINC: 14 = struct MCTL31_WINC(bool);
    }
    /// ADC12 Conversion Memory 0
    rw MEM0 @ 0x60: u16 = 0_0 {
        /// ADC12 Conversion Memory 0
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// ADC12 Conversion Memory 1
    rw MEM1 @ 0x62: u16 = 0_0 {
        /// ADC12 Conversion Memory 1
        MEM1: 0..15 = struct MEM1Field(u16);
    }
    /// ADC12 Conversion Memory 2
    rw MEM2 @ 0x64: u16 = 0_0 {
        /// ADC12 Conversion Memory 2
        MEM2: 0..15 = struct MEM2Field(u16);
    }
    /// ADC12 Conversion Memory 3
    rw MEM3 @ 0x66: u16 = 0_0 {
        /// ADC12 Conversion Memory 3
        MEM3: 0..15 = struct MEM3Field(u16);
    }
    /// ADC12 Conversion Memory 4
    rw MEM4 @ 0x68: u16 = 0_0 {
        /// ADC12 Conversion Memory 4
        MEM4: 0..15 = struct MEM4Field(u16);
    }
    /// ADC12 Conversion Memory 5
    rw MEM5 @ 0x6a: u16 = 0_0 {
        /// ADC12 Conversion Memory 5
        MEM5: 0..15 = struct MEM5Field(u16);
    }
    /// ADC12 Conversion Memory 6
    rw MEM6 @ 0x6c: u16 = 0_0 {
        /// ADC12 Conversion Memory 6
        MEM6: 0..15 = struct MEM6Field(u16);
    }
    /// ADC12 Conversion Memory 7
    rw MEM7 @ 0x6e: u16 = 0_0 {
        /// ADC12 Conversion Memory 7
        MEM7: 0..15 = struct MEM7Field(u16);
    }
    /// ADC12 Conversion Memory 8
    rw MEM8 @ 0x70: u16 = 0_0 {
        /// ADC12 Conversion Memory 8
        MEM8: 0..15 = struct MEM8Field(u16);
    }
    /// ADC12 Conversion Memory 9
    rw MEM9 @ 0x72: u16 = 0_0 {
        /// ADC12 Conversion Memory 9
        MEM9: 0..15 = struct MEM9Field(u16);
    }
    /// ADC12 Conversion Memory 10
    rw MEM10 @ 0x74: u16 = 0_0 {
        /// ADC12 Conversion Memory 10
        MEM10: 0..15 = struct MEM10Field(u16);
    }
    /// ADC12 Conversion Memory 11
    rw MEM11 @ 0x76: u16 = 0_0 {
        /// ADC12 Conversion Memory 11
        MEM11: 0..15 = struct MEM11Field(u16);
    }
    /// ADC12 Conversion Memory 12
    rw MEM12 @ 0x78: u16 = 0_0 {
        /// ADC12 Conversion Memory 12
        MEM12: 0..15 = struct MEM12Field(u16);
    }
    /// ADC12 Conversion Memory 13
    rw MEM13 @ 0x7a: u16 = 0_0 {
        /// ADC12 Conversion Memory 13
        MEM13: 0..15 = struct MEM13Field(u16);
    }
    /// ADC12 Conversion Memory 14
    rw MEM14 @ 0x7c: u16 = 0_0 {
        /// ADC12 Conversion Memory 14
        MEM14: 0..15 = struct MEM14Field(u16);
    }
    /// ADC12 Conversion Memory 15
    rw MEM15 @ 0x7e: u16 = 0_0 {
        /// ADC12 Conversion Memory 15
        MEM15: 0..15 = struct MEM15Field(u16);
    }
    /// ADC12 Conversion Memory 16
    rw MEM16 @ 0x80: u16 = 0_0 {
        /// ADC12 Conversion Memory 16
        MEM16: 0..15 = struct MEM16Field(u16);
    }
    /// ADC12 Conversion Memory 17
    rw MEM17 @ 0x82: u16 = 0_0 {
        /// ADC12 Conversion Memory 17
        MEM17: 0..15 = struct MEM17Field(u16);
    }
    /// ADC12 Conversion Memory 18
    rw MEM18 @ 0x84: u16 = 0_0 {
        /// ADC12 Conversion Memory 18
        MEM18: 0..15 = struct MEM18Field(u16);
    }
    /// ADC12 Conversion Memory 19
    rw MEM19 @ 0x86: u16 = 0_0 {
        /// ADC12 Conversion Memory 19
        MEM19: 0..15 = struct MEM19Field(u16);
    }
    /// ADC12 Conversion Memory 20
    rw MEM20 @ 0x88: u16 = 0_0 {
        /// ADC12 Conversion Memory 20
        MEM20: 0..15 = struct MEM20Field(u16);
    }
    /// ADC12 Conversion Memory 21
    rw MEM21 @ 0x8a: u16 = 0_0 {
        /// ADC12 Conversion Memory 21
        MEM21: 0..15 = struct MEM21Field(u16);
    }
    /// ADC12 Conversion Memory 22
    rw MEM22 @ 0x8c: u16 = 0_0 {
        /// ADC12 Conversion Memory 22
        MEM22: 0..15 = struct MEM22Field(u16);
    }
    /// ADC12 Conversion Memory 23
    rw MEM23 @ 0x8e: u16 = 0_0 {
        /// ADC12 Conversion Memory 23
        MEM23: 0..15 = struct MEM23Field(u16);
    }
    /// ADC12 Conversion Memory 24
    rw MEM24 @ 0x90: u16 = 0_0 {
        /// ADC12 Conversion Memory 24
        MEM24: 0..15 = struct MEM24Field(u16);
    }
    /// ADC12 Conversion Memory 25
    rw MEM25 @ 0x92: u16 = 0_0 {
        /// ADC12 Conversion Memory 25
        MEM25: 0..15 = struct MEM25Field(u16);
    }
    /// ADC12 Conversion Memory 26
    rw MEM26 @ 0x94: u16 = 0_0 {
        /// ADC12 Conversion Memory 26
        MEM26: 0..15 = struct MEM26Field(u16);
    }
    /// ADC12 Conversion Memory 27
    rw MEM27 @ 0x96: u16 = 0_0 {
        /// ADC12 Conversion Memory 27
        MEM27: 0..15 = struct MEM27Field(u16);
    }
    /// ADC12 Conversion Memory 28
    rw MEM28 @ 0x98: u16 = 0_0 {
        /// ADC12 Conversion Memory 28
        MEM28: 0..15 = struct MEM28Field(u16);
    }
    /// ADC12 Conversion Memory 29
    rw MEM29 @ 0x9a: u16 = 0_0 {
        /// ADC12 Conversion Memory 29
        MEM29: 0..15 = struct MEM29Field(u16);
    }
    /// ADC12 Conversion Memory 30
    rw MEM30 @ 0x9c: u16 = 0_0 {
        /// ADC12 Conversion Memory 30
        MEM30: 0..15 = struct MEM30Field(u16);
    }
    /// ADC12 Conversion Memory 31
    rw MEM31 @ 0x9e: u16 = 0_0 {
        /// ADC12 Conversion Memory 31
        MEM31: 0..15 = struct MEM31Field(u16);
    }
}
