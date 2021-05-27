//! Real-Time Clock

utils::periph! {
    /// Real-Time Clock
    RealTimeClock;
    /// RTC control Register
    rw CTL @ 0x00: u16 = 0_0 {
        /// Low-Power-Counter Interrupt Flag
        IF: 0 = struct IF(bool);
        /// Low-Power-Counter Interrupt Enable
        IE: 1 = struct IE(bool);
        /// Low-Power-Counter Software Reset
        SR: 6 = struct SR(bool);
        /// Low-Power-Counter Clock Pre-divider Select Bit: 0
        PS: 8..10 = enum PS {
            /// Low-Power-Counter Clock Pre-divider Select: 0
            PS_0 = 0b000,
            /// Low-Power-Counter Clock Pre-divider Select: 1
            PS_1 = 0b001,
            /// Low-Power-Counter Clock Pre-divider Select: 2
            PS_2 = 0b010,
            /// Low-Power-Counter Clock Pre-divider Select: 3
            PS_3 = 0b011,
            /// Low-Power-Counter Clock Pre-divider Select: 4
            PS_4 = 0b100,
            /// Low-Power-Counter Clock Pre-divider Select: 5
            PS_5 = 0b101,
            /// Low-Power-Counter Clock Pre-divider Select: 6
            PS_6 = 0b110,
            /// Low-Power-Counter Clock Pre-divider Select: 7
            PS_7 = 0b111,
        }
        /// Low-Power-Counter Clock Source Select Bit: 0
        SS: 12..13 = enum SS {
            /// Low-Power-Counter Clock Source Select: 0
            SS_0 = 0b00,
            /// Low-Power-Counter Clock Source Select: 1
            SS_1 = 0b01,
            /// Low-Power-Counter Clock Source Select: 2
            SS_2 = 0b10,
            /// Low-Power-Counter Clock Source Select: 3
            SS_3 = 0b11,
        }
    }
    /// RTC interrupt vector
    rw IV @ 0x04: u16 = 0_0 {
        /// RTC interrupt vector
        IV: 0..15 = struct IVField(u16);
    }
    /// RTC moduloRegister
    rw MOD_ @ 0x08: u16 = 0_0 {
        /// RTC moduloRegister
        MOD: 0..15 = struct MODField(u16);
    }
    /// RTC counter Register
    rw CNT @ 0x0c: u16 = 0_0 {
        /// RTC counter Register
        CNT: 0..15 = struct CNTField(u16);
    }
}
