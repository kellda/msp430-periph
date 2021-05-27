//! CAPTIO

utils::periph! {
    /// CAPTIO
    CAPTIO;
    /// Capacitive Touch IO 0 Control Register
    rw CTL @ 0x00: u16 = 0_0 {
        /// Capacitive Touch IO pin select
        PISEL: 1..3 = enum PISEL {
            /// Px.0
            PISEL_0 = 0b000,
            /// Px.1
            PISEL_1 = 0b001,
            /// Px.2
            PISEL_2 = 0b010,
            /// Px.3
            PISEL_3 = 0b011,
            /// Px.4
            PISEL_4 = 0b100,
            /// Px.5
            PISEL_5 = 0b101,
            /// Px.6
            PISEL_6 = 0b110,
            /// Px.7
            PISEL_7 = 0b111,
        }
        /// Capacitive Touch IO port select
        POSEL: 4..7 = enum POSEL {
            /// Px = PJ
            PJ = 0b0000,
            /// Px = P1
            P1 = 0b0001,
            /// Px = P2
            P2 = 0b0010,
            /// Px = P3
            P3 = 0b0011,
            /// Px = P4
            P4 = 0b0100,
            /// Px = P5
            P5 = 0b0101,
            /// Px = P6
            P6 = 0b0110,
            /// Px = P7
            P7 = 0b0111,
            /// Px = P8
            P8 = 0b1000,
            /// Px = P9
            P9 = 0b1001,
            /// Px = P10
            P10 = 0b1010,
            /// Px = P11
            P11 = 0b1011,
            /// Px = P12
            P12 = 0b1100,
            /// Px = P13
            P13 = 0b1101,
            /// Px = P14
            P14 = 0b1110,
            /// Px = P15
            P15 = 0b1111,
        }
        /// Capacitive Touch IO enable
        EN: 8 = enum EN {
            /// All Capacitive Touch IOs are disabled. Signal towards timers is 0.
            OFF = 0b0,
            /// Selected Capacitive Touch IO is enabled
            ON = 0b1,
        }
        /// Capacitive Touch IO state
        CAPTIO: 9 = enum CAPTIOField {
            /// Curent state 0 or Capacitive Touch IO is disabled
            CAPTIO_0 = 0b0,
            /// Current state 1
            CAPTIO_1 = 0b1,
        }
    }
}
