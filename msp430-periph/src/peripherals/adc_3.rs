//! ADC

utils::periph! {
    /// ADC
    ADC;
    /// ADC Control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// start conversion
        SC: 0 = enum SC {
            /// No sample-and-conversion-start
            SC_0 = 0b0,
            /// Start sample-and-conversion
            SC_1 = 0b1,
        }
        /// enable conversion
        ENC: 1 = enum ENC {
            /// ADC disabled
            ENC_0 = 0b0,
            /// ADC enabled
            ENC_1 = 0b1,
        }
        /// ADC on
        ON: 4 = enum ON {
            /// ADC off
            ON_0 = 0b0,
            /// ADC on
            ON_1 = 0b1,
        }
        /// sample-and-hold time.
        MSC: 7 = enum MSC {
            /// The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert.
            MSC_0 = 0b0,
            /// The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed.
            MSC_1 = 0b1,
        }
        /// sample-and-hold time.
        SHT: 8..11 = enum SHT {
            /// 4 ADCCLK cycles
            SHT_0 = 0b0000,
            /// 8 ADCCLK cycles
            SHT_1 = 0b0001,
            /// 16 ADCCLK cycles
            SHT_2 = 0b0010,
            /// 32 ADCCLK cycles
            SHT_3 = 0b0011,
            /// 64 ADCCLK cycles
            SHT_4 = 0b0100,
            /// 96 ADCCLK cycles
            SHT_5 = 0b0101,
            /// 128 ADCCLK cycles
            SHT_6 = 0b0110,
            /// 192 ADCCLK cycles
            SHT_7 = 0b0111,
            /// 256 ADCCLK cycles
            SHT_8 = 0b1000,
            /// 384 ADCCLK cycles
            SHT_9 = 0b1001,
            /// 512 ADCCLK cycles
            SHT_10 = 0b1010,
            /// 768 ADCCLK cycles
            SHT_11 = 0b1011,
            /// 1024 ADCCLK cycles
            SHT_12 = 0b1100,
            /// 1024 ADCCLK cycles
            SHT_13 = 0b1101,
            /// 1024 ADCCLK cycles
            SHT_14 = 0b1110,
            /// 1024 ADCCLK cycles
            SHT_15 = 0b1111,
        }
    }
    /// ADC Control 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// ADC busy
        BUSY: 0 = enum BUSY {
            /// No operation is active.
            BUSY_0 = 0b0,
            /// A sequence, sample, or conversion is active.
            BUSY_1 = 0b1,
        }
        /// conversion sequence mode select
        CONSEQ: 1..2 = enum CONSEQ {
            /// Single-channel, single-conversion
            CONSEQ_0 = 0b00,
            /// Sequence-of-channels
            CONSEQ_1 = 0b01,
            /// Repeat-single-channel
            CONSEQ_2 = 0b10,
            /// Repeat-sequence-of-channels
            CONSEQ_3 = 0b11,
        }
        /// clock source select
        SSEL: 3..4 = enum SSEL {
            /// ADCOSC (MODOSC)
            SSEL_0 = 0b00,
            /// ACLK
            SSEL_1 = 0b01,
            /// MCLK
            SSEL_2 = 0b10,
            /// SMCLK
            SSEL_3 = 0b11,
        }
        /// clock divider
        DIV: 5..7 = enum DIV {
            /// /1
            DIV_0 = 0b000,
            /// /2
            DIV_1 = 0b001,
            /// /3
            DIV_2 = 0b010,
            /// /4
            DIV_3 = 0b011,
            /// /5
            DIV_4 = 0b100,
            /// /6
            DIV_5 = 0b101,
            /// /7
            DIV_6 = 0b110,
            /// /8
            DIV_7 = 0b111,
        }
        /// invert signal sample-and-hold
        ISSH: 8 = enum ISSH {
            /// The sample-input signal is not inverted.
            ISSH_0 = 0b0,
            /// The sample-input signal is inverted.
            ISSH_1 = 0b1,
        }
        /// sample-and-hold pulse-mode select
        SHP: 9 = enum SHP {
            /// SAMPCON signal is sourced from the sample-input signal.
            SHP_0 = 0b0,
            /// SAMPCON signal is sourced from the sampling timer.
            SHP_1 = 0b1,
        }
        /// sample-and-hold source select
        SHS: 10..11 = enum SHS {
            /// ADCSC bit
            SHS_0 = 0b00,
            /// see the device-specific data sheet for source
            SHS_1 = 0b01,
            /// see the device-specific data sheet for source
            SHS_2 = 0b10,
            /// see the device-specific data sheet for source
            SHS_3 = 0b11,
        }
    }
    /// ADC Control 2
    rw CTL2 @ 0x04: u16 = 0_0 {
        /// ADC sampling rate.
        SR: 2 = struct SR(bool);
        /// data read-back format
        DF: 3 = enum DF {
            /// Binary unsigned. Theoretically the analog input voltage V(REF) results in 0000h, the analog input voltage +V(REF) results in 03FFh.
            DF_0 = 0b0,
            /// Signed binary (2s complement), left aligned. Theoretically the analog input voltage V(REF) results in 8000h, the analog input voltage +V(REF) results in 7FC0h.
            DF_1 = 0b1,
        }
        /// resolution
        RES: 4..5 = enum RES {
            /// 8 bit
            RES_0 = 0b00,
            /// 10 bit
            RES_1 = 0b01,
            /// 12 bit
            RES_2 = 0b10,
            /// Reserved
            RES_3 = 0b11,
        }
        /// ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx.
        PDIV: 8..9 = enum PDIV {
            /// Predivide by 1
            _1 = 0b00,
            /// Predivide by 4
            _4 = 0b01,
            /// Predivide by 64
            _64 = 0b10,
            /// Reserved
            PDIV_3 = 0b11,
        }
    }
    /// ADC Window Comparator Low Threshold Register
    rw LO @ 0x06: u16 = 0_0 {
        /// ADC Window Comparator Low Threshold Register
        LO: 0..15 = struct LOField(u16);
    }
    /// ADC Window Comparator High Threshold Register
    rw HI @ 0x08: u16 = 0_0 {
        /// ADC Window Comparator High Threshold Register
        HI: 0..15 = struct HIField(u16);
    }
    /// ADC Conversion Memory Control Register
    rw MCTL0 @ 0x0a: u16 = 0_0 {
        /// Input channel select
        INCH: 0..3 = enum INCH {
            /// A0 - see device-specific data sheet
            INCH_0 = 0b0000,
            /// A1 - see device-specific data sheet
            INCH_1 = 0b0001,
            /// A2 - see device-specific data sheet
            INCH_2 = 0b0010,
            /// A3 - see device-specific data sheet
            INCH_3 = 0b0011,
            /// A4 - see device-specific data sheet
            INCH_4 = 0b0100,
            /// A5 - see device-specific data sheet
            INCH_5 = 0b0101,
            /// A2 - see device-specific data sheet
            INCH_6 = 0b0110,
            /// A7 - see device-specific data sheet
            INCH_7 = 0b0111,
            /// A8 - see device-specific data sheet
            INCH_8 = 0b1000,
            /// A9 - see device-specific data sheet
            INCH_9 = 0b1001,
            /// A10 - see device-specific data sheet
            INCH_10 = 0b1010,
            /// A11 - see device-specific data sheet
            INCH_11 = 0b1011,
            /// A12 - see device-specific data sheet
            INCH_12 = 0b1100,
            /// A13 - see device-specific data sheet
            INCH_13 = 0b1101,
            /// A14 - see device-specific data sheet
            INCH_14 = 0b1110,
            /// A15 - see device-specific data sheet
            INCH_15 = 0b1111,
        }
        /// Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active.
        SREF: 4..6 = enum SREF {
            /// 000b = V(R+) = AVCC and V(R-) = AVSS
            SREF_0 = 0b000,
            /// 001b = V(R+) = VREF and V(R-) = AVSS
            SREF_1 = 0b001,
            /// 010b = V(R+) = VEREF+ buffered and V(R-) = AVSS
            SREF_2 = 0b010,
            /// 011b =V(R+) = VEREF+ and V(R-) = AVSS
            SREF_3 = 0b011,
            /// 100b = V(R+) = AVCC and V(R-) = VEREF-
            SREF_4 = 0b100,
            /// 101b = V(R+) = VREF and V(R-) = VEREF-
            SREF_5 = 0b101,
            /// 110b = V(R+) = VEREF+ buffered and V(R-) = VEREF-
            SREF_6 = 0b110,
            /// 111b = V(R+) = VEREF+ and V(R-) = VEREF-
            SREF_7 = 0b111,
        }
        /// ADC input channels expanded
        EXPCHEN: 8 = enum EXPCHEN {
            /// ADC channel expanded disable
            EXPCHEN_0 = 0b0,
            /// ADC channel expanded enable
            EXPCHEN_1 = 0b1,
        }
    }
    /// ADC Conversion Memory Register
    rw MEM0 @ 0x12: u16 = 0_0 {
        /// ADC Conversion Memory Register
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// ADC Interrupt Enable 0
    rw IE @ 0x1a: u16 = 0_0 {
        /// Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion.
        IE0: 0 = enum IE0 {
            /// 0b = Interrupt disabled
            IE0_0 = 0b0,
            /// 1b = Interrupt enabled
            IE0_1 = 0b1,
        }
        /// Interrupt enable for the inside of window interrupt of the window comparator.
        INIE: 1 = enum INIE {
            /// 0b = Inside of window interrupt disabled
            INIE_0 = 0b0,
            /// 1b = Inside of window interrupt enabled
            INIE_1 = 0b1,
        }
        /// Interrupt enable for the below lower threshold interrupt of the window comparator.
        LOIE: 2 = enum LOIE {
            /// 0b = Below lower threshold interrupt disabled
            LOIE_0 = 0b0,
            /// 1b = Below lower threshold interrupt enabled
            LOIE_1 = 0b1,
        }
        /// Interrupt enable for the above upper threshold interrupt of the window comparator.
        HIIE: 3 = enum HIIE {
            /// 0b = Above upper threshold interrupt disabled
            HIIE_0 = 0b0,
            /// 1b = Above upper threshold interrupt enabled
            HIIE_1 = 0b1,
        }
        /// MEM0 overflow interrupt enable.
        OVIE: 4 = enum OVIE {
            /// 0b = Overflow interrupt disabled
            OVIE_0 = 0b0,
            /// 1b = Overflow interrupt enabled
            OVIE_1 = 0b1,
        }
        /// ADC conversion-time-overflow interrupt enable.
        TOVIE: 5 = enum TOVIE {
            /// 0b = Conversion time overflow interrupt disabled
            TOVIE_0 = 0b0,
            /// 1b = Conversion time overflow interrupt enabled
            TOVIE_1 = 0b1,
        }
    }
    /// ADC Interrupt Flag
    rw IFG @ 0x1c: u16 = 0_0 {
        /// ADCMEM0 interrupt flag
        IFG0: 0 = enum IFG0 {
            /// No interrupt pending
            IFG0_0 = 0b0,
            /// Interrupt pending
            IFG0_1 = 0b1,
        }
        /// The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers.
        INIFG: 1 = enum INIFG {
            /// No interrupt pending
            INIFG_0 = 0b0,
            /// Interrupt pending
            INIFG_1 = 0b1,
        }
        /// The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register.
        LOIFG: 2 = enum LOIFG {
            /// No interrupt pending
            LOIFG_0 = 0b0,
            /// Interrupt pending
            LOIFG_1 = 0b1,
        }
        /// The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register.
        HIIFG: 3 = enum HIIFG {
            /// No interrupt pending
            HIIFG_0 = 0b0,
            /// Interrupt pending
            HIIFG_1 = 0b1,
        }
        /// The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read.
        OVIFG: 4 = enum OVIFG {
            /// No interrupt pending
            OVIFG_0 = 0b0,
            /// Interrupt pending
            OVIFG_1 = 0b1,
        }
        /// The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed.
        TOVIFG: 5 = enum TOVIFG {
            /// No interrupt pending
            OVIFG_0 = 0b0,
            /// Interrupt pending
            TOVIFG_1 = 0b1,
        }
    }
    /// ADC Interrupt Vector
    rw IV @ 0x1e: u16 = 0_0 {
        /// interrupt vector value
        ADCIV: 0..15 = enum ADCIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest
            OVIFG = 0b0000000000000010,
            /// Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG
            TOVIFG = 0b0000000000000100,
            /// Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG
            HIIFG = 0b0000000000000110,
            /// Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG
            LOIFG = 0b0000000000001000,
            /// nterrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG
            INIFG = 0b0000000000001010,
            /// Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest
            IFG0 = 0b0000000000001100,
        }
    }
}
