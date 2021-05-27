//! SD24_A3

utils::periph! {
    /// SD24_A3
    SD24_A3;
    /// SD24 Input Control Register Channel 0
    rw INCTL0 @ 0x00: u8 = 0_0 {
        /// SD24 Input Channel select 0
        IN0INCH: 0..2 = enum IN0INCH {
            /// SD24 Input Channel select input
            INCH_0 = 0b000,
            /// SD24 Input Channel select input
            INCH_1 = 0b001,
            /// SD24 Input Channel select input
            INCH_2 = 0b010,
            /// SD24 Input Channel select input
            INCH_3 = 0b011,
            /// SD24 Input Channel select input
            INCH_4 = 0b100,
            /// SD24 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD24 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD24 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD24 Input Pre-Amplifier Gain Select 0
        IN0GAIN0: 3 = struct IN0GAIN0(bool);
        /// SD24 Input Pre-Amplifier Gain Select 1
        IN0GAIN1: 4 = struct IN0GAIN1(bool);
        /// SD24 Input Pre-Amplifier Gain Select 2
        IN0GAIN2: 5 = struct IN0GAIN2(bool);
        /// SD24 Interrupt Delay after 1.Conversion
        IN0INTDLY: 6 = struct IN0INTDLY(bool);
    }
    /// SD24 Input Control Register Channel 1
    rw INCTL1 @ 0x01: u8 = 0_0 {
        /// SD24 Input Channel select 0
        IN1INCH: 0..2 = enum IN1INCH {
            /// SD24 Input Channel select input
            INCH_0 = 0b000,
            /// SD24 Input Channel select input
            INCH_1 = 0b001,
            /// SD24 Input Channel select input
            INCH_2 = 0b010,
            /// SD24 Input Channel select input
            INCH_3 = 0b011,
            /// SD24 Input Channel select input
            INCH_4 = 0b100,
            /// SD24 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD24 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD24 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD24 Input Pre-Amplifier Gain Select 0
        IN1GAIN0: 3 = struct IN1GAIN0(bool);
        /// SD24 Input Pre-Amplifier Gain Select 1
        IN1GAIN1: 4 = struct IN1GAIN1(bool);
        /// SD24 Input Pre-Amplifier Gain Select 2
        IN1GAIN2: 5 = struct IN1GAIN2(bool);
        /// SD24 Interrupt Delay after 1.Conversion
        IN1INTDLY: 6 = struct IN1INTDLY(bool);
    }
    /// SD24 Input Control Register Channel 2
    rw INCTL2 @ 0x02: u8 = 0_0 {
        /// SD24 Input Channel select 0
        IN2INCH: 0..2 = enum IN2INCH {
            /// SD24 Input Channel select input
            INCH_0 = 0b000,
            /// SD24 Input Channel select input
            INCH_1 = 0b001,
            /// SD24 Input Channel select input
            INCH_2 = 0b010,
            /// SD24 Input Channel select input
            INCH_3 = 0b011,
            /// SD24 Input Channel select input
            INCH_4 = 0b100,
            /// SD24 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD24 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD24 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD24 Input Pre-Amplifier Gain Select 0
        IN2GAIN0: 3 = struct IN2GAIN0(bool);
        /// SD24 Input Pre-Amplifier Gain Select 1
        IN2GAIN1: 4 = struct IN2GAIN1(bool);
        /// SD24 Input Pre-Amplifier Gain Select 2
        IN2GAIN2: 5 = struct IN2GAIN2(bool);
        /// SD24 Interrupt Delay after 1.Conversion
        IN2INTDLY: 6 = struct IN2INTDLY(bool);
    }
    /// SD24 Preload Register Channel 0
    rw PRE0 @ 0x08: u8 = 0_0 {
        /// SD24 Preload Register Channel 0
        PRE0: 0..7 = struct PRE0Field(u8);
    }
    /// SD24 Preload Register Channel 1
    rw PRE1 @ 0x09: u8 = 0_0 {
        /// SD24 Preload Register Channel 1
        PRE1: 0..7 = struct PRE1Field(u8);
    }
    /// SD24 Preload Register Channel 2
    rw PRE2 @ 0x0a: u8 = 0_0 {
        /// SD24 Preload Register Channel 2
        PRE2: 0..7 = struct PRE2Field(u8);
    }
    /// SD24 Trim Register 1
    rw TRIM @ 0x0f: u8 = 0_0 {
        /// SD24 Trimming of bias current Bit: 0
        IBTRIM0: 0 = struct IBTRIM0(bool);
        /// SD24 Trimming of bias current Bit: 1
        IBTRIM1: 1 = struct IBTRIM1(bool);
        /// SD24 Trimming of bias current Bit: 2
        IBTRIM2: 2 = struct IBTRIM2(bool);
        /// SD24 Trimming of reference voltage buffer start up delay Bit: 0
        REFDLYTRIM0: 3 = struct REFDLYTRIM0(bool);
        /// SD24 Trimming of reference voltage buffer start up delay Bit: 1
        REFDLYTRIM1: 4 = struct REFDLYTRIM1(bool);
    }
    /// Sigma Delta ADC 24 Control Register
    rw CTL @ 0x50: u16 = 0_0 {
        /// SD24 Overflow Interupt Enable
        OVIE: 1 = struct OVIE(bool);
        /// SD24 Reference Select
        REFS: 2 = struct REFS(bool);
    }
    /// SD24 Channel 0 Control Register
    rw CCTL0 @ 0x52: u16 = 0_0 {
        /// SD24 Grouping of Channels: 0:Off/1:On
        C0GRP: 0 = struct C0GRP(bool);
        /// SD24 Start Conversion
        C0SC: 1 = struct C0SC(bool);
        /// SD24 Channel x Interrupt Flag
        C0IFG: 2 = struct C0IFG(bool);
        /// SD24 Channel x Interrupt Enable
        C0IE: 3 = struct C0IE(bool);
        /// SD24 Channel x Data Format: 0:Unipolar/1:Bipolar
        C0DF: 4 = struct C0DF(bool);
        /// SD24 Channel x Overflow Interrupt Flag
        C0OVIFG: 5 = struct C0OVIFG(bool);
        /// SD24 Channel x Access LSB of ADC
        C0LSBACC: 6 = struct C0LSBACC(bool);
        /// SD24 Channel x Toggle LSB Output of ADC
        C0LSBTOG: 7 = struct C0LSBTOG(bool);
        /// SD24 Channel x OverSampling Ratio 0
        C0OSR0: 8 = struct C0OSR0(bool);
        /// SD24 Channel x OverSampling Ratio 1
        C0OSR1: 9 = struct C0OSR1(bool);
        /// SD24 Channel x Single Conversion On/Off
        C0SNGL: 10 = struct C0SNGL(bool);
    }
    /// SD24 Channel 1 Control Register
    rw CCTL1 @ 0x54: u16 = 0_0 {
        /// SD24 Grouping of Channels: 0:Off/1:On
        C1GRP: 0 = struct C1GRP(bool);
        /// SD24 Start Conversion
        C1SC: 1 = struct C1SC(bool);
        /// SD24 Channel x Interrupt Flag
        C1IFG: 2 = struct C1IFG(bool);
        /// SD24 Channel x Interrupt Enable
        C1IE: 3 = struct C1IE(bool);
        /// SD24 Channel x Data Format: 0:Unipolar/1:Bipolar
        C1DF: 4 = struct C1DF(bool);
        /// SD24 Channel x Overflow Interrupt Flag
        C1OVIFG: 5 = struct C1OVIFG(bool);
        /// SD24 Channel x Access LSB of ADC
        C1LSBACC: 6 = struct C1LSBACC(bool);
        /// SD24 Channel x Toggle LSB Output of ADC
        C1LSBTOG: 7 = struct C1LSBTOG(bool);
        /// SD24 Channel x OverSampling Ratio 0
        C1OSR0: 8 = struct C1OSR0(bool);
        /// SD24 Channel x OverSampling Ratio 1
        C1OSR1: 9 = struct C1OSR1(bool);
        /// SD24 Channel x Single Conversion On/Off
        C1SNGL: 10 = struct C1SNGL(bool);
    }
    /// SD24 Channel 2 Control Register
    rw CCTL2 @ 0x56: u16 = 0_0 {
        /// SD24 Grouping of Channels: 0:Off/1:On
        C2GRP: 0 = struct C2GRP(bool);
        /// SD24 Start Conversion
        C2SC: 1 = struct C2SC(bool);
        /// SD24 Channel x Interrupt Flag
        C2IFG: 2 = struct C2IFG(bool);
        /// SD24 Channel x Interrupt Enable
        C2IE: 3 = struct C2IE(bool);
        /// SD24 Channel x Data Format: 0:Unipolar/1:Bipolar
        C2DF: 4 = struct C2DF(bool);
        /// SD24 Channel x Overflow Interrupt Flag
        C2OVIFG: 5 = struct C2OVIFG(bool);
        /// SD24 Channel x Access LSB of ADC
        C2LSBACC: 6 = struct C2LSBACC(bool);
        /// SD24 Channel x Toggle LSB Output of ADC
        C2LSBTOG: 7 = struct C2LSBTOG(bool);
        /// SD24 Channel x OverSampling Ratio 0
        C2OSR0: 8 = struct C2OSR0(bool);
        /// SD24 Channel x OverSampling Ratio 1
        C2OSR1: 9 = struct C2OSR1(bool);
        /// SD24 Channel x Single Conversion On/Off
        C2SNGL: 10 = struct C2SNGL(bool);
    }
    /// SD24 Channel 0 Conversion Memory
    rw MEM0 @ 0x60: u16 = 0_0 {
        /// SD24 Channel 0 Conversion Memory
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// SD24 Channel 1 Conversion Memory
    rw MEM1 @ 0x62: u16 = 0_0 {
        /// SD24 Channel 1 Conversion Memory
        MEM1: 0..15 = struct MEM1Field(u16);
    }
    /// SD24 Channel 2 Conversion Memory
    rw MEM2 @ 0x64: u16 = 0_0 {
        /// SD24 Channel 2 Conversion Memory
        MEM2: 0..15 = struct MEM2Field(u16);
    }
    /// SD24 Interrupt Vector Register
    rw IV @ 0x140: u16 = 0_0 {
        /// SD24 Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
}
