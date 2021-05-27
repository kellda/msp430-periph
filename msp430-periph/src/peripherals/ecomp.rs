//! eCOMP

utils::periph! {
    /// eCOMP
    eCOMP;
    /// Comparator Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// Channel input enable for the V+ terminal
        PEN: 4 = enum PEN {
            /// Selected analog input channel for V+ terminal is disabled.
            PEN_0 = 0b0,
            /// Selected analog input channel for V+ terminal is enabled.
            PEN_1 = 0b1,
        }
        /// Channel input selected for the - terminal
        NSEL: 8..10 = enum NSEL {
            /// select external input source
            NSEL_0 = 0b000,
            /// select external input source
            NSEL_1 = 0b001,
            /// select external input source
            NSEL_2 = 0b010,
            /// select external input source
            NSEL_3 = 0b011,
            /// device specific, please refer to device data sheet for details
            NSEL_4 = 0b100,
            /// device specific, please refer to device data sheet for details
            NSEL_5 = 0b101,
            /// 6-bit DAC
            NSEL_6 = 0b110,
            /// Reserved
            NSEL_7 = 0b111,
        }
        /// Channel input enable for the - terminal
        NEN: 12 = enum NEN {
            /// Selected analog input channel for V- terminal is disabled.
            NEN_0 = 0b0,
            /// Selected analog input channel for V- terminal is enabled.
            NEN_1 = 0b1,
        }
        /// Channel input selected for the V+ terminal
        PSEL: 0..2 = enum PSEL {
            /// select external input source
            PSEL_0 = 0b000,
            /// select external input source
            PSEL_1 = 0b001,
            /// select external input source
            PSEL_2 = 0b010,
            /// select external input source
            PSEL_3 = 0b011,
            /// device specific, please refer to device data sheet for details
            PSEL_4 = 0b100,
            /// device specific, please refer to device data sheet for details
            PSEL_5 = 0b101,
            /// 6-bit DAC
            PSEL_6 = 0b110,
            /// Reserved
            PSEL_7 = 0b111,
        }
    }
    /// Comparator Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// Comparator output value
        OUT: 0 = struct OUT(bool);
        /// Comparator output polarity
        INV: 1 = enum INV {
            /// Comparator output is non-inverted
            INV_0 = 0b0,
            /// Comparator output is inverted
            INV_1 = 0b1,
        }
        /// Interrupt edge select for CEIIFG and CEIFG
        IES: 4 = enum IES {
            /// Rising edge for CPIFG, falling edge for CPIIFG
            IES_0 = 0b0,
            /// Falling edge for CPIFG, rising edge for CPIIFG
            IES_1 = 0b1,
        }
        /// Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag.
        FLT: 5 = enum FLT {
            /// Comparator output is not filtered
            FLT_0 = 0b0,
            /// Comparator output is filtered
            FLT_1 = 0b1,
        }
        /// Analog Filter Delay selection. These bits are used to select the analog filter delay
        FLTDLY: 6..7 = enum FLTDLY {
            /// Typical filter delay of 450ns
            FLTDLY_0 = 0b00,
            /// Typical filter delay of 900ns
            FLTDLY_1 = 0b01,
            /// Typical filter delay of 1800ns
            FLTDLY_2 = 0b10,
            /// Typical filter delay of 3600ns
            FLTDLY_3 = 0b11,
        }
        /// Power mode selection.
        MSEL: 8 = enum MSEL {
            /// High-power & High speed mode (500nA)
            MSEL_0 = 0b0,
            /// Low-power & Low speed mode (10nA)
            MSEL_1 = 0b1,
        }
        /// Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power.
        EN: 9 = enum EN {
            /// Comparator is disabled
            EN_0 = 0b0,
            /// Comparator is enabled
            EN_1 = 0b1,
        }
        /// Programable Hysteresis mode. These bits are used to select the Hysteresis mode.
        HSEL: 10..11 = enum HSEL {
            /// disable
            HSEL_0 = 0b00,
            /// 10mV
            HSEL_1 = 0b01,
            /// 20mV
            HSEL_2 = 0b10,
            /// 30mV
            HSEL_3 = 0b11,
        }
        /// Comparator interrupt output enable bit
        IE: 14 = enum IE {
            /// Interrupt output is disabled
            IE_0 = 0b0,
            /// Interrupt output is enabled
            IE_1 = 0b1,
        }
        /// Comparator inverted interrupt output enable bit
        IIE: 15 = enum IIE {
            /// Interrupt inverted output is disabled
            IIE_0 = 0b0,
            /// Interrupt inverted output is enabled
            IIE_1 = 0b1,
        }
    }
    /// Comparator Interrupt Control Register
    rw INT @ 0x06: u16 = 0_0 {
        /// Comparator output interrupt flag
        IFG: 0 = enum IFG {
            /// No interrupt pending.
            IFG_0 = 0b0,
            /// Output interrupt pending.
            IFG_1 = 0b1,
        }
        /// Comparator output inverted interrupt flag
        IIFG: 1 = enum IIFG {
            /// No interrupt pending.
            IIFG_0 = 0b0,
            /// Output interrupt pending.
            IIFG_1 = 0b1,
        }
    }
    /// Comparator Interrupt Vector Word Register
    r IV @ 0x08: u16 = 0_0 {
        /// Comparator interrupt vector word register
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// CPIFG
            IFG = 0b0000000000000010,
            /// CPIIFG
            IIFG = 0b0000000000000100,
        }
    }
    /// 6-bit Comparator built-in DAC Control Register
    rw DACCTL @ 0x10: u16 = 0_0 {
        /// This bit is only valid when DACBUFS is set to 1.
        DACSW: 0 = enum DACSW {
            /// CPDACBUF1 selected
            DACSW_0 = 0b0,
            /// CPDACBUF2 selected
            DACSW_1 = 0b1,
        }
        /// Comparator built-in DAC buffer controlled source selection.
        DACBUFS: 1 = enum DACBUFS {
            /// Comparator output is selected as the buffer control source
            DACBUFS_0 = 0b0,
            /// CPDACSW bit is selected as the buffer control source
            DACBUFS_1 = 0b1,
        }
        /// Comparator built-in DAC reference voltage selection
        DACREFS: 2 = enum DACREFS {
            /// VDD selected
            DACREFS_0 = 0b0,
            /// on-chip VREF selected
            DACREFS_1 = 0b1,
        }
        /// Comparator built-in DAC output control bit.
        DACEN: 7 = enum DACEN {
            /// DAC output is disabled.
            DACEN_0 = 0b0,
            /// DAC output is enabled.
            DACEN_1 = 0b1,
        }
    }
    /// 6-bit Comparator built-in DAC Data Register
    rw DACDATA @ 0x12: u16 = 0_0 {
        /// 1st 6-bit DAC buffer Data
        DACBUF1: 0..5 = enum DACBUF1 {
            /// 0v
            DACBUF1_0 = 0b000000,
            /// selected reference voltage * 1/64
            DACBUF1_1 = 0b000001,
            /// selected reference voltage * 2/64
            DACBUF1_2 = 0b000010,
            /// selected reference voltage * 3/64
            DACBUF1_3 = 0b000011,
            /// selected reference voltage * 4/64
            DACBUF1_4 = 0b000100,
            /// selected reference voltage * 5/64
            DACBUF1_5 = 0b000101,
            /// selected reference voltage * 6/64
            DACBUF1_6 = 0b000110,
            /// selected reference voltage * 7/64
            DACBUF1_7 = 0b000111,
            /// selected reference voltage * 8/64
            DACBUF1_8 = 0b001000,
            /// selected reference voltage *9/64
            DACBUF1_9 = 0b001001,
            /// selected reference voltage * 10/64
            DACBUF1_10 = 0b001010,
            /// selected reference voltage * 11/64
            DACBUF1_11 = 0b001011,
            /// selected reference voltage * 12/64
            DACBUF1_12 = 0b001100,
            /// selected reference voltage * 13/64
            DACBUF1_13 = 0b001101,
            /// selected reference voltage * 14/64
            DACBUF1_14 = 0b001110,
            /// selected reference voltage * 15/64
            DACBUF1_15 = 0b001111,
            /// selected reference voltage * 16/64
            DACBUF1_16 = 0b010000,
            /// selected reference voltage * 17/64
            DACBUF1_17 = 0b010001,
            /// selected reference voltage * 18/64
            DACBUF1_18 = 0b010010,
            /// selected reference voltage * 19/64
            DACBUF1_19 = 0b010011,
            /// selected reference voltage * 20/64
            DACBUF1_20 = 0b010100,
            /// selected reference voltage * 21/64
            DACBUF1_21 = 0b010101,
            /// selected reference voltage * 22/64
            DACBUF1_22 = 0b010110,
            /// selected reference voltage * 23/64
            DACBUF1_23 = 0b010111,
            /// selected reference voltage * 24/64
            DACBUF1_24 = 0b011000,
            /// selected reference voltage * 25/64
            DACBUF1_25 = 0b011001,
            /// selected reference voltage * 26/64
            DACBUF1_26 = 0b011010,
            /// selected reference voltage * 27/64
            DACBUF1_27 = 0b011011,
            /// selected reference voltage * 28/64
            DACBUF1_28 = 0b011100,
            /// selected reference voltage * 29/64
            DACBUF1_29 = 0b011101,
            /// selected reference voltage * 30/64
            DACBUF1_30 = 0b011110,
            /// selected reference voltage * 31/64
            DACBUF1_31 = 0b011111,
            /// selected reference voltage * 32/64
            DACBUF1_32 = 0b100000,
            /// selected reference voltage * 33/64
            DACBUF1_33 = 0b100001,
            /// selected reference voltage * 34/64
            DACBUF1_34 = 0b100010,
            /// selected reference voltage * 35/64
            DACBUF1_35 = 0b100011,
            /// selected reference voltage * 36/64
            DACBUF1_36 = 0b100100,
            /// selected reference voltage * 37/64
            DACBUF1_37 = 0b100101,
            /// selected reference voltage * 38/64
            DACBUF1_38 = 0b100110,
            /// selected reference voltage * 39/64
            DACBUF1_39 = 0b100111,
            /// selected reference voltage * 40/64
            DACBUF1_40 = 0b101000,
            /// selected reference voltage * 41/64
            DACBUF1_41 = 0b101001,
            /// selected reference voltage * 42/64
            DACBUF1_42 = 0b101010,
            /// selected reference voltage * 43/64
            DACBUF1_43 = 0b101011,
            /// selected reference voltage * 44/64
            DACBUF1_44 = 0b101100,
            /// selected reference voltage * 45/64
            DACBUF1_45 = 0b101101,
            /// selected reference voltage * 46/64
            DACBUF1_46 = 0b101110,
            /// selected reference voltage * 47/64
            DACBUF1_47 = 0b101111,
            /// selected reference voltage * 48/64
            DACBUF1_48 = 0b110000,
            /// selected reference voltage * 49/64
            DACBUF1_49 = 0b110001,
            /// selected reference voltage * 50/64
            DACBUF1_50 = 0b110010,
            /// selected reference voltage * 51/64
            DACBUF1_51 = 0b110011,
            /// selected reference voltage * 52/64
            DACBUF1_52 = 0b110100,
            /// selected reference voltage * 53/64
            DACBUF1_53 = 0b110101,
            /// selected reference voltage * 54/64
            DACBUF1_54 = 0b110110,
            /// selected reference voltage * 55/64
            DACBUF1_55 = 0b110111,
            /// selected reference voltage * 56/64
            DACBUF1_56 = 0b111000,
            /// selected reference voltage * 57/64
            DACBUF1_57 = 0b111001,
            /// selected reference voltage * 58/64
            DACBUF1_58 = 0b111010,
            /// selected reference voltage * 59/64
            DACBUF1_59 = 0b111011,
            /// selected reference voltage * 60/64
            DACBUF1_60 = 0b111100,
            /// selected reference voltage * 61/64
            DACBUF1_61 = 0b111101,
            /// selected reference voltage * 62/64
            DACBUF1_62 = 0b111110,
            /// selected reference voltage * 63/64
            DACBUF1_63 = 0b111111,
        }
        /// 2nd 6-bit DAC buffer Data
        DACBUF2: 8..13 = enum DACBUF2 {
            /// 0v
            DACBUF2_0 = 0b000000,
            /// selected reference voltage * 1/64
            DACBUF2_1 = 0b000001,
            /// selected reference voltage * 2/64
            DACBUF2_2 = 0b000010,
            /// selected reference voltage * 3/64
            DACBUF2_3 = 0b000011,
            /// selected reference voltage * 4/64
            DACBUF2_4 = 0b000100,
            /// selected reference voltage * 5/64
            DACBUF2_5 = 0b000101,
            /// selected reference voltage * 6/64
            DACBUF2_6 = 0b000110,
            /// selected reference voltage * 7/64
            DACBUF2_7 = 0b000111,
            /// selected reference voltage * 8/64
            DACBUF2_8 = 0b001000,
            /// selected reference voltage * 9/64
            DACBUF2_9 = 0b001001,
            /// selected reference voltage * 10/64
            DACBUF2_10 = 0b001010,
            /// selected reference voltage * 11/64
            DACBUF2_11 = 0b001011,
            /// selected reference voltage * 12/64
            DACBUF2_12 = 0b001100,
            /// selected reference voltage * 13/64
            DACBUF2_13 = 0b001101,
            /// selected reference voltage * 14/64
            DACBUF2_14 = 0b001110,
            /// selected reference voltage * 15/64
            DACBUF2_15 = 0b001111,
            /// selected reference voltage * 16/64
            DACBUF2_16 = 0b010000,
            /// selected reference voltage * 17/64
            DACBUF2_17 = 0b010001,
            /// selected reference voltage * 18/64
            DACBUF2_18 = 0b010010,
            /// selected reference voltage * 19/64
            DACBUF2_19 = 0b010011,
            /// selected reference voltage * 20/64
            DACBUF2_20 = 0b010100,
            /// selected reference voltage * 21/64
            DACBUF2_21 = 0b010101,
            /// selected reference voltage * 22/64
            DACBUF2_22 = 0b010110,
            /// selected reference voltage * 23/64
            DACBUF2_23 = 0b010111,
            /// selected reference voltage * 24/64
            DACBUF2_24 = 0b011000,
            /// selected reference voltage * 25/64
            DACBUF2_25 = 0b011001,
            /// selected reference voltage * 26/64
            DACBUF2_26 = 0b011010,
            /// selected reference voltage * 27/64
            DACBUF2_27 = 0b011011,
            /// selected reference voltage * 28/64
            DACBUF2_28 = 0b011100,
            /// selected reference voltage * 29/64
            DACBUF2_29 = 0b011101,
            /// selected reference voltage * 30/64
            DACBUF2_30 = 0b011110,
            /// selected reference voltage * 31/64
            DACBUF2_31 = 0b011111,
            /// selected reference voltage * 32/64
            DACBUF2_32 = 0b100000,
            /// selected reference voltage * 33/64
            DACBUF2_33 = 0b100001,
            /// selected reference voltage * 34/64
            DACBUF2_34 = 0b100010,
            /// selected reference voltage * 35/64
            DACBUF2_35 = 0b100011,
            /// selected reference voltage * 36/64
            DACBUF2_36 = 0b100100,
            /// selected reference voltage * 37/64
            DACBUF2_37 = 0b100101,
            /// selected reference voltage * 38/64
            DACBUF2_38 = 0b100110,
            /// selected reference voltage * 39/64
            DACBUF2_39 = 0b100111,
            /// selected reference voltage * 40/64
            DACBUF2_40 = 0b101000,
            /// selected reference voltage * 41/64
            DACBUF2_41 = 0b101001,
            /// selected reference voltage * 42/64
            DACBUF2_42 = 0b101010,
            /// selected reference voltage * 43/64
            DACBUF2_43 = 0b101011,
            /// selected reference voltage * 44/64
            DACBUF2_44 = 0b101100,
            /// selected reference voltage * 45/64
            DACBUF2_45 = 0b101101,
            /// selected reference voltage * 46/64
            DACBUF2_46 = 0b101110,
            /// selected reference voltage * 47/64
            DACBUF2_47 = 0b101111,
            /// selected reference voltage * 48/64
            DACBUF2_48 = 0b110000,
            /// selected reference voltage * 49/64
            DACBUF2_49 = 0b110001,
            /// selected reference voltage * 50/64
            DACBUF2_50 = 0b110010,
            /// selected reference voltage * 51/64
            DACBUF2_51 = 0b110011,
            /// selected reference voltage * 52/64
            DACBUF2_52 = 0b110100,
            /// selected reference voltage * 53/64
            DACBUF2_53 = 0b110101,
            /// selected reference voltage * 54/64
            DACBUF2_54 = 0b110110,
            /// selected reference voltage * 55/64
            DACBUF2_55 = 0b110111,
            /// selected reference voltage * 56/64
            DACBUF2_56 = 0b111000,
            /// selected reference voltage * 57/64
            DACBUF2_57 = 0b111001,
            /// selected reference voltage * 58/64
            DACBUF2_58 = 0b111010,
            /// selected reference voltage * 59/64
            DACBUF2_59 = 0b111011,
            /// selected reference voltage * 60/64
            DACBUF2_60 = 0b111100,
            /// selected reference voltage * 61/64
            DACBUF2_61 = 0b111101,
            /// selected reference voltage * 62/64
            DACBUF2_62 = 0b111110,
            /// selected reference voltage * 63/64
            DACBUF2_63 = 0b111111,
        }
    }
}
