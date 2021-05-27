//! LCD_C

utils::periph! {
    /// LCD_C
    LCD_C;
    /// LCD_C control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// LCD on
        ON: 0 = enum ON {
            /// LCD_C module off
            OFF = 0b0,
            /// LCD_C module on
            ON = 0b1,
        }
        /// LCD low-power waveform
        LP: 1 = enum LP {
            /// Standard LCD waveforms on segment and common lines selected
            LP_0 = 0b0,
            /// Low-power LCD waveforms on segment and common lines selected
            LP_1 = 0b1,
        }
        /// LCD segments on
        SON: 2 = enum SON {
            /// All LCD segments are off
            OFF = 0b0,
            /// All LCD segments are enabled and on or off according to their corresponding memory location
            ON = 0b1,
        }
        /// LCD mux rate
        MX: 3..5 = enum MX {
            /// Static
            STATIC = 0b000,
            /// 2-mux
            _2MUX = 0b001,
            /// 3-mux
            _3MUX = 0b010,
            /// 4-mux
            _4MUX = 0b011,
            /// 5-mux
            _5MUX = 0b100,
            /// 6-mux
            _6MUX = 0b101,
            /// 7-mux
            _7MUX = 0b110,
            /// 8-mux
            _8MUX = 0b111,
        }
        /// Clock source select for LCD and blinking frequency
        SSEL: 7 = enum SSEL {
            /// ACLK (30 kHz to 40 kHz)
            ACLK = 0b0,
            /// VLOCLK
            VLOCLK = 0b1,
        }
        /// LCD frequency pre-scaler
        PRE: 8..10 = enum PRE {
            /// Divide by 1
            _1 = 0b000,
            /// Divide by 2
            _2 = 0b001,
            /// Divide by 4
            _4 = 0b010,
            /// Divide by 8
            _8 = 0b011,
            /// Divide by 16
            _16 = 0b100,
            /// Divide by 32
            _32 = 0b101,
        }
        /// LCD frequency divider
        DIV: 11..15 = enum DIV {
            /// Divide by 1
            _1 = 0b00000,
            /// Divide by 2
            _2 = 0b00001,
            /// Divide by 3
            _3 = 0b00010,
            /// Divide by 4
            _4 = 0b00011,
            /// Divide by 5
            _5 = 0b00100,
            /// Divide by 6
            _6 = 0b00101,
            /// Divide by 7
            _7 = 0b00110,
            /// Divide by 8
            _8 = 0b00111,
            /// Divide by 9
            _9 = 0b01000,
            /// Divide by 10
            _10 = 0b01001,
            /// Divide by 11
            _11 = 0b01010,
            /// Divide by 12
            _12 = 0b01011,
            /// Divide by 13
            _13 = 0b01100,
            /// Divide by 14
            _14 = 0b01101,
            /// Divide by 15
            _15 = 0b01110,
            /// Divide by 16
            _16 = 0b01111,
            /// Divide by 17
            _17 = 0b10000,
            /// Divide by 18
            _18 = 0b10001,
            /// Divide by 19
            _19 = 0b10010,
            /// Divide by 20
            _20 = 0b10011,
            /// Divide by 21
            _21 = 0b10100,
            /// Divide by 22
            _22 = 0b10101,
            /// Divide by 23
            _23 = 0b10110,
            /// Divide by 24
            _24 = 0b10111,
            /// Divide by 25
            _25 = 0b11000,
            /// Divide by 26
            _26 = 0b11001,
            /// Divide by 27
            _27 = 0b11010,
            /// Divide by 28
            _28 = 0b11011,
            /// Divide by 29
            _29 = 0b11100,
            /// Divide by 30
            _30 = 0b11101,
            /// Divide by 31
            _31 = 0b11110,
            /// Divide by 32
            _32 = 0b11111,
        }
    }
    /// LCD_C control 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// LCD frame interrupt flag
        FRMIFG: 0 = enum FRMIFG {
            /// No interrupt pending
            FRMIFG_0 = 0b0,
            /// Interrupt pending
            FRMIFG_1 = 0b1,
        }
        /// LCD blinking interrupt flag, segments switched off
        BLKOFFIFG: 1 = enum BLKOFFIFG {
            /// No interrupt pending
            BLKOFFIFG_0 = 0b0,
            /// Interrupt pending
            BLKOFFIFG_1 = 0b1,
        }
        /// LCD blinking interrupt flag, segments switched on
        BLKONIFG: 2 = enum BLKONIFG {
            /// No interrupt pending
            BLKONIFG_0 = 0b0,
            /// Interrupt pending
            BLKONIFG_1 = 0b1,
        }
        /// No capacitance connected interrupt flag
        NOCAPIFG: 3 = enum NOCAPIFG {
            /// No interrupt pending
            NOCAPIFG_0 = 0b0,
            /// Interrupt pending
            NOCAPIFG_1 = 0b1,
        }
        /// LCD frame interrupt enable
        FRMIE: 8 = enum FRMIE {
            /// Interrupt disabled
            FRMIE_0 = 0b0,
            /// Interrupt enabled
            FRMIE_1 = 0b1,
        }
        /// LCD blinking interrupt enable, segments switched off
        BLKOFFIE: 9 = enum BLKOFFIE {
            /// Interrupt disabled
            BLKOFFIE_0 = 0b0,
            /// Interrupt enabled
            BLKOFFIE_1 = 0b1,
        }
        /// LCD blinking interrupt enable, segments switched on
        BLKONIE: 10 = enum BLKONIE {
            /// Interrupt disabled
            BLKONIE_0 = 0b0,
            /// Interrupt enabled
            BLKONIE_1 = 0b1,
        }
        /// No capacitance connected interrupt enable
        NOCAPIE: 11 = enum NOCAPIE {
            /// Interrupt disabled
            NOCAPIE_0 = 0b0,
            /// Interrupt enabled
            NOCAPIE_1 = 0b1,
        }
    }
    /// LCD_C blinking control
    rw BLKCTL @ 0x04: u16 = 0_0 {
        /// Blinking mode
        BLKMOD: 0..1 = enum BLKMOD {
            /// Blinking disabled
            BLKMOD_0 = 0b00,
            /// Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode 5 blinking is disabled.
            BLKMOD_1 = 0b01,
            /// Blinking of all segments
            BLKMOD_2 = 0b10,
            /// Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode 5 blinking is disabled.
            BLKMOD_3 = 0b11,
        }
        /// Clock pre-scaler for blinking frequency
        BLKPRE: 2..4 = enum BLKPRE {
            /// Divide by 512
            _512 = 0b000,
            /// Divide by 1024
            _1024 = 0b001,
            /// Divide by 2048
            _2048 = 0b010,
            /// Divide by 4096
            _4096 = 0b011,
            /// Divide by 8162
            _8162 = 0b100,
            /// Divide by 16384
            _16384 = 0b101,
            /// Divide by 32768
            _32768 = 0b110,
            /// Divide by 65536
            _65536 = 0b111,
        }
        /// Clock divider for blinking frequency
        BLKDIV: 5..7 = enum BLKDIV {
            /// Divide by 1
            _1 = 0b000,
            /// Divide by 2
            _2 = 0b001,
            /// Divide by 3
            _3 = 0b010,
            /// Divide by 4
            _4 = 0b011,
            /// Divide by 5
            _5 = 0b100,
            /// Divide by 6
            _6 = 0b101,
            /// Divide by 7
            _7 = 0b110,
            /// Divide by 8
            _8 = 0b111,
        }
    }
    /// LCD_C memory control
    rw MEMCTL @ 0x06: u16 = 0_0 {
        /// Select LCD memory registers for display
        DISP: 0 = enum DISP {
            /// Display content of LCD memory registers LCDM
            DISP_0 = 0b0,
            /// Display content of LCD blinking memory registers LCDBM
            DISP_1 = 0b1,
        }
        /// Clear LCD memory
        LRM: 1 = enum LRM {
            /// Contents of LCD memory registers LCDMx remain unchanged
            LRM_0 = 0b0,
            /// Clear content of all LCD memory registers LCDM
            LRM_1 = 0b1,
        }
        /// Clear LCD blinking memory
        LRBM: 2 = enum LRBM {
            /// Contents of blinking memory registers LCDBM remain unchanged
            LRBM_0 = 0b0,
            /// Clear content of all blinking memory registers LCDBM
            LRBM_1 = 0b1,
        }
    }
    /// LCD_C Voltage Control Register
    rw VCTL @ 0x08: u16 = 0_0 {
        /// Bias select
        LCD2B: 0 = enum LCD2B {
            /// 1/3 bias
            LCD2B_0 = 0b0,
            /// 1/2 bias
            LCD2B_1 = 0b1,
        }
        /// Charge pump reference select
        VLCDREF: 1..2 = enum VLCDREF {
            /// Internal reference voltage
            VLCDREF_0 = 0b00,
            /// External reference voltage
            VLCDREF_1 = 0b01,
            /// Internal reference voltage switched to external pin LCDREF/R13
            VLCDREF_2 = 0b10,
            /// Reserved (defaults to external reference voltage)
            VLCDREF_3 = 0b11,
        }
        /// Charge pump enable
        CPEN: 3 = enum CPEN {
            /// Charge pump disabled
            CPEN_0 = 0b0,
            /// Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCD  0 or VLCDREF  0
            CPEN_1 = 0b1,
        }
        /// VLCD source select
        VLCDEXT: 4 = enum VLCDEXT {
            /// VLCD is generated internally
            VLCDEXT_0 = 0b0,
            /// VLCD is sourced externally
            VLCDEXT_1 = 0b1,
        }
        /// V2 to V4 voltage select
        EXTBIAS: 5 = enum EXTBIAS {
            /// V2 to V4 are generated internally
            EXTBIAS_0 = 0b0,
            /// V2 to V4 are sourced externally and the internal bias generator is switched off
            EXTBIAS_1 = 0b1,
        }
        /// V5 voltage select
        R03EXT: 6 = enum R03EXT {
            /// V5 is VSS
            VSS = 0b0,
            /// V5 is sourced from the R03 pin
            R03 = 0b1,
        }
        /// V2 to V4 voltage on external Rx3 pins
        REXT: 7 = enum REXT {
            /// Internally generated V2 to V4 are not switched to pins (LCDEXTBIAS = 0)
            REXT_0 = 0b0,
            /// Internally generated V2 to V4 are switched to pins (LCDEXTBIAS = 0)
            REXT_1 = 0b1,
        }
        /// Charge pump voltage select
        VLCD: 9..12 = enum VLCD {
            /// Charge pump disabled
            DISABLED = 0b0000,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF
            _2_60 = 0b0001,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _2_66 = 0b0010,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _2_72 = 0b0011,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _2_78 = 0b0100,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _2_84 = 0b0101,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _2_90 = 0b0110,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _2_96 = 0b0111,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _3_02 = 0b1000,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _3_08 = 0b1001,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _3_14 = 0b1010,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _3_20 = 0b1011,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _3_26 = 0b1100,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _3_32 = 0b1101,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (VLCD  1) * 0.06 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (VLCD  1) * 0.05 * VREF
            _3_38 = 0b1110,
            /// If VLCDREF = 00 or 10: VLCD = 2.60 V + (15  1) * 0.06 V = 3.44 V; If VLCDREF = 01 or 11: VLCD = 2.17 * VREF + (15  1) * 0.05 * VREF = 2.87 * VREF
            _3_44 = 0b1111,
        }
    }
    /// LCD_C port control 0
    rw PCTL0 @ 0x0a: u16 = 0_0 {
        /// LCD segment line 0 enable
        S0: 0 = enum S0 {
            /// Multiplexed pins are port functions
            S0_0 = 0b0,
            /// Pins are LCD functions
            S0_1 = 0b1,
        }
        /// LCD segment line 1 enable
        S1: 1 = enum S1 {
            /// Multiplexed pins are port functions
            S1_0 = 0b0,
            /// Pins are LCD functions
            S1_1 = 0b1,
        }
        /// LCD segment line 2 enable
        S2: 2 = enum S2 {
            /// Multiplexed pins are port functions
            S2_0 = 0b0,
            /// Pins are LCD functions
            S2_1 = 0b1,
        }
        /// LCD segment line 3 enable
        S3: 3 = enum S3 {
            /// Multiplexed pins are port functions
            S3_0 = 0b0,
            /// Pins are LCD functions
            S3_1 = 0b1,
        }
        /// LCD segment line 4 enable
        S4: 4 = enum S4 {
            /// Multiplexed pins are port functions
            S4_0 = 0b0,
            /// Pins are LCD functions
            S4_1 = 0b1,
        }
        /// LCD segment line 5 enable
        S5: 5 = enum S5 {
            /// Multiplexed pins are port functions
            S5_0 = 0b0,
            /// Pins are LCD functions
            S5_1 = 0b1,
        }
        /// LCD segment line 6 enable
        S6: 6 = enum S6 {
            /// Multiplexed pins are port functions
            S6_0 = 0b0,
            /// Pins are LCD functions
            S6_1 = 0b1,
        }
        /// LCD segment line 7 enable
        S7: 7 = enum S7 {
            /// Multiplexed pins are port functions
            S7_0 = 0b0,
            /// Pins are LCD functions
            S7_1 = 0b1,
        }
        /// LCD segment line 8 enable
        S8: 8 = enum S8 {
            /// Multiplexed pins are port functions
            S8_0 = 0b0,
            /// Pins are LCD functions
            S8_1 = 0b1,
        }
        /// LCD segment line 9 enable
        S9: 9 = enum S9 {
            /// Multiplexed pins are port functions
            S9_0 = 0b0,
            /// Pins are LCD functions
            S9_1 = 0b1,
        }
        /// LCD segment line 10 enable
        S10: 10 = enum S10 {
            /// Multiplexed pins are port functions
            S10_0 = 0b0,
            /// Pins are LCD functions
            S10_1 = 0b1,
        }
        /// LCD segment line 11 enable
        S11: 11 = enum S11 {
            /// Multiplexed pins are port functions
            S11_0 = 0b0,
            /// Pins are LCD functions
            S11_1 = 0b1,
        }
        /// LCD segment line 12 enable
        S12: 12 = enum S12 {
            /// Multiplexed pins are port functions
            S12_0 = 0b0,
            /// Pins are LCD functions
            S12_1 = 0b1,
        }
        /// LCD segment line 13 enable
        S13: 13 = enum S13 {
            /// Multiplexed pins are port functions
            S13_0 = 0b0,
            /// Pins are LCD functions
            S13_1 = 0b1,
        }
        /// LCD segment line 14 enable
        S14: 14 = enum S14 {
            /// Multiplexed pins are port functions
            S14_0 = 0b0,
            /// Pins are LCD functions
            S14_1 = 0b1,
        }
        /// LCD segment line 15 enable
        S15: 15 = enum S15 {
            /// Multiplexed pins are port functions
            S15_0 = 0b0,
            /// Pins are LCD functions
            S15_1 = 0b1,
        }
    }
    /// LCD_C port control 1
    rw PCTL1 @ 0x0c: u16 = 0_0 {
        /// LCD segment line 16 enable
        S16: 0 = enum S16 {
            /// Multiplexed pins are port functions
            S16_0 = 0b0,
            /// Pins are LCD functions
            S16_1 = 0b1,
        }
        /// LCD segment line 17 enable
        S17: 1 = enum S17 {
            /// Multiplexed pins are port functions
            S17_0 = 0b0,
            /// Pins are LCD functions
            S17_1 = 0b1,
        }
        /// LCD segment line 18 enable
        S18: 2 = enum S18 {
            /// Multiplexed pins are port functions
            S18_0 = 0b0,
            /// Pins are LCD functions
            S18_1 = 0b1,
        }
        /// LCD segment line 19 enable
        S19: 3 = enum S19 {
            /// Multiplexed pins are port functions
            S19_0 = 0b0,
            /// Pins are LCD functions
            S19_1 = 0b1,
        }
        /// LCD segment line 20 enable
        S20: 4 = enum S20 {
            /// Multiplexed pins are port functions
            S20_0 = 0b0,
            /// Pins are LCD functions
            S20_1 = 0b1,
        }
        /// LCD segment line 21 enable
        S21: 5 = enum S21 {
            /// Multiplexed pins are port functions
            S21_0 = 0b0,
            /// Pins are LCD functions
            S21_1 = 0b1,
        }
        /// LCD segment line 22 enable
        S22: 6 = enum S22 {
            /// Multiplexed pins are port functions
            S22_0 = 0b0,
            /// Pins are LCD functions
            S22_1 = 0b1,
        }
        /// LCD segment line 23 enable
        S23: 7 = enum S23 {
            /// Multiplexed pins are port functions
            S23_0 = 0b0,
            /// Pins are LCD functions
            S23_1 = 0b1,
        }
        /// LCD segment line 24 enable
        S24: 8 = enum S24 {
            /// Multiplexed pins are port functions
            S24_0 = 0b0,
            /// Pins are LCD functions
            S24_1 = 0b1,
        }
        /// LCD segment line 25 enable
        S25: 9 = enum S25 {
            /// Multiplexed pins are port functions
            S25_0 = 0b0,
            /// Pins are LCD functions
            S25_1 = 0b1,
        }
        /// LCD segment line 26 enable
        S26: 10 = enum S26 {
            /// Multiplexed pins are port functions
            S26_0 = 0b0,
            /// Pins are LCD functions
            S26_1 = 0b1,
        }
        /// LCD segment line 27 enable
        S27: 11 = enum S27 {
            /// Multiplexed pins are port functions
            S27_0 = 0b0,
            /// Pins are LCD functions
            S27_1 = 0b1,
        }
        /// LCD segment line 28 enable
        S28: 12 = enum S28 {
            /// Multiplexed pins are port functions
            S28_0 = 0b0,
            /// Pins are LCD functions
            S28_1 = 0b1,
        }
        /// LCD segment line 29 enable
        S29: 13 = enum S29 {
            /// Multiplexed pins are port functions
            S29_0 = 0b0,
            /// Pins are LCD functions
            S29_1 = 0b1,
        }
        /// LCD segment line 30 enable
        S30: 14 = enum S30 {
            /// Multiplexed pins are port functions
            S30_0 = 0b0,
            /// Pins are LCD functions
            S30_1 = 0b1,
        }
        /// LCD segment line 31 enable
        S31: 15 = enum S31 {
            /// Multiplexed pins are port functions
            S31_0 = 0b0,
            /// Pins are LCD functions
            S31_1 = 0b1,
        }
    }
    /// LCD_C port control 2 (256 segments)
    rw PCTL2 @ 0x0e: u16 = 0_0 {
        /// LCD segment line 32 enable
        S32: 0 = enum S32 {
            /// Multiplexed pins are port functions
            S32_0 = 0b0,
            /// Pins are LCD functions
            S32_1 = 0b1,
        }
        /// LCD segment line 33 enable
        S33: 1 = enum S33 {
            /// Multiplexed pins are port functions
            S33_0 = 0b0,
            /// Pins are LCD functions
            S33_1 = 0b1,
        }
        /// LCD segment line 34 enable
        S34: 2 = enum S34 {
            /// Multiplexed pins are port functions
            S34_0 = 0b0,
            /// Pins are LCD functions
            S34_1 = 0b1,
        }
        /// LCD segment line 35 enable
        S35: 3 = enum S35 {
            /// Multiplexed pins are port functions
            S35_0 = 0b0,
            /// Pins are LCD functions
            S35_1 = 0b1,
        }
        /// LCD segment line 36 enable
        S36: 4 = enum S36 {
            /// Multiplexed pins are port functions
            S36_0 = 0b0,
            /// Pins are LCD functions
            S36_1 = 0b1,
        }
        /// LCD segment line 37 enable
        S37: 5 = enum S37 {
            /// Multiplexed pins are port functions
            S37_0 = 0b0,
            /// Pins are LCD functions
            S37_1 = 0b1,
        }
        /// LCD segment line 38 enable
        S38: 6 = enum S38 {
            /// Multiplexed pins are port functions
            S38_0 = 0b0,
            /// Pins are LCD functions
            S38_1 = 0b1,
        }
        /// LCD segment line 39 enable
        S39: 7 = enum S39 {
            /// Multiplexed pins are port functions
            S39_0 = 0b0,
            /// Pins are LCD functions
            S39_1 = 0b1,
        }
        /// LCD segment line 40 enable
        S40: 8 = enum S40 {
            /// Multiplexed pins are port functions
            S40_0 = 0b0,
            /// Pins are LCD functions
            S40_1 = 0b1,
        }
        /// LCD segment line 41 enable
        S41: 9 = enum S41 {
            /// Multiplexed pins are port functions
            S41_0 = 0b0,
            /// Pins are LCD functions
            S41_1 = 0b1,
        }
        /// LCD segment line 42 enable
        S42: 10 = enum S42 {
            /// Multiplexed pins are port functions
            S42_0 = 0b0,
            /// Pins are LCD functions
            S42_1 = 0b1,
        }
        /// LCD segment line 43 enable
        S43: 11 = enum S43 {
            /// Multiplexed pins are port functions
            S43_0 = 0b0,
            /// Pins are LCD functions
            S43_1 = 0b1,
        }
        /// LCD segment line 44 enable
        S44: 12 = enum S44 {
            /// Multiplexed pins are port functions
            S44_0 = 0b0,
            /// Pins are LCD functions
            S44_1 = 0b1,
        }
        /// LCD segment line 45 enable
        S45: 13 = enum S45 {
            /// Multiplexed pins are port functions
            S45_0 = 0b0,
            /// Pins are LCD functions
            S45_1 = 0b1,
        }
        /// LCD segment line 46 enable
        S46: 14 = enum S46 {
            /// Multiplexed pins are port functions
            S46_0 = 0b0,
            /// Pins are LCD functions
            S46_1 = 0b1,
        }
        /// LCD segment line 47 enable
        S47: 15 = enum S47 {
            /// Multiplexed pins are port functions
            S47_0 = 0b0,
            /// Pins are LCD functions
            S47_1 = 0b1,
        }
    }
    /// LCD_C port control 3 (384 segments)
    rw PCTL3 @ 0x10: u16 = 0_0 {
        /// LCD segment line 48 enable
        S48: 0 = enum S48 {
            /// Multiplexed pins are port functions
            S48_0 = 0b0,
            /// Pins are LCD functions
            S48_1 = 0b1,
        }
        /// LCD segment line 49 enable
        S49: 1 = enum S49 {
            /// Multiplexed pins are port functions
            S49_0 = 0b0,
            /// Pins are LCD functions
            S49_1 = 0b1,
        }
        /// LCD segment line 50 enable
        S50: 2 = enum S50 {
            /// Multiplexed pins are port functions
            S50_0 = 0b0,
            /// Pins are LCD functions
            S50_1 = 0b1,
        }
        /// LCD segment line 51 enable
        S51: 3 = enum S51 {
            /// Multiplexed pins are port functions
            S51_0 = 0b0,
            /// Pins are LCD functions
            S51_1 = 0b1,
        }
        /// LCD segment line 52 enable
        S52: 4 = enum S52 {
            /// Multiplexed pins are port functions
            S52_0 = 0b0,
            /// Pins are LCD functions
            S52_1 = 0b1,
        }
        /// LCD segment line 53 enable
        S53: 5 = enum S53 {
            /// Multiplexed pins are port functions
            S53_0 = 0b0,
            /// Pins are LCD functions
            S53_1 = 0b1,
        }
    }
    /// LCD_C charge pump control
    rw CPCTL @ 0x12: u16 = 0_0 {
        /// LCD charge pump disable
        CPDIS: 0..7 = struct CPDIS(u16);
        /// LCD charge pump clock synchronization
        CPCLKSYNC: 15 = enum CPCLKSYNC {
            /// Synchronization disabled
            CPCLKSYNC_0 = 0b0,
            /// Synchronization enabled
            CPCLKSYNC_1 = 0b1,
        }
    }
    /// LCD_C interrupt vector
    r IV @ 0x1e: u16 = 0_0 {
        /// LCD_C interrupt vector value
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: No capacitor connected; Interrupt Flag: LCDNOCAPIFG; Interrupt Priority: Highest
            NOCAPIFG = 0b0000000000000010,
            /// Interrupt Source: Blink, segments off; Interrupt Flag: LCDBLKOFFIFG
            BLKOFFIFG = 0b0000000000000100,
            /// Interrupt Source: Blink, segments on; Interrupt Flag: LCDBLKONIFG
            BLKONIFG = 0b0000000000000110,
            /// Interrupt Source: Frame interrupt; Interrupt Flag: LCDFRMIFG; Interrupt Priority: Lowest
            FRMIFG = 0b0000000000001000,
        }
    }
    /// LCD memory 1
    rw M1 @ 0x20: u8 = 0_0 {
        /// LCD memory 1
        M1: 0..7 = struct M1Field(u8);
    }
    /// LCD memory 2
    rw M2 @ 0x21: u8 = 0_0 {
        /// LCD memory 2
        M2: 0..7 = struct M2Field(u8);
    }
    /// LCD memory 3
    rw M3 @ 0x22: u8 = 0_0 {
        /// LCD memory 3
        M3: 0..7 = struct M3Field(u8);
    }
    /// LCD memory 4
    rw M4 @ 0x23: u8 = 0_0 {
        /// LCD memory 4
        M4: 0..7 = struct M4Field(u8);
    }
    /// LCD memory 5
    rw M5 @ 0x24: u8 = 0_0 {
        /// LCD memory 5
        M5: 0..7 = struct M5Field(u8);
    }
    /// LCD memory 6
    rw M6 @ 0x25: u8 = 0_0 {
        /// LCD memory 6
        M6: 0..7 = struct M6Field(u8);
    }
    /// LCD memory 7
    rw M7 @ 0x26: u8 = 0_0 {
        /// LCD memory 7
        M7: 0..7 = struct M7Field(u8);
    }
    /// LCD memory 8
    rw M8 @ 0x27: u8 = 0_0 {
        /// LCD memory 8
        M8: 0..7 = struct M8Field(u8);
    }
    /// LCD memory 9
    rw M9 @ 0x28: u8 = 0_0 {
        /// LCD memory 9
        M9: 0..7 = struct M9Field(u8);
    }
    /// LCD memory 10
    rw M10 @ 0x29: u8 = 0_0 {
        /// LCD memory 10
        M10: 0..7 = struct M10Field(u8);
    }
    /// LCD memory 11
    rw M11 @ 0x2a: u8 = 0_0 {
        /// LCD memory 11
        M11: 0..7 = struct M11Field(u8);
    }
    /// LCD memory 12
    rw M12 @ 0x2b: u8 = 0_0 {
        /// LCD memory 12
        M12: 0..7 = struct M12Field(u8);
    }
    /// LCD memory 13
    rw M13 @ 0x2c: u8 = 0_0 {
        /// LCD memory 13
        M13: 0..7 = struct M13Field(u8);
    }
    /// LCD memory 14
    rw M14 @ 0x2d: u8 = 0_0 {
        /// LCD memory 14
        M14: 0..7 = struct M14Field(u8);
    }
    /// LCD memory 15
    rw M15 @ 0x2e: u8 = 0_0 {
        /// LCD memory 15
        M15: 0..7 = struct M15Field(u8);
    }
    /// LCD memory 16
    rw M16 @ 0x2f: u8 = 0_0 {
        /// LCD memory 16
        M16: 0..7 = struct M16Field(u8);
    }
    /// LCD memory 17
    rw M17 @ 0x30: u8 = 0_0 {
        /// LCD memory 17
        M17: 0..7 = struct M17Field(u8);
    }
    /// LCD memory 18
    rw M18 @ 0x31: u8 = 0_0 {
        /// LCD memory 18
        M18: 0..7 = struct M18Field(u8);
    }
    /// LCD memory 19
    rw M19 @ 0x32: u8 = 0_0 {
        /// LCD memory 19
        M19: 0..7 = struct M19Field(u8);
    }
    /// LCD memory 20
    rw M20 @ 0x33: u8 = 0_0 {
        /// LCD memory 20
        M20: 0..7 = struct M20Field(u8);
    }
    /// LCD memory 21
    rw M21 @ 0x34: u8 = 0_0 {
        /// LCD memory 21
        M21: 0..7 = struct M21Field(u8);
    }
    /// LCD memory 22
    rw M22 @ 0x35: u8 = 0_0 {
        /// LCD memory 22
        M22: 0..7 = struct M22Field(u8);
    }
    /// LCD memory 23
    rw M23 @ 0x36: u8 = 0_0 {
        /// LCD memory 23
        M23: 0..7 = struct M23Field(u8);
    }
    /// LCD memory 24
    rw M24 @ 0x37: u8 = 0_0 {
        /// LCD memory 24
        M24: 0..7 = struct M24Field(u8);
    }
    /// LCD memory 25
    rw M25 @ 0x38: u8 = 0_0 {
        /// LCD memory 25
        M25: 0..7 = struct M25Field(u8);
    }
    /// LCD memory 26
    rw M26 @ 0x39: u8 = 0_0 {
        /// LCD memory 26
        M26: 0..7 = struct M26Field(u8);
    }
    /// LCD memory 27
    rw M27 @ 0x3a: u8 = 0_0 {
        /// LCD memory 27
        M27: 0..7 = struct M27Field(u8);
    }
    /// LCD memory 28
    rw M28 @ 0x3b: u8 = 0_0 {
        /// LCD memory 28
        M28: 0..7 = struct M28Field(u8);
    }
    /// LCD memory 29
    rw M29 @ 0x3c: u8 = 0_0 {
        /// LCD memory 29
        M29: 0..7 = struct M29Field(u8);
    }
    /// LCD memory 30
    rw M30 @ 0x3d: u8 = 0_0 {
        /// LCD memory 30
        M30: 0..7 = struct M30Field(u8);
    }
    /// LCD memory 31
    rw M31 @ 0x3e: u8 = 0_0 {
        /// LCD memory 31
        M31: 0..7 = struct M31Field(u8);
    }
    /// LCD memory 32
    rw M32 @ 0x3f: u8 = 0_0 {
        /// LCD memory 32
        M32: 0..7 = struct M32Field(u8);
    }
    /// LCD memory 33 / LCD blinking memory 1
    rw M33_LCDBM1 @ 0x40: u8 = 0_0 {
        /// LCD memory 33 / LCD blinking memory 1
        M33_LCDBM1: 0..7 = struct M33_LCDBM1Field(u8);
    }
    /// LCD memory 34 / LCD blinking memory 2
    rw M34_LCDBM2 @ 0x41: u8 = 0_0 {
        /// LCD memory 34 / LCD blinking memory 2
        M34_LCDBM2: 0..7 = struct M34_LCDBM2Field(u8);
    }
    /// LCD memory 35 / LCD blinking memory 3
    rw M35_LCDBM3 @ 0x42: u8 = 0_0 {
        /// LCD memory 35 / LCD blinking memory 3
        M35_LCDBM3: 0..7 = struct M35_LCDBM3Field(u8);
    }
    /// LCD memory 36 / LCD blinking memory 4
    rw M36_LCDBM4 @ 0x43: u8 = 0_0 {
        /// LCD memory 36 / LCD blinking memory 4
        M36_LCDBM4: 0..7 = struct M36_LCDBM4Field(u8);
    }
    /// LCD memory 37 / LCD blinking memory 5
    rw M37_LCDBM5 @ 0x44: u8 = 0_0 {
        /// LCD memory 37 / LCD blinking memory 5
        M37_LCDBM5: 0..7 = struct M37_LCDBM5Field(u8);
    }
    /// LCD memory 38 / LCD blinking memory 6
    rw M38_LCDBM6 @ 0x45: u8 = 0_0 {
        /// LCD memory 38 / LCD blinking memory 6
        M38_LCDBM6: 0..7 = struct M38_LCDBM6Field(u8);
    }
    /// LCD memory 39 / LCD blinking memory 7
    rw M39_LCDBM7 @ 0x46: u8 = 0_0 {
        /// LCD memory 39 / LCD blinking memory 7
        M39_LCDBM7: 0..7 = struct M39_LCDBM7Field(u8);
    }
    /// LCD memory 40 / LCD blinking memory 8
    rw M40_LCDBM8 @ 0x47: u8 = 0_0 {
        /// LCD memory 40 / LCD blinking memory 8
        M40_LCDBM8: 0..7 = struct M40_LCDBM8Field(u8);
    }
    /// LCD memory 41 / LCD blinking memory 9
    rw M41_LCDBM9 @ 0x48: u8 = 0_0 {
        /// LCD memory 41 / LCD blinking memory 9
        M41_LCDBM9: 0..7 = struct M41_LCDBM9Field(u8);
    }
    /// LCD memory 42 / LCD blinking memory 10
    rw M42_LCDBM10 @ 0x49: u8 = 0_0 {
        /// LCD memory 42 / LCD blinking memory 10
        M42_LCDBM10: 0..7 = struct M42_LCDBM10Field(u8);
    }
    /// LCD memory 43 / LCD blinking memory 11
    rw M43_LCDBM11 @ 0x4a: u8 = 0_0 {
        /// LCD memory 43 / LCD blinking memory 11
        M43_LCDBM11: 0..7 = struct M43_LCDBM11Field(u8);
    }
    /// LCD memory 44 / LCD blinking memory 11
    rw M44_LCDBM12 @ 0x4b: u8 = 0_0 {
        /// LCD memory 44 / LCD blinking memory 11
        M44_LCDBM12: 0..7 = struct M44_LCDBM12Field(u8);
    }
    /// LCD memory 45 / LCD blinking memory 13
    rw M45_LCDBM13 @ 0x4c: u8 = 0_0 {
        /// LCD memory 45 / LCD blinking memory 13
        M45_LCDBM13: 0..7 = struct M45_LCDBM13Field(u8);
    }
    /// LCD memory 46 / LCD blinking memory 14
    rw M46_LCDBM14 @ 0x4d: u8 = 0_0 {
        /// LCD memory 46 / LCD blinking memory 14
        M46_LCDBM14: 0..7 = struct M46_LCDBM14Field(u8);
    }
    /// LCD memory 47 / LCD blinking memory 15
    rw M47_LCDBM15 @ 0x4e: u8 = 0_0 {
        /// LCD memory 47 / LCD blinking memory 15
        M47_LCDBM15: 0..7 = struct M47_LCDBM15Field(u8);
    }
    /// LCD memory 48 / LCD blinking memory 16
    rw M48_LCDBM16 @ 0x4f: u8 = 0_0 {
        /// LCD memory 48 / LCD blinking memory 16
        M48_LCDBM16: 0..7 = struct M48_LCDBM16Field(u8);
    }
    /// LCD memory 49 / LCD blinking memory 17
    rw M49_LCDBM17 @ 0x50: u8 = 0_0 {
        /// LCD memory 49 / LCD blinking memory 17
        M49_LCDBM17: 0..7 = struct M49_LCDBM17Field(u8);
    }
    /// LCD memory 50 / LCD blinking memory 18
    rw M50_LCDBM18 @ 0x51: u8 = 0_0 {
        /// LCD memory 50 / LCD blinking memory 18
        M50_LCDBM18: 0..7 = struct M50_LCDBM18Field(u8);
    }
    /// LCD memory 51 / LCD blinking memory 19
    rw M51_LCDBM19 @ 0x52: u8 = 0_0 {
        /// LCD memory 51 / LCD blinking memory 19
        M51_LCDBM19: 0..7 = struct M51_LCDBM19Field(u8);
    }
    /// LCD memory 52 / LCD blinking memory 20
    rw M52_LCDBM20 @ 0x53: u8 = 0_0 {
        /// LCD memory 52 / LCD blinking memory 20
        M52_LCDBM20: 0..7 = struct M52_LCDBM20Field(u8);
    }
}
