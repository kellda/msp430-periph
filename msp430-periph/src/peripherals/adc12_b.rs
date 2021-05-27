//! ADC12_B

utils::periph! {
    /// ADC12_B
    ADC12_B;
    /// ADC12_B Control 0
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
            /// ADC12_B disabled
            ENC_0 = 0b0,
            /// ADC12_B enabled
            ENC_1 = 0b1,
        }
        /// ADC on
        ON: 4 = enum ON {
            /// ADC12_B off
            ON_0 = 0b0,
            /// ADC12_B on
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
        SHT0: 8..11 = enum SHT0 {
            /// 4 ADC12CLK cycles
            SHT0_0 = 0b0000,
            /// 8 ADC12CLK cycles
            SHT0_1 = 0b0001,
            /// 16 ADC12CLK cycles
            SHT0_2 = 0b0010,
            /// 32 ADC12CLK cycles
            SHT0_3 = 0b0011,
            /// 64 ADC12CLK cycles
            SHT0_4 = 0b0100,
            /// 96 ADC12CLK cycles
            SHT0_5 = 0b0101,
            /// 128 ADC12CLK cycles
            SHT0_6 = 0b0110,
            /// 192 ADC12CLK cycles
            SHT0_7 = 0b0111,
            /// 256 ADC12CLK cycles
            SHT0_8 = 0b1000,
            /// 384 ADC12CLK cycles
            SHT0_9 = 0b1001,
            /// 512 ADC12CLK cycles
            SHT0_10 = 0b1010,
            /// Reserved
            SHT0_11 = 0b1011,
            /// Reserved
            SHT0_12 = 0b1100,
            /// Reserved
            SHT0_13 = 0b1101,
            /// Reserved
            SHT0_14 = 0b1110,
            /// Reserved
            SHT0_15 = 0b1111,
        }
        /// sample-and-hold time.
        SHT1: 12..15 = enum SHT1 {
            /// 4 ADC12CLK cycles
            SHT1_0 = 0b0000,
            /// 8 ADC12CLK cycles
            SHT1_1 = 0b0001,
            /// 16 ADC12CLK cycles
            SHT1_2 = 0b0010,
            /// 32 ADC12CLK cycles
            SHT1_3 = 0b0011,
            /// 64 ADC12CLK cycles
            SHT1_4 = 0b0100,
            /// 96 ADC12CLK cycles
            SHT1_5 = 0b0101,
            /// 128 ADC12CLK cycles
            SHT1_6 = 0b0110,
            /// 192 ADC12CLK cycles
            SHT1_7 = 0b0111,
            /// 256 ADC12CLK cycles
            SHT1_8 = 0b1000,
            /// 384 ADC12CLK cycles
            SHT1_9 = 0b1001,
            /// 512 ADC12CLK cycles
            SHT1_10 = 0b1010,
            /// Reserved
            SHT1_11 = 0b1011,
            /// Reserved
            SHT1_12 = 0b1100,
            /// Reserved
            SHT1_13 = 0b1101,
            /// Reserved
            SHT1_14 = 0b1110,
            /// Reserved
            SHT1_15 = 0b1111,
        }
    }
    /// ADC12_B Control 1
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
            /// ADC12OSC (MODOSC)
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
        SHS: 10..12 = enum SHS {
            /// ADC12SC bit
            SHS_0 = 0b000,
            /// see the device-specific data sheet for source
            SHS_1 = 0b001,
            /// see the device-specific data sheet for source
            SHS_2 = 0b010,
            /// see the device-specific data sheet for source
            SHS_3 = 0b011,
            /// see the device-specific data sheet for source
            SHS_4 = 0b100,
            /// see the device-specific data sheet for source
            SHS_5 = 0b101,
            /// see the device-specific data sheet for source
            SHS_6 = 0b110,
            /// see the device-specific data sheet for source
            SHS_7 = 0b111,
        }
        /// predivider
        PDIV: 13..14 = enum PDIV {
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
    rw CTL2 @ 0x04: u16 = 0_0 {
        /// low-power mode
        PWRMD: 0 = enum PWRMD {
            /// Regular power mode where sample rate is not restricted
            PWRMD_0 = 0b0,
            /// Low power mode enable, ADC12CLK can not be greater than 1/4 the device-specific data sheet specified maximum for PWRMD = 0
            PWRMD_1 = 0b1,
        }
        /// data read-back format
        DF: 3 = enum DF {
            /// Binary unsigned. Theoretically for DIF = 0 and 12-bit mode the analog input voltage  VREF results in 0000h, the analog input voltage + VREF results in 0FFFh.
            DF_0 = 0b0,
            /// Signed binary (2s complement), left aligned. Theoretically, for DIF = 0 and 12-bit mode, the analog input voltage  VREF results in 8000h, the analog input voltage + VREF results in 7FF0h.
            DF_1 = 0b1,
        }
        /// resolution
        RES: 4..5 = enum RES {
            /// 8 bit (10 clock cycle conversion time)
            _8BIT = 0b00,
            /// 10 bit (12 clock cycle conversion time)
            _10BIT = 0b01,
            /// 12 bit (14 clock cycle conversion time)
            _12BIT = 0b10,
            /// Reserved
            RES_3 = 0b11,
        }
    }
    /// ADC12_B Control 3
    rw CTL3 @ 0x06: u16 = 0_0 {
        /// conversion start address
        CSTARTADD: 0..4 = enum CSTARTADD {
            /// Conversion start address ADC12MEM0
            MEM0 = 0b00000,
            /// Conversion start address ADC12MEM1
            MEM1 = 0b00001,
            /// Conversion start address ADC12MEM2
            MEM2 = 0b00010,
            /// Conversion start address ADC12MEM3
            MEM3 = 0b00011,
            /// Conversion start address ADC12MEM4
            MEM4 = 0b00100,
            /// Conversion start address ADC12MEM5
            MEM5 = 0b00101,
            /// Conversion start address ADC12MEM6
            MEM6 = 0b00110,
            /// Conversion start address ADC12MEM7
            MEM7 = 0b00111,
            /// Conversion start address ADC12MEM8
            MEM8 = 0b01000,
            /// Conversion start address ADC12MEM9
            MEM9 = 0b01001,
            /// Conversion start address ADC12MEM10
            MEM10 = 0b01010,
            /// Conversion start address ADC12MEM10
            MEM11 = 0b01011,
            /// Conversion start address ADC12MEM12
            MEM12 = 0b01100,
            /// Conversion start address ADC12MEM13
            MEM13 = 0b01101,
            /// Conversion start address ADC12MEM14
            MEM14 = 0b01110,
            /// Conversion start address ADC12MEM15
            MEM15 = 0b01111,
            /// Conversion start address ADC12MEM16
            MEM16 = 0b10000,
            /// Conversion start address ADC12MEM17
            MEM17 = 0b10001,
            /// Conversion start address ADC12MEM18
            MEM18 = 0b10010,
            /// Conversion start address ADC12MEM19
            MEM19 = 0b10011,
            /// Conversion start address ADC12MEM20
            MEM20 = 0b10100,
            /// Conversion start address ADC12MEM21
            MEM21 = 0b10101,
            /// Conversion start address ADC12MEM22
            MEM22 = 0b10110,
            /// Conversion start address ADC12MEM23
            MEM23 = 0b10111,
            /// Conversion start address ADC12MEM24
            MEM24 = 0b11000,
            /// Conversion start address ADC12MEM25
            MEM25 = 0b11001,
            /// Conversion start address ADC12MEM26
            MEM26 = 0b11010,
            /// Conversion start address ADC12MEM27
            MEM27 = 0b11011,
            /// Conversion start address ADC12MEM28
            MEM28 = 0b11100,
            /// Conversion start address ADC12MEM29
            MEM29 = 0b11101,
            /// Conversion start address ADC12MEM30
            MEM30 = 0b11110,
            /// Conversion start address ADC12MEM31
            MEM31 = 0b11111,
        }
        /// 1/2 AVCC ADC input channel selection
        BATMAP: 6 = enum BATMAP {
            /// external pin is selected for ADC input channel A31
            BATMAP_0 = 0b0,
            /// ADC internal 1/2 x AVCC channel is selected for ADC input channel A31
            BATMAP_1 = 0b1,
        }
        /// temperature sensor ADC input channel selection
        TCMAP: 7 = enum TCMAP {
            /// external pin is selected for ADC input channel A30
            TCMAP_0 = 0b0,
            /// ADC internal temperature sensor channel is selected for ADC input channel A30
            TCMAP_1 = 0b1,
        }
        /// int ch 0 sel to ADC in ch A29
        ICH0MAP: 8 = enum ICH0MAP {
            /// external pin is selected for ADC input channel A29
            ICH0MAP_0 = 0b0,
            /// ADC input channel internal 0 is selected for ADC input channel A29, see device-specific data sheet for availability
            ICH0MAP_1 = 0b1,
        }
        /// int ch 1 sel to ADC in ch A28
        ICH1MAP: 9 = enum ICH1MAP {
            /// external pin is selected for ADC input channel A28
            ICH1MAP_0 = 0b0,
            /// ADC input channel internal 1 is selected for ADC input channel A28, see device-specific data sheet for availability
            ICH1MAP_1 = 0b1,
        }
        /// int ch 2 sel to ADC in ch A27
        ICH2MAP: 10 = enum ICH2MAP {
            /// external pin is selected for ADC input channel A27
            ICH2MAP_0 = 0b0,
            /// ADC input channel internal 2 is selected for ADC input channel A27, see device-specific data sheet for availability
            ICH2MAP_1 = 0b1,
        }
        /// int ch 3 sel to ADC in ch A26
        ICH3MAP: 11 = enum ICH3MAP {
            /// external pin is selected for ADC input channel A26
            ICH3MAP_0 = 0b0,
            /// ADC input channel internal 3 is selected for ADC input channel A26, see device-specific data sheet for availability
            ICH3MAP_1 = 0b1,
        }
    }
    /// ADC12_B Window Comparator Low Threshold Register
    rw LO @ 0x08: u16 = 0_0 {
        /// ADC12_B Window Comparator Low Threshold Register
        LO: 0..15 = struct LOField(u16);
    }
    /// ADC12_B Window Comparator High Threshold Register
    rw HI @ 0x0a: u16 = 0_0 {
        /// ADC12_B Window Comparator High Threshold Register
        HI: 0..15 = struct HIField(u16);
    }
    /// ADC12_B Interrupt Flag 0
    rw IFGR0 @ 0x0c: u16 = 0_0 {
        /// ADC12MEM0 interrupt flag
        IFG0: 0 = enum IFG0 {
            /// No interrupt pending
            IFG0_0 = 0b0,
            /// Interrupt pending
            IFG0_1 = 0b1,
        }
        /// ADC12MEM1 interrupt flag
        IFG1: 1 = enum IFG1 {
            /// No interrupt pending
            IFG1_0 = 0b0,
            /// Interrupt pending
            IFG1_1 = 0b1,
        }
        /// ADC12MEM2 interrupt flag
        IFG2: 2 = enum IFG2 {
            /// No interrupt pending
            IFG2_0 = 0b0,
            /// Interrupt pending
            IFG2_1 = 0b1,
        }
        /// ADC12MEM3 interrupt flag
        IFG3: 3 = enum IFG3 {
            /// No interrupt pending
            IFG3_0 = 0b0,
            /// Interrupt pending
            IFG3_1 = 0b1,
        }
        /// ADC12MEM4 interrupt flag
        IFG4: 4 = enum IFG4 {
            /// No interrupt pending
            IFG4_0 = 0b0,
            /// Interrupt pending
            IFG4_1 = 0b1,
        }
        /// ADC12MEM5 interrupt flag
        IFG5: 5 = enum IFG5 {
            /// No interrupt pending
            IFG5_0 = 0b0,
            /// Interrupt pending
            IFG5_1 = 0b1,
        }
        /// ADC12MEM6 interrupt flag
        IFG6: 6 = enum IFG6 {
            /// No interrupt pending
            IFG6_0 = 0b0,
            /// Interrupt pending
            IFG6_1 = 0b1,
        }
        /// ADC12MEM7 interrupt flag
        IFG7: 7 = enum IFG7 {
            /// No interrupt pending
            IFG7_0 = 0b0,
            /// Interrupt pending
            IFG7_1 = 0b1,
        }
        /// ADC12MEM8 interrupt flag
        IFG8: 8 = enum IFG8 {
            /// No interrupt pending
            IFG8_0 = 0b0,
            /// Interrupt pending
            IFG8_1 = 0b1,
        }
        /// ADC12MEM9 interrupt flag
        IFG9: 9 = enum IFG9 {
            /// No interrupt pending
            IFG9_0 = 0b0,
            /// Interrupt pending
            IFG9_1 = 0b1,
        }
        /// ADC12MEM10 interrupt flag
        IFG10: 10 = enum IFG10 {
            /// No interrupt pending
            IFG10_0 = 0b0,
            /// Interrupt pending
            IFG10_1 = 0b1,
        }
        /// ADC12MEM11 interrupt flag
        IFG11: 11 = enum IFG11 {
            /// No interrupt pending
            IFG11_0 = 0b0,
            /// Interrupt pending
            IFG11_1 = 0b1,
        }
        /// ADC12MEM12 interrupt flag
        IFG12: 12 = enum IFG12 {
            /// No interrupt pending
            IFG12_0 = 0b0,
            /// Interrupt pending
            IFG12_1 = 0b1,
        }
        /// ADC12MEM13 interrupt flag
        IFG13: 13 = enum IFG13 {
            /// No interrupt pending
            IFG13_0 = 0b0,
            /// Interrupt pending
            IFG13_1 = 0b1,
        }
        /// ADC12MEM14 interrupt flag
        IFG14: 14 = enum IFG14 {
            /// No interrupt pending
            IFG14_0 = 0b0,
            /// Interrupt pending
            IFG14_1 = 0b1,
        }
        /// ADC12MEM15 interrupt flag
        IFG15: 15 = enum IFG15 {
            /// No interrupt pending
            IFG15_0 = 0b0,
            /// Interrupt pending
            IFG15_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Flag 1
    rw IFGR1 @ 0x0e: u16 = 0_0 {
        /// ADC12MEM16 interrupt flag
        IFG16: 0 = enum IFG16 {
            /// No interrupt pending
            IFG16_0 = 0b0,
            /// Interrupt pending
            IFG16_1 = 0b1,
        }
        /// ADC12MEM17 interrupt flag
        IFG17: 1 = enum IFG17 {
            /// No interrupt pending
            IFG17_0 = 0b0,
            /// Interrupt pending
            IFG17_1 = 0b1,
        }
        /// ADC12MEM18 interrupt flag
        IFG18: 2 = enum IFG18 {
            /// No interrupt pending
            IFG18_0 = 0b0,
            /// Interrupt pending
            IFG18_1 = 0b1,
        }
        /// ADC12MEM19 interrupt flag
        IFG19: 3 = enum IFG19 {
            /// No interrupt pending
            IFG19_0 = 0b0,
            /// Interrupt pending
            IFG19_1 = 0b1,
        }
        /// ADC12MEM20 interrupt flag
        IFG20: 4 = enum IFG20 {
            /// No interrupt pending
            IFG20_0 = 0b0,
            /// Interrupt pending
            IFG20_1 = 0b1,
        }
        /// ADC12MEM21 interrupt flag
        IFG21: 5 = enum IFG21 {
            /// No interrupt pending
            IFG21_0 = 0b0,
            /// Interrupt pending
            IFG21_1 = 0b1,
        }
        /// ADC12MEM22 interrupt flag
        IFG22: 6 = enum IFG22 {
            /// No interrupt pending
            IFG22_0 = 0b0,
            /// Interrupt pending
            IFG22_1 = 0b1,
        }
        /// ADC12MEM23 interrupt flag
        IFG23: 7 = enum IFG23 {
            /// No interrupt pending
            IFG23_0 = 0b0,
            /// Interrupt pending
            IFG23_1 = 0b1,
        }
        /// ADC12MEM24 interrupt flag
        IFG24: 8 = enum IFG24 {
            /// No interrupt pending
            IFG24_0 = 0b0,
            /// Interrupt pending
            IFG24_1 = 0b1,
        }
        /// ADC12MEM25 interrupt flag
        IFG25: 9 = enum IFG25 {
            /// No interrupt pending
            IFG25_0 = 0b0,
            /// Interrupt pending
            IFG25_1 = 0b1,
        }
        /// ADC12MEM26 interrupt flag
        IFG26: 10 = enum IFG26 {
            /// No interrupt pending
            IFG26_0 = 0b0,
            /// Interrupt pending
            IFG26_1 = 0b1,
        }
        /// ADC12MEM27 interrupt flag
        IFG27: 11 = enum IFG27 {
            /// No interrupt pending
            IFG27_0 = 0b0,
            /// Interrupt pending
            IFG27_1 = 0b1,
        }
        /// ADC12MEM28 interrupt flag
        IFG28: 12 = enum IFG28 {
            /// No interrupt pending
            IFG28_0 = 0b0,
            /// Interrupt pending
            IFG28_1 = 0b1,
        }
        /// ADC12MEM29 interrupt flag
        IFG29: 13 = enum IFG29 {
            /// No interrupt pending
            IFG29_0 = 0b0,
            /// Interrupt pending
            IFG29_1 = 0b1,
        }
        /// ADC12MEM30 interrupt flag
        IFG30: 14 = enum IFG30 {
            /// No interrupt pending
            IFG30_0 = 0b0,
            /// Interrupt pending
            IFG30_1 = 0b1,
        }
        /// ADC12MEM31 interrupt flag
        IFG31: 15 = enum IFG31 {
            /// No interrupt pending
            IFG31_0 = 0b0,
            /// Interrupt pending
            IFG31_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Flag 2
    rw IFGR2 @ 0x10: u16 = 0_0 {
        /// Interrupt flag for MEMx between ADC12HI and ADC12LO
        INIFG: 1 = enum INIFG {
            /// No interrupt pending
            INIFG_0 = 0b0,
            /// Interrupt pending
            INIFG_1 = 0b1,
        }
        /// Interrupt flag for MEMx ADC12LO
        LOIFG: 2 = enum LOIFG {
            /// No interrupt pending
            LOIFG_0 = 0b0,
            /// Interrupt pending
            LOIFG_1 = 0b1,
        }
        /// Interrupt flag for MEMx ADC12HI
        HIIFG: 3 = enum HIIFG {
            /// No interrupt pending
            HIIFG_0 = 0b0,
            /// Interrupt pending
            HIIFG_1 = 0b1,
        }
        /// ADC12MEMx overflow-interrupt flag.
        OVIFG: 4 = enum OVIFG {
            /// No interrupt pending
            OVIFG_0 = 0b0,
            /// Interrupt pending
            OVIFG_1 = 0b1,
        }
        /// conversion-time-overflow interrupt flag
        TOVIFG: 5 = enum TOVIFG {
            /// No interrupt pending
            TOVIFG_0 = 0b0,
            /// Interrupt pending
            TOVIFG_1 = 0b1,
        }
        /// reference buffer ready interrupt flag
        RDYIFG: 6 = enum RDYIFG {
            /// No interrupt pending
            RDYIFG_0 = 0b0,
            /// Interrupt pending
            RDYIFG_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Enable 0
    rw IER0 @ 0x12: u16 = 0_0 {
        /// Interrupt enable 0
        IE0: 0 = enum IE0 {
            /// Interrupt disabled
            IE0_0 = 0b0,
            /// Interrupt enabled
            IE0_1 = 0b1,
        }
        /// interrupt enable 1
        IE1: 1 = enum IE1 {
            /// Interrupt disabled
            IE1_0 = 0b0,
            /// Interrupt enabled
            IE1_1 = 0b1,
        }
        /// interrupt enable 2
        IE2: 2 = enum IE2 {
            /// Interrupt disabled
            IE2_0 = 0b0,
            /// Interrupt enabled
            IE2_1 = 0b1,
        }
        /// interrupt enable 3
        IE3: 3 = enum IE3 {
            /// Interrupt disabled
            IE3_0 = 0b0,
            /// Interrupt enabled
            IE3_1 = 0b1,
        }
        /// interrupt enable 4
        IE4: 4 = enum IE4 {
            /// Interrupt disabled
            IE4_0 = 0b0,
            /// Interrupt enabled
            IE4_1 = 0b1,
        }
        /// interrupt enable 5
        IE5: 5 = enum IE5 {
            /// Interrupt disabled
            IE5_0 = 0b0,
            /// Interrupt enabled
            IE5_1 = 0b1,
        }
        /// interrupt enable 6
        IE6: 6 = enum IE6 {
            /// Interrupt disabled
            IE6_0 = 0b0,
            /// Interrupt enabled
            IE6_1 = 0b1,
        }
        /// interrupt enable 7
        IE7: 7 = enum IE7 {
            /// Interrupt disabled
            IE7_0 = 0b0,
            /// Interrupt enabled
            IE7_1 = 0b1,
        }
        /// interrupt enable 8
        IE8: 8 = enum IE8 {
            /// Interrupt disabled
            IE8_0 = 0b0,
            /// Interrupt enabled
            IE8_1 = 0b1,
        }
        /// interrupt enable 9
        IE9: 9 = enum IE9 {
            /// Interrupt disabled
            IE9_0 = 0b0,
            /// Interrupt enabled
            IE9_1 = 0b1,
        }
        /// interrupt enable 10
        IE10: 10 = enum IE10 {
            /// Interrupt disabled
            IE10_0 = 0b0,
            /// Interrupt enabled
            IE10_1 = 0b1,
        }
        /// interrupt enable  11
        IE11: 11 = enum IE11 {
            /// Interrupt disabled
            IE11_0 = 0b0,
            /// Interrupt enabled
            IE11_1 = 0b1,
        }
        /// interrupt enable 12
        IE12: 12 = enum IE12 {
            /// Interrupt disabled
            IE12_0 = 0b0,
            /// Interrupt enabled
            IE12_1 = 0b1,
        }
        /// interrupt enable  13
        IE13: 13 = enum IE13 {
            /// Interrupt disabled
            IE13_0 = 0b0,
            /// Interrupt enabled
            IE13_1 = 0b1,
        }
        /// interrupt enable 14
        IE14: 14 = enum IE14 {
            /// Interrupt disabled
            IE14_0 = 0b0,
            /// Interrupt enabled
            IE14_1 = 0b1,
        }
        /// interrupt enable 15
        IE15: 15 = enum IE15 {
            /// Interrupt disabled
            IE15_0 = 0b0,
            /// Interrupt enabled
            IE15_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Enable 1
    rw IER1 @ 0x14: u16 = 0_0 {
        /// interrupt enable 16
        IE16: 0 = enum IE16 {
            /// Interrupt disabled
            IE16_0 = 0b0,
            /// Interrupt enabled
            IE16_1 = 0b1,
        }
        /// interrupt enable 17
        IE17: 1 = enum IE17 {
            /// Interrupt disabled
            IE17_0 = 0b0,
            /// Interrupt enabled
            IE17_1 = 0b1,
        }
        /// interrupt enable 18
        IE18: 2 = enum IE18 {
            /// Interrupt disabled
            IE18_0 = 0b0,
            /// Interrupt enabled
            IE18_1 = 0b1,
        }
        /// interrupt enable  19
        IE19: 3 = enum IE19 {
            /// Interrupt disabled
            IE19_0 = 0b0,
            /// Interrupt enabled
            IE19_1 = 0b1,
        }
        /// interrupt enable 19
        IE20: 4 = enum IE20 {
            /// Interrupt disabled
            IE20_0 = 0b0,
            /// Interrupt enabled
            IE20_1 = 0b1,
        }
        /// interrupt enable 21
        IE21: 5 = enum IE21 {
            /// Interrupt disabled
            IE21_0 = 0b0,
            /// Interrupt enabled
            IE21_1 = 0b1,
        }
        /// interrupt enable 22
        IE22: 6 = enum IE22 {
            /// Interrupt disabled
            IE22_0 = 0b0,
            /// Interrupt enabled
            IE22_1 = 0b1,
        }
        /// interrupt enable 23
        IE23: 7 = enum IE23 {
            /// Interrupt disabled
            IE23_0 = 0b0,
            /// Interrupt enabled
            IE23_1 = 0b1,
        }
        /// interrupt enable 24
        IE24: 8 = enum IE24 {
            /// Interrupt disabled
            IE24_0 = 0b0,
            /// Interrupt enabled
            IE24_1 = 0b1,
        }
        /// interrupt enable 25
        IE25: 9 = enum IE25 {
            /// Interrupt disabled
            IE25_0 = 0b0,
            /// Interrupt enabled
            IE25_1 = 0b1,
        }
        /// interrupt enable 26
        IE26: 10 = enum IE26 {
            /// Interrupt disabled
            IE26_0 = 0b0,
            /// Interrupt enabled
            IE26_1 = 0b1,
        }
        /// interrupt enable 27
        IE27: 11 = enum IE27 {
            /// Interrupt disabled
            IE27_0 = 0b0,
            /// Interrupt enabled
            IE27_1 = 0b1,
        }
        /// interrupt enable  28
        IE28: 12 = enum IE28 {
            /// Interrupt disabled
            IE28_0 = 0b0,
            /// Interrupt enabled
            IE28_1 = 0b1,
        }
        /// interrupt enable 29
        IE29: 13 = enum IE29 {
            /// Interrupt disabled
            IE29_0 = 0b0,
            /// Interrupt enabled
            IE29_1 = 0b1,
        }
        /// interrupt enable 30
        IE30: 14 = enum IE30 {
            /// Interrupt disabled
            IE30_0 = 0b0,
            /// Interrupt enabled
            IE30_1 = 0b1,
        }
        /// interrupt enable 30
        IE31: 15 = enum IE31 {
            /// Interrupt disabled
            IE31_0 = 0b0,
            /// Interrupt enabled
            IE31_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Enable 2
    rw IER2 @ 0x16: u16 = 0_0 {
        /// interrupt enable MEMx between ADC12HI and LO
        INIE: 1 = enum INIE {
            /// Interrupt disabled
            INIE_0 = 0b0,
            /// Interrupt enabled
            INIE_1 = 0b1,
        }
        /// interrupt enable MEMx  LO
        LOIE: 2 = enum LOIE {
            /// Interrupt disabled
            LOIE_0 = 0b0,
            /// Interrupt enabled
            LOIE_1 = 0b1,
        }
        /// interrupt enable MEMx  HI
        HIIE: 3 = enum HIIE {
            /// Interrupt disabled
            HIIE_0 = 0b0,
            /// Interrupt enabled
            HIIE_1 = 0b1,
        }
        /// ADC12MEMx overflow-interrupt enable
        OVIE: 4 = enum OVIE {
            /// Interrupt disabled
            OVIE_0 = 0b0,
            /// Interrupt enabled
            OVIE_1 = 0b1,
        }
        /// conversion-time-overflow interrupt enable
        TOVIE: 5 = enum TOVIE {
            /// Interrupt disabled
            TOVIE_0 = 0b0,
            /// Interrupt enabled
            TOVIE_1 = 0b1,
        }
        /// interrupt enable ADC ref buffer ready
        RDYIE: 6 = enum RDYIE {
            /// Interrupt disabled
            RDYIE_0 = 0b0,
            /// Interrupt enabled
            RDYIE_1 = 0b1,
        }
    }
    /// ADC12_B Interrupt Vector
    rw IV @ 0x18: u16 = 0_0 {
        /// interrupt vector value
        IV: 0..15 = enum IVField {
            /// Interrupt Source: No interrupt pending, Interrupt Flag: None
            NONE = 0b0000000000000000,
            /// Interrupt Source: ADC12MEMx overflow, Interrupt Flag: ADC12OVIFG, Interrupt Priority: Highest
            OVIFG = 0b0000000000000010,
            /// Interrupt Source: Conversion time overflow, Interrupt Flag: ADC12TOVIFG
            TOVIFG = 0b0000000000000100,
            /// Interrupt Source: ADC12 window high interrupt flag, Interrupt Flag: ADC12HIIFG
            HIIFG = 0b0000000000000110,
            /// Interrupt Source: ADC12 window low interrupt flag, Interrupt Flag: ADC12LOIFG
            LOIFG = 0b0000000000001000,
            /// Interrupt Source: ADC12 in-window interrupt flag, Interrupt Flag: ADC12INIFG
            INIFG = 0b0000000000001010,
            /// Interrupt Source: ADC12MEM0 interrupt flag, Interrupt Flag: ADC12IFG0
            IFG0 = 0b0000000000001100,
            /// Interrupt Source: ADC12MEM1 interrupt flag, Interrupt Flag: ADC12IFG1
            IFG1 = 0b0000000000001110,
            /// Interrupt Source: ADC12MEM2 interrupt flag, Interrupt Flag: ADC12IFG2
            IFG2 = 0b0000000000010000,
            /// Interrupt Source: ADC12MEM3 interrupt flag, Interrupt Flag: ADC12IFG3
            IFG3 = 0b0000000000010010,
            /// Interrupt Source: ADC12MEM4 interrupt flag, Interrupt Flag: ADC12IFG4
            IFG4 = 0b0000000000010100,
            /// Interrupt Source: ADC12MEM5 interrupt flag, Interrupt Flag: ADC12IFG5
            IFG5 = 0b0000000000010110,
            /// Interrupt Source: ADC12MEM6 interrupt flag, Interrupt Flag: ADC12IFG6
            IFG6 = 0b0000000000011000,
            /// Interrupt Source: ADC12MEM7 interrupt flag, Interrupt Flag: ADC12IFG7
            IFG7 = 0b0000000000011010,
            /// Interrupt Source: ADC12MEM8 interrupt flag, Interrupt Flag: ADC12IFG8
            IFG8 = 0b0000000000011100,
            /// Interrupt Source: ADC12MEM9 interrupt flag, Interrupt Flag: ADC12IFG9
            IFG9 = 0b0000000000011110,
            /// Interrupt Source: ADC12MEM10 interrupt flag, Interrupt Flag: ADC12IFG10
            IFG10 = 0b0000000000100000,
            /// Interrupt Source: ADC12MEM11 interrupt flag, Interrupt Flag: ADC12IFG11
            IFG11 = 0b0000000000100010,
            /// Interrupt Source: ADC12MEM12 interrupt flag, Interrupt Flag: ADC12IFG12
            IFG12 = 0b0000000000100100,
            /// Interrupt Source: ADC12MEM13 interrupt flag, Interrupt Flag: ADC12IFG13
            IFG13 = 0b0000000000100110,
            /// Interrupt Source: ADC12MEM14 interrupt flag, Interrupt Flag: ADC12IFG14
            IFG14 = 0b0000000000101000,
            /// Interrupt Source: ADC12MEM15 interrupt flag, Interrupt Flag: ADC12IFG15
            IFG15 = 0b0000000000101010,
            /// Interrupt Source: ADC12MEM16 interrupt flag, Interrupt Flag: ADC12IFG16
            IFG16 = 0b0000000000101100,
            /// Interrupt Source: ADC12MEM17 interrupt flag, Interrupt Flag: ADC12IFG17
            IFG17 = 0b0000000000101110,
            /// Interrupt Source: ADC12MEM18 interrupt flag, Interrupt Flag: ADC12IFG18
            IFG18 = 0b0000000000110000,
            /// Interrupt Source: ADC12MEM19 interrupt flag, Interrupt Flag: ADC12IFG19
            IFG19 = 0b0000000000110010,
            /// Interrupt Source: ADC12MEM20 interrupt flag, Interrupt Flag: ADC12IFG20
            IFG20 = 0b0000000000110100,
            /// Interrupt Source: ADC12MEM21 interrupt flag, Interrupt Flag: ADC12IFG21
            IFG21 = 0b0000000000110110,
            /// Interrupt Source: ADC12MEM22 interrupt flag, Interrupt Flag: ADC12IFG22
            IFG22 = 0b0000000000111000,
            /// Interrupt Source: ADC12MEM23 interrupt flag, Interrupt Flag: ADC12IFG23
            IFG23 = 0b0000000000111010,
            /// Interrupt Source: ADC12MEM24 interrupt flag, Interrupt Flag: ADC12IFG24
            IFG24 = 0b0000000000111100,
            /// Interrupt Source: ADC12MEM25 interrupt flag, Interrupt Flag: ADC12IFG25
            IFG25 = 0b0000000000111110,
            /// Interrupt Source: ADC12MEM26 interrupt flag, Interrupt Flag: ADC12IFG26
            IFG26 = 0b0000000001000000,
            /// Interrupt Source: ADC12MEM27 interrupt flag, Interrupt Flag: ADC12IFG27
            IFG27 = 0b0000000001000010,
            /// Interrupt Source: ADC12MEM28 interrupt flag, Interrupt Flag: ADC12IFG28
            IFG28 = 0b0000000001000100,
            /// Interrupt Source: ADC12MEM29 interrupt flag, Interrupt Flag: ADC12IFG29
            IFG29 = 0b0000000001000110,
            /// Interrupt Source: ADC12MEM30 interrupt flag, Interrupt Flag: ADC12IFG30
            IFG30 = 0b0000000001001000,
            /// Interrupt Source: ADC12MEM31 interrupt flag, Interrupt Flag: ADC12IFG31
            IFG31 = 0b0000000001001010,
            /// Interrupt Source: ADC12RDYIFG interrupt flag, Interrupt Flag: ADC12RDYIFG
            RDYIFG = 0b0000000001001100,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL0 @ 0x20: u16 = 0_0 {
        /// Input channel select
        MCTL0_INCH: 0..4 = enum MCTL0_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL0_EOS: 7 = enum MCTL0_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL0_VRSEL: 8..11 = enum MCTL0_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL0_DIF: 13 = enum MCTL0_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL0_WINC: 14 = enum MCTL0_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL1 @ 0x22: u16 = 0_0 {
        /// Input channel select
        MCTL1_INCH: 0..4 = enum MCTL1_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL1_EOS: 7 = enum MCTL1_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL1_VRSEL: 8..11 = enum MCTL1_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL1_DIF: 13 = enum MCTL1_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL1_WINC: 14 = enum MCTL1_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL2 @ 0x24: u16 = 0_0 {
        /// Input channel select
        MCTL2_INCH: 0..4 = enum MCTL2_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL2_EOS: 7 = enum MCTL2_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL2_VRSEL: 8..11 = enum MCTL2_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL2_DIF: 13 = enum MCTL2_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL2_WINC: 14 = enum MCTL2_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL3 @ 0x26: u16 = 0_0 {
        /// Input channel select
        MCTL3_INCH: 0..4 = enum MCTL3_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL3_EOS: 7 = enum MCTL3_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL3_VRSEL: 8..11 = enum MCTL3_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL3_DIF: 13 = enum MCTL3_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL3_WINC: 14 = enum MCTL3_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL4 @ 0x28: u16 = 0_0 {
        /// Input channel select
        MCTL4_INCH: 0..4 = enum MCTL4_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL4_EOS: 7 = enum MCTL4_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL4_VRSEL: 8..11 = enum MCTL4_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL4_DIF: 13 = enum MCTL4_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL4_WINC: 14 = enum MCTL4_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL5 @ 0x2a: u16 = 0_0 {
        /// Input channel select
        MCTL5_INCH: 0..4 = enum MCTL5_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL5_EOS: 7 = enum MCTL5_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL5_VRSEL: 8..11 = enum MCTL5_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL5_DIF: 13 = enum MCTL5_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL5_WINC: 14 = enum MCTL5_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL6 @ 0x2c: u16 = 0_0 {
        /// Input channel select
        MCTL6_INCH: 0..4 = enum MCTL6_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL6_EOS: 7 = enum MCTL6_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL6_VRSEL: 8..11 = enum MCTL6_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL6_DIF: 13 = enum MCTL6_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL6_WINC: 14 = enum MCTL6_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL7 @ 0x2e: u16 = 0_0 {
        /// Input channel select
        MCTL7_INCH: 0..4 = enum MCTL7_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL7_EOS: 7 = enum MCTL7_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL7_VRSEL: 8..11 = enum MCTL7_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL7_DIF: 13 = enum MCTL7_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL7_WINC: 14 = enum MCTL7_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL8 @ 0x30: u16 = 0_0 {
        /// Input channel select
        MCTL8_INCH: 0..4 = enum MCTL8_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL8_EOS: 7 = enum MCTL8_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL8_VRSEL: 8..11 = enum MCTL8_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL8_DIF: 13 = enum MCTL8_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL8_WINC: 14 = enum MCTL8_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL9 @ 0x32: u16 = 0_0 {
        /// Input channel select
        MCTL9_INCH: 0..4 = enum MCTL9_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL9_EOS: 7 = enum MCTL9_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL9_VRSEL: 8..11 = enum MCTL9_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL9_DIF: 13 = enum MCTL9_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL9_WINC: 14 = enum MCTL9_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL10 @ 0x34: u16 = 0_0 {
        /// Input channel select
        MCTL10_INCH: 0..4 = enum MCTL10_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL10_EOS: 7 = enum MCTL10_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL10_VRSEL: 8..11 = enum MCTL10_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL10_DIF: 13 = enum MCTL10_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL10_WINC: 14 = enum MCTL10_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL11 @ 0x36: u16 = 0_0 {
        /// Input channel select
        MCTL11_INCH: 0..4 = enum MCTL11_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL11_EOS: 7 = enum MCTL11_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL11_VRSEL: 8..11 = enum MCTL11_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL11_DIF: 13 = enum MCTL11_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL11_WINC: 14 = enum MCTL11_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL12 @ 0x38: u16 = 0_0 {
        /// Input channel select
        MCTL12_INCH: 0..4 = enum MCTL12_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL12_EOS: 7 = enum MCTL12_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL12_VRSEL: 8..11 = enum MCTL12_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL12_DIF: 13 = enum MCTL12_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL12_WINC: 14 = enum MCTL12_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL13 @ 0x3a: u16 = 0_0 {
        /// Input channel select
        MCTL13_INCH: 0..4 = enum MCTL13_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL13_EOS: 7 = enum MCTL13_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL13_VRSEL: 8..11 = enum MCTL13_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL13_DIF: 13 = enum MCTL13_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL13_WINC: 14 = enum MCTL13_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL14 @ 0x3c: u16 = 0_0 {
        /// Input channel select
        MCTL14_INCH: 0..4 = enum MCTL14_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL14_EOS: 7 = enum MCTL14_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL14_VRSEL: 8..11 = enum MCTL14_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL14_DIF: 13 = enum MCTL14_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL14_WINC: 14 = enum MCTL14_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL15 @ 0x3e: u16 = 0_0 {
        /// Input channel select
        MCTL15_INCH: 0..4 = enum MCTL15_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL15_EOS: 7 = enum MCTL15_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL15_VRSEL: 8..11 = enum MCTL15_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL15_DIF: 13 = enum MCTL15_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL15_WINC: 14 = enum MCTL15_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL16 @ 0x40: u16 = 0_0 {
        /// Input channel select
        MCTL16_INCH: 0..4 = enum MCTL16_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL16_EOS: 7 = enum MCTL16_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL16_VRSEL: 8..11 = enum MCTL16_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL16_DIF: 13 = enum MCTL16_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL16_WINC: 14 = enum MCTL16_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL17 @ 0x42: u16 = 0_0 {
        /// Input channel select
        MCTL17_INCH: 0..4 = enum MCTL17_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL17_EOS: 7 = enum MCTL17_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL17_VRSEL: 8..11 = enum MCTL17_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL17_DIF: 13 = enum MCTL17_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL17_WINC: 14 = enum MCTL17_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL18 @ 0x44: u16 = 0_0 {
        /// Input channel select
        MCTL18_INCH: 0..4 = enum MCTL18_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL18_EOS: 7 = enum MCTL18_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL18_VRSEL: 8..11 = enum MCTL18_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL18_DIF: 13 = enum MCTL18_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL18_WINC: 14 = enum MCTL18_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL19 @ 0x46: u16 = 0_0 {
        /// Input channel select
        MCTL19_INCH: 0..4 = enum MCTL19_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL19_EOS: 7 = enum MCTL19_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL19_VRSEL: 8..11 = enum MCTL19_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL19_DIF: 13 = enum MCTL19_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL19_WINC: 14 = enum MCTL19_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL20 @ 0x48: u16 = 0_0 {
        /// Input channel select
        MCTL20_INCH: 0..4 = enum MCTL20_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL20_EOS: 7 = enum MCTL20_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL20_VRSEL: 8..11 = enum MCTL20_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL20_DIF: 13 = enum MCTL20_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL20_WINC: 14 = enum MCTL20_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL21 @ 0x4a: u16 = 0_0 {
        /// Input channel select
        MCTL21_INCH: 0..4 = enum MCTL21_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL21_EOS: 7 = enum MCTL21_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL21_VRSEL: 8..11 = enum MCTL21_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL21_DIF: 13 = enum MCTL21_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL21_WINC: 14 = enum MCTL21_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL22 @ 0x4c: u16 = 0_0 {
        /// Input channel select
        MCTL22_INCH: 0..4 = enum MCTL22_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL22_EOS: 7 = enum MCTL22_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL22_VRSEL: 8..11 = enum MCTL22_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL22_DIF: 13 = enum MCTL22_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL22_WINC: 14 = enum MCTL22_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL23 @ 0x4e: u16 = 0_0 {
        /// Input channel select
        MCTL23_INCH: 0..4 = enum MCTL23_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL23_EOS: 7 = enum MCTL23_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL23_VRSEL: 8..11 = enum MCTL23_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL23_DIF: 13 = enum MCTL23_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL23_WINC: 14 = enum MCTL23_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL24 @ 0x50: u16 = 0_0 {
        /// Input channel select
        MCTL24_INCH: 0..4 = enum MCTL24_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL24_EOS: 7 = enum MCTL24_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL24_VRSEL: 8..11 = enum MCTL24_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL24_DIF: 13 = enum MCTL24_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL24_WINC: 14 = enum MCTL24_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL25 @ 0x52: u16 = 0_0 {
        /// Input channel select
        MCTL25_INCH: 0..4 = enum MCTL25_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL25_EOS: 7 = enum MCTL25_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL25_VRSEL: 8..11 = enum MCTL25_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL25_DIF: 13 = enum MCTL25_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL25_WINC: 14 = enum MCTL25_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL26 @ 0x54: u16 = 0_0 {
        /// Input channel select
        MCTL26_INCH: 0..4 = enum MCTL26_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL26_EOS: 7 = enum MCTL26_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL26_VRSEL: 8..11 = enum MCTL26_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL26_DIF: 13 = enum MCTL26_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL26_WINC: 14 = enum MCTL26_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL27 @ 0x56: u16 = 0_0 {
        /// Input channel select
        MCTL27_INCH: 0..4 = enum MCTL27_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL27_EOS: 7 = enum MCTL27_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL27_VRSEL: 8..11 = enum MCTL27_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL27_DIF: 13 = enum MCTL27_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL27_WINC: 14 = enum MCTL27_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL28 @ 0x58: u16 = 0_0 {
        /// Input channel select
        MCTL28_INCH: 0..4 = enum MCTL28_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL28_EOS: 7 = enum MCTL28_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL28_VRSEL: 8..11 = enum MCTL28_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL28_DIF: 13 = enum MCTL28_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL28_WINC: 14 = enum MCTL28_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL29 @ 0x5a: u16 = 0_0 {
        /// Input channel select
        MCTL29_INCH: 0..4 = enum MCTL29_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL29_EOS: 7 = enum MCTL29_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL29_VRSEL: 8..11 = enum MCTL29_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL29_DIF: 13 = enum MCTL29_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL29_WINC: 14 = enum MCTL29_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL30 @ 0x5c: u16 = 0_0 {
        /// Input channel select
        MCTL30_INCH: 0..4 = enum MCTL30_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL30_EOS: 7 = enum MCTL30_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL30_VRSEL: 8..11 = enum MCTL30_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL30_DIF: 13 = enum MCTL30_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL30_WINC: 14 = enum MCTL30_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register
    rw MCTL31 @ 0x5e: u16 = 0_0 {
        /// Input channel select
        MCTL31_INCH: 0..4 = enum MCTL31_INCH {
            /// If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_0 = 0b00000,
            /// If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1
            INCH_1 = 0b00001,
            /// If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_2 = 0b00010,
            /// If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3
            INCH_3 = 0b00011,
            /// If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_4 = 0b00100,
            /// If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5
            INCH_5 = 0b00101,
            /// If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_6 = 0b00110,
            /// If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7
            INCH_7 = 0b00111,
            /// If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_8 = 0b01000,
            /// If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9
            INCH_9 = 0b01001,
            /// If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_10 = 0b01010,
            /// If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11
            INCH_11 = 0b01011,
            /// If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_12 = 0b01100,
            /// If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13
            INCH_13 = 0b01101,
            /// If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_14 = 0b01110,
            /// If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15
            INCH_15 = 0b01111,
            /// If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_16 = 0b10000,
            /// If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17
            INCH_17 = 0b10001,
            /// If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_18 = 0b10010,
            /// If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19
            INCH_19 = 0b10011,
            /// If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_20 = 0b10100,
            /// If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21
            INCH_21 = 0b10101,
            /// If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_22 = 0b10110,
            /// If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23
            INCH_23 = 0b10111,
            /// If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_24 = 0b11000,
            /// If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25
            INCH_25 = 0b11001,
            /// If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27
            INCH_26 = 0b11010,
            /// If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27
            INCH_27 = 0b11011,
            /// If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_28 = 0b11100,
            /// If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29
            INCH_29 = 0b11101,
            /// If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_30 = 0b11110,
            /// If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31
            INCH_31 = 0b11111,
        }
        /// End of sequence
        MCTL31_EOS: 7 = enum MCTL31_EOS {
            /// Not end of sequence
            EOS_0 = 0b0,
            /// End of sequence
            EOS_1 = 0b1,
        }
        /// reference selection
        MCTL31_VRSEL: 8..11 = enum MCTL31_VRSEL {
            /// VR+ = AVCC, VR- = AVSS
            VRSEL_0 = 0b0000,
            /// VR+ = VREF buffered, VR- = AVSS
            VRSEL_1 = 0b0001,
            /// VR+ = VeREF-, VR- = AVSS
            VRSEL_2 = 0b0010,
            /// VR+ = VeREF+ buffered, VR- = AVSS
            VRSEL_3 = 0b0011,
            /// VR+ = VeREF+, VR- = AVSS
            VRSEL_4 = 0b0100,
            /// VR+ = AVCC, VR- = VeREF+ buffered
            VRSEL_5 = 0b0101,
            /// VR+ = AVCC, VR- = VeREF+
            VRSEL_6 = 0b0110,
            /// VR+ = VREF buffered, VR- = VeREF+
            VRSEL_7 = 0b0111,
            /// Reserved
            VRSEL_8 = 0b1000,
            /// VR+ = AVCC, VR- = VREF buffered
            VRSEL_9 = 0b1001,
            /// Reserved
            VRSEL_10 = 0b1010,
            /// VR+ = VeREF+, VR- = VREF buffered
            VRSEL_11 = 0b1011,
            /// VR+ = AVCC, VR- = VeREF-
            VRSEL_12 = 0b1100,
            /// VR+ = VREF buffered, VR- = VeREF-
            VRSEL_13 = 0b1101,
            /// VR+ = VeREF+, VR- = VeREF-
            VRSEL_14 = 0b1110,
            /// VR+ = VeREF+ buffered, VR- = VeREF-
            VRSEL_15 = 0b1111,
        }
        /// Differential mode.
        MCTL31_DIF: 13 = enum MCTL31_DIF {
            /// Single-ended mode enabled
            DIF_0 = 0b0,
            /// Differential mode enabled
            DIF_1 = 0b1,
        }
        /// Comparator window enable
        MCTL31_WINC: 14 = enum MCTL31_WINC {
            /// Comparator window disabled
            WINC_0 = 0b0,
            /// Comparator window enabled
            WINC_1 = 0b1,
        }
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM0 @ 0x60: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM1 @ 0x62: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM1: 0..15 = struct MEM1Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM2 @ 0x64: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM2: 0..15 = struct MEM2Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM3 @ 0x66: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM3: 0..15 = struct MEM3Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM4 @ 0x68: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM4: 0..15 = struct MEM4Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM5 @ 0x6a: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM5: 0..15 = struct MEM5Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM6 @ 0x6c: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM6: 0..15 = struct MEM6Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM7 @ 0x6e: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM7: 0..15 = struct MEM7Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM8 @ 0x70: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM8: 0..15 = struct MEM8Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM9 @ 0x72: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM9: 0..15 = struct MEM9Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM10 @ 0x74: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM10: 0..15 = struct MEM10Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM11 @ 0x76: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM11: 0..15 = struct MEM11Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM12 @ 0x78: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM12: 0..15 = struct MEM12Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM13 @ 0x7a: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM13: 0..15 = struct MEM13Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM14 @ 0x7c: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM14: 0..15 = struct MEM14Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM15 @ 0x7e: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM15: 0..15 = struct MEM15Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM16 @ 0x80: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM16: 0..15 = struct MEM16Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM17 @ 0x82: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM17: 0..15 = struct MEM17Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM18 @ 0x84: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM18: 0..15 = struct MEM18Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM19 @ 0x86: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM19: 0..15 = struct MEM19Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM20 @ 0x88: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM20: 0..15 = struct MEM20Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM21 @ 0x8a: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM21: 0..15 = struct MEM21Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM22 @ 0x8c: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM22: 0..15 = struct MEM22Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM23 @ 0x8e: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM23: 0..15 = struct MEM23Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM24 @ 0x90: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM24: 0..15 = struct MEM24Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM25 @ 0x92: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM25: 0..15 = struct MEM25Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM26 @ 0x94: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM26: 0..15 = struct MEM26Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM27 @ 0x96: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM27: 0..15 = struct MEM27Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM28 @ 0x98: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM28: 0..15 = struct MEM28Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM29 @ 0x9a: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM29: 0..15 = struct MEM29Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM30 @ 0x9c: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM30: 0..15 = struct MEM30Field(u16);
    }
    /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
    rw MEM31 @ 0x9e: u16 = 0_0 {
        /// ADC12_B Memory 0 Register to ADC12_B Memory 31 Register
        MEM31: 0..15 = struct MEM31Field(u16);
    }
}
