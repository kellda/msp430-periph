//! eCOMP0

utils::periph! {
    /// eCOMP0
    eCOMP0;
    /// Comparator Control Register 0
    rw CP0CTL0 @ 0x00: u16 = 0_0 {
        /// Channel input enable for the V+ terminal
        CPPEN: 4 = enum CPPEN {
            /// Selected analog input channel for V+ terminal is disabled.
            CPPEN_0 = 0b0,
            /// Selected analog input channel for V+ terminal is enabled.
            CPPEN_1 = 0b1,
        }
        /// Channel input selected for the - terminal
        CPNSEL: 8..10 = enum CPNSEL {
            /// select external input source
            CPNSEL_0 = 0b000,
            /// select external input source
            CPNSEL_1 = 0b001,
            /// select external input source
            CPNSEL_2 = 0b010,
            /// select external input source
            CPNSEL_3 = 0b011,
            /// device specific, please refer to device data sheet for details
            CPNSEL_4 = 0b100,
            /// device specific, please refer to device data sheet for details
            CPNSEL_5 = 0b101,
            /// 6-bit DAC
            CPNSEL_6 = 0b110,
            /// Reserved
            CPNSEL_7 = 0b111,
        }
        /// Channel input enable for the - terminal
        CPNEN: 12 = enum CPNEN {
            /// Selected analog input channel for V- terminal is disabled.
            CPNEN_0 = 0b0,
            /// Selected analog input channel for V- terminal is enabled.
            CPNEN_1 = 0b1,
        }
        /// Channel input selected for the V+ terminal
        CPPSEL: 0..2 = enum CPPSEL {
            /// select external input source
            CPPSEL_0 = 0b000,
            /// select external input source
            CPPSEL_1 = 0b001,
            /// select external input source
            CPPSEL_2 = 0b010,
            /// select external input source
            CPPSEL_3 = 0b011,
            /// device specific, please refer to device data sheet for details
            CPPSEL_4 = 0b100,
            /// device specific, please refer to device data sheet for details
            CPPSEL_5 = 0b101,
            /// 6-bit DAC
            CPPSEL_6 = 0b110,
            /// Reserved
            CPPSEL_7 = 0b111,
        }
    }
    /// Comparator Control Register 1
    rw CP0CTL1 @ 0x02: u16 = 0_0 {
        /// Comparator output value
        CPOUT: 0 = struct CPOUT(bool);
        /// Comparator output polarity
        CPINV: 1 = enum CPINV {
            /// Comparator output is non-inverted
            CPINV_0 = 0b0,
            /// Comparator output is inverted
            CPINV_1 = 0b1,
        }
        /// Interrupt edge select for CEIIFG and CEIFG
        CPIES: 4 = enum CPIES {
            /// Rising edge for CPIFG, falling edge for CPIIFG
            CPIES_0 = 0b0,
            /// Falling edge for CPIFG, rising edge for CPIIFG
            CPIES_1 = 0b1,
        }
        /// Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag.
        CPFLT: 5 = enum CPFLT {
            /// Comparator output is not filtered
            CPFLT_0 = 0b0,
            /// Comparator output is filtered
            CPFLT_1 = 0b1,
        }
        /// Analog Filter Delay selection. These bits are used to select the analog filter delay
        CPFLTDLY: 6..7 = enum CPFLTDLY {
            /// Typical filter delay of 450ns
            CPFLTDLY_0 = 0b00,
            /// Typical filter delay of 900ns
            CPFLTDLY_1 = 0b01,
            /// Typical filter delay of 1800ns
            CPFLTDLY_2 = 0b10,
            /// Typical filter delay of 3600ns
            CPFLTDLY_3 = 0b11,
        }
        /// Power mode selection.
        CPMSEL: 8 = enum CPMSEL {
            /// High-power & High speed mode (500nA)
            CPMSEL_0 = 0b0,
            /// Low-power & Low speed mode (10nA)
            CPMSEL_1 = 0b1,
        }
        /// Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power.
        CPEN: 9 = enum CPEN {
            /// Comparator is disabled
            CPEN_0 = 0b0,
            /// Comparator is enabled
            CPEN_1 = 0b1,
        }
        /// Programable Hysteresis mode. These bits are used to select the Hysteresis mode.
        CPHSEL: 10..11 = enum CPHSEL {
            /// disable
            CPHSEL_0 = 0b00,
            /// 10mV
            CPHSEL_1 = 0b01,
            /// 20mV
            CPHSEL_2 = 0b10,
            /// 30mV
            CPHSEL_3 = 0b11,
        }
        /// Comparator interrupt output enable bit
        CPIE: 14 = enum CPIE {
            /// Interrupt output is disabled
            CPIE_0 = 0b0,
            /// Interrupt output is enabled
            CPIE_1 = 0b1,
        }
        /// Comparator inverted interrupt output enable bit
        CPIIE: 15 = enum CPIIE {
            /// Interrupt inverted output is disabled
            CPIIE_0 = 0b0,
            /// Interrupt inverted output is enabled
            CPIIE_1 = 0b1,
        }
    }
    /// Comparator Interrupt Control Register
    rw CP0INT @ 0x06: u16 = 0_0 {
        /// Comparator output interrupt flag
        CPIFG: 0 = enum CPIFG {
            /// No interrupt pending.
            CPIFG_0 = 0b0,
            /// Output interrupt pending.
            CPIFG_1 = 0b1,
        }
        /// Comparator output inverted interrupt flag
        CPIIFG: 1 = enum CPIIFG {
            /// No interrupt pending.
            CPIIFG_0 = 0b0,
            /// Output interrupt pending.
            CPIIFG_1 = 0b1,
        }
    }
    /// Comparator Interrupt Vector Word Register
    r CP0IV @ 0x08: u16 = 0_0 {
        /// Comparator interrupt vector word register
        CPIV: 0..15 = enum CPIV {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// CPIFG
            CPIFG = 0b0000000000000010,
            /// CPIIFG
            CPIIFG = 0b0000000000000100,
        }
    }
    /// 6-bit Comparator built-in DAC Control Register
    rw CP0DACCTL @ 0x10: u16 = 0_0 {
        /// This bit is only valid when CPDACBUFS is set to 1.
        CPDACSW: 0 = enum CPDACSW {
            /// CPDACBUF1 selected
            CPDACSW_0 = 0b0,
            /// CPDACBUF2 selected
            CPDACSW_1 = 0b1,
        }
        /// Comparator built-in DAC buffer controlled source selection.
        CPDACBUFS: 1 = enum CPDACBUFS {
            /// Comparator output is selected as the buffer control source
            CPDACBUFS_0 = 0b0,
            /// CPDACSW bit is selected as the buffer control source
            CPDACBUFS_1 = 0b1,
        }
        /// Comparator built-in DAC reference voltage selection
        CPDACREFS: 2 = enum CPDACREFS {
            /// VDD selected
            CPDACREFS_0 = 0b0,
            /// on-chip VREF selected
            CPDACREFS_1 = 0b1,
        }
        /// Comparator built-in DAC output control bit.
        CPDACEN: 7 = enum CPDACEN {
            /// DAC output is disabled.
            CPDACEN_0 = 0b0,
            /// DAC output is enabled.
            CPDACEN_1 = 0b1,
        }
    }
    /// 6-bit Comparator built-in DAC Data Register
    rw CP0DACDATA @ 0x12: u16 = 0_0 {
        /// 1st 6-bit DAC buffer Data
        CPDACBUF1: 0..5 = enum CPDACBUF1 {
            /// 0v
            CPDACBUF1_0 = 0b000000,
            /// selected reference voltage * 1/64
            CPDACBUF1_1 = 0b000001,
            /// selected reference voltage * 2/64
            CPDACBUF1_2 = 0b000010,
            /// selected reference voltage * 3/64
            CPDACBUF1_3 = 0b000011,
            /// selected reference voltage * 4/64
            CPDACBUF1_4 = 0b000100,
            /// selected reference voltage * 5/64
            CPDACBUF1_5 = 0b000101,
            /// selected reference voltage * 6/64
            CPDACBUF1_6 = 0b000110,
            /// selected reference voltage * 7/64
            CPDACBUF1_7 = 0b000111,
            /// selected reference voltage * 8/64
            CPDACBUF1_8 = 0b001000,
            /// selected reference voltage *9/64
            CPDACBUF1_9 = 0b001001,
            /// selected reference voltage * 10/64
            CPDACBUF1_10 = 0b001010,
            /// selected reference voltage * 11/64
            CPDACBUF1_11 = 0b001011,
            /// selected reference voltage * 12/64
            CPDACBUF1_12 = 0b001100,
            /// selected reference voltage * 13/64
            CPDACBUF1_13 = 0b001101,
            /// selected reference voltage * 14/64
            CPDACBUF1_14 = 0b001110,
            /// selected reference voltage * 15/64
            CPDACBUF1_15 = 0b001111,
            /// selected reference voltage * 16/64
            CPDACBUF1_16 = 0b010000,
            /// selected reference voltage * 17/64
            CPDACBUF1_17 = 0b010001,
            /// selected reference voltage * 18/64
            CPDACBUF1_18 = 0b010010,
            /// selected reference voltage * 19/64
            CPDACBUF1_19 = 0b010011,
            /// selected reference voltage * 20/64
            CPDACBUF1_20 = 0b010100,
            /// selected reference voltage * 21/64
            CPDACBUF1_21 = 0b010101,
            /// selected reference voltage * 22/64
            CPDACBUF1_22 = 0b010110,
            /// selected reference voltage * 23/64
            CPDACBUF1_23 = 0b010111,
            /// selected reference voltage * 24/64
            CPDACBUF1_24 = 0b011000,
            /// selected reference voltage * 25/64
            CPDACBUF1_25 = 0b011001,
            /// selected reference voltage * 26/64
            CPDACBUF1_26 = 0b011010,
            /// selected reference voltage * 27/64
            CPDACBUF1_27 = 0b011011,
            /// selected reference voltage * 28/64
            CPDACBUF1_28 = 0b011100,
            /// selected reference voltage * 29/64
            CPDACBUF1_29 = 0b011101,
            /// selected reference voltage * 30/64
            CPDACBUF1_30 = 0b011110,
            /// selected reference voltage * 31/64
            CPDACBUF1_31 = 0b011111,
            /// selected reference voltage * 32/64
            CPDACBUF1_32 = 0b100000,
            /// selected reference voltage * 33/64
            CPDACBUF1_33 = 0b100001,
            /// selected reference voltage * 34/64
            CPDACBUF1_34 = 0b100010,
            /// selected reference voltage * 35/64
            CPDACBUF1_35 = 0b100011,
            /// selected reference voltage * 36/64
            CPDACBUF1_36 = 0b100100,
            /// selected reference voltage * 37/64
            CPDACBUF1_37 = 0b100101,
            /// selected reference voltage * 38/64
            CPDACBUF1_38 = 0b100110,
            /// selected reference voltage * 39/64
            CPDACBUF1_39 = 0b100111,
            /// selected reference voltage * 40/64
            CPDACBUF1_40 = 0b101000,
            /// selected reference voltage * 41/64
            CPDACBUF1_41 = 0b101001,
            /// selected reference voltage * 42/64
            CPDACBUF1_42 = 0b101010,
            /// selected reference voltage * 43/64
            CPDACBUF1_43 = 0b101011,
            /// selected reference voltage * 44/64
            CPDACBUF1_44 = 0b101100,
            /// selected reference voltage * 45/64
            CPDACBUF1_45 = 0b101101,
            /// selected reference voltage * 46/64
            CPDACBUF1_46 = 0b101110,
            /// selected reference voltage * 47/64
            CPDACBUF1_47 = 0b101111,
            /// selected reference voltage * 48/64
            CPDACBUF1_48 = 0b110000,
            /// selected reference voltage * 49/64
            CPDACBUF1_49 = 0b110001,
            /// selected reference voltage * 50/64
            CPDACBUF1_50 = 0b110010,
            /// selected reference voltage * 51/64
            CPDACBUF1_51 = 0b110011,
            /// selected reference voltage * 52/64
            CPDACBUF1_52 = 0b110100,
            /// selected reference voltage * 53/64
            CPDACBUF1_53 = 0b110101,
            /// selected reference voltage * 54/64
            CPDACBUF1_54 = 0b110110,
            /// selected reference voltage * 55/64
            CPDACBUF1_55 = 0b110111,
            /// selected reference voltage * 56/64
            CPDACBUF1_56 = 0b111000,
            /// selected reference voltage * 57/64
            CPDACBUF1_57 = 0b111001,
            /// selected reference voltage * 58/64
            CPDACBUF1_58 = 0b111010,
            /// selected reference voltage * 59/64
            CPDACBUF1_59 = 0b111011,
            /// selected reference voltage * 60/64
            CPDACBUF1_60 = 0b111100,
            /// selected reference voltage * 61/64
            CPDACBUF1_61 = 0b111101,
            /// selected reference voltage * 62/64
            CPDACBUF1_62 = 0b111110,
            /// selected reference voltage * 63/64
            CPDACBUF1_63 = 0b111111,
        }
        /// 2nd 6-bit DAC buffer Data
        CPDACBUF2: 8..13 = enum CPDACBUF2 {
            /// 0v
            CPDACBUF2_0 = 0b000000,
            /// selected reference voltage * 1/64
            CPDACBUF2_1 = 0b000001,
            /// selected reference voltage * 2/64
            CPDACBUF2_2 = 0b000010,
            /// selected reference voltage * 3/64
            CPDACBUF2_3 = 0b000011,
            /// selected reference voltage * 4/64
            CPDACBUF2_4 = 0b000100,
            /// selected reference voltage * 5/64
            CPDACBUF2_5 = 0b000101,
            /// selected reference voltage * 6/64
            CPDACBUF2_6 = 0b000110,
            /// selected reference voltage * 7/64
            CPDACBUF2_7 = 0b000111,
            /// selected reference voltage * 8/64
            CPDACBUF2_8 = 0b001000,
            /// selected reference voltage * 9/64
            CPDACBUF2_9 = 0b001001,
            /// selected reference voltage * 10/64
            CPDACBUF2_10 = 0b001010,
            /// selected reference voltage * 11/64
            CPDACBUF2_11 = 0b001011,
            /// selected reference voltage * 12/64
            CPDACBUF2_12 = 0b001100,
            /// selected reference voltage * 13/64
            CPDACBUF2_13 = 0b001101,
            /// selected reference voltage * 14/64
            CPDACBUF2_14 = 0b001110,
            /// selected reference voltage * 15/64
            CPDACBUF2_15 = 0b001111,
            /// selected reference voltage * 16/64
            CPDACBUF2_16 = 0b010000,
            /// selected reference voltage * 17/64
            CPDACBUF2_17 = 0b010001,
            /// selected reference voltage * 18/64
            CPDACBUF2_18 = 0b010010,
            /// selected reference voltage * 19/64
            CPDACBUF2_19 = 0b010011,
            /// selected reference voltage * 20/64
            CPDACBUF2_20 = 0b010100,
            /// selected reference voltage * 21/64
            CPDACBUF2_21 = 0b010101,
            /// selected reference voltage * 22/64
            CPDACBUF2_22 = 0b010110,
            /// selected reference voltage * 23/64
            CPDACBUF2_23 = 0b010111,
            /// selected reference voltage * 24/64
            CPDACBUF2_24 = 0b011000,
            /// selected reference voltage * 25/64
            CPDACBUF2_25 = 0b011001,
            /// selected reference voltage * 26/64
            CPDACBUF2_26 = 0b011010,
            /// selected reference voltage * 27/64
            CPDACBUF2_27 = 0b011011,
            /// selected reference voltage * 28/64
            CPDACBUF2_28 = 0b011100,
            /// selected reference voltage * 29/64
            CPDACBUF2_29 = 0b011101,
            /// selected reference voltage * 30/64
            CPDACBUF2_30 = 0b011110,
            /// selected reference voltage * 31/64
            CPDACBUF2_31 = 0b011111,
            /// selected reference voltage * 32/64
            CPDACBUF2_32 = 0b100000,
            /// selected reference voltage * 33/64
            CPDACBUF2_33 = 0b100001,
            /// selected reference voltage * 34/64
            CPDACBUF2_34 = 0b100010,
            /// selected reference voltage * 35/64
            CPDACBUF2_35 = 0b100011,
            /// selected reference voltage * 36/64
            CPDACBUF2_36 = 0b100100,
            /// selected reference voltage * 37/64
            CPDACBUF2_37 = 0b100101,
            /// selected reference voltage * 38/64
            CPDACBUF2_38 = 0b100110,
            /// selected reference voltage * 39/64
            CPDACBUF2_39 = 0b100111,
            /// selected reference voltage * 40/64
            CPDACBUF2_40 = 0b101000,
            /// selected reference voltage * 41/64
            CPDACBUF2_41 = 0b101001,
            /// selected reference voltage * 42/64
            CPDACBUF2_42 = 0b101010,
            /// selected reference voltage * 43/64
            CPDACBUF2_43 = 0b101011,
            /// selected reference voltage * 44/64
            CPDACBUF2_44 = 0b101100,
            /// selected reference voltage * 45/64
            CPDACBUF2_45 = 0b101101,
            /// selected reference voltage * 46/64
            CPDACBUF2_46 = 0b101110,
            /// selected reference voltage * 47/64
            CPDACBUF2_47 = 0b101111,
            /// selected reference voltage * 48/64
            CPDACBUF2_48 = 0b110000,
            /// selected reference voltage * 49/64
            CPDACBUF2_49 = 0b110001,
            /// selected reference voltage * 50/64
            CPDACBUF2_50 = 0b110010,
            /// selected reference voltage * 51/64
            CPDACBUF2_51 = 0b110011,
            /// selected reference voltage * 52/64
            CPDACBUF2_52 = 0b110100,
            /// selected reference voltage * 53/64
            CPDACBUF2_53 = 0b110101,
            /// selected reference voltage * 54/64
            CPDACBUF2_54 = 0b110110,
            /// selected reference voltage * 55/64
            CPDACBUF2_55 = 0b110111,
            /// selected reference voltage * 56/64
            CPDACBUF2_56 = 0b111000,
            /// selected reference voltage * 57/64
            CPDACBUF2_57 = 0b111001,
            /// selected reference voltage * 58/64
            CPDACBUF2_58 = 0b111010,
            /// selected reference voltage * 59/64
            CPDACBUF2_59 = 0b111011,
            /// selected reference voltage * 60/64
            CPDACBUF2_60 = 0b111100,
            /// selected reference voltage * 61/64
            CPDACBUF2_61 = 0b111101,
            /// selected reference voltage * 62/64
            CPDACBUF2_62 = 0b111110,
            /// selected reference voltage * 63/64
            CPDACBUF2_63 = 0b111111,
        }
    }
}
