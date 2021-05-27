//! RTC

utils::periph! {
    /// RTC
    RTC;
    /// RTCCTL0 Register
    rw CTL @ 0x00: u16 = 0_0 {
        /// Real-time interrupt flag. This bit reports the status of a pending interrupt. This read only bit can be cleared by reading RTCIV register.
        IFG: 0 = enum IFG {
            /// No interrupt pending
            IFG_0 = 0b0,
            /// Interrupt pending
            IFG_1 = 0b1,
        }
        /// Real-time interrupt enable
        IE: 1 = enum IE {
            /// Interrupt disabled
            IE_0 = 0b0,
            /// Interrupt enabled
            IE_1 = 0b1,
        }
        /// Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect
        SR: 6 = enum SR {
            /// Write 0 has no effect
            SR_0 = 0b0,
            /// Write 1 to this bit clears the counter value and reloads the shadow register value from the modulo register at the next tick of the selected source clock. No overflow event or interrupt is generated.
            SR_1 = 0b1,
        }
        /// Real-time clock pre-divider select
        PS: 8..10 = enum PS {
            /// /1
            _1 = 0b000,
            /// /10
            _10 = 0b001,
            /// /100
            _100 = 0b010,
            /// /1000
            _1000 = 0b011,
            /// /16
            _16 = 0b100,
            /// /64
            _64 = 0b101,
            /// /256
            _256 = 0b110,
            /// /1024
            _1024 = 0b111,
        }
        /// Real-time clock source select
        SS: 12..13 = enum SS {
            /// Disabled
            DISABLED = 0b00,
            /// SMCLK
            SMCLK = 0b01,
            /// XT1CLK
            XT1CLK = 0b10,
            /// VLOCLK
            VLOCLK = 0b11,
        }
    }
    /// Real-Time Clock Interrupt Vector Register
    r IV @ 0x04: u16 = 0_0 {
        /// Real-time clock interrupt vector value
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// upt Source: RTC Counter Overflow; Interrupt Flag: RTCIFG
            IFG = 0b0000000000000010,
        }
    }
    /// RTC Counter Modulo Register
    rw MOD_ @ 0x08: u16 = 0_0 {
        /// RTC Counter Modulo Register
        MOD: 0..15 = struct MODField(u16);
    }
    /// RTC Counter Register
    rw CNT @ 0x0c: u16 = 0_0 {
        /// RTC Counter Register
        CNT: 0..15 = struct CNTField(u16);
    }
}
