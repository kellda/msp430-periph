//! ADC

utils::periph! {
    /// ADC
    ADC;
    /// ADC Control 0
    rw ADCCTL0 @ 0x00: u16 = 0_0 {
        /// ADC Start Conversion
        ADCSC: 0 = struct ADCSC(bool);
        /// ADC Enable Conversion
        ADCENC: 1 = struct ADCENC(bool);
        /// ADC On/enable
        ADCON: 4 = struct ADCON(bool);
        /// ADC Multiple SampleConversion
        ADCMSC: 7 = struct ADCMSC(bool);
        /// ADC Sample Hold Select Bit: 0
        ADCSHT: 8..11 = enum ADCSHT {
            /// ADC Sample Hold Select 0
            ADCSHT_0 = 0b0000,
            /// ADC Sample Hold Select 1
            ADCSHT_1 = 0b0001,
            /// ADC Sample Hold Select 2
            ADCSHT_2 = 0b0010,
            /// ADC Sample Hold Select 3
            ADCSHT_3 = 0b0011,
            /// ADC Sample Hold Select 4
            ADCSHT_4 = 0b0100,
            /// ADC Sample Hold Select 5
            ADCSHT_5 = 0b0101,
            /// ADC Sample Hold Select 6
            ADCSHT_6 = 0b0110,
            /// ADC Sample Hold Select 7
            ADCSHT_7 = 0b0111,
            /// ADC Sample Hold Select 8
            ADCSHT_8 = 0b1000,
            /// ADC Sample Hold Select 9
            ADCSHT_9 = 0b1001,
            /// ADC Sample Hold Select 10
            ADCSHT_10 = 0b1010,
            /// ADC Sample Hold Select 11
            ADCSHT_11 = 0b1011,
            /// ADC Sample Hold Select 12
            ADCSHT_12 = 0b1100,
            /// ADC Sample Hold Select 13
            ADCSHT_13 = 0b1101,
            /// ADC Sample Hold Select 14
            ADCSHT_14 = 0b1110,
            /// ADC Sample Hold Select 15
            ADCSHT_15 = 0b1111,
        }
    }
    /// ADC Control 1
    rw ADCCTL1 @ 0x02: u16 = 0_0 {
        /// ADC Busy
        ADCBUSY: 0 = struct ADCBUSY(bool);
        /// ADC Conversion Sequence Select 0
        ADCCONSEQ: 1..2 = enum ADCCONSEQ {
            /// ADC Conversion Sequence Select: 0
            ADCCONSEQ_0 = 0b00,
            /// ADC Conversion Sequence Select: 1
            ADCCONSEQ_1 = 0b01,
            /// ADC Conversion Sequence Select: 2
            ADCCONSEQ_2 = 0b10,
            /// ADC Conversion Sequence Select: 3
            ADCCONSEQ_3 = 0b11,
        }
        /// ADC Clock Source Select 0
        ADCSSEL: 3..4 = enum ADCSSEL {
            /// ADC Clock Source Select: 0
            ADCSSEL_0 = 0b00,
            /// ADC Clock Source Select: 1
            ADCSSEL_1 = 0b01,
            /// ADC Clock Source Select: 2
            ADCSSEL_2 = 0b10,
            /// ADC Clock Source Select: 3
            ADCSSEL_3 = 0b11,
        }
        /// ADC Clock Divider Select 0
        ADCDIV: 5..7 = enum ADCDIV {
            /// ADC Clock Divider Select: 0
            ADCDIV_0 = 0b000,
            /// ADC Clock Divider Select: 1
            ADCDIV_1 = 0b001,
            /// ADC Clock Divider Select: 2
            ADCDIV_2 = 0b010,
            /// ADC Clock Divider Select: 3
            ADCDIV_3 = 0b011,
            /// ADC Clock Divider Select: 4
            ADCDIV_4 = 0b100,
            /// ADC Clock Divider Select: 5
            ADCDIV_5 = 0b101,
            /// ADC Clock Divider Select: 6
            ADCDIV_6 = 0b110,
            /// ADC Clock Divider Select: 7
            ADCDIV_7 = 0b111,
        }
        /// ADC Invert Sample Hold Signal
        ADCISSH: 8 = struct ADCISSH(bool);
        /// ADC Sample/Hold Pulse Mode
        ADCSHP: 9 = struct ADCSHP(bool);
        /// ADC Sample/Hold Source 0
        ADCSHS: 10..11 = enum ADCSHS {
            /// ADC Sample/Hold Source: 0
            ADCSHS_0 = 0b00,
            /// ADC Sample/Hold Source: 1
            ADCSHS_1 = 0b01,
            /// ADC Sample/Hold Source: 2
            ADCSHS_2 = 0b10,
            /// ADC Sample/Hold Source: 3
            ADCSHS_3 = 0b11,
        }
    }
    /// ADC Control 2
    rw ADCCTL2 @ 0x04: u16 = 0_0 {
        /// ADC Sampling Rate
        ADCSR: 2 = struct ADCSR(bool);
        /// ADC Data Format
        ADCDF: 3 = struct ADCDF(bool);
        /// ADC Resolution
        ADCRES: 4..5 = enum ADCRES {
            /// 8 bit
            ADCRES_0 = 0b00,
            /// 10 bit
            ADCRES_1 = 0b01,
            /// Reserved
            ADCRES_2 = 0b10,
            /// Reserved
            ADCRES_3 = 0b11,
        }
        /// ADC predivider Bit: 0
        ADCPDIV: 8..9 = enum ADCPDIV {
            /// ADC predivider /1
            ADCPDIV_0 = 0b00,
            /// ADC predivider /2
            ADCPDIV_1 = 0b01,
            /// ADC predivider /64
            ADCPDIV_2 = 0b10,
            /// ADC predivider reserved
            ADCPDIV_3 = 0b11,
        }
    }
    /// ADC Window Comparator High Threshold
    rw ADCLO @ 0x06: u16 = 0_0 {
        /// ADC Window Comparator High Threshold
        ADCLO: 0..15 = struct ADCLOField(u16);
    }
    /// ADC Window Comparator High Threshold
    rw ADCHI @ 0x08: u16 = 0_0 {
        /// ADC Window Comparator High Threshold
        ADCHI: 0..15 = struct ADCHIField(u16);
    }
    /// ADC Memory Control 0
    rw ADCMCTL0 @ 0x0a: u16 = 0_0 {
        /// ADC Input Channel Select Bit 0
        ADCINCH: 0..3 = enum ADCINCH {
            /// ADC Input Channel 0
            ADCINCH_0 = 0b0000,
            /// ADC Input Channel 1
            ADCINCH_1 = 0b0001,
            /// ADC Input Channel 2
            ADCINCH_2 = 0b0010,
            /// ADC Input Channel 3
            ADCINCH_3 = 0b0011,
            /// ADC Input Channel 4
            ADCINCH_4 = 0b0100,
            /// ADC Input Channel 5
            ADCINCH_5 = 0b0101,
            /// ADC Input Channel 6
            ADCINCH_6 = 0b0110,
            /// ADC Input Channel 7
            ADCINCH_7 = 0b0111,
            /// ADC Input Channel 8
            ADCINCH_8 = 0b1000,
            /// ADC Input Channel 9
            ADCINCH_9 = 0b1001,
            /// ADC Input Channel 10
            ADCINCH_10 = 0b1010,
            /// ADC Input Channel 11
            ADCINCH_11 = 0b1011,
            /// ADC Input Channel 12
            ADCINCH_12 = 0b1100,
            /// ADC Input Channel 13
            ADCINCH_13 = 0b1101,
            /// ADC Input Channel 14
            ADCINCH_14 = 0b1110,
            /// ADC Input Channel 15
            ADCINCH_15 = 0b1111,
        }
        /// ADC Select Reference Bit 0
        ADCSREF: 4..6 = enum ADCSREF {
            /// ADC Select Reference 0
            ADCSREF_0 = 0b000,
            /// ADC Select Reference 1
            ADCSREF_1 = 0b001,
            /// ADC Select Reference 2
            ADCSREF_2 = 0b010,
            /// ADC Select Reference 3
            ADCSREF_3 = 0b011,
            /// ADC Select Reference 4
            ADCSREF_4 = 0b100,
            /// ADC Select Reference 5
            ADCSREF_5 = 0b101,
            /// ADC Select Reference 6
            ADCSREF_6 = 0b110,
            /// ADC Select Reference 7
            ADCSREF_7 = 0b111,
        }
    }
    /// ADC Conversion Memory 0
    rw ADCMEM0 @ 0x12: u16 = 0_0 {
        /// ADC Conversion Memory 0
        ADCMEM0: 0..15 = struct ADCMEM0Field(u16);
    }
    /// ADC Interrupt Enable
    rw ADCIE @ 0x1a: u16 = 0_0 {
        /// ADC Interrupt enable
        ADCIE0: 0 = struct ADCIE0(bool);
        /// ADC Interrupt enable for the inside of window of the Window comparator
        ADCINIE: 1 = struct ADCINIE(bool);
        /// ADC Interrupt enable for lower threshold of the Window comparator
        ADCLOIE: 2 = struct ADCLOIE(bool);
        /// ADC Interrupt enable for upper threshold of the Window comparator
        ADCHIIE: 3 = struct ADCHIIE(bool);
        /// ADC ADCMEM overflow Interrupt enable
        ADCOVIE: 4 = struct ADCOVIE(bool);
        /// ADC conversion-time-overflow Interrupt enable
        ADCTOVIE: 5 = struct ADCTOVIE(bool);
    }
    /// ADC Interrupt Flag
    rw ADCIFG @ 0x1c: u16 = 0_0 {
        /// ADC Interrupt Flag
        ADCIFG0: 0 = struct ADCIFG0(bool);
        /// ADC Interrupt Flag for the inside of window of the Window comparator
        ADCINIFG: 1 = struct ADCINIFG(bool);
        /// ADC Interrupt Flag for lower threshold of the Window comparator
        ADCLOIFG: 2 = struct ADCLOIFG(bool);
        /// ADC Interrupt Flag for upper threshold of the Window comparator
        ADCHIIFG: 3 = struct ADCHIIFG(bool);
        /// ADC ADCMEM overflow Interrupt Flag
        ADCOVIFG: 4 = struct ADCOVIFG(bool);
        /// ADC conversion-time-overflow Interrupt Flag
        ADCTOVIFG: 5 = struct ADCTOVIFG(bool);
    }
    /// ADC Interrupt Vector Word
    rw ADCIV @ 0x1e: u16 = 0_0 {
        /// ADC Interrupt Vector Word
        ADCIV: 0..15 = struct ADCIVField(u16);
    }
}
