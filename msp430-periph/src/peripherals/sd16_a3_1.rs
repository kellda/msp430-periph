//! SD16_A3

utils::periph! {
    /// SD16_A3
    SD16_A3;
    /// SD16 Input Control Register Channel 0
    rw SD16INCTL0 @ 0x00: u8 = 0_0 {
        /// SD16 Input Channel select 0
        SD16INCTL0_SD16INCH: 0..2 = enum SD16INCTL0_SD16INCH {
            /// SD16 Input Channel select input
            SD16INCH_0 = 0b000,
            /// SD16 Input Channel select input
            SD16INCH_1 = 0b001,
            /// SD16 Input Channel select input
            SD16INCH_2 = 0b010,
            /// SD16 Input Channel select input
            SD16INCH_3 = 0b011,
            /// SD16 Input Channel select input
            SD16INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            SD16INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            SD16INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            SD16INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        SD16INCTL0_SD16GAIN0: 3 = struct SD16INCTL0_SD16GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        SD16INCTL0_SD16GAIN1: 4 = struct SD16INCTL0_SD16GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        SD16INCTL0_SD16GAIN2: 5 = struct SD16INCTL0_SD16GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        SD16INCTL0_SD16INTDLY: 6..7 = enum SD16INCTL0_SD16INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            SD16INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            SD16INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            SD16INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            SD16INTDLY_3 = 0b11,
        }
    }
    /// SD16 Input Control Register Channel 1
    rw SD16INCTL1 @ 0x01: u8 = 0_0 {
        /// SD16 Input Channel select 0
        SD16INCTL1_SD16INCH: 0..2 = enum SD16INCTL1_SD16INCH {
            /// SD16 Input Channel select input
            SD16INCH_0 = 0b000,
            /// SD16 Input Channel select input
            SD16INCH_1 = 0b001,
            /// SD16 Input Channel select input
            SD16INCH_2 = 0b010,
            /// SD16 Input Channel select input
            SD16INCH_3 = 0b011,
            /// SD16 Input Channel select input
            SD16INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            SD16INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            SD16INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            SD16INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        SD16INCTL1_SD16GAIN0: 3 = struct SD16INCTL1_SD16GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        SD16INCTL1_SD16GAIN1: 4 = struct SD16INCTL1_SD16GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        SD16INCTL1_SD16GAIN2: 5 = struct SD16INCTL1_SD16GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        SD16INCTL1_SD16INTDLY: 6..7 = enum SD16INCTL1_SD16INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            SD16INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            SD16INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            SD16INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            SD16INTDLY_3 = 0b11,
        }
    }
    /// SD16 Input Control Register Channel 2
    rw SD16INCTL2 @ 0x02: u8 = 0_0 {
        /// SD16 Input Channel select 0
        SD16INCTL2_SD16INCH: 0..2 = enum SD16INCTL2_SD16INCH {
            /// SD16 Input Channel select input
            SD16INCH_0 = 0b000,
            /// SD16 Input Channel select input
            SD16INCH_1 = 0b001,
            /// SD16 Input Channel select input
            SD16INCH_2 = 0b010,
            /// SD16 Input Channel select input
            SD16INCH_3 = 0b011,
            /// SD16 Input Channel select input
            SD16INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            SD16INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            SD16INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            SD16INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        SD16INCTL2_SD16GAIN0: 3 = struct SD16INCTL2_SD16GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        SD16INCTL2_SD16GAIN1: 4 = struct SD16INCTL2_SD16GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        SD16INCTL2_SD16GAIN2: 5 = struct SD16INCTL2_SD16GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        SD16INCTL2_SD16INTDLY: 6..7 = enum SD16INCTL2_SD16INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            SD16INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            SD16INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            SD16INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            SD16INTDLY_3 = 0b11,
        }
    }
    /// SD16 Preload Register Channel 0
    rw SD16PRE0 @ 0x08: u8 = 0_0 {
        /// SD16 Preload Register Channel 0
        SD16PRE0: 0..7 = struct SD16PRE0Field(u8);
    }
    /// SD16 Preload Register Channel 1
    rw SD16PRE1 @ 0x09: u8 = 0_0 {
        /// SD16 Preload Register Channel 1
        SD16PRE1: 0..7 = struct SD16PRE1Field(u8);
    }
    /// SD16 Preload Register Channel 2
    rw SD16PRE2 @ 0x0a: u8 = 0_0 {
        /// SD16 Preload Register Channel 2
        SD16PRE2: 0..7 = struct SD16PRE2Field(u8);
    }
    /// Sigma Delta ADC 16 Control Register
    rw SD16CTL @ 0x50: u16 = 0_0 {
        /// SD16 Overflow Interupt Enable
        SD16OVIE: 1 = struct SD16OVIE(bool);
        /// SD16 Switch internal Reference on
        SD16REFON: 2 = struct SD16REFON(bool);
        /// SD16 Switch Vmid Buffer on
        SD16VMIDON: 3 = struct SD16VMIDON(bool);
        /// SD16 Clock Source Select 0
        SD16SSEL: 4..5 = enum SD16SSEL {
            /// SD16 Clock Source Select MCLK
            SD16SSEL_0 = 0b00,
            /// SD16 Clock Source Select SMCLK
            SD16SSEL_1 = 0b01,
            /// SD16 Clock Source Select ACLK
            SD16SSEL_2 = 0b10,
            /// SD16 Clock Source Select TACLK
            SD16SSEL_3 = 0b11,
        }
        /// SD16 Clock Divider Select 0
        SD16DIV: 6..7 = enum SD16DIV {
            /// SD16 Clock Divider Select /1
            SD16DIV_0 = 0b00,
            /// SD16 Clock Divider Select /2
            SD16DIV_1 = 0b01,
            /// SD16 Clock Divider Select /4
            SD16DIV_2 = 0b10,
            /// SD16 Clock Divider Select /8
            SD16DIV_3 = 0b11,
        }
        /// SD16 Low Power Mode Enable
        SD16LP: 8 = struct SD16LP(bool);
        /// SD16 2.Clock Divider Select 0
        SD16XDIV: 9..11 = enum SD16XDIV {
            /// SD16 2.Clock Divider Select /1
            SD16XDIV_0 = 0b000,
            /// SD16 2.Clock Divider Select /3
            SD16XDIV_1 = 0b001,
            /// SD16 2.Clock Divider Select /16
            SD16XDIV_2 = 0b010,
            /// SD16 2.Clock Divider Select /48
            SD16XDIV_3 = 0b011,
        }
    }
    /// SD16 Channel 0 Control Register
    rw SD16CCTL0 @ 0x52: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        SD16CCTL0_SD16GRP: 0 = struct SD16CCTL0_SD16GRP(bool);
        /// SD16 Start Conversion
        SD16CCTL0_SD16SC: 1 = struct SD16CCTL0_SD16SC(bool);
        /// SD16 Channel x Interrupt Flag
        SD16CCTL0_SD16IFG: 2 = struct SD16CCTL0_SD16IFG(bool);
        /// SD16 Channel x Interrupt Enable
        SD16CCTL0_SD16IE: 3 = struct SD16CCTL0_SD16IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        SD16CCTL0_SD16DF: 4 = struct SD16CCTL0_SD16DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        SD16CCTL0_SD16OVIFG: 5 = struct SD16CCTL0_SD16OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        SD16CCTL0_SD16LSBACC: 6 = struct SD16CCTL0_SD16LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        SD16CCTL0_SD16LSBTOG: 7 = struct SD16CCTL0_SD16LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        SD16CCTL0_SD16OSR0: 8 = struct SD16CCTL0_SD16OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        SD16CCTL0_SD16OSR1: 9 = struct SD16CCTL0_SD16OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        SD16CCTL0_SD16SNGL: 10 = struct SD16CCTL0_SD16SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        SD16CCTL0_SD16XOSR: 11 = struct SD16CCTL0_SD16XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        SD16CCTL0_SD16UNI: 12 = struct SD16CCTL0_SD16UNI(bool);
    }
    /// SD16 Channel 1 Control Register
    rw SD16CCTL1 @ 0x54: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        SD16CCTL1_SD16GRP: 0 = struct SD16CCTL1_SD16GRP(bool);
        /// SD16 Start Conversion
        SD16CCTL1_SD16SC: 1 = struct SD16CCTL1_SD16SC(bool);
        /// SD16 Channel x Interrupt Flag
        SD16CCTL1_SD16IFG: 2 = struct SD16CCTL1_SD16IFG(bool);
        /// SD16 Channel x Interrupt Enable
        SD16CCTL1_SD16IE: 3 = struct SD16CCTL1_SD16IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        SD16CCTL1_SD16DF: 4 = struct SD16CCTL1_SD16DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        SD16CCTL1_SD16OVIFG: 5 = struct SD16CCTL1_SD16OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        SD16CCTL1_SD16LSBACC: 6 = struct SD16CCTL1_SD16LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        SD16CCTL1_SD16LSBTOG: 7 = struct SD16CCTL1_SD16LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        SD16CCTL1_SD16OSR0: 8 = struct SD16CCTL1_SD16OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        SD16CCTL1_SD16OSR1: 9 = struct SD16CCTL1_SD16OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        SD16CCTL1_SD16SNGL: 10 = struct SD16CCTL1_SD16SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        SD16CCTL1_SD16XOSR: 11 = struct SD16CCTL1_SD16XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        SD16CCTL1_SD16UNI: 12 = struct SD16CCTL1_SD16UNI(bool);
    }
    /// SD16 Channel 2 Control Register
    rw SD16CCTL2 @ 0x56: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        SD16CCTL2_SD16GRP: 0 = struct SD16CCTL2_SD16GRP(bool);
        /// SD16 Start Conversion
        SD16CCTL2_SD16SC: 1 = struct SD16CCTL2_SD16SC(bool);
        /// SD16 Channel x Interrupt Flag
        SD16CCTL2_SD16IFG: 2 = struct SD16CCTL2_SD16IFG(bool);
        /// SD16 Channel x Interrupt Enable
        SD16CCTL2_SD16IE: 3 = struct SD16CCTL2_SD16IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        SD16CCTL2_SD16DF: 4 = struct SD16CCTL2_SD16DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        SD16CCTL2_SD16OVIFG: 5 = struct SD16CCTL2_SD16OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        SD16CCTL2_SD16LSBACC: 6 = struct SD16CCTL2_SD16LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        SD16CCTL2_SD16LSBTOG: 7 = struct SD16CCTL2_SD16LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        SD16CCTL2_SD16OSR0: 8 = struct SD16CCTL2_SD16OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        SD16CCTL2_SD16OSR1: 9 = struct SD16CCTL2_SD16OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        SD16CCTL2_SD16SNGL: 10 = struct SD16CCTL2_SD16SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        SD16CCTL2_SD16XOSR: 11 = struct SD16CCTL2_SD16XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        SD16CCTL2_SD16UNI: 12 = struct SD16CCTL2_SD16UNI(bool);
    }
    /// SD16 Interrupt Vector Register
    rw SD16IV @ 0x60: u16 = 0_0 {
        /// SD16 Interrupt Vector Register
        SD16IV: 0..15 = struct SD16IVField(u16);
    }
    /// SD16 Channel 0 Conversion Memory
    rw SD16MEM0 @ 0x62: u16 = 0_0 {
        /// SD16 Channel 0 Conversion Memory
        SD16MEM0: 0..15 = struct SD16MEM0Field(u16);
    }
    /// SD16 Channel 1 Conversion Memory
    rw SD16MEM1 @ 0x64: u16 = 0_0 {
        /// SD16 Channel 1 Conversion Memory
        SD16MEM1: 0..15 = struct SD16MEM1Field(u16);
    }
    /// SD16 Channel 2 Conversion Memory
    rw SD16MEM2 @ 0x66: u16 = 0_0 {
        /// SD16 Channel 2 Conversion Memory
        SD16MEM2: 0..15 = struct SD16MEM2Field(u16);
    }
}
