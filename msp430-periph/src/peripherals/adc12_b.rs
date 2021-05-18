//! ADC12_B

utils::periph! {
    /// ADC12_B
    ADC12_B;
    /// ADC12_B Control 0
    rw ADC12CTL0 @ 0x00: u16 = 0_0 {
        /// start conversion
        ADC12SC: 0 = enum ADC12SC {
            /// No sample-and-conversion-start
            ADC12SC_0 = 0b0,
            /// Start sample-and-conversion
            ADC12SC_1 = 0b1,
        }
        /// enable conversion
        ADC12ENC: 1 = enum ADC12ENC {
            /// ADC12_B disabled
            ADC12ENC_0 = 0b0,
            /// ADC12_B enabled
            ADC12ENC_1 = 0b1,
        }
        /// ADC on
        ADC12ON: 4 = enum ADC12ON {
            /// ADC12_B off
            ADC12ON_0 = 0b0,
            /// ADC12_B on
            ADC12ON_1 = 0b1,
        }
        /// sample-and-hold time.
        ADC12MSC: 7 = enum ADC12MSC {
            /// The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert.
            ADC12MSC_0 = 0b0,
            /// The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed.
            ADC12MSC_1 = 0b1,
        }
        /// sample-and-hold time.
        ADC12SHT0: 8..11 = enum ADC12SHT0 {
            /// 4 ADC12CLK cycles
            ADC12SHT0_0 = 0b0000,
            /// 8 ADC12CLK cycles
            ADC12SHT0_1 = 0b0001,
            /// 16 ADC12CLK cycles
            ADC12SHT0_2 = 0b0010,
            /// 32 ADC12CLK cycles
            ADC12SHT0_3 = 0b0011,
            /// 64 ADC12CLK cycles
            ADC12SHT0_4 = 0b0100,
            /// 96 ADC12CLK cycles
            ADC12SHT0_5 = 0b0101,
            /// 128 ADC12CLK cycles
            ADC12SHT0_6 = 0b0110,
            /// 192 ADC12CLK cycles
            ADC12SHT0_7 = 0b0111,
            /// 256 ADC12CLK cycles
            ADC12SHT0_8 = 0b1000,
            /// 384 ADC12CLK cycles
            ADC12SHT0_9 = 0b1001,
            /// 512 ADC12CLK cycles
            ADC12SHT0_10 = 0b1010,
            /// Reserved
            ADC12SHT0_11 = 0b1011,
            /// Reserved
            ADC12SHT0_12 = 0b1100,
            /// Reserved
            ADC12SHT0_13 = 0b1101,
            /// Reserved
            ADC12SHT0_14 = 0b1110,
            /// Reserved
            ADC12SHT0_15 = 0b1111,
        }
        /// sample-and-hold time.
        ADC12SHT1: 12..15 = enum ADC12SHT1 {
            /// 4 ADC12CLK cycles
            ADC12SHT1_0 = 0b0000,
            /// 8 ADC12CLK cycles
            ADC12SHT1_1 = 0b0001,
            /// 16 ADC12CLK cycles
            ADC12SHT1_2 = 0b0010,
            /// 32 ADC12CLK cycles
            ADC12SHT1_3 = 0b0011,
            /// 64 ADC12CLK cycles
            ADC12SHT1_4 = 0b0100,
            /// 96 ADC12CLK cycles
            ADC12SHT1_5 = 0b0101,
            /// 128 ADC12CLK cycles
            ADC12SHT1_6 = 0b0110,
            /// 192 ADC12CLK cycles
            ADC12SHT1_7 = 0b0111,
            /// 256 ADC12CLK cycles
            ADC12SHT1_8 = 0b1000,
            /// 384 ADC12CLK cycles
            ADC12SHT1_9 = 0b1001,
            /// 512 ADC12CLK cycles
            ADC12SHT1_10 = 0b1010,
            /// Reserved
            ADC12SHT1_11 = 0b1011,
            /// Reserved
            ADC12SHT1_12 = 0b1100,
            /// Reserved
            ADC12SHT1_13 = 0b1101,
            /// Reserved
            ADC12SHT1_14 = 0b1110,
            /// Reserved
            ADC12SHT1_15 = 0b1111,
        }
    }
    /// ADC12_B Control 1
    rw ADC12CTL1 @ 0x02: u16 = 0_0 {
        /// ADC busy
        ADC12BUSY: 0 = enum ADC12BUSY {
            /// No operation is active.
            ADC12BUSY_0 = 0b0,
            /// A sequence, sample, or conversion is active.
            ADC12BUSY_1 = 0b1,
        }
        /// conversion sequence mode select
        ADC12CONSEQ: 1..2 = enum ADC12CONSEQ {
            /// Single-channel, single-conversion
            ADC12CONSEQ_0 = 0b00,
            /// Sequence-of-channels
            ADC12CONSEQ_1 = 0b01,
            /// Repeat-single-channel
            ADC12CONSEQ_2 = 0b10,
            /// Repeat-sequence-of-channels
            ADC12CONSEQ_3 = 0b11,
        }
        /// clock source select
        ADC12SSEL: 3..4 = enum ADC12SSEL {
            /// ADC12OSC (MODOSC)
            ADC12SSEL_0 = 0b00,
            /// ACLK
            ADC12SSEL_1 = 0b01,
            /// MCLK
            ADC12SSEL_2 = 0b10,
            /// SMCLK
            ADC12SSEL_3 = 0b11,
        }
        /// clock divider
        ADC12DIV: 5..7 = enum ADC12DIV {
            /// /1
            ADC12DIV_0 = 0b000,
            /// /2
            ADC12DIV_1 = 0b001,
            /// /3
            ADC12DIV_2 = 0b010,
            /// /4
            ADC12DIV_3 = 0b011,
            /// /5
            ADC12DIV_4 = 0b100,
            /// /6
            ADC12DIV_5 = 0b101,
            /// /7
            ADC12DIV_6 = 0b110,
            /// /8
            ADC12DIV_7 = 0b111,
        }
        /// invert signal sample-and-hold
        ADC12ISSH: 8 = enum ADC12ISSH {
            /// The sample-input signal is not inverted.
            ADC12ISSH_0 = 0b0,
            /// The sample-input signal is inverted.
            ADC12ISSH_1 = 0b1,
        }
        /// sample-and-hold pulse-mode select
        ADC12SHP: 9 = enum ADC12SHP {
            /// SAMPCON signal is sourced from the sample-input signal.
            ADC12SHP_0 = 0b0,
            /// SAMPCON signal is sourced from the sampling timer.
            ADC12SHP_1 = 0b1,
        }
        /// sample-and-hold source select
        ADC12SHS: 10..12 = enum ADC12SHS {
            /// ADC12SC bit
            ADC12SHS_0 = 0b000,
            /// see the device-specific data sheet for source
            ADC12SHS_1 = 0b001,
            /// see the device-specific data sheet for source
            ADC12SHS_2 = 0b010,
            /// see the device-specific data sheet for source
            ADC12SHS_3 = 0b011,
            /// see the device-specific data sheet for source
            ADC12SHS_4 = 0b100,
            /// see the device-specific data sheet for source
            ADC12SHS_5 = 0b101,
            /// see the device-specific data sheet for source
            ADC12SHS_6 = 0b110,
            /// see the device-specific data sheet for source
            ADC12SHS_7 = 0b111,
        }
        /// predivider
        ADC12PDIV: 13..14 = enum ADC12PDIV {
            /// Predivide by 1
            _1 = 0b00,
            /// Predivide by 4
            _4 = 0b01,
            /// Predivide by 32
            _32 = 0b10,
            /// Predivide by 64
            _64 = 0b11,
        }
    }
    /// ADC12_B Control 2
    rw ADC12CTL2 @ 0x04: u16 = 0_0 {
        /// low-power mode
        ADC12PWRMD: 0 = enum ADC12PWRMD {
            /// Regular power mode where sample rate is not restricted
            ADC12PWRMD_0 = 0b0,
            /// Low power mode enable, ADC12CLK can not be greater than 1/4 the device-specific data sheet specified maximum for ADC12PWRMD = 0
            ADC12PWRMD_1 = 0b1,
        }
        /// data read-back format
        ADC12DF: 3 = enum ADC12DF {
            /// Binary unsigned. Theoretically for ADC12DIF = 0 and 12-bit mode the analog input voltage  VREF results in 0000h, the analog input voltage + VREF results in 0FFFh.
            ADC12DF_0 = 0b0,
            /// Signed binary (2s complement), left aligned. Theoretically, for ADC12DIF = 0 and 12-bit mode, the analog input voltage  VREF results in 8000h, the analog input voltage + VREF results in 7FF0h.
            ADC12DF_1 = 0b1,
        }
        /// resolution
        ADC12RES: 4..5 = enum ADC12RES {
            /// 8 bit (10 clock cycle conversion time)
            _8BIT = 0b00,
            /// 10 bit (12 clock cycle conversion time)
            _10BIT = 0b01,
            /// 12 bit (14 clock cycle conversion time)
            _12BIT = 0b10,
            /// Reserved
            ADC12RES_3 = 0b11,
        }
    }
    /// ADC12_B Control 3
    rw ADC12CTL3 @ 0x06: u16 = 0_0 {
        /// conversion start address
        ADC12CSTARTADD: 0..4 = enum ADC12CSTARTADD {
            /// Conversion start address ADC12MEM0
            ADC12MEM0 = 0b00000,
            /// Conversion start address ADC12MEM1
            ADC12MEM1 = 0b00001,
            /// Conversion start address ADC12MEM2
            ADC12MEM2 = 0b00010,
            /// Conversion start address ADC12MEM3
            ADC12MEM3 = 0b00011,
            /// Conversion start address ADC12MEM4
            ADC12MEM4 = 0b00100,
            /// Conversion start address ADC12MEM5
            ADC12MEM5 = 0b00101,
            /// Conversion start address ADC12MEM6
            ADC12MEM6 = 0b00110,
            /// Conversion start address ADC12MEM7
            ADC12MEM7 = 0b00111,
            /// Conversion start address ADC12MEM8
            ADC12MEM8 = 0b01000,
            /// Conversion start address ADC12MEM9
            ADC12MEM9 = 0b01001,
            /// Conversion start address ADC12MEM10
            ADC12MEM10 = 0b01010,
            /// Conversion start address ADC12MEM10
            ADC12MEM11 = 0b01011,
            /// Conversion start address ADC12MEM12
            ADC12MEM12 = 0b01100,
            /// Conversion start address ADC12MEM13
            ADC12MEM13 = 0b01101,
            /// Conversion start address ADC12MEM14
            ADC12MEM14 = 0b01110,
            /// Conversion start address ADC12MEM15
            ADC12MEM15 = 0b01111,
            /// Conversion start address ADC12MEM16
            ADC12MEM16 = 0b10000,
            /// Conversion start address ADC12MEM17
            ADC12MEM17 = 0b10001,
            /// Conversion start address ADC12MEM18
            ADC12MEM18 = 0b10010,
            /// Conversion start address ADC12MEM19
            ADC12MEM19 = 0b10011,
            /// Conversion start address ADC12MEM20
            ADC12MEM20 = 0b10100,
            /// Conversion start address ADC12MEM21
            ADC12MEM21 = 0b10101,
            /// Conversion start address ADC12MEM22
            ADC12MEM22 = 0b10110,
            /// Conversion start address ADC12MEM23
            ADC12MEM23 = 0b10111,
            /// Conversion start address ADC12MEM24
            ADC12MEM24 = 0b11000,
            /// Conversion start address ADC12MEM25
            ADC12MEM25 = 0b11001,
            /// Conversion start address ADC12MEM26
            ADC12MEM26 = 0b11010,
            /// Conversion start address ADC12MEM27
            ADC12MEM27 = 0b11011,
            /// Conversion start address ADC12MEM28
            ADC12MEM28 = 0b11100,
            /// Conversion start address ADC12MEM29
            ADC12MEM29 = 0b11101,
            /// Conversion start address ADC12MEM30
            ADC12MEM30 = 0b11110,
            /// Conversion start address ADC12MEM31
            ADC12MEM31 = 0b11111,
        }
        /// 1/2 AVCC ADC input channel selection
        ADC12BATMAP: 6 = enum ADC12BATMAP {
            /// external pin is selected for ADC input channel A31
            ADC12BATMAP_0 = 0b0,
            /// ADC internal 1/2 x AVCC channel is selected for ADC input channel A31
            ADC12BATMAP_1 = 0b1,
        }
        /// temperature sensor ADC input channel selection
        ADC12TCMAP: 7 = enum ADC12TCMAP {
            /// external pin is selected for ADC input channel A30
            ADC12TCMAP_0 = 0b0,
            /// ADC internal temperature sensor channel is selected for ADC input channel A30
            ADC12TCMAP_1 = 0b1,
        }
        /// int ch 0 sel to ADC in ch A29
        ADC12ICH0MAP: 8 = enum ADC12ICH0MAP {
            /// external pin is selected for ADC input channel A29
            ADC12ICH0MAP_0 = 0b0,
            /// ADC input channel internal 0 is selected for ADC input channel A29, see device-specific data sheet for availability
            ADC12ICH0MAP_1 = 0b1,
        }
        /// int ch 1 sel to ADC in ch A28
        ADC12ICH1MAP: 9 = enum ADC12ICH1MAP {
            /// external pin is selected for ADC input channel A28
            ADC12ICH1MAP_0 = 0b0,
            /// ADC input channel internal 1 is selected for ADC input channel A28, see device-specific data sheet for availability
            ADC12ICH1MAP_1 = 0b1,
        }
        /// int ch 2 sel to ADC in ch A27
        ADC12ICH2MAP: 10 = enum ADC12ICH2MAP {
            /// external pin is selected for ADC input channel A27
            ADC12ICH2MAP_0 = 0b0,
            /// ADC input channel internal 2 is selected for ADC input channel A27, see device-specific data sheet for availability
            ADC12ICH2MAP_1 = 0b1,
        }
        /// int ch 3 sel to ADC in ch A26
        ADC12ICH3MAP: 11 = enum ADC12ICH3MAP {
            /// external pin is selected for ADC input channel A26
            ADC12ICH3MAP_0 = 0b0,
            /// ADC input channel internal 3 is selected for ADC input channel A26, see device-specific data sheet for availability
            ADC12ICH3MAP_1 = 0b1,
        }
    }
    /// ADC12_B Window Comparator Low Threshold Register
    rw ADC12LO @ 0x08: u16 = 0_0 {
        /// ADC12_B Window Comparator Low Threshold Register
        ADC12LO: 0..15 = struct ADC12LOField(u16);
    }
    /// ADC12_B Window Comparator High Threshold Register
    rw ADC12HI @ 0x0a: u16 = 0_0 {
        /// ADC12_B Window Comparator High Threshold Register
        ADC12HI: 0..15 = struct ADC12HIField(u16);
    }
    /// ADC12_B Interrupt Flag 0
    rw ADC12IFGR0 @ 0x0c: u16 = 0_0 {
        /// ADC12MEM0 interrupt flag
        ADC12IFG0: 0 = enum ADC12IFG0 {
            /// No interrupt pending
            ADC12IFG0_0 = 0b0,
            /// Interrupt pending
            ADC12IFG0_1 = 0b1,
        }
        /// ADC12MEM1 interrupt flag
        ADC12IFG1: 1 = enum ADC12IFG1 {
            /// No interrupt pending
            ADC12IFG1_0 = 0b0,
            /// Interrupt pending
            ADC12IFG1_1 = 0b1,
        }
        /// ADC12MEM2 interrupt flag
        ADC12IFG2: 2 = enum ADC12IFG2 {
            /// No interrupt pending
            ADC12IFG2_0 = 0b0,
            /// Interrupt pending
            ADC12IFG2_1 = 0b1,
        }
        /// ADC12MEM3 interrupt flag
        ADC12IFG3: 3 = enum ADC12IFG3 {
            /// No interrupt pending
            ADC12IFG3_0 = 0b0,
            /// Interrupt pending
            ADC12IFG3_1 = 0b1,
        }
        /// ADC12MEM4 interrupt flag
        ADC12IFG4: 4 = enum ADC12IFG4 {
            /// No interrupt pending
            ADC12IFG4_0 = 0b0,
            /// Interrupt pending
            ADC12IFG4_1 = 0b1,
        }
        /// ADC12MEM5 interrupt flag
        ADC12IFG5: 5 = enum ADC12IFG5 {
            /// No interrupt pending
            ADC12IFG5_0 = 0b0,
            /// Interrupt pending
            ADC12IFG5_1 = 0b1,
        }
        /// ADC12MEM6 interrupt flag
        ADC12IFG6: 6 = enum ADC12IFG6 {
            /// No interrupt pending
            ADC12IFG6_0 = 0b0,
            /// Interrupt pending
            ADC12IFG6_1 = 0b1,
        }
        /// ADC12MEM7 interrupt flag
        ADC12IFG7: 7 = enum ADC12IFG7 {
            /// No interrupt pending
            ADC12IFG7_0 = 0b0,
            /// Interrupt pending
            ADC12IFG7_1 = 0b1,
        }
        /// ADC12MEM8 interrupt flag
        ADC12IFG8: 8 = enum ADC12IFG8 {
            /// No interrupt pending
            ADC12IFG8_0 = 0b0,
            /// Interrupt pending
            ADC12IFG8_1 = 0b1,
        }
        /// ADC12MEM9 interrupt flag
        ADC12IFG9: 9 = enum ADC12IFG9 {
            /// No interrupt pending
            ADC12IFG9_0 = 0b0,
            /// Interrupt pending
            ADC12IFG9_1 = 0b1,
        }
        /// ADC12MEM10 interrupt flag
        ADC12IFG10: 10 = enum ADC12IFG10 {
            /// No interrupt pending
            ADC12IFG10_0 = 0b0,
            /// Interrupt pending
            ADC12IFG10_1 = 0b1,
        }
        /// ADC12MEM11 interrupt flag
        ADC12IFG11: 11 = enum ADC12IFG11 {
            /// No interrupt pending
            ADC12IFG11_0 = 0b0,
            /// Interrupt pending
            ADC12IFG11_1 = 0b1,
        }
        /// ADC12MEM12 interrupt flag
        ADC12IFG12: 12 = enum ADC12IFG12 {
            /// No interrupt pending
            ADC12IFG12_0 = 0b0,
            /// Interrupt pending
            ADC12IFG12_1 = 0b1,
        }
        /// ADC12MEM13 interrupt flag
        ADC12IFG13: 13 = enum ADC12IFG13 {
            /// No interrupt pending
            ADC12IFG13_0 = 0b0,
            /// Interrupt pending
            ADC12IFG13_1 = 0b1,
        }
        /// ADC12MEM14 interrupt flag
        ADC12IFG14: 14 = enum ADC12IFG14 {
            /// No interrupt pending
            ADC12IFG14_0 = 0b0,
            /// Interrupt pending
            ADC12IFG14_1 = 0b1,
        }
        /// ADC12MEM15 interrupt flag
        ADC12IFG15: 15 = enum ADC12IFG15 {
            /// No interrupt pending
            ADC12IFG15_0 = 0b0,
            /// Interrupt pending
            ADC12IFG15_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Flag 1
    rw ADC12IFGR1 @ 0x0e: u16 = 0_0 {
        /// ADC12MEM16 interrupt flag
        ADC12IFG16: 0 = enum ADC12IFG16 {
            /// No interrupt pending
            ADC12IFG16_0 = 0b0,
            /// Interrupt pending
            ADC12IFG16_1 = 0b1,
        }
        /// ADC12MEM17 interrupt flag
        ADC12IFG17: 1 = enum ADC12IFG17 {
            /// No interrupt pending
            ADC12IFG17_0 = 0b0,
            /// Interrupt pending
            ADC12IFG17_1 = 0b1,
        }
        /// ADC12MEM18 interrupt flag
        ADC12IFG18: 2 = enum ADC12IFG18 {
            /// No interrupt pending
            ADC12IFG18_0 = 0b0,
            /// Interrupt pending
            ADC12IFG18_1 = 0b1,
        }
        /// ADC12MEM19 interrupt flag
        ADC12IFG19: 3 = enum ADC12IFG19 {
            /// No interrupt pending
            ADC12IFG19_0 = 0b0,
            /// Interrupt pending
            ADC12IFG19_1 = 0b1,
        }
        /// ADC12MEM20 interrupt flag
        ADC12IFG20: 4 = enum ADC12IFG20 {
            /// No interrupt pending
            ADC12IFG20_0 = 0b0,
            /// Interrupt pending
            ADC12IFG20_1 = 0b1,
        }
        /// ADC12MEM21 interrupt flag
        ADC12IFG21: 5 = enum ADC12IFG21 {
            /// No interrupt pending
            ADC12IFG21_0 = 0b0,
            /// Interrupt pending
            ADC12IFG21_1 = 0b1,
        }
        /// ADC12MEM22 interrupt flag
        ADC12IFG22: 6 = enum ADC12IFG22 {
            /// No interrupt pending
            ADC12IFG22_0 = 0b0,
            /// Interrupt pending
            ADC12IFG22_1 = 0b1,
        }
        /// ADC12MEM23 interrupt flag
        ADC12IFG23: 7 = enum ADC12IFG23 {
            /// No interrupt pending
            ADC12IFG23_0 = 0b0,
            /// Interrupt pending
            ADC12IFG23_1 = 0b1,
        }
        /// ADC12MEM24 interrupt flag
        ADC12IFG24: 8 = enum ADC12IFG24 {
            /// No interrupt pending
            ADC12IFG24_0 = 0b0,
            /// Interrupt pending
            ADC12IFG24_1 = 0b1,
        }
        /// ADC12MEM25 interrupt flag
        ADC12IFG25: 9 = enum ADC12IFG25 {
            /// No interrupt pending
            ADC12IFG25_0 = 0b0,
            /// Interrupt pending
            ADC12IFG25_1 = 0b1,
        }
        /// ADC12MEM26 interrupt flag
        ADC12IFG26: 10 = enum ADC12IFG26 {
            /// No interrupt pending
            ADC12IFG26_0 = 0b0,
            /// Interrupt pending
            ADC12IFG26_1 = 0b1,
        }
        /// ADC12MEM27 interrupt flag
        ADC12IFG27: 11 = enum ADC12IFG27 {
            /// No interrupt pending
            ADC12IFG27_0 = 0b0,
            /// Interrupt pending
            ADC12IFG27_1 = 0b1,
        }
        /// ADC12MEM28 interrupt flag
        ADC12IFG28: 12 = enum ADC12IFG28 {
            /// No interrupt pending
            ADC12IFG28_0 = 0b0,
            /// Interrupt pending
            ADC12IFG28_1 = 0b1,
        }
        /// ADC12MEM29 interrupt flag
        ADC12IFG29: 13 = enum ADC12IFG29 {
            /// No interrupt pending
            ADC12IFG29_0 = 0b0,
            /// Interrupt pending
            ADC12IFG29_1 = 0b1,
        }
        /// ADC12MEM30 interrupt flag
        ADC12IFG30: 14 = enum ADC12IFG30 {
            /// No interrupt pending
            ADC12IFG30_0 = 0b0,
            /// Interrupt pending
            ADC12IFG30_1 = 0b1,
        }
        /// ADC12MEM31 interrupt flag
        ADC12IFG31: 15 = enum ADC12IFG31 {
            /// No interrupt pending
            ADC12IFG31_0 = 0b0,
            /// Interrupt pending
            ADC12IFG31_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Flag 2
    rw ADC12IFGR2 @ 0x10: u16 = 0_0 {
        /// Interrupt flag for ADC12MEMx between ADC12HI and ADC12LO
        ADC12INIFG: 1 = enum ADC12INIFG {
            /// No interrupt pending
            ADC12INIFG_0 = 0b0,
            /// Interrupt pending
            ADC12INIFG_1 = 0b1,
        }
        /// Interrupt flag for ADC12MEMx ADC12LO
        ADC12LOIFG: 2 = enum ADC12LOIFG {
            /// No interrupt pending
            ADC12LOIFG_0 = 0b0,
            /// Interrupt pending
            ADC12LOIFG_1 = 0b1,
        }
        /// Interrupt flag for ADC12MEMx ADC12HI
        ADC12HIIFG: 3 = enum ADC12HIIFG {
            /// No interrupt pending
            ADC12HIIFG_0 = 0b0,
            /// Interrupt pending
            ADC12HIIFG_1 = 0b1,
        }
        /// ADC12MEMx overflow-interrupt flag.
        ADC12OVIFG: 4 = enum ADC12OVIFG {
            /// No interrupt pending
            ADC12OVIFG_0 = 0b0,
            /// Interrupt pending
            ADC12OVIFG_1 = 0b1,
        }
        /// conversion-time-overflow interrupt flag
        ADC12TOVIFG: 5 = enum ADC12TOVIFG {
            /// No interrupt pending
            ADC12TOVIFG_0 = 0b0,
            /// Interrupt pending
            ADC12TOVIFG_1 = 0b1,
        }
        /// reference buffer ready interrupt flag
        ADC12RDYIFG: 6 = enum ADC12RDYIFG {
            /// No interrupt pending
            ADC12RDYIFG_0 = 0b0,
            /// Interrupt pending
            ADC12RDYIFG_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Enable 0
    rw ADC12IER0 @ 0x12: u16 = 0_0 {
        /// Interrupt enable 0
        ADC12IE0: 0 = enum ADC12IE0 {
            /// Interrupt disabled
            ADC12IE0_0 = 0b0,
            /// Interrupt enabled
            ADC12IE0_1 = 0b1,
        }
        /// interrupt enable 1
        ADC12IE1: 1 = enum ADC12IE1 {
            /// Interrupt disabled
            ADC12IE1_0 = 0b0,
            /// Interrupt enabled
            ADC12IE1_1 = 0b1,
        }
        /// interrupt enable 2
        ADC12IE2: 2 = enum ADC12IE2 {
            /// Interrupt disabled
            ADC12IE2_0 = 0b0,
            /// Interrupt enabled
            ADC12IE2_1 = 0b1,
        }
        /// interrupt enable 3
        ADC12IE3: 3 = enum ADC12IE3 {
            /// Interrupt disabled
            ADC12IE3_0 = 0b0,
            /// Interrupt enabled
            ADC12IE3_1 = 0b1,
        }
        /// interrupt enable 4
        ADC12IE4: 4 = enum ADC12IE4 {
            /// Interrupt disabled
            ADC12IE4_0 = 0b0,
            /// Interrupt enabled
            ADC12IE4_1 = 0b1,
        }
        /// interrupt enable 5
        ADC12IE5: 5 = enum ADC12IE5 {
            /// Interrupt disabled
            ADC12IE5_0 = 0b0,
            /// Interrupt enabled
            ADC12IE5_1 = 0b1,
        }
        /// interrupt enable 6
        ADC12IE6: 6 = enum ADC12IE6 {
            /// Interrupt disabled
            ADC12IE6_0 = 0b0,
            /// Interrupt enabled
            ADC12IE6_1 = 0b1,
        }
        /// interrupt enable 7
        ADC12IE7: 7 = enum ADC12IE7 {
            /// Interrupt disabled
            ADC12IE7_0 = 0b0,
            /// Interrupt enabled
            ADC12IE7_1 = 0b1,
        }
        /// interrupt enable 8
        ADC12IE8: 8 = enum ADC12IE8 {
            /// Interrupt disabled
            ADC12IE8_0 = 0b0,
            /// Interrupt enabled
            ADC12IE8_1 = 0b1,
        }
        /// interrupt enable 9
        ADC12IE9: 9 = enum ADC12IE9 {
            /// Interrupt disabled
            ADC12IE9_0 = 0b0,
            /// Interrupt enabled
            ADC12IE9_1 = 0b1,
        }
        /// interrupt enable 10
        ADC12IE10: 10 = enum ADC12IE10 {
            /// Interrupt disabled
            ADC12IE10_0 = 0b0,
            /// Interrupt enabled
            ADC12IE10_1 = 0b1,
        }
        /// interrupt enable  11
        ADC12IE11: 11 = enum ADC12IE11 {
            /// Interrupt disabled
            ADC12IE11_0 = 0b0,
            /// Interrupt enabled
            ADC12IE11_1 = 0b1,
        }
        /// interrupt enable 12
        ADC12IE12: 12 = enum ADC12IE12 {
            /// Interrupt disabled
            ADC12IE12_0 = 0b0,
            /// Interrupt enabled
            ADC12IE12_1 = 0b1,
        }
        /// interrupt enable  13
        ADC12IE13: 13 = enum ADC12IE13 {
            /// Interrupt disabled
            ADC12IE13_0 = 0b0,
            /// Interrupt enabled
            ADC12IE13_1 = 0b1,
        }
        /// interrupt enable 14
        ADC12IE14: 14 = enum ADC12IE14 {
            /// Interrupt disabled
            ADC12IE14_0 = 0b0,
            /// Interrupt enabled
            ADC12IE14_1 = 0b1,
        }
        /// interrupt enable 15
        ADC12IE15: 15 = enum ADC12IE15 {
            /// Interrupt disabled
            ADC12IE15_0 = 0b0,
            /// Interrupt enabled
            ADC12IE15_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Enable 1
    rw ADC12IER1 @ 0x14: u16 = 0_0 {
        /// interrupt enable 16
        ADC12IE16: 0 = enum ADC12IE16 {
            /// Interrupt disabled
            ADC12IE16_0 = 0b0,
            /// Interrupt enabled
            ADC12IE16_1 = 0b1,
        }
        /// interrupt enable 17
        ADC12IE17: 1 = enum ADC12IE17 {
            /// Interrupt disabled
            ADC12IE17_0 = 0b0,
            /// Interrupt enabled
            ADC12IE17_1 = 0b1,
        }
        /// interrupt enable 18
        ADC12IE18: 2 = enum ADC12IE18 {
            /// Interrupt disabled
            ADC12IE18_0 = 0b0,
            /// Interrupt enabled
            ADC12IE18_1 = 0b1,
        }
        /// interrupt enable  19
        ADC12IE19: 3 = enum ADC12IE19 {
            /// Interrupt disabled
            ADC12IE19_0 = 0b0,
            /// Interrupt enabled
            ADC12IE19_1 = 0b1,
        }
        /// interrupt enable 19
        ADC12IE20: 4 = enum ADC12IE20 {
            /// Interrupt disabled
            ADC12IE20_0 = 0b0,
            /// Interrupt enabled
            ADC12IE20_1 = 0b1,
        }
        /// interrupt enable 21
        ADC12IE21: 5 = enum ADC12IE21 {
            /// Interrupt disabled
            ADC12IE21_0 = 0b0,
            /// Interrupt enabled
            ADC12IE21_1 = 0b1,
        }
        /// interrupt enable 22
        ADC12IE22: 6 = enum ADC12IE22 {
            /// Interrupt disabled
            ADC12IE22_0 = 0b0,
            /// Interrupt enabled
            ADC12IE22_1 = 0b1,
        }
        /// interrupt enable 23
        ADC12IE23: 7 = enum ADC12IE23 {
            /// Interrupt disabled
            ADC12IE23_0 = 0b0,
            /// Interrupt enabled
            ADC12IE23_1 = 0b1,
        }
        /// interrupt enable 24
        ADC12IE24: 8 = enum ADC12IE24 {
            /// Interrupt disabled
            ADC12IE24_0 = 0b0,
            /// Interrupt enabled
            ADC12IE24_1 = 0b1,
        }
        /// interrupt enable 25
        ADC12IE25: 9 = enum ADC12IE25 {
            /// Interrupt disabled
            ADC12IE25_0 = 0b0,
            /// Interrupt enabled
            ADC12IE25_1 = 0b1,
        }
        /// interrupt enable 26
        ADC12IE26: 10 = enum ADC12IE26 {
            /// Interrupt disabled
            ADC12IE26_0 = 0b0,
            /// Interrupt enabled
            ADC12IE26_1 = 0b1,
        }
        /// interrupt enable 27
        ADC12IE27: 11 = enum ADC12IE27 {
            /// Interrupt disabled
            ADC12IE27_0 = 0b0,
            /// Interrupt enabled
            ADC12IE27_1 = 0b1,
        }
        /// interrupt enable  28
        ADC12IE28: 12 = enum ADC12IE28 {
            /// Interrupt disabled
            ADC12IE28_0 = 0b0,
            /// Interrupt enabled
            ADC12IE28_1 = 0b1,
        }
        /// interrupt enable 29
        ADC12IE29: 13 = enum ADC12IE29 {
            /// Interrupt disabled
            ADC12IE29_0 = 0b0,
            /// Interrupt enabled
            ADC12IE29_1 = 0b1,
        }
        /// interrupt enable 30
        ADC12IE30: 14 = enum ADC12IE30 {
            /// Interrupt disabled
            ADC12IE30_0 = 0b0,
            /// Interrupt enabled
            ADC12IE30_1 = 0b1,
        }
        /// interrupt enable 30
        ADC12IE31: 15 = enum ADC12IE31 {
            /// Interrupt disabled
            ADC12IE31_0 = 0b0,
            /// Interrupt enabled
            ADC12IE31_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Enable 2
    rw ADC12IER2 @ 0x16: u16 = 0_0 {
        /// interrupt enable MEMx between ADC12HI and LO
        ADC12INIE: 1 = enum ADC12INIE {
            /// Interrupt disabled
            ADC12INIE_0 = 0b0,
            /// Interrupt enabled
            ADC12INIE_1 = 0b1,
        }
        /// interrupt enable MEMx  ADC12LO
        ADC12LOIE: 2 = enum ADC12LOIE {
            /// Interrupt disabled
            ADC12LOIE_0 = 0b0,
            /// Interrupt enabled
            ADC12LOIE_1 = 0b1,
        }
        /// interrupt enable MEMx  ADC12HI
        ADC12HIIE: 3 = enum ADC12HIIE {
            /// Interrupt disabled
            ADC12HIIE_0 = 0b0,
            /// Interrupt enabled
            ADC12HIIE_1 = 0b1,
        }
        /// ADC12MEMx overflow-interrupt enable
        ADC12OVIE: 4 = enum ADC12OVIE {
            /// Interrupt disabled
            ADC12OVIE_0 = 0b0,
            /// Interrupt enabled
            ADC12OVIE_1 = 0b1,
        }
        /// conversion-time-overflow interrupt enable
        ADC12TOVIE: 5 = enum ADC12TOVIE {
            /// Interrupt disabled
            ADC12TOVIE_0 = 0b0,
            /// Interrupt enabled
            ADC12TOVIE_1 = 0b1,
        }
        /// interrupt enable ADC ref buffer ready
        ADC12RDYIE: 6 = enum ADC12RDYIE {
            /// Interrupt disabled
            ADC12RDYIE_0 = 0b0,
            /// Interrupt enabled
            ADC12RDYIE_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Vector
    rw ADC12IV @ 0x18: u16 = 0_0 {
        /// interrupt vector value
        ADC12IV: 0..15 = enum ADC12IVField {
            /// Interrupt Source: No interrupt pending, Interrupt Flag: None
            NONE = 0b0000000000000000,
            /// Interrupt Source: ADC12MEMx overflow, Interrupt Flag: ADC12OVIFG, Interrupt Priority: Highest
            ADC12OVIFG = 0b0000000000000010,
            /// Interrupt Source: Conversion time overflow, Interrupt Flag: ADC12TOVIFG
            ADC12TOVIFG = 0b0000000000000100,
            /// Interrupt Source: ADC12 window high interrupt flag, Interrupt Flag: ADC12HIIFG
            ADC12HIIFG = 0b0000000000000110,
            /// Interrupt Source: ADC12 window low interrupt flag, Interrupt Flag: ADC12LOIFG
            ADC12LOIFG = 0b0000000000001000,
            /// Interrupt Source: ADC12 in-window interrupt flag, Interrupt Flag: ADC12INIFG
            ADC12INIFG = 0b0000000000001010,
            /// Interrupt Source: ADC12MEM0 interrupt flag, Interrupt Flag: ADC12IFG0
            ADC12IFG0 = 0b0000000000001100,
            /// Interrupt Source: ADC12MEM1 interrupt flag, Interrupt Flag: ADC12IFG1
            ADC12IFG1 = 0b0000000000001110,
            /// Interrupt Source: ADC12MEM2 interrupt flag, Interrupt Flag: ADC12IFG2
            ADC12IFG2 = 0b0000000000010000,
            /// Interrupt Source: ADC12MEM3 interrupt flag, Interrupt Flag: ADC12IFG3
            ADC12IFG3 = 0b0000000000010010,
            /// Interrupt Source: ADC12MEM4 interrupt flag, Interrupt Flag: ADC12IFG4
            ADC12IFG4 = 0b0000000000010100,
            /// Interrupt Source: ADC12MEM5 interrupt flag, Interrupt Flag: ADC12IFG5
            ADC12IFG5 = 0b0000000000010110,
            /// Interrupt Source: ADC12MEM6 interrupt flag, Interrupt Flag: ADC12IFG6
            ADC12IFG6 = 0b0000000000011000,
            /// Interrupt Source: ADC12MEM7 interrupt flag, Interrupt Flag: ADC12IFG7
            ADC12IFG7 = 0b0000000000011010,
            /// Interrupt Source: ADC12MEM8 interrupt flag, Interrupt Flag: ADC12IFG8
            ADC12IFG8 = 0b0000000000011100,
            /// Interrupt Source: ADC12MEM9 interrupt flag, Interrupt Flag: ADC12IFG9
            ADC12IFG9 = 0b0000000000011110,
            /// Interrupt Source: ADC12MEM10 interrupt flag, Interrupt Flag: ADC12IFG10
            ADC12IFG10 = 0b0000000000100000,
            /// Interrupt Source: ADC12MEM11 interrupt flag, Interrupt Flag: ADC12IFG11
            ADC12IFG11 = 0b0000000000100010,
            /// Interrupt Source: ADC12MEM12 interrupt flag, Interrupt Flag: ADC12IFG12
            ADC12IFG12 = 0b0000000000100100,
            /// Interrupt Source: ADC12MEM13 interrupt flag, Interrupt Flag: ADC12IFG13
            ADC12IFG13 = 0b0000000000100110,
            /// Interrupt Source: ADC12MEM14 interrupt flag, Interrupt Flag: ADC12IFG14
            ADC12IFG14 = 0b0000000000101000,
            /// Interrupt Source: ADC12MEM15 interrupt flag, Interrupt Flag: ADC12IFG15
            ADC12IFG15 = 0b0000000000101010,
            /// Interrupt Source: ADC12MEM16 interrupt flag, Interrupt Flag: ADC12IFG16
            ADC12IFG16 = 0b0000000000101100,
            /// Interrupt Source: ADC12MEM17 interrupt flag, Interrupt Flag: ADC12IFG17
            ADC12IFG17 = 0b0000000000101110,
            /// Interrupt Source: ADC12MEM18 interrupt flag, Interrupt Flag: ADC12IFG18
            ADC12IFG18 = 0b0000000000110000,
            /// Interrupt Source: ADC12MEM19 interrupt flag, Interrupt Flag: ADC12IFG19
            ADC12IFG19 = 0b0000000000110010,
            /// Interrupt Source: ADC12MEM20 interrupt flag, Interrupt Flag: ADC12IFG20
            ADC12IFG20 = 0b0000000000110100,
            /// Interrupt Source: ADC12MEM21 interrupt flag, Interrupt Flag: ADC12IFG21
            ADC12IFG21 = 0b0000000000110110,
            /// Interrupt Source: ADC12MEM22 interrupt flag, Interrupt Flag: ADC12IFG22
            ADC12IFG22 = 0b0000000000111000,
            /// Interrupt Source: ADC12MEM23 interrupt flag, Interrupt Flag: ADC12IFG23
            ADC12IFG23 = 0b0000000000111010,
            /// Interrupt Source: ADC12MEM24 interrupt flag, Interrupt Flag: ADC12IFG24
            ADC12IFG24 = 0b0000000000111100,
            /// Interrupt Source: ADC12MEM25 interrupt flag, Interrupt Flag: ADC12IFG25
            ADC12IFG25 = 0b0000000000111110,
            /// Interrupt Source: ADC12MEM26 interrupt flag, Interrupt Flag: ADC12IFG26
            ADC12IFG26 = 0b0000000001000000,
            /// Interrupt Source: ADC12MEM27 interrupt flag, Interrupt Flag: ADC12IFG27
            ADC12IFG27 = 0b0000000001000010,
            /// Interrupt Source: ADC12MEM28 interrupt flag, Interrupt Flag: ADC12IFG28
            ADC12IFG28 = 0b0000000001000100,
            /// Interrupt Source: ADC12MEM29 interrupt flag, Interrupt Flag: ADC12IFG29
            ADC12IFG29 = 0b0000000001000110,
            /// Interrupt Source: ADC12MEM30 interrupt flag, Interrupt Flag: ADC12IFG30
            ADC12IFG30 = 0b0000000001001000,
            /// Interrupt Source: ADC12MEM31 interrupt flag, Interrupt Flag: ADC12IFG31
            ADC12IFG31 = 0b0000000001001010,
            /// Interrupt Source: ADC12RDYIFG interrupt flag, Interrupt Flag: ADC12RDYIFG
            ADC12RDYIFG = 0b0000000001001100,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL0 @ 0x20: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL0_ADC12INCH: 0..4 = enum ADC12MCTL0_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL0_ADC12EOS: 7 = enum ADC12MCTL0_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL0_ADC12VRSEL: 8..11 = enum ADC12MCTL0_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL0_ADC12DIF: 13 = enum ADC12MCTL0_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL0_ADC12WINC: 14 = enum ADC12MCTL0_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL1 @ 0x22: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL1_ADC12INCH: 0..4 = enum ADC12MCTL1_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL1_ADC12EOS: 7 = enum ADC12MCTL1_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL1_ADC12VRSEL: 8..11 = enum ADC12MCTL1_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL1_ADC12DIF: 13 = enum ADC12MCTL1_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL1_ADC12WINC: 14 = enum ADC12MCTL1_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL2 @ 0x24: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL2_ADC12INCH: 0..4 = enum ADC12MCTL2_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL2_ADC12EOS: 7 = enum ADC12MCTL2_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL2_ADC12VRSEL: 8..11 = enum ADC12MCTL2_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL2_ADC12DIF: 13 = enum ADC12MCTL2_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL2_ADC12WINC: 14 = enum ADC12MCTL2_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL3 @ 0x26: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL3_ADC12INCH: 0..4 = enum ADC12MCTL3_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL3_ADC12EOS: 7 = enum ADC12MCTL3_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL3_ADC12VRSEL: 8..11 = enum ADC12MCTL3_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL3_ADC12DIF: 13 = enum ADC12MCTL3_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL3_ADC12WINC: 14 = enum ADC12MCTL3_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL4 @ 0x28: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL4_ADC12INCH: 0..4 = enum ADC12MCTL4_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL4_ADC12EOS: 7 = enum ADC12MCTL4_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL4_ADC12VRSEL: 8..11 = enum ADC12MCTL4_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL4_ADC12DIF: 13 = enum ADC12MCTL4_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL4_ADC12WINC: 14 = enum ADC12MCTL4_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL5 @ 0x2a: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL5_ADC12INCH: 0..4 = enum ADC12MCTL5_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL5_ADC12EOS: 7 = enum ADC12MCTL5_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL5_ADC12VRSEL: 8..11 = enum ADC12MCTL5_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL5_ADC12DIF: 13 = enum ADC12MCTL5_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL5_ADC12WINC: 14 = enum ADC12MCTL5_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL6 @ 0x2c: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL6_ADC12INCH: 0..4 = enum ADC12MCTL6_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL6_ADC12EOS: 7 = enum ADC12MCTL6_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL6_ADC12VRSEL: 8..11 = enum ADC12MCTL6_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL6_ADC12DIF: 13 = enum ADC12MCTL6_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL6_ADC12WINC: 14 = enum ADC12MCTL6_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL7 @ 0x2e: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL7_ADC12INCH: 0..4 = enum ADC12MCTL7_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL7_ADC12EOS: 7 = enum ADC12MCTL7_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL7_ADC12VRSEL: 8..11 = enum ADC12MCTL7_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL7_ADC12DIF: 13 = enum ADC12MCTL7_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL7_ADC12WINC: 14 = enum ADC12MCTL7_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL8 @ 0x30: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL8_ADC12INCH: 0..4 = enum ADC12MCTL8_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL8_ADC12EOS: 7 = enum ADC12MCTL8_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL8_ADC12VRSEL: 8..11 = enum ADC12MCTL8_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL8_ADC12DIF: 13 = enum ADC12MCTL8_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL8_ADC12WINC: 14 = enum ADC12MCTL8_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL9 @ 0x32: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL9_ADC12INCH: 0..4 = enum ADC12MCTL9_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL9_ADC12EOS: 7 = enum ADC12MCTL9_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL9_ADC12VRSEL: 8..11 = enum ADC12MCTL9_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL9_ADC12DIF: 13 = enum ADC12MCTL9_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL9_ADC12WINC: 14 = enum ADC12MCTL9_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL10 @ 0x34: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL10_ADC12INCH: 0..4 = enum ADC12MCTL10_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL10_ADC12EOS: 7 = enum ADC12MCTL10_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL10_ADC12VRSEL: 8..11 = enum ADC12MCTL10_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL10_ADC12DIF: 13 = enum ADC12MCTL10_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL10_ADC12WINC: 14 = enum ADC12MCTL10_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL11 @ 0x36: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL11_ADC12INCH: 0..4 = enum ADC12MCTL11_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL11_ADC12EOS: 7 = enum ADC12MCTL11_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL11_ADC12VRSEL: 8..11 = enum ADC12MCTL11_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL11_ADC12DIF: 13 = enum ADC12MCTL11_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL11_ADC12WINC: 14 = enum ADC12MCTL11_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL12 @ 0x38: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL12_ADC12INCH: 0..4 = enum ADC12MCTL12_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL12_ADC12EOS: 7 = enum ADC12MCTL12_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL12_ADC12VRSEL: 8..11 = enum ADC12MCTL12_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL12_ADC12DIF: 13 = enum ADC12MCTL12_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL12_ADC12WINC: 14 = enum ADC12MCTL12_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL13 @ 0x3a: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL13_ADC12INCH: 0..4 = enum ADC12MCTL13_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL13_ADC12EOS: 7 = enum ADC12MCTL13_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL13_ADC12VRSEL: 8..11 = enum ADC12MCTL13_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL13_ADC12DIF: 13 = enum ADC12MCTL13_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL13_ADC12WINC: 14 = enum ADC12MCTL13_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL14 @ 0x3c: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL14_ADC12INCH: 0..4 = enum ADC12MCTL14_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL14_ADC12EOS: 7 = enum ADC12MCTL14_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL14_ADC12VRSEL: 8..11 = enum ADC12MCTL14_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL14_ADC12DIF: 13 = enum ADC12MCTL14_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL14_ADC12WINC: 14 = enum ADC12MCTL14_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL15 @ 0x3e: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL15_ADC12INCH: 0..4 = enum ADC12MCTL15_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL15_ADC12EOS: 7 = enum ADC12MCTL15_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL15_ADC12VRSEL: 8..11 = enum ADC12MCTL15_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL15_ADC12DIF: 13 = enum ADC12MCTL15_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL15_ADC12WINC: 14 = enum ADC12MCTL15_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL16 @ 0x40: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL16_ADC12INCH: 0..4 = enum ADC12MCTL16_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL16_ADC12EOS: 7 = enum ADC12MCTL16_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL16_ADC12VRSEL: 8..11 = enum ADC12MCTL16_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL16_ADC12DIF: 13 = enum ADC12MCTL16_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL16_ADC12WINC: 14 = enum ADC12MCTL16_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL17 @ 0x42: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL17_ADC12INCH: 0..4 = enum ADC12MCTL17_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL17_ADC12EOS: 7 = enum ADC12MCTL17_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL17_ADC12VRSEL: 8..11 = enum ADC12MCTL17_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL17_ADC12DIF: 13 = enum ADC12MCTL17_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL17_ADC12WINC: 14 = enum ADC12MCTL17_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL18 @ 0x44: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL18_ADC12INCH: 0..4 = enum ADC12MCTL18_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL18_ADC12EOS: 7 = enum ADC12MCTL18_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL18_ADC12VRSEL: 8..11 = enum ADC12MCTL18_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL18_ADC12DIF: 13 = enum ADC12MCTL18_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL18_ADC12WINC: 14 = enum ADC12MCTL18_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL19 @ 0x46: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL19_ADC12INCH: 0..4 = enum ADC12MCTL19_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL19_ADC12EOS: 7 = enum ADC12MCTL19_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL19_ADC12VRSEL: 8..11 = enum ADC12MCTL19_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL19_ADC12DIF: 13 = enum ADC12MCTL19_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL19_ADC12WINC: 14 = enum ADC12MCTL19_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL20 @ 0x48: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL20_ADC12INCH: 0..4 = enum ADC12MCTL20_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL20_ADC12EOS: 7 = enum ADC12MCTL20_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL20_ADC12VRSEL: 8..11 = enum ADC12MCTL20_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL20_ADC12DIF: 13 = enum ADC12MCTL20_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL20_ADC12WINC: 14 = enum ADC12MCTL20_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL21 @ 0x4a: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL21_ADC12INCH: 0..4 = enum ADC12MCTL21_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL21_ADC12EOS: 7 = enum ADC12MCTL21_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL21_ADC12VRSEL: 8..11 = enum ADC12MCTL21_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL21_ADC12DIF: 13 = enum ADC12MCTL21_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL21_ADC12WINC: 14 = enum ADC12MCTL21_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL22 @ 0x4c: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL22_ADC12INCH: 0..4 = enum ADC12MCTL22_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL22_ADC12EOS: 7 = enum ADC12MCTL22_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL22_ADC12VRSEL: 8..11 = enum ADC12MCTL22_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL22_ADC12DIF: 13 = enum ADC12MCTL22_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL22_ADC12WINC: 14 = enum ADC12MCTL22_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL23 @ 0x4e: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL23_ADC12INCH: 0..4 = enum ADC12MCTL23_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL23_ADC12EOS: 7 = enum ADC12MCTL23_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL23_ADC12VRSEL: 8..11 = enum ADC12MCTL23_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL23_ADC12DIF: 13 = enum ADC12MCTL23_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL23_ADC12WINC: 14 = enum ADC12MCTL23_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL24 @ 0x50: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL24_ADC12INCH: 0..4 = enum ADC12MCTL24_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL24_ADC12EOS: 7 = enum ADC12MCTL24_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL24_ADC12VRSEL: 8..11 = enum ADC12MCTL24_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL24_ADC12DIF: 13 = enum ADC12MCTL24_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL24_ADC12WINC: 14 = enum ADC12MCTL24_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL25 @ 0x52: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL25_ADC12INCH: 0..4 = enum ADC12MCTL25_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL25_ADC12EOS: 7 = enum ADC12MCTL25_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL25_ADC12VRSEL: 8..11 = enum ADC12MCTL25_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL25_ADC12DIF: 13 = enum ADC12MCTL25_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL25_ADC12WINC: 14 = enum ADC12MCTL25_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL26 @ 0x54: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL26_ADC12INCH: 0..4 = enum ADC12MCTL26_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL26_ADC12EOS: 7 = enum ADC12MCTL26_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL26_ADC12VRSEL: 8..11 = enum ADC12MCTL26_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL26_ADC12DIF: 13 = enum ADC12MCTL26_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL26_ADC12WINC: 14 = enum ADC12MCTL26_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL27 @ 0x56: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL27_ADC12INCH: 0..4 = enum ADC12MCTL27_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL27_ADC12EOS: 7 = enum ADC12MCTL27_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL27_ADC12VRSEL: 8..11 = enum ADC12MCTL27_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL27_ADC12DIF: 13 = enum ADC12MCTL27_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL27_ADC12WINC: 14 = enum ADC12MCTL27_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL28 @ 0x58: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL28_ADC12INCH: 0..4 = enum ADC12MCTL28_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL28_ADC12EOS: 7 = enum ADC12MCTL28_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL28_ADC12VRSEL: 8..11 = enum ADC12MCTL28_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL28_ADC12DIF: 13 = enum ADC12MCTL28_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL28_ADC12WINC: 14 = enum ADC12MCTL28_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL29 @ 0x5a: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL29_ADC12INCH: 0..4 = enum ADC12MCTL29_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL29_ADC12EOS: 7 = enum ADC12MCTL29_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL29_ADC12VRSEL: 8..11 = enum ADC12MCTL29_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL29_ADC12DIF: 13 = enum ADC12MCTL29_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL29_ADC12WINC: 14 = enum ADC12MCTL29_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL30 @ 0x5c: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL30_ADC12INCH: 0..4 = enum ADC12MCTL30_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL30_ADC12EOS: 7 = enum ADC12MCTL30_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL30_ADC12VRSEL: 8..11 = enum ADC12MCTL30_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL30_ADC12DIF: 13 = enum ADC12MCTL30_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL30_ADC12WINC: 14 = enum ADC12MCTL30_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw ADC12MCTL31 @ 0x5e: u16 = 0_0 {
        /// Input channel select
        ADC12MCTL31_ADC12INCH: 0..4 = enum ADC12MCTL31_ADC12INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            ADC12INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            ADC12INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            ADC12INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            ADC12INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            ADC12INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            ADC12INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            ADC12INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            ADC12INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            ADC12INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            ADC12INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            ADC12INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            ADC12INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            ADC12INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            ADC12INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            ADC12INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            ADC12INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            ADC12INCH_31 = 0b11111,
        }
        /// End of sequence
        ADC12MCTL31_ADC12EOS: 7 = enum ADC12MCTL31_ADC12EOS {
            /// Not end of sequence
            ADC12EOS_0 = 0b0,
            /// End of sequence
            ADC12EOS_1 = 0b1,
        }
        /// reference selection
        ADC12MCTL31_ADC12VRSEL: 8..11 = enum ADC12MCTL31_ADC12VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            ADC12VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            ADC12VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            ADC12VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            ADC12VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            ADC12VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            ADC12VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            ADC12VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            ADC12VRSEL_7 = 0b0111,
            /// Reserved
            ADC12VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            ADC12VRSEL_9 = 0b1001,
            /// Reserved
            ADC12VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            ADC12VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            ADC12VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            ADC12VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            ADC12VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            ADC12VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        ADC12MCTL31_ADC12DIF: 13 = enum ADC12MCTL31_ADC12DIF {
            /// Single-ended mode enabled
            ADC12DIF_0 = 0b0,
            /// Differential mode enabled
            ADC12DIF_1 = 0b1,
        }
        /// Comparator window enable
        ADC12MCTL31_ADC12WINC: 14 = enum ADC12MCTL31_ADC12WINC {
            /// Comparator window disabled
            ADC12WINC_0 = 0b0,
            /// Comparator window enabled
            ADC12WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM0 @ 0x60: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM0: 0..15 = struct ADC12MEM0Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM1 @ 0x62: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM1: 0..15 = struct ADC12MEM1Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM2 @ 0x64: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM2: 0..15 = struct ADC12MEM2Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM3 @ 0x66: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM3: 0..15 = struct ADC12MEM3Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM4 @ 0x68: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM4: 0..15 = struct ADC12MEM4Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM5 @ 0x6a: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM5: 0..15 = struct ADC12MEM5Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM6 @ 0x6c: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM6: 0..15 = struct ADC12MEM6Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM7 @ 0x6e: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM7: 0..15 = struct ADC12MEM7Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM8 @ 0x70: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM8: 0..15 = struct ADC12MEM8Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM9 @ 0x72: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM9: 0..15 = struct ADC12MEM9Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM10 @ 0x74: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM10: 0..15 = struct ADC12MEM10Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM11 @ 0x76: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM11: 0..15 = struct ADC12MEM11Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM12 @ 0x78: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM12: 0..15 = struct ADC12MEM12Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM13 @ 0x7a: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM13: 0..15 = struct ADC12MEM13Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM14 @ 0x7c: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM14: 0..15 = struct ADC12MEM14Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM15 @ 0x7e: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM15: 0..15 = struct ADC12MEM15Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM16 @ 0x80: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM16: 0..15 = struct ADC12MEM16Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM17 @ 0x82: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM17: 0..15 = struct ADC12MEM17Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM18 @ 0x84: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM18: 0..15 = struct ADC12MEM18Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM19 @ 0x86: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM19: 0..15 = struct ADC12MEM19Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM20 @ 0x88: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM20: 0..15 = struct ADC12MEM20Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM21 @ 0x8a: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM21: 0..15 = struct ADC12MEM21Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM22 @ 0x8c: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM22: 0..15 = struct ADC12MEM22Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM23 @ 0x8e: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM23: 0..15 = struct ADC12MEM23Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM24 @ 0x90: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM24: 0..15 = struct ADC12MEM24Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM25 @ 0x92: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM25: 0..15 = struct ADC12MEM25Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM26 @ 0x94: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM26: 0..15 = struct ADC12MEM26Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM27 @ 0x96: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM27: 0..15 = struct ADC12MEM27Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM28 @ 0x98: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM28: 0..15 = struct ADC12MEM28Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM29 @ 0x9a: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM29: 0..15 = struct ADC12MEM29Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM30 @ 0x9c: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM30: 0..15 = struct ADC12MEM30Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw ADC12MEM31 @ 0x9e: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        ADC12MEM31: 0..15 = struct ADC12MEM31Field(u16);
    }
}
