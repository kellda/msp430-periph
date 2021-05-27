//! CAPTIVATE

utils::periph! {
    /// CAPTIVATE
    CAPTIVATE;
    /// Captivate Interrupt Enable Register
    rw IE @ 0x00: u16 = 0_0 {
        /// End of conversion interrupt enable
        ///
        /// When enabled, an interrupt is called when EOCIFG = 1; that is, at the end of each conversion. EOCIFG must be cleared during the interrupt service routine.
        EOCIEN: 0 = enum EOCIEN {
            /// Interrupt disabled
            EOCIEN_0 = 0b0,
            /// Interrupt enabled
            EOCIEN_1 = 0b1,
        }
        /// Captivate detection interrupt enable
        DTCTIEN: 1 = enum DTCTIEN {
            /// Interrupt disabled
            DTCTIEN_0 = 0b0,
            /// Interrupt enabled
            DTCTIEN_1 = 0b1,
        }
        /// Captivate Timer interrupt enable
        TIEN: 2 = enum TIEN {
            /// Interrupt disabled
            TIEN_0 = 0b0,
            /// Interrupt enabled
            TIEN_1 = 0b1,
        }
        /// Captivate Conversion Counter interrupt enable
        CNTRIEN: 3 = enum CNTRIEN {
            /// Interrupt disabled
            CNTRIEN_0 = 0b0,
            /// Interrupt enabled
            CNTRIEN_1 = 0b1,
        }
        /// Captivate maximum count interrupt enable
        MAXIEN: 8 = enum MAXIEN {
            /// Interrupt disabled
            MAXIEN_0 = 0b0,
            /// Interrupt enabled
            MAXIEN_1 = 0b1,
        }
    }
    /// Captivate Interrupt Flag Register
    rw IFG @ 0x02: u16 = 0_0 {
        /// End of conversion interrupt flag
        ///
        /// This bit is set by hardware when each of the enabled CRx channels has finished converting and its results are ready. This bit is cleared by hardware when a conversion is launched (when CIPF becomes 1) or when CAPPWR = 0. If EOCITEN = 1, the Captivate interrupt occurs when EOCIFG transitions to 1. EOCIFG must be cleared by software before exiting the interrupt service routine.
        EOCIFG: 0 = enum EOCIFG {
            /// No end of conversion has occurred
            EOCIFG_0 = 0b0,
            /// End of conversion has occurred
            EOCIFG_1 = 0b1,
        }
        /// Captivate detection interrupt flag
        DTCTIFG: 1 = enum DTCTIFG {
            /// No interrupt pending
            DTCTIFG_0 = 0b0,
            /// Interrupt pending
            DTCTIFG_1 = 0b1,
        }
        /// Captivate timer interrupt flag
        TIFG: 2 = enum TIFG {
            /// No interrupt pending
            TIFG_0 = 0b0,
            /// Interrupt pending
            TIFG_1 = 0b1,
        }
        /// specified number of conversion have been reached
        CNTRIFG: 3 = enum CNTRIFG {
            /// No interrupt pending
            CNTRIFG_0 = 0b0,
            /// Interrupt pending
            CNTRIFG_1 = 0b1,
        }
        /// Captivate maximum count interrupt flag
        MAXIFG: 8 = enum MAXIFG {
            /// Maximum count not reached
            MAXIFG_0 = 0b0,
            /// Maximum count reached
            MAXIFG_1 = 0b1,
        }
    }
    /// Captivate Interrupt Vector Register
    r IV @ 0x04: u16 = 0_0 {
        /// Captivate Interrupt vector value
        ///
        /// It generates an value that can be used as address offset for fast interrupt service routine handling. 000Ch to FFFEh = Reserved Read will clear highest priority interrupt. Write will clear all pending interrupts.
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            IV_0 = 0b0000000000000000,
            /// Interrupt source: End of conversion interrupt, Flag = EOCIFG
            IV_2 = 0b0000000000000010,
            /// Interrupt source: Detection interrupt, Flag = CAPDTCTIFG
            IV_4 = 0b0000000000000100,
            /// Interrupt source: Captivate Timer interrupt, Flag = CAPTIFG
            IV_6 = 0b0000000000000110,
            /// Interrupt source: Captivate Counter interrupt, Flag = CAPCNTRIFG
            IV_8 = 0b0000000000001000,
            /// Interrupt source: max count value reached, Flag = CAPMAXIFG
            IV_10 = 0b0000000000001010,
        }
    }
}
