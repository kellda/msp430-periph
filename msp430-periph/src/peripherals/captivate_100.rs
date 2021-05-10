//! CAPTIVATE

utils::periph! {
    /// CAPTIVATE
    CAPTIVATE;
    /// CapTIvate Interrupt Enable Register
    rw CAPIE @ 0x00: u16 = 0_0 {
        /// End of conversion interrupt enable When enabled, an interrupt is called when EOCIFG = 1; that is, at the end of each conversion. EOCIFG must be cleared during the interrupt service routine.
        EOCIEN: 0 = enum EOCIEN {
            /// Interrupt disabled
            EOCIEN_0 = 0b0,
            /// Interrupt enabled
            EOCIEN_1 = 0b1,
        }
        /// CapTIvate detection interrupt enable
        CAPDTCTIEN: 1 = enum CAPDTCTIEN {
            /// Interrupt disabled
            CAPDTCTIEN_0 = 0b0,
            /// Interrupt enabled
            CAPDTCTIEN_1 = 0b1,
        }
        /// CapTIvate Timer interrupt enable
        CAPTIEN: 2 = enum CAPTIEN {
            /// Interrupt disabled
            CAPTIEN_0 = 0b0,
            /// Interrupt enabled
            CAPTIEN_1 = 0b1,
        }
        /// CapTIvate Conversion Counter interrupt enable
        CAPCNTRIEN: 3 = enum CAPCNTRIEN {
            /// Interrupt disabled
            CAPCNTRIEN_0 = 0b0,
            /// Interrupt enabled
            CAPCNTRIEN_1 = 0b1,
        }
        /// CapTIvate maximum count interrupt enable
        CAPMAXIEN: 8 = enum CAPMAXIEN {
            /// Interrupt disabled
            CAPMAXIEN_0 = 0b0,
            /// Interrupt enabled
            CAPMAXIEN_1 = 0b1,
        }
    }
    /// CapTIvate Interrupt Flag Register
    rw CAPIFG @ 0x02: u16 = 0_0 {
        /// End of conversion interrupt flag This bit is set by hardware when each of the enabled CRx channels has finished converting and its results are ready. This bit is cleared by hardware when a conversion is launched (when CIPF becomes 1) or when CAPPWR = 0. If EOCITEN = 1, the CapTIvate interrupt occurs when EOCIFG transitions to 1. EOCIFG must be cleared by software before exiting the interrupt service routine.
        EOCIFG: 0 = enum EOCIFG {
            /// No end of conversion has occurred
            EOCIFG_0 = 0b0,
            /// End of conversion has occurred
            EOCIFG_1 = 0b1,
        }
        /// CapTIvate detection interrupt flag
        CAPDTCTIFG: 1 = enum CAPDTCTIFG {
            /// No interrupt pending
            CAPDTCTIFG_0 = 0b0,
            /// Interrupt pending
            CAPDTCTIFG_1 = 0b1,
        }
        /// CapTIvate timer interrupt flag
        CAPTIFG: 2 = enum CAPTIFG {
            /// No interrupt pending
            CAPTIFG_0 = 0b0,
            /// Interrupt pending
            CAPTIFG_1 = 0b1,
        }
        /// specified number of conversion have been reached
        CAPCNTRIFG: 3 = enum CAPCNTRIFG {
            /// No interrupt pending
            CAPCNTRIFG_0 = 0b0,
            /// Interrupt pending
            CAPCNTRIFG_1 = 0b1,
        }
        /// CapTIvate maximum count interrupt flag
        CAPMAXIFG: 8 = enum CAPMAXIFG {
            /// Maximum count not reached
            CAPMAXIFG_0 = 0b0,
            /// Maximum count reached
            CAPMAXIFG_1 = 0b1,
        }
    }
    /// CapTIvate Interrupt Vector Register
    r CAPIV @ 0x04: u16 = 0_0 {
        /// CapTIvate Interrupt vector value. Reports a value that can be used as address offset for fast interrupt service routine handling. A read clears the highest priority interrupt. A write clears all pending interrupts. 000Ch to FFFEh = Reserved
        CAPIV: 0..15 = enum CAPIVField {
            /// No interrupt pending
            CAPIV_0 = 0b0000000000000000,
            /// Interrupt source: End of conversion interrupt, Flag = EOCIFG
            CAPIV_2 = 0b0000000000000010,
            /// Interrupt source: Detection interrupt, Flag = CAPDTCTIFG
            CAPIV_4 = 0b0000000000000100,
            /// Interrupt source: CapTIvate Timer interrupt, Flag = CAPTIFG
            CAPIV_6 = 0b0000000000000110,
            /// Interrupt source: CapTIvate Counter interrupt, Flag = CAPCNTRIFG
            CAPIV_8 = 0b0000000000001000,
            /// Interrupt source: max count value reached, Flag = CAPMAXIFG
            CAPIV_10 = 0b0000000000001010,
        }
    }
}
