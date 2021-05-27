//! ADC12+2 A-to-D Converter

utils::periph! {
    /// ADC12+2 A-to-D Converter
    ADC;
    /// ADC Input
    rw IN_ @ 0x00: u16 = 0_0 {
        /// ADC Input
        IN: 0..15 = struct INField(u16);
    }
    /// ADC Input Enable
    rw EN @ 0x02: u16 = 0_0 {
        /// ADC Input Enable
        EN: 0..15 = struct ENField(u16);
    }
    /// ADC Control
    rw CTL @ 0x04: u16 = 0_0 {
        /// ADSOC
        SOC: 0 = struct SOC(bool);
        /// ADSVCC
        SVCC: 1 = struct SVCC(bool);
        /// ADIN0
        IN0: 2 = struct IN0(bool);
        /// ADIN1
        IN1: 3 = struct IN1(bool);
        /// ADIN2
        IN2: 4 = struct IN2(bool);
        /// ADINOFF
        INOFF: 5 = struct INOFF(bool);
        /// ADCSRC0
        CSRC0: 6 = struct CSRC0(bool);
        /// ADCSRC1
        CSRC1: 7 = struct CSRC1(bool);
        /// ADCSRCOFF
        CSRCOFF: 8 = struct CSRCOFF(bool);
        /// ADRNG0
        RNG0: 9 = struct RNG0(bool);
        /// ADRNG1
        RNG1: 10 = struct RNG1(bool);
        /// ADAUTO
        AUTO: 11 = struct AUTO(bool);
        /// ADPD
        PD: 12 = struct PD(bool);
    }
    /// ADC Data
    rw DAT @ 0x08: u16 = 0_0 {
        /// ADC Data
        DAT: 0..15 = struct DATField(u16);
    }
}
