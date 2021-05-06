//! COMP_E

utils::periph! {
    /// COMP_E
    COMP_E;
    /// Comparator Control Register 0
    rw CECTL0 @ 0x00: u16 = 0_0 {
        /// Channel input selected for the V+ terminal
        CEIPSEL: 0..3 = enum CEIPSEL {
            /// Channel 0 selected
            CEIPSEL_0 = 0b0000,
            /// Channel 1 selected
            CEIPSEL_1 = 0b0001,
            /// Channel 2 selected
            CEIPSEL_2 = 0b0010,
            /// Channel 3 selected
            CEIPSEL_3 = 0b0011,
            /// Channel 4 selected
            CEIPSEL_4 = 0b0100,
            /// Channel 5 selected
            CEIPSEL_5 = 0b0101,
            /// Channel 6 selected
            CEIPSEL_6 = 0b0110,
            /// Channel 7 selected
            CEIPSEL_7 = 0b0111,
            /// Channel 8 selected
            CEIPSEL_8 = 0b1000,
            /// Channel 9 selected
            CEIPSEL_9 = 0b1001,
            /// Channel 10 selected
            CEIPSEL_10 = 0b1010,
            /// Channel 11 selected
            CEIPSEL_11 = 0b1011,
            /// Channel 12 selected
            CEIPSEL_12 = 0b1100,
            /// Channel 13 selected
            CEIPSEL_13 = 0b1101,
            /// Channel 14 selected
            CEIPSEL_14 = 0b1110,
            /// Channel 15 selected
            CEIPSEL_15 = 0b1111,
        }
        /// Channel input enable for the V+ terminal
        CEIPEN: 7..7 = enum CEIPEN {
            /// Selected analog input channel for V+ terminal is disabled
            DISABLE = 0b0,
            /// Selected analog input channel for V+ terminal is enabled
            ENABLE = 0b1,
        }
        /// Channel input selected for the - terminal
        CEIMSEL: 8..11 = enum CEIMSEL {
            /// Channel 0 selected
            CEIMSEL_0 = 0b0000,
            /// Channel 1 selected
            CEIMSEL_1 = 0b0001,
            /// Channel 2 selected
            CEIMSEL_2 = 0b0010,
            /// Channel 3 selected
            CEIMSEL_3 = 0b0011,
            /// Channel 4 selected
            CEIMSEL_4 = 0b0100,
            /// Channel 5 selected
            CEIMSEL_5 = 0b0101,
            /// Channel 6 selected
            CEIMSEL_6 = 0b0110,
            /// Channel 7 selected
            CEIMSEL_7 = 0b0111,
            /// Channel 8 selected
            CEIMSEL_8 = 0b1000,
            /// Channel 9 selected
            CEIMSEL_9 = 0b1001,
            /// Channel 10 selected
            CEIMSEL_10 = 0b1010,
            /// Channel 11 selected
            CEIMSEL_11 = 0b1011,
            /// Channel 12 selected
            CEIMSEL_12 = 0b1100,
            /// Channel 13 selected
            CEIMSEL_13 = 0b1101,
            /// Channel 14 selected
            CEIMSEL_14 = 0b1110,
            /// Channel 15 selected
            CEIMSEL_15 = 0b1111,
        }
        /// Channel input enable for the - terminal
        CEIMEN: 15..15 = enum CEIMEN {
            /// Selected analog input channel for V- terminal is disabled
            DISABLE = 0b0,
            /// Selected analog input channel for V- terminal is enabled
            ENABLE = 0b1,
        }
    }
    /// Comparator Control Register 1
    rw CECTL1 @ 0x02: u16 = 0_0 {
        /// Comparator output value
        CEOUT: 0 = struct CEOUT(bool);
        /// Comparator output polarity
        CEOUTPOL: 1..1 = enum CEOUTPOL {
            /// Noninverted
            CEOUTPOL_0 = 0b0,
            /// Inverted
            CEOUTPOL_1 = 0b1,
        }
        /// Comparator output filter
        CEF: 2..2 = enum CEF {
            /// Comparator output is not filtered
            CEF_0 = 0b0,
            /// Comparator output is filtered
            CEF_1 = 0b1,
        }
        /// Interrupt edge select for CEIIFG and CEIFG
        CEIES: 3..3 = enum CEIES {
            /// Rising edge for CEIFG, falling edge for CEIIFG
            CEIES_0 = 0b0,
            /// Falling edge for CEIFG, rising edge for CEIIFG
            CEIES_1 = 0b1,
        }
        /// Input short
        CESHORT: 4..4 = enum CESHORT {
            /// Inputs not shorted
            CESHORT_0 = 0b0,
            /// Inputs shorted
            CESHORT_1 = 0b1,
        }
        /// Exchange
        CEEX: 5 = struct CEEX(bool);
        /// Filter delay
        CEFDLY: 6..7 = enum CEFDLY {
            /// Typical filter delay of TBD (450) ns
            CEFDLY_0 = 0b00,
            /// Typical filter delay of TBD (900) ns
            CEFDLY_1 = 0b01,
            /// Typical filter delay of TBD (1800) ns
            CEFDLY_2 = 0b10,
            /// Typical filter delay of TBD (3600) ns
            CEFDLY_3 = 0b11,
        }
        /// Power Mode
        CEPWRMD: 8..9 = enum CEPWRMD {
            /// High-speed mode
            CEPWRMD_0 = 0b00,
            /// Normal mode
            CEPWRMD_1 = 0b01,
            /// Ultra-low power mode
            CEPWRMD_2 = 0b10,
        }
        /// Comparator On
        CEON: 10..10 = enum CEON {
            /// Off
            OFF = 0b0,
            /// On
            ON = 0b1,
        }
        /// This bit is valid of CEMRVS is set to 1
        CEMRVL: 11..11 = enum CEMRVL {
            /// VREF0 is selected if CERS = 00, 01, or 10
            VREF0 = 0b0,
            /// VREF1 is selected if CERS = 00, 01, or 10
            VREF1 = 0b1,
        }
        /// This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10.
        CEMRVS: 12..12 = enum CEMRVS {
            /// Comparator output state selects between VREF0 or VREF1
            CEMRVS_0 = 0b0,
            /// CEMRVL selects between VREF0 or VREF1
            CEMRVS_1 = 0b1,
        }
    }
    /// Comparator Control Register 2
    rw CECTL2 @ 0x04: u16 = 0_0 {
        /// Reference resistor tap 0
        CEREF0: 0..4 = struct CEREF0(u16);
        /// Reference select
        CERSEL: 5..5 = enum CERSEL {
            /// When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal
            CERSEL_0 = 0b0,
            /// When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal
            CERSEL_1 = 0b1,
        }
        /// Reference source
        CERS: 6..7 = enum CERS {
            /// No current is drawn by the reference circuitry
            CERS_0 = 0b00,
            /// VCC applied to the resistor ladder
            CERS_1 = 0b01,
            /// Shared reference voltage applied to the resistor ladder
            CERS_2 = 0b10,
            /// Shared reference voltage supplied to V(CREF). Resistor ladder is off
            CERS_3 = 0b11,
        }
        /// Reference resistor tap 1
        CEREF1: 8..12 = struct CEREF1(u16);
        /// Reference voltage level
        CEREFL: 13..14 = enum CEREFL {
            /// Reference amplifier is disabled. No reference voltage is requested
            OFF = 0b00,
            /// 1.2 V is selected as shared reference voltage input
            _1P2V = 0b01,
            /// 2.0 V is selected as shared reference voltage input
            _2P0V = 0b10,
            /// 2.5 V is selected as shared reference voltage input
            _2P5V = 0b11,
        }
        /// Reference accuracy
        CEREFACC: 15..15 = enum CEREFACC {
            /// Static mode
            STATIC = 0b0,
            /// Clocked (low power, low accuracy) mode
            CLOCKED = 0b1,
        }
    }
    /// Comparator Control Register 3
    rw CECTL3 @ 0x06: u16 = 0_0 {
        /// Port disable
        CEPD0: 0..0 = enum CEPD0 {
            /// The input buffer is enabled
            CEPD0_0 = 0b0,
            /// The input buffer is disabled
            CEPD0_1 = 0b1,
        }
        /// Port disable
        CEPD1: 1..1 = enum CEPD1 {
            /// The input buffer is enabled
            CEPD1_0 = 0b0,
            /// The input buffer is disabled
            CEPD1_1 = 0b1,
        }
        /// Port disable
        CEPD2: 2..2 = enum CEPD2 {
            /// The input buffer is enabled
            CEPD2_0 = 0b0,
            /// The input buffer is disabled
            CEPD2_1 = 0b1,
        }
        /// Port disable
        CEPD3: 3..3 = enum CEPD3 {
            /// The input buffer is enabled
            CEPD3_0 = 0b0,
            /// The input buffer is disabled
            CEPD3_1 = 0b1,
        }
        /// Port disable
        CEPD4: 4..4 = enum CEPD4 {
            /// The input buffer is enabled
            CEPD4_0 = 0b0,
            /// The input buffer is disabled
            CEPD4_1 = 0b1,
        }
        /// Port disable
        CEPD5: 5..5 = enum CEPD5 {
            /// The input buffer is enabled
            CEPD5_0 = 0b0,
            /// The input buffer is disabled
            CEPD5_1 = 0b1,
        }
        /// Port disable
        CEPD6: 6..6 = enum CEPD6 {
            /// The input buffer is enabled
            CEPD6_0 = 0b0,
            /// The input buffer is disabled
            CEPD6_1 = 0b1,
        }
        /// Port disable
        CEPD7: 7..7 = enum CEPD7 {
            /// The input buffer is enabled
            CEPD7_0 = 0b0,
            /// The input buffer is disabled
            CEPD7_1 = 0b1,
        }
        /// Port disable
        CEPD8: 8..8 = enum CEPD8 {
            /// The input buffer is enabled
            CEPD8_0 = 0b0,
            /// The input buffer is disabled
            CEPD8_1 = 0b1,
        }
        /// Port disable
        CEPD9: 9..9 = enum CEPD9 {
            /// The input buffer is enabled
            CEPD9_0 = 0b0,
            /// The input buffer is disabled
            CEPD9_1 = 0b1,
        }
        /// Port disable
        CEPD10: 10..10 = enum CEPD10 {
            /// The input buffer is enabled
            CEPD10_0 = 0b0,
            /// The input buffer is disabled
            CEPD10_1 = 0b1,
        }
        /// Port disable
        CEPD11: 11..11 = enum CEPD11 {
            /// The input buffer is enabled
            CEPD11_0 = 0b0,
            /// The input buffer is disabled
            CEPD11_1 = 0b1,
        }
        /// Port disable
        CEPD12: 12..12 = enum CEPD12 {
            /// The input buffer is enabled
            CEPD12_0 = 0b0,
            /// The input buffer is disabled
            CEPD12_1 = 0b1,
        }
        /// Port disable
        CEPD13: 13..13 = enum CEPD13 {
            /// The input buffer is enabled
            CEPD13_0 = 0b0,
            /// The input buffer is disabled
            CEPD13_1 = 0b1,
        }
        /// Port disable
        CEPD14: 14..14 = enum CEPD14 {
            /// The input buffer is enabled
            CEPD14_0 = 0b0,
            /// The input buffer is disabled
            CEPD14_1 = 0b1,
        }
        /// Port disable
        CEPD15: 15..15 = enum CEPD15 {
            /// The input buffer is enabled
            CEPD15_0 = 0b0,
            /// The input buffer is disabled
            CEPD15_1 = 0b1,
        }
    }
    /// Comparator Interrupt Control Register
    rw CEINT @ 0x0c: u16 = 0_0 {
        /// Comparator output interrupt flag
        CEIFG: 0..0 = enum CEIFG {
            /// No interrupt pending
            CEIFG_0 = 0b0,
            /// Interrupt pending
            CEIFG_1 = 0b1,
        }
        /// Comparator output inverted interrupt flag
        CEIIFG: 1..1 = enum CEIIFG {
            /// No interrupt pending
            CEIIFG_0 = 0b0,
            /// Interrupt pending
            CEIIFG_1 = 0b1,
        }
        /// Comparator ready interrupt flag
        CERDYIFG: 4..4 = enum CERDYIFG {
            /// No interrupt pending
            CERDYIFG_0 = 0b0,
            /// Interrupt pending
            CERDYIFG_1 = 0b1,
        }
        /// Comparator output interrupt enable
        CEIE: 8..8 = enum CEIE {
            /// Interrupt disabled
            DISABLE = 0b0,
            /// Interrupt enabled
            ENABLE = 0b1,
        }
        /// Comparator output interrupt enable inverted polarity
        CEIIE: 9..9 = enum CEIIE {
            /// Interrupt disabled
            DISABLE = 0b0,
            /// Interrupt enabled
            ENABLE = 0b1,
        }
        /// Comparator ready interrupt enable
        CERDYIE: 12..12 = enum CERDYIE {
            /// Interrupt disabled
            DISABLE = 0b0,
            /// Interrupt enabled
            ENABLE = 0b1,
        }
    }
    /// Comparator Interrupt Vector Word Register
    r CEIV @ 0x0e: u16 = 0_0 {
        /// Comparator interrupt vector word register
        CEIV: 0..15 = enum CEIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest
            CEIFG = 0b0000000000000010,
            /// Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG
            CEIIFG = 0b0000000000000100,
            /// Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest
            CERDYIFG = 0b0000000000001010,
        }
    }
}
