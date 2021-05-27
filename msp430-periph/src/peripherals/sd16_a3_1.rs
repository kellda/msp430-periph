//! SD16_A3

utils::periph! {
    /// SD16_A3
    SD16_A3;
    /// SD16 Input Control Register Channel 0
    rw INCTL0 @ 0x00: u8 = 0_0 {
        /// SD16 Input Channel select 0
        IN0INCH: 0..2 = enum IN0INCH {
            /// SD16 Input Channel select input
            INCH_0 = 0b000,
            /// SD16 Input Channel select input
            INCH_1 = 0b001,
            /// SD16 Input Channel select input
            INCH_2 = 0b010,
            /// SD16 Input Channel select input
            INCH_3 = 0b011,
            /// SD16 Input Channel select input
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        IN0GAIN0: 3 = struct IN0GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        IN0GAIN1: 4 = struct IN0GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        IN0GAIN2: 5 = struct IN0GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        IN0INTDLY: 6..7 = enum IN0INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD16 Input Control Register Channel 1
    rw INCTL1 @ 0x01: u8 = 0_0 {
        /// SD16 Input Channel select 0
        IN1INCH: 0..2 = enum IN1INCH {
            /// SD16 Input Channel select input
            INCH_0 = 0b000,
            /// SD16 Input Channel select input
            INCH_1 = 0b001,
            /// SD16 Input Channel select input
            INCH_2 = 0b010,
            /// SD16 Input Channel select input
            INCH_3 = 0b011,
            /// SD16 Input Channel select input
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        IN1GAIN0: 3 = struct IN1GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        IN1GAIN1: 4 = struct IN1GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        IN1GAIN2: 5 = struct IN1GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        IN1INTDLY: 6..7 = enum IN1INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD16 Input Control Register Channel 2
    rw INCTL2 @ 0x02: u8 = 0_0 {
        /// SD16 Input Channel select 0
        IN2INCH: 0..2 = enum IN2INCH {
            /// SD16 Input Channel select input
            INCH_0 = 0b000,
            /// SD16 Input Channel select input
            INCH_1 = 0b001,
            /// SD16 Input Channel select input
            INCH_2 = 0b010,
            /// SD16 Input Channel select input
            INCH_3 = 0b011,
            /// SD16 Input Channel select input
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        IN2GAIN0: 3 = struct IN2GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        IN2GAIN1: 4 = struct IN2GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        IN2GAIN2: 5 = struct IN2GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        IN2INTDLY: 6..7 = enum IN2INTDLY {
            /// SD16 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD16 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD16 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD16 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD16 Preload Register Channel 0
    rw PRE0 @ 0x08: u8 = 0_0 {
        /// SD16 Preload Register Channel 0
        PRE0: 0..7 = struct PRE0Field(u8);
    }
    /// SD16 Preload Register Channel 1
    rw PRE1 @ 0x09: u8 = 0_0 {
        /// SD16 Preload Register Channel 1
        PRE1: 0..7 = struct PRE1Field(u8);
    }
    /// SD16 Preload Register Channel 2
    rw PRE2 @ 0x0a: u8 = 0_0 {
        /// SD16 Preload Register Channel 2
        PRE2: 0..7 = struct PRE2Field(u8);
    }
    /// Sigma Delta ADC 16 Control Register
    rw CTL @ 0x50: u16 = 0_0 {
        /// SD16 Overflow Interupt Enable
        OVIE: 1 = struct OVIE(bool);
        /// SD16 Switch internal Reference on
        REFON: 2 = struct REFON(bool);
        /// SD16 Switch Vmid Buffer on
        VMIDON: 3 = struct VMIDON(bool);
        /// SD16 Clock Source Select 0
        SSEL: 4..5 = enum SSEL {
            /// SD16 Clock Source Select MCLK
            SSEL_0 = 0b00,
            /// SD16 Clock Source Select SMCLK
            SSEL_1 = 0b01,
            /// SD16 Clock Source Select ACLK
            SSEL_2 = 0b10,
            /// SD16 Clock Source Select TACLK
            SSEL_3 = 0b11,
        }
        /// SD16 Clock Divider Select 0
        DIV: 6..7 = enum DIV {
            /// SD16 Clock Divider Select /1
            DIV_0 = 0b00,
            /// SD16 Clock Divider Select /2
            DIV_1 = 0b01,
            /// SD16 Clock Divider Select /4
            DIV_2 = 0b10,
            /// SD16 Clock Divider Select /8
            DIV_3 = 0b11,
        }
        /// SD16 Low Power Mode Enable
        LP: 8 = struct LP(bool);
        /// SD16 2.Clock Divider Select 0
        XDIV: 9..11 = enum XDIV {
            /// SD16 2.Clock Divider Select /1
            XDIV_0 = 0b000,
            /// SD16 2.Clock Divider Select /3
            XDIV_1 = 0b001,
            /// SD16 2.Clock Divider Select /16
            XDIV_2 = 0b010,
            /// SD16 2.Clock Divider Select /48
            XDIV_3 = 0b011,
        }
    }
    /// SD16 Channel 0 Control Register
    rw CCTL0 @ 0x52: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        C0GRP: 0 = struct C0GRP(bool);
        /// SD16 Start Conversion
        C0SC: 1 = struct C0SC(bool);
        /// SD16 Channel x Interrupt Flag
        C0IFG: 2 = struct C0IFG(bool);
        /// SD16 Channel x Interrupt Enable
        C0IE: 3 = struct C0IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        C0DF: 4 = struct C0DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        C0OVIFG: 5 = struct C0OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        C0LSBACC: 6 = struct C0LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        C0LSBTOG: 7 = struct C0LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        C0OSR0: 8 = struct C0OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        C0OSR1: 9 = struct C0OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        C0SNGL: 10 = struct C0SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        C0XOSR: 11 = struct C0XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        C0UNI: 12 = struct C0UNI(bool);
    }
    /// SD16 Channel 1 Control Register
    rw CCTL1 @ 0x54: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        C1GRP: 0 = struct C1GRP(bool);
        /// SD16 Start Conversion
        C1SC: 1 = struct C1SC(bool);
        /// SD16 Channel x Interrupt Flag
        C1IFG: 2 = struct C1IFG(bool);
        /// SD16 Channel x Interrupt Enable
        C1IE: 3 = struct C1IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        C1DF: 4 = struct C1DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        C1OVIFG: 5 = struct C1OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        C1LSBACC: 6 = struct C1LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        C1LSBTOG: 7 = struct C1LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        C1OSR0: 8 = struct C1OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        C1OSR1: 9 = struct C1OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        C1SNGL: 10 = struct C1SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        C1XOSR: 11 = struct C1XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        C1UNI: 12 = struct C1UNI(bool);
    }
    /// SD16 Channel 2 Control Register
    rw CCTL2 @ 0x56: u16 = 0_0 {
        /// SD16 Grouping of Channels: 0:Off/1:On
        C2GRP: 0 = struct C2GRP(bool);
        /// SD16 Start Conversion
        C2SC: 1 = struct C2SC(bool);
        /// SD16 Channel x Interrupt Flag
        C2IFG: 2 = struct C2IFG(bool);
        /// SD16 Channel x Interrupt Enable
        C2IE: 3 = struct C2IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        C2DF: 4 = struct C2DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        C2OVIFG: 5 = struct C2OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        C2LSBACC: 6 = struct C2LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        C2LSBTOG: 7 = struct C2LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        C2OSR0: 8 = struct C2OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        C2OSR1: 9 = struct C2OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        C2SNGL: 10 = struct C2SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        C2XOSR: 11 = struct C2XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        C2UNI: 12 = struct C2UNI(bool);
    }
    /// SD16 Interrupt Vector Register
    rw IV @ 0x60: u16 = 0_0 {
        /// SD16 Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
    /// SD16 Channel 0 Conversion Memory
    rw MEM0 @ 0x62: u16 = 0_0 {
        /// SD16 Channel 0 Conversion Memory
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// SD16 Channel 1 Conversion Memory
    rw MEM1 @ 0x64: u16 = 0_0 {
        /// SD16 Channel 1 Conversion Memory
        MEM1: 0..15 = struct MEM1Field(u16);
    }
    /// SD16 Channel 2 Conversion Memory
    rw MEM2 @ 0x66: u16 = 0_0 {
        /// SD16 Channel 2 Conversion Memory
        MEM2: 0..15 = struct MEM2Field(u16);
    }
}
