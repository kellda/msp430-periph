//! LCD_E

utils::periph! {
    /// LCD_E
    LCD_E;
    /// LCD_E Control Register 0
    rw LCDCTL0 @ 0x00: u16 = 0_0 {
        /// LCD_E LCD On
        LCDON: 0 = struct LCDON(bool);
        /// LCD_E Low Power Waveform
        LCDLP: 1 = struct LCDLP(bool);
        /// LCD_E LCD Segments On
        LCDSON: 2 = struct LCDSON(bool);
        /// LCD_E Mux Rate Bit: 0
        LCDMX0: 3 = struct LCDMX0(bool);
        /// LCD_E Mux Rate Bit: 1
        LCDMX1: 4 = struct LCDMX1(bool);
        /// LCD_E Mux Rate Bit: 2
        LCDMX2: 5 = struct LCDMX2(bool);
        /// LCD_E Clock Select Bit: 0
        LCDSSEL: 6..7 = enum LCDSSEL {
            /// LCD_E Clock Select: 0
            LCDSSEL_0 = 0b00,
            /// LCD_E Clock Select: 1
            LCDSSEL_1 = 0b01,
            /// LCD_E Clock Select: 2
            LCDSSEL_2 = 0b10,
            /// LCD_E Clock Select: 3
            LCDSSEL_3 = 0b11,
        }
        /// LCD_E LCD frequency divider Bit: 0
        LCDDIV: 11..15 = enum LCDDIV {
            /// LCD_E LCD frequency divider: /1
            LCDDIV_0 = 0b00000,
            /// LCD_E LCD frequency divider: /2
            LCDDIV_1 = 0b00001,
            /// LCD_E LCD frequency divider: /3
            LCDDIV_2 = 0b00010,
            /// LCD_E LCD frequency divider: /4
            LCDDIV_3 = 0b00011,
            /// LCD_E LCD frequency divider: /5
            LCDDIV_4 = 0b00100,
            /// LCD_E LCD frequency divider: /6
            LCDDIV_5 = 0b00101,
            /// LCD_E LCD frequency divider: /7
            LCDDIV_6 = 0b00110,
            /// LCD_E LCD frequency divider: /8
            LCDDIV_7 = 0b00111,
            /// LCD_E LCD frequency divider: /9
            LCDDIV_8 = 0b01000,
            /// LCD_E LCD frequency divider: /10
            LCDDIV_9 = 0b01001,
            /// LCD_E LCD frequency divider: /11
            LCDDIV_10 = 0b01010,
            /// LCD_E LCD frequency divider: /12
            LCDDIV_11 = 0b01011,
            /// LCD_E LCD frequency divider: /13
            LCDDIV_12 = 0b01100,
            /// LCD_E LCD frequency divider: /14
            LCDDIV_13 = 0b01101,
            /// LCD_E LCD frequency divider: /15
            LCDDIV_14 = 0b01110,
            /// LCD_E LCD frequency divider: /16
            LCDDIV_15 = 0b01111,
            /// LCD_E LCD frequency divider: /17
            LCDDIV_16 = 0b10000,
            /// LCD_E LCD frequency divider: /18
            LCDDIV_17 = 0b10001,
            /// LCD_E LCD frequency divider: /19
            LCDDIV_18 = 0b10010,
            /// LCD_E LCD frequency divider: /20
            LCDDIV_19 = 0b10011,
            /// LCD_E LCD frequency divider: /21
            LCDDIV_20 = 0b10100,
            /// LCD_E LCD frequency divider: /22
            LCDDIV_21 = 0b10101,
            /// LCD_E LCD frequency divider: /23
            LCDDIV_22 = 0b10110,
            /// LCD_E LCD frequency divider: /24
            LCDDIV_23 = 0b10111,
            /// LCD_E LCD frequency divider: /25
            LCDDIV_24 = 0b11000,
            /// LCD_E LCD frequency divider: /26
            LCDDIV_25 = 0b11001,
            /// LCD_E LCD frequency divider: /27
            LCDDIV_26 = 0b11010,
            /// LCD_E LCD frequency divider: /28
            LCDDIV_27 = 0b11011,
            /// LCD_E LCD frequency divider: /29
            LCDDIV_28 = 0b11100,
            /// LCD_E LCD frequency divider: /30
            LCDDIV_29 = 0b11101,
            /// LCD_E LCD frequency divider: /31
            LCDDIV_30 = 0b11110,
            /// LCD_E LCD frequency divider: /32
            LCDDIV_31 = 0b11111,
        }
    }
    /// LCD_E Control Register 1
    rw LCDCTL1 @ 0x02: u16 = 0_0 {
        /// LCD_E LCD frame interrupt flag
        LCDFRMIFG: 0 = struct LCDFRMIFG(bool);
        /// LCD_E LCD blinking off interrupt flag
        LCDBLKOFFIFG: 1 = struct LCDBLKOFFIFG(bool);
        /// LCD_E LCD blinking on interrupt flag
        LCDBLKONIFG: 2 = struct LCDBLKONIFG(bool);
        /// LCD_E LCD frame interrupt enable
        LCDFRMIE: 8 = struct LCDFRMIE(bool);
        /// LCD_E LCD blinking off interrupt flag
        LCDBLKOFFIE: 9 = struct LCDBLKOFFIE(bool);
        /// LCD_E LCD blinking on interrupt flag
        LCDBLKONIE: 10 = struct LCDBLKONIE(bool);
    }
    /// LCD_E blinking control register
    rw LCDBLKCTL @ 0x04: u16 = 0_0 {
        /// LCD_E Blinking mode Bit: 0
        LCDBLKMOD: 0..1 = enum LCDBLKMOD {
            /// LCD_E Blinking mode: Off
            LCDBLKMOD_0 = 0b00,
            /// LCD_E Blinking mode: Individual
            LCDBLKMOD_1 = 0b01,
            /// LCD_E Blinking mode: All
            LCDBLKMOD_2 = 0b10,
            /// LCD_E Blinking mode: Switching
            LCDBLKMOD_3 = 0b11,
        }
        /// LCD_E Clock pre-scaler for blinking frequency Bit: 0
        LCDBLKPRE: 2..4 = enum LCDBLKPRE {
            /// LCD_E Clock pre-scaler for blinking frequency: 0
            LCDBLKPRE_0 = 0b000,
            /// LCD_E Clock pre-scaler for blinking frequency: 1
            LCDBLKPRE_1 = 0b001,
            /// LCD_E Clock pre-scaler for blinking frequency: 2
            LCDBLKPRE_2 = 0b010,
            /// LCD_E Clock pre-scaler for blinking frequency: 3
            LCDBLKPRE_3 = 0b011,
            /// LCD_E Clock pre-scaler for blinking frequency: 4
            LCDBLKPRE_4 = 0b100,
            /// LCD_E Clock pre-scaler for blinking frequency: 5
            LCDBLKPRE_5 = 0b101,
            /// LCD_E Clock pre-scaler for blinking frequency: 6
            LCDBLKPRE_6 = 0b110,
            /// LCD_E Clock pre-scaler for blinking frequency: 7
            LCDBLKPRE_7 = 0b111,
        }
    }
    /// LCD_E memory control register
    rw LCDMEMCTL @ 0x06: u16 = 0_0 {
        /// LCD_E LCD memory registers for display
        LCDDISP: 0 = struct LCDDISP(bool);
        /// LCD_E Clear LCD memory
        LCDCLRM: 1 = struct LCDCLRM(bool);
        /// LCD_E Clear LCD blinking memory
        LCDCLRBM: 2 = struct LCDCLRBM(bool);
    }
    /// LCD_E Voltage Control Register
    rw LCDVCTL @ 0x08: u16 = 0_0 {
        /// Selects wether R13 voltage is switched or in static mode
        LCDREFMODE: 0 = struct LCDREFMODE(bool);
        /// selects if R33 is supplied either from Vcc internally or from charge pump
        LCDSELVDD: 5 = struct LCDSELVDD(bool);
        /// Internal reference voltage enable on R13
        LCDREFEN: 6 = struct LCDREFEN(bool);
        /// Charge pump enable
        LCDCPEN: 7 = struct LCDCPEN(bool);
        /// VLCD select: 0
        VLCD: 8..11 = enum VLCD {
            /// VLCD = 2.60V
            VLCD_0 = 0b0000,
            /// VLCD = 2.66V
            VLCD_1 = 0b0001,
            /// VLCD = 2.72V
            VLCD_2 = 0b0010,
            /// VLCD = 2.78V
            VLCD_3 = 0b0011,
            /// VLCD = 2.84V
            VLCD_4 = 0b0100,
            /// VLCD = 2.90V
            VLCD_5 = 0b0101,
            /// VLCD = 2.96V
            VLCD_6 = 0b0110,
            /// VLCD = 3.02V
            VLCD_7 = 0b0111,
            /// VLCD = 3.08V
            VLCD_8 = 0b1000,
            /// VLCD = 3.14V
            VLCD_9 = 0b1001,
            /// VLCD = 3.20V
            VLCD_10 = 0b1010,
            /// VLCD = 3.26V
            VLCD_11 = 0b1011,
            /// VLCD = 3.32V
            VLCD_12 = 0b1100,
            /// VLCD = 3.38V
            VLCD_13 = 0b1101,
            /// VLCD = 3.44V
            VLCD_14 = 0b1110,
            /// VLCD = 3.50V
            VLCD_15 = 0b1111,
        }
        /// Charge pump frequency selection Bit: 0
        LCDCPFSEL0: 12 = struct LCDCPFSEL0(bool);
        /// Charge pump frequency selection Bit: 1
        LCDCPFSEL1: 13 = struct LCDCPFSEL1(bool);
        /// Charge pump frequency selection Bit: 2
        LCDCPFSEL2: 14 = struct LCDCPFSEL2(bool);
        /// Charge pump frequency selection Bit: 3
        LCDCPFSEL3: 15 = struct LCDCPFSEL3(bool);
    }
    /// LCD_E Port Control Register 0
    rw LCDPCTL0 @ 0x0a: u16 = 0_0 {
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
    /// LCD_E Port Control Register 1
    rw LCDPCTL1 @ 0x0c: u16 = 0_0 {
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
    /// LCD_E Port Control Register 2
    rw LCDPCTL2 @ 0x0e: u16 = 0_0 {
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
    /// LCD_E COM/SEG select register 0
    rw LCDCSSEL0 @ 0x14: u16 = 0_0 {
        /// Selects pin L0  as either common or segment line
        LCDCSS0: 0 = struct LCDCSS0(bool);
        /// Selects pin L1  as either common or segment line
        LCDCSS1: 1 = struct LCDCSS1(bool);
        /// Selects pin L2  as either common or segment line
        LCDCSS2: 2 = struct LCDCSS2(bool);
        /// Selects pin L3  as either common or segment line
        LCDCSS3: 3 = struct LCDCSS3(bool);
        /// Selects pin L4  as either common or segment line
        LCDCSS4: 4 = struct LCDCSS4(bool);
        /// Selects pin L5  as either common or segment line
        LCDCSS5: 5 = struct LCDCSS5(bool);
        /// Selects pin L6  as either common or segment line
        LCDCSS6: 6 = struct LCDCSS6(bool);
        /// Selects pin L7  as either common or segment line
        LCDCSS7: 7 = struct LCDCSS7(bool);
        /// Selects pin L8  as either common or segment line
        LCDCSS8: 8 = struct LCDCSS8(bool);
        /// Selects pin L9  as either common or segment line
        LCDCSS9: 9 = struct LCDCSS9(bool);
        /// Selects pin L10 as either common or segment line
        LCDCSS10: 10 = struct LCDCSS10(bool);
        /// Selects pin L11 as either common or segment line
        LCDCSS11: 11 = struct LCDCSS11(bool);
        /// Selects pin L12 as either common or segment line
        LCDCSS12: 12 = struct LCDCSS12(bool);
        /// Selects pin L13 as either common or segment line
        LCDCSS13: 13 = struct LCDCSS13(bool);
        /// Selects pin L14 as either common or segment line
        LCDCSS14: 14 = struct LCDCSS14(bool);
        /// Selects pin L15 as either common or segment line
        LCDCSS15: 15 = struct LCDCSS15(bool);
    }
    /// LCD_E COM/SEG select register 1
    rw LCDCSSEL1 @ 0x16: u16 = 0_0 {
        /// Selects pin L16 as either common or segment line
        LCDCSS16: 0 = struct LCDCSS16(bool);
        /// Selects pin L17 as either common or segment line
        LCDCSS17: 1 = struct LCDCSS17(bool);
        /// Selects pin L18 as either common or segment line
        LCDCSS18: 2 = struct LCDCSS18(bool);
        /// Selects pin L19 as either common or segment line
        LCDCSS19: 3 = struct LCDCSS19(bool);
        /// Selects pin L20 as either common or segment line
        LCDCSS20: 4 = struct LCDCSS20(bool);
        /// Selects pin L21 as either common or segment line
        LCDCSS21: 5 = struct LCDCSS21(bool);
        /// Selects pin L22 as either common or segment line
        LCDCSS22: 6 = struct LCDCSS22(bool);
        /// Selects pin L23 as either common or segment line
        LCDCSS23: 7 = struct LCDCSS23(bool);
        /// Selects pin L24 as either common or segment line
        LCDCSS24: 8 = struct LCDCSS24(bool);
        /// Selects pin L25 as either common or segment line
        LCDCSS25: 9 = struct LCDCSS25(bool);
        /// Selects pin L26 as either common or segment line
        LCDCSS26: 10 = struct LCDCSS26(bool);
        /// Selects pin L27 as either common or segment line
        LCDCSS27: 11 = struct LCDCSS27(bool);
        /// Selects pin L28 as either common or segment line
        LCDCSS28: 12 = struct LCDCSS28(bool);
        /// Selects pin L29 as either common or segment line
        LCDCSS29: 13 = struct LCDCSS29(bool);
        /// Selects pin L30 as either common or segment line
        LCDCSS30: 14 = struct LCDCSS30(bool);
        /// Selects pin L31 as either common or segment line
        LCDCSS31: 15 = struct LCDCSS31(bool);
    }
    /// LCD_E COM/SEG select register 2
    rw LCDCSSEL2 @ 0x18: u16 = 0_0 {
        /// Selects pin L32 as either common or segment line
        LCDCSS32: 0 = struct LCDCSS32(bool);
        /// Selects pin L33 as either common or segment line
        LCDCSS33: 1 = struct LCDCSS33(bool);
        /// Selects pin L34 as either common or segment line
        LCDCSS34: 2 = struct LCDCSS34(bool);
        /// Selects pin L35 as either common or segment line
        LCDCSS35: 3 = struct LCDCSS35(bool);
        /// Selects pin L36 as either common or segment line
        LCDCSS36: 4 = struct LCDCSS36(bool);
        /// Selects pin L37 as either common or segment line
        LCDCSS37: 5 = struct LCDCSS37(bool);
        /// Selects pin L38 as either common or segment line
        LCDCSS38: 6 = struct LCDCSS38(bool);
        /// Selects pin L39 as either common or segment line
        LCDCSS39: 7 = struct LCDCSS39(bool);
        /// Selects pin L40 as either common or segment line
        LCDCSS40: 8 = struct LCDCSS40(bool);
        /// Selects pin L41 as either common or segment line
        LCDCSS41: 9 = struct LCDCSS41(bool);
        /// Selects pin L42 as either common or segment line
        LCDCSS42: 10 = struct LCDCSS42(bool);
        /// Selects pin L43 as either common or segment line
        LCDCSS43: 11 = struct LCDCSS43(bool);
        /// Selects pin L44 as either common or segment line
        LCDCSS44: 12 = struct LCDCSS44(bool);
        /// Selects pin L45 as either common or segment line
        LCDCSS45: 13 = struct LCDCSS45(bool);
        /// Selects pin L46 as either common or segment line
        LCDCSS46: 14 = struct LCDCSS46(bool);
        /// Selects pin L47 as either common or segment line
        LCDCSS47: 15 = struct LCDCSS47(bool);
    }
    /// LCD_E Interrupt Vector Register
    rw LCDIV @ 0x1e: u16 = 0_0 {
        /// LCD_E Interrupt Vector Register
        LCDIV: 0..15 = struct LCDIVField(u16);
    }
    /// LCD Memory 0/1
    rw LCDM0W @ 0x20: u16 = 0_0 {
        /// LCD Memory 0/1
        LCDM0W: 0..15 = struct LCDM0WField(u16);
    }
    /// LCD Memory 2/3
    rw LCDM2W @ 0x22: u16 = 0_0 {
        /// LCD Memory 2/3
        LCDM2W: 0..15 = struct LCDM2WField(u16);
    }
    /// LCD Memory 4/5
    rw LCDM4W @ 0x24: u16 = 0_0 {
        /// LCD Memory 4/5
        LCDM4W: 0..15 = struct LCDM4WField(u16);
    }
    /// LCD Memory 6/7
    rw LCDM6W @ 0x26: u16 = 0_0 {
        /// LCD Memory 6/7
        LCDM6W: 0..15 = struct LCDM6WField(u16);
    }
    /// LCD Memory 8/9
    rw LCDM8W @ 0x28: u16 = 0_0 {
        /// LCD Memory 8/9
        LCDM8W: 0..15 = struct LCDM8WField(u16);
    }
    /// LCD Memory 10/11
    rw LCDM10W @ 0x2a: u16 = 0_0 {
        /// LCD Memory 10/11
        LCDM10W: 0..15 = struct LCDM10WField(u16);
    }
    /// LCD Memory 12/13
    rw LCDM12W @ 0x2c: u16 = 0_0 {
        /// LCD Memory 12/13
        LCDM12W: 0..15 = struct LCDM12WField(u16);
    }
    /// LCD Memory 14/15
    rw LCDM14W @ 0x2e: u16 = 0_0 {
        /// LCD Memory 14/15
        LCDM14W: 0..15 = struct LCDM14WField(u16);
    }
    /// LCD Memory 16/17
    rw LCDM16W @ 0x30: u16 = 0_0 {
        /// LCD Memory 16/17
        LCDM16W: 0..15 = struct LCDM16WField(u16);
    }
    /// LCD Memory 18/19
    rw LCDM18W @ 0x32: u16 = 0_0 {
        /// LCD Memory 18/19
        LCDM18W: 0..15 = struct LCDM18WField(u16);
    }
    /// LCD Memory 20/21
    rw LCDM20W @ 0x34: u16 = 0_0 {
        /// LCD Memory 20/21
        LCDM20W: 0..15 = struct LCDM20WField(u16);
    }
    /// LCD Memory 22/23
    rw LCDM22W @ 0x36: u16 = 0_0 {
        /// LCD Memory 22/23
        LCDM22W: 0..15 = struct LCDM22WField(u16);
    }
    /// LCD Memory 24/25
    rw LCDM24W @ 0x38: u16 = 0_0 {
        /// LCD Memory 24/25
        LCDM24W: 0..15 = struct LCDM24WField(u16);
    }
    /// LCD Memory 26/27
    rw LCDM26W @ 0x3a: u16 = 0_0 {
        /// LCD Memory 26/27
        LCDM26W: 0..15 = struct LCDM26WField(u16);
    }
    /// LCD Memory 28/29
    rw LCDM28W @ 0x3c: u16 = 0_0 {
        /// LCD Memory 28/29
        LCDM28W: 0..15 = struct LCDM28WField(u16);
    }
    /// LCD Memory 30/31
    rw LCDM30W @ 0x3e: u16 = 0_0 {
        /// LCD Memory 30/31
        LCDM30W: 0..15 = struct LCDM30WField(u16);
    }
    /// LCD Memory 32/33
    rw LCDM32W @ 0x40: u16 = 0_0 {
        /// LCD Memory 32/33
        LCDM32W: 0..15 = struct LCDM32WField(u16);
    }
    /// LCD Memory 34/35
    rw LCDM34W @ 0x42: u16 = 0_0 {
        /// LCD Memory 34/35
        LCDM34W: 0..15 = struct LCDM34WField(u16);
    }
    /// LCD Memory 36/37
    rw LCDM36W @ 0x44: u16 = 0_0 {
        /// LCD Memory 36/37
        LCDM36W: 0..15 = struct LCDM36WField(u16);
    }
    /// LCD Memory 38/39
    rw LCDM38W @ 0x46: u16 = 0_0 {
        /// LCD Memory 38/39
        LCDM38W: 0..15 = struct LCDM38WField(u16);
    }
    /// LCD Blinking Memory 0/1
    rw LCDBM0W @ 0x40: u16 = 0_0 {
        /// LCD Blinking Memory 0/1
        LCDBM0W: 0..15 = struct LCDBM0WField(u16);
    }
    /// LCD Blinking Memory 2/3
    rw LCDBM2W @ 0x42: u16 = 0_0 {
        /// LCD Blinking Memory 2/3
        LCDBM2W: 0..15 = struct LCDBM2WField(u16);
    }
    /// LCD Blinking Memory 4/5
    rw LCDBM4W @ 0x44: u16 = 0_0 {
        /// LCD Blinking Memory 4/5
        LCDBM4W: 0..15 = struct LCDBM4WField(u16);
    }
    /// LCD Blinking Memory 6/7
    rw LCDBM6W @ 0x46: u16 = 0_0 {
        /// LCD Blinking Memory 6/7
        LCDBM6W: 0..15 = struct LCDBM6WField(u16);
    }
    /// LCD Blinking Memory 8/9
    rw LCDBM8W @ 0x48: u16 = 0_0 {
        /// LCD Blinking Memory 8/9
        LCDBM8W: 0..15 = struct LCDBM8WField(u16);
    }
    /// LCD Blinking Memory 10/11
    rw LCDBM10W @ 0x4a: u16 = 0_0 {
        /// LCD Blinking Memory 10/11
        LCDBM10W: 0..15 = struct LCDBM10WField(u16);
    }
    /// LCD Blinking Memory 12/13
    rw LCDBM12W @ 0x4c: u16 = 0_0 {
        /// LCD Blinking Memory 12/13
        LCDBM12W: 0..15 = struct LCDBM12WField(u16);
    }
    /// LCD Blinking Memory 14/15
    rw LCDBM14W @ 0x4e: u16 = 0_0 {
        /// LCD Blinking Memory 14/15
        LCDBM14W: 0..15 = struct LCDBM14WField(u16);
    }
    /// LCD Blinking Memory 16/17
    rw LCDBM16W @ 0x50: u16 = 0_0 {
        /// LCD Blinking Memory 16/17
        LCDBM16W: 0..15 = struct LCDBM16WField(u16);
    }
    /// LCD Blinking Memory 18/19
    rw LCDBM18W @ 0x52: u16 = 0_0 {
        /// LCD Blinking Memory 18/19
        LCDBM18W: 0..15 = struct LCDBM18WField(u16);
    }
}
