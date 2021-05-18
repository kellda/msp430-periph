//! Comparator B

utils::periph! {
    /// Comparator B
    ComparatorB;
    /// Comparator B Control Register 0
    rw CBCTL0 @ 0x00: u16 = 0_0 {
        /// Comp. B Pos. Channel Input Select 0
        CBIPSEL: 0..3 = enum CBIPSEL {
            /// Comp. B V+ terminal Input Select: Channel 0
            CBIPSEL_0 = 0b0000,
            /// Comp. B V+ terminal Input Select: Channel 1
            CBIPSEL_1 = 0b0001,
            /// Comp. B V+ terminal Input Select: Channel 2
            CBIPSEL_2 = 0b0010,
            /// Comp. B V+ terminal Input Select: Channel 3
            CBIPSEL_3 = 0b0011,
            /// Comp. B V+ terminal Input Select: Channel 4
            CBIPSEL_4 = 0b0100,
            /// Comp. B V+ terminal Input Select: Channel 5
            CBIPSEL_5 = 0b0101,
            /// Comp. B V+ terminal Input Select: Channel 6
            CBIPSEL_6 = 0b0110,
            /// Comp. B V+ terminal Input Select: Channel 7
            CBIPSEL_7 = 0b0111,
            /// Comp. B V+ terminal Input Select: Channel 8
            CBIPSEL_8 = 0b1000,
            /// Comp. B V+ terminal Input Select: Channel 9
            CBIPSEL_9 = 0b1001,
            /// Comp. B V+ terminal Input Select: Channel 10
            CBIPSEL_10 = 0b1010,
            /// Comp. B V+ terminal Input Select: Channel 11
            CBIPSEL_11 = 0b1011,
            /// Comp. B V+ terminal Input Select: Channel 12
            CBIPSEL_12 = 0b1100,
            /// Comp. B V+ terminal Input Select: Channel 13
            CBIPSEL_13 = 0b1101,
            /// Comp. B V+ terminal Input Select: Channel 14
            CBIPSEL_14 = 0b1110,
            /// Comp. B V+ terminal Input Select: Channel 15
            CBIPSEL_15 = 0b1111,
        }
        /// Comp. B Pos. Channel Input Enable
        CBIPEN: 7 = struct CBIPEN(bool);
        /// Comp. B Neg. Channel Input Select 0
        CBIMSEL: 8..11 = enum CBIMSEL {
            /// Comp. B V- Terminal Input Select: Channel 0
            CBIMSEL_0 = 0b0000,
            /// Comp. B V- Terminal Input Select: Channel 1
            CBIMSEL_1 = 0b0001,
            /// Comp. B V- Terminal Input Select: Channel 2
            CBIMSEL_2 = 0b0010,
            /// Comp. B V- Terminal Input Select: Channel 3
            CBIMSEL_3 = 0b0011,
            /// Comp. B V- Terminal Input Select: Channel 4
            CBIMSEL_4 = 0b0100,
            /// Comp. B V- Terminal Input Select: Channel 5
            CBIMSEL_5 = 0b0101,
            /// Comp. B V- Terminal Input Select: Channel 6
            CBIMSEL_6 = 0b0110,
            /// Comp. B V- Terminal Input Select: Channel 7
            CBIMSEL_7 = 0b0111,
            /// Comp. B V- terminal Input Select: Channel 8
            CBIMSEL_8 = 0b1000,
            /// Comp. B V- terminal Input Select: Channel 9
            CBIMSEL_9 = 0b1001,
            /// Comp. B V- terminal Input Select: Channel 10
            CBIMSEL_10 = 0b1010,
            /// Comp. B V- terminal Input Select: Channel 11
            CBIMSEL_11 = 0b1011,
            /// Comp. B V- terminal Input Select: Channel 12
            CBIMSEL_12 = 0b1100,
            /// Comp. B V- terminal Input Select: Channel 13
            CBIMSEL_13 = 0b1101,
            /// Comp. B V- terminal Input Select: Channel 14
            CBIMSEL_14 = 0b1110,
            /// Comp. B V- terminal Input Select: Channel 15
            CBIMSEL_15 = 0b1111,
        }
        /// Comp. B Neg. Channel Input Enable
        CBIMEN: 15 = struct CBIMEN(bool);
    }
    /// Comparator B Control Register 1
    rw CBCTL1 @ 0x02: u16 = 0_0 {
        /// Comp. B Output
        CBOUT: 0 = struct CBOUT(bool);
        /// Comp. B Output Polarity
        CBOUTPOL: 1 = struct CBOUTPOL(bool);
        /// Comp. B Enable Output Filter
        CBF: 2 = struct CBF(bool);
        /// Comp. B Interrupt Edge Select
        CBIES: 3 = struct CBIES(bool);
        /// Comp. B Input Short
        CBSHORT: 4 = struct CBSHORT(bool);
        /// Comp. B Exchange Inputs
        CBEX: 5 = struct CBEX(bool);
        /// Comp. B Filter delay Bit 0
        CBFDLY: 6..7 = enum CBFDLY {
            /// Comp. B Filter delay 0 : 450ns
            CBFDLY_0 = 0b00,
            /// Comp. B Filter delay 1 : 900ns
            CBFDLY_1 = 0b01,
            /// Comp. B Filter delay 2 : 1800ns
            CBFDLY_2 = 0b10,
            /// Comp. B Filter delay 3 : 3600ns
            CBFDLY_3 = 0b11,
        }
        /// Comp. B Power Mode Bit 0
        CBPWRMD: 8..9 = enum CBPWRMD {
            /// Comp. B Power Mode 0 : High speed
            CBPWRMD_0 = 0b00,
            /// Comp. B Power Mode 1 : Normal
            CBPWRMD_1 = 0b01,
            /// Comp. B Power Mode 2 : Ultra-Low
            CBPWRMD_2 = 0b10,
            /// Comp. B Power Mode 3 : Reserved
            CBPWRMD_3 = 0b11,
        }
        /// Comp. B enable
        CBON: 10 = struct CBON(bool);
        /// Comp. B CBMRV Level
        CBMRVL: 11 = struct CBMRVL(bool);
        /// Comp. B Output selects between VREF0 or VREF1
        CBMRVS: 12 = struct CBMRVS(bool);
    }
    /// Comparator B Control Register 2
    rw CBCTL2 @ 0x04: u16 = 0_0 {
        /// Comp. B Reference 0 Resistor Select Bit : 0
        CBREF0: 0..4 = enum CBREF0 {
            /// Comp. B Int. Ref.0 Select 0 : 1/32
            CBREF0_0 = 0b00000,
            /// Comp. B Int. Ref.0 Select 1 : 2/32
            CBREF0_1 = 0b00001,
            /// Comp. B Int. Ref.0 Select 2 : 3/32
            CBREF0_2 = 0b00010,
            /// Comp. B Int. Ref.0 Select 3 : 4/32
            CBREF0_3 = 0b00011,
            /// Comp. B Int. Ref.0 Select 4 : 5/32
            CBREF0_4 = 0b00100,
            /// Comp. B Int. Ref.0 Select 5 : 6/32
            CBREF0_5 = 0b00101,
            /// Comp. B Int. Ref.0 Select 6 : 7/32
            CBREF0_6 = 0b00110,
            /// Comp. B Int. Ref.0 Select 7 : 8/32
            CBREF0_7 = 0b00111,
            /// Comp. B Int. Ref.0 Select 0 : 9/32
            CBREF0_8 = 0b01000,
            /// Comp. B Int. Ref.0 Select 1 : 10/32
            CBREF0_9 = 0b01001,
            /// Comp. B Int. Ref.0 Select 2 : 11/32
            CBREF0_10 = 0b01010,
            /// Comp. B Int. Ref.0 Select 3 : 12/32
            CBREF0_11 = 0b01011,
            /// Comp. B Int. Ref.0 Select 4 : 13/32
            CBREF0_12 = 0b01100,
            /// Comp. B Int. Ref.0 Select 5 : 14/32
            CBREF0_13 = 0b01101,
            /// Comp. B Int. Ref.0 Select 6 : 15/32
            CBREF0_14 = 0b01110,
            /// Comp. B Int. Ref.0 Select 7 : 16/32
            CBREF0_15 = 0b01111,
            /// Comp. B Int. Ref.0 Select 0 : 17/32
            CBREF0_16 = 0b10000,
            /// Comp. B Int. Ref.0 Select 1 : 18/32
            CBREF0_17 = 0b10001,
            /// Comp. B Int. Ref.0 Select 2 : 19/32
            CBREF0_18 = 0b10010,
            /// Comp. B Int. Ref.0 Select 3 : 20/32
            CBREF0_19 = 0b10011,
            /// Comp. B Int. Ref.0 Select 4 : 21/32
            CBREF0_20 = 0b10100,
            /// Comp. B Int. Ref.0 Select 5 : 22/32
            CBREF0_21 = 0b10101,
            /// Comp. B Int. Ref.0 Select 6 : 23/32
            CBREF0_22 = 0b10110,
            /// Comp. B Int. Ref.0 Select 7 : 24/32
            CBREF0_23 = 0b10111,
            /// Comp. B Int. Ref.0 Select 0 : 25/32
            CBREF0_24 = 0b11000,
            /// Comp. B Int. Ref.0 Select 1 : 26/32
            CBREF0_25 = 0b11001,
            /// Comp. B Int. Ref.0 Select 2 : 27/32
            CBREF0_26 = 0b11010,
            /// Comp. B Int. Ref.0 Select 3 : 28/32
            CBREF0_27 = 0b11011,
            /// Comp. B Int. Ref.0 Select 4 : 29/32
            CBREF0_28 = 0b11100,
            /// Comp. B Int. Ref.0 Select 5 : 30/32
            CBREF0_29 = 0b11101,
            /// Comp. B Int. Ref.0 Select 6 : 31/32
            CBREF0_30 = 0b11110,
            /// Comp. B Int. Ref.0 Select 7 : 32/32
            CBREF0_31 = 0b11111,
        }
        /// Comp. B Reference select
        CBRSEL: 5 = struct CBRSEL(bool);
        /// Comp. B Reference Source Bit : 0
        CBRS: 6..7 = enum CBRS {
            /// Comp. B Reference Source 0 : Off
            CBRS_0 = 0b00,
            /// Comp. B Reference Source 1 : Vcc
            CBRS_1 = 0b01,
            /// Comp. B Reference Source 2 : Shared Ref.
            CBRS_2 = 0b10,
            /// Comp. B Reference Source 3 : Shared Ref. / Off
            CBRS_3 = 0b11,
        }
        /// Comp. B Reference 1 Resistor Select Bit : 0
        CBREF1: 8..12 = enum CBREF1 {
            /// Comp. B Int. Ref.1 Select 0 : 1/32
            CBREF1_0 = 0b00000,
            /// Comp. B Int. Ref.1 Select 1 : 2/32
            CBREF1_1 = 0b00001,
            /// Comp. B Int. Ref.1 Select 2 : 3/32
            CBREF1_2 = 0b00010,
            /// Comp. B Int. Ref.1 Select 3 : 4/32
            CBREF1_3 = 0b00011,
            /// Comp. B Int. Ref.1 Select 4 : 5/32
            CBREF1_4 = 0b00100,
            /// Comp. B Int. Ref.1 Select 5 : 6/32
            CBREF1_5 = 0b00101,
            /// Comp. B Int. Ref.1 Select 6 : 7/32
            CBREF1_6 = 0b00110,
            /// Comp. B Int. Ref.1 Select 7 : 8/32
            CBREF1_7 = 0b00111,
            /// Comp. B Int. Ref.1 Select 0 : 9/32
            CBREF1_8 = 0b01000,
            /// Comp. B Int. Ref.1 Select 1 : 10/32
            CBREF1_9 = 0b01001,
            /// Comp. B Int. Ref.1 Select 2 : 11/32
            CBREF1_10 = 0b01010,
            /// Comp. B Int. Ref.1 Select 3 : 12/32
            CBREF1_11 = 0b01011,
            /// Comp. B Int. Ref.1 Select 4 : 13/32
            CBREF1_12 = 0b01100,
            /// Comp. B Int. Ref.1 Select 5 : 14/32
            CBREF1_13 = 0b01101,
            /// Comp. B Int. Ref.1 Select 6 : 15/32
            CBREF1_14 = 0b01110,
            /// Comp. B Int. Ref.1 Select 7 : 16/32
            CBREF1_15 = 0b01111,
            /// Comp. B Int. Ref.1 Select 0 : 17/32
            CBREF1_16 = 0b10000,
            /// Comp. B Int. Ref.1 Select 1 : 18/32
            CBREF1_17 = 0b10001,
            /// Comp. B Int. Ref.1 Select 2 : 19/32
            CBREF1_18 = 0b10010,
            /// Comp. B Int. Ref.1 Select 3 : 20/32
            CBREF1_19 = 0b10011,
            /// Comp. B Int. Ref.1 Select 4 : 21/32
            CBREF1_20 = 0b10100,
            /// Comp. B Int. Ref.1 Select 5 : 22/32
            CBREF1_21 = 0b10101,
            /// Comp. B Int. Ref.1 Select 6 : 23/32
            CBREF1_22 = 0b10110,
            /// Comp. B Int. Ref.1 Select 7 : 24/32
            CBREF1_23 = 0b10111,
            /// Comp. B Int. Ref.1 Select 0 : 25/32
            CBREF1_24 = 0b11000,
            /// Comp. B Int. Ref.1 Select 1 : 26/32
            CBREF1_25 = 0b11001,
            /// Comp. B Int. Ref.1 Select 2 : 27/32
            CBREF1_26 = 0b11010,
            /// Comp. B Int. Ref.1 Select 3 : 28/32
            CBREF1_27 = 0b11011,
            /// Comp. B Int. Ref.1 Select 4 : 29/32
            CBREF1_28 = 0b11100,
            /// Comp. B Int. Ref.1 Select 5 : 30/32
            CBREF1_29 = 0b11101,
            /// Comp. B Int. Ref.1 Select 6 : 31/32
            CBREF1_30 = 0b11110,
            /// Comp. B Int. Ref.1 Select 7 : 32/32
            CBREF1_31 = 0b11111,
        }
        /// Comp. B Reference voltage level Bit : 0
        CBREFL: 13..14 = enum CBREFL {
            /// Comp. B Reference voltage level 0 : None
            CBREFL_0 = 0b00,
            /// Comp. B Reference voltage level 1 : 1.5V
            CBREFL_1 = 0b01,
            /// Comp. B Reference voltage level 2 : 2.0V
            CBREFL_2 = 0b10,
            /// Comp. B Reference voltage level 3 : 2.5V
            CBREFL_3 = 0b11,
        }
        /// Comp. B Reference Accuracy
        CBREFACC: 15 = struct CBREFACC(bool);
    }
    /// Comparator B Control Register 3
    rw CBCTL3 @ 0x06: u16 = 0_0 {
        /// Comp. B Disable Input Buffer of Port Register .0
        CBPD0: 0 = struct CBPD0(bool);
        /// Comp. B Disable Input Buffer of Port Register .1
        CBPD1: 1 = struct CBPD1(bool);
        /// Comp. B Disable Input Buffer of Port Register .2
        CBPD2: 2 = struct CBPD2(bool);
        /// Comp. B Disable Input Buffer of Port Register .3
        CBPD3: 3 = struct CBPD3(bool);
        /// Comp. B Disable Input Buffer of Port Register .4
        CBPD4: 4 = struct CBPD4(bool);
        /// Comp. B Disable Input Buffer of Port Register .5
        CBPD5: 5 = struct CBPD5(bool);
        /// Comp. B Disable Input Buffer of Port Register .6
        CBPD6: 6 = struct CBPD6(bool);
        /// Comp. B Disable Input Buffer of Port Register .7
        CBPD7: 7 = struct CBPD7(bool);
        /// Comp. B Disable Input Buffer of Port Register .8
        CBPD8: 8 = struct CBPD8(bool);
        /// Comp. B Disable Input Buffer of Port Register .9
        CBPD9: 9 = struct CBPD9(bool);
        /// Comp. B Disable Input Buffer of Port Register .10
        CBPD10: 10 = struct CBPD10(bool);
        /// Comp. B Disable Input Buffer of Port Register .11
        CBPD11: 11 = struct CBPD11(bool);
        /// Comp. B Disable Input Buffer of Port Register .12
        CBPD12: 12 = struct CBPD12(bool);
        /// Comp. B Disable Input Buffer of Port Register .13
        CBPD13: 13 = struct CBPD13(bool);
        /// Comp. B Disable Input Buffer of Port Register .14
        CBPD14: 14 = struct CBPD14(bool);
        /// Comp. B Disable Input Buffer of Port Register .15
        CBPD15: 15 = struct CBPD15(bool);
    }
    /// Comparator B Interrupt Register
    rw CBINT @ 0x0c: u16 = 0_0 {
        /// Comp. B Interrupt Flag
        CBIFG: 0 = struct CBIFG(bool);
        /// Comp. B Interrupt Flag Inverted Polarity
        CBIIFG: 1 = struct CBIIFG(bool);
        /// Comp. B Interrupt Enable
        CBIE: 8 = struct CBIE(bool);
        /// Comp. B Interrupt Enable Inverted Polarity
        CBIIE: 9 = struct CBIIE(bool);
    }
    /// Comparator B Interrupt Vector Word
    rw CBIV @ 0x0e: u16 = 0_0 {
        /// Comparator B Interrupt Vector Word
        CBIV: 0..15 = struct CBIVField(u16);
    }
}
