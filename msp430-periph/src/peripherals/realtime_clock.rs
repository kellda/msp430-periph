//! Real-Time Clock

utils::periph! {
    /// Real-Time Clock
    RealTimeClock;
    /// RTC control Register
    rw RTCCTL @ 0x00: u16 = 0_0 {
        /// Low-Power-Counter Interrupt Flag
        RTCIF: 0 = struct RTCIF(bool);
        /// Low-Power-Counter Interrupt Enable
        RTCIE: 1 = struct RTCIE(bool);
        /// Low-Power-Counter Software Reset
        RTCSR: 6 = struct RTCSR(bool);
        /// Low-Power-Counter Clock Pre-divider Select Bit: 0
        RTCPS: 8..10 = enum RTCPS {
            /// Low-Power-Counter Clock Pre-divider Select: 0
            RTCPS_0 = 0b000,
            /// Low-Power-Counter Clock Pre-divider Select: 1
            RTCPS_1 = 0b001,
            /// Low-Power-Counter Clock Pre-divider Select: 2
            RTCPS_2 = 0b010,
            /// Low-Power-Counter Clock Pre-divider Select: 3
            RTCPS_3 = 0b011,
            /// Low-Power-Counter Clock Pre-divider Select: 4
            RTCPS_4 = 0b100,
            /// Low-Power-Counter Clock Pre-divider Select: 5
            RTCPS_5 = 0b101,
            /// Low-Power-Counter Clock Pre-divider Select: 6
            RTCPS_6 = 0b110,
            /// Low-Power-Counter Clock Pre-divider Select: 7
            RTCPS_7 = 0b111,
        }
        /// Low-Power-Counter Clock Source Select Bit: 0
        RTCSS: 12..13 = enum RTCSS {
            /// Low-Power-Counter Clock Source Select: 0
            RTCSS_0 = 0b00,
            /// Low-Power-Counter Clock Source Select: 1
            RTCSS_1 = 0b01,
            /// Low-Power-Counter Clock Source Select: 2
            RTCSS_2 = 0b10,
            /// Low-Power-Counter Clock Source Select: 3
            RTCSS_3 = 0b11,
        }
    }
    /// RTC interrupt vector
    rw RTCIV @ 0x04: u16 = 0_0 {
        /// RTC interrupt vector
        RTCIV: 0..15 = struct RTCIVField(u16);
    }
    /// RTC moduloRegister
    rw RTCMOD @ 0x08: u16 = 0_0 {
        /// RTC moduloRegister
        RTCMOD: 0..15 = struct RTCMODField(u16);
    }
    /// RTC counter Register
    rw RTCCNT @ 0x0c: u16 = 0_0 {
        /// RTC counter Register
        RTCCNT: 0..15 = struct RTCCNTField(u16);
    }
}
