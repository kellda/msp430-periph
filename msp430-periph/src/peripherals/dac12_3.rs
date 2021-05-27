//! DAC12

utils::periph! {
    /// DAC12
    DAC12;
    /// DAC12_0 Control
    rw DAC0CTL @ 0x00: u16 = 0_0 {
        /// DAC12 group (not used)
        GRP: 0 = struct GRP(bool);
        /// DAC12 enable conversion
        ENC: 1 = struct ENC(bool);
        /// DAC12 interrupt flag
        IFG: 2 = struct IFG(bool);
        /// DAC12 interrupt enable
        IE: 3 = struct IE(bool);
        /// DAC12 data format
        DF: 4 = struct DF(bool);
        /// DAC12 amplifier bit 0
        AMP: 5..7 = enum AMP {
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
        IR: 8 = struct IR(bool);
        /// DAC12 calibration
        CALON: 9 = struct CALON(bool);
        /// DAC12 load select bit 0
        LSEL: 10..11 = enum LSEL {
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
        RES: 12 = struct RES(bool);
        /// DAC12 reference bit 0
        SREF: 13..14 = enum SREF {
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
        OPS: 15 = struct OPS(bool);
    }
    /// DAC12_0 Data
    rw DAC0DAT @ 0x08: u16 = 0_0 {
        /// DAC12_0 Data
        DAT: 0..15 = struct DATField(u16);
    }
}
