//! ADC12

utils::periph! {
    /// ADC12
    ADC12;
    /// ADC12 B Control 0
    rw ADC12CTL0 @ 0x00: u16 = 0_0 {
        /// ADC12 Start Conversion
        ADC12SC: 0 = struct ADC12SC(bool);
        /// ADC12 Enable Conversion
        ADC12ENC: 1 = struct ADC12ENC(bool);
        /// ADC12 On/enable
        ADC12ON: 4 = struct ADC12ON(bool);
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
    /// ADC12 B Control 1
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
        ADC12SHS: 10..12 = enum ADC12SHS {
            /// ADC12 Sample/Hold Source: 0
            ADC12SHS_0 = 0b000,
            /// ADC12 Sample/Hold Source: 1
            ADC12SHS_1 = 0b001,
            /// ADC12 Sample/Hold Source: 2
            ADC12SHS_2 = 0b010,
            /// ADC12 Sample/Hold Source: 3
            ADC12SHS_3 = 0b011,
            /// ADC12 Sample/Hold Source: 4
            ADC12SHS_4 = 0b100,
            /// ADC12 Sample/Hold Source: 5
            ADC12SHS_5 = 0b101,
            /// ADC12 Sample/Hold Source: 6
            ADC12SHS_6 = 0b110,
            /// ADC12 Sample/Hold Source: 7
            ADC12SHS_7 = 0b111,
        }
        /// ADC12 Predivider Bit: 0
        ADC12PDIV: 13..14 = enum ADC12PDIV {
            /// ADC12 Clock predivider Select 0
            ADC12PDIV_0 = 0b00,
            /// ADC12 Clock predivider Select 1
            ADC12PDIV_1 = 0b01,
            /// ADC12 Clock predivider Select 2
            ADC12PDIV_2 = 0b10,
            /// ADC12 Clock predivider Select 3
            ADC12PDIV_3 = 0b11,
        }
    }
    /// ADC12 B Control 2
    rw ADC12CTL2 @ 0x04: u16 = 0_0 {
        /// ADC12 Power Mode
        ADC12PWRMD: 0 = struct ADC12PWRMD(bool);
        /// ADC12 Data Format
        ADC12DF: 3 = struct ADC12DF(bool);
        /// ADC12 Resolution Bit: 0
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
    }
    /// ADC12 B Control 3
    rw ADC12CTL3 @ 0x06: u16 = 0_0 {
        /// ADC12 Conversion Start Address Bit: 0
        ADC12CSTARTADD: 0..4 = enum ADC12CSTARTADD {
            /// ADC12 Conversion Start Address: 0
            ADC12CSTARTADD_0 = 0b00000,
            /// ADC12 Conversion Start Address: 1
            ADC12CSTARTADD_1 = 0b00001,
            /// ADC12 Conversion Start Address: 2
            ADC12CSTARTADD_2 = 0b00010,
            /// ADC12 Conversion Start Address: 3
            ADC12CSTARTADD_3 = 0b00011,
            /// ADC12 Conversion Start Address: 4
            ADC12CSTARTADD_4 = 0b00100,
            /// ADC12 Conversion Start Address: 5
            ADC12CSTARTADD_5 = 0b00101,
            /// ADC12 Conversion Start Address: 6
            ADC12CSTARTADD_6 = 0b00110,
            /// ADC12 Conversion Start Address: 7
            ADC12CSTARTADD_7 = 0b00111,
            /// ADC12 Conversion Start Address: 8
            ADC12CSTARTADD_8 = 0b01000,
            /// ADC12 Conversion Start Address: 9
            ADC12CSTARTADD_9 = 0b01001,
            /// ADC12 Conversion Start Address: 10
            ADC12CSTARTADD_10 = 0b01010,
            /// ADC12 Conversion Start Address: 11
            ADC12CSTARTADD_11 = 0b01011,
            /// ADC12 Conversion Start Address: 12
            ADC12CSTARTADD_12 = 0b01100,
            /// ADC12 Conversion Start Address: 13
            ADC12CSTARTADD_13 = 0b01101,
            /// ADC12 Conversion Start Address: 14
            ADC12CSTARTADD_14 = 0b01110,
            /// ADC12 Conversion Start Address: 15
            ADC12CSTARTADD_15 = 0b01111,
            /// ADC12 Conversion Start Address: 16
            ADC12CSTARTADD_16 = 0b10000,
            /// ADC12 Conversion Start Address: 17
            ADC12CSTARTADD_17 = 0b10001,
            /// ADC12 Conversion Start Address: 18
            ADC12CSTARTADD_18 = 0b10010,
            /// ADC12 Conversion Start Address: 19
            ADC12CSTARTADD_19 = 0b10011,
            /// ADC12 Conversion Start Address: 20
            ADC12CSTARTADD_20 = 0b10100,
            /// ADC12 Conversion Start Address: 21
            ADC12CSTARTADD_21 = 0b10101,
            /// ADC12 Conversion Start Address: 22
            ADC12CSTARTADD_22 = 0b10110,
            /// ADC12 Conversion Start Address: 23
            ADC12CSTARTADD_23 = 0b10111,
            /// ADC12 Conversion Start Address: 24
            ADC12CSTARTADD_24 = 0b11000,
            /// ADC12 Conversion Start Address: 25
            ADC12CSTARTADD_25 = 0b11001,
            /// ADC12 Conversion Start Address: 26
            ADC12CSTARTADD_26 = 0b11010,
            /// ADC12 Conversion Start Address: 27
            ADC12CSTARTADD_27 = 0b11011,
            /// ADC12 Conversion Start Address: 28
            ADC12CSTARTADD_28 = 0b11100,
            /// ADC12 Conversion Start Address: 29
            ADC12CSTARTADD_29 = 0b11101,
            /// ADC12 Conversion Start Address: 30
            ADC12CSTARTADD_30 = 0b11110,
            /// ADC12 Conversion Start Address: 31
            ADC12CSTARTADD_31 = 0b11111,
        }
        /// ADC12 Internal AVCC/2 select
        ADC12BATMAP: 6 = struct ADC12BATMAP(bool);
        /// ADC12 Internal TempSensor select
        ADC12TCMAP: 7 = struct ADC12TCMAP(bool);
        /// ADC12 Internal Channel 0 select
        ADC12ICH0MAP: 8 = struct ADC12ICH0MAP(bool);
        /// ADC12 Internal Channel 1 select
        ADC12ICH1MAP: 9 = struct ADC12ICH1MAP(bool);
        /// ADC12 Internal Channel 2 select
        ADC12ICH2MAP: 10 = struct ADC12ICH2MAP(bool);
        /// ADC12 Internal Channel 3 select
        ADC12ICH3MAP: 11 = struct ADC12ICH3MAP(bool);
    }
    /// ADC12 B Window Comparator High Threshold
    rw ADC12LO @ 0x08: u16 = 0_0 {
        /// ADC12 B Window Comparator High Threshold
        ADC12LO: 0..15 = struct ADC12LOField(u16);
    }
    /// ADC12 B Window Comparator High Threshold
    rw ADC12HI @ 0x0a: u16 = 0_0 {
        /// ADC12 B Window Comparator High Threshold
        ADC12HI: 0..15 = struct ADC12HIField(u16);
    }
    /// ADC12 B Interrupt Flag 0
    rw ADC12IFGR0 @ 0x0c: u16 = 0_0 {
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
    /// ADC12 B Interrupt Flag 1
    rw ADC12IFGR1 @ 0x0e: u16 = 0_0 {
        /// ADC12 Memory 16 Interrupt Flag
        ADC12IFG16: 0 = struct ADC12IFG16(bool);
        /// ADC12 Memory 17 Interrupt Flag
        ADC12IFG17: 1 = struct ADC12IFG17(bool);
        /// ADC12 Memory 18 Interrupt Flag
        ADC12IFG18: 2 = struct ADC12IFG18(bool);
        /// ADC12 Memory 19 Interrupt Flag
        ADC12IFG19: 3 = struct ADC12IFG19(bool);
        /// ADC12 Memory 20 Interrupt Flag
        ADC12IFG20: 4 = struct ADC12IFG20(bool);
        /// ADC12 Memory 21 Interrupt Flag
        ADC12IFG21: 5 = struct ADC12IFG21(bool);
        /// ADC12 Memory 22 Interrupt Flag
        ADC12IFG22: 6 = struct ADC12IFG22(bool);
        /// ADC12 Memory 23 Interrupt Flag
        ADC12IFG23: 7 = struct ADC12IFG23(bool);
        /// ADC12 Memory 24 Interrupt Flag
        ADC12IFG24: 8 = struct ADC12IFG24(bool);
        /// ADC12 Memory 25 Interrupt Flag
        ADC12IFG25: 9 = struct ADC12IFG25(bool);
        /// ADC12 Memory 26 Interrupt Flag
        ADC12IFG26: 10 = struct ADC12IFG26(bool);
        /// ADC12 Memory 27 Interrupt Flag
        ADC12IFG27: 11 = struct ADC12IFG27(bool);
        /// ADC12 Memory 28 Interrupt Flag
        ADC12IFG28: 12 = struct ADC12IFG28(bool);
        /// ADC12 Memory 29 Interrupt Flag
        ADC12IFG29: 13 = struct ADC12IFG29(bool);
        /// ADC12 Memory 30 Interrupt Flag
        ADC12IFG30: 14 = struct ADC12IFG30(bool);
        /// ADC12 Memory 31 Interrupt Flag
        ADC12IFG31: 15 = struct ADC12IFG31(bool);
    }
    /// ADC12 B Interrupt Flag 2
    rw ADC12IFGR2 @ 0x10: u16 = 0_0 {
        /// ADC12 Interrupt Flag for the inside of window of the Window comparator
        ADC12INIFG: 1 = struct ADC12INIFG(bool);
        /// ADC12 Interrupt Flag for lower threshold of the Window comparator
        ADC12LOIFG: 2 = struct ADC12LOIFG(bool);
        /// ADC12 Interrupt Flag for upper threshold of the Window comparator
        ADC12HIIFG: 3 = struct ADC12HIIFG(bool);
        /// ADC12 ADC12MEMx Overflow interrupt Flag
        ADC12OVIFG: 4 = struct ADC12OVIFG(bool);
        /// ADC12 Timer Overflow interrupt Flag
        ADC12TOVIFG: 5 = struct ADC12TOVIFG(bool);
        /// ADC12 local buffered reference ready interrupt Flag
        ADC12RDYIFG: 6 = struct ADC12RDYIFG(bool);
    }
    /// ADC12 B Interrupt Enable 0
    rw ADC12IER0 @ 0x12: u16 = 0_0 {
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
    /// ADC12 B Interrupt Enable 1
    rw ADC12IER1 @ 0x14: u16 = 0_0 {
        /// ADC12 Memory 16 Interrupt Enable
        ADC12IE16: 0 = struct ADC12IE16(bool);
        /// ADC12 Memory 17 Interrupt Enable
        ADC12IE17: 1 = struct ADC12IE17(bool);
        /// ADC12 Memory 18 Interrupt Enable
        ADC12IE18: 2 = struct ADC12IE18(bool);
        /// ADC12 Memory 19 Interrupt Enable
        ADC12IE19: 3 = struct ADC12IE19(bool);
        /// ADC12 Memory 20 Interrupt Enable
        ADC12IE20: 4 = struct ADC12IE20(bool);
        /// ADC12 Memory 21 Interrupt Enable
        ADC12IE21: 5 = struct ADC12IE21(bool);
        /// ADC12 Memory 22 Interrupt Enable
        ADC12IE22: 6 = struct ADC12IE22(bool);
        /// ADC12 Memory 23 Interrupt Enable
        ADC12IE23: 7 = struct ADC12IE23(bool);
        /// ADC12 Memory 24 Interrupt Enable
        ADC12IE24: 8 = struct ADC12IE24(bool);
        /// ADC12 Memory 25 Interrupt Enable
        ADC12IE25: 9 = struct ADC12IE25(bool);
        /// ADC12 Memory 26 Interrupt Enable
        ADC12IE26: 10 = struct ADC12IE26(bool);
        /// ADC12 Memory 27 Interrupt Enable
        ADC12IE27: 11 = struct ADC12IE27(bool);
        /// ADC12 Memory 28 Interrupt Enable
        ADC12IE28: 12 = struct ADC12IE28(bool);
        /// ADC12 Memory 29 Interrupt Enable
        ADC12IE29: 13 = struct ADC12IE29(bool);
        /// ADC12 Memory 30 Interrupt Enable
        ADC12IE30: 14 = struct ADC12IE30(bool);
        /// ADC12 Memory 31 Interrupt Enable
        ADC12IE31: 15 = struct ADC12IE31(bool);
    }
    /// ADC12 B Interrupt Enable 2
    rw ADC12IER2 @ 0x16: u16 = 0_0 {
        /// ADC12 Interrupt enable for the inside of window of the Window comparator
        ADC12INIE: 1 = struct ADC12INIE(bool);
        /// ADC12 Interrupt enable for lower threshold of the Window comparator
        ADC12LOIE: 2 = struct ADC12LOIE(bool);
        /// ADC12 Interrupt enable for upper threshold of the Window comparator
        ADC12HIIE: 3 = struct ADC12HIIE(bool);
        /// ADC12 ADC12MEMx Overflow interrupt enable
        ADC12OVIE: 4 = struct ADC12OVIE(bool);
        /// ADC12 Timer Overflow interrupt enable
        ADC12TOVIE: 5 = struct ADC12TOVIE(bool);
        /// ADC12 local buffered reference ready interrupt enable
        ADC12RDYIE: 6 = struct ADC12RDYIE(bool);
    }
    /// ADC12 B Interrupt Vector Word
    rw ADC12IV @ 0x18: u16 = 0_0 {
        /// ADC12 B Interrupt Vector Word
        ADC12IV: 0..15 = struct ADC12IVField(u16);
    }
    /// ADC12 Memory Control 0
    rw ADC12MCTL0 @ 0x20: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL0_ADC12INCH: 0..4 = enum ADC12MCTL0_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL0_ADC12EOS: 7 = struct ADC12MCTL0_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL0_ADC12VRSEL: 8..11 = enum ADC12MCTL0_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL0_ADC12DIF: 13 = struct ADC12MCTL0_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL0_ADC12WINC: 14 = struct ADC12MCTL0_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 1
    rw ADC12MCTL1 @ 0x22: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL1_ADC12INCH: 0..4 = enum ADC12MCTL1_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL1_ADC12EOS: 7 = struct ADC12MCTL1_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL1_ADC12VRSEL: 8..11 = enum ADC12MCTL1_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL1_ADC12DIF: 13 = struct ADC12MCTL1_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL1_ADC12WINC: 14 = struct ADC12MCTL1_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 2
    rw ADC12MCTL2 @ 0x24: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL2_ADC12INCH: 0..4 = enum ADC12MCTL2_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL2_ADC12EOS: 7 = struct ADC12MCTL2_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL2_ADC12VRSEL: 8..11 = enum ADC12MCTL2_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL2_ADC12DIF: 13 = struct ADC12MCTL2_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL2_ADC12WINC: 14 = struct ADC12MCTL2_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 3
    rw ADC12MCTL3 @ 0x26: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL3_ADC12INCH: 0..4 = enum ADC12MCTL3_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL3_ADC12EOS: 7 = struct ADC12MCTL3_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL3_ADC12VRSEL: 8..11 = enum ADC12MCTL3_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL3_ADC12DIF: 13 = struct ADC12MCTL3_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL3_ADC12WINC: 14 = struct ADC12MCTL3_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 4
    rw ADC12MCTL4 @ 0x28: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL4_ADC12INCH: 0..4 = enum ADC12MCTL4_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL4_ADC12EOS: 7 = struct ADC12MCTL4_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL4_ADC12VRSEL: 8..11 = enum ADC12MCTL4_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL4_ADC12DIF: 13 = struct ADC12MCTL4_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL4_ADC12WINC: 14 = struct ADC12MCTL4_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 5
    rw ADC12MCTL5 @ 0x2a: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL5_ADC12INCH: 0..4 = enum ADC12MCTL5_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL5_ADC12EOS: 7 = struct ADC12MCTL5_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL5_ADC12VRSEL: 8..11 = enum ADC12MCTL5_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL5_ADC12DIF: 13 = struct ADC12MCTL5_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL5_ADC12WINC: 14 = struct ADC12MCTL5_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 6
    rw ADC12MCTL6 @ 0x2c: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL6_ADC12INCH: 0..4 = enum ADC12MCTL6_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL6_ADC12EOS: 7 = struct ADC12MCTL6_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL6_ADC12VRSEL: 8..11 = enum ADC12MCTL6_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL6_ADC12DIF: 13 = struct ADC12MCTL6_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL6_ADC12WINC: 14 = struct ADC12MCTL6_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 7
    rw ADC12MCTL7 @ 0x2e: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL7_ADC12INCH: 0..4 = enum ADC12MCTL7_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL7_ADC12EOS: 7 = struct ADC12MCTL7_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL7_ADC12VRSEL: 8..11 = enum ADC12MCTL7_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL7_ADC12DIF: 13 = struct ADC12MCTL7_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL7_ADC12WINC: 14 = struct ADC12MCTL7_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 8
    rw ADC12MCTL8 @ 0x30: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL8_ADC12INCH: 0..4 = enum ADC12MCTL8_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL8_ADC12EOS: 7 = struct ADC12MCTL8_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL8_ADC12VRSEL: 8..11 = enum ADC12MCTL8_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL8_ADC12DIF: 13 = struct ADC12MCTL8_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL8_ADC12WINC: 14 = struct ADC12MCTL8_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 9
    rw ADC12MCTL9 @ 0x32: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL9_ADC12INCH: 0..4 = enum ADC12MCTL9_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL9_ADC12EOS: 7 = struct ADC12MCTL9_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL9_ADC12VRSEL: 8..11 = enum ADC12MCTL9_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL9_ADC12DIF: 13 = struct ADC12MCTL9_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL9_ADC12WINC: 14 = struct ADC12MCTL9_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 10
    rw ADC12MCTL10 @ 0x34: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL10_ADC12INCH: 0..4 = enum ADC12MCTL10_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL10_ADC12EOS: 7 = struct ADC12MCTL10_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL10_ADC12VRSEL: 8..11 = enum ADC12MCTL10_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL10_ADC12DIF: 13 = struct ADC12MCTL10_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL10_ADC12WINC: 14 = struct ADC12MCTL10_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 11
    rw ADC12MCTL11 @ 0x36: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL11_ADC12INCH: 0..4 = enum ADC12MCTL11_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL11_ADC12EOS: 7 = struct ADC12MCTL11_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL11_ADC12VRSEL: 8..11 = enum ADC12MCTL11_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL11_ADC12DIF: 13 = struct ADC12MCTL11_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL11_ADC12WINC: 14 = struct ADC12MCTL11_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 12
    rw ADC12MCTL12 @ 0x38: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL12_ADC12INCH: 0..4 = enum ADC12MCTL12_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL12_ADC12EOS: 7 = struct ADC12MCTL12_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL12_ADC12VRSEL: 8..11 = enum ADC12MCTL12_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL12_ADC12DIF: 13 = struct ADC12MCTL12_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL12_ADC12WINC: 14 = struct ADC12MCTL12_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 13
    rw ADC12MCTL13 @ 0x3a: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL13_ADC12INCH: 0..4 = enum ADC12MCTL13_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL13_ADC12EOS: 7 = struct ADC12MCTL13_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL13_ADC12VRSEL: 8..11 = enum ADC12MCTL13_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL13_ADC12DIF: 13 = struct ADC12MCTL13_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL13_ADC12WINC: 14 = struct ADC12MCTL13_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 14
    rw ADC12MCTL14 @ 0x3c: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL14_ADC12INCH: 0..4 = enum ADC12MCTL14_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL14_ADC12EOS: 7 = struct ADC12MCTL14_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL14_ADC12VRSEL: 8..11 = enum ADC12MCTL14_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL14_ADC12DIF: 13 = struct ADC12MCTL14_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL14_ADC12WINC: 14 = struct ADC12MCTL14_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 15
    rw ADC12MCTL15 @ 0x3e: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL15_ADC12INCH: 0..4 = enum ADC12MCTL15_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL15_ADC12EOS: 7 = struct ADC12MCTL15_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL15_ADC12VRSEL: 8..11 = enum ADC12MCTL15_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL15_ADC12DIF: 13 = struct ADC12MCTL15_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL15_ADC12WINC: 14 = struct ADC12MCTL15_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 16
    rw ADC12MCTL16 @ 0x40: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL16_ADC12INCH: 0..4 = enum ADC12MCTL16_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL16_ADC12EOS: 7 = struct ADC12MCTL16_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL16_ADC12VRSEL: 8..11 = enum ADC12MCTL16_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL16_ADC12DIF: 13 = struct ADC12MCTL16_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL16_ADC12WINC: 14 = struct ADC12MCTL16_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 17
    rw ADC12MCTL17 @ 0x42: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL17_ADC12INCH: 0..4 = enum ADC12MCTL17_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL17_ADC12EOS: 7 = struct ADC12MCTL17_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL17_ADC12VRSEL: 8..11 = enum ADC12MCTL17_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL17_ADC12DIF: 13 = struct ADC12MCTL17_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL17_ADC12WINC: 14 = struct ADC12MCTL17_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 18
    rw ADC12MCTL18 @ 0x44: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL18_ADC12INCH: 0..4 = enum ADC12MCTL18_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL18_ADC12EOS: 7 = struct ADC12MCTL18_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL18_ADC12VRSEL: 8..11 = enum ADC12MCTL18_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL18_ADC12DIF: 13 = struct ADC12MCTL18_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL18_ADC12WINC: 14 = struct ADC12MCTL18_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 19
    rw ADC12MCTL19 @ 0x46: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL19_ADC12INCH: 0..4 = enum ADC12MCTL19_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL19_ADC12EOS: 7 = struct ADC12MCTL19_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL19_ADC12VRSEL: 8..11 = enum ADC12MCTL19_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL19_ADC12DIF: 13 = struct ADC12MCTL19_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL19_ADC12WINC: 14 = struct ADC12MCTL19_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 20
    rw ADC12MCTL20 @ 0x48: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL20_ADC12INCH: 0..4 = enum ADC12MCTL20_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL20_ADC12EOS: 7 = struct ADC12MCTL20_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL20_ADC12VRSEL: 8..11 = enum ADC12MCTL20_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL20_ADC12DIF: 13 = struct ADC12MCTL20_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL20_ADC12WINC: 14 = struct ADC12MCTL20_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 21
    rw ADC12MCTL21 @ 0x4a: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL21_ADC12INCH: 0..4 = enum ADC12MCTL21_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL21_ADC12EOS: 7 = struct ADC12MCTL21_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL21_ADC12VRSEL: 8..11 = enum ADC12MCTL21_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL21_ADC12DIF: 13 = struct ADC12MCTL21_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL21_ADC12WINC: 14 = struct ADC12MCTL21_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 22
    rw ADC12MCTL22 @ 0x4c: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL22_ADC12INCH: 0..4 = enum ADC12MCTL22_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL22_ADC12EOS: 7 = struct ADC12MCTL22_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL22_ADC12VRSEL: 8..11 = enum ADC12MCTL22_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL22_ADC12DIF: 13 = struct ADC12MCTL22_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL22_ADC12WINC: 14 = struct ADC12MCTL22_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 23
    rw ADC12MCTL23 @ 0x4e: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL23_ADC12INCH: 0..4 = enum ADC12MCTL23_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL23_ADC12EOS: 7 = struct ADC12MCTL23_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL23_ADC12VRSEL: 8..11 = enum ADC12MCTL23_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL23_ADC12DIF: 13 = struct ADC12MCTL23_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL23_ADC12WINC: 14 = struct ADC12MCTL23_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 24
    rw ADC12MCTL24 @ 0x50: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL24_ADC12INCH: 0..4 = enum ADC12MCTL24_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL24_ADC12EOS: 7 = struct ADC12MCTL24_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL24_ADC12VRSEL: 8..11 = enum ADC12MCTL24_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL24_ADC12DIF: 13 = struct ADC12MCTL24_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL24_ADC12WINC: 14 = struct ADC12MCTL24_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 25
    rw ADC12MCTL25 @ 0x52: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL25_ADC12INCH: 0..4 = enum ADC12MCTL25_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL25_ADC12EOS: 7 = struct ADC12MCTL25_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL25_ADC12VRSEL: 8..11 = enum ADC12MCTL25_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL25_ADC12DIF: 13 = struct ADC12MCTL25_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL25_ADC12WINC: 14 = struct ADC12MCTL25_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 26
    rw ADC12MCTL26 @ 0x54: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL26_ADC12INCH: 0..4 = enum ADC12MCTL26_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL26_ADC12EOS: 7 = struct ADC12MCTL26_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL26_ADC12VRSEL: 8..11 = enum ADC12MCTL26_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL26_ADC12DIF: 13 = struct ADC12MCTL26_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL26_ADC12WINC: 14 = struct ADC12MCTL26_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 27
    rw ADC12MCTL27 @ 0x56: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL27_ADC12INCH: 0..4 = enum ADC12MCTL27_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL27_ADC12EOS: 7 = struct ADC12MCTL27_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL27_ADC12VRSEL: 8..11 = enum ADC12MCTL27_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL27_ADC12DIF: 13 = struct ADC12MCTL27_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL27_ADC12WINC: 14 = struct ADC12MCTL27_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 28
    rw ADC12MCTL28 @ 0x58: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL28_ADC12INCH: 0..4 = enum ADC12MCTL28_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL28_ADC12EOS: 7 = struct ADC12MCTL28_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL28_ADC12VRSEL: 8..11 = enum ADC12MCTL28_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL28_ADC12DIF: 13 = struct ADC12MCTL28_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL28_ADC12WINC: 14 = struct ADC12MCTL28_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 29
    rw ADC12MCTL29 @ 0x5a: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL29_ADC12INCH: 0..4 = enum ADC12MCTL29_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL29_ADC12EOS: 7 = struct ADC12MCTL29_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL29_ADC12VRSEL: 8..11 = enum ADC12MCTL29_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL29_ADC12DIF: 13 = struct ADC12MCTL29_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL29_ADC12WINC: 14 = struct ADC12MCTL29_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 30
    rw ADC12MCTL30 @ 0x5c: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL30_ADC12INCH: 0..4 = enum ADC12MCTL30_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL30_ADC12EOS: 7 = struct ADC12MCTL30_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL30_ADC12VRSEL: 8..11 = enum ADC12MCTL30_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL30_ADC12DIF: 13 = struct ADC12MCTL30_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL30_ADC12WINC: 14 = struct ADC12MCTL30_ADC12WINC(bool);
    }
    /// ADC12 Memory Control 31
    rw ADC12MCTL31 @ 0x5e: u16 = 0_0 {
        /// ADC12 Input Channel Select Bit 0
        ADC12MCTL31_ADC12INCH: 0..4 = enum ADC12MCTL31_ADC12INCH {
            /// ADC12 Input Channel 0
            ADC12INCH_0 = 0b00000,
            /// ADC12 Input Channel 1
            ADC12INCH_1 = 0b00001,
            /// ADC12 Input Channel 2
            ADC12INCH_2 = 0b00010,
            /// ADC12 Input Channel 3
            ADC12INCH_3 = 0b00011,
            /// ADC12 Input Channel 4
            ADC12INCH_4 = 0b00100,
            /// ADC12 Input Channel 5
            ADC12INCH_5 = 0b00101,
            /// ADC12 Input Channel 6
            ADC12INCH_6 = 0b00110,
            /// ADC12 Input Channel 7
            ADC12INCH_7 = 0b00111,
            /// ADC12 Input Channel 8
            ADC12INCH_8 = 0b01000,
            /// ADC12 Input Channel 9
            ADC12INCH_9 = 0b01001,
            /// ADC12 Input Channel 10
            ADC12INCH_10 = 0b01010,
            /// ADC12 Input Channel 11
            ADC12INCH_11 = 0b01011,
            /// ADC12 Input Channel 12
            ADC12INCH_12 = 0b01100,
            /// ADC12 Input Channel 13
            ADC12INCH_13 = 0b01101,
            /// ADC12 Input Channel 14
            ADC12INCH_14 = 0b01110,
            /// ADC12 Input Channel 15
            ADC12INCH_15 = 0b01111,
            /// ADC12 Input Channel 16
            ADC12INCH_16 = 0b10000,
            /// ADC12 Input Channel 17
            ADC12INCH_17 = 0b10001,
            /// ADC12 Input Channel 18
            ADC12INCH_18 = 0b10010,
            /// ADC12 Input Channel 19
            ADC12INCH_19 = 0b10011,
            /// ADC12 Input Channel 20
            ADC12INCH_20 = 0b10100,
            /// ADC12 Input Channel 21
            ADC12INCH_21 = 0b10101,
            /// ADC12 Input Channel 22
            ADC12INCH_22 = 0b10110,
            /// ADC12 Input Channel 23
            ADC12INCH_23 = 0b10111,
            /// ADC12 Input Channel 24
            ADC12INCH_24 = 0b11000,
            /// ADC12 Input Channel 25
            ADC12INCH_25 = 0b11001,
            /// ADC12 Input Channel 26
            ADC12INCH_26 = 0b11010,
            /// ADC12 Input Channel 27
            ADC12INCH_27 = 0b11011,
            /// ADC12 Input Channel 28
            ADC12INCH_28 = 0b11100,
            /// ADC12 Input Channel 29
            ADC12INCH_29 = 0b11101,
            /// ADC12 Input Channel 30
            ADC12INCH_30 = 0b11110,
            /// ADC12 Input Channel 31
            ADC12INCH_31 = 0b11111,
        }
        /// ADC12 End of Sequence
        ADC12MCTL31_ADC12EOS: 7 = struct ADC12MCTL31_ADC12EOS(bool);
        /// ADC12 VR Select Bit 0
        ADC12MCTL31_ADC12VRSEL: 8..11 = enum ADC12MCTL31_ADC12VRSEL {
            /// ADC12 Select Reference 0
            ADC12VRSEL_0 = 0b0000,
            /// ADC12 Select Reference 1
            ADC12VRSEL_1 = 0b0001,
            /// ADC12 Select Reference 2
            ADC12VRSEL_2 = 0b0010,
            /// ADC12 Select Reference 3
            ADC12VRSEL_3 = 0b0011,
            /// ADC12 Select Reference 4
            ADC12VRSEL_4 = 0b0100,
            /// ADC12 Select Reference 5
            ADC12VRSEL_5 = 0b0101,
            /// ADC12 Select Reference 6
            ADC12VRSEL_6 = 0b0110,
            /// ADC12 Select Reference 7
            ADC12VRSEL_7 = 0b0111,
            /// ADC12 Select Reference 8
            ADC12VRSEL_8 = 0b1000,
            /// ADC12 Select Reference 9
            ADC12VRSEL_9 = 0b1001,
            /// ADC12 Select Reference 10
            ADC12VRSEL_10 = 0b1010,
            /// ADC12 Select Reference 11
            ADC12VRSEL_11 = 0b1011,
            /// ADC12 Select Reference 12
            ADC12VRSEL_12 = 0b1100,
            /// ADC12 Select Reference 13
            ADC12VRSEL_13 = 0b1101,
            /// ADC12 Select Reference 14
            ADC12VRSEL_14 = 0b1110,
            /// ADC12 Select Reference 15
            ADC12VRSEL_15 = 0b1111,
        }
        /// ADC12 Differential mode (only for even Registers)
        ADC12MCTL31_ADC12DIF: 13 = struct ADC12MCTL31_ADC12DIF(bool);
        /// ADC12 Comparator window enable
        ADC12MCTL31_ADC12WINC: 14 = struct ADC12MCTL31_ADC12WINC(bool);
    }
    /// ADC12 Conversion Memory 0
    rw ADC12MEM0 @ 0x60: u16 = 0_0 {
        /// ADC12 Conversion Memory 0
        ADC12MEM0: 0..15 = struct ADC12MEM0Field(u16);
    }
    /// ADC12 Conversion Memory 1
    rw ADC12MEM1 @ 0x62: u16 = 0_0 {
        /// ADC12 Conversion Memory 1
        ADC12MEM1: 0..15 = struct ADC12MEM1Field(u16);
    }
    /// ADC12 Conversion Memory 2
    rw ADC12MEM2 @ 0x64: u16 = 0_0 {
        /// ADC12 Conversion Memory 2
        ADC12MEM2: 0..15 = struct ADC12MEM2Field(u16);
    }
    /// ADC12 Conversion Memory 3
    rw ADC12MEM3 @ 0x66: u16 = 0_0 {
        /// ADC12 Conversion Memory 3
        ADC12MEM3: 0..15 = struct ADC12MEM3Field(u16);
    }
    /// ADC12 Conversion Memory 4
    rw ADC12MEM4 @ 0x68: u16 = 0_0 {
        /// ADC12 Conversion Memory 4
        ADC12MEM4: 0..15 = struct ADC12MEM4Field(u16);
    }
    /// ADC12 Conversion Memory 5
    rw ADC12MEM5 @ 0x6a: u16 = 0_0 {
        /// ADC12 Conversion Memory 5
        ADC12MEM5: 0..15 = struct ADC12MEM5Field(u16);
    }
    /// ADC12 Conversion Memory 6
    rw ADC12MEM6 @ 0x6c: u16 = 0_0 {
        /// ADC12 Conversion Memory 6
        ADC12MEM6: 0..15 = struct ADC12MEM6Field(u16);
    }
    /// ADC12 Conversion Memory 7
    rw ADC12MEM7 @ 0x6e: u16 = 0_0 {
        /// ADC12 Conversion Memory 7
        ADC12MEM7: 0..15 = struct ADC12MEM7Field(u16);
    }
    /// ADC12 Conversion Memory 8
    rw ADC12MEM8 @ 0x70: u16 = 0_0 {
        /// ADC12 Conversion Memory 8
        ADC12MEM8: 0..15 = struct ADC12MEM8Field(u16);
    }
    /// ADC12 Conversion Memory 9
    rw ADC12MEM9 @ 0x72: u16 = 0_0 {
        /// ADC12 Conversion Memory 9
        ADC12MEM9: 0..15 = struct ADC12MEM9Field(u16);
    }
    /// ADC12 Conversion Memory 10
    rw ADC12MEM10 @ 0x74: u16 = 0_0 {
        /// ADC12 Conversion Memory 10
        ADC12MEM10: 0..15 = struct ADC12MEM10Field(u16);
    }
    /// ADC12 Conversion Memory 11
    rw ADC12MEM11 @ 0x76: u16 = 0_0 {
        /// ADC12 Conversion Memory 11
        ADC12MEM11: 0..15 = struct ADC12MEM11Field(u16);
    }
    /// ADC12 Conversion Memory 12
    rw ADC12MEM12 @ 0x78: u16 = 0_0 {
        /// ADC12 Conversion Memory 12
        ADC12MEM12: 0..15 = struct ADC12MEM12Field(u16);
    }
    /// ADC12 Conversion Memory 13
    rw ADC12MEM13 @ 0x7a: u16 = 0_0 {
        /// ADC12 Conversion Memory 13
        ADC12MEM13: 0..15 = struct ADC12MEM13Field(u16);
    }
    /// ADC12 Conversion Memory 14
    rw ADC12MEM14 @ 0x7c: u16 = 0_0 {
        /// ADC12 Conversion Memory 14
        ADC12MEM14: 0..15 = struct ADC12MEM14Field(u16);
    }
    /// ADC12 Conversion Memory 15
    rw ADC12MEM15 @ 0x7e: u16 = 0_0 {
        /// ADC12 Conversion Memory 15
        ADC12MEM15: 0..15 = struct ADC12MEM15Field(u16);
    }
    /// ADC12 Conversion Memory 16
    rw ADC12MEM16 @ 0x80: u16 = 0_0 {
        /// ADC12 Conversion Memory 16
        ADC12MEM16: 0..15 = struct ADC12MEM16Field(u16);
    }
    /// ADC12 Conversion Memory 17
    rw ADC12MEM17 @ 0x82: u16 = 0_0 {
        /// ADC12 Conversion Memory 17
        ADC12MEM17: 0..15 = struct ADC12MEM17Field(u16);
    }
    /// ADC12 Conversion Memory 18
    rw ADC12MEM18 @ 0x84: u16 = 0_0 {
        /// ADC12 Conversion Memory 18
        ADC12MEM18: 0..15 = struct ADC12MEM18Field(u16);
    }
    /// ADC12 Conversion Memory 19
    rw ADC12MEM19 @ 0x86: u16 = 0_0 {
        /// ADC12 Conversion Memory 19
        ADC12MEM19: 0..15 = struct ADC12MEM19Field(u16);
    }
    /// ADC12 Conversion Memory 20
    rw ADC12MEM20 @ 0x88: u16 = 0_0 {
        /// ADC12 Conversion Memory 20
        ADC12MEM20: 0..15 = struct ADC12MEM20Field(u16);
    }
    /// ADC12 Conversion Memory 21
    rw ADC12MEM21 @ 0x8a: u16 = 0_0 {
        /// ADC12 Conversion Memory 21
        ADC12MEM21: 0..15 = struct ADC12MEM21Field(u16);
    }
    /// ADC12 Conversion Memory 22
    rw ADC12MEM22 @ 0x8c: u16 = 0_0 {
        /// ADC12 Conversion Memory 22
        ADC12MEM22: 0..15 = struct ADC12MEM22Field(u16);
    }
    /// ADC12 Conversion Memory 23
    rw ADC12MEM23 @ 0x8e: u16 = 0_0 {
        /// ADC12 Conversion Memory 23
        ADC12MEM23: 0..15 = struct ADC12MEM23Field(u16);
    }
    /// ADC12 Conversion Memory 24
    rw ADC12MEM24 @ 0x90: u16 = 0_0 {
        /// ADC12 Conversion Memory 24
        ADC12MEM24: 0..15 = struct ADC12MEM24Field(u16);
    }
    /// ADC12 Conversion Memory 25
    rw ADC12MEM25 @ 0x92: u16 = 0_0 {
        /// ADC12 Conversion Memory 25
        ADC12MEM25: 0..15 = struct ADC12MEM25Field(u16);
    }
    /// ADC12 Conversion Memory 26
    rw ADC12MEM26 @ 0x94: u16 = 0_0 {
        /// ADC12 Conversion Memory 26
        ADC12MEM26: 0..15 = struct ADC12MEM26Field(u16);
    }
    /// ADC12 Conversion Memory 27
    rw ADC12MEM27 @ 0x96: u16 = 0_0 {
        /// ADC12 Conversion Memory 27
        ADC12MEM27: 0..15 = struct ADC12MEM27Field(u16);
    }
    /// ADC12 Conversion Memory 28
    rw ADC12MEM28 @ 0x98: u16 = 0_0 {
        /// ADC12 Conversion Memory 28
        ADC12MEM28: 0..15 = struct ADC12MEM28Field(u16);
    }
    /// ADC12 Conversion Memory 29
    rw ADC12MEM29 @ 0x9a: u16 = 0_0 {
        /// ADC12 Conversion Memory 29
        ADC12MEM29: 0..15 = struct ADC12MEM29Field(u16);
    }
    /// ADC12 Conversion Memory 30
    rw ADC12MEM30 @ 0x9c: u16 = 0_0 {
        /// ADC12 Conversion Memory 30
        ADC12MEM30: 0..15 = struct ADC12MEM30Field(u16);
    }
    /// ADC12 Conversion Memory 31
    rw ADC12MEM31 @ 0x9e: u16 = 0_0 {
        /// ADC12 Conversion Memory 31
        ADC12MEM31: 0..15 = struct ADC12MEM31Field(u16);
    }
}
