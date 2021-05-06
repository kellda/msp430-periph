//! DAC12

utils::periph! {
    /// DAC12
    DAC12;
    /// DAC12_0 Control
    rw DAC12_0CTL @ 0x00: u16 = 0_0 {
        /// DAC12 group
        DAC12_0CTL_DAC12GRP: 0 = struct DAC12_0CTL_DAC12GRP(bool);
        /// DAC12 enable conversion
        DAC12_0CTL_DAC12ENC: 1 = struct DAC12_0CTL_DAC12ENC(bool);
        /// DAC12 interrupt flag
        DAC12_0CTL_DAC12IFG: 2 = struct DAC12_0CTL_DAC12IFG(bool);
        /// DAC12 interrupt enable
        DAC12_0CTL_DAC12IE: 3 = struct DAC12_0CTL_DAC12IE(bool);
        /// DAC12 data format
        DAC12_0CTL_DAC12DF: 4 = struct DAC12_0CTL_DAC12DF(bool);
        /// DAC12 amplifier bit 0
        DAC12_0CTL_DAC12AMP: 5..7 = enum DAC12_0CTL_DAC12AMP {
            /// DAC12 amplifier 0: off
            DAC12AMP_0 = 0b000,
            /// DAC12 amplifier 1: off
            DAC12AMP_1 = 0b001,
            /// DAC12 amplifier 2: low
            DAC12AMP_2 = 0b010,
            /// DAC12 amplifier 3: low
            DAC12AMP_3 = 0b011,
            /// DAC12 amplifier 4: low
            DAC12AMP_4 = 0b100,
            /// DAC12 amplifier 5: medium
            DAC12AMP_5 = 0b101,
            /// DAC12 amplifier 6: medium
            DAC12AMP_6 = 0b110,
            /// DAC12 amplifier 7: high
            DAC12AMP_7 = 0b111,
        }
        /// DAC12 input reference and output range
        DAC12_0CTL_DAC12IR: 8 = struct DAC12_0CTL_DAC12IR(bool);
        /// DAC12 calibration
        DAC12_0CTL_DAC12CALON: 9 = struct DAC12_0CTL_DAC12CALON(bool);
        /// DAC12 load select bit 0
        DAC12_0CTL_DAC12LSEL: 10..11 = enum DAC12_0CTL_DAC12LSEL {
            /// DAC12 load select 0: direct
            DAC12LSEL_0 = 0b00,
            /// DAC12 load select 1: latched with DAT
            DAC12LSEL_1 = 0b01,
            /// DAC12 load select 2: latched with pos. Timer_A3.OUT1
            DAC12LSEL_2 = 0b10,
            /// DAC12 load select 3: latched with pos. Timer_B7.OUT1
            DAC12LSEL_3 = 0b11,
        }
        /// DAC12 resolution
        DAC12_0CTL_DAC12RES: 12 = struct DAC12_0CTL_DAC12RES(bool);
        /// DAC12 reference bit 0
        DAC12_0CTL_DAC12SREF: 13..14 = enum DAC12_0CTL_DAC12SREF {
            /// DAC12 reference 0: Vref+
            DAC12SREF_0 = 0b00,
            /// DAC12 reference 1: Vref+
            DAC12SREF_1 = 0b01,
            /// DAC12 reference 2: Veref+
            DAC12SREF_2 = 0b10,
            /// DAC12 reference 3: Veref+
            DAC12SREF_3 = 0b11,
        }
    }
    /// DAC12_1 Control
    rw DAC12_1CTL @ 0x02: u16 = 0_0 {
        /// DAC12 group
        DAC12_1CTL_DAC12GRP: 0 = struct DAC12_1CTL_DAC12GRP(bool);
        /// DAC12 enable conversion
        DAC12_1CTL_DAC12ENC: 1 = struct DAC12_1CTL_DAC12ENC(bool);
        /// DAC12 interrupt flag
        DAC12_1CTL_DAC12IFG: 2 = struct DAC12_1CTL_DAC12IFG(bool);
        /// DAC12 interrupt enable
        DAC12_1CTL_DAC12IE: 3 = struct DAC12_1CTL_DAC12IE(bool);
        /// DAC12 data format
        DAC12_1CTL_DAC12DF: 4 = struct DAC12_1CTL_DAC12DF(bool);
        /// DAC12 amplifier bit 0
        DAC12_1CTL_DAC12AMP: 5..7 = enum DAC12_1CTL_DAC12AMP {
            /// DAC12 amplifier 0: off
            DAC12AMP_0 = 0b000,
            /// DAC12 amplifier 1: off
            DAC12AMP_1 = 0b001,
            /// DAC12 amplifier 2: low
            DAC12AMP_2 = 0b010,
            /// DAC12 amplifier 3: low
            DAC12AMP_3 = 0b011,
            /// DAC12 amplifier 4: low
            DAC12AMP_4 = 0b100,
            /// DAC12 amplifier 5: medium
            DAC12AMP_5 = 0b101,
            /// DAC12 amplifier 6: medium
            DAC12AMP_6 = 0b110,
            /// DAC12 amplifier 7: high
            DAC12AMP_7 = 0b111,
        }
        /// DAC12 input reference and output range
        DAC12_1CTL_DAC12IR: 8 = struct DAC12_1CTL_DAC12IR(bool);
        /// DAC12 calibration
        DAC12_1CTL_DAC12CALON: 9 = struct DAC12_1CTL_DAC12CALON(bool);
        /// DAC12 load select bit 0
        DAC12_1CTL_DAC12LSEL: 10..11 = enum DAC12_1CTL_DAC12LSEL {
            /// DAC12 load select 0: direct
            DAC12LSEL_0 = 0b00,
            /// DAC12 load select 1: latched with DAT
            DAC12LSEL_1 = 0b01,
            /// DAC12 load select 2: latched with pos. Timer_A3.OUT1
            DAC12LSEL_2 = 0b10,
            /// DAC12 load select 3: latched with pos. Timer_B7.OUT1
            DAC12LSEL_3 = 0b11,
        }
        /// DAC12 resolution
        DAC12_1CTL_DAC12RES: 12 = struct DAC12_1CTL_DAC12RES(bool);
        /// DAC12 reference bit 0
        DAC12_1CTL_DAC12SREF: 13..14 = enum DAC12_1CTL_DAC12SREF {
            /// DAC12 reference 0: Vref+
            DAC12SREF_0 = 0b00,
            /// DAC12 reference 1: Vref+
            DAC12SREF_1 = 0b01,
            /// DAC12 reference 2: Veref+
            DAC12SREF_2 = 0b10,
            /// DAC12 reference 3: Veref+
            DAC12SREF_3 = 0b11,
        }
    }
    /// DAC12_0 Data
    rw DAC12_0DAT @ 0x08: u16 = 0_0 {
        /// DAC12_0 Data
        DAC12_0DAT: 0..15 = struct DAC12_0DATField(u16);
    }
    /// DAC12_1 Data
    rw DAC12_1DAT @ 0x0a: u16 = 0_0 {
        /// DAC12_1 Data
        DAC12_1DAT: 0..15 = struct DAC12_1DATField(u16);
    }
}
