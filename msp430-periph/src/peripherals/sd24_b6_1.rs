//! SD24_B6

utils::periph! {
    /// SD24_B6
    SD24_B6;
    /// SD24B Control Register 0
    rw SD24BCTL0 @ 0x00: u16 = 0_0 {
        /// SD24B Overflow Control
        SD24OV32: 1 = struct SD24OV32(bool);
        /// SD24B Reference Select
        SD24REFS: 2 = struct SD24REFS(bool);
        /// SD24B Clock Source Select 0
        SD24SSEL: 4..5 = enum SD24SSEL {
            /// SD24B Clock Source Select MCLK
            SD24SSEL_0 = 0b00,
            /// SD24B Clock Source Select SMCLK
            SD24SSEL_1 = 0b01,
            /// SD24B Clock Source Select ACLK
            SD24SSEL_2 = 0b10,
            /// SD24B Clock Source Select TACLK
            SD24SSEL_3 = 0b11,
        }
        /// SD24B Modulator clock to Manchester decoder clock ratio
        SD24M4: 6 = struct SD24M4(bool);
        /// SD24B Clock Output Select
        SD24CLKOS: 7 = struct SD24CLKOS(bool);
        /// SD24B Frequency pre-scaler Bit 0
        SD24PDIV: 8..10 = enum SD24PDIV {
            /// SD24B Frequency pre-scaler  /1
            SD24PDIV_0 = 0b000,
            /// SD24B Frequency pre-scaler  /2
            SD24PDIV_1 = 0b001,
            /// SD24B Frequency pre-scaler  /4
            SD24PDIV_2 = 0b010,
            /// SD24B Frequency pre-scaler  /8
            SD24PDIV_3 = 0b011,
            /// SD24B Frequency pre-scaler  /16
            SD24PDIV_4 = 0b100,
            /// SD24B Frequency pre-scaler  /32
            SD24PDIV_5 = 0b101,
            /// SD24B Frequency pre-scaler  /64
            SD24PDIV_6 = 0b110,
            /// SD24B Frequency pre-scaler  /128
            SD24PDIV_7 = 0b111,
        }
        /// SD24B Frequency Divider Bit 0
        SD24DIV0: 11 = struct SD24DIV0(bool);
        /// SD24B Frequency Divider Bit 1
        SD24DIV1: 12 = struct SD24DIV1(bool);
        /// SD24B Frequency Divider Bit 2
        SD24DIV2: 13 = struct SD24DIV2(bool);
        /// SD24B Frequency Divider Bit 3
        SD24DIV3: 14 = struct SD24DIV3(bool);
        /// SD24B Frequency Divider Bit 4
        SD24DIV4: 15 = struct SD24DIV4(bool);
    }
    /// SD24B Control Register 1
    rw SD24BCTL1 @ 0x02: u16 = 0_0 {
        /// SD24B Group 0 Start Conversion
        SD24GRP0SC: 0 = struct SD24GRP0SC(bool);
        /// SD24B Group 1 Start Conversion
        SD24GRP1SC: 1 = struct SD24GRP1SC(bool);
        /// SD24B Group 2 Start Conversion
        SD24GRP2SC: 2 = struct SD24GRP2SC(bool);
        /// SD24B Group 3 Start Conversion
        SD24GRP3SC: 3 = struct SD24GRP3SC(bool);
        /// SD24B DMA Trigger Select Bit 0
        SD24DMA: 8..11 = enum SD24DMA {
            /// SD24B DMA Trigger: 0
            SD24DMA_0 = 0b0000,
            /// SD24B DMA Trigger: 1
            SD24DMA_1 = 0b0001,
            /// SD24B DMA Trigger: 2
            SD24DMA_2 = 0b0010,
            /// SD24B DMA Trigger: 3
            SD24DMA_3 = 0b0011,
            /// SD24B DMA Trigger: 4
            SD24DMA_4 = 0b0100,
            /// SD24B DMA Trigger: 5
            SD24DMA_5 = 0b0101,
            /// SD24B DMA Trigger: 6
            SD24DMA_6 = 0b0110,
            /// SD24B DMA Trigger: 7
            SD24DMA_7 = 0b0111,
            /// SD24B DMA Trigger: 8
            SD24DMA_8 = 0b1000,
        }
    }
    /// SD24B Trigger Control Register
    rw SD24BTRGCTL @ 0x04: u16 = 0_0 {
        /// SD24B Start Conversion
        SD24BTRGCTL_SD24SC: 0 = struct SD24BTRGCTL_SD24SC(bool);
        /// SD24B Start Conversion Select Bit 0
        SD24BTRGCTL_SD24SCS: 1..3 = enum SD24BTRGCTL_SD24SCS {
            /// SD24B Start Conversion Select: 0
            SD24SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SD24SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SD24SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SD24SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SD24SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SD24SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SD24SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SD24SCS_7 = 0b111,
        }
        /// SD24B Single Trigger Mode
        SD24BTRGCTL_SD24SNGL: 8 = struct SD24BTRGCTL_SD24SNGL(bool);
        /// SD24B Trigger Interrupt Flag
        SD24TRGIFG: 10 = struct SD24TRGIFG(bool);
        /// SD24B Trigger Interrupt Enable
        SD24TRGIE: 11 = struct SD24TRGIE(bool);
    }
    /// SD24B Trigger OSR Control Register
    rw SD24BTRGOSR @ 0x06: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        SD24BTRGOSR_OSR0: 0 = struct SD24BTRGOSR_OSR0(bool);
        /// SD24B Oversampling Rate Bit: 1
        SD24BTRGOSR_OSR1: 1 = struct SD24BTRGOSR_OSR1(bool);
        /// SD24B Oversampling Rate Bit: 2
        SD24BTRGOSR_OSR2: 2 = struct SD24BTRGOSR_OSR2(bool);
        /// SD24B Oversampling Rate Bit: 3
        SD24BTRGOSR_OSR3: 3 = struct SD24BTRGOSR_OSR3(bool);
        /// SD24B Oversampling Rate Bit: 4
        SD24BTRGOSR_OSR4: 4 = struct SD24BTRGOSR_OSR4(bool);
        /// SD24B Oversampling Rate Bit: 5
        SD24BTRGOSR_OSR5: 5 = struct SD24BTRGOSR_OSR5(bool);
        /// SD24B Oversampling Rate Bit: 6
        SD24BTRGOSR_OSR6: 6 = struct SD24BTRGOSR_OSR6(bool);
        /// SD24B Oversampling Rate Bit: 7
        SD24BTRGOSR_OSR7: 7 = struct SD24BTRGOSR_OSR7(bool);
        /// SD24B Oversampling Rate Bit: 8
        SD24BTRGOSR_OSR8: 8 = struct SD24BTRGOSR_OSR8(bool);
        /// SD24B Oversampling Rate Bit: 9
        SD24BTRGOSR_OSR9: 9 = struct SD24BTRGOSR_OSR9(bool);
        /// SD24B Oversampling Rate Bit: 10
        SD24BTRGOSR_OSR10: 10 = struct SD24BTRGOSR_OSR10(bool);
    }
    /// SD24B Trigger Preload Register
    rw SD24BTRGPRE @ 0x08: u16 = 0_0 {
        /// SD24B Trigger Preload Register
        SD24BTRGPRE: 0..15 = struct SD24BTRGPREField(u16);
    }
    /// SD24B Interrupt Flag Register
    rw SD24BIFG @ 0x0a: u16 = 0_0 {
        /// SD24B Channel 0 Interrupt Flag
        SD24IFG0: 0 = struct SD24IFG0(bool);
        /// SD24B Channel 1 Interrupt Flag
        SD24IFG1: 1 = struct SD24IFG1(bool);
        /// SD24B Channel 2 Interrupt Flag
        SD24IFG2: 2 = struct SD24IFG2(bool);
        /// SD24B Channel 3 Interrupt Flag
        SD24IFG3: 3 = struct SD24IFG3(bool);
        /// SD24B Channel 4 Interrupt Flag
        SD24IFG4: 4 = struct SD24IFG4(bool);
        /// SD24B Channel 5 Interrupt Flag
        SD24IFG5: 5 = struct SD24IFG5(bool);
        /// SD24B Channel 0 Overflow Interrupt Flag
        SD24OVIFG0: 8 = struct SD24OVIFG0(bool);
        /// SD24B Channel 1 Overflow Interrupt Flag
        SD24OVIFG1: 9 = struct SD24OVIFG1(bool);
        /// SD24B Channel 2 Overflow Interrupt Flag
        SD24OVIFG2: 10 = struct SD24OVIFG2(bool);
        /// SD24B Channel 3 Overflow Interrupt Flag
        SD24OVIFG3: 11 = struct SD24OVIFG3(bool);
        /// SD24B Channel 4 Overflow Interrupt Flag
        SD24OVIFG4: 12 = struct SD24OVIFG4(bool);
        /// SD24B Channel 5 Overflow Interrupt Flag
        SD24OVIFG5: 13 = struct SD24OVIFG5(bool);
    }
    /// SD24B Interrupt Enable Register
    rw SD24BIE @ 0x0c: u16 = 0_0 {
        /// SD24B Channel 0 Interrupt Enable
        SD24IE0: 0 = struct SD24IE0(bool);
        /// SD24B Channel 1 Interrupt Enable
        SD24IE1: 1 = struct SD24IE1(bool);
        /// SD24B Channel 2 Interrupt Enable
        SD24IE2: 2 = struct SD24IE2(bool);
        /// SD24B Channel 3 Interrupt Enable
        SD24IE3: 3 = struct SD24IE3(bool);
        /// SD24B Channel 4 Interrupt Enable
        SD24IE4: 4 = struct SD24IE4(bool);
        /// SD24B Channel 5 Interrupt Enable
        SD24IE5: 5 = struct SD24IE5(bool);
        /// SD24B Channel 0 Overflow Interrupt Enable
        SD24OVIE0: 8 = struct SD24OVIE0(bool);
        /// SD24B Channel 1 Overflow Interrupt Enable
        SD24OVIE1: 9 = struct SD24OVIE1(bool);
        /// SD24B Channel 2 Overflow Interrupt Enable
        SD24OVIE2: 10 = struct SD24OVIE2(bool);
        /// SD24B Channel 3 Overflow Interrupt Enable
        SD24OVIE3: 11 = struct SD24OVIE3(bool);
        /// SD24B Channel 4 Overflow Interrupt Enable
        SD24OVIE4: 12 = struct SD24OVIE4(bool);
        /// SD24B Channel 5 Overflow Interrupt Enable
        SD24OVIE5: 13 = struct SD24OVIE5(bool);
    }
    /// SD24B Interrupt Vector Register
    rw SD24BIV @ 0x0e: u16 = 0_0 {
        /// SD24B Interrupt Vector Register
        SD24BIV: 0..15 = struct SD24BIVField(u16);
    }
    /// SD24B Channel 0 Control Register
    rw SD24BCCTL0 @ 0x10: u16 = 0_0 {
        /// SD24B Start Conversion
        SD24BCCTL0_SD24SC: 0 = struct SD24BCCTL0_SD24SC(bool);
        /// SD24B Start Conversion Select Bit 0
        SD24BCCTL0_SD24SCS: 1..3 = enum SD24BCCTL0_SD24SCS {
            /// SD24B Start Conversion Select: 0
            SD24SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SD24SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SD24SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SD24SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SD24SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SD24SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SD24SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SD24SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        SD24BCCTL0_SD24DF: 4..5 = enum SD24BCCTL0_SD24DF {
            /// SD24B Data Format: Offset Binary
            SD24DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            SD24DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        SD24BCCTL0_SD24ALGN: 6 = struct SD24BCCTL0_SD24ALGN(bool);
        /// SD24B Single Trigger Mode
        SD24BCCTL0_SD24SNGL: 8 = struct SD24BCCTL0_SD24SNGL(bool);
        /// SD24B Calibration
        SD24BCCTL0_SD24CAL: 9 = struct SD24BCCTL0_SD24CAL(bool);
        /// SD24B Digital Filter Bit: 0
        SD24BCCTL0_SD24DFS: 10..11 = enum SD24BCCTL0_SD24DFS {
            /// SD24B Digital Filter 0
            SD24DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            SD24DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            SD24DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            SD24DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        SD24BCCTL0_SD24DI: 12 = struct SD24BCCTL0_SD24DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        SD24BCCTL0_SD24MC: 13..14 = enum SD24BCCTL0_SD24MC {
            /// SD24B Manchaster Encoding 0
            SD24MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            SD24MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            SD24MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            SD24MC_3 = 0b11,
        }
    }
    /// SD24B Channel 0 Input Control Register
    rw SD24BINCTL0 @ 0x12: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        SD24BINCTL0_SD24GAIN: 3..5 = enum SD24BINCTL0_SD24GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            SD24GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            SD24GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            SD24GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            SD24GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            SD24GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            SD24GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            SD24GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            SD24GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        SD24BINCTL0_SD24INTDLY: 6..7 = enum SD24BINCTL0_SD24INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            SD24INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            SD24INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            SD24INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            SD24INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 0 OSR Control Register
    rw SD24BOSR0 @ 0x14: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        SD24BOSR0_OSR0: 0 = struct SD24BOSR0_OSR0(bool);
        /// SD24B Oversampling Rate Bit: 1
        SD24BOSR0_OSR1: 1 = struct SD24BOSR0_OSR1(bool);
        /// SD24B Oversampling Rate Bit: 2
        SD24BOSR0_OSR2: 2 = struct SD24BOSR0_OSR2(bool);
        /// SD24B Oversampling Rate Bit: 3
        SD24BOSR0_OSR3: 3 = struct SD24BOSR0_OSR3(bool);
        /// SD24B Oversampling Rate Bit: 4
        SD24BOSR0_OSR4: 4 = struct SD24BOSR0_OSR4(bool);
        /// SD24B Oversampling Rate Bit: 5
        SD24BOSR0_OSR5: 5 = struct SD24BOSR0_OSR5(bool);
        /// SD24B Oversampling Rate Bit: 6
        SD24BOSR0_OSR6: 6 = struct SD24BOSR0_OSR6(bool);
        /// SD24B Oversampling Rate Bit: 7
        SD24BOSR0_OSR7: 7 = struct SD24BOSR0_OSR7(bool);
        /// SD24B Oversampling Rate Bit: 8
        SD24BOSR0_OSR8: 8 = struct SD24BOSR0_OSR8(bool);
        /// SD24B Oversampling Rate Bit: 9
        SD24BOSR0_OSR9: 9 = struct SD24BOSR0_OSR9(bool);
        /// SD24B Oversampling Rate Bit: 10
        SD24BOSR0_OSR10: 10 = struct SD24BOSR0_OSR10(bool);
    }
    /// SD24B Channel 0 Preload Register
    rw SD24BPRE0 @ 0x16: u16 = 0_0 {
        /// SD24B Channel 0 Preload Register
        SD24BPRE0: 0..15 = struct SD24BPRE0Field(u16);
    }
    /// SD24B Channel 1 Control Register
    rw SD24BCCTL1 @ 0x18: u16 = 0_0 {
        /// SD24B Start Conversion
        SD24BCCTL1_SD24SC: 0 = struct SD24BCCTL1_SD24SC(bool);
        /// SD24B Start Conversion Select Bit 0
        SD24BCCTL1_SD24SCS: 1..3 = enum SD24BCCTL1_SD24SCS {
            /// SD24B Start Conversion Select: 0
            SD24SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SD24SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SD24SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SD24SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SD24SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SD24SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SD24SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SD24SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        SD24BCCTL1_SD24DF: 4..5 = enum SD24BCCTL1_SD24DF {
            /// SD24B Data Format: Offset Binary
            SD24DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            SD24DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        SD24BCCTL1_SD24ALGN: 6 = struct SD24BCCTL1_SD24ALGN(bool);
        /// SD24B Single Trigger Mode
        SD24BCCTL1_SD24SNGL: 8 = struct SD24BCCTL1_SD24SNGL(bool);
        /// SD24B Calibration
        SD24BCCTL1_SD24CAL: 9 = struct SD24BCCTL1_SD24CAL(bool);
        /// SD24B Digital Filter Bit: 0
        SD24BCCTL1_SD24DFS: 10..11 = enum SD24BCCTL1_SD24DFS {
            /// SD24B Digital Filter 0
            SD24DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            SD24DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            SD24DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            SD24DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        SD24BCCTL1_SD24DI: 12 = struct SD24BCCTL1_SD24DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        SD24BCCTL1_SD24MC: 13..14 = enum SD24BCCTL1_SD24MC {
            /// SD24B Manchaster Encoding 0
            SD24MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            SD24MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            SD24MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            SD24MC_3 = 0b11,
        }
    }
    /// SD24B Channel 1 Input Control Register
    rw SD24BINCTL1 @ 0x1a: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        SD24BINCTL1_SD24GAIN: 3..5 = enum SD24BINCTL1_SD24GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            SD24GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            SD24GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            SD24GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            SD24GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            SD24GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            SD24GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            SD24GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            SD24GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        SD24BINCTL1_SD24INTDLY: 6..7 = enum SD24BINCTL1_SD24INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            SD24INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            SD24INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            SD24INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            SD24INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 1 OSR Control Register
    rw SD24BOSR1 @ 0x1c: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        SD24BOSR1_OSR0: 0 = struct SD24BOSR1_OSR0(bool);
        /// SD24B Oversampling Rate Bit: 1
        SD24BOSR1_OSR1: 1 = struct SD24BOSR1_OSR1(bool);
        /// SD24B Oversampling Rate Bit: 2
        SD24BOSR1_OSR2: 2 = struct SD24BOSR1_OSR2(bool);
        /// SD24B Oversampling Rate Bit: 3
        SD24BOSR1_OSR3: 3 = struct SD24BOSR1_OSR3(bool);
        /// SD24B Oversampling Rate Bit: 4
        SD24BOSR1_OSR4: 4 = struct SD24BOSR1_OSR4(bool);
        /// SD24B Oversampling Rate Bit: 5
        SD24BOSR1_OSR5: 5 = struct SD24BOSR1_OSR5(bool);
        /// SD24B Oversampling Rate Bit: 6
        SD24BOSR1_OSR6: 6 = struct SD24BOSR1_OSR6(bool);
        /// SD24B Oversampling Rate Bit: 7
        SD24BOSR1_OSR7: 7 = struct SD24BOSR1_OSR7(bool);
        /// SD24B Oversampling Rate Bit: 8
        SD24BOSR1_OSR8: 8 = struct SD24BOSR1_OSR8(bool);
        /// SD24B Oversampling Rate Bit: 9
        SD24BOSR1_OSR9: 9 = struct SD24BOSR1_OSR9(bool);
        /// SD24B Oversampling Rate Bit: 10
        SD24BOSR1_OSR10: 10 = struct SD24BOSR1_OSR10(bool);
    }
    /// SD24B Channel 1 Preload Register
    rw SD24BPRE1 @ 0x1e: u16 = 0_0 {
        /// SD24B Channel 1 Preload Register
        SD24BPRE1: 0..15 = struct SD24BPRE1Field(u16);
    }
    /// SD24B Channel 2 Control Register
    rw SD24BCCTL2 @ 0x20: u16 = 0_0 {
        /// SD24B Start Conversion
        SD24BCCTL2_SD24SC: 0 = struct SD24BCCTL2_SD24SC(bool);
        /// SD24B Start Conversion Select Bit 0
        SD24BCCTL2_SD24SCS: 1..3 = enum SD24BCCTL2_SD24SCS {
            /// SD24B Start Conversion Select: 0
            SD24SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SD24SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SD24SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SD24SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SD24SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SD24SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SD24SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SD24SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        SD24BCCTL2_SD24DF: 4..5 = enum SD24BCCTL2_SD24DF {
            /// SD24B Data Format: Offset Binary
            SD24DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            SD24DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        SD24BCCTL2_SD24ALGN: 6 = struct SD24BCCTL2_SD24ALGN(bool);
        /// SD24B Single Trigger Mode
        SD24BCCTL2_SD24SNGL: 8 = struct SD24BCCTL2_SD24SNGL(bool);
        /// SD24B Calibration
        SD24BCCTL2_SD24CAL: 9 = struct SD24BCCTL2_SD24CAL(bool);
        /// SD24B Digital Filter Bit: 0
        SD24BCCTL2_SD24DFS: 10..11 = enum SD24BCCTL2_SD24DFS {
            /// SD24B Digital Filter 0
            SD24DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            SD24DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            SD24DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            SD24DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        SD24BCCTL2_SD24DI: 12 = struct SD24BCCTL2_SD24DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        SD24BCCTL2_SD24MC: 13..14 = enum SD24BCCTL2_SD24MC {
            /// SD24B Manchaster Encoding 0
            SD24MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            SD24MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            SD24MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            SD24MC_3 = 0b11,
        }
    }
    /// SD24B Channel 2 Input Control Register
    rw SD24BINCTL2 @ 0x22: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        SD24BINCTL2_SD24GAIN: 3..5 = enum SD24BINCTL2_SD24GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            SD24GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            SD24GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            SD24GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            SD24GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            SD24GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            SD24GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            SD24GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            SD24GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        SD24BINCTL2_SD24INTDLY: 6..7 = enum SD24BINCTL2_SD24INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            SD24INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            SD24INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            SD24INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            SD24INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 2 OSR Control Register
    rw SD24BOSR2 @ 0x24: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        SD24BOSR2_OSR0: 0 = struct SD24BOSR2_OSR0(bool);
        /// SD24B Oversampling Rate Bit: 1
        SD24BOSR2_OSR1: 1 = struct SD24BOSR2_OSR1(bool);
        /// SD24B Oversampling Rate Bit: 2
        SD24BOSR2_OSR2: 2 = struct SD24BOSR2_OSR2(bool);
        /// SD24B Oversampling Rate Bit: 3
        SD24BOSR2_OSR3: 3 = struct SD24BOSR2_OSR3(bool);
        /// SD24B Oversampling Rate Bit: 4
        SD24BOSR2_OSR4: 4 = struct SD24BOSR2_OSR4(bool);
        /// SD24B Oversampling Rate Bit: 5
        SD24BOSR2_OSR5: 5 = struct SD24BOSR2_OSR5(bool);
        /// SD24B Oversampling Rate Bit: 6
        SD24BOSR2_OSR6: 6 = struct SD24BOSR2_OSR6(bool);
        /// SD24B Oversampling Rate Bit: 7
        SD24BOSR2_OSR7: 7 = struct SD24BOSR2_OSR7(bool);
        /// SD24B Oversampling Rate Bit: 8
        SD24BOSR2_OSR8: 8 = struct SD24BOSR2_OSR8(bool);
        /// SD24B Oversampling Rate Bit: 9
        SD24BOSR2_OSR9: 9 = struct SD24BOSR2_OSR9(bool);
        /// SD24B Oversampling Rate Bit: 10
        SD24BOSR2_OSR10: 10 = struct SD24BOSR2_OSR10(bool);
    }
    /// SD24B Channel 2 Preload Register
    rw SD24BPRE2 @ 0x26: u16 = 0_0 {
        /// SD24B Channel 2 Preload Register
        SD24BPRE2: 0..15 = struct SD24BPRE2Field(u16);
    }
    /// SD24B Channel 3 Control Register
    rw SD24BCCTL3 @ 0x28: u16 = 0_0 {
        /// SD24B Start Conversion
        SD24BCCTL3_SD24SC: 0 = struct SD24BCCTL3_SD24SC(bool);
        /// SD24B Start Conversion Select Bit 0
        SD24BCCTL3_SD24SCS: 1..3 = enum SD24BCCTL3_SD24SCS {
            /// SD24B Start Conversion Select: 0
            SD24SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SD24SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SD24SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SD24SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SD24SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SD24SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SD24SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SD24SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        SD24BCCTL3_SD24DF: 4..5 = enum SD24BCCTL3_SD24DF {
            /// SD24B Data Format: Offset Binary
            SD24DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            SD24DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        SD24BCCTL3_SD24ALGN: 6 = struct SD24BCCTL3_SD24ALGN(bool);
        /// SD24B Single Trigger Mode
        SD24BCCTL3_SD24SNGL: 8 = struct SD24BCCTL3_SD24SNGL(bool);
        /// SD24B Calibration
        SD24BCCTL3_SD24CAL: 9 = struct SD24BCCTL3_SD24CAL(bool);
        /// SD24B Digital Filter Bit: 0
        SD24BCCTL3_SD24DFS: 10..11 = enum SD24BCCTL3_SD24DFS {
            /// SD24B Digital Filter 0
            SD24DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            SD24DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            SD24DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            SD24DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        SD24BCCTL3_SD24DI: 12 = struct SD24BCCTL3_SD24DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        SD24BCCTL3_SD24MC: 13..14 = enum SD24BCCTL3_SD24MC {
            /// SD24B Manchaster Encoding 0
            SD24MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            SD24MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            SD24MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            SD24MC_3 = 0b11,
        }
    }
    /// SD24B Channel 3 Input Control Register
    rw SD24BINCTL3 @ 0x2a: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        SD24BINCTL3_SD24GAIN: 3..5 = enum SD24BINCTL3_SD24GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            SD24GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            SD24GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            SD24GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            SD24GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            SD24GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            SD24GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            SD24GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            SD24GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        SD24BINCTL3_SD24INTDLY: 6..7 = enum SD24BINCTL3_SD24INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            SD24INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            SD24INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            SD24INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            SD24INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 3 OSR Control Register
    rw SD24BOSR3 @ 0x2c: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        SD24BOSR3_OSR0: 0 = struct SD24BOSR3_OSR0(bool);
        /// SD24B Oversampling Rate Bit: 1
        SD24BOSR3_OSR1: 1 = struct SD24BOSR3_OSR1(bool);
        /// SD24B Oversampling Rate Bit: 2
        SD24BOSR3_OSR2: 2 = struct SD24BOSR3_OSR2(bool);
        /// SD24B Oversampling Rate Bit: 3
        SD24BOSR3_OSR3: 3 = struct SD24BOSR3_OSR3(bool);
        /// SD24B Oversampling Rate Bit: 4
        SD24BOSR3_OSR4: 4 = struct SD24BOSR3_OSR4(bool);
        /// SD24B Oversampling Rate Bit: 5
        SD24BOSR3_OSR5: 5 = struct SD24BOSR3_OSR5(bool);
        /// SD24B Oversampling Rate Bit: 6
        SD24BOSR3_OSR6: 6 = struct SD24BOSR3_OSR6(bool);
        /// SD24B Oversampling Rate Bit: 7
        SD24BOSR3_OSR7: 7 = struct SD24BOSR3_OSR7(bool);
        /// SD24B Oversampling Rate Bit: 8
        SD24BOSR3_OSR8: 8 = struct SD24BOSR3_OSR8(bool);
        /// SD24B Oversampling Rate Bit: 9
        SD24BOSR3_OSR9: 9 = struct SD24BOSR3_OSR9(bool);
        /// SD24B Oversampling Rate Bit: 10
        SD24BOSR3_OSR10: 10 = struct SD24BOSR3_OSR10(bool);
    }
    /// SD24B Channel 3 Preload Register
    rw SD24BPRE3 @ 0x2e: u16 = 0_0 {
        /// SD24B Channel 3 Preload Register
        SD24BPRE3: 0..15 = struct SD24BPRE3Field(u16);
    }
    /// SD24B Channel 4 Control Register
    rw SD24BCCTL4 @ 0x30: u16 = 0_0 {
        /// SD24B Start Conversion
        SD24BCCTL4_SD24SC: 0 = struct SD24BCCTL4_SD24SC(bool);
        /// SD24B Start Conversion Select Bit 0
        SD24BCCTL4_SD24SCS: 1..3 = enum SD24BCCTL4_SD24SCS {
            /// SD24B Start Conversion Select: 0
            SD24SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SD24SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SD24SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SD24SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SD24SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SD24SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SD24SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SD24SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        SD24BCCTL4_SD24DF: 4..5 = enum SD24BCCTL4_SD24DF {
            /// SD24B Data Format: Offset Binary
            SD24DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            SD24DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        SD24BCCTL4_SD24ALGN: 6 = struct SD24BCCTL4_SD24ALGN(bool);
        /// SD24B Single Trigger Mode
        SD24BCCTL4_SD24SNGL: 8 = struct SD24BCCTL4_SD24SNGL(bool);
        /// SD24B Calibration
        SD24BCCTL4_SD24CAL: 9 = struct SD24BCCTL4_SD24CAL(bool);
        /// SD24B Digital Filter Bit: 0
        SD24BCCTL4_SD24DFS: 10..11 = enum SD24BCCTL4_SD24DFS {
            /// SD24B Digital Filter 0
            SD24DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            SD24DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            SD24DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            SD24DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        SD24BCCTL4_SD24DI: 12 = struct SD24BCCTL4_SD24DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        SD24BCCTL4_SD24MC: 13..14 = enum SD24BCCTL4_SD24MC {
            /// SD24B Manchaster Encoding 0
            SD24MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            SD24MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            SD24MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            SD24MC_3 = 0b11,
        }
    }
    /// SD24B Channel 4 Input Control Register
    rw SD24BINCTL4 @ 0x32: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        SD24BINCTL4_SD24GAIN: 3..5 = enum SD24BINCTL4_SD24GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            SD24GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            SD24GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            SD24GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            SD24GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            SD24GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            SD24GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            SD24GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            SD24GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        SD24BINCTL4_SD24INTDLY: 6..7 = enum SD24BINCTL4_SD24INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            SD24INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            SD24INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            SD24INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            SD24INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 4 OSR Control Register
    rw SD24BOSR4 @ 0x34: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        SD24BOSR4_OSR0: 0 = struct SD24BOSR4_OSR0(bool);
        /// SD24B Oversampling Rate Bit: 1
        SD24BOSR4_OSR1: 1 = struct SD24BOSR4_OSR1(bool);
        /// SD24B Oversampling Rate Bit: 2
        SD24BOSR4_OSR2: 2 = struct SD24BOSR4_OSR2(bool);
        /// SD24B Oversampling Rate Bit: 3
        SD24BOSR4_OSR3: 3 = struct SD24BOSR4_OSR3(bool);
        /// SD24B Oversampling Rate Bit: 4
        SD24BOSR4_OSR4: 4 = struct SD24BOSR4_OSR4(bool);
        /// SD24B Oversampling Rate Bit: 5
        SD24BOSR4_OSR5: 5 = struct SD24BOSR4_OSR5(bool);
        /// SD24B Oversampling Rate Bit: 6
        SD24BOSR4_OSR6: 6 = struct SD24BOSR4_OSR6(bool);
        /// SD24B Oversampling Rate Bit: 7
        SD24BOSR4_OSR7: 7 = struct SD24BOSR4_OSR7(bool);
        /// SD24B Oversampling Rate Bit: 8
        SD24BOSR4_OSR8: 8 = struct SD24BOSR4_OSR8(bool);
        /// SD24B Oversampling Rate Bit: 9
        SD24BOSR4_OSR9: 9 = struct SD24BOSR4_OSR9(bool);
        /// SD24B Oversampling Rate Bit: 10
        SD24BOSR4_OSR10: 10 = struct SD24BOSR4_OSR10(bool);
    }
    /// SD24B Channel 4 Preload Register
    rw SD24BPRE4 @ 0x36: u16 = 0_0 {
        /// SD24B Channel 4 Preload Register
        SD24BPRE4: 0..15 = struct SD24BPRE4Field(u16);
    }
    /// SD24B Channel 5 Control Register
    rw SD24BCCTL5 @ 0x38: u16 = 0_0 {
        /// SD24B Start Conversion
        SD24BCCTL5_SD24SC: 0 = struct SD24BCCTL5_SD24SC(bool);
        /// SD24B Start Conversion Select Bit 0
        SD24BCCTL5_SD24SCS: 1..3 = enum SD24BCCTL5_SD24SCS {
            /// SD24B Start Conversion Select: 0
            SD24SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SD24SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SD24SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SD24SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SD24SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SD24SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SD24SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SD24SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        SD24BCCTL5_SD24DF: 4..5 = enum SD24BCCTL5_SD24DF {
            /// SD24B Data Format: Offset Binary
            SD24DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            SD24DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        SD24BCCTL5_SD24ALGN: 6 = struct SD24BCCTL5_SD24ALGN(bool);
        /// SD24B Single Trigger Mode
        SD24BCCTL5_SD24SNGL: 8 = struct SD24BCCTL5_SD24SNGL(bool);
        /// SD24B Calibration
        SD24BCCTL5_SD24CAL: 9 = struct SD24BCCTL5_SD24CAL(bool);
        /// SD24B Digital Filter Bit: 0
        SD24BCCTL5_SD24DFS: 10..11 = enum SD24BCCTL5_SD24DFS {
            /// SD24B Digital Filter 0
            SD24DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            SD24DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            SD24DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            SD24DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        SD24BCCTL5_SD24DI: 12 = struct SD24BCCTL5_SD24DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        SD24BCCTL5_SD24MC: 13..14 = enum SD24BCCTL5_SD24MC {
            /// SD24B Manchaster Encoding 0
            SD24MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            SD24MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            SD24MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            SD24MC_3 = 0b11,
        }
    }
    /// SD24B Channel 5 Input Control Register
    rw SD24BINCTL5 @ 0x3a: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        SD24BINCTL5_SD24GAIN: 3..5 = enum SD24BINCTL5_SD24GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            SD24GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            SD24GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            SD24GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            SD24GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            SD24GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            SD24GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            SD24GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            SD24GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        SD24BINCTL5_SD24INTDLY: 6..7 = enum SD24BINCTL5_SD24INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            SD24INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            SD24INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            SD24INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            SD24INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 5 OSR Control Register
    rw SD24BOSR5 @ 0x3c: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        SD24BOSR5_OSR0: 0 = struct SD24BOSR5_OSR0(bool);
        /// SD24B Oversampling Rate Bit: 1
        SD24BOSR5_OSR1: 1 = struct SD24BOSR5_OSR1(bool);
        /// SD24B Oversampling Rate Bit: 2
        SD24BOSR5_OSR2: 2 = struct SD24BOSR5_OSR2(bool);
        /// SD24B Oversampling Rate Bit: 3
        SD24BOSR5_OSR3: 3 = struct SD24BOSR5_OSR3(bool);
        /// SD24B Oversampling Rate Bit: 4
        SD24BOSR5_OSR4: 4 = struct SD24BOSR5_OSR4(bool);
        /// SD24B Oversampling Rate Bit: 5
        SD24BOSR5_OSR5: 5 = struct SD24BOSR5_OSR5(bool);
        /// SD24B Oversampling Rate Bit: 6
        SD24BOSR5_OSR6: 6 = struct SD24BOSR5_OSR6(bool);
        /// SD24B Oversampling Rate Bit: 7
        SD24BOSR5_OSR7: 7 = struct SD24BOSR5_OSR7(bool);
        /// SD24B Oversampling Rate Bit: 8
        SD24BOSR5_OSR8: 8 = struct SD24BOSR5_OSR8(bool);
        /// SD24B Oversampling Rate Bit: 9
        SD24BOSR5_OSR9: 9 = struct SD24BOSR5_OSR9(bool);
        /// SD24B Oversampling Rate Bit: 10
        SD24BOSR5_OSR10: 10 = struct SD24BOSR5_OSR10(bool);
    }
    /// SD24B Channel 5 Preload Register
    rw SD24BPRE5 @ 0x3e: u16 = 0_0 {
        /// SD24B Channel 5 Preload Register
        SD24BPRE5: 0..15 = struct SD24BPRE5Field(u16);
    }
    /// SD24B Channel 0 Conversion Memory Low word
    rw SD24BMEML0 @ 0x50: u16 = 0_0 {
        /// SD24B Channel 0 Conversion Memory Low word
        SD24BMEML0: 0..15 = struct SD24BMEML0Field(u16);
    }
    /// SD24B Channel 0 Conversion Memory High Word
    rw SD24BMEMH0 @ 0x52: u16 = 0_0 {
        /// SD24B Channel 0 Conversion Memory High Word
        SD24BMEMH0: 0..15 = struct SD24BMEMH0Field(u16);
    }
    /// SD24B Channel 1 Conversion Memory Low word
    rw SD24BMEML1 @ 0x54: u16 = 0_0 {
        /// SD24B Channel 1 Conversion Memory Low word
        SD24BMEML1: 0..15 = struct SD24BMEML1Field(u16);
    }
    /// SD24B Channel 1 Conversion Memory High Word
    rw SD24BMEMH1 @ 0x56: u16 = 0_0 {
        /// SD24B Channel 1 Conversion Memory High Word
        SD24BMEMH1: 0..15 = struct SD24BMEMH1Field(u16);
    }
    /// SD24B Channel 2 Conversion Memory Low word
    rw SD24BMEML2 @ 0x58: u16 = 0_0 {
        /// SD24B Channel 2 Conversion Memory Low word
        SD24BMEML2: 0..15 = struct SD24BMEML2Field(u16);
    }
    /// SD24B Channel 2 Conversion Memory High Word
    rw SD24BMEMH2 @ 0x5a: u16 = 0_0 {
        /// SD24B Channel 2 Conversion Memory High Word
        SD24BMEMH2: 0..15 = struct SD24BMEMH2Field(u16);
    }
    /// SD24B Channel 3 Conversion Memory Low word
    rw SD24BMEML3 @ 0x5c: u16 = 0_0 {
        /// SD24B Channel 3 Conversion Memory Low word
        SD24BMEML3: 0..15 = struct SD24BMEML3Field(u16);
    }
    /// SD24B Channel 3 Conversion Memory High Word
    rw SD24BMEMH3 @ 0x5e: u16 = 0_0 {
        /// SD24B Channel 3 Conversion Memory High Word
        SD24BMEMH3: 0..15 = struct SD24BMEMH3Field(u16);
    }
    /// SD24B Channel 4 Conversion Memory Low word
    rw SD24BMEML4 @ 0x60: u16 = 0_0 {
        /// SD24B Channel 4 Conversion Memory Low word
        SD24BMEML4: 0..15 = struct SD24BMEML4Field(u16);
    }
    /// SD24B Channel 4 Conversion Memory High Word
    rw SD24BMEMH4 @ 0x62: u16 = 0_0 {
        /// SD24B Channel 4 Conversion Memory High Word
        SD24BMEMH4: 0..15 = struct SD24BMEMH4Field(u16);
    }
    /// SD24B Channel 5 Conversion Memory Low word
    rw SD24BMEML5 @ 0x64: u16 = 0_0 {
        /// SD24B Channel 5 Conversion Memory Low word
        SD24BMEML5: 0..15 = struct SD24BMEML5Field(u16);
    }
    /// SD24B Channel 5 Conversion Memory High Word
    rw SD24BMEMH5 @ 0x66: u16 = 0_0 {
        /// SD24B Channel 5 Conversion Memory High Word
        SD24BMEMH5: 0..15 = struct SD24BMEMH5Field(u16);
    }
}
