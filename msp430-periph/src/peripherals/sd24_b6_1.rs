//! SD24_B6

utils::periph! {
    /// SD24_B6
    SD24_B6;
    /// SD24B Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// SD24B Overflow Control
        OV32: 1 = struct OV32(bool);
        /// SD24B Reference Select
        REFS: 2 = struct REFS(bool);
        /// SD24B Clock Source Select 0
        SSEL: 4..5 = enum SSEL {
            /// SD24B Clock Source Select MCLK
            SSEL_0 = 0b00,
            /// SD24B Clock Source Select SMCLK
            SSEL_1 = 0b01,
            /// SD24B Clock Source Select ACLK
            SSEL_2 = 0b10,
            /// SD24B Clock Source Select TACLK
            SSEL_3 = 0b11,
        }
        /// SD24B Modulator clock to Manchester decoder clock ratio
        M4: 6 = struct M4(bool);
        /// SD24B Clock Output Select
        CLKOS: 7 = struct CLKOS(bool);
        /// SD24B Frequency pre-scaler Bit 0
        PDIV: 8..10 = enum PDIV {
            /// SD24B Frequency pre-scaler  /1
            PDIV_0 = 0b000,
            /// SD24B Frequency pre-scaler  /2
            PDIV_1 = 0b001,
            /// SD24B Frequency pre-scaler  /4
            PDIV_2 = 0b010,
            /// SD24B Frequency pre-scaler  /8
            PDIV_3 = 0b011,
            /// SD24B Frequency pre-scaler  /16
            PDIV_4 = 0b100,
            /// SD24B Frequency pre-scaler  /32
            PDIV_5 = 0b101,
            /// SD24B Frequency pre-scaler  /64
            PDIV_6 = 0b110,
            /// SD24B Frequency pre-scaler  /128
            PDIV_7 = 0b111,
        }
        /// SD24B Frequency Divider Bit 0
        DIV0: 11 = struct DIV0(bool);
        /// SD24B Frequency Divider Bit 1
        DIV1: 12 = struct DIV1(bool);
        /// SD24B Frequency Divider Bit 2
        DIV2: 13 = struct DIV2(bool);
        /// SD24B Frequency Divider Bit 3
        DIV3: 14 = struct DIV3(bool);
        /// SD24B Frequency Divider Bit 4
        DIV4: 15 = struct DIV4(bool);
    }
    /// SD24B Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// SD24B Group 0 Start Conversion
        GRP0SC: 0 = struct GRP0SC(bool);
        /// SD24B Group 1 Start Conversion
        GRP1SC: 1 = struct GRP1SC(bool);
        /// SD24B Group 2 Start Conversion
        GRP2SC: 2 = struct GRP2SC(bool);
        /// SD24B Group 3 Start Conversion
        GRP3SC: 3 = struct GRP3SC(bool);
        /// SD24B DMA Trigger Select Bit 0
        DMA: 8..11 = enum DMA {
            /// SD24B DMA Trigger: 0
            DMA_0 = 0b0000,
            /// SD24B DMA Trigger: 1
            DMA_1 = 0b0001,
            /// SD24B DMA Trigger: 2
            DMA_2 = 0b0010,
            /// SD24B DMA Trigger: 3
            DMA_3 = 0b0011,
            /// SD24B DMA Trigger: 4
            DMA_4 = 0b0100,
            /// SD24B DMA Trigger: 5
            DMA_5 = 0b0101,
            /// SD24B DMA Trigger: 6
            DMA_6 = 0b0110,
            /// SD24B DMA Trigger: 7
            DMA_7 = 0b0111,
            /// SD24B DMA Trigger: 8
            DMA_8 = 0b1000,
        }
    }
    /// SD24B Trigger Control Register
    rw TRGCTL @ 0x04: u16 = 0_0 {
        /// SD24B Start Conversion
        TRGCTL_SC: 0 = struct TRGCTL_SC(bool);
        /// SD24B Start Conversion Select Bit 0
        TRGCTL_SCS: 1..3 = enum TRGCTL_SCS {
            /// SD24B Start Conversion Select: 0
            SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SCS_7 = 0b111,
        }
        /// SD24B Single Trigger Mode
        TRGCTL_SNGL: 8 = struct TRGCTL_SNGL(bool);
        /// SD24B Trigger Interrupt Flag
        TRGIFG: 10 = struct TRGIFG(bool);
        /// SD24B Trigger Interrupt Enable
        TRGIE: 11 = struct TRGIE(bool);
    }
    /// SD24B Trigger OSR Control Register
    rw TRGOSR @ 0x06: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        TRGOSR_0: 0 = struct TRGOSR_0(bool);
        /// SD24B Oversampling Rate Bit: 1
        TRGOSR_1: 1 = struct TRGOSR_1(bool);
        /// SD24B Oversampling Rate Bit: 2
        TRGOSR_2: 2 = struct TRGOSR_2(bool);
        /// SD24B Oversampling Rate Bit: 3
        TRGOSR_3: 3 = struct TRGOSR_3(bool);
        /// SD24B Oversampling Rate Bit: 4
        TRGOSR_4: 4 = struct TRGOSR_4(bool);
        /// SD24B Oversampling Rate Bit: 5
        TRGOSR_5: 5 = struct TRGOSR_5(bool);
        /// SD24B Oversampling Rate Bit: 6
        TRGOSR_6: 6 = struct TRGOSR_6(bool);
        /// SD24B Oversampling Rate Bit: 7
        TRGOSR_7: 7 = struct TRGOSR_7(bool);
        /// SD24B Oversampling Rate Bit: 8
        TRGOSR_8: 8 = struct TRGOSR_8(bool);
        /// SD24B Oversampling Rate Bit: 9
        TRGOSR_9: 9 = struct TRGOSR_9(bool);
        /// SD24B Oversampling Rate Bit: 10
        TRGOSR_10: 10 = struct TRGOSR_10(bool);
    }
    /// SD24B Trigger Preload Register
    rw TRGPRE @ 0x08: u16 = 0_0 {
        /// SD24B Trigger Preload Register
        TRGPRE: 0..15 = struct TRGPREField(u16);
    }
    /// SD24B Interrupt Flag Register
    rw IFG @ 0x0a: u16 = 0_0 {
        /// SD24B Channel 0 Interrupt Flag
        IFG0: 0 = struct IFG0(bool);
        /// SD24B Channel 1 Interrupt Flag
        IFG1: 1 = struct IFG1(bool);
        /// SD24B Channel 2 Interrupt Flag
        IFG2: 2 = struct IFG2(bool);
        /// SD24B Channel 3 Interrupt Flag
        IFG3: 3 = struct IFG3(bool);
        /// SD24B Channel 4 Interrupt Flag
        IFG4: 4 = struct IFG4(bool);
        /// SD24B Channel 5 Interrupt Flag
        IFG5: 5 = struct IFG5(bool);
        /// SD24B Channel 0 Overflow Interrupt Flag
        OVIFG0: 8 = struct OVIFG0(bool);
        /// SD24B Channel 1 Overflow Interrupt Flag
        OVIFG1: 9 = struct OVIFG1(bool);
        /// SD24B Channel 2 Overflow Interrupt Flag
        OVIFG2: 10 = struct OVIFG2(bool);
        /// SD24B Channel 3 Overflow Interrupt Flag
        OVIFG3: 11 = struct OVIFG3(bool);
        /// SD24B Channel 4 Overflow Interrupt Flag
        OVIFG4: 12 = struct OVIFG4(bool);
        /// SD24B Channel 5 Overflow Interrupt Flag
        OVIFG5: 13 = struct OVIFG5(bool);
    }
    /// SD24B Interrupt Enable Register
    rw IE @ 0x0c: u16 = 0_0 {
        /// SD24B Channel 0 Interrupt Enable
        IE0: 0 = struct IE0(bool);
        /// SD24B Channel 1 Interrupt Enable
        IE1: 1 = struct IE1(bool);
        /// SD24B Channel 2 Interrupt Enable
        IE2: 2 = struct IE2(bool);
        /// SD24B Channel 3 Interrupt Enable
        IE3: 3 = struct IE3(bool);
        /// SD24B Channel 4 Interrupt Enable
        IE4: 4 = struct IE4(bool);
        /// SD24B Channel 5 Interrupt Enable
        IE5: 5 = struct IE5(bool);
        /// SD24B Channel 0 Overflow Interrupt Enable
        OVIE0: 8 = struct OVIE0(bool);
        /// SD24B Channel 1 Overflow Interrupt Enable
        OVIE1: 9 = struct OVIE1(bool);
        /// SD24B Channel 2 Overflow Interrupt Enable
        OVIE2: 10 = struct OVIE2(bool);
        /// SD24B Channel 3 Overflow Interrupt Enable
        OVIE3: 11 = struct OVIE3(bool);
        /// SD24B Channel 4 Overflow Interrupt Enable
        OVIE4: 12 = struct OVIE4(bool);
        /// SD24B Channel 5 Overflow Interrupt Enable
        OVIE5: 13 = struct OVIE5(bool);
    }
    /// SD24B Interrupt Vector Register
    rw IV @ 0x0e: u16 = 0_0 {
        /// SD24B Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
    /// SD24B Channel 0 Control Register
    rw CCTL0 @ 0x10: u16 = 0_0 {
        /// SD24B Start Conversion
        C0SC: 0 = struct C0SC(bool);
        /// SD24B Start Conversion Select Bit 0
        C0SCS: 1..3 = enum C0SCS {
            /// SD24B Start Conversion Select: 0
            SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        C0DF: 4..5 = enum C0DF {
            /// SD24B Data Format: Offset Binary
            DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        C0ALGN: 6 = struct C0ALGN(bool);
        /// SD24B Single Trigger Mode
        C0SNGL: 8 = struct C0SNGL(bool);
        /// SD24B Calibration
        C0CAL: 9 = struct C0CAL(bool);
        /// SD24B Digital Filter Bit: 0
        C0DFS: 10..11 = enum C0DFS {
            /// SD24B Digital Filter 0
            DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        C0DI: 12 = struct C0DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        C0MC: 13..14 = enum C0MC {
            /// SD24B Manchaster Encoding 0
            MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            MC_3 = 0b11,
        }
    }
    /// SD24B Channel 0 Input Control Register
    rw INCTL0 @ 0x12: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        IN0GAIN: 3..5 = enum IN0GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        IN0INTDLY: 6..7 = enum IN0INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 0 OSR Control Register
    rw OSR0 @ 0x14: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        OSR0_0: 0 = struct OSR0_0(bool);
        /// SD24B Oversampling Rate Bit: 1
        OSR0_1: 1 = struct OSR0_1(bool);
        /// SD24B Oversampling Rate Bit: 2
        OSR0_2: 2 = struct OSR0_2(bool);
        /// SD24B Oversampling Rate Bit: 3
        OSR0_3: 3 = struct OSR0_3(bool);
        /// SD24B Oversampling Rate Bit: 4
        OSR0_4: 4 = struct OSR0_4(bool);
        /// SD24B Oversampling Rate Bit: 5
        OSR0_5: 5 = struct OSR0_5(bool);
        /// SD24B Oversampling Rate Bit: 6
        OSR0_6: 6 = struct OSR0_6(bool);
        /// SD24B Oversampling Rate Bit: 7
        OSR0_7: 7 = struct OSR0_7(bool);
        /// SD24B Oversampling Rate Bit: 8
        OSR0_8: 8 = struct OSR0_8(bool);
        /// SD24B Oversampling Rate Bit: 9
        OSR0_9: 9 = struct OSR0_9(bool);
        /// SD24B Oversampling Rate Bit: 10
        OSR0_10: 10 = struct OSR0_10(bool);
    }
    /// SD24B Channel 0 Preload Register
    rw PRE0 @ 0x16: u16 = 0_0 {
        /// SD24B Channel 0 Preload Register
        PRE0: 0..15 = struct PRE0Field(u16);
    }
    /// SD24B Channel 1 Control Register
    rw CCTL1 @ 0x18: u16 = 0_0 {
        /// SD24B Start Conversion
        C1SC: 0 = struct C1SC(bool);
        /// SD24B Start Conversion Select Bit 0
        C1SCS: 1..3 = enum C1SCS {
            /// SD24B Start Conversion Select: 0
            SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        C1DF: 4..5 = enum C1DF {
            /// SD24B Data Format: Offset Binary
            DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        C1ALGN: 6 = struct C1ALGN(bool);
        /// SD24B Single Trigger Mode
        C1SNGL: 8 = struct C1SNGL(bool);
        /// SD24B Calibration
        C1CAL: 9 = struct C1CAL(bool);
        /// SD24B Digital Filter Bit: 0
        C1DFS: 10..11 = enum C1DFS {
            /// SD24B Digital Filter 0
            DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        C1DI: 12 = struct C1DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        C1MC: 13..14 = enum C1MC {
            /// SD24B Manchaster Encoding 0
            MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            MC_3 = 0b11,
        }
    }
    /// SD24B Channel 1 Input Control Register
    rw INCTL1 @ 0x1a: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        IN1GAIN: 3..5 = enum IN1GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        IN1INTDLY: 6..7 = enum IN1INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 1 OSR Control Register
    rw OSR1 @ 0x1c: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        OSR1_0: 0 = struct OSR1_0(bool);
        /// SD24B Oversampling Rate Bit: 1
        OSR1_1: 1 = struct OSR1_1(bool);
        /// SD24B Oversampling Rate Bit: 2
        OSR1_2: 2 = struct OSR1_2(bool);
        /// SD24B Oversampling Rate Bit: 3
        OSR1_3: 3 = struct OSR1_3(bool);
        /// SD24B Oversampling Rate Bit: 4
        OSR1_4: 4 = struct OSR1_4(bool);
        /// SD24B Oversampling Rate Bit: 5
        OSR1_5: 5 = struct OSR1_5(bool);
        /// SD24B Oversampling Rate Bit: 6
        OSR1_6: 6 = struct OSR1_6(bool);
        /// SD24B Oversampling Rate Bit: 7
        OSR1_7: 7 = struct OSR1_7(bool);
        /// SD24B Oversampling Rate Bit: 8
        OSR1_8: 8 = struct OSR1_8(bool);
        /// SD24B Oversampling Rate Bit: 9
        OSR1_9: 9 = struct OSR1_9(bool);
        /// SD24B Oversampling Rate Bit: 10
        OSR1_10: 10 = struct OSR1_10(bool);
    }
    /// SD24B Channel 1 Preload Register
    rw PRE1 @ 0x1e: u16 = 0_0 {
        /// SD24B Channel 1 Preload Register
        PRE1: 0..15 = struct PRE1Field(u16);
    }
    /// SD24B Channel 2 Control Register
    rw CCTL2 @ 0x20: u16 = 0_0 {
        /// SD24B Start Conversion
        C2SC: 0 = struct C2SC(bool);
        /// SD24B Start Conversion Select Bit 0
        C2SCS: 1..3 = enum C2SCS {
            /// SD24B Start Conversion Select: 0
            SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        C2DF: 4..5 = enum C2DF {
            /// SD24B Data Format: Offset Binary
            DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        C2ALGN: 6 = struct C2ALGN(bool);
        /// SD24B Single Trigger Mode
        C2SNGL: 8 = struct C2SNGL(bool);
        /// SD24B Calibration
        C2CAL: 9 = struct C2CAL(bool);
        /// SD24B Digital Filter Bit: 0
        C2DFS: 10..11 = enum C2DFS {
            /// SD24B Digital Filter 0
            DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        C2DI: 12 = struct C2DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        C2MC: 13..14 = enum C2MC {
            /// SD24B Manchaster Encoding 0
            MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            MC_3 = 0b11,
        }
    }
    /// SD24B Channel 2 Input Control Register
    rw INCTL2 @ 0x22: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        IN2GAIN: 3..5 = enum IN2GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        IN2INTDLY: 6..7 = enum IN2INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 2 OSR Control Register
    rw OSR2 @ 0x24: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        OSR2_0: 0 = struct OSR2_0(bool);
        /// SD24B Oversampling Rate Bit: 1
        OSR2_1: 1 = struct OSR2_1(bool);
        /// SD24B Oversampling Rate Bit: 2
        OSR2_2: 2 = struct OSR2_2(bool);
        /// SD24B Oversampling Rate Bit: 3
        OSR2_3: 3 = struct OSR2_3(bool);
        /// SD24B Oversampling Rate Bit: 4
        OSR2_4: 4 = struct OSR2_4(bool);
        /// SD24B Oversampling Rate Bit: 5
        OSR2_5: 5 = struct OSR2_5(bool);
        /// SD24B Oversampling Rate Bit: 6
        OSR2_6: 6 = struct OSR2_6(bool);
        /// SD24B Oversampling Rate Bit: 7
        OSR2_7: 7 = struct OSR2_7(bool);
        /// SD24B Oversampling Rate Bit: 8
        OSR2_8: 8 = struct OSR2_8(bool);
        /// SD24B Oversampling Rate Bit: 9
        OSR2_9: 9 = struct OSR2_9(bool);
        /// SD24B Oversampling Rate Bit: 10
        OSR2_10: 10 = struct OSR2_10(bool);
    }
    /// SD24B Channel 2 Preload Register
    rw PRE2 @ 0x26: u16 = 0_0 {
        /// SD24B Channel 2 Preload Register
        PRE2: 0..15 = struct PRE2Field(u16);
    }
    /// SD24B Channel 3 Control Register
    rw CCTL3 @ 0x28: u16 = 0_0 {
        /// SD24B Start Conversion
        C3SC: 0 = struct C3SC(bool);
        /// SD24B Start Conversion Select Bit 0
        C3SCS: 1..3 = enum C3SCS {
            /// SD24B Start Conversion Select: 0
            SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        C3DF: 4..5 = enum C3DF {
            /// SD24B Data Format: Offset Binary
            DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        C3ALGN: 6 = struct C3ALGN(bool);
        /// SD24B Single Trigger Mode
        C3SNGL: 8 = struct C3SNGL(bool);
        /// SD24B Calibration
        C3CAL: 9 = struct C3CAL(bool);
        /// SD24B Digital Filter Bit: 0
        C3DFS: 10..11 = enum C3DFS {
            /// SD24B Digital Filter 0
            DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        C3DI: 12 = struct C3DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        C3MC: 13..14 = enum C3MC {
            /// SD24B Manchaster Encoding 0
            MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            MC_3 = 0b11,
        }
    }
    /// SD24B Channel 3 Input Control Register
    rw INCTL3 @ 0x2a: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        IN3GAIN: 3..5 = enum IN3GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        IN3INTDLY: 6..7 = enum IN3INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 3 OSR Control Register
    rw OSR3 @ 0x2c: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        OSR3_0: 0 = struct OSR3_0(bool);
        /// SD24B Oversampling Rate Bit: 1
        OSR3_1: 1 = struct OSR3_1(bool);
        /// SD24B Oversampling Rate Bit: 2
        OSR3_2: 2 = struct OSR3_2(bool);
        /// SD24B Oversampling Rate Bit: 3
        OSR3_3: 3 = struct OSR3_3(bool);
        /// SD24B Oversampling Rate Bit: 4
        OSR3_4: 4 = struct OSR3_4(bool);
        /// SD24B Oversampling Rate Bit: 5
        OSR3_5: 5 = struct OSR3_5(bool);
        /// SD24B Oversampling Rate Bit: 6
        OSR3_6: 6 = struct OSR3_6(bool);
        /// SD24B Oversampling Rate Bit: 7
        OSR3_7: 7 = struct OSR3_7(bool);
        /// SD24B Oversampling Rate Bit: 8
        OSR3_8: 8 = struct OSR3_8(bool);
        /// SD24B Oversampling Rate Bit: 9
        OSR3_9: 9 = struct OSR3_9(bool);
        /// SD24B Oversampling Rate Bit: 10
        OSR3_10: 10 = struct OSR3_10(bool);
    }
    /// SD24B Channel 3 Preload Register
    rw PRE3 @ 0x2e: u16 = 0_0 {
        /// SD24B Channel 3 Preload Register
        PRE3: 0..15 = struct PRE3Field(u16);
    }
    /// SD24B Channel 4 Control Register
    rw CCTL4 @ 0x30: u16 = 0_0 {
        /// SD24B Start Conversion
        C4SC: 0 = struct C4SC(bool);
        /// SD24B Start Conversion Select Bit 0
        C4SCS: 1..3 = enum C4SCS {
            /// SD24B Start Conversion Select: 0
            SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        C4DF: 4..5 = enum C4DF {
            /// SD24B Data Format: Offset Binary
            DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        C4ALGN: 6 = struct C4ALGN(bool);
        /// SD24B Single Trigger Mode
        C4SNGL: 8 = struct C4SNGL(bool);
        /// SD24B Calibration
        C4CAL: 9 = struct C4CAL(bool);
        /// SD24B Digital Filter Bit: 0
        C4DFS: 10..11 = enum C4DFS {
            /// SD24B Digital Filter 0
            DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        C4DI: 12 = struct C4DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        C4MC: 13..14 = enum C4MC {
            /// SD24B Manchaster Encoding 0
            MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            MC_3 = 0b11,
        }
    }
    /// SD24B Channel 4 Input Control Register
    rw INCTL4 @ 0x32: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        IN4GAIN: 3..5 = enum IN4GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        IN4INTDLY: 6..7 = enum IN4INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 4 OSR Control Register
    rw OSR4 @ 0x34: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        OSR4_0: 0 = struct OSR4_0(bool);
        /// SD24B Oversampling Rate Bit: 1
        OSR4_1: 1 = struct OSR4_1(bool);
        /// SD24B Oversampling Rate Bit: 2
        OSR4_2: 2 = struct OSR4_2(bool);
        /// SD24B Oversampling Rate Bit: 3
        OSR4_3: 3 = struct OSR4_3(bool);
        /// SD24B Oversampling Rate Bit: 4
        OSR4_4: 4 = struct OSR4_4(bool);
        /// SD24B Oversampling Rate Bit: 5
        OSR4_5: 5 = struct OSR4_5(bool);
        /// SD24B Oversampling Rate Bit: 6
        OSR4_6: 6 = struct OSR4_6(bool);
        /// SD24B Oversampling Rate Bit: 7
        OSR4_7: 7 = struct OSR4_7(bool);
        /// SD24B Oversampling Rate Bit: 8
        OSR4_8: 8 = struct OSR4_8(bool);
        /// SD24B Oversampling Rate Bit: 9
        OSR4_9: 9 = struct OSR4_9(bool);
        /// SD24B Oversampling Rate Bit: 10
        OSR4_10: 10 = struct OSR4_10(bool);
    }
    /// SD24B Channel 4 Preload Register
    rw PRE4 @ 0x36: u16 = 0_0 {
        /// SD24B Channel 4 Preload Register
        PRE4: 0..15 = struct PRE4Field(u16);
    }
    /// SD24B Channel 5 Control Register
    rw CCTL5 @ 0x38: u16 = 0_0 {
        /// SD24B Start Conversion
        C5SC: 0 = struct C5SC(bool);
        /// SD24B Start Conversion Select Bit 0
        C5SCS: 1..3 = enum C5SCS {
            /// SD24B Start Conversion Select: 0
            SCS_0 = 0b000,
            /// SD24B Start Conversion Select: 1
            SCS_1 = 0b001,
            /// SD24B Start Conversion Select: 2
            SCS_2 = 0b010,
            /// SD24B Start Conversion Select: 3
            SCS_3 = 0b011,
            /// SD24B Start Conversion Select: 4
            SCS_4 = 0b100,
            /// SD24B Start Conversion Select: 5
            SCS_5 = 0b101,
            /// SD24B Start Conversion Select: 6
            SCS_6 = 0b110,
            /// SD24B Start Conversion Select: 7
            SCS_7 = 0b111,
        }
        /// SD24B Data Format Bit: 0
        C5DF: 4..5 = enum C5DF {
            /// SD24B Data Format: Offset Binary
            DF_0 = 0b00,
            /// SD24B Data Format: 2's complement
            DF_1 = 0b01,
        }
        /// SD24B Data Alignment
        C5ALGN: 6 = struct C5ALGN(bool);
        /// SD24B Single Trigger Mode
        C5SNGL: 8 = struct C5SNGL(bool);
        /// SD24B Calibration
        C5CAL: 9 = struct C5CAL(bool);
        /// SD24B Digital Filter Bit: 0
        C5DFS: 10..11 = enum C5DFS {
            /// SD24B Digital Filter 0
            DFS_0 = 0b00,
            /// SD24B Digital Filter 1
            DFS_1 = 0b01,
            /// SD24B Digital Filter 2
            DFS_2 = 0b10,
            /// SD24B Digital Filter 3
            DFS_3 = 0b11,
        }
        /// SD24B Digital Bitstream Input
        C5DI: 12 = struct C5DI(bool);
        /// SD24B Manchaster Encoding Bit: 0
        C5MC: 13..14 = enum C5MC {
            /// SD24B Manchaster Encoding 0
            MC_0 = 0b00,
            /// SD24B Manchaster Encoding 1
            MC_1 = 0b01,
            /// SD24B Manchaster Encoding 2
            MC_2 = 0b10,
            /// SD24B Manchaster Encoding 3
            MC_3 = 0b11,
        }
    }
    /// SD24B Channel 5 Input Control Register
    rw INCTL5 @ 0x3a: u16 = 0_0 {
        /// SD24B Input Pre-Amplifier Gain Select 0
        IN5GAIN: 3..5 = enum IN5GAIN {
            /// SD24B Input Pre-Amplifier Gain Select *1
            GAIN_1 = 0b000,
            /// SD24B Input Pre-Amplifier Gain Select *2
            GAIN_2 = 0b001,
            /// SD24B Input Pre-Amplifier Gain Select *4
            GAIN_4 = 0b010,
            /// SD24B Input Pre-Amplifier Gain Select *8
            GAIN_8 = 0b011,
            /// SD24B Input Pre-Amplifier Gain Select *16
            GAIN_16 = 0b100,
            /// SD24B Input Pre-Amplifier Gain Select *32
            GAIN_32 = 0b101,
            /// SD24B Input Pre-Amplifier Gain Select *64
            GAIN_64 = 0b110,
            /// SD24B Input Pre-Amplifier Gain Select *128
            GAIN_128 = 0b111,
        }
        /// SD24B Interrupt Delay after 1.Conversion 0
        IN5INTDLY: 6..7 = enum IN5INTDLY {
            /// SD24B Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD24B Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD24B Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD24B Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD24B Channel 5 OSR Control Register
    rw OSR5 @ 0x3c: u16 = 0_0 {
        /// SD24B Oversampling Rate Bit: 0
        OSR5_0: 0 = struct OSR5_0(bool);
        /// SD24B Oversampling Rate Bit: 1
        OSR5_1: 1 = struct OSR5_1(bool);
        /// SD24B Oversampling Rate Bit: 2
        OSR5_2: 2 = struct OSR5_2(bool);
        /// SD24B Oversampling Rate Bit: 3
        OSR5_3: 3 = struct OSR5_3(bool);
        /// SD24B Oversampling Rate Bit: 4
        OSR5_4: 4 = struct OSR5_4(bool);
        /// SD24B Oversampling Rate Bit: 5
        OSR5_5: 5 = struct OSR5_5(bool);
        /// SD24B Oversampling Rate Bit: 6
        OSR5_6: 6 = struct OSR5_6(bool);
        /// SD24B Oversampling Rate Bit: 7
        OSR5_7: 7 = struct OSR5_7(bool);
        /// SD24B Oversampling Rate Bit: 8
        OSR5_8: 8 = struct OSR5_8(bool);
        /// SD24B Oversampling Rate Bit: 9
        OSR5_9: 9 = struct OSR5_9(bool);
        /// SD24B Oversampling Rate Bit: 10
        OSR5_10: 10 = struct OSR5_10(bool);
    }
    /// SD24B Channel 5 Preload Register
    rw PRE5 @ 0x3e: u16 = 0_0 {
        /// SD24B Channel 5 Preload Register
        PRE5: 0..15 = struct PRE5Field(u16);
    }
    /// SD24B Channel 0 Conversion Memory Low word
    rw MEML0 @ 0x50: u16 = 0_0 {
        /// SD24B Channel 0 Conversion Memory Low word
        MEML0: 0..15 = struct MEML0Field(u16);
    }
    /// SD24B Channel 0 Conversion Memory High Word
    rw MEMH0 @ 0x52: u16 = 0_0 {
        /// SD24B Channel 0 Conversion Memory High Word
        MEMH0: 0..15 = struct MEMH0Field(u16);
    }
    /// SD24B Channel 1 Conversion Memory Low word
    rw MEML1 @ 0x54: u16 = 0_0 {
        /// SD24B Channel 1 Conversion Memory Low word
        MEML1: 0..15 = struct MEML1Field(u16);
    }
    /// SD24B Channel 1 Conversion Memory High Word
    rw MEMH1 @ 0x56: u16 = 0_0 {
        /// SD24B Channel 1 Conversion Memory High Word
        MEMH1: 0..15 = struct MEMH1Field(u16);
    }
    /// SD24B Channel 2 Conversion Memory Low word
    rw MEML2 @ 0x58: u16 = 0_0 {
        /// SD24B Channel 2 Conversion Memory Low word
        MEML2: 0..15 = struct MEML2Field(u16);
    }
    /// SD24B Channel 2 Conversion Memory High Word
    rw MEMH2 @ 0x5a: u16 = 0_0 {
        /// SD24B Channel 2 Conversion Memory High Word
        MEMH2: 0..15 = struct MEMH2Field(u16);
    }
    /// SD24B Channel 3 Conversion Memory Low word
    rw MEML3 @ 0x5c: u16 = 0_0 {
        /// SD24B Channel 3 Conversion Memory Low word
        MEML3: 0..15 = struct MEML3Field(u16);
    }
    /// SD24B Channel 3 Conversion Memory High Word
    rw MEMH3 @ 0x5e: u16 = 0_0 {
        /// SD24B Channel 3 Conversion Memory High Word
        MEMH3: 0..15 = struct MEMH3Field(u16);
    }
    /// SD24B Channel 4 Conversion Memory Low word
    rw MEML4 @ 0x60: u16 = 0_0 {
        /// SD24B Channel 4 Conversion Memory Low word
        MEML4: 0..15 = struct MEML4Field(u16);
    }
    /// SD24B Channel 4 Conversion Memory High Word
    rw MEMH4 @ 0x62: u16 = 0_0 {
        /// SD24B Channel 4 Conversion Memory High Word
        MEMH4: 0..15 = struct MEMH4Field(u16);
    }
    /// SD24B Channel 5 Conversion Memory Low word
    rw MEML5 @ 0x64: u16 = 0_0 {
        /// SD24B Channel 5 Conversion Memory Low word
        MEML5: 0..15 = struct MEML5Field(u16);
    }
    /// SD24B Channel 5 Conversion Memory High Word
    rw MEMH5 @ 0x66: u16 = 0_0 {
        /// SD24B Channel 5 Conversion Memory High Word
        MEMH5: 0..15 = struct MEMH5Field(u16);
    }
}
