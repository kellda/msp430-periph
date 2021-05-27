//! LCD_B

utils::periph! {
    /// LCD_B
    LCD_B;
    /// LCD_B Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// LCD_B LCD On
        ON: 0 = struct ON(bool);
        /// LCD_B LCD Segments On
        SON: 2 = struct SON(bool);
        /// LCD_B Mux Rate Bit: 0
        MX0: 3 = struct MX0(bool);
        /// LCD_B Mux Rate Bit: 1
        MX1: 4 = struct MX1(bool);
        /// LCD_B Clock Select
        SSEL: 7 = struct SSEL(bool);
        /// LCD_B LCD frequency pre-scaler Bit: 0
        PRE: 8..10 = enum PRE {
            /// LCD_B LCD frequency pre-scaler: /1
            PRE_0 = 0b000,
            /// LCD_B LCD frequency pre-scaler: /2
            PRE_1 = 0b001,
            /// LCD_B LCD frequency pre-scaler: /4
            PRE_2 = 0b010,
            /// LCD_B LCD frequency pre-scaler: /8
            PRE_3 = 0b011,
            /// LCD_B LCD frequency pre-scaler: /16
            PRE_4 = 0b100,
            /// LCD_B LCD frequency pre-scaler: /32
            PRE_5 = 0b101,
        }
        /// LCD_B LCD frequency divider Bit: 0
        DIV: 11..15 = enum DIV {
            /// LCD_B LCD frequency divider: /1
            DIV_0 = 0b00000,
            /// LCD_B LCD frequency divider: /2
            DIV_1 = 0b00001,
            /// LCD_B LCD frequency divider: /3
            DIV_2 = 0b00010,
            /// LCD_B LCD frequency divider: /4
            DIV_3 = 0b00011,
            /// LCD_B LCD frequency divider: /5
            DIV_4 = 0b00100,
            /// LCD_B LCD frequency divider: /6
            DIV_5 = 0b00101,
            /// LCD_B LCD frequency divider: /7
            DIV_6 = 0b00110,
            /// LCD_B LCD frequency divider: /8
            DIV_7 = 0b00111,
            /// LCD_B LCD frequency divider: /9
            DIV_8 = 0b01000,
            /// LCD_B LCD frequency divider: /10
            DIV_9 = 0b01001,
            /// LCD_B LCD frequency divider: /11
            DIV_10 = 0b01010,
            /// LCD_B LCD frequency divider: /12
            DIV_11 = 0b01011,
            /// LCD_B LCD frequency divider: /13
            DIV_12 = 0b01100,
            /// LCD_B LCD frequency divider: /14
            DIV_13 = 0b01101,
            /// LCD_B LCD frequency divider: /15
            DIV_14 = 0b01110,
            /// LCD_B LCD frequency divider: /16
            DIV_15 = 0b01111,
            /// LCD_B LCD frequency divider: /17
            DIV_16 = 0b10000,
            /// LCD_B LCD frequency divider: /18
            DIV_17 = 0b10001,
            /// LCD_B LCD frequency divider: /19
            DIV_18 = 0b10010,
            /// LCD_B LCD frequency divider: /20
            DIV_19 = 0b10011,
            /// LCD_B LCD frequency divider: /21
            DIV_20 = 0b10100,
            /// LCD_B LCD frequency divider: /22
            DIV_21 = 0b10101,
            /// LCD_B LCD frequency divider: /23
            DIV_22 = 0b10110,
            /// LCD_B LCD frequency divider: /24
            DIV_23 = 0b10111,
            /// LCD_B LCD frequency divider: /25
            DIV_24 = 0b11000,
            /// LCD_B LCD frequency divider: /26
            DIV_25 = 0b11001,
            /// LCD_B LCD frequency divider: /27
            DIV_26 = 0b11010,
            /// LCD_B LCD frequency divider: /28
            DIV_27 = 0b11011,
            /// LCD_B LCD frequency divider: /29
            DIV_28 = 0b11100,
            /// LCD_B LCD frequency divider: /30
            DIV_29 = 0b11101,
            /// LCD_B LCD frequency divider: /31
            DIV_30 = 0b11110,
            /// LCD_B LCD frequency divider: /32
            DIV_31 = 0b11111,
        }
    }
    /// LCD_B Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// LCD_B LCD frame interrupt flag
        FRMIFG: 0 = struct FRMIFG(bool);
        /// LCD_B LCD blinking off interrupt flag
        BLKOFFIFG: 1 = struct BLKOFFIFG(bool);
        /// LCD_B LCD blinking on interrupt flag
        BLKONIFG: 2 = struct BLKONIFG(bool);
        /// LCD_B No cpacitance connected interrupt flag
        NOCAPIFG: 3 = struct NOCAPIFG(bool);
        /// LCD_B LCD frame interrupt enable
        FRMIE: 8 = struct FRMIE(bool);
        /// LCD_B LCD blinking off interrupt flag
        LKOFFIE: 9 = struct LKOFFIE(bool);
        /// LCD_B LCD blinking on interrupt flag
        LKONIE: 10 = struct LKONIE(bool);
        /// LCD_B No cpacitance connected interrupt enable
        NOCAPIE: 11 = struct NOCAPIE(bool);
    }
    /// LCD_B blinking control register
    rw BLKCTL @ 0x04: u16 = 0_0 {
        /// LCD_B Blinking mode Bit: 0
        LKMOD: 0..1 = enum LKMOD {
            /// LCD_B Blinking mode: Off
            LKMOD_0 = 0b00,
            /// LCD_B Blinking mode: Individual
            LKMOD_1 = 0b01,
            /// LCD_B Blinking mode: All
            LKMOD_2 = 0b10,
            /// LCD_B Blinking mode: Switching
            LKMOD_3 = 0b11,
        }
        /// LCD_B Clock pre-scaler for blinking frequency Bit: 0
        BLKPRE: 2..4 = enum BLKPRE {
            /// LCD_B Clock pre-scaler for blinking frequency: 0
            BLKPRE_0 = 0b000,
            /// LCD_B Clock pre-scaler for blinking frequency: 1
            BLKPRE_1 = 0b001,
            /// LCD_B Clock pre-scaler for blinking frequency: 2
            BLKPRE_2 = 0b010,
            /// LCD_B Clock pre-scaler for blinking frequency: 3
            BLKPRE_3 = 0b011,
            /// LCD_B Clock pre-scaler for blinking frequency: 4
            BLKPRE_4 = 0b100,
            /// LCD_B Clock pre-scaler for blinking frequency: 5
            BLKPRE_5 = 0b101,
            /// LCD_B Clock pre-scaler for blinking frequency: 6
            BLKPRE_6 = 0b110,
            /// LCD_B Clock pre-scaler for blinking frequency: 7
            BLKPRE_7 = 0b111,
        }
        /// LCD_B Clock divider for blinking frequency Bit: 0
        BLKDIV: 5..7 = enum LCDBLKDIV {
            /// LCD_B Clock divider for blinking frequency: 0
            BLKDIV_0 = 0b000,
            /// LCD_B Clock divider for blinking frequency: 1
            BLKDIV_1 = 0b001,
            /// LCD_B Clock divider for blinking frequency: 2
            BLKDIV_2 = 0b010,
            /// LCD_B Clock divider for blinking frequency: 3
            BLKDIV_3 = 0b011,
            /// LCD_B Clock divider for blinking frequency: 4
            BLKDIV_4 = 0b100,
            /// LCD_B Clock divider for blinking frequency: 5
            BLKDIV_5 = 0b101,
            /// LCD_B Clock divider for blinking frequency: 6
            BLKDIV_6 = 0b110,
            /// LCD_B Clock divider for blinking frequency: 7
            BLKDIV_7 = 0b111,
        }
    }
    /// LCD_B memory control register
    rw MEMCTL @ 0x06: u16 = 0_0 {
        /// LCD_B LCD memory registers for display
        DISP: 0 = struct LCDDISP(bool);
        /// LCD_B Clear LCD memory
        CLRM: 1 = struct LCDCLRM(bool);
        /// LCD_B Clear LCD blinking memory
        CLRBM: 2 = struct LCDCLRBM(bool);
    }
    /// LCD_B Voltage Control Register
    rw VCTL @ 0x08: u16 = 0_0 {
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
        CPEN: 3 = struct CPEN(bool);
        /// Select external source for VLCD.
        VLCDEXT: 4 = struct VLCDEXT(bool);
        /// V2 - V4 voltage select.
        EXTBIAS: 5 = struct EXTBIAS(bool);
        /// Selects external connections for LCD mid voltages.
        R03EXT: 6 = struct R03EXT(bool);
        /// Selects external connection for lowest LCD voltage.
        REXT: 7 = struct REXT(bool);
        /// VLCD select: 0
        VLCD: 9..12 = enum VLCD {
            /// Charge pump disabled
            VLCD_0 = 0b0000,
            /// VLCD = 2.60V
            VLCD_1 = 0b0001,
            /// VLCD = 2.66V
            VLCD_2 = 0b0010,
            /// VLCD = 2.72V
            VLCD_3 = 0b0011,
            /// VLCD = 2.78V
            VLCD_4 = 0b0100,
            /// VLCD = 2.84V
            VLCD_5 = 0b0101,
            /// VLCD = 2.90V
            VLCD_6 = 0b0110,
            /// VLCD = 2.96V
            VLCD_7 = 0b0111,
            /// VLCD = 3.02V
            VLCD_8 = 0b1000,
            /// VLCD = 3.08V
            VLCD_9 = 0b1001,
            /// VLCD = 3.14V
            VLCD_10 = 0b1010,
            /// VLCD = 3.20V
            VLCD_11 = 0b1011,
            /// VLCD = 3.26V
            VLCD_12 = 0b1100,
            /// VLCD = 3.32V
            VLCD_13 = 0b1101,
            /// VLCD = 3.38V
            VLCD_14 = 0b1110,
            /// VLCD = 3.44V
            VLCD_15 = 0b1111,
        }
    }
    /// LCD_B Port Control Register 0
    rw PCTL0 @ 0x0a: u16 = 0_0 {
        /// LCD Segment  0 enable.
        S0: 0 = struct S0(bool);
        /// LCD Segment  1 enable.
        S1: 1 = struct S1(bool);
        /// LCD Segment  2 enable.
        S2: 2 = struct S2(bool);
        /// LCD Segment  3 enable.
        S3: 3 = struct S3(bool);
        /// LCD Segment  4 enable.
        S4: 4 = struct S4(bool);
        /// LCD Segment  5 enable.
        S5: 5 = struct S5(bool);
        /// LCD Segment  6 enable.
        S6: 6 = struct S6(bool);
        /// LCD Segment  7 enable.
        S7: 7 = struct S7(bool);
        /// LCD Segment  8 enable.
        S8: 8 = struct S8(bool);
        /// LCD Segment  9 enable.
        S9: 9 = struct S9(bool);
        /// LCD Segment 10 enable.
        S10: 10 = struct S10(bool);
        /// LCD Segment 11 enable.
        S11: 11 = struct S11(bool);
        /// LCD Segment 12 enable.
        S12: 12 = struct S12(bool);
        /// LCD Segment 13 enable.
        S13: 13 = struct S13(bool);
        /// LCD Segment 14 enable.
        S14: 14 = struct S14(bool);
        /// LCD Segment 15 enable.
        S15: 15 = struct S15(bool);
    }
    /// LCD_B Port Control Register 1
    rw PCTL1 @ 0x0c: u16 = 0_0 {
        /// LCD Segment 16 enable.
        S16: 0 = struct S16(bool);
        /// LCD Segment 17 enable.
        S17: 1 = struct S17(bool);
        /// LCD Segment 18 enable.
        S18: 2 = struct S18(bool);
        /// LCD Segment 19 enable.
        S19: 3 = struct S19(bool);
        /// LCD Segment 20 enable.
        S20: 4 = struct S20(bool);
        /// LCD Segment 21 enable.
        S21: 5 = struct S21(bool);
        /// LCD Segment 22 enable.
        S22: 6 = struct S22(bool);
        /// LCD Segment 23 enable.
        S23: 7 = struct S23(bool);
        /// LCD Segment 24 enable.
        S24: 8 = struct S24(bool);
        /// LCD Segment 25 enable.
        S25: 9 = struct S25(bool);
        /// LCD Segment 26 enable.
        S26: 10 = struct S26(bool);
        /// LCD Segment 27 enable.
        S27: 11 = struct S27(bool);
        /// LCD Segment 28 enable.
        S28: 12 = struct S28(bool);
        /// LCD Segment 29 enable.
        S29: 13 = struct S29(bool);
        /// LCD Segment 30 enable.
        S30: 14 = struct S30(bool);
        /// LCD Segment 31 enable.
        S31: 15 = struct S31(bool);
    }
    /// LCD_B Port Control Register 2
    rw PCTL2 @ 0x0e: u16 = 0_0 {
        /// LCD Segment 32 enable.
        S32: 0 = struct S32(bool);
        /// LCD Segment 33 enable.
        S33: 1 = struct S33(bool);
        /// LCD Segment 34 enable.
        S34: 2 = struct S34(bool);
        /// LCD Segment 35 enable.
        S35: 3 = struct S35(bool);
        /// LCD Segment 36 enable.
        S36: 4 = struct S36(bool);
        /// LCD Segment 37 enable.
        S37: 5 = struct S37(bool);
        /// LCD Segment 38 enable.
        S38: 6 = struct S38(bool);
        /// LCD Segment 39 enable.
        S39: 7 = struct S39(bool);
        /// LCD Segment 40 enable.
        S40: 8 = struct S40(bool);
        /// LCD Segment 41 enable.
        S41: 9 = struct S41(bool);
        /// LCD Segment 42 enable.
        S42: 10 = struct S42(bool);
        /// LCD Segment 43 enable.
        S43: 11 = struct S43(bool);
        /// LCD Segment 44 enable.
        S44: 12 = struct S44(bool);
        /// LCD Segment 45 enable.
        S45: 13 = struct S45(bool);
        /// LCD Segment 46 enable.
        S46: 14 = struct S46(bool);
        /// LCD Segment 47 enable.
        S47: 15 = struct S47(bool);
    }
    /// LCD_B Port Control Register 3
    rw PCTL3 @ 0x10: u16 = 0_0 {
        /// LCD Segment 48 enable.
        S48: 0 = struct S48(bool);
        /// LCD Segment 49 enable.
        S49: 1 = struct S49(bool);
        /// LCD Segment 50 enable.
        S50: 2 = struct S50(bool);
    }
    /// LCD_B Charge Pump Control Register 3
    rw CPCTL @ 0x12: u16 = 0_0 {
        /// LCD charge pump disable
        CPDIS0: 0 = struct CPDIS0(bool);
        /// LCD charge pump disable
        CPDIS1: 1 = struct CPDIS1(bool);
        /// LCD charge pump disable
        CPDIS2: 2 = struct CPDIS2(bool);
        /// LCD charge pump disable
        CPDIS3: 3 = struct CPDIS3(bool);
        /// LCD charge pump disable
        CPDIS4: 4 = struct CPDIS4(bool);
        /// LCD charge pump disable
        CPDIS5: 5 = struct CPDIS5(bool);
        /// LCD charge pump disable
        CPDIS6: 6 = struct CPDIS6(bool);
        /// LCD charge pump disable
        CPDIS7: 7 = struct CPDIS7(bool);
        /// LCD charge pump clock synchronization
        CPCLKSYNC: 15 = struct CPCLKSYNC(bool);
    }
    /// LCD_B Interrupt Vector Register
    rw IV @ 0x1e: u16 = 0_0 {
        /// LCD_B Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
    /// LCD Memory 1
    rw M1 @ 0x20: u8 = 0_0 {
        /// LCD Memory 1
        M1: 0..7 = struct M1Field(u8);
    }
    /// LCD Memory 2
    rw M2 @ 0x21: u8 = 0_0 {
        /// LCD Memory 2
        M2: 0..7 = struct M2Field(u8);
    }
    /// LCD Memory 3
    rw M3 @ 0x22: u8 = 0_0 {
        /// LCD Memory 3
        M3: 0..7 = struct M3Field(u8);
    }
    /// LCD Memory 4
    rw M4 @ 0x23: u8 = 0_0 {
        /// LCD Memory 4
        M4: 0..7 = struct M4Field(u8);
    }
    /// LCD Memory 5
    rw M5 @ 0x24: u8 = 0_0 {
        /// LCD Memory 5
        M5: 0..7 = struct M5Field(u8);
    }
    /// LCD Memory 6
    rw M6 @ 0x25: u8 = 0_0 {
        /// LCD Memory 6
        M6: 0..7 = struct M6Field(u8);
    }
    /// LCD Memory 7
    rw M7 @ 0x26: u8 = 0_0 {
        /// LCD Memory 7
        M7: 0..7 = struct M7Field(u8);
    }
    /// LCD Memory 8
    rw M8 @ 0x27: u8 = 0_0 {
        /// LCD Memory 8
        M8: 0..7 = struct M8Field(u8);
    }
    /// LCD Memory 9
    rw M9 @ 0x28: u8 = 0_0 {
        /// LCD Memory 9
        M9: 0..7 = struct M9Field(u8);
    }
    /// LCD Memory 10
    rw M10 @ 0x29: u8 = 0_0 {
        /// LCD Memory 10
        M10: 0..7 = struct M10Field(u8);
    }
    /// LCD Memory 11
    rw M11 @ 0x2a: u8 = 0_0 {
        /// LCD Memory 11
        M11: 0..7 = struct M11Field(u8);
    }
    /// LCD Memory 12
    rw M12 @ 0x2b: u8 = 0_0 {
        /// LCD Memory 12
        M12: 0..7 = struct M12Field(u8);
    }
    /// LCD Memory 13
    rw M13 @ 0x2c: u8 = 0_0 {
        /// LCD Memory 13
        M13: 0..7 = struct M13Field(u8);
    }
    /// LCD Memory 14
    rw M14 @ 0x2d: u8 = 0_0 {
        /// LCD Memory 14
        M14: 0..7 = struct M14Field(u8);
    }
    /// LCD Memory 15
    rw M15 @ 0x2e: u8 = 0_0 {
        /// LCD Memory 15
        M15: 0..7 = struct M15Field(u8);
    }
    /// LCD Memory 16
    rw M16 @ 0x2f: u8 = 0_0 {
        /// LCD Memory 16
        M16: 0..7 = struct M16Field(u8);
    }
    /// LCD Memory 17
    rw M17 @ 0x30: u8 = 0_0 {
        /// LCD Memory 17
        M17: 0..7 = struct M17Field(u8);
    }
    /// LCD Memory 18
    rw M18 @ 0x31: u8 = 0_0 {
        /// LCD Memory 18
        M18: 0..7 = struct M18Field(u8);
    }
    /// LCD Memory 19
    rw M19 @ 0x32: u8 = 0_0 {
        /// LCD Memory 19
        M19: 0..7 = struct M19Field(u8);
    }
    /// LCD Memory 20
    rw M20 @ 0x33: u8 = 0_0 {
        /// LCD Memory 20
        M20: 0..7 = struct M20Field(u8);
    }
    /// LCD Memory 21
    rw M21 @ 0x34: u8 = 0_0 {
        /// LCD Memory 21
        M21: 0..7 = struct M21Field(u8);
    }
    /// LCD Memory 22
    rw M22 @ 0x35: u8 = 0_0 {
        /// LCD Memory 22
        M22: 0..7 = struct M22Field(u8);
    }
    /// LCD Memory 23
    rw M23 @ 0x36: u8 = 0_0 {
        /// LCD Memory 23
        M23: 0..7 = struct M23Field(u8);
    }
    /// LCD Memory 24
    rw M24 @ 0x37: u8 = 0_0 {
        /// LCD Memory 24
        M24: 0..7 = struct M24Field(u8);
    }
    /// LCD Blinking Memory 1
    rw BM1 @ 0x40: u8 = 0_0 {
        /// LCD Blinking Memory 1
        BM1: 0..7 = struct BM1Field(u8);
    }
    /// LCD Blinking Memory 2
    rw BM2 @ 0x41: u8 = 0_0 {
        /// LCD Blinking Memory 2
        BM2: 0..7 = struct BM2Field(u8);
    }
    /// LCD Blinking Memory 3
    rw BM3 @ 0x42: u8 = 0_0 {
        /// LCD Blinking Memory 3
        BM3: 0..7 = struct BM3Field(u8);
    }
    /// LCD Blinking Memory 4
    rw BM4 @ 0x43: u8 = 0_0 {
        /// LCD Blinking Memory 4
        BM4: 0..7 = struct BM4Field(u8);
    }
    /// LCD Blinking Memory 5
    rw BM5 @ 0x44: u8 = 0_0 {
        /// LCD Blinking Memory 5
        BM5: 0..7 = struct BM5Field(u8);
    }
    /// LCD Blinking Memory 6
    rw BM6 @ 0x45: u8 = 0_0 {
        /// LCD Blinking Memory 6
        BM6: 0..7 = struct BM6Field(u8);
    }
    /// LCD Blinking Memory 7
    rw BM7 @ 0x46: u8 = 0_0 {
        /// LCD Blinking Memory 7
        BM7: 0..7 = struct BM7Field(u8);
    }
    /// LCD Blinking Memory 8
    rw BM8 @ 0x47: u8 = 0_0 {
        /// LCD Blinking Memory 8
        BM8: 0..7 = struct BM8Field(u8);
    }
    /// LCD Blinking Memory 9
    rw BM9 @ 0x48: u8 = 0_0 {
        /// LCD Blinking Memory 9
        BM9: 0..7 = struct BM9Field(u8);
    }
    /// LCD Blinking Memory 10
    rw BM10 @ 0x49: u8 = 0_0 {
        /// LCD Blinking Memory 10
        BM10: 0..7 = struct BM10Field(u8);
    }
    /// LCD Blinking Memory 11
    rw BM11 @ 0x4a: u8 = 0_0 {
        /// LCD Blinking Memory 11
        BM11: 0..7 = struct BM11Field(u8);
    }
    /// LCD Blinking Memory 12
    rw BM12 @ 0x4b: u8 = 0_0 {
        /// LCD Blinking Memory 12
        BM12: 0..7 = struct BM12Field(u8);
    }
    /// LCD Blinking Memory 13
    rw BM13 @ 0x4c: u8 = 0_0 {
        /// LCD Blinking Memory 13
        BM13: 0..7 = struct BM13Field(u8);
    }
    /// LCD Blinking Memory 14
    rw BM14 @ 0x4d: u8 = 0_0 {
        /// LCD Blinking Memory 14
        BM14: 0..7 = struct BM14Field(u8);
    }
    /// LCD Blinking Memory 15
    rw BM15 @ 0x4e: u8 = 0_0 {
        /// LCD Blinking Memory 15
        BM15: 0..7 = struct BM15Field(u8);
    }
    /// LCD Blinking Memory 16
    rw BM16 @ 0x4f: u8 = 0_0 {
        /// LCD Blinking Memory 16
        BM16: 0..7 = struct BM16Field(u8);
    }
    /// LCD Blinking Memory 17
    rw BM17 @ 0x50: u8 = 0_0 {
        /// LCD Blinking Memory 17
        BM17: 0..7 = struct BM17Field(u8);
    }
    /// LCD Blinking Memory 18
    rw BM18 @ 0x51: u8 = 0_0 {
        /// LCD Blinking Memory 18
        BM18: 0..7 = struct BM18Field(u8);
    }
    /// LCD Blinking Memory 19
    rw BM19 @ 0x52: u8 = 0_0 {
        /// LCD Blinking Memory 19
        BM19: 0..7 = struct BM19Field(u8);
    }
    /// LCD Blinking Memory 20
    rw BM20 @ 0x53: u8 = 0_0 {
        /// LCD Blinking Memory 20
        BM20: 0..7 = struct BM20Field(u8);
    }
    /// LCD Blinking Memory 21
    rw BM21 @ 0x54: u8 = 0_0 {
        /// LCD Blinking Memory 21
        BM21: 0..7 = struct BM21Field(u8);
    }
    /// LCD Blinking Memory 22
    rw BM22 @ 0x55: u8 = 0_0 {
        /// LCD Blinking Memory 22
        BM22: 0..7 = struct BM22Field(u8);
    }
    /// LCD Blinking Memory 23
    rw BM23 @ 0x56: u8 = 0_0 {
        /// LCD Blinking Memory 23
        BM23: 0..7 = struct BM23Field(u8);
    }
    /// LCD Blinking Memory 24
    rw BM24 @ 0x57: u8 = 0_0 {
        /// LCD Blinking Memory 24
        BM24: 0..7 = struct BM24Field(u8);
    }
}
