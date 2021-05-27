//! SD16_A1

utils::periph! {
    /// SD16_A1
    SD16_A1;
    /// SD16 Input Control Register Channel 0
    rw INCTL0 @ 0x00: u8 = 0_0 {
        /// SD16 Input Channel select 0
        INCH: 0..2 = enum INCH {
            /// SD16 Input Channel select A0
            INCH_0 = 0b000,
            /// SD16 Input Channel select A1
            INCH_1 = 0b001,
            /// SD16 Input Channel select A2
            INCH_2 = 0b010,
            /// SD16 Input Channel select A3
            INCH_3 = 0b011,
            /// SD16 Input Channel select A4
            INCH_4 = 0b100,
            /// SD16 Input Channel select Vcc divider
            INCH_5 = 0b101,
            /// SD16 Input Channel select Temp
            INCH_6 = 0b110,
            /// SD16 Input Channel select Offset
            INCH_7 = 0b111,
        }
        /// SD16 Input Pre-Amplifier Gain Select 0
        GAIN0: 3 = struct GAIN0(bool);
        /// SD16 Input Pre-Amplifier Gain Select 1
        GAIN1: 4 = struct GAIN1(bool);
        /// SD16 Input Pre-Amplifier Gain Select 2
        GAIN2: 5 = struct GAIN2(bool);
        /// SD16 Interrupt Delay after 1.Conversion 0
        INTDLY: 6..7 = enum INTDLY {
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
    /// SD16 Analog Input Enable Register
    rw AE @ 0x07: u8 = 0_0 {
        /// SD16 External Input Enable 0
        AE0: 0 = struct AE0(bool);
        /// SD16 External Input Enable 1
        AE1: 1 = struct AE1(bool);
        /// SD16 External Input Enable 2
        AE2: 2 = struct AE2(bool);
        /// SD16 External Input Enable 3
        AE3: 3 = struct AE3(bool);
        /// SD16 External Input Enable 4
        AE4: 4 = struct AE4(bool);
        /// SD16 External Input Enable 5
        AE5: 5 = struct AE5(bool);
        /// SD16 External Input Enable 6
        AE6: 6 = struct AE6(bool);
        /// SD16 External Input Enable 7
        AE7: 7 = struct AE7(bool);
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
        /// SD16 Start Conversion
        SC: 1 = struct SC(bool);
        /// SD16 Channel x Interrupt Flag
        IFG: 2 = struct IFG(bool);
        /// SD16 Channel x Interrupt Enable
        IE: 3 = struct IE(bool);
        /// SD16 Channel x Data Format: 0:Unipolar/1:Bipolar
        DF: 4 = struct DF(bool);
        /// SD16 Channel x Overflow Interrupt Flag
        OVIFG: 5 = struct OVIFG(bool);
        /// SD16 Channel x Access LSB of ADC
        LSBACC: 6 = struct LSBACC(bool);
        /// SD16 Channel x Toggle LSB Output of ADC
        LSBTOG: 7 = struct LSBTOG(bool);
        /// SD16 Channel x OverSampling Ratio 0
        OSR0: 8 = struct OSR0(bool);
        /// SD16 Channel x OverSampling Ratio 1
        OSR1: 9 = struct OSR1(bool);
        /// SD16 Channel x Single Conversion On/Off
        SNGL: 10 = struct SNGL(bool);
        /// SD16 Channel x Extended OverSampling Ratio
        XOSR: 11 = struct XOSR(bool);
        /// SD16 Channel x Bipolar(0) / Unipolar(1) Mode
        UNI: 12 = struct UNI(bool);
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
}
