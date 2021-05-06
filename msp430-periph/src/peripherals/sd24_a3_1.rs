//! SD24_A3

utils::periph! {
    /// SD24_A3
    SD24_A3;
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
        /// SD24 Interrupt Delay after 1.Conversion 0
        SD24INCTL0_SD24INTDLY: 6..7 = enum SD24INCTL0_SD24INTDLY {
            /// SD24 Interrupt Delay: Int. after 4.Conversion
            SD24INTDLY_0 = 0b00,
            /// SD24 Interrupt Delay: Int. after 3.Conversion
            SD24INTDLY_1 = 0b01,
            /// SD24 Interrupt Delay: Int. after 2.Conversion
            SD24INTDLY_2 = 0b10,
            /// SD24 Interrupt Delay: Int. after 1.Conversion
            SD24INTDLY_3 = 0b11,
        }
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
        /// SD24 Interrupt Delay after 1.Conversion 0
        SD24INCTL1_SD24INTDLY: 6..7 = enum SD24INCTL1_SD24INTDLY {
            /// SD24 Interrupt Delay: Int. after 4.Conversion
            SD24INTDLY_0 = 0b00,
            /// SD24 Interrupt Delay: Int. after 3.Conversion
            SD24INTDLY_1 = 0b01,
            /// SD24 Interrupt Delay: Int. after 2.Conversion
            SD24INTDLY_2 = 0b10,
            /// SD24 Interrupt Delay: Int. after 1.Conversion
            SD24INTDLY_3 = 0b11,
        }
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
        /// SD24 Interrupt Delay after 1.Conversion 0
        SD24INCTL2_SD24INTDLY: 6..7 = enum SD24INCTL2_SD24INTDLY {
            /// SD24 Interrupt Delay: Int. after 4.Conversion
            SD24INTDLY_0 = 0b00,
            /// SD24 Interrupt Delay: Int. after 3.Conversion
            SD24INTDLY_1 = 0b01,
            /// SD24 Interrupt Delay: Int. after 2.Conversion
            SD24INTDLY_2 = 0b10,
            /// SD24 Interrupt Delay: Int. after 1.Conversion
            SD24INTDLY_3 = 0b11,
        }
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
    /// Sigma Delta ADC 24 Control Register
    rw SD24CTL @ 0x50: u16 = 0_0 {
        /// SD24 Overflow Interupt Enable
        SD24OVIE: 1 = struct SD24OVIE(bool);
        /// SD24 Switch internal Reference on
        SD24REFON: 2 = struct SD24REFON(bool);
        /// SD24 Switch Vmid Buffer on
        SD24VMIDON: 3 = struct SD24VMIDON(bool);
        /// SD24 Clock Source Select 0
        SD24SSEL: 4..5 = enum SD24SSEL {
            /// SD24 Clock Source Select MCLK
            SD24SSEL_0 = 0b00,
            /// SD24 Clock Source Select SMCLK
            SD24SSEL_1 = 0b01,
            /// SD24 Clock Source Select ACLK
            SD24SSEL_2 = 0b10,
            /// SD24 Clock Source Select TACLK
            SD24SSEL_3 = 0b11,
        }
        /// SD24 Clock Divider Select 0
        SD24DIV: 6..7 = enum SD24DIV {
            /// SD24 Clock Divider Select /1
            SD24DIV_0 = 0b00,
            /// SD24 Clock Divider Select /2
            SD24DIV_1 = 0b01,
            /// SD24 Clock Divider Select /4
            SD24DIV_2 = 0b10,
            /// SD24 Clock Divider Select /8
            SD24DIV_3 = 0b11,
        }
        /// SD24 Low Power Mode Enable
        SD24LP: 8 = struct SD24LP(bool);
        /// SD24 2.Clock Divider Select 0
        SD24XDIV: 9..11 = enum SD24XDIV {
            /// SD24 2.Clock Divider Select /1
            SD24XDIV_0 = 0b000,
            /// SD24 2.Clock Divider Select /3
            SD24XDIV_1 = 0b001,
            /// SD24 2.Clock Divider Select /16
            SD24XDIV_2 = 0b010,
            /// SD24 2.Clock Divider Select /48
            SD24XDIV_3 = 0b011,
        }
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
        /// SD24 Channel x Extended OverSampling Ratio
        SD24CCTL0_SD24XOSR: 11 = struct SD24CCTL0_SD24XOSR(bool);
        /// SD24 Channel x Bipolar(0) / Unipolar(1) Mode
        SD24CCTL0_SD24UNI: 12 = struct SD24CCTL0_SD24UNI(bool);
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
        /// SD24 Channel x Extended OverSampling Ratio
        SD24CCTL1_SD24XOSR: 11 = struct SD24CCTL1_SD24XOSR(bool);
        /// SD24 Channel x Bipolar(0) / Unipolar(1) Mode
        SD24CCTL1_SD24UNI: 12 = struct SD24CCTL1_SD24UNI(bool);
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
        /// SD24 Channel x Extended OverSampling Ratio
        SD24CCTL2_SD24XOSR: 11 = struct SD24CCTL2_SD24XOSR(bool);
        /// SD24 Channel x Bipolar(0) / Unipolar(1) Mode
        SD24CCTL2_SD24UNI: 12 = struct SD24CCTL2_SD24UNI(bool);
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
    /// SD24 Interrupt Vector Register
    rw SD24IV @ 0xfe: u16 = 0_0 {
        /// SD24 Interrupt Vector Register
        SD24IV: 0..15 = struct SD24IVField(u16);
    }
}
