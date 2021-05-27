//! Comparator D

utils::periph! {
    /// Comparator D
    ComparatorD;
    /// Comparator D Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// Comp. D Pos. Channel Input Select 0
        IPSEL: 0..3 = enum IPSEL {
            /// Comp. D V+ terminal Input Select: Channel 0
            IPSEL_0 = 0b0000,
            /// Comp. D V+ terminal Input Select: Channel 1
            IPSEL_1 = 0b0001,
            /// Comp. D V+ terminal Input Select: Channel 2
            IPSEL_2 = 0b0010,
            /// Comp. D V+ terminal Input Select: Channel 3
            IPSEL_3 = 0b0011,
            /// Comp. D V+ terminal Input Select: Channel 4
            IPSEL_4 = 0b0100,
            /// Comp. D V+ terminal Input Select: Channel 5
            IPSEL_5 = 0b0101,
            /// Comp. D V+ terminal Input Select: Channel 6
            IPSEL_6 = 0b0110,
            /// Comp. D V+ terminal Input Select: Channel 7
            IPSEL_7 = 0b0111,
            /// Comp. D V+ terminal Input Select: Channel 8
            IPSEL_8 = 0b1000,
            /// Comp. D V+ terminal Input Select: Channel 9
            IPSEL_9 = 0b1001,
            /// Comp. D V+ terminal Input Select: Channel 10
            IPSEL_10 = 0b1010,
            /// Comp. D V+ terminal Input Select: Channel 11
            IPSEL_11 = 0b1011,
            /// Comp. D V+ terminal Input Select: Channel 12
            IPSEL_12 = 0b1100,
            /// Comp. D V+ terminal Input Select: Channel 13
            IPSEL_13 = 0b1101,
            /// Comp. D V+ terminal Input Select: Channel 14
            IPSEL_14 = 0b1110,
            /// Comp. D V+ terminal Input Select: Channel 15
            IPSEL_15 = 0b1111,
        }
        /// Comp. D Pos. Channel Input Enable
        IPEN: 7 = struct IPEN(bool);
        /// Comp. D Neg. Channel Input Select 0
        IMSEL: 8..11 = enum IMSEL {
            /// Comp. D V- Terminal Input Select: Channel 0
            IMSEL_0 = 0b0000,
            /// Comp. D V- Terminal Input Select: Channel 1
            IMSEL_1 = 0b0001,
            /// Comp. D V- Terminal Input Select: Channel 2
            IMSEL_2 = 0b0010,
            /// Comp. D V- Terminal Input Select: Channel 3
            IMSEL_3 = 0b0011,
            /// Comp. D V- Terminal Input Select: Channel 4
            IMSEL_4 = 0b0100,
            /// Comp. D V- Terminal Input Select: Channel 5
            IMSEL_5 = 0b0101,
            /// Comp. D V- Terminal Input Select: Channel 6
            IMSEL_6 = 0b0110,
            /// Comp. D V- Terminal Input Select: Channel 7
            IMSEL_7 = 0b0111,
            /// Comp. D V- terminal Input Select: Channel 8
            IMSEL_8 = 0b1000,
            /// Comp. D V- terminal Input Select: Channel 9
            IMSEL_9 = 0b1001,
            /// Comp. D V- terminal Input Select: Channel 10
            IMSEL_10 = 0b1010,
            /// Comp. D V- terminal Input Select: Channel 11
            IMSEL_11 = 0b1011,
            /// Comp. D V- terminal Input Select: Channel 12
            IMSEL_12 = 0b1100,
            /// Comp. D V- terminal Input Select: Channel 13
            IMSEL_13 = 0b1101,
            /// Comp. D V- terminal Input Select: Channel 14
            IMSEL_14 = 0b1110,
            /// Comp. D V- terminal Input Select: Channel 15
            IMSEL_15 = 0b1111,
        }
        /// Comp. D Neg. Channel Input Enable
        IMEN: 15 = struct IMEN(bool);
    }
    /// Comparator D Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// Comp. D Output
        OUT: 0 = struct OUT(bool);
        /// Comp. D Output Polarity
        OUTPOL: 1 = struct OUTPOL(bool);
        /// Comp. D Enable Output Filter
        F: 2 = struct F(bool);
        /// Comp. D Interrupt Edge Select
        IES: 3 = struct IES(bool);
        /// Comp. D Input Short
        SHORT: 4 = struct SHORT(bool);
        /// Comp. D Exchange Inputs
        EX: 5 = struct EX(bool);
        /// Comp. D Filter delay Bit 0
        FDLY: 6..7 = enum FDLY {
            /// Comp. D Filter delay 0 : 450ns
            FDLY_0 = 0b00,
            /// Comp. D Filter delay 1 : 900ns
            FDLY_1 = 0b01,
            /// Comp. D Filter delay 2 : 1800ns
            FDLY_2 = 0b10,
            /// Comp. D Filter delay 3 : 3600ns
            FDLY_3 = 0b11,
        }
        /// Comp. D enable
        ON: 10 = struct ON(bool);
        /// Comp. D CDMRV Level
        MRVL: 11 = struct MRVL(bool);
        /// Comp. D Output selects between VREF0 or VREF1
        MRVS: 12 = struct MRVS(bool);
    }
    /// Comparator D Control Register 2
    rw CTL2 @ 0x04: u16 = 0_0 {
        /// Comp. D Reference 0 Resistor Select Bit : 0
        REF0: 0..4 = enum REF0 {
            /// Comp. D Int. Ref.0 Select 0 : 1/32
            REF0_0 = 0b00000,
            /// Comp. D Int. Ref.0 Select 1 : 2/32
            REF0_1 = 0b00001,
            /// Comp. D Int. Ref.0 Select 2 : 3/32
            REF0_2 = 0b00010,
            /// Comp. D Int. Ref.0 Select 3 : 4/32
            REF0_3 = 0b00011,
            /// Comp. D Int. Ref.0 Select 4 : 5/32
            REF0_4 = 0b00100,
            /// Comp. D Int. Ref.0 Select 5 : 6/32
            REF0_5 = 0b00101,
            /// Comp. D Int. Ref.0 Select 6 : 7/32
            REF0_6 = 0b00110,
            /// Comp. D Int. Ref.0 Select 7 : 8/32
            REF0_7 = 0b00111,
            /// Comp. D Int. Ref.0 Select 0 : 9/32
            REF0_8 = 0b01000,
            /// Comp. D Int. Ref.0 Select 1 : 10/32
            REF0_9 = 0b01001,
            /// Comp. D Int. Ref.0 Select 2 : 11/32
            REF0_10 = 0b01010,
            /// Comp. D Int. Ref.0 Select 3 : 12/32
            REF0_11 = 0b01011,
            /// Comp. D Int. Ref.0 Select 4 : 13/32
            REF0_12 = 0b01100,
            /// Comp. D Int. Ref.0 Select 5 : 14/32
            REF0_13 = 0b01101,
            /// Comp. D Int. Ref.0 Select 6 : 15/32
            REF0_14 = 0b01110,
            /// Comp. D Int. Ref.0 Select 7 : 16/32
            REF0_15 = 0b01111,
            /// Comp. D Int. Ref.0 Select 0 : 17/32
            REF0_16 = 0b10000,
            /// Comp. D Int. Ref.0 Select 1 : 18/32
            REF0_17 = 0b10001,
            /// Comp. D Int. Ref.0 Select 2 : 19/32
            REF0_18 = 0b10010,
            /// Comp. D Int. Ref.0 Select 3 : 20/32
            REF0_19 = 0b10011,
            /// Comp. D Int. Ref.0 Select 4 : 21/32
            REF0_20 = 0b10100,
            /// Comp. D Int. Ref.0 Select 5 : 22/32
            REF0_21 = 0b10101,
            /// Comp. D Int. Ref.0 Select 6 : 23/32
            REF0_22 = 0b10110,
            /// Comp. D Int. Ref.0 Select 7 : 24/32
            REF0_23 = 0b10111,
            /// Comp. D Int. Ref.0 Select 0 : 25/32
            REF0_24 = 0b11000,
            /// Comp. D Int. Ref.0 Select 1 : 26/32
            REF0_25 = 0b11001,
            /// Comp. D Int. Ref.0 Select 2 : 27/32
            REF0_26 = 0b11010,
            /// Comp. D Int. Ref.0 Select 3 : 28/32
            REF0_27 = 0b11011,
            /// Comp. D Int. Ref.0 Select 4 : 29/32
            REF0_28 = 0b11100,
            /// Comp. D Int. Ref.0 Select 5 : 30/32
            REF0_29 = 0b11101,
            /// Comp. D Int. Ref.0 Select 6 : 31/32
            REF0_30 = 0b11110,
            /// Comp. D Int. Ref.0 Select 7 : 32/32
            REF0_31 = 0b11111,
        }
        /// Comp. D Reference select
        RSEL: 5 = struct RSEL(bool);
        /// Comp. D Reference Source Bit : 0
        RS: 6..7 = enum RS {
            /// Comp. D Reference Source 0 : Off
            RS_0 = 0b00,
            /// Comp. D Reference Source 1 : Vcc
            RS_1 = 0b01,
            /// Comp. D Reference Source 2 : Shared Ref.
            RS_2 = 0b10,
            /// Comp. D Reference Source 3 : Shared Ref. / Off
            RS_3 = 0b11,
        }
        /// Comp. D Reference 1 Resistor Select Bit : 0
        REF1: 8..12 = enum REF1 {
            /// Comp. D Int. Ref.1 Select 0 : 1/32
            REF1_0 = 0b00000,
            /// Comp. D Int. Ref.1 Select 1 : 2/32
            REF1_1 = 0b00001,
            /// Comp. D Int. Ref.1 Select 2 : 3/32
            REF1_2 = 0b00010,
            /// Comp. D Int. Ref.1 Select 3 : 4/32
            REF1_3 = 0b00011,
            /// Comp. D Int. Ref.1 Select 4 : 5/32
            REF1_4 = 0b00100,
            /// Comp. D Int. Ref.1 Select 5 : 6/32
            REF1_5 = 0b00101,
            /// Comp. D Int. Ref.1 Select 6 : 7/32
            REF1_6 = 0b00110,
            /// Comp. D Int. Ref.1 Select 7 : 8/32
            REF1_7 = 0b00111,
            /// Comp. D Int. Ref.1 Select 0 : 9/32
            REF1_8 = 0b01000,
            /// Comp. D Int. Ref.1 Select 1 : 10/32
            REF1_9 = 0b01001,
            /// Comp. D Int. Ref.1 Select 2 : 11/32
            REF1_10 = 0b01010,
            /// Comp. D Int. Ref.1 Select 3 : 12/32
            REF1_11 = 0b01011,
            /// Comp. D Int. Ref.1 Select 4 : 13/32
            REF1_12 = 0b01100,
            /// Comp. D Int. Ref.1 Select 5 : 14/32
            REF1_13 = 0b01101,
            /// Comp. D Int. Ref.1 Select 6 : 15/32
            REF1_14 = 0b01110,
            /// Comp. D Int. Ref.1 Select 7 : 16/32
            REF1_15 = 0b01111,
            /// Comp. D Int. Ref.1 Select 0 : 17/32
            REF1_16 = 0b10000,
            /// Comp. D Int. Ref.1 Select 1 : 18/32
            REF1_17 = 0b10001,
            /// Comp. D Int. Ref.1 Select 2 : 19/32
            REF1_18 = 0b10010,
            /// Comp. D Int. Ref.1 Select 3 : 20/32
            REF1_19 = 0b10011,
            /// Comp. D Int. Ref.1 Select 4 : 21/32
            REF1_20 = 0b10100,
            /// Comp. D Int. Ref.1 Select 5 : 22/32
            REF1_21 = 0b10101,
            /// Comp. D Int. Ref.1 Select 6 : 23/32
            REF1_22 = 0b10110,
            /// Comp. D Int. Ref.1 Select 7 : 24/32
            REF1_23 = 0b10111,
            /// Comp. D Int. Ref.1 Select 0 : 25/32
            REF1_24 = 0b11000,
            /// Comp. D Int. Ref.1 Select 1 : 26/32
            REF1_25 = 0b11001,
            /// Comp. D Int. Ref.1 Select 2 : 27/32
            REF1_26 = 0b11010,
            /// Comp. D Int. Ref.1 Select 3 : 28/32
            REF1_27 = 0b11011,
            /// Comp. D Int. Ref.1 Select 4 : 29/32
            REF1_28 = 0b11100,
            /// Comp. D Int. Ref.1 Select 5 : 30/32
            REF1_29 = 0b11101,
            /// Comp. D Int. Ref.1 Select 6 : 31/32
            REF1_30 = 0b11110,
            /// Comp. D Int. Ref.1 Select 7 : 32/32
            REF1_31 = 0b11111,
        }
        /// Comp. D Reference voltage level Bit : 0
        REFL: 13..14 = enum REFL {
            /// Comp. D Reference voltage level 0 : None
            REFL_0 = 0b00,
            /// Comp. D Reference voltage level 1 : 1.5V
            REFL_1 = 0b01,
            /// Comp. D Reference voltage level 2 : 2.0V
            REFL_2 = 0b10,
            /// Comp. D Reference voltage level 3 : 2.5V
            REFL_3 = 0b11,
        }
        /// Comp. D Reference Accuracy
        REFACC: 15 = struct REFACC(bool);
    }
    /// Comparator D Control Register 3
    rw CTL3 @ 0x06: u16 = 0_0 {
        /// Comp. D Disable Input Buffer of Port Register .0
        PD0: 0 = struct PD0(bool);
        /// Comp. D Disable Input Buffer of Port Register .1
        PD1: 1 = struct PD1(bool);
        /// Comp. D Disable Input Buffer of Port Register .2
        PD2: 2 = struct PD2(bool);
        /// Comp. D Disable Input Buffer of Port Register .3
        PD3: 3 = struct PD3(bool);
        /// Comp. D Disable Input Buffer of Port Register .4
        PD4: 4 = struct PD4(bool);
        /// Comp. D Disable Input Buffer of Port Register .5
        PD5: 5 = struct PD5(bool);
        /// Comp. D Disable Input Buffer of Port Register .6
        PD6: 6 = struct PD6(bool);
        /// Comp. D Disable Input Buffer of Port Register .7
        PD7: 7 = struct PD7(bool);
        /// Comp. D Disable Input Buffer of Port Register .8
        PD8: 8 = struct PD8(bool);
        /// Comp. D Disable Input Buffer of Port Register .9
        PD9: 9 = struct PD9(bool);
        /// Comp. D Disable Input Buffer of Port Register .10
        PD10: 10 = struct PD10(bool);
        /// Comp. D Disable Input Buffer of Port Register .11
        PD11: 11 = struct PD11(bool);
        /// Comp. D Disable Input Buffer of Port Register .12
        PD12: 12 = struct PD12(bool);
        /// Comp. D Disable Input Buffer of Port Register .13
        PD13: 13 = struct PD13(bool);
        /// Comp. D Disable Input Buffer of Port Register .14
        PD14: 14 = struct PD14(bool);
        /// Comp. D Disable Input Buffer of Port Register .15
        PD15: 15 = struct PD15(bool);
    }
    /// Comparator D Interrupt Register
    rw INT @ 0x0c: u16 = 0_0 {
        /// Comp. D Interrupt Flag
        IFG: 0 = struct IFG(bool);
        /// Comp. D Interrupt Flag Inverted Polarity
        IIFG: 1 = struct IIFG(bool);
        /// Comp. D Interrupt Enable
        IE: 8 = struct IE(bool);
        /// Comp. D Interrupt Enable Inverted Polarity
        IIE: 9 = struct IIE(bool);
    }
    /// Comparator D Interrupt Vector Word
    rw IV @ 0x0e: u16 = 0_0 {
        /// Comparator D Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
}
