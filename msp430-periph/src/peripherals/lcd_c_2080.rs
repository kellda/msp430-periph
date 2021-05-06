//! LCD_C

utils::periph! {
    /// LCD_C
    LCD_C;
    /// LCD_C control 0
    rw LCDCCTL0 @ 0x00: u16 = 0_0 {
        /// LCD on
        LCDON: 0..0 = enum LCDON {
            /// LCD_C module off
            OFF = 0b0,
            /// LCD_C module on
            ON = 0b1,
        }
        /// LCD low-power waveform
        LCDLP: 1..1 = enum LCDLP {
            /// Standard LCD waveforms on segment and common lines selected
            LCDLP_0 = 0b0,
            /// Low-power LCD waveforms on segment and common lines selected
            LCDLP_1 = 0b1,
        }
        /// LCD segments on
        LCDSON: 2..2 = enum LCDSON {
            /// All LCD segments are off
            OFF = 0b0,
            /// All LCD segments are enabled and on or off according to their corresponding memory location
            ON = 0b1,
        }
        /// LCD mux rate
        LCDMX: 3..5 = enum LCDMX {
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
        LCDSSEL: 7..7 = enum LCDSSEL {
            /// ACLK (30 kHz to 40 kHz)
            ACLK = 0b0,
            /// VLOCLK
            VLOCLK = 0b1,
        }
        /// LCD frequency pre-scaler
        LCDPRE: 8..10 = enum LCDPRE {
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
        LCDDIV: 11..15 = enum LCDDIV {
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
    rw LCDCCTL1 @ 0x02: u16 = 0_0 {
        /// LCD frame interrupt flag
        LCDFRMIFG: 0..0 = enum LCDFRMIFG {
            /// No interrupt pending
            LCDFRMIFG_0 = 0b0,
            /// Interrupt pending
            LCDFRMIFG_1 = 0b1,
        }
        /// LCD blinking interrupt flag, segments switched off
        LCDBLKOFFIFG: 1..1 = enum LCDBLKOFFIFG {
            /// No interrupt pending
            LCDBLKOFFIFG_0 = 0b0,
            /// Interrupt pending
            LCDBLKOFFIFG_1 = 0b1,
        }
        /// LCD blinking interrupt flag, segments switched on
        LCDBLKONIFG: 2..2 = enum LCDBLKONIFG {
            /// No interrupt pending
            LCDBLKONIFG_0 = 0b0,
            /// Interrupt pending
            LCDBLKONIFG_1 = 0b1,
        }
        /// No capacitance connected interrupt flag
        LCDNOCAPIFG: 3..3 = enum LCDNOCAPIFG {
            /// No interrupt pending
            LCDNOCAPIFG_0 = 0b0,
            /// Interrupt pending
            LCDNOCAPIFG_1 = 0b1,
        }
        /// LCD frame interrupt enable
        LCDFRMIE: 8..8 = enum LCDFRMIE {
            /// Interrupt disabled
            LCDFRMIE_0 = 0b0,
            /// Interrupt enabled
            LCDFRMIE_1 = 0b1,
        }
        /// LCD blinking interrupt enable, segments switched off
        LCDBLKOFFIE: 9..9 = enum LCDBLKOFFIE {
            /// Interrupt disabled
            LCDBLKOFFIE_0 = 0b0,
            /// Interrupt enabled
            LCDBLKOFFIE_1 = 0b1,
        }
        /// LCD blinking interrupt enable, segments switched on
        LCDBLKONIE: 10..10 = enum LCDBLKONIE {
            /// Interrupt disabled
            LCDBLKONIE_0 = 0b0,
            /// Interrupt enabled
            LCDBLKONIE_1 = 0b1,
        }
        /// No capacitance connected interrupt enable
        LCDNOCAPIE: 11..11 = enum LCDNOCAPIE {
            /// Interrupt disabled
            LCDNOCAPIE_0 = 0b0,
            /// Interrupt enabled
            LCDNOCAPIE_1 = 0b1,
        }
    }
    /// LCD_C blinking control
    rw LCDCBLKCTL @ 0x04: u16 = 0_0 {
        /// Blinking mode
        LCDBLKMOD: 0..1 = enum LCDBLKMOD {
            /// Blinking disabled
            LCDBLKMOD_0 = 0b00,
            /// Blinking of individual segments as enabled in blinking memory register LCDBMx. In mux mode 5 blinking is disabled.
            LCDBLKMOD_1 = 0b01,
            /// Blinking of all segments
            LCDBLKMOD_2 = 0b10,
            /// Switching between display contents as stored in LCDMx and LCDBMx memory registers. In mux mode 5 blinking is disabled.
            LCDBLKMOD_3 = 0b11,
        }
        /// Clock pre-scaler for blinking frequency
        LCDBLKPRE: 2..4 = enum LCDBLKPRE {
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
        LCDBLKDIV: 5..7 = enum LCDBLKDIV {
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
    rw LCDCMEMCTL @ 0x06: u16 = 0_0 {
        /// Select LCD memory registers for display
        LCDDISP: 0..0 = enum LCDDISP {
            /// Display content of LCD memory registers LCDM
            LCDDISP_0 = 0b0,
            /// Display content of LCD blinking memory registers LCDBM
            LCDDISP_1 = 0b1,
        }
        /// Clear LCD memory
        LCDCLRM: 1..1 = enum LCDCLRM {
            /// Contents of LCD memory registers LCDMx remain unchanged
            LCDCLRM_0 = 0b0,
            /// Clear content of all LCD memory registers LCDM
            LCDCLRM_1 = 0b1,
        }
        /// Clear LCD blinking memory
        LCDCLRBM: 2..2 = enum LCDCLRBM {
            /// Contents of blinking memory registers LCDBM remain unchanged
            LCDCLRBM_0 = 0b0,
            /// Clear content of all blinking memory registers LCDBM
            LCDCLRBM_1 = 0b1,
        }
    }
    /// LCD_C Voltage Control Register
    rw LCDCVCTL @ 0x08: u16 = 0_0 {
        /// Bias select
        LCD2B: 0..0 = enum LCD2B {
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
        LCDCPEN: 3..3 = enum LCDCPEN {
            /// Charge pump disabled
            LCDCPEN_0 = 0b0,
            /// Charge pump enabled when VLCD is generated internally (VLCDEXT = 0) and VLCD  0 or VLCDREF  0
            LCDCPEN_1 = 0b1,
        }
        /// VLCD source select
        VLCDEXT: 4..4 = enum VLCDEXT {
            /// VLCD is generated internally
            VLCDEXT_0 = 0b0,
            /// VLCD is sourced externally
            VLCDEXT_1 = 0b1,
        }
        /// V2 to V4 voltage select
        LCDEXTBIAS: 5..5 = enum LCDEXTBIAS {
            /// V2 to V4 are generated internally
            LCDEXTBIAS_0 = 0b0,
            /// V2 to V4 are sourced externally and the internal bias generator is switched off
            LCDEXTBIAS_1 = 0b1,
        }
        /// V5 voltage select
        R03EXT: 6..6 = enum R03EXT {
            /// V5 is VSS
            VSS = 0b0,
            /// V5 is sourced from the R03 pin
            R03 = 0b1,
        }
        /// V2 to V4 voltage on external Rx3 pins
        LCDREXT: 7..7 = enum LCDREXT {
            /// Internally generated V2 to V4 are not switched to pins (LCDEXTBIAS = 0)
            LCDREXT_0 = 0b0,
            /// Internally generated V2 to V4 are switched to pins (LCDEXTBIAS = 0)
            LCDREXT_1 = 0b1,
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
    rw LCDCPCTL0 @ 0x0a: u16 = 0_0 {
        /// LCD segment line 0 enable
        LCDS0: 0..0 = enum LCDS0 {
            /// Multiplexed pins are port functions
            LCDS0_0 = 0b0,
            /// Pins are LCD functions
            LCDS0_1 = 0b1,
        }
        /// LCD segment line 1 enable
        LCDS1: 1..1 = enum LCDS1 {
            /// Multiplexed pins are port functions
            LCDS1_0 = 0b0,
            /// Pins are LCD functions
            LCDS1_1 = 0b1,
        }
        /// LCD segment line 2 enable
        LCDS2: 2..2 = enum LCDS2 {
            /// Multiplexed pins are port functions
            LCDS2_0 = 0b0,
            /// Pins are LCD functions
            LCDS2_1 = 0b1,
        }
        /// LCD segment line 3 enable
        LCDS3: 3..3 = enum LCDS3 {
            /// Multiplexed pins are port functions
            LCDS3_0 = 0b0,
            /// Pins are LCD functions
            LCDS3_1 = 0b1,
        }
        /// LCD segment line 4 enable
        LCDS4: 4..4 = enum LCDS4 {
            /// Multiplexed pins are port functions
            LCDS4_0 = 0b0,
            /// Pins are LCD functions
            LCDS4_1 = 0b1,
        }
        /// LCD segment line 5 enable
        LCDS5: 5..5 = enum LCDS5 {
            /// Multiplexed pins are port functions
            LCDS5_0 = 0b0,
            /// Pins are LCD functions
            LCDS5_1 = 0b1,
        }
        /// LCD segment line 6 enable
        LCDS6: 6..6 = enum LCDS6 {
            /// Multiplexed pins are port functions
            LCDS6_0 = 0b0,
            /// Pins are LCD functions
            LCDS6_1 = 0b1,
        }
        /// LCD segment line 7 enable
        LCDS7: 7..7 = enum LCDS7 {
            /// Multiplexed pins are port functions
            LCDS7_0 = 0b0,
            /// Pins are LCD functions
            LCDS7_1 = 0b1,
        }
        /// LCD segment line 8 enable
        LCDS8: 8..8 = enum LCDS8 {
            /// Multiplexed pins are port functions
            LCDS8_0 = 0b0,
            /// Pins are LCD functions
            LCDS8_1 = 0b1,
        }
        /// LCD segment line 9 enable
        LCDS9: 9..9 = enum LCDS9 {
            /// Multiplexed pins are port functions
            LCDS9_0 = 0b0,
            /// Pins are LCD functions
            LCDS9_1 = 0b1,
        }
        /// LCD segment line 10 enable
        LCDS10: 10..10 = enum LCDS10 {
            /// Multiplexed pins are port functions
            LCDS10_0 = 0b0,
            /// Pins are LCD functions
            LCDS10_1 = 0b1,
        }
        /// LCD segment line 11 enable
        LCDS11: 11..11 = enum LCDS11 {
            /// Multiplexed pins are port functions
            LCDS11_0 = 0b0,
            /// Pins are LCD functions
            LCDS11_1 = 0b1,
        }
        /// LCD segment line 12 enable
        LCDS12: 12..12 = enum LCDS12 {
            /// Multiplexed pins are port functions
            LCDS12_0 = 0b0,
            /// Pins are LCD functions
            LCDS12_1 = 0b1,
        }
        /// LCD segment line 13 enable
        LCDS13: 13..13 = enum LCDS13 {
            /// Multiplexed pins are port functions
            LCDS13_0 = 0b0,
            /// Pins are LCD functions
            LCDS13_1 = 0b1,
        }
        /// LCD segment line 14 enable
        LCDS14: 14..14 = enum LCDS14 {
            /// Multiplexed pins are port functions
            LCDS14_0 = 0b0,
            /// Pins are LCD functions
            LCDS14_1 = 0b1,
        }
        /// LCD segment line 15 enable
        LCDS15: 15..15 = enum LCDS15 {
            /// Multiplexed pins are port functions
            LCDS15_0 = 0b0,
            /// Pins are LCD functions
            LCDS15_1 = 0b1,
        }
    }
    /// LCD_C port control 1
    rw LCDCPCTL1 @ 0x0c: u16 = 0_0 {
        /// LCD segment line 16 enable
        LCDS16: 0..0 = enum LCDS16 {
            /// Multiplexed pins are port functions
            LCDS16_0 = 0b0,
            /// Pins are LCD functions
            LCDS16_1 = 0b1,
        }
        /// LCD segment line 17 enable
        LCDS17: 1..1 = enum LCDS17 {
            /// Multiplexed pins are port functions
            LCDS17_0 = 0b0,
            /// Pins are LCD functions
            LCDS17_1 = 0b1,
        }
        /// LCD segment line 18 enable
        LCDS18: 2..2 = enum LCDS18 {
            /// Multiplexed pins are port functions
            LCDS18_0 = 0b0,
            /// Pins are LCD functions
            LCDS18_1 = 0b1,
        }
        /// LCD segment line 19 enable
        LCDS19: 3..3 = enum LCDS19 {
            /// Multiplexed pins are port functions
            LCDS19_0 = 0b0,
            /// Pins are LCD functions
            LCDS19_1 = 0b1,
        }
        /// LCD segment line 20 enable
        LCDS20: 4..4 = enum LCDS20 {
            /// Multiplexed pins are port functions
            LCDS20_0 = 0b0,
            /// Pins are LCD functions
            LCDS20_1 = 0b1,
        }
        /// LCD segment line 21 enable
        LCDS21: 5..5 = enum LCDS21 {
            /// Multiplexed pins are port functions
            LCDS21_0 = 0b0,
            /// Pins are LCD functions
            LCDS21_1 = 0b1,
        }
        /// LCD segment line 22 enable
        LCDS22: 6..6 = enum LCDS22 {
            /// Multiplexed pins are port functions
            LCDS22_0 = 0b0,
            /// Pins are LCD functions
            LCDS22_1 = 0b1,
        }
        /// LCD segment line 23 enable
        LCDS23: 7..7 = enum LCDS23 {
            /// Multiplexed pins are port functions
            LCDS23_0 = 0b0,
            /// Pins are LCD functions
            LCDS23_1 = 0b1,
        }
        /// LCD segment line 24 enable
        LCDS24: 8..8 = enum LCDS24 {
            /// Multiplexed pins are port functions
            LCDS24_0 = 0b0,
            /// Pins are LCD functions
            LCDS24_1 = 0b1,
        }
        /// LCD segment line 25 enable
        LCDS25: 9..9 = enum LCDS25 {
            /// Multiplexed pins are port functions
            LCDS25_0 = 0b0,
            /// Pins are LCD functions
            LCDS25_1 = 0b1,
        }
        /// LCD segment line 26 enable
        LCDS26: 10..10 = enum LCDS26 {
            /// Multiplexed pins are port functions
            LCDS26_0 = 0b0,
            /// Pins are LCD functions
            LCDS26_1 = 0b1,
        }
        /// LCD segment line 27 enable
        LCDS27: 11..11 = enum LCDS27 {
            /// Multiplexed pins are port functions
            LCDS27_0 = 0b0,
            /// Pins are LCD functions
            LCDS27_1 = 0b1,
        }
        /// LCD segment line 28 enable
        LCDS28: 12..12 = enum LCDS28 {
            /// Multiplexed pins are port functions
            LCDS28_0 = 0b0,
            /// Pins are LCD functions
            LCDS28_1 = 0b1,
        }
        /// LCD segment line 29 enable
        LCDS29: 13..13 = enum LCDS29 {
            /// Multiplexed pins are port functions
            LCDS29_0 = 0b0,
            /// Pins are LCD functions
            LCDS29_1 = 0b1,
        }
        /// LCD segment line 30 enable
        LCDS30: 14..14 = enum LCDS30 {
            /// Multiplexed pins are port functions
            LCDS30_0 = 0b0,
            /// Pins are LCD functions
            LCDS30_1 = 0b1,
        }
        /// LCD segment line 31 enable
        LCDS31: 15..15 = enum LCDS31 {
            /// Multiplexed pins are port functions
            LCDS31_0 = 0b0,
            /// Pins are LCD functions
            LCDS31_1 = 0b1,
        }
    }
    /// LCD_C port control 2 (256 segments)
    rw LCDCPCTL2 @ 0x0e: u16 = 0_0 {
        /// LCD segment line 32 enable
        LCDS32: 0..0 = enum LCDS32 {
            /// Multiplexed pins are port functions
            LCDS32_0 = 0b0,
            /// Pins are LCD functions
            LCDS32_1 = 0b1,
        }
        /// LCD segment line 33 enable
        LCDS33: 1..1 = enum LCDS33 {
            /// Multiplexed pins are port functions
            LCDS33_0 = 0b0,
            /// Pins are LCD functions
            LCDS33_1 = 0b1,
        }
        /// LCD segment line 34 enable
        LCDS34: 2..2 = enum LCDS34 {
            /// Multiplexed pins are port functions
            LCDS34_0 = 0b0,
            /// Pins are LCD functions
            LCDS34_1 = 0b1,
        }
        /// LCD segment line 35 enable
        LCDS35: 3..3 = enum LCDS35 {
            /// Multiplexed pins are port functions
            LCDS35_0 = 0b0,
            /// Pins are LCD functions
            LCDS35_1 = 0b1,
        }
        /// LCD segment line 36 enable
        LCDS36: 4..4 = enum LCDS36 {
            /// Multiplexed pins are port functions
            LCDS36_0 = 0b0,
            /// Pins are LCD functions
            LCDS36_1 = 0b1,
        }
        /// LCD segment line 37 enable
        LCDS37: 5..5 = enum LCDS37 {
            /// Multiplexed pins are port functions
            LCDS37_0 = 0b0,
            /// Pins are LCD functions
            LCDS37_1 = 0b1,
        }
        /// LCD segment line 38 enable
        LCDS38: 6..6 = enum LCDS38 {
            /// Multiplexed pins are port functions
            LCDS38_0 = 0b0,
            /// Pins are LCD functions
            LCDS38_1 = 0b1,
        }
        /// LCD segment line 39 enable
        LCDS39: 7..7 = enum LCDS39 {
            /// Multiplexed pins are port functions
            LCDS39_0 = 0b0,
            /// Pins are LCD functions
            LCDS39_1 = 0b1,
        }
        /// LCD segment line 40 enable
        LCDS40: 8..8 = enum LCDS40 {
            /// Multiplexed pins are port functions
            LCDS40_0 = 0b0,
            /// Pins are LCD functions
            LCDS40_1 = 0b1,
        }
        /// LCD segment line 41 enable
        LCDS41: 9..9 = enum LCDS41 {
            /// Multiplexed pins are port functions
            LCDS41_0 = 0b0,
            /// Pins are LCD functions
            LCDS41_1 = 0b1,
        }
        /// LCD segment line 42 enable
        LCDS42: 10..10 = enum LCDS42 {
            /// Multiplexed pins are port functions
            LCDS42_0 = 0b0,
            /// Pins are LCD functions
            LCDS42_1 = 0b1,
        }
        /// LCD segment line 43 enable
        LCDS43: 11..11 = enum LCDS43 {
            /// Multiplexed pins are port functions
            LCDS43_0 = 0b0,
            /// Pins are LCD functions
            LCDS43_1 = 0b1,
        }
        /// LCD segment line 44 enable
        LCDS44: 12..12 = enum LCDS44 {
            /// Multiplexed pins are port functions
            LCDS44_0 = 0b0,
            /// Pins are LCD functions
            LCDS44_1 = 0b1,
        }
        /// LCD segment line 45 enable
        LCDS45: 13..13 = enum LCDS45 {
            /// Multiplexed pins are port functions
            LCDS45_0 = 0b0,
            /// Pins are LCD functions
            LCDS45_1 = 0b1,
        }
        /// LCD segment line 46 enable
        LCDS46: 14..14 = enum LCDS46 {
            /// Multiplexed pins are port functions
            LCDS46_0 = 0b0,
            /// Pins are LCD functions
            LCDS46_1 = 0b1,
        }
        /// LCD segment line 47 enable
        LCDS47: 15..15 = enum LCDS47 {
            /// Multiplexed pins are port functions
            LCDS47_0 = 0b0,
            /// Pins are LCD functions
            LCDS47_1 = 0b1,
        }
    }
    /// LCD_C port control 3 (384 segments)
    rw LCDCPCTL3 @ 0x10: u16 = 0_0 {
        /// LCD segment line 48 enable
        LCDS48: 0..0 = enum LCDS48 {
            /// Multiplexed pins are port functions
            LCDS48_0 = 0b0,
            /// Pins are LCD functions
            LCDS48_1 = 0b1,
        }
        /// LCD segment line 49 enable
        LCDS49: 1..1 = enum LCDS49 {
            /// Multiplexed pins are port functions
            LCDS49_0 = 0b0,
            /// Pins are LCD functions
            LCDS49_1 = 0b1,
        }
        /// LCD segment line 50 enable
        LCDS50: 2..2 = enum LCDS50 {
            /// Multiplexed pins are port functions
            LCDS50_0 = 0b0,
            /// Pins are LCD functions
            LCDS50_1 = 0b1,
        }
        /// LCD segment line 51 enable
        LCDS51: 3..3 = enum LCDS51 {
            /// Multiplexed pins are port functions
            LCDS51_0 = 0b0,
            /// Pins are LCD functions
            LCDS51_1 = 0b1,
        }
        /// LCD segment line 52 enable
        LCDS52: 4..4 = enum LCDS52 {
            /// Multiplexed pins are port functions
            LCDS52_0 = 0b0,
            /// Pins are LCD functions
            LCDS52_1 = 0b1,
        }
        /// LCD segment line 53 enable
        LCDS53: 5..5 = enum LCDS53 {
            /// Multiplexed pins are port functions
            LCDS53_0 = 0b0,
            /// Pins are LCD functions
            LCDS53_1 = 0b1,
        }
    }
    /// LCD_C charge pump control
    rw LCDCCPCTL @ 0x12: u16 = 0_0 {
        /// LCD charge pump disable
        LCDCPDIS: 0..7 = struct LCDCPDIS(u16);
        /// LCD charge pump clock synchronization
        LCDCPCLKSYNC: 15..15 = enum LCDCPCLKSYNC {
            /// Synchronization disabled
            LCDCPCLKSYNC_0 = 0b0,
            /// Synchronization enabled
            LCDCPCLKSYNC_1 = 0b1,
        }
    }
    /// LCD_C interrupt vector
    r LCDCIV @ 0x1e: u16 = 0_0 {
        /// LCD_C interrupt vector value
        LCDCIV: 0..15 = enum LCDCIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: No capacitor connected; Interrupt Flag: LCDNOCAPIFG; Interrupt Priority: Highest
            LCDNOCAPIFG = 0b0000000000000010,
            /// Interrupt Source: Blink, segments off; Interrupt Flag: LCDBLKOFFIFG
            LCDBLKOFFIFG = 0b0000000000000100,
            /// Interrupt Source: Blink, segments on; Interrupt Flag: LCDBLKONIFG
            LCDBLKONIFG = 0b0000000000000110,
            /// Interrupt Source: Frame interrupt; Interrupt Flag: LCDFRMIFG; Interrupt Priority: Lowest
            LCDFRMIFG = 0b0000000000001000,
        }
    }
    /// LCD memory 1
    rw LCDM1 @ 0x20: u8 = 0_0 {
        /// LCD memory 1
        LCDM1: 0..7 = struct LCDM1Field(u8);
    }
    /// LCD memory 2
    rw LCDM2 @ 0x21: u8 = 0_0 {
        /// LCD memory 2
        LCDM2: 0..7 = struct LCDM2Field(u8);
    }
    /// LCD memory 3
    rw LCDM3 @ 0x22: u8 = 0_0 {
        /// LCD memory 3
        LCDM3: 0..7 = struct LCDM3Field(u8);
    }
    /// LCD memory 4
    rw LCDM4 @ 0x23: u8 = 0_0 {
        /// LCD memory 4
        LCDM4: 0..7 = struct LCDM4Field(u8);
    }
    /// LCD memory 5
    rw LCDM5 @ 0x24: u8 = 0_0 {
        /// LCD memory 5
        LCDM5: 0..7 = struct LCDM5Field(u8);
    }
    /// LCD memory 6
    rw LCDM6 @ 0x25: u8 = 0_0 {
        /// LCD memory 6
        LCDM6: 0..7 = struct LCDM6Field(u8);
    }
    /// LCD memory 7
    rw LCDM7 @ 0x26: u8 = 0_0 {
        /// LCD memory 7
        LCDM7: 0..7 = struct LCDM7Field(u8);
    }
    /// LCD memory 8
    rw LCDM8 @ 0x27: u8 = 0_0 {
        /// LCD memory 8
        LCDM8: 0..7 = struct LCDM8Field(u8);
    }
    /// LCD memory 9
    rw LCDM9 @ 0x28: u8 = 0_0 {
        /// LCD memory 9
        LCDM9: 0..7 = struct LCDM9Field(u8);
    }
    /// LCD memory 10
    rw LCDM10 @ 0x29: u8 = 0_0 {
        /// LCD memory 10
        LCDM10: 0..7 = struct LCDM10Field(u8);
    }
    /// LCD memory 11
    rw LCDM11 @ 0x2a: u8 = 0_0 {
        /// LCD memory 11
        LCDM11: 0..7 = struct LCDM11Field(u8);
    }
    /// LCD memory 12
    rw LCDM12 @ 0x2b: u8 = 0_0 {
        /// LCD memory 12
        LCDM12: 0..7 = struct LCDM12Field(u8);
    }
    /// LCD memory 13
    rw LCDM13 @ 0x2c: u8 = 0_0 {
        /// LCD memory 13
        LCDM13: 0..7 = struct LCDM13Field(u8);
    }
    /// LCD memory 14
    rw LCDM14 @ 0x2d: u8 = 0_0 {
        /// LCD memory 14
        LCDM14: 0..7 = struct LCDM14Field(u8);
    }
    /// LCD memory 15
    rw LCDM15 @ 0x2e: u8 = 0_0 {
        /// LCD memory 15
        LCDM15: 0..7 = struct LCDM15Field(u8);
    }
    /// LCD memory 16
    rw LCDM16 @ 0x2f: u8 = 0_0 {
        /// LCD memory 16
        LCDM16: 0..7 = struct LCDM16Field(u8);
    }
    /// LCD memory 17
    rw LCDM17 @ 0x30: u8 = 0_0 {
        /// LCD memory 17
        LCDM17: 0..7 = struct LCDM17Field(u8);
    }
    /// LCD memory 18
    rw LCDM18 @ 0x31: u8 = 0_0 {
        /// LCD memory 18
        LCDM18: 0..7 = struct LCDM18Field(u8);
    }
    /// LCD memory 19
    rw LCDM19 @ 0x32: u8 = 0_0 {
        /// LCD memory 19
        LCDM19: 0..7 = struct LCDM19Field(u8);
    }
    /// LCD memory 20
    rw LCDM20 @ 0x33: u8 = 0_0 {
        /// LCD memory 20
        LCDM20: 0..7 = struct LCDM20Field(u8);
    }
    /// LCD memory 21
    rw LCDM21 @ 0x34: u8 = 0_0 {
        /// LCD memory 21
        LCDM21: 0..7 = struct LCDM21Field(u8);
    }
    /// LCD memory 22
    rw LCDM22 @ 0x35: u8 = 0_0 {
        /// LCD memory 22
        LCDM22: 0..7 = struct LCDM22Field(u8);
    }
    /// LCD memory 23
    rw LCDM23 @ 0x36: u8 = 0_0 {
        /// LCD memory 23
        LCDM23: 0..7 = struct LCDM23Field(u8);
    }
    /// LCD memory 24
    rw LCDM24 @ 0x37: u8 = 0_0 {
        /// LCD memory 24
        LCDM24: 0..7 = struct LCDM24Field(u8);
    }
    /// LCD memory 25
    rw LCDM25 @ 0x38: u8 = 0_0 {
        /// LCD memory 25
        LCDM25: 0..7 = struct LCDM25Field(u8);
    }
    /// LCD memory 26
    rw LCDM26 @ 0x39: u8 = 0_0 {
        /// LCD memory 26
        LCDM26: 0..7 = struct LCDM26Field(u8);
    }
    /// LCD memory 27
    rw LCDM27 @ 0x3a: u8 = 0_0 {
        /// LCD memory 27
        LCDM27: 0..7 = struct LCDM27Field(u8);
    }
    /// LCD memory 28
    rw LCDM28 @ 0x3b: u8 = 0_0 {
        /// LCD memory 28
        LCDM28: 0..7 = struct LCDM28Field(u8);
    }
    /// LCD memory 29
    rw LCDM29 @ 0x3c: u8 = 0_0 {
        /// LCD memory 29
        LCDM29: 0..7 = struct LCDM29Field(u8);
    }
    /// LCD memory 30
    rw LCDM30 @ 0x3d: u8 = 0_0 {
        /// LCD memory 30
        LCDM30: 0..7 = struct LCDM30Field(u8);
    }
    /// LCD memory 31
    rw LCDM31 @ 0x3e: u8 = 0_0 {
        /// LCD memory 31
        LCDM31: 0..7 = struct LCDM31Field(u8);
    }
    /// LCD memory 32
    rw LCDM32 @ 0x3f: u8 = 0_0 {
        /// LCD memory 32
        LCDM32: 0..7 = struct LCDM32Field(u8);
    }
    /// LCD memory 33 / LCD blinking memory 1
    rw LCDM33_LCDBM1 @ 0x40: u8 = 0_0 {
        /// LCD memory 33 / LCD blinking memory 1
        LCDM33_LCDBM1: 0..7 = struct LCDM33_LCDBM1Field(u8);
    }
    /// LCD memory 34 / LCD blinking memory 2
    rw LCDM34_LCDBM2 @ 0x41: u8 = 0_0 {
        /// LCD memory 34 / LCD blinking memory 2
        LCDM34_LCDBM2: 0..7 = struct LCDM34_LCDBM2Field(u8);
    }
    /// LCD memory 35 / LCD blinking memory 3
    rw LCDM35_LCDBM3 @ 0x42: u8 = 0_0 {
        /// LCD memory 35 / LCD blinking memory 3
        LCDM35_LCDBM3: 0..7 = struct LCDM35_LCDBM3Field(u8);
    }
    /// LCD memory 36 / LCD blinking memory 4
    rw LCDM36_LCDBM4 @ 0x43: u8 = 0_0 {
        /// LCD memory 36 / LCD blinking memory 4
        LCDM36_LCDBM4: 0..7 = struct LCDM36_LCDBM4Field(u8);
    }
    /// LCD memory 37 / LCD blinking memory 5
    rw LCDM37_LCDBM5 @ 0x44: u8 = 0_0 {
        /// LCD memory 37 / LCD blinking memory 5
        LCDM37_LCDBM5: 0..7 = struct LCDM37_LCDBM5Field(u8);
    }
    /// LCD memory 38 / LCD blinking memory 6
    rw LCDM38_LCDBM6 @ 0x45: u8 = 0_0 {
        /// LCD memory 38 / LCD blinking memory 6
        LCDM38_LCDBM6: 0..7 = struct LCDM38_LCDBM6Field(u8);
    }
    /// LCD memory 39 / LCD blinking memory 7
    rw LCDM39_LCDBM7 @ 0x46: u8 = 0_0 {
        /// LCD memory 39 / LCD blinking memory 7
        LCDM39_LCDBM7: 0..7 = struct LCDM39_LCDBM7Field(u8);
    }
    /// LCD memory 40 / LCD blinking memory 8
    rw LCDM40_LCDBM8 @ 0x47: u8 = 0_0 {
        /// LCD memory 40 / LCD blinking memory 8
        LCDM40_LCDBM8: 0..7 = struct LCDM40_LCDBM8Field(u8);
    }
    /// LCD memory 41 / LCD blinking memory 9
    rw LCDM41_LCDBM9 @ 0x48: u8 = 0_0 {
        /// LCD memory 41 / LCD blinking memory 9
        LCDM41_LCDBM9: 0..7 = struct LCDM41_LCDBM9Field(u8);
    }
    /// LCD memory 42 / LCD blinking memory 10
    rw LCDM42_LCDBM10 @ 0x49: u8 = 0_0 {
        /// LCD memory 42 / LCD blinking memory 10
        LCDM42_LCDBM10: 0..7 = struct LCDM42_LCDBM10Field(u8);
    }
    /// LCD memory 43 / LCD blinking memory 11
    rw LCDM43_LCDBM11 @ 0x4a: u8 = 0_0 {
        /// LCD memory 43 / LCD blinking memory 11
        LCDM43_LCDBM11: 0..7 = struct LCDM43_LCDBM11Field(u8);
    }
    /// LCD memory 44 / LCD blinking memory 11
    rw LCDM44_LCDBM12 @ 0x4b: u8 = 0_0 {
        /// LCD memory 44 / LCD blinking memory 11
        LCDM44_LCDBM12: 0..7 = struct LCDM44_LCDBM12Field(u8);
    }
    /// LCD memory 45 / LCD blinking memory 13
    rw LCDM45_LCDBM13 @ 0x4c: u8 = 0_0 {
        /// LCD memory 45 / LCD blinking memory 13
        LCDM45_LCDBM13: 0..7 = struct LCDM45_LCDBM13Field(u8);
    }
    /// LCD memory 46 / LCD blinking memory 14
    rw LCDM46_LCDBM14 @ 0x4d: u8 = 0_0 {
        /// LCD memory 46 / LCD blinking memory 14
        LCDM46_LCDBM14: 0..7 = struct LCDM46_LCDBM14Field(u8);
    }
    /// LCD memory 47 / LCD blinking memory 15
    rw LCDM47_LCDBM15 @ 0x4e: u8 = 0_0 {
        /// LCD memory 47 / LCD blinking memory 15
        LCDM47_LCDBM15: 0..7 = struct LCDM47_LCDBM15Field(u8);
    }
    /// LCD memory 48 / LCD blinking memory 16
    rw LCDM48_LCDBM16 @ 0x4f: u8 = 0_0 {
        /// LCD memory 48 / LCD blinking memory 16
        LCDM48_LCDBM16: 0..7 = struct LCDM48_LCDBM16Field(u8);
    }
    /// LCD memory 49 / LCD blinking memory 17
    rw LCDM49_LCDBM17 @ 0x50: u8 = 0_0 {
        /// LCD memory 49 / LCD blinking memory 17
        LCDM49_LCDBM17: 0..7 = struct LCDM49_LCDBM17Field(u8);
    }
    /// LCD memory 50 / LCD blinking memory 18
    rw LCDM50_LCDBM18 @ 0x51: u8 = 0_0 {
        /// LCD memory 50 / LCD blinking memory 18
        LCDM50_LCDBM18: 0..7 = struct LCDM50_LCDBM18Field(u8);
    }
    /// LCD memory 51 / LCD blinking memory 19
    rw LCDM51_LCDBM19 @ 0x52: u8 = 0_0 {
        /// LCD memory 51 / LCD blinking memory 19
        LCDM51_LCDBM19: 0..7 = struct LCDM51_LCDBM19Field(u8);
    }
    /// LCD memory 52 / LCD blinking memory 20
    rw LCDM52_LCDBM20 @ 0x53: u8 = 0_0 {
        /// LCD memory 52 / LCD blinking memory 20
        LCDM52_LCDBM20: 0..7 = struct LCDM52_LCDBM20Field(u8);
    }
}
