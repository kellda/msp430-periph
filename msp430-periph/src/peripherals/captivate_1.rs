//! Captivate

utils::periph! {
    /// Captivate
    Captivate;
    /// Captivate Interrupt Enable Register
    rw CAPIE @ 0x00: u16 = 0_0 {
        /// End of conversion interrupt enable
        EOCIEN: 0 = struct EOCIEN(bool);
        /// Captivate detection interrupt enable
        CAPDTCTIEN: 1 = struct CAPDTCTIEN(bool);
        /// Captivate Timer interrupt enable
        CAPTIEN: 2 = struct CAPTIEN(bool);
        /// Captivate Conversion Counter interrupt enable
        CAPCNTRIEN: 3 = struct CAPCNTRIEN(bool);
        /// Captivate maximum count interrupt enable
        CAPMAXIEN: 8 = struct CAPMAXIEN(bool);
    }
    /// Captivate Interrupt Flag Register
    rw CAPIFG @ 0x02: u16 = 0_0 {
        /// End of conversion interrupt flag
        EOCIFG: 0 = struct EOCIFG(bool);
        /// Captivate detection interrupt flag
        CAPDTCTIFG: 1 = struct CAPDTCTIFG(bool);
        /// Captivate Timer interrupt flag
        CAPTIFG: 2 = struct CAPTIFG(bool);
        /// Captivate Conversion Counter interrupt flag
        CAPCNTRIFG: 3 = struct CAPCNTRIFG(bool);
        /// Captivate maximum count interrupt flag
        CAPMAXIFG: 8 = struct CAPMAXIFG(bool);
    }
    /// Captivate Interrupt Vector Register
    rw CAPIV @ 0x04: u16 = 0_0 {
        /// Captivate Interrupt Vector Register
        CAPIV: 0..15 = struct CAPIVField(u16);
    }
}
