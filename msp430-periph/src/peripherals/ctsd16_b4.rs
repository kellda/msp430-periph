//! CTSD16_B4

utils::periph! {
    /// CTSD16_B4
    CTSD16_B4;
    /// CTSD16 Control Register 0
    rw CTSD16CTL @ 0x00: u16 = 0_0 {
        /// CTSD16 reference select
        CTSD16REFS: 2 = struct CTSD16REFS(bool);
        /// CTSD16CLK resistor select
        CTSD16CLKR: 3 = struct CTSD16CLKR(bool);
        /// CTSD16 clock fault flag
        CTSD16OFFG: 4 = struct CTSD16OFFG(bool);
        /// Rail-to-rail input charge pump burst mode request
        CTSD16RRIBURST: 8 = struct CTSD16RRIBURST(bool);
        /// CTSD16 Rail-to-rail input ready
        CTSD16RRIRDY: 9 = struct CTSD16RRIRDY(bool);
        /// CTSD16 Rail-to-rail input error
        CTSD16RRIERR: 10 = struct CTSD16RRIERR(bool);
    }
    /// CTSD16 Interrupt Flag Register
    rw CTSD16IFG @ 0x2c: u16 = 0_0 {
        /// CTSD16 Channel 0 Interrupt Flag
        CTSD16IFG0: 0 = struct CTSD16IFG0(bool);
        /// CTSD16 Channel 0 Overflow Interrupt Flag
        CTSD16OVIFG0: 8 = struct CTSD16OVIFG0(bool);
    }
    /// CTSD16 Interrupt Enable Register
    rw CTSD16IE @ 0x2e: u16 = 0_0 {
        /// CTSD16 Channel 0 Interrupt Enable
        CTSD16IE0: 0 = struct CTSD16IE0(bool);
        /// CTSD16 Channel 0 Overflow Interrupt Enable
        CTSD16OVIE0: 8 = struct CTSD16OVIE0(bool);
    }
    /// CTSD16 Interrupt Vector Register
    rw CTSD16IV @ 0x30: u16 = 0_0 {
        /// CTSD16 Interrupt Vector Register
        CTSD16IV: 0..15 = struct CTSD16IVField(u16);
    }
    /// CTSD16 Channel 0 Control Register
    rw CTSD16CCTL0 @ 0x02: u16 = 0_0 {
        /// CTSD16 group
        CTSD16GRP: 0 = struct CTSD16GRP(bool);
        /// CTSD16 Start Conversion
        CTSD16SC: 1 = struct CTSD16SC(bool);
        /// CTSD16 Data Format
        CTSD16DF: 4 = struct CTSD16DF(bool);
        /// CTSD16 LSB access
        CTSD16LSBACC: 6 = struct CTSD16LSBACC(bool);
        /// CTSD16 LSB toggle
        CTSD16LSBTOG: 7 = struct CTSD16LSBTOG(bool);
        /// CTSD16 Channel OverSampling Ratio 0
        CTSD16OSR: 8..9 = enum CTSD16OSR {
            /// CTSD16 Channel OverSampling Ratio 0
            CTSD16OSR_0 = 0b00,
            /// CTSD16 Channel OverSampling Ratio 1
            CTSD16OSR_1 = 0b01,
            /// CTSD16 Channel OverSampling Ratio 2
            CTSD16OSR_2 = 0b10,
            /// CTSD16 Channel OverSampling Ratio 3
            CTSD16OSR_3 = 0b11,
        }
        /// CTSD16 Channel Single Conversion Mode
        CTSD16SNGL: 10 = struct CTSD16SNGL(bool);
    }
    /// CTSD16 Channel 0 Conversion Memory
    rw CTSD16MEM0 @ 0x32: u16 = 0_0 {
        /// CTSD16 Channel 0 Conversion Memory
        CTSD16MEM0: 0..15 = struct CTSD16MEM0Field(u16);
    }
    /// CTSD16 Channel 0 Input Control Register
    rw CTSD16INCTL0 @ 0x04: u16 = 0_0 {
        /// CTSD16 Input Channel select 0
        CTSD16INCH: 0..4 = enum CTSD16INCH {
            /// CTSD16 Input Channel select A0
            CTSD16INCH_0 = 0b00000,
            /// CTSD16 Input Channel select A1
            CTSD16INCH_1 = 0b00001,
            /// CTSD16 Input Channel select A2
            CTSD16INCH_2 = 0b00010,
            /// CTSD16 Input Channel select A3
            CTSD16INCH_3 = 0b00011,
            /// CTSD16 Input Channel select A4
            CTSD16INCH_4 = 0b00100,
            /// CTSD16 Input Channel select A5
            CTSD16INCH_5 = 0b00101,
            /// CTSD16 Input Channel select Temp
            CTSD16INCH_6 = 0b00110,
            /// CTSD16 Input Channel select Vcc
            CTSD16INCH_7 = 0b00111,
            /// CTSD16 Input Channel select VBat
            CTSD16INCH_8 = 0b01000,
            /// CTSD16 Input Channel select AD0+ / AD0-
            CTSD16INCH_9 = 0b01001,
            /// CTSD16 Input Channel select AD0+ / VREF
            CTSD16INCH_10 = 0b01010,
            /// CTSD16 Input Channel select AD1+ / AD1-
            CTSD16INCH_11 = 0b01011,
            /// CTSD16 Input Channel select AD1+ / VREF
            CTSD16INCH_12 = 0b01100,
            /// CTSD16 Input Channel select AD2+ / AD2-
            CTSD16INCH_13 = 0b01101,
            /// CTSD16 Input Channel select AD2+ / VREF
            CTSD16INCH_14 = 0b01110,
            /// CTSD16 Input Channel select AD3+ / AD3 -
            CTSD16INCH_15 = 0b01111,
            /// CTSD16 Input Channel select AD3+ / VREF
            CTSD16INCH_16 = 0b10000,
            /// CTSD16 Input Channel select AD4x - VREF
            CTSD16INCH_17 = 0b10001,
            /// CTSD16 Input Channel select AD4x - DAC0
            CTSD16INCH_18 = 0b10010,
        }
        /// CTSD16 Input Pre-Amplifier Gain Select 0
        CTSD16GAIN: 5..7 = enum CTSD16GAIN {
            /// CTSD16 Input Pre-Amplifier Gain Select *1
            CTSD16GAIN_1 = 0b000,
            /// CTSD16 Input Pre-Amplifier Gain Select *2
            CTSD16GAIN_2 = 0b001,
            /// CTSD16 Input Pre-Amplifier Gain Select *4
            CTSD16GAIN_4 = 0b010,
            /// CTSD16 Input Pre-Amplifier Gain Select *8
            CTSD16GAIN_8 = 0b011,
            /// CTSD16 Input Pre-Amplifier Gain Select *16
            CTSD16GAIN_16 = 0b100,
        }
        /// CTSD16 Interrupt Delay after 1.Conversion
        CTSD16INTDLY: 8 = struct CTSD16INTDLY(bool);
        /// CTSD16 Controls input buffer
        CTSD16BUFOFF: 10 = struct CTSD16BUFOFF(bool);
        /// CTSD16 Rail-to-rail input enable
        CTSD16RRI: 11 = struct CTSD16RRI(bool);
    }
    /// CTSD16 Channel 0 Preload Register
    rw CTSD16PRE0 @ 0x06: u16 = 0_0 {
        /// CTSD16 Channel 0 Preload Register
        CTSD16PRE0: 0..15 = struct CTSD16PRE0Field(u16);
    }
}
