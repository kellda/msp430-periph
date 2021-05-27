//! CTSD16_B4

utils::periph! {
    /// CTSD16_B4
    CTSD16_B4;
    /// CTSD16 Control Register 0
    rw CTL @ 0x00: u16 = 0_0 {
        /// CTSD16 reference select
        REFS: 2 = struct REFS(bool);
        /// CTSD16CLK resistor select
        CLKR: 3 = struct CLKR(bool);
        /// CTSD16 clock fault flag
        OFFG: 4 = struct OFFG(bool);
        /// Rail-to-rail input charge pump burst mode request
        RRIBURST: 8 = struct RRIBURST(bool);
        /// CTSD16 Rail-to-rail input ready
        RRIRDY: 9 = struct RRIRDY(bool);
        /// CTSD16 Rail-to-rail input error
        RRIERR: 10 = struct RRIERR(bool);
    }
    /// CTSD16 Interrupt Flag Register
    rw IFG @ 0x2c: u16 = 0_0 {
        /// CTSD16 Channel 0 Interrupt Flag
        IFG0: 0 = struct IFG0(bool);
        /// CTSD16 Channel 0 Overflow Interrupt Flag
        OVIFG0: 8 = struct OVIFG0(bool);
    }
    /// CTSD16 Interrupt Enable Register
    rw IE @ 0x2e: u16 = 0_0 {
        /// CTSD16 Channel 0 Interrupt Enable
        IE0: 0 = struct IE0(bool);
        /// CTSD16 Channel 0 Overflow Interrupt Enable
        OVIE0: 8 = struct OVIE0(bool);
    }
    /// CTSD16 Interrupt Vector Register
    rw IV @ 0x30: u16 = 0_0 {
        /// CTSD16 Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
    /// CTSD16 Channel 0 Control Register
    rw CCTL0 @ 0x02: u16 = 0_0 {
        /// CTSD16 group
        GRP: 0 = struct GRP(bool);
        /// CTSD16 Start Conversion
        SC: 1 = struct SC(bool);
        /// CTSD16 Data Format
        DF: 4 = struct DF(bool);
        /// CTSD16 LSB access
        LSBACC: 6 = struct LSBACC(bool);
        /// CTSD16 LSB toggle
        LSBTOG: 7 = struct LSBTOG(bool);
        /// CTSD16 Channel OverSampling Ratio 0
        OSR: 8..9 = enum OSR {
            /// CTSD16 Channel OverSampling Ratio 0
            OSR_0 = 0b00,
            /// CTSD16 Channel OverSampling Ratio 1
            OSR_1 = 0b01,
            /// CTSD16 Channel OverSampling Ratio 2
            OSR_2 = 0b10,
            /// CTSD16 Channel OverSampling Ratio 3
            OSR_3 = 0b11,
        }
        /// CTSD16 Channel Single Conversion Mode
        SNGL: 10 = struct SNGL(bool);
    }
    /// CTSD16 Channel 0 Conversion Memory
    rw MEM0 @ 0x32: u16 = 0_0 {
        /// CTSD16 Channel 0 Conversion Memory
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// CTSD16 Channel 0 Input Control Register
    rw INCTL0 @ 0x04: u16 = 0_0 {
        /// CTSD16 Input Channel select 0
        INCH: 0..4 = enum INCH {
            /// CTSD16 Input Channel select A0
            INCH_0 = 0b00000,
            /// CTSD16 Input Channel select A1
            INCH_1 = 0b00001,
            /// CTSD16 Input Channel select A2
            INCH_2 = 0b00010,
            /// CTSD16 Input Channel select A3
            INCH_3 = 0b00011,
            /// CTSD16 Input Channel select A4
            INCH_4 = 0b00100,
            /// CTSD16 Input Channel select A5
            INCH_5 = 0b00101,
            /// CTSD16 Input Channel select Temp
            INCH_6 = 0b00110,
            /// CTSD16 Input Channel select Vcc
            INCH_7 = 0b00111,
            /// CTSD16 Input Channel select VBat
            INCH_8 = 0b01000,
            /// CTSD16 Input Channel select AD0+ / AD0-
            INCH_9 = 0b01001,
            /// CTSD16 Input Channel select AD0+ / VREF
            INCH_10 = 0b01010,
            /// CTSD16 Input Channel select AD1+ / AD1-
            INCH_11 = 0b01011,
            /// CTSD16 Input Channel select AD1+ / VREF
            INCH_12 = 0b01100,
            /// CTSD16 Input Channel select AD2+ / AD2-
            INCH_13 = 0b01101,
            /// CTSD16 Input Channel select AD2+ / VREF
            INCH_14 = 0b01110,
            /// CTSD16 Input Channel select AD3+ / AD3 -
            INCH_15 = 0b01111,
            /// CTSD16 Input Channel select AD3+ / VREF
            INCH_16 = 0b10000,
            /// CTSD16 Input Channel select AD4x - VREF
            INCH_17 = 0b10001,
            /// CTSD16 Input Channel select AD4x - DAC0
            INCH_18 = 0b10010,
        }
        /// CTSD16 Input Pre-Amplifier Gain Select 0
        GAIN: 5..7 = enum GAIN {
            /// CTSD16 Input Pre-Amplifier Gain Select *1
            GAIN_1 = 0b000,
            /// CTSD16 Input Pre-Amplifier Gain Select *2
            GAIN_2 = 0b001,
            /// CTSD16 Input Pre-Amplifier Gain Select *4
            GAIN_4 = 0b010,
            /// CTSD16 Input Pre-Amplifier Gain Select *8
            GAIN_8 = 0b011,
            /// CTSD16 Input Pre-Amplifier Gain Select *16
            GAIN_16 = 0b100,
        }
        /// CTSD16 Interrupt Delay after 1.Conversion
        INTDLY: 8 = struct INTDLY(bool);
        /// CTSD16 Controls input buffer
        BUFOFF: 10 = struct BUFOFF(bool);
        /// CTSD16 Rail-to-rail input enable
        RRI: 11 = struct RRI(bool);
    }
    /// CTSD16 Channel 0 Preload Register
    rw PRE0 @ 0x06: u16 = 0_0 {
        /// CTSD16 Channel 0 Preload Register
        PRE0: 0..15 = struct PRE0Field(u16);
    }
}
