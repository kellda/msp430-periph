//! LCD_C

utils::periph! {
    /// LCD_C
    LCD_C;
    /// LCD_C Control Register 0
    rw LCDCCTL0 @ 0x00: u16 = 0_0 {
        /// LCD_C LCD On
        LCDON: 0 = struct LCDON(bool);
        /// LCD_C Low Power Waveform
        LCDLP: 1 = struct LCDLP(bool);
        /// LCD_C LCD Segments On
        LCDSON: 2 = struct LCDSON(bool);
        /// LCD_C Mux Rate Bit: 0
        LCDMX0: 3 = struct LCDMX0(bool);
        /// LCD_C Mux Rate Bit: 1
        LCDMX1: 4 = struct LCDMX1(bool);
        /// LCD_C Mux Rate Bit: 2
        LCDMX2: 5 = struct LCDMX2(bool);
        /// LCD_C Clock Select
        LCDSSEL: 7 = struct LCDSSEL(bool);
        /// LCD_C LCD frequency pre-scaler Bit: 0
        LCDPRE: 8..10 = enum LCDPRE {
            /// LCD_C LCD frequency pre-scaler: /1
            LCDPRE_0 = 0b000,
            /// LCD_C LCD frequency pre-scaler: /2
            LCDPRE_1 = 0b001,
            /// LCD_C LCD frequency pre-scaler: /4
            LCDPRE_2 = 0b010,
            /// LCD_C LCD frequency pre-scaler: /8
            LCDPRE_3 = 0b011,
            /// LCD_C LCD frequency pre-scaler: /16
            LCDPRE_4 = 0b100,
            /// LCD_C LCD frequency pre-scaler: /32
            LCDPRE_5 = 0b101,
        }
        /// LCD_C LCD frequency divider Bit: 0
        LCDDIV: 11..15 = enum LCDDIV {
            /// LCD_C LCD frequency divider: /1
            LCDDIV_0 = 0b00000,
            /// LCD_C LCD frequency divider: /2
            LCDDIV_1 = 0b00001,
            /// LCD_C LCD frequency divider: /3
            LCDDIV_2 = 0b00010,
            /// LCD_C LCD frequency divider: /4
            LCDDIV_3 = 0b00011,
            /// LCD_C LCD frequency divider: /5
            LCDDIV_4 = 0b00100,
            /// LCD_C LCD frequency divider: /6
            LCDDIV_5 = 0b00101,
            /// LCD_C LCD frequency divider: /7
            LCDDIV_6 = 0b00110,
            /// LCD_C LCD frequency divider: /8
            LCDDIV_7 = 0b00111,
            /// LCD_C LCD frequency divider: /9
            LCDDIV_8 = 0b01000,
            /// LCD_C LCD frequency divider: /10
            LCDDIV_9 = 0b01001,
            /// LCD_C LCD frequency divider: /11
            LCDDIV_10 = 0b01010,
            /// LCD_C LCD frequency divider: /12
            LCDDIV_11 = 0b01011,
            /// LCD_C LCD frequency divider: /13
            LCDDIV_12 = 0b01100,
            /// LCD_C LCD frequency divider: /14
            LCDDIV_13 = 0b01101,
            /// LCD_C LCD frequency divider: /15
            LCDDIV_14 = 0b01110,
            /// LCD_C LCD frequency divider: /16
            LCDDIV_15 = 0b01111,
            /// LCD_C LCD frequency divider: /17
            LCDDIV_16 = 0b10000,
            /// LCD_C LCD frequency divider: /18
            LCDDIV_17 = 0b10001,
            /// LCD_C LCD frequency divider: /19
            LCDDIV_18 = 0b10010,
            /// LCD_C LCD frequency divider: /20
            LCDDIV_19 = 0b10011,
            /// LCD_C LCD frequency divider: /21
            LCDDIV_20 = 0b10100,
            /// LCD_C LCD frequency divider: /22
            LCDDIV_21 = 0b10101,
            /// LCD_C LCD frequency divider: /23
            LCDDIV_22 = 0b10110,
            /// LCD_C LCD frequency divider: /24
            LCDDIV_23 = 0b10111,
            /// LCD_C LCD frequency divider: /25
            LCDDIV_24 = 0b11000,
            /// LCD_C LCD frequency divider: /26
            LCDDIV_25 = 0b11001,
            /// LCD_C LCD frequency divider: /27
            LCDDIV_26 = 0b11010,
            /// LCD_C LCD frequency divider: /28
            LCDDIV_27 = 0b11011,
            /// LCD_C LCD frequency divider: /29
            LCDDIV_28 = 0b11100,
            /// LCD_C LCD frequency divider: /30
            LCDDIV_29 = 0b11101,
            /// LCD_C LCD frequency divider: /31
            LCDDIV_30 = 0b11110,
            /// LCD_C LCD frequency divider: /32
            LCDDIV_31 = 0b11111,
        }
    }
    /// LCD_C Control Register 1
    rw LCDCCTL1 @ 0x02: u16 = 0_0 {
        /// LCD_C LCD frame interrupt flag
        LCDFRMIFG: 0 = struct LCDFRMIFG(bool);
        /// LCD_C LCD blinking off interrupt flag
        LCDBLKOFFIFG: 1 = struct LCDBLKOFFIFG(bool);
        /// LCD_C LCD blinking on interrupt flag
        LCDBLKONIFG: 2 = struct LCDBLKONIFG(bool);
        /// LCD_C No cpacitance connected interrupt flag
        LCDNOCAPIFG: 3 = struct LCDNOCAPIFG(bool);
        /// LCD_C LCD frame interrupt enable
        LCDFRMIE: 8 = struct LCDFRMIE(bool);
        /// LCD_C LCD blinking off interrupt flag
        LCDBLKOFFIE: 9 = struct LCDBLKOFFIE(bool);
        /// LCD_C LCD blinking on interrupt flag
        LCDBLKONIE: 10 = struct LCDBLKONIE(bool);
        /// LCD_C No cpacitance connected interrupt enable
        LCDNOCAPIE: 11 = struct LCDNOCAPIE(bool);
    }
    /// LCD_C blinking control register
    rw LCDCBLKCTL @ 0x04: u16 = 0_0 {
        /// LCD_C Blinking mode Bit: 0
        LCDBLKMOD: 0..1 = enum LCDBLKMOD {
            /// LCD_C Blinking mode: Off
            LCDBLKMOD_0 = 0b00,
            /// LCD_C Blinking mode: Individual
            LCDBLKMOD_1 = 0b01,
            /// LCD_C Blinking mode: All
            LCDBLKMOD_2 = 0b10,
            /// LCD_C Blinking mode: Switching
            LCDBLKMOD_3 = 0b11,
        }
        /// LCD_C Clock pre-scaler for blinking frequency Bit: 0
        LCDBLKPRE: 2..4 = enum LCDBLKPRE {
            /// LCD_C Clock pre-scaler for blinking frequency: 0
            LCDBLKPRE_0 = 0b000,
            /// LCD_C Clock pre-scaler for blinking frequency: 1
            LCDBLKPRE_1 = 0b001,
            /// LCD_C Clock pre-scaler for blinking frequency: 2
            LCDBLKPRE_2 = 0b010,
            /// LCD_C Clock pre-scaler for blinking frequency: 3
            LCDBLKPRE_3 = 0b011,
            /// LCD_C Clock pre-scaler for blinking frequency: 4
            LCDBLKPRE_4 = 0b100,
            /// LCD_C Clock pre-scaler for blinking frequency: 5
            LCDBLKPRE_5 = 0b101,
            /// LCD_C Clock pre-scaler for blinking frequency: 6
            LCDBLKPRE_6 = 0b110,
            /// LCD_C Clock pre-scaler for blinking frequency: 7
            LCDBLKPRE_7 = 0b111,
        }
        /// LCD_C Clock divider for blinking frequency Bit: 0
        LCDBLKDIV: 5..7 = enum LCDBLKDIV {
            /// LCD_C Clock divider for blinking frequency: 0
            LCDBLKDIV_0 = 0b000,
            /// LCD_C Clock divider for blinking frequency: 1
            LCDBLKDIV_1 = 0b001,
            /// LCD_C Clock divider for blinking frequency: 2
            LCDBLKDIV_2 = 0b010,
            /// LCD_C Clock divider for blinking frequency: 3
            LCDBLKDIV_3 = 0b011,
            /// LCD_C Clock divider for blinking frequency: 4
            LCDBLKDIV_4 = 0b100,
            /// LCD_C Clock divider for blinking frequency: 5
            LCDBLKDIV_5 = 0b101,
            /// LCD_C Clock divider for blinking frequency: 6
            LCDBLKDIV_6 = 0b110,
            /// LCD_C Clock divider for blinking frequency: 7
            LCDBLKDIV_7 = 0b111,
        }
    }
    /// LCD_C memory control register
    rw LCDCMEMCTL @ 0x06: u16 = 0_0 {
        /// LCD_C LCD memory registers for display
        LCDDISP: 0 = struct LCDDISP(bool);
        /// LCD_C Clear LCD memory
        LCDCLRM: 1 = struct LCDCLRM(bool);
        /// LCD_C Clear LCD blinking memory
        LCDCLRBM: 2 = struct LCDCLRBM(bool);
    }
    /// LCD_C Voltage Control Register
    rw LCDCVCTL @ 0x08: u16 = 0_0 {
        /// Selects 1/2 bias.
        LCD2B: 0 = struct LCD2B(bool);
        /// Selects reference voltage for regulated charge pump: 0
        VLCDREF: 1..2 = enum VLCDREF {
            /// Internal
            VLCDREF_0 = 0b00,
            /// External
            VLCDREF_1 = 0b01,
            /// Reserved
            VLCDREF_2 = 0b10,
            /// Reserved
            VLCDREF_3 = 0b11,
        }
        /// LCD Voltage Charge Pump Enable.
        LCDCPEN: 3 = struct LCDCPEN(bool);
        /// Select external source for VLCD.
        VLCDEXT: 4 = struct VLCDEXT(bool);
        /// V2 - V4 voltage select.
        LCDEXTBIAS: 5 = struct LCDEXTBIAS(bool);
        /// Selects external connections for LCD mid voltages.
        R03EXT: 6 = struct R03EXT(bool);
        /// Selects external connection for lowest LCD voltage.
        LCDREXT: 7 = struct LCDREXT(bool);
        /// VLCD select: 0
        VLCD: 9..14 = enum VLCD {
            /// Charge pump disabled
            VLCD_0 = 0b000000,
            /// VLCD = 2.60V
            VLCD_1 = 0b000001,
            /// VLCD = 2.66V
            VLCD_2 = 0b000010,
            /// VLCD = 2.72V
            VLCD_3 = 0b000011,
            /// VLCD = 2.78V
            VLCD_4 = 0b000100,
            /// VLCD = 2.84V
            VLCD_5 = 0b000101,
            /// VLCD = 2.90V
            VLCD_6 = 0b000110,
            /// VLCD = 2.96V
            VLCD_7 = 0b000111,
            /// VLCD = 3.02V
            VLCD_8 = 0b001000,
            /// VLCD = 3.08V
            VLCD_9 = 0b001001,
            /// VLCD = 3.14V
            VLCD_10 = 0b001010,
            /// VLCD = 3.20V
            VLCD_11 = 0b001011,
            /// VLCD = 3.26V
            VLCD_12 = 0b001100,
            /// VLCD = 3.32V
            VLCD_13 = 0b001101,
            /// VLCD = 3.38V
            VLCD_14 = 0b001110,
            /// VLCD = 3.44V
            VLCD_15 = 0b001111,
        }
    }
    /// LCD_C Port Control Register 0
    rw LCDCPCTL0 @ 0x0a: u16 = 0_0 {
        /// LCD Segment  0 enable.
        LCDS0: 0 = struct LCDS0(bool);
        /// LCD Segment  1 enable.
        LCDS1: 1 = struct LCDS1(bool);
        /// LCD Segment  2 enable.
        LCDS2: 2 = struct LCDS2(bool);
        /// LCD Segment  3 enable.
        LCDS3: 3 = struct LCDS3(bool);
        /// LCD Segment  4 enable.
        LCDS4: 4 = struct LCDS4(bool);
        /// LCD Segment  5 enable.
        LCDS5: 5 = struct LCDS5(bool);
        /// LCD Segment  6 enable.
        LCDS6: 6 = struct LCDS6(bool);
        /// LCD Segment  7 enable.
        LCDS7: 7 = struct LCDS7(bool);
        /// LCD Segment  8 enable.
        LCDS8: 8 = struct LCDS8(bool);
        /// LCD Segment  9 enable.
        LCDS9: 9 = struct LCDS9(bool);
        /// LCD Segment 10 enable.
        LCDS10: 10 = struct LCDS10(bool);
        /// LCD Segment 11 enable.
        LCDS11: 11 = struct LCDS11(bool);
        /// LCD Segment 12 enable.
        LCDS12: 12 = struct LCDS12(bool);
        /// LCD Segment 13 enable.
        LCDS13: 13 = struct LCDS13(bool);
        /// LCD Segment 14 enable.
        LCDS14: 14 = struct LCDS14(bool);
        /// LCD Segment 15 enable.
        LCDS15: 15 = struct LCDS15(bool);
    }
    /// LCD_C Port Control Register 1
    rw LCDCPCTL1 @ 0x0c: u16 = 0_0 {
        /// LCD Segment 16 enable.
        LCDS16: 0 = struct LCDS16(bool);
        /// LCD Segment 17 enable.
        LCDS17: 1 = struct LCDS17(bool);
        /// LCD Segment 18 enable.
        LCDS18: 2 = struct LCDS18(bool);
        /// LCD Segment 19 enable.
        LCDS19: 3 = struct LCDS19(bool);
        /// LCD Segment 20 enable.
        LCDS20: 4 = struct LCDS20(bool);
        /// LCD Segment 21 enable.
        LCDS21: 5 = struct LCDS21(bool);
        /// LCD Segment 22 enable.
        LCDS22: 6 = struct LCDS22(bool);
        /// LCD Segment 23 enable.
        LCDS23: 7 = struct LCDS23(bool);
        /// LCD Segment 24 enable.
        LCDS24: 8 = struct LCDS24(bool);
        /// LCD Segment 25 enable.
        LCDS25: 9 = struct LCDS25(bool);
        /// LCD Segment 26 enable.
        LCDS26: 10 = struct LCDS26(bool);
        /// LCD Segment 27 enable.
        LCDS27: 11 = struct LCDS27(bool);
        /// LCD Segment 28 enable.
        LCDS28: 12 = struct LCDS28(bool);
        /// LCD Segment 29 enable.
        LCDS29: 13 = struct LCDS29(bool);
        /// LCD Segment 30 enable.
        LCDS30: 14 = struct LCDS30(bool);
        /// LCD Segment 31 enable.
        LCDS31: 15 = struct LCDS31(bool);
    }
    /// LCD_C Port Control Register 2
    rw LCDCPCTL2 @ 0x0e: u16 = 0_0 {
        /// LCD Segment 32 enable.
        LCDS32: 0 = struct LCDS32(bool);
        /// LCD Segment 33 enable.
        LCDS33: 1 = struct LCDS33(bool);
        /// LCD Segment 34 enable.
        LCDS34: 2 = struct LCDS34(bool);
        /// LCD Segment 35 enable.
        LCDS35: 3 = struct LCDS35(bool);
        /// LCD Segment 36 enable.
        LCDS36: 4 = struct LCDS36(bool);
        /// LCD Segment 37 enable.
        LCDS37: 5 = struct LCDS37(bool);
        /// LCD Segment 38 enable.
        LCDS38: 6 = struct LCDS38(bool);
        /// LCD Segment 39 enable.
        LCDS39: 7 = struct LCDS39(bool);
        /// LCD Segment 40 enable.
        LCDS40: 8 = struct LCDS40(bool);
        /// LCD Segment 41 enable.
        LCDS41: 9 = struct LCDS41(bool);
        /// LCD Segment 42 enable.
        LCDS42: 10 = struct LCDS42(bool);
        /// LCD Segment 43 enable.
        LCDS43: 11 = struct LCDS43(bool);
        /// LCD Segment 44 enable.
        LCDS44: 12 = struct LCDS44(bool);
        /// LCD Segment 45 enable.
        LCDS45: 13 = struct LCDS45(bool);
        /// LCD Segment 46 enable.
        LCDS46: 14 = struct LCDS46(bool);
        /// LCD Segment 47 enable.
        LCDS47: 15 = struct LCDS47(bool);
    }
    /// LCD_C Charge Pump Control Register 3
    rw LCDCCPCTL @ 0x12: u16 = 0_0 {
        /// LCD charge pump disable
        LCDCPDIS0: 0 = struct LCDCPDIS0(bool);
        /// LCD charge pump disable
        LCDCPDIS1: 1 = struct LCDCPDIS1(bool);
        /// LCD charge pump disable
        LCDCPDIS2: 2 = struct LCDCPDIS2(bool);
        /// LCD charge pump disable
        LCDCPDIS3: 3 = struct LCDCPDIS3(bool);
        /// LCD charge pump disable
        LCDCPDIS4: 4 = struct LCDCPDIS4(bool);
        /// LCD charge pump disable
        LCDCPDIS5: 5 = struct LCDCPDIS5(bool);
        /// LCD charge pump disable
        LCDCPDIS6: 6 = struct LCDCPDIS6(bool);
        /// LCD charge pump disable
        LCDCPDIS7: 7 = struct LCDCPDIS7(bool);
        /// LCD charge pump clock synchronization
        LCDCPCLKSYNC: 15 = struct LCDCPCLKSYNC(bool);
    }
    /// LCD_C Interrupt Vector Register
    rw LCDCIV @ 0x1e: u16 = 0_0 {
        /// LCD_C Interrupt Vector Register
        LCDCIV: 0..15 = struct LCDCIVField(u16);
    }
    /// LCD Memory 1
    rw LCDM1 @ 0x20: u8 = 0_0 {
        /// LCD Memory 1
        LCDM1: 0..7 = struct LCDM1Field(u8);
    }
    /// LCD Memory 2
    rw LCDM2 @ 0x21: u8 = 0_0 {
        /// LCD Memory 2
        LCDM2: 0..7 = struct LCDM2Field(u8);
    }
    /// LCD Memory 3
    rw LCDM3 @ 0x22: u8 = 0_0 {
        /// LCD Memory 3
        LCDM3: 0..7 = struct LCDM3Field(u8);
    }
    /// LCD Memory 4
    rw LCDM4 @ 0x23: u8 = 0_0 {
        /// LCD Memory 4
        LCDM4: 0..7 = struct LCDM4Field(u8);
    }
    /// LCD Memory 5
    rw LCDM5 @ 0x24: u8 = 0_0 {
        /// LCD Memory 5
        LCDM5: 0..7 = struct LCDM5Field(u8);
    }
    /// LCD Memory 6
    rw LCDM6 @ 0x25: u8 = 0_0 {
        /// LCD Memory 6
        LCDM6: 0..7 = struct LCDM6Field(u8);
    }
    /// LCD Memory 7
    rw LCDM7 @ 0x26: u8 = 0_0 {
        /// LCD Memory 7
        LCDM7: 0..7 = struct LCDM7Field(u8);
    }
    /// LCD Memory 8
    rw LCDM8 @ 0x27: u8 = 0_0 {
        /// LCD Memory 8
        LCDM8: 0..7 = struct LCDM8Field(u8);
    }
    /// LCD Memory 9
    rw LCDM9 @ 0x28: u8 = 0_0 {
        /// LCD Memory 9
        LCDM9: 0..7 = struct LCDM9Field(u8);
    }
    /// LCD Memory 10
    rw LCDM10 @ 0x29: u8 = 0_0 {
        /// LCD Memory 10
        LCDM10: 0..7 = struct LCDM10Field(u8);
    }
    /// LCD Memory 11
    rw LCDM11 @ 0x2a: u8 = 0_0 {
        /// LCD Memory 11
        LCDM11: 0..7 = struct LCDM11Field(u8);
    }
    /// LCD Memory 12
    rw LCDM12 @ 0x2b: u8 = 0_0 {
        /// LCD Memory 12
        LCDM12: 0..7 = struct LCDM12Field(u8);
    }
    /// LCD Memory 13
    rw LCDM13 @ 0x2c: u8 = 0_0 {
        /// LCD Memory 13
        LCDM13: 0..7 = struct LCDM13Field(u8);
    }
    /// LCD Memory 14
    rw LCDM14 @ 0x2d: u8 = 0_0 {
        /// LCD Memory 14
        LCDM14: 0..7 = struct LCDM14Field(u8);
    }
    /// LCD Memory 15
    rw LCDM15 @ 0x2e: u8 = 0_0 {
        /// LCD Memory 15
        LCDM15: 0..7 = struct LCDM15Field(u8);
    }
    /// LCD Memory 16
    rw LCDM16 @ 0x2f: u8 = 0_0 {
        /// LCD Memory 16
        LCDM16: 0..7 = struct LCDM16Field(u8);
    }
    /// LCD Memory 17
    rw LCDM17 @ 0x30: u8 = 0_0 {
        /// LCD Memory 17
        LCDM17: 0..7 = struct LCDM17Field(u8);
    }
    /// LCD Memory 18
    rw LCDM18 @ 0x31: u8 = 0_0 {
        /// LCD Memory 18
        LCDM18: 0..7 = struct LCDM18Field(u8);
    }
    /// LCD Memory 19
    rw LCDM19 @ 0x32: u8 = 0_0 {
        /// LCD Memory 19
        LCDM19: 0..7 = struct LCDM19Field(u8);
    }
    /// LCD Memory 20
    rw LCDM20 @ 0x33: u8 = 0_0 {
        /// LCD Memory 20
        LCDM20: 0..7 = struct LCDM20Field(u8);
    }
    /// LCD Memory 21
    rw LCDM21 @ 0x34: u8 = 0_0 {
        /// LCD Memory 21
        LCDM21: 0..7 = struct LCDM21Field(u8);
    }
    /// LCD Memory 22
    rw LCDM22 @ 0x35: u8 = 0_0 {
        /// LCD Memory 22
        LCDM22: 0..7 = struct LCDM22Field(u8);
    }
    /// LCD Memory 23
    rw LCDM23 @ 0x36: u8 = 0_0 {
        /// LCD Memory 23
        LCDM23: 0..7 = struct LCDM23Field(u8);
    }
    /// LCD Memory 24
    rw LCDM24 @ 0x37: u8 = 0_0 {
        /// LCD Memory 24
        LCDM24: 0..7 = struct LCDM24Field(u8);
    }
    /// LCD Memory 25
    rw LCDM25 @ 0x38: u8 = 0_0 {
        /// LCD Memory 25
        LCDM25: 0..7 = struct LCDM25Field(u8);
    }
    /// LCD Memory 26
    rw LCDM26 @ 0x39: u8 = 0_0 {
        /// LCD Memory 26
        LCDM26: 0..7 = struct LCDM26Field(u8);
    }
    /// LCD Memory 27
    rw LCDM27 @ 0x3a: u8 = 0_0 {
        /// LCD Memory 27
        LCDM27: 0..7 = struct LCDM27Field(u8);
    }
    /// LCD Memory 28
    rw LCDM28 @ 0x3b: u8 = 0_0 {
        /// LCD Memory 28
        LCDM28: 0..7 = struct LCDM28Field(u8);
    }
    /// LCD Memory 29
    rw LCDM29 @ 0x3c: u8 = 0_0 {
        /// LCD Memory 29
        LCDM29: 0..7 = struct LCDM29Field(u8);
    }
    /// LCD Memory 30
    rw LCDM30 @ 0x3d: u8 = 0_0 {
        /// LCD Memory 30
        LCDM30: 0..7 = struct LCDM30Field(u8);
    }
    /// LCD Memory 31
    rw LCDM31 @ 0x3e: u8 = 0_0 {
        /// LCD Memory 31
        LCDM31: 0..7 = struct LCDM31Field(u8);
    }
    /// LCD Memory 32
    rw LCDM32 @ 0x3f: u8 = 0_0 {
        /// LCD Memory 32
        LCDM32: 0..7 = struct LCDM32Field(u8);
    }
    /// LCD Memory 33
    rw LCDM33 @ 0x40: u8 = 0_0 {
        /// LCD Memory 33
        LCDM33: 0..7 = struct LCDM33Field(u8);
    }
    /// LCD Memory 34
    rw LCDM34 @ 0x41: u8 = 0_0 {
        /// LCD Memory 34
        LCDM34: 0..7 = struct LCDM34Field(u8);
    }
    /// LCD Memory 35
    rw LCDM35 @ 0x42: u8 = 0_0 {
        /// LCD Memory 35
        LCDM35: 0..7 = struct LCDM35Field(u8);
    }
    /// LCD Memory 36
    rw LCDM36 @ 0x43: u8 = 0_0 {
        /// LCD Memory 36
        LCDM36: 0..7 = struct LCDM36Field(u8);
    }
    /// LCD Memory 37
    rw LCDM37 @ 0x44: u8 = 0_0 {
        /// LCD Memory 37
        LCDM37: 0..7 = struct LCDM37Field(u8);
    }
    /// LCD Memory 38
    rw LCDM38 @ 0x45: u8 = 0_0 {
        /// LCD Memory 38
        LCDM38: 0..7 = struct LCDM38Field(u8);
    }
    /// LCD Memory 39
    rw LCDM39 @ 0x46: u8 = 0_0 {
        /// LCD Memory 39
        LCDM39: 0..7 = struct LCDM39Field(u8);
    }
    /// LCD Memory 40
    rw LCDM40 @ 0x47: u8 = 0_0 {
        /// LCD Memory 40
        LCDM40: 0..7 = struct LCDM40Field(u8);
    }
    /// LCD Blinking Memory 1
    rw LCDBM1 @ 0x40: u8 = 0_0 {
        /// LCD Blinking Memory 1
        LCDBM1: 0..7 = struct LCDBM1Field(u8);
    }
    /// LCD Blinking Memory 2
    rw LCDBM2 @ 0x41: u8 = 0_0 {
        /// LCD Blinking Memory 2
        LCDBM2: 0..7 = struct LCDBM2Field(u8);
    }
    /// LCD Blinking Memory 3
    rw LCDBM3 @ 0x42: u8 = 0_0 {
        /// LCD Blinking Memory 3
        LCDBM3: 0..7 = struct LCDBM3Field(u8);
    }
    /// LCD Blinking Memory 4
    rw LCDBM4 @ 0x43: u8 = 0_0 {
        /// LCD Blinking Memory 4
        LCDBM4: 0..7 = struct LCDBM4Field(u8);
    }
    /// LCD Blinking Memory 5
    rw LCDBM5 @ 0x44: u8 = 0_0 {
        /// LCD Blinking Memory 5
        LCDBM5: 0..7 = struct LCDBM5Field(u8);
    }
    /// LCD Blinking Memory 6
    rw LCDBM6 @ 0x45: u8 = 0_0 {
        /// LCD Blinking Memory 6
        LCDBM6: 0..7 = struct LCDBM6Field(u8);
    }
    /// LCD Blinking Memory 7
    rw LCDBM7 @ 0x46: u8 = 0_0 {
        /// LCD Blinking Memory 7
        LCDBM7: 0..7 = struct LCDBM7Field(u8);
    }
    /// LCD Blinking Memory 8
    rw LCDBM8 @ 0x47: u8 = 0_0 {
        /// LCD Blinking Memory 8
        LCDBM8: 0..7 = struct LCDBM8Field(u8);
    }
    /// LCD Blinking Memory 9
    rw LCDBM9 @ 0x48: u8 = 0_0 {
        /// LCD Blinking Memory 9
        LCDBM9: 0..7 = struct LCDBM9Field(u8);
    }
    /// LCD Blinking Memory 10
    rw LCDBM10 @ 0x49: u8 = 0_0 {
        /// LCD Blinking Memory 10
        LCDBM10: 0..7 = struct LCDBM10Field(u8);
    }
    /// LCD Blinking Memory 11
    rw LCDBM11 @ 0x4a: u8 = 0_0 {
        /// LCD Blinking Memory 11
        LCDBM11: 0..7 = struct LCDBM11Field(u8);
    }
    /// LCD Blinking Memory 12
    rw LCDBM12 @ 0x4b: u8 = 0_0 {
        /// LCD Blinking Memory 12
        LCDBM12: 0..7 = struct LCDBM12Field(u8);
    }
    /// LCD Blinking Memory 13
    rw LCDBM13 @ 0x4c: u8 = 0_0 {
        /// LCD Blinking Memory 13
        LCDBM13: 0..7 = struct LCDBM13Field(u8);
    }
    /// LCD Blinking Memory 14
    rw LCDBM14 @ 0x4d: u8 = 0_0 {
        /// LCD Blinking Memory 14
        LCDBM14: 0..7 = struct LCDBM14Field(u8);
    }
    /// LCD Blinking Memory 15
    rw LCDBM15 @ 0x4e: u8 = 0_0 {
        /// LCD Blinking Memory 15
        LCDBM15: 0..7 = struct LCDBM15Field(u8);
    }
    /// LCD Blinking Memory 16
    rw LCDBM16 @ 0x4f: u8 = 0_0 {
        /// LCD Blinking Memory 16
        LCDBM16: 0..7 = struct LCDBM16Field(u8);
    }
    /// LCD Blinking Memory 17
    rw LCDBM17 @ 0x50: u8 = 0_0 {
        /// LCD Blinking Memory 17
        LCDBM17: 0..7 = struct LCDBM17Field(u8);
    }
    /// LCD Blinking Memory 18
    rw LCDBM18 @ 0x51: u8 = 0_0 {
        /// LCD Blinking Memory 18
        LCDBM18: 0..7 = struct LCDBM18Field(u8);
    }
    /// LCD Blinking Memory 19
    rw LCDBM19 @ 0x52: u8 = 0_0 {
        /// LCD Blinking Memory 19
        LCDBM19: 0..7 = struct LCDBM19Field(u8);
    }
    /// LCD Blinking Memory 20
    rw LCDBM20 @ 0x53: u8 = 0_0 {
        /// LCD Blinking Memory 20
        LCDBM20: 0..7 = struct LCDBM20Field(u8);
    }
}
