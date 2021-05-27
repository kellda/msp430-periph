//! COMP_E

utils::periph! {
    /// COMP_E
    COMP_E;
    /// Comparator Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// Channel input selected for the V+ terminal
        IPSEL: 0..3 = enum IPSEL {
            /// Channel 0 selected
            IPSEL_0 = 0b0000,
            /// Channel 1 selected
            IPSEL_1 = 0b0001,
            /// Channel 2 selected
            IPSEL_2 = 0b0010,
            /// Channel 3 selected
            IPSEL_3 = 0b0011,
            /// Channel 4 selected
            IPSEL_4 = 0b0100,
            /// Channel 5 selected
            IPSEL_5 = 0b0101,
            /// Channel 6 selected
            IPSEL_6 = 0b0110,
            /// Channel 7 selected
            IPSEL_7 = 0b0111,
            /// Channel 8 selected
            IPSEL_8 = 0b1000,
            /// Channel 9 selected
            IPSEL_9 = 0b1001,
            /// Channel 10 selected
            IPSEL_10 = 0b1010,
            /// Channel 11 selected
            IPSEL_11 = 0b1011,
            /// Channel 12 selected
            IPSEL_12 = 0b1100,
            /// Channel 13 selected
            IPSEL_13 = 0b1101,
            /// Channel 14 selected
            IPSEL_14 = 0b1110,
            /// Channel 15 selected
            IPSEL_15 = 0b1111,
        }
        /// Channel input enable for the V+ terminal
        IPEN: 7 = enum IPEN {
            /// Selected analog input channel for V+ terminal is disabled
            DISABLE = 0b0,
            /// Selected analog input channel for V+ terminal is enabled
            ENABLE = 0b1,
        }
        /// Channel input selected for the - terminal
        IMSEL: 8..11 = enum IMSEL {
            /// Channel 0 selected
            IMSEL_0 = 0b0000,
            /// Channel 1 selected
            IMSEL_1 = 0b0001,
            /// Channel 2 selected
            IMSEL_2 = 0b0010,
            /// Channel 3 selected
            IMSEL_3 = 0b0011,
            /// Channel 4 selected
            IMSEL_4 = 0b0100,
            /// Channel 5 selected
            IMSEL_5 = 0b0101,
            /// Channel 6 selected
            IMSEL_6 = 0b0110,
            /// Channel 7 selected
            IMSEL_7 = 0b0111,
            /// Channel 8 selected
            IMSEL_8 = 0b1000,
            /// Channel 9 selected
            IMSEL_9 = 0b1001,
            /// Channel 10 selected
            IMSEL_10 = 0b1010,
            /// Channel 11 selected
            IMSEL_11 = 0b1011,
            /// Channel 12 selected
            IMSEL_12 = 0b1100,
            /// Channel 13 selected
            IMSEL_13 = 0b1101,
            /// Channel 14 selected
            IMSEL_14 = 0b1110,
            /// Channel 15 selected
            IMSEL_15 = 0b1111,
        }
        /// Channel input enable for the - terminal
        IMEN: 15 = enum IMEN {
            /// Selected analog input channel for V- terminal is disabled
            DISABLE = 0b0,
            /// Selected analog input channel for V- terminal is enabled
            ENABLE = 0b1,
        }
    }
    /// Comparator Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// Comparator output value
        OUT: 0 = struct OUT(bool);
        /// Comparator output polarity
        OUTPOL: 1 = enum OUTPOL {
            /// Noninverted
            OUTPOL_0 = 0b0,
            /// Inverted
            OUTPOL_1 = 0b1,
        }
        /// Comparator output filter
        F: 2 = enum F {
            /// Comparator output is not filtered
            F_0 = 0b0,
            /// Comparator output is filtered
            F_1 = 0b1,
        }
        /// Interrupt edge select for CEIIFG and CEIFG
        IES: 3 = enum IES {
            /// Rising edge for CEIFG, falling edge for CEIIFG
            IES_0 = 0b0,
            /// Falling edge for CEIFG, rising edge for CEIIFG
            IES_1 = 0b1,
        }
        /// Input short
        SHORT: 4 = enum SHORT {
            /// Inputs not shorted
            SHORT_0 = 0b0,
            /// Inputs shorted
            SHORT_1 = 0b1,
        }
        /// Exchange
        EX: 5 = struct EX(bool);
        /// Filter delay
        FDLY: 6..7 = enum FDLY {
            /// Typical filter delay of TBD (450) ns
            FDLY_0 = 0b00,
            /// Typical filter delay of TBD (900) ns
            FDLY_1 = 0b01,
            /// Typical filter delay of TBD (1800) ns
            FDLY_2 = 0b10,
            /// Typical filter delay of TBD (3600) ns
            FDLY_3 = 0b11,
        }
        /// Power Mode
        PWRMD: 8..9 = enum PWRMD {
            /// High-speed mode
            PWRMD_0 = 0b00,
            /// Normal mode
            PWRMD_1 = 0b01,
            /// Ultra-low power mode
            PWRMD_2 = 0b10,
        }
        /// Comparator On
        ON: 10 = enum ON {
            /// Off
            OFF = 0b0,
            /// On
            ON = 0b1,
        }
        /// This bit is valid of CEMRVS is set to 1
        MRVL: 11 = enum MRVL {
            /// VREF0 is selected if CERS = 00, 01, or 10
            VREF0 = 0b0,
            /// VREF1 is selected if CERS = 00, 01, or 10
            VREF1 = 0b1,
        }
        /// This bit defines if the comparator output selects between VREF0 or VREF1 if CERS = 00, 01, or 10.
        MRVS: 12 = enum MRVS {
            /// Comparator output state selects between VREF0 or VREF1
            MRVS_0 = 0b0,
            /// CEMRVL selects between VREF0 or VREF1
            MRVS_1 = 0b1,
        }
    }
    /// Comparator Control Register 2
    rw CTL2 @ 0x04: u16 = 0_0 {
        /// Reference resistor tap 0
        REF0: 0..4 = struct REF0(u16);
        /// Reference select
        RSEL: 5 = enum RSEL {
            /// When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal
            RSEL_0 = 0b0,
            /// When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal
            RSEL_1 = 0b1,
        }
        /// Reference source
        RS: 6..7 = enum RS {
            /// No current is drawn by the reference circuitry
            RS_0 = 0b00,
            /// VCC applied to the resistor ladder
            RS_1 = 0b01,
            /// Shared reference voltage applied to the resistor ladder
            RS_2 = 0b10,
            /// Shared reference voltage supplied to V(CREF). Resistor ladder is off
            RS_3 = 0b11,
        }
        /// Reference resistor tap 1
        REF1: 8..12 = struct REF1(u16);
        /// Reference voltage level
        REFL: 13..14 = enum REFL {
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
        REFACC: 15 = enum REFACC {
            /// Static mode
            STATIC = 0b0,
            /// Clocked (low power, low accuracy) mode
            CLOCKED = 0b1,
        }
    }
    /// Comparator Control Register 3
    rw CTL3 @ 0x06: u16 = 0_0 {
        /// Port disable
        PD0: 0 = enum PD0 {
            /// The input buffer is enabled
            PD0_0 = 0b0,
            /// The input buffer is disabled
            PD0_1 = 0b1,
        }
        /// Port disable
        PD1: 1 = enum PD1 {
            /// The input buffer is enabled
            PD1_0 = 0b0,
            /// The input buffer is disabled
            PD1_1 = 0b1,
        }
        /// Port disable
        PD2: 2 = enum PD2 {
            /// The input buffer is enabled
            PD2_0 = 0b0,
            /// The input buffer is disabled
            PD2_1 = 0b1,
        }
        /// Port disable
        PD3: 3 = enum PD3 {
            /// The input buffer is enabled
            PD3_0 = 0b0,
            /// The input buffer is disabled
            PD3_1 = 0b1,
        }
        /// Port disable
        PD4: 4 = enum PD4 {
            /// The input buffer is enabled
            PD4_0 = 0b0,
            /// The input buffer is disabled
            PD4_1 = 0b1,
        }
        /// Port disable
        PD5: 5 = enum PD5 {
            /// The input buffer is enabled
            PD5_0 = 0b0,
            /// The input buffer is disabled
            PD5_1 = 0b1,
        }
        /// Port disable
        PD6: 6 = enum PD6 {
            /// The input buffer is enabled
            PD6_0 = 0b0,
            /// The input buffer is disabled
            PD6_1 = 0b1,
        }
        /// Port disable
        PD7: 7 = enum PD7 {
            /// The input buffer is enabled
            PD7_0 = 0b0,
            /// The input buffer is disabled
            PD7_1 = 0b1,
        }
        /// Port disable
        PD8: 8 = enum PD8 {
            /// The input buffer is enabled
            PD8_0 = 0b0,
            /// The input buffer is disabled
            PD8_1 = 0b1,
        }
        /// Port disable
        PD9: 9 = enum PD9 {
            /// The input buffer is enabled
            PD9_0 = 0b0,
            /// The input buffer is disabled
            PD9_1 = 0b1,
        }
        /// Port disable
        PD10: 10 = enum PD10 {
            /// The input buffer is enabled
            PD10_0 = 0b0,
            /// The input buffer is disabled
            PD10_1 = 0b1,
        }
        /// Port disable
        PD11: 11 = enum PD11 {
            /// The input buffer is enabled
            PD11_0 = 0b0,
            /// The input buffer is disabled
            PD11_1 = 0b1,
        }
        /// Port disable
        PD12: 12 = enum PD12 {
            /// The input buffer is enabled
            PD12_0 = 0b0,
            /// The input buffer is disabled
            PD12_1 = 0b1,
        }
        /// Port disable
        PD13: 13 = enum PD13 {
            /// The input buffer is enabled
            PD13_0 = 0b0,
            /// The input buffer is disabled
            PD13_1 = 0b1,
        }
        /// Port disable
        PD14: 14 = enum PD14 {
            /// The input buffer is enabled
            PD14_0 = 0b0,
            /// The input buffer is disabled
            PD14_1 = 0b1,
        }
        /// Port disable
        PD15: 15 = enum PD15 {
            /// The input buffer is enabled
            PD15_0 = 0b0,
            /// The input buffer is disabled
            PD15_1 = 0b1,
        }
    }
    /// Comparator Interrupt Control Register
    rw INT @ 0x0c: u16 = 0_0 {
        /// Comparator output interrupt flag
        IFG: 0 = enum IFG {
            /// No interrupt pending
            IFG_0 = 0b0,
            /// Interrupt pending
            IFG_1 = 0b1,
        }
        /// Comparator output inverted interrupt flag
        IIFG: 1 = enum IIFG {
            /// No interrupt pending
            IIFG_0 = 0b0,
            /// Interrupt pending
            IIFG_1 = 0b1,
        }
        /// Comparator ready interrupt flag
        RDYIFG: 4 = enum RDYIFG {
            /// No interrupt pending
            RDYIFG_0 = 0b0,
            /// Interrupt pending
            RDYIFG_1 = 0b1,
        }
        /// Comparator output interrupt enable
        IE: 8 = enum IE {
            /// Interrupt disabled
            DISABLE = 0b0,
            /// Interrupt enabled
            ENABLE = 0b1,
        }
        /// Comparator output interrupt enable inverted polarity
        IIE: 9 = enum IIE {
            /// Interrupt disabled
            DISABLE = 0b0,
            /// Interrupt enabled
            ENABLE = 0b1,
        }
        /// Comparator ready interrupt enable
        RDYIE: 12 = enum RDYIE {
            /// Interrupt disabled
            DISABLE = 0b0,
            /// Interrupt enabled
            ENABLE = 0b1,
        }
    }
    /// Comparator Interrupt Vector Word Register
    r IV @ 0x0e: u16 = 0_0 {
        /// Comparator interrupt vector word register
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest
            IFG = 0b0000000000000010,
            /// Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG
            IIFG = 0b0000000000000100,
            /// Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest
            RDYIFG = 0b0000000000001010,
        }
    }
}
