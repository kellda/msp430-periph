//! Operational Amplifier

utils::periph! {
    /// Operational Amplifier
    OperationalAmplifier;
    /// OA0 Control register 0
    rw OA0CTL0 @ 0x00: u8 = 0_0 {
        /// OAx output to ADC12 input channel select 0
        OA0ADC0: 0 = struct OA0ADC0(bool);
        /// OAx output to ADC12 input channel select 1
        OA0ADC1: 1 = struct OA0ADC1(bool);
        /// OAx Power mode select 0
        OA0PM: 2..3 = enum OA0PM {
            /// OAx Power mode select: off
            OAPM_0 = 0b00,
            /// OAx Power mode select: slow
            OAPM_1 = 0b01,
            /// OAx Power mode select: meduim
            OAPM_2 = 0b10,
            /// OAx Power mode select: fast
            OAPM_3 = 0b11,
        }
        /// OAx Non-inverting input select 0
        OA0P: 4..5 = enum OA0P {
            /// OAx Non-inverting input select 00
            OAP_0 = 0b00,
            /// OAx Non-inverting input select 01
            OAP_1 = 0b01,
            /// OAx Non-inverting input select 10
            OAP_2 = 0b10,
            /// OAx Non-inverting input select 11
            OAP_3 = 0b11,
        }
        /// OAx Inverting input select 0
        OA0N: 6..7 = enum OA0N {
            /// OAx Inverting input select 00
            OAN_0 = 0b00,
            /// OAx Inverting input select 01
            OAN_1 = 0b01,
            /// OAx Inverting input select 10
            OAN_2 = 0b10,
            /// OAx Inverting input select 11
            OAN_3 = 0b11,
        }
    }
    /// OA0 Control register 1
    rw OA0CTL1 @ 0x01: u8 = 0_0 {
        /// OAx Rail-to-Rail Input off
        OA0RRIP: 0 = struct OA0RRIP(bool);
        /// OAx Inverting input external
        OA0NEXT: 1 = struct OA0NEXT(bool);
        /// OAx Function control 0
        OA0FC: 2..4 = enum OA0FC {
            /// OAx Function: Gen. Purpose
            OAFC_0 = 0b000,
            /// OAx Function: Unity gain buffer
            OAFC_1 = 0b001,
            /// OAx Function: Reserved
            OAFC_2 = 0b010,
            /// OAx Function: Comparator
            OAFC_3 = 0b011,
            /// OAx Function: Non-inverting PGA
            OAFC_4 = 0b100,
            /// OAx Function: Cascaded non-inverting PGA
            OAFC_5 = 0b101,
            /// OAx Function: Inverting PGA
            OAFC_6 = 0b110,
            /// OAx Function: Differential amplifier
            OAFC_7 = 0b111,
        }
        /// OAx Feedback resistor select 0
        OA0FBR: 5..7 = enum OA0FBR {
            /// OAx Feedback resistor: Tap 0
            OAFBR_0 = 0b000,
            /// OAx Feedback resistor: Tap 1
            OAFBR_1 = 0b001,
            /// OAx Feedback resistor: Tap 2
            OAFBR_2 = 0b010,
            /// OAx Feedback resistor: Tap 3
            OAFBR_3 = 0b011,
            /// OAx Feedback resistor: Tap 4
            OAFBR_4 = 0b100,
            /// OAx Feedback resistor: Tap 5
            OAFBR_5 = 0b101,
            /// OAx Feedback resistor: Tap 6
            OAFBR_6 = 0b110,
            /// OAx Feedback resistor: Tap 7
            OAFBR_7 = 0b111,
        }
    }
    /// OA1 Control register 0
    rw OA1CTL0 @ 0x02: u8 = 0_0 {
        /// OAx output to ADC12 input channel select 0
        OA1ADC0: 0 = struct OA1ADC0(bool);
        /// OAx output to ADC12 input channel select 1
        OA1ADC1: 1 = struct OA1ADC1(bool);
        /// OAx Power mode select 0
        OA1PM: 2..3 = enum OA1PM {
            /// OAx Power mode select: off
            OAPM_0 = 0b00,
            /// OAx Power mode select: slow
            OAPM_1 = 0b01,
            /// OAx Power mode select: meduim
            OAPM_2 = 0b10,
            /// OAx Power mode select: fast
            OAPM_3 = 0b11,
        }
        /// OAx Non-inverting input select 0
        OA1P: 4..5 = enum OA1P {
            /// OAx Non-inverting input select 00
            OAP_0 = 0b00,
            /// OAx Non-inverting input select 01
            OAP_1 = 0b01,
            /// OAx Non-inverting input select 10
            OAP_2 = 0b10,
            /// OAx Non-inverting input select 11
            OAP_3 = 0b11,
        }
        /// OAx Inverting input select 0
        OA1N: 6..7 = enum OA1N {
            /// OAx Inverting input select 00
            OAN_0 = 0b00,
            /// OAx Inverting input select 01
            OAN_1 = 0b01,
            /// OAx Inverting input select 10
            OAN_2 = 0b10,
            /// OAx Inverting input select 11
            OAN_3 = 0b11,
        }
    }
    /// OA1 Control register 1
    rw OA1CTL1 @ 0x03: u8 = 0_0 {
        /// OAx Rail-to-Rail Input off
        OA1RRIP: 0 = struct OA1RRIP(bool);
        /// OAx Inverting input external
        OA1NEXT: 1 = struct OA1NEXT(bool);
        /// OAx Function control 0
        OA1FC: 2..4 = enum OA1FC {
            /// OAx Function: Gen. Purpose
            OAFC_0 = 0b000,
            /// OAx Function: Unity gain buffer
            OAFC_1 = 0b001,
            /// OAx Function: Reserved
            OAFC_2 = 0b010,
            /// OAx Function: Comparator
            OAFC_3 = 0b011,
            /// OAx Function: Non-inverting PGA
            OAFC_4 = 0b100,
            /// OAx Function: Cascaded non-inverting PGA
            OAFC_5 = 0b101,
            /// OAx Function: Inverting PGA
            OAFC_6 = 0b110,
            /// OAx Function: Differential amplifier
            OAFC_7 = 0b111,
        }
        /// OAx Feedback resistor select 0
        OA1FBR: 5..7 = enum OA1FBR {
            /// OAx Feedback resistor: Tap 0
            OAFBR_0 = 0b000,
            /// OAx Feedback resistor: Tap 1
            OAFBR_1 = 0b001,
            /// OAx Feedback resistor: Tap 2
            OAFBR_2 = 0b010,
            /// OAx Feedback resistor: Tap 3
            OAFBR_3 = 0b011,
            /// OAx Feedback resistor: Tap 4
            OAFBR_4 = 0b100,
            /// OAx Feedback resistor: Tap 5
            OAFBR_5 = 0b101,
            /// OAx Feedback resistor: Tap 6
            OAFBR_6 = 0b110,
            /// OAx Feedback resistor: Tap 7
            OAFBR_7 = 0b111,
        }
    }
}
