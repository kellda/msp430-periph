//! DAC12

utils::periph! {
    /// DAC12
    DAC12;
    /// DAC12_0 Control
    rw DAC0CTL @ 0x00: u16 = 0_0 {
        /// DAC12 group
        DAC0GRP: 0 = struct DAC0GRP(bool);
        /// DAC12 enable conversion
        DAC0ENC: 1 = struct DAC0ENC(bool);
        /// DAC12 interrupt flag
        DAC0IFG: 2 = struct DAC0IFG(bool);
        /// DAC12 interrupt enable
        DAC0IE: 3 = struct DAC0IE(bool);
        /// DAC12 data format
        DAC0DF: 4 = struct DAC0DF(bool);
        /// DAC12 amplifier bit 0
        DAC0AMP: 5..7 = enum DAC0AMP {
            /// DAC12 amplifier 0: off
            AMP_0 = 0b000,
            /// DAC12 amplifier 1: off
            AMP_1 = 0b001,
            /// DAC12 amplifier 2: low
            AMP_2 = 0b010,
            /// DAC12 amplifier 3: low
            AMP_3 = 0b011,
            /// DAC12 amplifier 4: low
            AMP_4 = 0b100,
            /// DAC12 amplifier 5: medium
            AMP_5 = 0b101,
            /// DAC12 amplifier 6: medium
            AMP_6 = 0b110,
            /// DAC12 amplifier 7: high
            AMP_7 = 0b111,
        }
        /// DAC12 input reference and output range
        DAC0IR: 8 = struct DAC0IR(bool);
        /// DAC12 calibration
        DAC0CALON: 9 = struct DAC0CALON(bool);
        /// DAC12 load select bit 0
        DAC0LSEL: 10..11 = enum DAC0LSEL {
            /// DAC12 load select 0: direct
            LSEL_0 = 0b00,
            /// DAC12 load select 1: latched with DAT
            LSEL_1 = 0b01,
            /// DAC12 load select 2: latched with pos. Timer_A3.OUT1
            LSEL_2 = 0b10,
            /// DAC12 load select 3: latched with pos. Timer_B7.OUT1
            LSEL_3 = 0b11,
        }
        /// DAC12 resolution
        DAC0RES: 12 = struct DAC0RES(bool);
        /// DAC12 reference bit 0
        DAC0SREF: 13..14 = enum DAC0SREF {
            /// DAC12 reference 0: Vref+
            SREF_0 = 0b00,
            /// DAC12 reference 1: Vref+
            SREF_1 = 0b01,
            /// DAC12 reference 2: Veref+
            SREF_2 = 0b10,
            /// DAC12 reference 3: Veref+
            SREF_3 = 0b11,
        }
        /// DAC12 Operation Amp.
        DAC0OPS: 15 = struct DAC0OPS(bool);
    }
    /// DAC12_1 Control
    rw DAC1CTL @ 0x02: u16 = 0_0 {
        /// DAC12 group
        DAC1GRP: 0 = struct DAC1GRP(bool);
        /// DAC12 enable conversion
        DAC1ENC: 1 = struct DAC1ENC(bool);
        /// DAC12 interrupt flag
        DAC1IFG: 2 = struct DAC1IFG(bool);
        /// DAC12 interrupt enable
        DAC1IE: 3 = struct DAC1IE(bool);
        /// DAC12 data format
        DAC1DF: 4 = struct DAC1DF(bool);
        /// DAC12 amplifier bit 0
        DAC1AMP: 5..7 = enum DAC1AMP {
            /// DAC12 amplifier 0: off
            AMP_0 = 0b000,
            /// DAC12 amplifier 1: off
            AMP_1 = 0b001,
            /// DAC12 amplifier 2: low
            AMP_2 = 0b010,
            /// DAC12 amplifier 3: low
            AMP_3 = 0b011,
            /// DAC12 amplifier 4: low
            AMP_4 = 0b100,
            /// DAC12 amplifier 5: medium
            AMP_5 = 0b101,
            /// DAC12 amplifier 6: medium
            AMP_6 = 0b110,
            /// DAC12 amplifier 7: high
            AMP_7 = 0b111,
        }
        /// DAC12 input reference and output range
        DAC1IR: 8 = struct DAC1IR(bool);
        /// DAC12 calibration
        DAC1CALON: 9 = struct DAC1CALON(bool);
        /// DAC12 load select bit 0
        DAC1LSEL: 10..11 = enum DAC1LSEL {
            /// DAC12 load select 0: direct
            LSEL_0 = 0b00,
            /// DAC12 load select 1: latched with DAT
            LSEL_1 = 0b01,
            /// DAC12 load select 2: latched with pos. Timer_A3.OUT1
            LSEL_2 = 0b10,
            /// DAC12 load select 3: latched with pos. Timer_B7.OUT1
            LSEL_3 = 0b11,
        }
        /// DAC12 resolution
        DAC1RES: 12 = struct DAC1RES(bool);
        /// DAC12 reference bit 0
        DAC1SREF: 13..14 = enum DAC1SREF {
            /// DAC12 reference 0: Vref+
            SREF_0 = 0b00,
            /// DAC12 reference 1: Vref+
            SREF_1 = 0b01,
            /// DAC12 reference 2: Veref+
            SREF_2 = 0b10,
            /// DAC12 reference 3: Veref+
            SREF_3 = 0b11,
        }
        /// DAC12 Operation Amp.
        DAC1OPS: 15 = struct DAC1OPS(bool);
    }
    /// DAC12_0 Data
    rw DAC0DAT @ 0x08: u16 = 0_0 {
        /// DAC12_0 Data
        DAC0DAT: 0..15 = struct DAC0DATField(u16);
    }
    /// DAC12_1 Data
    rw DAC1DAT @ 0x0a: u16 = 0_0 {
        /// DAC12_1 Data
        DAC1DAT: 0..15 = struct DAC1DATField(u16);
    }
}
