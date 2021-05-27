//! SD24_A1

utils::periph! {
    /// SD24_A1
    SD24_A1;
    /// SD24 Input Control Register Channel 0
    rw INCTL0 @ 0x00: u8 = 0_0 {
        /// SD24 Input Channel select 0
        INCH: 0..2 = enum INCH {
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
        GAIN0: 3 = struct GAIN0(bool);
        /// SD24 Input Pre-Amplifier Gain Select 1
        GAIN1: 4 = struct GAIN1(bool);
        /// SD24 Input Pre-Amplifier Gain Select 2
        GAIN2: 5 = struct GAIN2(bool);
        /// SD24 Interrupt Delay after 1.Conversion 0
        INTDLY: 6..7 = enum INTDLY {
            /// SD24 Interrupt Delay: Int. after 4.Conversion
            INTDLY_0 = 0b00,
            /// SD24 Interrupt Delay: Int. after 3.Conversion
            INTDLY_1 = 0b01,
            /// SD24 Interrupt Delay: Int. after 2.Conversion
            INTDLY_2 = 0b10,
            /// SD24 Interrupt Delay: Int. after 1.Conversion
            INTDLY_3 = 0b11,
        }
    }
    /// SD24 Preload Register Channel 0
    rw PRE0 @ 0x08: u8 = 0_0 {
        /// SD24 Preload Register Channel 0
        PRE0: 0..7 = struct PRE0Field(u8);
    }
    /// Sigma Delta ADC 24 Control Register
    rw CTL @ 0x50: u16 = 0_0 {
        /// SD24 Overflow Interupt Enable
        OVIE: 1 = struct OVIE(bool);
        /// SD24 Switch internal Reference on
        REFON: 2 = struct REFON(bool);
        /// SD24 Switch Vmid Buffer on
        VMIDON: 3 = struct VMIDON(bool);
        /// SD24 Clock Source Select 0
        SSEL: 4..5 = enum SSEL {
            /// SD24 Clock Source Select MCLK
            SSEL_0 = 0b00,
            /// SD24 Clock Source Select SMCLK
            SSEL_1 = 0b01,
            /// SD24 Clock Source Select ACLK
            SSEL_2 = 0b10,
            /// SD24 Clock Source Select TACLK
            SSEL_3 = 0b11,
        }
        /// SD24 Clock Divider Select 0
        DIV: 6..7 = enum DIV {
            /// SD24 Clock Divider Select /1
            DIV_0 = 0b00,
            /// SD24 Clock Divider Select /2
            DIV_1 = 0b01,
            /// SD24 Clock Divider Select /4
            DIV_2 = 0b10,
            /// SD24 Clock Divider Select /8
            DIV_3 = 0b11,
        }
        /// SD24 Low Power Mode Enable
        LP: 8 = struct LP(bool);
        /// SD24 2.Clock Divider Select 0
        XDIV: 9..11 = enum XDIV {
            /// SD24 2.Clock Divider Select /1
            XDIV_0 = 0b000,
            /// SD24 2.Clock Divider Select /3
            XDIV_1 = 0b001,
            /// SD24 2.Clock Divider Select /16
            XDIV_2 = 0b010,
            /// SD24 2.Clock Divider Select /48
            XDIV_3 = 0b011,
        }
    }
    /// SD24 Channel 0 Control Register
    rw CCTL0 @ 0x52: u16 = 0_0 {
        /// SD24 Grouping of Channels: 0:Off/1:On
        GRP: 0 = struct GRP(bool);
        /// SD24 Start Conversion
        SC: 1 = struct SC(bool);
        /// SD24 Channel x Interrupt Flag
        IFG: 2 = struct IFG(bool);
        /// SD24 Channel x Interrupt Enable
        IE: 3 = struct IE(bool);
        /// SD24 Channel x Data Format: 0:Unipolar/1:Bipolar
        DF: 4 = struct DF(bool);
        /// SD24 Channel x Overflow Interrupt Flag
        OVIFG: 5 = struct OVIFG(bool);
        /// SD24 Channel x Access LSB of ADC
        LSBACC: 6 = struct LSBACC(bool);
        /// SD24 Channel x Toggle LSB Output of ADC
        LSBTOG: 7 = struct LSBTOG(bool);
        /// SD24 Channel x OverSampling Ratio 0
        OSR0: 8 = struct OSR0(bool);
        /// SD24 Channel x OverSampling Ratio 1
        OSR1: 9 = struct OSR1(bool);
        /// SD24 Channel x Single Conversion On/Off
        SNGL: 10 = struct SNGL(bool);
        /// SD24 Channel x Extended OverSampling Ratio
        XOSR: 11 = struct XOSR(bool);
        /// SD24 Channel x Bipolar(0) / Unipolar(1) Mode
        UNI: 12 = struct UNI(bool);
    }
    /// SD24 Channel 0 Conversion Memory
    rw MEM0 @ 0x60: u16 = 0_0 {
        /// SD24 Channel 0 Conversion Memory
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// SD24 Interrupt Vector Register
    rw IV @ 0xfe: u16 = 0_0 {
        /// SD24 Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
}
