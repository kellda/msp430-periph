//! SD24_A4

utils::periph! {
    /// SD24_A4
    SD24_A4;
    /// SD24 Input Control Register Channel 0
    rw SD24INCTL0 @ 0x00: u8 = 0_0 {
        /// SD24 Input Channel select 0
        SD24INCTL0_SD24INCH: 0..2 = enum SD24INCTL0_SD24INCH {
            /// SD24 Input Channel select input
            SD24INCH_0 = 0b000,
            /// SD24 Input Channel select input
            SD24INCH_1 = 0b001,
            /// SD24 Input Channel select input
            SD24INCH_2 = 0b010,
            /// SD24 Input Channel select input
            SD24INCH_3 = 0b011,
            /// SD24 Input Channel select input
            SD24INCH_4 = 0b100,
            /// SD24 Input Channel select Vcc divider
            SD24INCH_5 = 0b101,
            /// SD24 Input Channel select Temp
            SD24INCH_6 = 0b110,
            /// SD24 Input Channel select Offset
            SD24INCH_7 = 0b111,
        }
        /// SD24 Input Pre-Amplifier Gain Select 0
        SD24INCTL0_SD24GAIN0: 3 = struct SD24INCTL0_SD24GAIN0(bool);
        /// SD24 Input Pre-Amplifier Gain Select 1
        SD24INCTL0_SD24GAIN1: 4 = struct SD24INCTL0_SD24GAIN1(bool);
        /// SD24 Input Pre-Amplifier Gain Select 2
        SD24INCTL0_SD24GAIN2: 5 = struct SD24INCTL0_SD24GAIN2(bool);
        /// SD24 Interrupt Delay after 1.Conversion
        SD24INCTL0_SD24INTDLY: 6 = struct SD24INCTL0_SD24INTDLY(bool);
    }
    /// SD24 Input Control Register Channel 1
    rw SD24INCTL1 @ 0x01: u8 = 0_0 {
        /// SD24 Input Channel select 0
        SD24INCTL1_SD24INCH: 0..2 = enum SD24INCTL1_SD24INCH {
            /// SD24 Input Channel select input
            SD24INCH_0 = 0b000,
            /// SD24 Input Channel select input
            SD24INCH_1 = 0b001,
            /// SD24 Input Channel select input
            SD24INCH_2 = 0b010,
            /// SD24 Input Channel select input
            SD24INCH_3 = 0b011,
            /// SD24 Input Channel select input
            SD24INCH_4 = 0b100,
            /// SD24 Input Channel select Vcc divider
            SD24INCH_5 = 0b101,
            /// SD24 Input Channel select Temp
            SD24INCH_6 = 0b110,
            /// SD24 Input Channel select Offset
            SD24INCH_7 = 0b111,
        }
        /// SD24 Input Pre-Amplifier Gain Select 0
        SD24INCTL1_SD24GAIN0: 3 = struct SD24INCTL1_SD24GAIN0(bool);
        /// SD24 Input Pre-Amplifier Gain Select 1
        SD24INCTL1_SD24GAIN1: 4 = struct SD24INCTL1_SD24GAIN1(bool);
        /// SD24 Input Pre-Amplifier Gain Select 2
        SD24INCTL1_SD24GAIN2: 5 = struct SD24INCTL1_SD24GAIN2(bool);
        /// SD24 Interrupt Delay after 1.Conversion
        SD24INCTL1_SD24INTDLY: 6 = struct SD24INCTL1_SD24INTDLY(bool);
    }
    /// SD24 Input Control Register Channel 2
    rw SD24INCTL2 @ 0x02: u8 = 0_0 {
        /// SD24 Input Channel select 0
        SD24INCTL2_SD24INCH: 0..2 = enum SD24INCTL2_SD24INCH {
            /// SD24 Input Channel select input
            SD24INCH_0 = 0b000,
            /// SD24 Input Channel select input
            SD24INCH_1 = 0b001,
            /// SD24 Input Channel select input
            SD24INCH_2 = 0b010,
            /// SD24 Input Channel select input
            SD24INCH_3 = 0b011,
            /// SD24 Input Channel select input
            SD24INCH_4 = 0b100,
            /// SD24 Input Channel select Vcc divider
            SD24INCH_5 = 0b101,
            /// SD24 Input Channel select Temp
            SD24INCH_6 = 0b110,
            /// SD24 Input Channel select Offset
            SD24INCH_7 = 0b111,
        }
        /// SD24 Input Pre-Amplifier Gain Select 0
        SD24INCTL2_SD24GAIN0: 3 = struct SD24INCTL2_SD24GAIN0(bool);
        /// SD24 Input Pre-Amplifier Gain Select 1
        SD24INCTL2_SD24GAIN1: 4 = struct SD24INCTL2_SD24GAIN1(bool);
        /// SD24 Input Pre-Amplifier Gain Select 2
        SD24INCTL2_SD24GAIN2: 5 = struct SD24INCTL2_SD24GAIN2(bool);
        /// SD24 Interrupt Delay after 1.Conversion
        SD24INCTL2_SD24INTDLY: 6 = struct SD24INCTL2_SD24INTDLY(bool);
    }
    /// SD24 Input Control Register Channel 3
    rw SD24INCTL3 @ 0x03: u8 = 0_0 {
        /// SD24 Input Channel select 0
        SD24INCTL3_SD24INCH: 0..2 = enum SD24INCTL3_SD24INCH {
            /// SD24 Input Channel select input
            SD24INCH_0 = 0b000,
            /// SD24 Input Channel select input
            SD24INCH_1 = 0b001,
            /// SD24 Input Channel select input
            SD24INCH_2 = 0b010,
            /// SD24 Input Channel select input
            SD24INCH_3 = 0b011,
            /// SD24 Input Channel select input
            SD24INCH_4 = 0b100,
            /// SD24 Input Channel select Vcc divider
            SD24INCH_5 = 0b101,
            /// SD24 Input Channel select Temp
            SD24INCH_6 = 0b110,
            /// SD24 Input Channel select Offset
            SD24INCH_7 = 0b111,
        }
        /// SD24 Input Pre-Amplifier Gain Select 0
        SD24INCTL3_SD24GAIN0: 3 = struct SD24INCTL3_SD24GAIN0(bool);
        /// SD24 Input Pre-Amplifier Gain Select 1
        SD24INCTL3_SD24GAIN1: 4 = struct SD24INCTL3_SD24GAIN1(bool);
        /// SD24 Input Pre-Amplifier Gain Select 2
        SD24INCTL3_SD24GAIN2: 5 = struct SD24INCTL3_SD24GAIN2(bool);
        /// SD24 Interrupt Delay after 1.Conversion
        SD24INCTL3_SD24INTDLY: 6 = struct SD24INCTL3_SD24INTDLY(bool);
    }
    /// SD24 Preload Register Channel 0
    rw SD24PRE0 @ 0x08: u8 = 0_0 {
        /// SD24 Preload Register Channel 0
        SD24PRE0: 0..7 = struct SD24PRE0Field(u8);
    }
    /// SD24 Preload Register Channel 1
    rw SD24PRE1 @ 0x09: u8 = 0_0 {
        /// SD24 Preload Register Channel 1
        SD24PRE1: 0..7 = struct SD24PRE1Field(u8);
    }
    /// SD24 Preload Register Channel 2
    rw SD24PRE2 @ 0x0a: u8 = 0_0 {
        /// SD24 Preload Register Channel 2
        SD24PRE2: 0..7 = struct SD24PRE2Field(u8);
    }
    /// SD24 Preload Register Channel 3
    rw SD24PRE3 @ 0x0b: u8 = 0_0 {
        /// SD24 Preload Register Channel 3
        SD24PRE3: 0..7 = struct SD24PRE3Field(u8);
    }
    /// SD24 Trim Register 1
    rw SD24TRIM @ 0x0f: u8 = 0_0 {
        /// SD24 Trimming of bias current Bit: 0
        SD24IBTRIM0: 0 = struct SD24IBTRIM0(bool);
        /// SD24 Trimming of bias current Bit: 1
        SD24IBTRIM1: 1 = struct SD24IBTRIM1(bool);
        /// SD24 Trimming of bias current Bit: 2
        SD24IBTRIM2: 2 = struct SD24IBTRIM2(bool);
        /// SD24 Trimming of reference voltage buffer start up delay Bit: 0
        SD24REFDLYTRIM0: 3 = struct SD24REFDLYTRIM0(bool);
        /// SD24 Trimming of reference voltage buffer start up delay Bit: 1
        SD24REFDLYTRIM1: 4 = struct SD24REFDLYTRIM1(bool);
    }
    /// Sigma Delta ADC 24 Control Register
    rw SD24CTL @ 0x50: u16 = 0_0 {
        /// SD24 Overflow Interupt Enable
        SD24OVIE: 1 = struct SD24OVIE(bool);
        /// SD24 Reference Select
        SD24REFS: 2 = struct SD24REFS(bool);
    }
    /// SD24 Channel 0 Control Register
    rw SD24CCTL0 @ 0x52: u16 = 0_0 {
        /// SD24 Grouping of Channels: 0:Off/1:On
        SD24CCTL0_SD24GRP: 0 = struct SD24CCTL0_SD24GRP(bool);
        /// SD24 Start Conversion
        SD24CCTL0_SD24SC: 1 = struct SD24CCTL0_SD24SC(bool);
        /// SD24 Channel x Interrupt Flag
        SD24CCTL0_SD24IFG: 2 = struct SD24CCTL0_SD24IFG(bool);
        /// SD24 Channel x Interrupt Enable
        SD24CCTL0_SD24IE: 3 = struct SD24CCTL0_SD24IE(bool);
        /// SD24 Channel x Data Format: 0:Unipolar/1:Bipolar
        SD24CCTL0_SD24DF: 4 = struct SD24CCTL0_SD24DF(bool);
        /// SD24 Channel x Overflow Interrupt Flag
        SD24CCTL0_SD24OVIFG: 5 = struct SD24CCTL0_SD24OVIFG(bool);
        /// SD24 Channel x Access LSB of ADC
        SD24CCTL0_SD24LSBACC: 6 = struct SD24CCTL0_SD24LSBACC(bool);
        /// SD24 Channel x Toggle LSB Output of ADC
        SD24CCTL0_SD24LSBTOG: 7 = struct SD24CCTL0_SD24LSBTOG(bool);
        /// SD24 Channel x OverSampling Ratio 0
        SD24CCTL0_SD24OSR0: 8 = struct SD24CCTL0_SD24OSR0(bool);
        /// SD24 Channel x OverSampling Ratio 1
        SD24CCTL0_SD24OSR1: 9 = struct SD24CCTL0_SD24OSR1(bool);
        /// SD24 Channel x Single Conversion On/Off
        SD24CCTL0_SD24SNGL: 10 = struct SD24CCTL0_SD24SNGL(bool);
    }
    /// SD24 Channel 1 Control Register
    rw SD24CCTL1 @ 0x54: u16 = 0_0 {
        /// SD24 Grouping of Channels: 0:Off/1:On
        SD24CCTL1_SD24GRP: 0 = struct SD24CCTL1_SD24GRP(bool);
        /// SD24 Start Conversion
        SD24CCTL1_SD24SC: 1 = struct SD24CCTL1_SD24SC(bool);
        /// SD24 Channel x Interrupt Flag
        SD24CCTL1_SD24IFG: 2 = struct SD24CCTL1_SD24IFG(bool);
        /// SD24 Channel x Interrupt Enable
        SD24CCTL1_SD24IE: 3 = struct SD24CCTL1_SD24IE(bool);
        /// SD24 Channel x Data Format: 0:Unipolar/1:Bipolar
        SD24CCTL1_SD24DF: 4 = struct SD24CCTL1_SD24DF(bool);
        /// SD24 Channel x Overflow Interrupt Flag
        SD24CCTL1_SD24OVIFG: 5 = struct SD24CCTL1_SD24OVIFG(bool);
        /// SD24 Channel x Access LSB of ADC
        SD24CCTL1_SD24LSBACC: 6 = struct SD24CCTL1_SD24LSBACC(bool);
        /// SD24 Channel x Toggle LSB Output of ADC
        SD24CCTL1_SD24LSBTOG: 7 = struct SD24CCTL1_SD24LSBTOG(bool);
        /// SD24 Channel x OverSampling Ratio 0
        SD24CCTL1_SD24OSR0: 8 = struct SD24CCTL1_SD24OSR0(bool);
        /// SD24 Channel x OverSampling Ratio 1
        SD24CCTL1_SD24OSR1: 9 = struct SD24CCTL1_SD24OSR1(bool);
        /// SD24 Channel x Single Conversion On/Off
        SD24CCTL1_SD24SNGL: 10 = struct SD24CCTL1_SD24SNGL(bool);
    }
    /// SD24 Channel 2 Control Register
    rw SD24CCTL2 @ 0x56: u16 = 0_0 {
        /// SD24 Grouping of Channels: 0:Off/1:On
        SD24CCTL2_SD24GRP: 0 = struct SD24CCTL2_SD24GRP(bool);
        /// SD24 Start Conversion
        SD24CCTL2_SD24SC: 1 = struct SD24CCTL2_SD24SC(bool);
        /// SD24 Channel x Interrupt Flag
        SD24CCTL2_SD24IFG: 2 = struct SD24CCTL2_SD24IFG(bool);
        /// SD24 Channel x Interrupt Enable
        SD24CCTL2_SD24IE: 3 = struct SD24CCTL2_SD24IE(bool);
        /// SD24 Channel x Data Format: 0:Unipolar/1:Bipolar
        SD24CCTL2_SD24DF: 4 = struct SD24CCTL2_SD24DF(bool);
        /// SD24 Channel x Overflow Interrupt Flag
        SD24CCTL2_SD24OVIFG: 5 = struct SD24CCTL2_SD24OVIFG(bool);
        /// SD24 Channel x Access LSB of ADC
        SD24CCTL2_SD24LSBACC: 6 = struct SD24CCTL2_SD24LSBACC(bool);
        /// SD24 Channel x Toggle LSB Output of ADC
        SD24CCTL2_SD24LSBTOG: 7 = struct SD24CCTL2_SD24LSBTOG(bool);
        /// SD24 Channel x OverSampling Ratio 0
        SD24CCTL2_SD24OSR0: 8 = struct SD24CCTL2_SD24OSR0(bool);
        /// SD24 Channel x OverSampling Ratio 1
        SD24CCTL2_SD24OSR1: 9 = struct SD24CCTL2_SD24OSR1(bool);
        /// SD24 Channel x Single Conversion On/Off
        SD24CCTL2_SD24SNGL: 10 = struct SD24CCTL2_SD24SNGL(bool);
    }
    /// SD24 Channel 3 Control Register
    rw SD24CCTL3 @ 0x58: u16 = 0_0 {
        /// SD24 Grouping of Channels: 0:Off/1:On
        SD24CCTL3_SD24GRP: 0 = struct SD24CCTL3_SD24GRP(bool);
        /// SD24 Start Conversion
        SD24CCTL3_SD24SC: 1 = struct SD24CCTL3_SD24SC(bool);
        /// SD24 Channel x Interrupt Flag
        SD24CCTL3_SD24IFG: 2 = struct SD24CCTL3_SD24IFG(bool);
        /// SD24 Channel x Interrupt Enable
        SD24CCTL3_SD24IE: 3 = struct SD24CCTL3_SD24IE(bool);
        /// SD24 Channel x Data Format: 0:Unipolar/1:Bipolar
        SD24CCTL3_SD24DF: 4 = struct SD24CCTL3_SD24DF(bool);
        /// SD24 Channel x Overflow Interrupt Flag
        SD24CCTL3_SD24OVIFG: 5 = struct SD24CCTL3_SD24OVIFG(bool);
        /// SD24 Channel x Access LSB of ADC
        SD24CCTL3_SD24LSBACC: 6 = struct SD24CCTL3_SD24LSBACC(bool);
        /// SD24 Channel x Toggle LSB Output of ADC
        SD24CCTL3_SD24LSBTOG: 7 = struct SD24CCTL3_SD24LSBTOG(bool);
        /// SD24 Channel x OverSampling Ratio 0
        SD24CCTL3_SD24OSR0: 8 = struct SD24CCTL3_SD24OSR0(bool);
        /// SD24 Channel x OverSampling Ratio 1
        SD24CCTL3_SD24OSR1: 9 = struct SD24CCTL3_SD24OSR1(bool);
        /// SD24 Channel x Single Conversion On/Off
        SD24CCTL3_SD24SNGL: 10 = struct SD24CCTL3_SD24SNGL(bool);
    }
    /// SD24 Channel 0 Conversion Memory
    rw SD24MEM0 @ 0x60: u16 = 0_0 {
        /// SD24 Channel 0 Conversion Memory
        SD24MEM0: 0..15 = struct SD24MEM0Field(u16);
    }
    /// SD24 Channel 1 Conversion Memory
    rw SD24MEM1 @ 0x62: u16 = 0_0 {
        /// SD24 Channel 1 Conversion Memory
        SD24MEM1: 0..15 = struct SD24MEM1Field(u16);
    }
    /// SD24 Channel 2 Conversion Memory
    rw SD24MEM2 @ 0x64: u16 = 0_0 {
        /// SD24 Channel 2 Conversion Memory
        SD24MEM2: 0..15 = struct SD24MEM2Field(u16);
    }
    /// SD24 Channel 3 Conversion Memory
    rw SD24MEM3 @ 0x66: u16 = 0_0 {
        /// SD24 Channel 3 Conversion Memory
        SD24MEM3: 0..15 = struct SD24MEM3Field(u16);
    }
    /// SD24 Interrupt Vector Register
    rw SD24IV @ 0x140: u16 = 0_0 {
        /// SD24 Interrupt Vector Register
        SD24IV: 0..15 = struct SD24IVField(u16);
    }
}
