//! ADC

utils::periph! {
    /// ADC
    ADC;
    /// ADC Control 0
    rw ADCCTL0 @ 0x00: u16 = 0_0 {
        /// start conversion
        ADCSC: 0 = enum ADCSC {
            /// No sample-and-conversion-start
            ADCSC_0 = 0b0,
            /// Start sample-and-conversion
            ADCSC_1 = 0b1,
        }
        /// enable conversion
        ADCENC: 1 = enum ADCENC {
            /// ADC disabled
            ADCENC_0 = 0b0,
            /// ADC enabled
            ADCENC_1 = 0b1,
        }
        /// ADC on
        ADCON: 4 = enum ADCON {
            /// ADC off
            ADCON_0 = 0b0,
            /// ADC on
            ADCON_1 = 0b1,
        }
        /// sample-and-hold time.
        ADCMSC: 7 = enum ADCMSC {
            /// The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert.
            ADCMSC_0 = 0b0,
            /// The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed.
            ADCMSC_1 = 0b1,
        }
        /// sample-and-hold time.
        ADCSHT: 8..11 = enum ADCSHT {
            /// 4 ADCCLK cycles
            ADCSHT_0 = 0b0000,
            /// 8 ADCCLK cycles
            ADCSHT_1 = 0b0001,
            /// 16 ADCCLK cycles
            ADCSHT_2 = 0b0010,
            /// 32 ADCCLK cycles
            ADCSHT_3 = 0b0011,
            /// 64 ADCCLK cycles
            ADCSHT_4 = 0b0100,
            /// 96 ADCCLK cycles
            ADCSHT_5 = 0b0101,
            /// 128 ADCCLK cycles
            ADCSHT_6 = 0b0110,
            /// 192 ADCCLK cycles
            ADCSHT_7 = 0b0111,
            /// 256 ADCCLK cycles
            ADCSHT_8 = 0b1000,
            /// 384 ADCCLK cycles
            ADCSHT_9 = 0b1001,
            /// 512 ADCCLK cycles
            ADCSHT_10 = 0b1010,
            /// 768 ADCCLK cycles
            ADCSHT_11 = 0b1011,
            /// 1024 ADCCLK cycles
            ADCSHT_12 = 0b1100,
            /// 1024 ADCCLK cycles
            ADCSHT_13 = 0b1101,
            /// 1024 ADCCLK cycles
            ADCSHT_14 = 0b1110,
            /// 1024 ADCCLK cycles
            ADCSHT_15 = 0b1111,
        }
    }
    /// ADC Control 1
    rw ADCCTL1 @ 0x02: u16 = 0_0 {
        /// ADC busy
        ADCBUSY: 0 = enum ADCBUSY {
            /// No operation is active.
            ADCBUSY_0 = 0b0,
            /// A sequence, sample, or conversion is active.
            ADCBUSY_1 = 0b1,
        }
        /// conversion sequence mode select
        ADCCONSEQ: 1..2 = enum ADCCONSEQ {
            /// Single-channel, single-conversion
            ADCCONSEQ_0 = 0b00,
            /// Sequence-of-channels
            ADCCONSEQ_1 = 0b01,
            /// Repeat-single-channel
            ADCCONSEQ_2 = 0b10,
            /// Repeat-sequence-of-channels
            ADCCONSEQ_3 = 0b11,
        }
        /// clock source select
        ADCSSEL: 3..4 = enum ADCSSEL {
            /// ADCOSC (MODOSC)
            ADCSSEL_0 = 0b00,
            /// ACLK
            ADCSSEL_1 = 0b01,
            /// MCLK
            ADCSSEL_2 = 0b10,
            /// SMCLK
            ADCSSEL_3 = 0b11,
        }
        /// clock divider
        ADCDIV: 5..7 = enum ADCDIV {
            /// /1
            ADCDIV_0 = 0b000,
            /// /2
            ADCDIV_1 = 0b001,
            /// /3
            ADCDIV_2 = 0b010,
            /// /4
            ADCDIV_3 = 0b011,
            /// /5
            ADCDIV_4 = 0b100,
            /// /6
            ADCDIV_5 = 0b101,
            /// /7
            ADCDIV_6 = 0b110,
            /// /8
            ADCDIV_7 = 0b111,
        }
        /// invert signal sample-and-hold
        ADCISSH: 8 = enum ADCISSH {
            /// The sample-input signal is not inverted.
            ADCISSH_0 = 0b0,
            /// The sample-input signal is inverted.
            ADCISSH_1 = 0b1,
        }
        /// sample-and-hold pulse-mode select
        ADCSHP: 9 = enum ADCSHP {
            /// SAMPCON signal is sourced from the sample-input signal.
            ADCSHP_0 = 0b0,
            /// SAMPCON signal is sourced from the sampling timer.
            ADCSHP_1 = 0b1,
        }
        /// sample-and-hold source select
        ADCSHS: 10..11 = enum ADCSHS {
            /// ADCSC bit
            ADCSHS_0 = 0b00,
            /// see the device-specific data sheet for source
            ADCSHS_1 = 0b01,
            /// see the device-specific data sheet for source
            ADCSHS_2 = 0b10,
            /// see the device-specific data sheet for source
            ADCSHS_3 = 0b11,
        }
    }
    /// ADC Control 2
    rw ADCCTL2 @ 0x04: u16 = 0_0 {
        /// ADC sampling rate.
        ADCSR: 2 = struct ADCSR(bool);
        /// data read-back format
        ADCDF: 3 = enum ADCDF {
            /// Binary unsigned. Theoretically the analog input voltage V(REF) results in 0000h, the analog input voltage +V(REF) results in 03FFh.
            ADCDF_0 = 0b0,
            /// Signed binary (2s complement), left aligned. Theoretically the analog input voltage V(REF) results in 8000h, the analog input voltage +V(REF) results in 7FC0h.
            ADCDF_1 = 0b1,
        }
        /// resolution
        ADCRES: 4..5 = enum ADCRES {
            /// 8 bit
            ADCRES_0 = 0b00,
            /// 10 bit
            ADCRES_1 = 0b01,
            /// 12 bit
            ADCRES_2 = 0b10,
            /// Reserved
            ADCRES_3 = 0b11,
        }
        /// ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx.
        ADCPDIV: 8..9 = enum ADCPDIV {
            /// Predivide by 1
            _1 = 0b00,
            /// Predivide by 4
            _4 = 0b01,
            /// Predivide by 64
            _64 = 0b10,
            /// Reserved
            ADCPDIV_3 = 0b11,
        }
    }
    /// ADC Window Comparator Low Threshold Register
    rw ADCLO @ 0x06: u16 = 0_0 {
        /// ADC Window Comparator Low Threshold Register
        ADCLO: 0..15 = struct ADCLOField(u16);
    }
    /// ADC Window Comparator High Threshold Register
    rw ADCHI @ 0x08: u16 = 0_0 {
        /// ADC Window Comparator High Threshold Register
        ADCHI: 0..15 = struct ADCHIField(u16);
    }
    /// ADC Conversion Memory Control Register
    rw ADCMCTL0 @ 0x0a: u16 = 0_0 {
        /// Input channel select
        ADCINCH: 0..3 = enum ADCINCH {
            /// A0 - see device-specific data sheet
            ADCINCH_0 = 0b0000,
            /// A1 - see device-specific data sheet
            ADCINCH_1 = 0b0001,
            /// A2 - see device-specific data sheet
            ADCINCH_2 = 0b0010,
            /// A3 - see device-specific data sheet
            ADCINCH_3 = 0b0011,
            /// A4 - see device-specific data sheet
            ADCINCH_4 = 0b0100,
            /// A5 - see device-specific data sheet
            ADCINCH_5 = 0b0101,
            /// A2 - see device-specific data sheet
            ADCINCH_6 = 0b0110,
            /// A7 - see device-specific data sheet
            ADCINCH_7 = 0b0111,
            /// A8 - see device-specific data sheet
            ADCINCH_8 = 0b1000,
            /// A9 - see device-specific data sheet
            ADCINCH_9 = 0b1001,
            /// A10 - see device-specific data sheet
            ADCINCH_10 = 0b1010,
            /// A11 - see device-specific data sheet
            ADCINCH_11 = 0b1011,
            /// A12 - see device-specific data sheet
            ADCINCH_12 = 0b1100,
            /// A13 - see device-specific data sheet
            ADCINCH_13 = 0b1101,
            /// A14 - see device-specific data sheet
            ADCINCH_14 = 0b1110,
            /// A15 - see device-specific data sheet
            ADCINCH_15 = 0b1111,
        }
        /// Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active.
        ADCSREF: 4..6 = enum ADCSREF {
            /// 000b = V(R+) = AVCC and V(R-) = AVSS
            ADCSREF_0 = 0b000,
            /// 001b = V(R+) = VREF and V(R-) = AVSS
            ADCSREF_1 = 0b001,
            /// 010b = V(R+) = VEREF+ buffered and V(R-) = AVSS
            ADCSREF_2 = 0b010,
            /// 011b =V(R+) = VEREF+ and V(R-) = AVSS
            ADCSREF_3 = 0b011,
            /// 100b = V(R+) = AVCC and V(R-) = VEREF-
            ADCSREF_4 = 0b100,
            /// 101b = V(R+) = VREF and V(R-) = VEREF-
            ADCSREF_5 = 0b101,
            /// 110b = V(R+) = VEREF+ buffered and V(R-) = VEREF-
            ADCSREF_6 = 0b110,
            /// 111b = V(R+) = VEREF+ and V(R-) = VEREF-
            ADCSREF_7 = 0b111,
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
    rw ADCMEM0 @ 0x12: u16 = 0_0 {
        /// ADC Conversion Memory Register
        ADCMEM0: 0..15 = struct ADCMEM0Field(u16);
    }
    /// ADC Interrupt Enable 0
    rw ADCIE @ 0x1a: u16 = 0_0 {
        /// Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion.
        ADCIE0: 0 = enum ADCIE0 {
            /// 0b = Interrupt disabled
            ADCIE0_0 = 0b0,
            /// 1b = Interrupt enabled
            ADCIE0_1 = 0b1,
        }
        /// Interrupt enable for the inside of window interrupt of the window comparator.
        ADCINIE: 1 = enum ADCINIE {
            /// 0b = Inside of window interrupt disabled
            ADCINIE_0 = 0b0,
            /// 1b = Inside of window interrupt enabled
            ADCINIE_1 = 0b1,
        }
        /// Interrupt enable for the below lower threshold interrupt of the window comparator.
        ADCLOIE: 2 = enum ADCLOIE {
            /// 0b = Below lower threshold interrupt disabled
            ADCLOIE_0 = 0b0,
            /// 1b = Below lower threshold interrupt enabled
            ADCLOIE_1 = 0b1,
        }
        /// Interrupt enable for the above upper threshold interrupt of the window comparator.
        ADCHIIE: 3 = enum ADCHIIE {
            /// 0b = Above upper threshold interrupt disabled
            ADCHIIE_0 = 0b0,
            /// 1b = Above upper threshold interrupt enabled
            ADCHIIE_1 = 0b1,
        }
        /// ADCMEM0 overflow interrupt enable.
        ADCOVIE: 4 = enum ADCOVIE {
            /// 0b = Overflow interrupt disabled
            ADCOVIE_0 = 0b0,
            /// 1b = Overflow interrupt enabled
            ADCOVIE_1 = 0b1,
        }
        /// ADC conversion-time-overflow interrupt enable.
        ADCTOVIE: 5 = enum ADCTOVIE {
            /// 0b = Conversion time overflow interrupt disabled
            ADCTOVIE_0 = 0b0,
            /// 1b = Conversion time overflow interrupt enabled
            ADCTOVIE_1 = 0b1,
        }
    }
    /// ADC Interrupt Flag
    rw ADCIFG @ 0x1c: u16 = 0_0 {
        /// ADCMEM0 interrupt flag
        ADCIFG0: 0 = enum ADCIFG0 {
            /// No interrupt pending
            ADCIFG0_0 = 0b0,
            /// Interrupt pending
            ADCIFG0_1 = 0b1,
        }
        /// The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers.
        ADCINIFG: 1 = enum ADCINIFG {
            /// No interrupt pending
            ADCINIFG_0 = 0b0,
            /// Interrupt pending
            ADCINIFG_1 = 0b1,
        }
        /// The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register.
        ADCLOIFG: 2 = enum ADCLOIFG {
            /// No interrupt pending
            ADCLOIFG_0 = 0b0,
            /// Interrupt pending
            ADCLOIFG_1 = 0b1,
        }
        /// The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register.
        ADCHIIFG: 3 = enum ADCHIIFG {
            /// No interrupt pending
            ADCHIIFG_0 = 0b0,
            /// Interrupt pending
            ADCHIIFG_1 = 0b1,
        }
        /// The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read.
        ADCOVIFG: 4 = enum ADCOVIFG {
            /// No interrupt pending
            ADCOVIFG_0 = 0b0,
            /// Interrupt pending
            ADCOVIFG_1 = 0b1,
        }
        /// The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed.
        ADCTOVIFG: 5 = enum ADCTOVIFG {
            /// No interrupt pending
            ADCOVIFG_0 = 0b0,
            /// Interrupt pending
            ADCTOVIFG_1 = 0b1,
        }
    }
    /// ADC Interrupt Vector
    rw ADCIV @ 0x1e: u16 = 0_0 {
        /// interrupt vector value
        ADCIV: 0..15 = enum ADCIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest
            ADCOVIFG = 0b0000000000000010,
            /// Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG
            ADCTOVIFG = 0b0000000000000100,
            /// Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG
            ADCHIIFG = 0b0000000000000110,
            /// Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG
            ADCLOIFG = 0b0000000000001000,
            /// nterrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG
            ADCINIFG = 0b0000000000001010,
            /// Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest
            ADCIFG0 = 0b0000000000001100,
        }
    }
}
