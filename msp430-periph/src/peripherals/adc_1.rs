//! ADC

utils::periph! {
    /// ADC
    ADC;
    /// ADC Input
    rw AIN @ 0x00: u16 = 0_0 {
        /// ADC Input
        AIN: 0..15 = struct AINField(u16);
    }
    /// ADC Input Enable
    rw AEN @ 0x02: u16 = 0_0 {
        /// ADC Input Enable
        AEN: 0..15 = struct AENField(u16);
    }
    /// ADC Control
    rw ACTL @ 0x04: u16 = 0_0 {
        /// ADSOC
        ADSOC: 0 = struct ADSOC(bool);
        /// ADSVCC
        ADSVCC: 1 = struct ADSVCC(bool);
        /// ADIN0
        ADIN0: 2 = struct ADIN0(bool);
        /// ADIN1
        ADIN1: 3 = struct ADIN1(bool);
        /// ADIN2
        ADIN2: 4 = struct ADIN2(bool);
        /// ADINOFF
        ADINOFF: 5 = struct ADINOFF(bool);
        /// ADCSRC0
        ADCSRC0: 6 = struct ADCSRC0(bool);
        /// ADCSRC1
        ADCSRC1: 7 = struct ADCSRC1(bool);
        /// ADCSRCOFF
        ADCSRCOFF: 8 = struct ADCSRCOFF(bool);
        /// ADRNG0
        ADRNG0: 9 = struct ADRNG0(bool);
        /// ADRNG1
        ADRNG1: 10 = struct ADRNG1(bool);
        /// ADAUTO
        ADAUTO: 11 = struct ADAUTO(bool);
        /// ADPD
        ADPD: 12 = struct ADPD(bool);
    }
    /// ADC Data
    rw ADAT @ 0x08: u16 = 0_0 {
        /// ADC Data
        ADAT: 0..15 = struct ADATField(u16);
    }
}
