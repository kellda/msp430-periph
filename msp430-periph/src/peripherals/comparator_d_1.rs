//! Comparator D

utils::periph! {
    /// Comparator D
    ComparatorD;
    /// Comparator D Control Register 0
    rw CDCTL0 @ 0x00: u16 = 0_0 {
        /// Comp. D Pos. Channel Input Select 0
        CDIPSEL: 0..3 = enum CDIPSEL {
            /// Comp. D V+ terminal Input Select: Channel 0
            CDIPSEL_0 = 0b0000,
            /// Comp. D V+ terminal Input Select: Channel 1
            CDIPSEL_1 = 0b0001,
            /// Comp. D V+ terminal Input Select: Channel 2
            CDIPSEL_2 = 0b0010,
            /// Comp. D V+ terminal Input Select: Channel 3
            CDIPSEL_3 = 0b0011,
            /// Comp. D V+ terminal Input Select: Channel 4
            CDIPSEL_4 = 0b0100,
            /// Comp. D V+ terminal Input Select: Channel 5
            CDIPSEL_5 = 0b0101,
            /// Comp. D V+ terminal Input Select: Channel 6
            CDIPSEL_6 = 0b0110,
            /// Comp. D V+ terminal Input Select: Channel 7
            CDIPSEL_7 = 0b0111,
            /// Comp. D V+ terminal Input Select: Channel 8
            CDIPSEL_8 = 0b1000,
            /// Comp. D V+ terminal Input Select: Channel 9
            CDIPSEL_9 = 0b1001,
            /// Comp. D V+ terminal Input Select: Channel 10
            CDIPSEL_10 = 0b1010,
            /// Comp. D V+ terminal Input Select: Channel 11
            CDIPSEL_11 = 0b1011,
            /// Comp. D V+ terminal Input Select: Channel 12
            CDIPSEL_12 = 0b1100,
            /// Comp. D V+ terminal Input Select: Channel 13
            CDIPSEL_13 = 0b1101,
            /// Comp. D V+ terminal Input Select: Channel 14
            CDIPSEL_14 = 0b1110,
            /// Comp. D V+ terminal Input Select: Channel 15
            CDIPSEL_15 = 0b1111,
        }
        /// Comp. D Pos. Channel Input Enable
        CDIPEN: 7 = struct CDIPEN(bool);
        /// Comp. D Neg. Channel Input Select 0
        CDIMSEL: 8..11 = enum CDIMSEL {
            /// Comp. D V- Terminal Input Select: Channel 0
            CDIMSEL_0 = 0b0000,
            /// Comp. D V- Terminal Input Select: Channel 1
            CDIMSEL_1 = 0b0001,
            /// Comp. D V- Terminal Input Select: Channel 2
            CDIMSEL_2 = 0b0010,
            /// Comp. D V- Terminal Input Select: Channel 3
            CDIMSEL_3 = 0b0011,
            /// Comp. D V- Terminal Input Select: Channel 4
            CDIMSEL_4 = 0b0100,
            /// Comp. D V- Terminal Input Select: Channel 5
            CDIMSEL_5 = 0b0101,
            /// Comp. D V- Terminal Input Select: Channel 6
            CDIMSEL_6 = 0b0110,
            /// Comp. D V- Terminal Input Select: Channel 7
            CDIMSEL_7 = 0b0111,
            /// Comp. D V- terminal Input Select: Channel 8
            CDIMSEL_8 = 0b1000,
            /// Comp. D V- terminal Input Select: Channel 9
            CDIMSEL_9 = 0b1001,
            /// Comp. D V- terminal Input Select: Channel 10
            CDIMSEL_10 = 0b1010,
            /// Comp. D V- terminal Input Select: Channel 11
            CDIMSEL_11 = 0b1011,
            /// Comp. D V- terminal Input Select: Channel 12
            CDIMSEL_12 = 0b1100,
            /// Comp. D V- terminal Input Select: Channel 13
            CDIMSEL_13 = 0b1101,
            /// Comp. D V- terminal Input Select: Channel 14
            CDIMSEL_14 = 0b1110,
            /// Comp. D V- terminal Input Select: Channel 15
            CDIMSEL_15 = 0b1111,
        }
        /// Comp. D Neg. Channel Input Enable
        CDIMEN: 15 = struct CDIMEN(bool);
    }
    /// Comparator D Control Register 1
    rw CDCTL1 @ 0x02: u16 = 0_0 {
        /// Comp. D Output
        CDOUT: 0 = struct CDOUT(bool);
        /// Comp. D Output Polarity
        CDOUTPOL: 1 = struct CDOUTPOL(bool);
        /// Comp. D Enable Output Filter
        CDF: 2 = struct CDF(bool);
        /// Comp. D Interrupt Edge Select
        CDIES: 3 = struct CDIES(bool);
        /// Comp. D Input Short
        CDSHORT: 4 = struct CDSHORT(bool);
        /// Comp. D Exchange Inputs
        CDEX: 5 = struct CDEX(bool);
        /// Comp. D Filter delay Bit 0
        CDFDLY: 6..7 = enum CDFDLY {
            /// Comp. D Filter delay 0 : 450ns
            CDFDLY_0 = 0b00,
            /// Comp. D Filter delay 1 : 900ns
            CDFDLY_1 = 0b01,
            /// Comp. D Filter delay 2 : 1800ns
            CDFDLY_2 = 0b10,
            /// Comp. D Filter delay 3 : 3600ns
            CDFDLY_3 = 0b11,
        }
        /// Comp. D enable
        CDON: 10 = struct CDON(bool);
        /// Comp. D CDMRV Level
        CDMRVL: 11 = struct CDMRVL(bool);
        /// Comp. D Output selects between VREF0 or VREF1
        CDMRVS: 12 = struct CDMRVS(bool);
    }
    /// Comparator D Control Register 2
    rw CDCTL2 @ 0x04: u16 = 0_0 {
        /// Comp. D Reference 0 Resistor Select Bit : 0
        CDREF0: 0..4 = enum CDREF0 {
            /// Comp. D Int. Ref.0 Select 0 : 1/32
            CDREF0_0 = 0b00000,
            /// Comp. D Int. Ref.0 Select 1 : 2/32
            CDREF0_1 = 0b00001,
            /// Comp. D Int. Ref.0 Select 2 : 3/32
            CDREF0_2 = 0b00010,
            /// Comp. D Int. Ref.0 Select 3 : 4/32
            CDREF0_3 = 0b00011,
            /// Comp. D Int. Ref.0 Select 4 : 5/32
            CDREF0_4 = 0b00100,
            /// Comp. D Int. Ref.0 Select 5 : 6/32
            CDREF0_5 = 0b00101,
            /// Comp. D Int. Ref.0 Select 6 : 7/32
            CDREF0_6 = 0b00110,
            /// Comp. D Int. Ref.0 Select 7 : 8/32
            CDREF0_7 = 0b00111,
            /// Comp. D Int. Ref.0 Select 0 : 9/32
            CDREF0_8 = 0b01000,
            /// Comp. D Int. Ref.0 Select 1 : 10/32
            CDREF0_9 = 0b01001,
            /// Comp. D Int. Ref.0 Select 2 : 11/32
            CDREF0_10 = 0b01010,
            /// Comp. D Int. Ref.0 Select 3 : 12/32
            CDREF0_11 = 0b01011,
            /// Comp. D Int. Ref.0 Select 4 : 13/32
            CDREF0_12 = 0b01100,
            /// Comp. D Int. Ref.0 Select 5 : 14/32
            CDREF0_13 = 0b01101,
            /// Comp. D Int. Ref.0 Select 6 : 15/32
            CDREF0_14 = 0b01110,
            /// Comp. D Int. Ref.0 Select 7 : 16/32
            CDREF0_15 = 0b01111,
            /// Comp. D Int. Ref.0 Select 0 : 17/32
            CDREF0_16 = 0b10000,
            /// Comp. D Int. Ref.0 Select 1 : 18/32
            CDREF0_17 = 0b10001,
            /// Comp. D Int. Ref.0 Select 2 : 19/32
            CDREF0_18 = 0b10010,
            /// Comp. D Int. Ref.0 Select 3 : 20/32
            CDREF0_19 = 0b10011,
            /// Comp. D Int. Ref.0 Select 4 : 21/32
            CDREF0_20 = 0b10100,
            /// Comp. D Int. Ref.0 Select 5 : 22/32
            CDREF0_21 = 0b10101,
            /// Comp. D Int. Ref.0 Select 6 : 23/32
            CDREF0_22 = 0b10110,
            /// Comp. D Int. Ref.0 Select 7 : 24/32
            CDREF0_23 = 0b10111,
            /// Comp. D Int. Ref.0 Select 0 : 25/32
            CDREF0_24 = 0b11000,
            /// Comp. D Int. Ref.0 Select 1 : 26/32
            CDREF0_25 = 0b11001,
            /// Comp. D Int. Ref.0 Select 2 : 27/32
            CDREF0_26 = 0b11010,
            /// Comp. D Int. Ref.0 Select 3 : 28/32
            CDREF0_27 = 0b11011,
            /// Comp. D Int. Ref.0 Select 4 : 29/32
            CDREF0_28 = 0b11100,
            /// Comp. D Int. Ref.0 Select 5 : 30/32
            CDREF0_29 = 0b11101,
            /// Comp. D Int. Ref.0 Select 6 : 31/32
            CDREF0_30 = 0b11110,
            /// Comp. D Int. Ref.0 Select 7 : 32/32
            CDREF0_31 = 0b11111,
        }
        /// Comp. D Reference select
        CDRSEL: 5 = struct CDRSEL(bool);
        /// Comp. D Reference Source Bit : 0
        CDRS: 6..7 = enum CDRS {
            /// Comp. D Reference Source 0 : Off
            CDRS_0 = 0b00,
            /// Comp. D Reference Source 1 : Vcc
            CDRS_1 = 0b01,
            /// Comp. D Reference Source 2 : Shared Ref.
            CDRS_2 = 0b10,
            /// Comp. D Reference Source 3 : Shared Ref. / Off
            CDRS_3 = 0b11,
        }
        /// Comp. D Reference 1 Resistor Select Bit : 0
        CDREF1: 8..12 = enum CDREF1 {
            /// Comp. D Int. Ref.1 Select 0 : 1/32
            CDREF1_0 = 0b00000,
            /// Comp. D Int. Ref.1 Select 1 : 2/32
            CDREF1_1 = 0b00001,
            /// Comp. D Int. Ref.1 Select 2 : 3/32
            CDREF1_2 = 0b00010,
            /// Comp. D Int. Ref.1 Select 3 : 4/32
            CDREF1_3 = 0b00011,
            /// Comp. D Int. Ref.1 Select 4 : 5/32
            CDREF1_4 = 0b00100,
            /// Comp. D Int. Ref.1 Select 5 : 6/32
            CDREF1_5 = 0b00101,
            /// Comp. D Int. Ref.1 Select 6 : 7/32
            CDREF1_6 = 0b00110,
            /// Comp. D Int. Ref.1 Select 7 : 8/32
            CDREF1_7 = 0b00111,
            /// Comp. D Int. Ref.1 Select 0 : 9/32
            CDREF1_8 = 0b01000,
            /// Comp. D Int. Ref.1 Select 1 : 10/32
            CDREF1_9 = 0b01001,
            /// Comp. D Int. Ref.1 Select 2 : 11/32
            CDREF1_10 = 0b01010,
            /// Comp. D Int. Ref.1 Select 3 : 12/32
            CDREF1_11 = 0b01011,
            /// Comp. D Int. Ref.1 Select 4 : 13/32
            CDREF1_12 = 0b01100,
            /// Comp. D Int. Ref.1 Select 5 : 14/32
            CDREF1_13 = 0b01101,
            /// Comp. D Int. Ref.1 Select 6 : 15/32
            CDREF1_14 = 0b01110,
            /// Comp. D Int. Ref.1 Select 7 : 16/32
            CDREF1_15 = 0b01111,
            /// Comp. D Int. Ref.1 Select 0 : 17/32
            CDREF1_16 = 0b10000,
            /// Comp. D Int. Ref.1 Select 1 : 18/32
            CDREF1_17 = 0b10001,
            /// Comp. D Int. Ref.1 Select 2 : 19/32
            CDREF1_18 = 0b10010,
            /// Comp. D Int. Ref.1 Select 3 : 20/32
            CDREF1_19 = 0b10011,
            /// Comp. D Int. Ref.1 Select 4 : 21/32
            CDREF1_20 = 0b10100,
            /// Comp. D Int. Ref.1 Select 5 : 22/32
            CDREF1_21 = 0b10101,
            /// Comp. D Int. Ref.1 Select 6 : 23/32
            CDREF1_22 = 0b10110,
            /// Comp. D Int. Ref.1 Select 7 : 24/32
            CDREF1_23 = 0b10111,
            /// Comp. D Int. Ref.1 Select 0 : 25/32
            CDREF1_24 = 0b11000,
            /// Comp. D Int. Ref.1 Select 1 : 26/32
            CDREF1_25 = 0b11001,
            /// Comp. D Int. Ref.1 Select 2 : 27/32
            CDREF1_26 = 0b11010,
            /// Comp. D Int. Ref.1 Select 3 : 28/32
            CDREF1_27 = 0b11011,
            /// Comp. D Int. Ref.1 Select 4 : 29/32
            CDREF1_28 = 0b11100,
            /// Comp. D Int. Ref.1 Select 5 : 30/32
            CDREF1_29 = 0b11101,
            /// Comp. D Int. Ref.1 Select 6 : 31/32
            CDREF1_30 = 0b11110,
            /// Comp. D Int. Ref.1 Select 7 : 32/32
            CDREF1_31 = 0b11111,
        }
        /// Comp. D Reference voltage level Bit : 0
        CDREFL: 13..14 = enum CDREFL {
            /// Comp. D Reference voltage level 0 : None
            CDREFL_0 = 0b00,
            /// Comp. D Reference voltage level 1 : 1.5V
            CDREFL_1 = 0b01,
            /// Comp. D Reference voltage level 2 : 2.0V
            CDREFL_2 = 0b10,
            /// Comp. D Reference voltage level 3 : 2.5V
            CDREFL_3 = 0b11,
        }
        /// Comp. D Reference Accuracy
        CDREFACC: 15 = struct CDREFACC(bool);
    }
    /// Comparator D Control Register 3
    rw CDCTL3 @ 0x06: u16 = 0_0 {
        /// Comp. D Disable Input Buffer of Port Register .0
        CDPD0: 0 = struct CDPD0(bool);
        /// Comp. D Disable Input Buffer of Port Register .1
        CDPD1: 1 = struct CDPD1(bool);
        /// Comp. D Disable Input Buffer of Port Register .2
        CDPD2: 2 = struct CDPD2(bool);
        /// Comp. D Disable Input Buffer of Port Register .3
        CDPD3: 3 = struct CDPD3(bool);
        /// Comp. D Disable Input Buffer of Port Register .4
        CDPD4: 4 = struct CDPD4(bool);
        /// Comp. D Disable Input Buffer of Port Register .5
        CDPD5: 5 = struct CDPD5(bool);
        /// Comp. D Disable Input Buffer of Port Register .6
        CDPD6: 6 = struct CDPD6(bool);
        /// Comp. D Disable Input Buffer of Port Register .7
        CDPD7: 7 = struct CDPD7(bool);
        /// Comp. D Disable Input Buffer of Port Register .8
        CDPD8: 8 = struct CDPD8(bool);
        /// Comp. D Disable Input Buffer of Port Register .9
        CDPD9: 9 = struct CDPD9(bool);
        /// Comp. D Disable Input Buffer of Port Register .10
        CDPD10: 10 = struct CDPD10(bool);
        /// Comp. D Disable Input Buffer of Port Register .11
        CDPD11: 11 = struct CDPD11(bool);
        /// Comp. D Disable Input Buffer of Port Register .12
        CDPD12: 12 = struct CDPD12(bool);
        /// Comp. D Disable Input Buffer of Port Register .13
        CDPD13: 13 = struct CDPD13(bool);
        /// Comp. D Disable Input Buffer of Port Register .14
        CDPD14: 14 = struct CDPD14(bool);
        /// Comp. D Disable Input Buffer of Port Register .15
        CDPD15: 15 = struct CDPD15(bool);
    }
    /// Comparator D Interrupt Register
    rw CDINT @ 0x0c: u16 = 0_0 {
        /// Comp. D Interrupt Flag
        CDIFG: 0 = struct CDIFG(bool);
        /// Comp. D Interrupt Flag Inverted Polarity
        CDIIFG: 1 = struct CDIIFG(bool);
        /// Comp. D Interrupt Enable
        CDIE: 8 = struct CDIE(bool);
        /// Comp. D Interrupt Enable Inverted Polarity
        CDIIE: 9 = struct CDIIE(bool);
    }
    /// Comparator D Interrupt Vector Word
    rw CDIV @ 0x0e: u16 = 0_0 {
        /// Comparator D Interrupt Vector Word
        CDIV: 0..15 = struct CDIVField(u16);
    }
}
