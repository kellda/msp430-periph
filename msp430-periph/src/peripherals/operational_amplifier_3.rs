//! Operational Amplifier

utils::periph! {
    /// Operational Amplifier
    OperationalAmplifier;
    /// OA0 Control register 0
    rw OA0CTL0 @ 0x00: u8 = 0_0 {
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
        /// OAx Offset Calibration
        OA0CTL1_OACAL: 1 = struct OA0CTL1_OACAL(bool);
        /// OAx Function control 0
        OA0CTL1_OAFC: 2..4 = enum OA0CTL1_OAFC {
            /// OAx Function: Gen. Purpose
            OAFC_0 = 0b000,
            /// OAx Function: Unity gain buffer
            OAFC_1 = 0b001,
            /// OAx Function: Reserved
            OAFC_2 = 0b010,
            /// OAx Function: Reserved
            OAFC_3 = 0b011,
            /// OAx Function: Reserved
            OAFC_4 = 0b100,
            /// OAx Function: Reserved
            OAFC_5 = 0b101,
            /// OAx Function: Inverting PGA
            OAFC_6 = 0b110,
            /// OAx Function: Reserved
            OAFC_7 = 0b111,
        }
    }
    /// OA1 Control register 0
    rw OA1CTL0 @ 0x02: u8 = 0_0 {
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
        /// OAx Offset Calibration
        OA1CTL1_OACAL: 1 = struct OA1CTL1_OACAL(bool);
        /// OAx Function control 0
        OA1CTL1_OAFC: 2..4 = enum OA1CTL1_OAFC {
            /// OAx Function: Gen. Purpose
            OAFC_0 = 0b000,
            /// OAx Function: Unity gain buffer
            OAFC_1 = 0b001,
            /// OAx Function: Reserved
            OAFC_2 = 0b010,
            /// OAx Function: Reserved
            OAFC_3 = 0b011,
            /// OAx Function: Reserved
            OAFC_4 = 0b100,
            /// OAx Function: Reserved
            OAFC_5 = 0b101,
            /// OAx Function: Inverting PGA
            OAFC_6 = 0b110,
            /// OAx Function: Reserved
            OAFC_7 = 0b111,
        }
    }
    /// OA  Analog Switches Control Register
    rw SWCTL @ 0x0f: u8 = 0_0 {
        /// OA  Analog Switch Control 0
        SWCTL0: 0 = struct SWCTL0(bool);
        /// OA  Analog Switch Control 1
        SWCTL1: 1 = struct SWCTL1(bool);
        /// OA  Analog Switch Control 2
        SWCTL2: 2 = struct SWCTL2(bool);
        /// OA  Analog Switch Control 3
        SWCTL3: 3 = struct SWCTL3(bool);
        /// OA  Analog Switch Control 4
        SWCTL4: 4 = struct SWCTL4(bool);
        /// OA  Analog Switch Control 5
        SWCTL5: 5 = struct SWCTL5(bool);
        /// OA  Analog Switch Control 6
        SWCTL6: 6 = struct SWCTL6(bool);
        /// OA  Analog Switch Control 7
        SWCTL7: 7 = struct SWCTL7(bool);
    }
}
