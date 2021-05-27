//! LCD_E

utils::periph! {
    /// LCD_E
    LCD_E;
    /// LCD_E Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// LCD_E LCD On
        ON: 0 = struct ON(bool);
        /// LCD_E Low Power Waveform
        LP: 1 = struct LP(bool);
        /// LCD_E LCD Segments On
        SON: 2 = struct SON(bool);
        /// LCD_E Mux Rate Bit: 0
        MX0: 3 = struct MX0(bool);
        /// LCD_E Mux Rate Bit: 1
        MX1: 4 = struct MX1(bool);
        /// LCD_E Mux Rate Bit: 2
        MX2: 5 = struct MX2(bool);
        /// LCD_E Clock Select Bit: 0
        SSEL: 6..7 = enum SSEL {
            /// LCD_E Clock Select: 0
            SSEL_0 = 0b00,
            /// LCD_E Clock Select: 1
            SSEL_1 = 0b01,
            /// LCD_E Clock Select: 2
            SSEL_2 = 0b10,
            /// LCD_E Clock Select: 3
            SSEL_3 = 0b11,
        }
        /// LCD_E LCD frequency divider Bit: 0
        DIV: 11..15 = enum DIV {
            /// LCD_E LCD frequency divider: /1
            DIV_0 = 0b00000,
            /// LCD_E LCD frequency divider: /2
            DIV_1 = 0b00001,
            /// LCD_E LCD frequency divider: /3
            DIV_2 = 0b00010,
            /// LCD_E LCD frequency divider: /4
            DIV_3 = 0b00011,
            /// LCD_E LCD frequency divider: /5
            DIV_4 = 0b00100,
            /// LCD_E LCD frequency divider: /6
            DIV_5 = 0b00101,
            /// LCD_E LCD frequency divider: /7
            DIV_6 = 0b00110,
            /// LCD_E LCD frequency divider: /8
            DIV_7 = 0b00111,
            /// LCD_E LCD frequency divider: /9
            DIV_8 = 0b01000,
            /// LCD_E LCD frequency divider: /10
            DIV_9 = 0b01001,
            /// LCD_E LCD frequency divider: /11
            DIV_10 = 0b01010,
            /// LCD_E LCD frequency divider: /12
            DIV_11 = 0b01011,
            /// LCD_E LCD frequency divider: /13
            DIV_12 = 0b01100,
            /// LCD_E LCD frequency divider: /14
            DIV_13 = 0b01101,
            /// LCD_E LCD frequency divider: /15
            DIV_14 = 0b01110,
            /// LCD_E LCD frequency divider: /16
            DIV_15 = 0b01111,
            /// LCD_E LCD frequency divider: /17
            DIV_16 = 0b10000,
            /// LCD_E LCD frequency divider: /18
            DIV_17 = 0b10001,
            /// LCD_E LCD frequency divider: /19
            DIV_18 = 0b10010,
            /// LCD_E LCD frequency divider: /20
            DIV_19 = 0b10011,
            /// LCD_E LCD frequency divider: /21
            DIV_20 = 0b10100,
            /// LCD_E LCD frequency divider: /22
            DIV_21 = 0b10101,
            /// LCD_E LCD frequency divider: /23
            DIV_22 = 0b10110,
            /// LCD_E LCD frequency divider: /24
            DIV_23 = 0b10111,
            /// LCD_E LCD frequency divider: /25
            DIV_24 = 0b11000,
            /// LCD_E LCD frequency divider: /26
            DIV_25 = 0b11001,
            /// LCD_E LCD frequency divider: /27
            DIV_26 = 0b11010,
            /// LCD_E LCD frequency divider: /28
            DIV_27 = 0b11011,
            /// LCD_E LCD frequency divider: /29
            DIV_28 = 0b11100,
            /// LCD_E LCD frequency divider: /30
            DIV_29 = 0b11101,
            /// LCD_E LCD frequency divider: /31
            DIV_30 = 0b11110,
            /// LCD_E LCD frequency divider: /32
            DIV_31 = 0b11111,
        }
    }
    /// LCD_E Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// LCD_E LCD frame interrupt flag
        FRMIFG: 0 = struct FRMIFG(bool);
        /// LCD_E LCD blinking off interrupt flag
        BLKOFFIFG: 1 = struct BLKOFFIFG(bool);
        /// LCD_E LCD blinking on interrupt flag
        BLKONIFG: 2 = struct BLKONIFG(bool);
        /// LCD_E LCD frame interrupt enable
        FRMIE: 8 = struct FRMIE(bool);
        /// LCD_E LCD blinking off interrupt flag
        BLKOFFIE: 9 = struct BLKOFFIE(bool);
        /// LCD_E LCD blinking on interrupt flag
        BLKONIE: 10 = struct BLKONIE(bool);
    }
    /// LCD_E blinking control register
    rw BLKCTL @ 0x04: u16 = 0_0 {
        /// LCD_E Blinking mode Bit: 0
        BLKMOD: 0..1 = enum BLKMOD {
            /// LCD_E Blinking mode: Off
            BLKMOD_0 = 0b00,
            /// LCD_E Blinking mode: Individual
            BLKMOD_1 = 0b01,
            /// LCD_E Blinking mode: All
            BLKMOD_2 = 0b10,
            /// LCD_E Blinking mode: Switching
            BLKMOD_3 = 0b11,
        }
        /// LCD_E Clock pre-scaler for blinking frequency Bit: 0
        BLKPRE: 2..4 = enum BLKPRE {
            /// LCD_E Clock pre-scaler for blinking frequency: 0
            BLKPRE_0 = 0b000,
            /// LCD_E Clock pre-scaler for blinking frequency: 1
            BLKPRE_1 = 0b001,
            /// LCD_E Clock pre-scaler for blinking frequency: 2
            BLKPRE_2 = 0b010,
            /// LCD_E Clock pre-scaler for blinking frequency: 3
            BLKPRE_3 = 0b011,
            /// LCD_E Clock pre-scaler for blinking frequency: 4
            BLKPRE_4 = 0b100,
            /// LCD_E Clock pre-scaler for blinking frequency: 5
            BLKPRE_5 = 0b101,
            /// LCD_E Clock pre-scaler for blinking frequency: 6
            BLKPRE_6 = 0b110,
            /// LCD_E Clock pre-scaler for blinking frequency: 7
            BLKPRE_7 = 0b111,
        }
    }
    /// LCD_E memory control register
    rw MEMCTL @ 0x06: u16 = 0_0 {
        /// LCD_E LCD memory registers for display
        DISP: 0 = struct DISP(bool);
        /// LCD_E Clear LCD memory
        CLRM: 1 = struct CLRM(bool);
        /// LCD_E Clear LCD blinking memory
        CLRBM: 2 = struct CLRBM(bool);
    }
    /// LCD_E Voltage Control Register
    rw VCTL @ 0x08: u16 = 0_0 {
        /// Selects wether R13 voltage is switched or in static mode
        REFMODE: 0 = struct REFMODE(bool);
        /// selects if R33 is supplied either from Vcc internally or from charge pump
        SELVDD: 5 = struct SELVDD(bool);
        /// Internal reference voltage enable on R13
        REFEN: 6 = struct REFEN(bool);
        /// Charge pump enable
        CPEN: 7 = struct CPEN(bool);
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
        CPFSEL0: 12 = struct CPFSEL0(bool);
        /// Charge pump frequency selection Bit: 1
        CPFSEL1: 13 = struct CPFSEL1(bool);
        /// Charge pump frequency selection Bit: 2
        CPFSEL2: 14 = struct CPFSEL2(bool);
        /// Charge pump frequency selection Bit: 3
        CPFSEL3: 15 = struct CPFSEL3(bool);
    }
    /// LCD_E Port Control Register 0
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
    /// LCD_E Port Control Register 1
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
    /// LCD_E Port Control Register 2
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
    /// LCD_E COM/SEG select register 0
    rw CSSEL0 @ 0x14: u16 = 0_0 {
        /// Selects pin L0  as either common or segment line
        CSS0: 0 = struct CSS0(bool);
        /// Selects pin L1  as either common or segment line
        CSS1: 1 = struct CSS1(bool);
        /// Selects pin L2  as either common or segment line
        CSS2: 2 = struct CSS2(bool);
        /// Selects pin L3  as either common or segment line
        CSS3: 3 = struct CSS3(bool);
        /// Selects pin L4  as either common or segment line
        CSS4: 4 = struct CSS4(bool);
        /// Selects pin L5  as either common or segment line
        CSS5: 5 = struct CSS5(bool);
        /// Selects pin L6  as either common or segment line
        CSS6: 6 = struct CSS6(bool);
        /// Selects pin L7  as either common or segment line
        CSS7: 7 = struct CSS7(bool);
        /// Selects pin L8  as either common or segment line
        CSS8: 8 = struct CSS8(bool);
        /// Selects pin L9  as either common or segment line
        CSS9: 9 = struct CSS9(bool);
        /// Selects pin L10 as either common or segment line
        CSS10: 10 = struct CSS10(bool);
        /// Selects pin L11 as either common or segment line
        CSS11: 11 = struct CSS11(bool);
        /// Selects pin L12 as either common or segment line
        CSS12: 12 = struct CSS12(bool);
        /// Selects pin L13 as either common or segment line
        CSS13: 13 = struct CSS13(bool);
        /// Selects pin L14 as either common or segment line
        CSS14: 14 = struct CSS14(bool);
        /// Selects pin L15 as either common or segment line
        CSS15: 15 = struct CSS15(bool);
    }
    /// LCD_E COM/SEG select register 1
    rw CSSEL1 @ 0x16: u16 = 0_0 {
        /// Selects pin L16 as either common or segment line
        CSS16: 0 = struct CSS16(bool);
        /// Selects pin L17 as either common or segment line
        CSS17: 1 = struct CSS17(bool);
        /// Selects pin L18 as either common or segment line
        CSS18: 2 = struct CSS18(bool);
        /// Selects pin L19 as either common or segment line
        CSS19: 3 = struct CSS19(bool);
        /// Selects pin L20 as either common or segment line
        CSS20: 4 = struct CSS20(bool);
        /// Selects pin L21 as either common or segment line
        CSS21: 5 = struct CSS21(bool);
        /// Selects pin L22 as either common or segment line
        CSS22: 6 = struct CSS22(bool);
        /// Selects pin L23 as either common or segment line
        CSS23: 7 = struct CSS23(bool);
        /// Selects pin L24 as either common or segment line
        CSS24: 8 = struct CSS24(bool);
        /// Selects pin L25 as either common or segment line
        CSS25: 9 = struct CSS25(bool);
        /// Selects pin L26 as either common or segment line
        CSS26: 10 = struct CSS26(bool);
        /// Selects pin L27 as either common or segment line
        CSS27: 11 = struct CSS27(bool);
        /// Selects pin L28 as either common or segment line
        CSS28: 12 = struct CSS28(bool);
        /// Selects pin L29 as either common or segment line
        CSS29: 13 = struct CSS29(bool);
        /// Selects pin L30 as either common or segment line
        CSS30: 14 = struct CSS30(bool);
        /// Selects pin L31 as either common or segment line
        CSS31: 15 = struct CSS31(bool);
    }
    /// LCD_E COM/SEG select register 2
    rw CSSEL2 @ 0x18: u16 = 0_0 {
        /// Selects pin L32 as either common or segment line
        CSS32: 0 = struct CSS32(bool);
        /// Selects pin L33 as either common or segment line
        CSS33: 1 = struct CSS33(bool);
        /// Selects pin L34 as either common or segment line
        CSS34: 2 = struct CSS34(bool);
        /// Selects pin L35 as either common or segment line
        CSS35: 3 = struct CSS35(bool);
        /// Selects pin L36 as either common or segment line
        CSS36: 4 = struct CSS36(bool);
        /// Selects pin L37 as either common or segment line
        CSS37: 5 = struct CSS37(bool);
        /// Selects pin L38 as either common or segment line
        CSS38: 6 = struct CSS38(bool);
        /// Selects pin L39 as either common or segment line
        CSS39: 7 = struct CSS39(bool);
        /// Selects pin L40 as either common or segment line
        CSS40: 8 = struct CSS40(bool);
        /// Selects pin L41 as either common or segment line
        CSS41: 9 = struct CSS41(bool);
        /// Selects pin L42 as either common or segment line
        CSS42: 10 = struct CSS42(bool);
        /// Selects pin L43 as either common or segment line
        CSS43: 11 = struct CSS43(bool);
        /// Selects pin L44 as either common or segment line
        CSS44: 12 = struct CSS44(bool);
        /// Selects pin L45 as either common or segment line
        CSS45: 13 = struct CSS45(bool);
        /// Selects pin L46 as either common or segment line
        CSS46: 14 = struct CSS46(bool);
        /// Selects pin L47 as either common or segment line
        CSS47: 15 = struct CSS47(bool);
    }
    /// LCD_E Interrupt Vector Register
    rw IV @ 0x1e: u16 = 0_0 {
        /// LCD_E Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
    /// LCD Memory 0/1
    rw M0W @ 0x20: u16 = 0_0 {
        /// LCD Memory 0/1
        M0W: 0..15 = struct M0WField(u16);
    }
    /// LCD Memory 2/3
    rw M2W @ 0x22: u16 = 0_0 {
        /// LCD Memory 2/3
        M2W: 0..15 = struct M2WField(u16);
    }
    /// LCD Memory 4/5
    rw M4W @ 0x24: u16 = 0_0 {
        /// LCD Memory 4/5
        M4W: 0..15 = struct M4WField(u16);
    }
    /// LCD Memory 6/7
    rw M6W @ 0x26: u16 = 0_0 {
        /// LCD Memory 6/7
        M6W: 0..15 = struct M6WField(u16);
    }
    /// LCD Memory 8/9
    rw M8W @ 0x28: u16 = 0_0 {
        /// LCD Memory 8/9
        M8W: 0..15 = struct M8WField(u16);
    }
    /// LCD Memory 10/11
    rw M10W @ 0x2a: u16 = 0_0 {
        /// LCD Memory 10/11
        M10W: 0..15 = struct M10WField(u16);
    }
    /// LCD Memory 12/13
    rw M12W @ 0x2c: u16 = 0_0 {
        /// LCD Memory 12/13
        M12W: 0..15 = struct M12WField(u16);
    }
    /// LCD Memory 14/15
    rw M14W @ 0x2e: u16 = 0_0 {
        /// LCD Memory 14/15
        M14W: 0..15 = struct M14WField(u16);
    }
    /// LCD Memory 16/17
    rw M16W @ 0x30: u16 = 0_0 {
        /// LCD Memory 16/17
        M16W: 0..15 = struct M16WField(u16);
    }
    /// LCD Memory 18/19
    rw M18W @ 0x32: u16 = 0_0 {
        /// LCD Memory 18/19
        M18W: 0..15 = struct M18WField(u16);
    }
    /// LCD Memory 20/21
    rw M20W @ 0x34: u16 = 0_0 {
        /// LCD Memory 20/21
        M20W: 0..15 = struct M20WField(u16);
    }
    /// LCD Memory 22/23
    rw M22W @ 0x36: u16 = 0_0 {
        /// LCD Memory 22/23
        M22W: 0..15 = struct M22WField(u16);
    }
    /// LCD Memory 24/25
    rw M24W @ 0x38: u16 = 0_0 {
        /// LCD Memory 24/25
        M24W: 0..15 = struct M24WField(u16);
    }
    /// LCD Memory 26/27
    rw M26W @ 0x3a: u16 = 0_0 {
        /// LCD Memory 26/27
        M26W: 0..15 = struct M26WField(u16);
    }
    /// LCD Memory 28/29
    rw M28W @ 0x3c: u16 = 0_0 {
        /// LCD Memory 28/29
        M28W: 0..15 = struct M28WField(u16);
    }
    /// LCD Memory 30/31
    rw M30W @ 0x3e: u16 = 0_0 {
        /// LCD Memory 30/31
        M30W: 0..15 = struct M30WField(u16);
    }
    /// LCD Memory 32/33
    rw M32W @ 0x40: u16 = 0_0 {
        /// LCD Memory 32/33
        M32W: 0..15 = struct M32WField(u16);
    }
    /// LCD Memory 34/35
    rw M34W @ 0x42: u16 = 0_0 {
        /// LCD Memory 34/35
        M34W: 0..15 = struct M34WField(u16);
    }
    /// LCD Memory 36/37
    rw M36W @ 0x44: u16 = 0_0 {
        /// LCD Memory 36/37
        M36W: 0..15 = struct M36WField(u16);
    }
    /// LCD Memory 38/39
    rw M38W @ 0x46: u16 = 0_0 {
        /// LCD Memory 38/39
        M38W: 0..15 = struct M38WField(u16);
    }
    /// LCD Blinking Memory 0/1
    rw BM0W @ 0x40: u16 = 0_0 {
        /// LCD Blinking Memory 0/1
        BM0W: 0..15 = struct BM0WField(u16);
    }
    /// LCD Blinking Memory 2/3
    rw BM2W @ 0x42: u16 = 0_0 {
        /// LCD Blinking Memory 2/3
        BM2W: 0..15 = struct BM2WField(u16);
    }
    /// LCD Blinking Memory 4/5
    rw BM4W @ 0x44: u16 = 0_0 {
        /// LCD Blinking Memory 4/5
        BM4W: 0..15 = struct BM4WField(u16);
    }
    /// LCD Blinking Memory 6/7
    rw BM6W @ 0x46: u16 = 0_0 {
        /// LCD Blinking Memory 6/7
        BM6W: 0..15 = struct BM6WField(u16);
    }
    /// LCD Blinking Memory 8/9
    rw BM8W @ 0x48: u16 = 0_0 {
        /// LCD Blinking Memory 8/9
        BM8W: 0..15 = struct BM8WField(u16);
    }
    /// LCD Blinking Memory 10/11
    rw BM10W @ 0x4a: u16 = 0_0 {
        /// LCD Blinking Memory 10/11
        BM10W: 0..15 = struct BM10WField(u16);
    }
    /// LCD Blinking Memory 12/13
    rw BM12W @ 0x4c: u16 = 0_0 {
        /// LCD Blinking Memory 12/13
        BM12W: 0..15 = struct BM12WField(u16);
    }
    /// LCD Blinking Memory 14/15
    rw BM14W @ 0x4e: u16 = 0_0 {
        /// LCD Blinking Memory 14/15
        BM14W: 0..15 = struct BM14WField(u16);
    }
    /// LCD Blinking Memory 16/17
    rw BM16W @ 0x50: u16 = 0_0 {
        /// LCD Blinking Memory 16/17
        BM16W: 0..15 = struct BM16WField(u16);
    }
    /// LCD Blinking Memory 18/19
    rw BM18W @ 0x52: u16 = 0_0 {
        /// LCD Blinking Memory 18/19
        BM18W: 0..15 = struct BM18WField(u16);
    }
}
