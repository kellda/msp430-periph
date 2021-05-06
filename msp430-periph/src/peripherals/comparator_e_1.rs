//! Comparator E

utils::periph! {
    /// Comparator E
    ComparatorE;
    /// Comparator E Control Register 0
    rw CECTL0 @ 0x00: u16 = 0_0 {
        /// Comp. E Pos. Channel Input Select 0
        CEIPSEL: 0..3 = enum CEIPSEL {
            /// Comp. E V+ terminal Input Select: Channel 0
            CEIPSEL_0 = 0b0000,
            /// Comp. E V+ terminal Input Select: Channel 1
            CEIPSEL_1 = 0b0001,
            /// Comp. E V+ terminal Input Select: Channel 2
            CEIPSEL_2 = 0b0010,
            /// Comp. E V+ terminal Input Select: Channel 3
            CEIPSEL_3 = 0b0011,
            /// Comp. E V+ terminal Input Select: Channel 4
            CEIPSEL_4 = 0b0100,
            /// Comp. E V+ terminal Input Select: Channel 5
            CEIPSEL_5 = 0b0101,
            /// Comp. E V+ terminal Input Select: Channel 6
            CEIPSEL_6 = 0b0110,
            /// Comp. E V+ terminal Input Select: Channel 7
            CEIPSEL_7 = 0b0111,
            /// Comp. E V+ terminal Input Select: Channel 8
            CEIPSEL_8 = 0b1000,
            /// Comp. E V+ terminal Input Select: Channel 9
            CEIPSEL_9 = 0b1001,
            /// Comp. E V+ terminal Input Select: Channel 10
            CEIPSEL_10 = 0b1010,
            /// Comp. E V+ terminal Input Select: Channel 11
            CEIPSEL_11 = 0b1011,
            /// Comp. E V+ terminal Input Select: Channel 12
            CEIPSEL_12 = 0b1100,
            /// Comp. E V+ terminal Input Select: Channel 13
            CEIPSEL_13 = 0b1101,
            /// Comp. E V+ terminal Input Select: Channel 14
            CEIPSEL_14 = 0b1110,
            /// Comp. E V+ terminal Input Select: Channel 15
            CEIPSEL_15 = 0b1111,
        }
        /// Comp. E Pos. Channel Input Enable
        CEIPEN: 7 = struct CEIPEN(bool);
        /// Comp. E Neg. Channel Input Select 0
        CEIMSEL: 8..11 = enum CEIMSEL {
            /// Comp. E V- Terminal Input Select: Channel 0
            CEIMSEL_0 = 0b0000,
            /// Comp. E V- Terminal Input Select: Channel 1
            CEIMSEL_1 = 0b0001,
            /// Comp. E V- Terminal Input Select: Channel 2
            CEIMSEL_2 = 0b0010,
            /// Comp. E V- Terminal Input Select: Channel 3
            CEIMSEL_3 = 0b0011,
            /// Comp. E V- Terminal Input Select: Channel 4
            CEIMSEL_4 = 0b0100,
            /// Comp. E V- Terminal Input Select: Channel 5
            CEIMSEL_5 = 0b0101,
            /// Comp. E V- Terminal Input Select: Channel 6
            CEIMSEL_6 = 0b0110,
            /// Comp. E V- Terminal Input Select: Channel 7
            CEIMSEL_7 = 0b0111,
            /// Comp. E V- terminal Input Select: Channel 8
            CEIMSEL_8 = 0b1000,
            /// Comp. E V- terminal Input Select: Channel 9
            CEIMSEL_9 = 0b1001,
            /// Comp. E V- terminal Input Select: Channel 10
            CEIMSEL_10 = 0b1010,
            /// Comp. E V- terminal Input Select: Channel 11
            CEIMSEL_11 = 0b1011,
            /// Comp. E V- terminal Input Select: Channel 12
            CEIMSEL_12 = 0b1100,
            /// Comp. E V- terminal Input Select: Channel 13
            CEIMSEL_13 = 0b1101,
            /// Comp. E V- terminal Input Select: Channel 14
            CEIMSEL_14 = 0b1110,
            /// Comp. E V- terminal Input Select: Channel 15
            CEIMSEL_15 = 0b1111,
        }
        /// Comp. E Neg. Channel Input Enable
        CEIMEN: 15 = struct CEIMEN(bool);
    }
    /// Comparator E Control Register 1
    rw CECTL1 @ 0x02: u16 = 0_0 {
        /// Comp. E Output
        CEOUT: 0 = struct CEOUT(bool);
        /// Comp. E Output Polarity
        CEOUTPOL: 1 = struct CEOUTPOL(bool);
        /// Comp. E Enable Output Filter
        CEF: 2 = struct CEF(bool);
        /// Comp. E Interrupt Edge Select
        CEIES: 3 = struct CEIES(bool);
        /// Comp. E Input Short
        CESHORT: 4 = struct CESHORT(bool);
        /// Comp. E Exchange Inputs
        CEEX: 5 = struct CEEX(bool);
        /// Comp. E Filter delay Bit 0
        CEFDLY: 6..7 = enum CEFDLY {
            /// Comp. E Filter delay 0 : 450ns
            CEFDLY_0 = 0b00,
            /// Comp. E Filter delay 1 : 900ns
            CEFDLY_1 = 0b01,
            /// Comp. E Filter delay 2 : 1800ns
            CEFDLY_2 = 0b10,
            /// Comp. E Filter delay 3 : 3600ns
            CEFDLY_3 = 0b11,
        }
        /// Comp. E Power mode Bit 0
        CEPWRMD: 8..9 = enum CEPWRMD {
            /// Comp. E Power mode 0
            CEPWRMD_0 = 0b00,
            /// Comp. E Power mode 1
            CEPWRMD_1 = 0b01,
            /// Comp. E Power mode 2
            CEPWRMD_2 = 0b10,
            /// Comp. E Power mode 3
            CEPWRMD_3 = 0b11,
        }
        /// Comp. E enable
        CEON: 10 = struct CEON(bool);
        /// Comp. E CEMRV Level
        CEMRVL: 11 = struct CEMRVL(bool);
        /// Comp. E Output selects between VREF0 or VREF1
        CEMRVS: 12 = struct CEMRVS(bool);
    }
    /// Comparator E Control Register 2
    rw CECTL2 @ 0x04: u16 = 0_0 {
        /// Comp. E Reference 0 Resistor Select Bit : 0
        CEREF0: 0..4 = enum CEREF0 {
            /// Comp. E Int. Ref.0 Select 0 : 1/32
            CEREF0_0 = 0b00000,
            /// Comp. E Int. Ref.0 Select 1 : 2/32
            CEREF0_1 = 0b00001,
            /// Comp. E Int. Ref.0 Select 2 : 3/32
            CEREF0_2 = 0b00010,
            /// Comp. E Int. Ref.0 Select 3 : 4/32
            CEREF0_3 = 0b00011,
            /// Comp. E Int. Ref.0 Select 4 : 5/32
            CEREF0_4 = 0b00100,
            /// Comp. E Int. Ref.0 Select 5 : 6/32
            CEREF0_5 = 0b00101,
            /// Comp. E Int. Ref.0 Select 6 : 7/32
            CEREF0_6 = 0b00110,
            /// Comp. E Int. Ref.0 Select 7 : 8/32
            CEREF0_7 = 0b00111,
            /// Comp. E Int. Ref.0 Select 0 : 9/32
            CEREF0_8 = 0b01000,
            /// Comp. E Int. Ref.0 Select 1 : 10/32
            CEREF0_9 = 0b01001,
            /// Comp. E Int. Ref.0 Select 2 : 11/32
            CEREF0_10 = 0b01010,
            /// Comp. E Int. Ref.0 Select 3 : 12/32
            CEREF0_11 = 0b01011,
            /// Comp. E Int. Ref.0 Select 4 : 13/32
            CEREF0_12 = 0b01100,
            /// Comp. E Int. Ref.0 Select 5 : 14/32
            CEREF0_13 = 0b01101,
            /// Comp. E Int. Ref.0 Select 6 : 15/32
            CEREF0_14 = 0b01110,
            /// Comp. E Int. Ref.0 Select 7 : 16/32
            CEREF0_15 = 0b01111,
            /// Comp. E Int. Ref.0 Select 0 : 17/32
            CEREF0_16 = 0b10000,
            /// Comp. E Int. Ref.0 Select 1 : 18/32
            CEREF0_17 = 0b10001,
            /// Comp. E Int. Ref.0 Select 2 : 19/32
            CEREF0_18 = 0b10010,
            /// Comp. E Int. Ref.0 Select 3 : 20/32
            CEREF0_19 = 0b10011,
            /// Comp. E Int. Ref.0 Select 4 : 21/32
            CEREF0_20 = 0b10100,
            /// Comp. E Int. Ref.0 Select 5 : 22/32
            CEREF0_21 = 0b10101,
            /// Comp. E Int. Ref.0 Select 6 : 23/32
            CEREF0_22 = 0b10110,
            /// Comp. E Int. Ref.0 Select 7 : 24/32
            CEREF0_23 = 0b10111,
            /// Comp. E Int. Ref.0 Select 0 : 25/32
            CEREF0_24 = 0b11000,
            /// Comp. E Int. Ref.0 Select 1 : 26/32
            CEREF0_25 = 0b11001,
            /// Comp. E Int. Ref.0 Select 2 : 27/32
            CEREF0_26 = 0b11010,
            /// Comp. E Int. Ref.0 Select 3 : 28/32
            CEREF0_27 = 0b11011,
            /// Comp. E Int. Ref.0 Select 4 : 29/32
            CEREF0_28 = 0b11100,
            /// Comp. E Int. Ref.0 Select 5 : 30/32
            CEREF0_29 = 0b11101,
            /// Comp. E Int. Ref.0 Select 6 : 31/32
            CEREF0_30 = 0b11110,
            /// Comp. E Int. Ref.0 Select 7 : 32/32
            CEREF0_31 = 0b11111,
        }
        /// Comp. E Reference select
        CERSEL: 5 = struct CERSEL(bool);
        /// Comp. E Reference Source Bit : 0
        CERS: 6..7 = enum CERS {
            /// Comp. E Reference Source 0 : Off
            CERS_0 = 0b00,
            /// Comp. E Reference Source 1 : Vcc
            CERS_1 = 0b01,
            /// Comp. E Reference Source 2 : Shared Ref.
            CERS_2 = 0b10,
            /// Comp. E Reference Source 3 : Shared Ref. / Off
            CERS_3 = 0b11,
        }
        /// Comp. E Reference 1 Resistor Select Bit : 0
        CEREF1: 8..12 = enum CEREF1 {
            /// Comp. E Int. Ref.1 Select 0 : 1/32
            CEREF1_0 = 0b00000,
            /// Comp. E Int. Ref.1 Select 1 : 2/32
            CEREF1_1 = 0b00001,
            /// Comp. E Int. Ref.1 Select 2 : 3/32
            CEREF1_2 = 0b00010,
            /// Comp. E Int. Ref.1 Select 3 : 4/32
            CEREF1_3 = 0b00011,
            /// Comp. E Int. Ref.1 Select 4 : 5/32
            CEREF1_4 = 0b00100,
            /// Comp. E Int. Ref.1 Select 5 : 6/32
            CEREF1_5 = 0b00101,
            /// Comp. E Int. Ref.1 Select 6 : 7/32
            CEREF1_6 = 0b00110,
            /// Comp. E Int. Ref.1 Select 7 : 8/32
            CEREF1_7 = 0b00111,
            /// Comp. E Int. Ref.1 Select 0 : 9/32
            CEREF1_8 = 0b01000,
            /// Comp. E Int. Ref.1 Select 1 : 10/32
            CEREF1_9 = 0b01001,
            /// Comp. E Int. Ref.1 Select 2 : 11/32
            CEREF1_10 = 0b01010,
            /// Comp. E Int. Ref.1 Select 3 : 12/32
            CEREF1_11 = 0b01011,
            /// Comp. E Int. Ref.1 Select 4 : 13/32
            CEREF1_12 = 0b01100,
            /// Comp. E Int. Ref.1 Select 5 : 14/32
            CEREF1_13 = 0b01101,
            /// Comp. E Int. Ref.1 Select 6 : 15/32
            CEREF1_14 = 0b01110,
            /// Comp. E Int. Ref.1 Select 7 : 16/32
            CEREF1_15 = 0b01111,
            /// Comp. E Int. Ref.1 Select 0 : 17/32
            CEREF1_16 = 0b10000,
            /// Comp. E Int. Ref.1 Select 1 : 18/32
            CEREF1_17 = 0b10001,
            /// Comp. E Int. Ref.1 Select 2 : 19/32
            CEREF1_18 = 0b10010,
            /// Comp. E Int. Ref.1 Select 3 : 20/32
            CEREF1_19 = 0b10011,
            /// Comp. E Int. Ref.1 Select 4 : 21/32
            CEREF1_20 = 0b10100,
            /// Comp. E Int. Ref.1 Select 5 : 22/32
            CEREF1_21 = 0b10101,
            /// Comp. E Int. Ref.1 Select 6 : 23/32
            CEREF1_22 = 0b10110,
            /// Comp. E Int. Ref.1 Select 7 : 24/32
            CEREF1_23 = 0b10111,
            /// Comp. E Int. Ref.1 Select 0 : 25/32
            CEREF1_24 = 0b11000,
            /// Comp. E Int. Ref.1 Select 1 : 26/32
            CEREF1_25 = 0b11001,
            /// Comp. E Int. Ref.1 Select 2 : 27/32
            CEREF1_26 = 0b11010,
            /// Comp. E Int. Ref.1 Select 3 : 28/32
            CEREF1_27 = 0b11011,
            /// Comp. E Int. Ref.1 Select 4 : 29/32
            CEREF1_28 = 0b11100,
            /// Comp. E Int. Ref.1 Select 5 : 30/32
            CEREF1_29 = 0b11101,
            /// Comp. E Int. Ref.1 Select 6 : 31/32
            CEREF1_30 = 0b11110,
            /// Comp. E Int. Ref.1 Select 7 : 32/32
            CEREF1_31 = 0b11111,
        }
        /// Comp. E Reference voltage level Bit : 0
        CEREFL: 13..14 = enum CEREFL {
            /// Comp. E Reference voltage level 0 : None
            CEREFL_0 = 0b00,
            /// Comp. E Reference voltage level 1 : 1.2V
            CEREFL_1 = 0b01,
            /// Comp. E Reference voltage level 2 : 2.0V
            CEREFL_2 = 0b10,
            /// Comp. E Reference voltage level 3 : 2.5V
            CEREFL_3 = 0b11,
        }
        /// Comp. E Reference Accuracy
        CEREFACC: 15 = struct CEREFACC(bool);
    }
    /// Comparator E Control Register 3
    rw CECTL3 @ 0x06: u16 = 0_0 {
        /// Comp. E Disable Input Buffer of Port Register .0
        CEPD0: 0 = struct CEPD0(bool);
        /// Comp. E Disable Input Buffer of Port Register .1
        CEPD1: 1 = struct CEPD1(bool);
        /// Comp. E Disable Input Buffer of Port Register .2
        CEPD2: 2 = struct CEPD2(bool);
        /// Comp. E Disable Input Buffer of Port Register .3
        CEPD3: 3 = struct CEPD3(bool);
        /// Comp. E Disable Input Buffer of Port Register .4
        CEPD4: 4 = struct CEPD4(bool);
        /// Comp. E Disable Input Buffer of Port Register .5
        CEPD5: 5 = struct CEPD5(bool);
        /// Comp. E Disable Input Buffer of Port Register .6
        CEPD6: 6 = struct CEPD6(bool);
        /// Comp. E Disable Input Buffer of Port Register .7
        CEPD7: 7 = struct CEPD7(bool);
        /// Comp. E Disable Input Buffer of Port Register .8
        CEPD8: 8 = struct CEPD8(bool);
        /// Comp. E Disable Input Buffer of Port Register .9
        CEPD9: 9 = struct CEPD9(bool);
        /// Comp. E Disable Input Buffer of Port Register .10
        CEPD10: 10 = struct CEPD10(bool);
        /// Comp. E Disable Input Buffer of Port Register .11
        CEPD11: 11 = struct CEPD11(bool);
        /// Comp. E Disable Input Buffer of Port Register .12
        CEPD12: 12 = struct CEPD12(bool);
        /// Comp. E Disable Input Buffer of Port Register .13
        CEPD13: 13 = struct CEPD13(bool);
        /// Comp. E Disable Input Buffer of Port Register .14
        CEPD14: 14 = struct CEPD14(bool);
        /// Comp. E Disable Input Buffer of Port Register .15
        CEPD15: 15 = struct CEPD15(bool);
    }
    /// Comparator E Interrupt Register
    rw CEINT @ 0x0c: u16 = 0_0 {
        /// Comp. E Interrupt Flag
        CEIFG: 0 = struct CEIFG(bool);
        /// Comp. E Interrupt Flag Inverted Polarity
        CEIIFG: 1 = struct CEIIFG(bool);
        /// Comp. E Comparator_E ready interrupt flag
        CERDYIFG: 4 = struct CERDYIFG(bool);
        /// Comp. E Interrupt Enable
        CEIE: 8 = struct CEIE(bool);
        /// Comp. E Interrupt Enable Inverted Polarity
        CEIIE: 9 = struct CEIIE(bool);
        /// Comp. E Comparator_E ready interrupt enable
        CERDYIE: 12 = struct CERDYIE(bool);
    }
    /// Comparator E Interrupt Vector Word
    rw CEIV @ 0x0e: u16 = 0_0 {
        /// Comparator E Interrupt Vector Word
        CEIV: 0..15 = struct CEIVField(u16);
    }
}
