//! Operational Amplifier

utils::periph! {
    /// Operational Amplifier
    OperationalAmplifier;
    /// OA0 Control register 0
    rw OA0CTL0 @ 0x00: u8 = 0_0 {
        /// OAx output to ADC12 input channel select 0
        OA0CTL0_OAADC0: 0 = struct OA0CTL0_OAADC0(bool);
        /// OAx output to ADC12 input channel select 1
        OA0CTL0_OAADC1: 1 = struct OA0CTL0_OAADC1(bool);
        /// OAx Power mode select 0
        OA0CTL0_OAPM: 2..3 = enum OA0CTL0_OAPM {
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
        OA0CTL0_OAP: 4..5 = enum OA0CTL0_OAP {
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
        OA0CTL0_OAN: 6..7 = enum OA0CTL0_OAN {
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
        OA0CTL1_OARRIP: 0 = struct OA0CTL1_OARRIP(bool);
        /// OAx Function control 0
        OA0CTL1_OAFC: 2..4 = enum OA0CTL1_OAFC {
            /// OAx Function: Gen. Purpose
            OAFC_0 = 0b000,
            /// OAx Function: Unity gain buffer
            OAFC_1 = 0b001,
            /// OAx Function: Reserved
            OAFC_2 = 0b010,
            /// OAx Function: Comparator
            OAFC_3 = 0b011,
            /// OAx Function: Non-Inverting PGA
            OAFC_4 = 0b100,
            /// OAx Function: Reserved
            OAFC_5 = 0b101,
            /// OAx Function: Inverting PGA
            OAFC_6 = 0b110,
            /// OAx Function: Differential PGA
            OAFC_7 = 0b111,
        }
        /// OAx Feedback resistor select 0
        OA0CTL1_OAFBR: 5..7 = enum OA0CTL1_OAFBR {
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
        OA1CTL0_OAADC0: 0 = struct OA1CTL0_OAADC0(bool);
        /// OAx output to ADC12 input channel select 1
        OA1CTL0_OAADC1: 1 = struct OA1CTL0_OAADC1(bool);
        /// OAx Power mode select 0
        OA1CTL0_OAPM: 2..3 = enum OA1CTL0_OAPM {
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
        OA1CTL0_OAP: 4..5 = enum OA1CTL0_OAP {
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
        OA1CTL0_OAN: 6..7 = enum OA1CTL0_OAN {
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
        OA1CTL1_OARRIP: 0 = struct OA1CTL1_OARRIP(bool);
        /// OAx Function control 0
        OA1CTL1_OAFC: 2..4 = enum OA1CTL1_OAFC {
            /// OAx Function: Gen. Purpose
            OAFC_0 = 0b000,
            /// OAx Function: Unity gain buffer
            OAFC_1 = 0b001,
            /// OAx Function: Reserved
            OAFC_2 = 0b010,
            /// OAx Function: Comparator
            OAFC_3 = 0b011,
            /// OAx Function: Non-Inverting PGA
            OAFC_4 = 0b100,
            /// OAx Function: Reserved
            OAFC_5 = 0b101,
            /// OAx Function: Inverting PGA
            OAFC_6 = 0b110,
            /// OAx Function: Differential PGA
            OAFC_7 = 0b111,
        }
        /// OAx Feedback resistor select 0
        OA1CTL1_OAFBR: 5..7 = enum OA1CTL1_OAFBR {
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
    /// OA2 Control register 0
    rw OA2CTL0 @ 0x04: u8 = 0_0 {
        /// OAx output to ADC12 input channel select 0
        OA2CTL0_OAADC0: 0 = struct OA2CTL0_OAADC0(bool);
        /// OAx output to ADC12 input channel select 1
        OA2CTL0_OAADC1: 1 = struct OA2CTL0_OAADC1(bool);
        /// OAx Power mode select 0
        OA2CTL0_OAPM: 2..3 = enum OA2CTL0_OAPM {
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
        OA2CTL0_OAP: 4..5 = enum OA2CTL0_OAP {
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
        OA2CTL0_OAN: 6..7 = enum OA2CTL0_OAN {
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
    /// OA2 Control register 1
    rw OA2CTL1 @ 0x05: u8 = 0_0 {
        /// OAx Rail-to-Rail Input off
        OA2CTL1_OARRIP: 0 = struct OA2CTL1_OARRIP(bool);
        /// OAx Function control 0
        OA2CTL1_OAFC: 2..4 = enum OA2CTL1_OAFC {
            /// OAx Function: Gen. Purpose
            OAFC_0 = 0b000,
            /// OAx Function: Unity gain buffer
            OAFC_1 = 0b001,
            /// OAx Function: Reserved
            OAFC_2 = 0b010,
            /// OAx Function: Comparator
            OAFC_3 = 0b011,
            /// OAx Function: Non-Inverting PGA
            OAFC_4 = 0b100,
            /// OAx Function: Reserved
            OAFC_5 = 0b101,
            /// OAx Function: Inverting PGA
            OAFC_6 = 0b110,
            /// OAx Function: Differential PGA
            OAFC_7 = 0b111,
        }
        /// OAx Feedback resistor select 0
        OA2CTL1_OAFBR: 5..7 = enum OA2CTL1_OAFBR {
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
