//! SD16_A1

utils::periph! {
    /// SD16_A1
    SD16_A1;
    /// SD16 Input Control Register Channel 0
    rw SD16INCTL0 @ 0x00: u8 = 0_0 {
        /// SD16 Input Channel select 0
        SD16INCH: 0..2 = enum SD16INCH {
            /// SD16 Input Channel select A0
            SD16INCH_0 = 0b000,
            /// SD16 Input Channel select A1
            SD16INCH_1 = 0b001,
            /// SD16 Input Channel select A2
            SD16INCH_2 = 0b010,
            /// SD16 Input Channel select A3
            SD16INCH_3 = 0b011,
            /// SD16 Input Channel select A4
            SD16INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            SD16INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            SD16INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            SD16INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        SD16GAIN0: 3 = struct SD16GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        SD16GAIN1: 4 = struct SD16GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        SD16GAIN2: 5 = struct SD16GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        SD16INTDLY: 6..7 = enum SD16INTDLY {
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
    /// SD16 Analog Input Enable Register
    rw SD16AE @ 0x07: u8 = 0_0 {
        /// SD16 External Input Enable 0
        SD16AE0: 0 = struct SD16AE0(bool);
        /// SD16 External Input Enable 1
        SD16AE1: 1 = struct SD16AE1(bool);
        /// SD16 External Input Enable 2
        SD16AE2: 2 = struct SD16AE2(bool);
        /// SD16 External Input Enable 3
        SD16AE3: 3 = struct SD16AE3(bool);
        /// SD16 External Input Enable 4
        SD16AE4: 4 = struct SD16AE4(bool);
        /// SD16 External Input Enable 5
        SD16AE5: 5 = struct SD16AE5(bool);
        /// SD16 External Input Enable 6
        SD16AE6: 6 = struct SD16AE6(bool);
        /// SD16 External Input Enable 7
        SD16AE7: 7 = struct SD16AE7(bool);
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
        /// SD16 Start Conversion
        SD16SC: 1 = struct SD16SC(bool);
        /// SD16 Channel x Interrupt Flag
        SD16IFG: 2 = struct SD16IFG(bool);
        /// SD16 Channel x Interrupt Enable
        SD16IE: 3 = struct SD16IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        SD16DF: 4 = struct SD16DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        SD16OVIFG: 5 = struct SD16OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        SD16LSBACC: 6 = struct SD16LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        SD16LSBTOG: 7 = struct SD16LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        SD16OSR0: 8 = struct SD16OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        SD16OSR1: 9 = struct SD16OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        SD16SNGL: 10 = struct SD16SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        SD16XOSR: 11 = struct SD16XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        SD16UNI: 12 = struct SD16UNI(bool);
        /// SD16 Channel x High Impedance Input Buffer Select: 0
        SD16BUF: 13..14 = enum SD16BUF {
            /// SD16 High Imp. Input Buffer: Disabled
            SD16BUF_0 = 0b00,
            /// SD16 High Imp. Input Buffer: Slow
            SD16BUF_1 = 0b01,
            /// SD16 High Imp. Input Buffer: Meduim
            SD16BUF_2 = 0b10,
            /// SD16 High Imp. Input Buffer: Fast
            SD16BUF_3 = 0b11,
        }
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
}
