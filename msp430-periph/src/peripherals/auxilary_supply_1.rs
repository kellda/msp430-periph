//! Auxilary Supply

utils::periph! {
    /// Auxilary Supply
    AuxilarySupply;
    /// Auxiliary Supply Control register 0
    rw AUXCTL0 @ 0x00: u16 = 0_0 {
        /// Lock auxiliary supply system.
        LOCKAUX: 0 = struct LOCKAUX(bool);
        /// DVCC switch state.
        AUX0SW: 1 = struct AUX0SW(bool);
        /// AUX1 switch state.
        AUX1SW: 2 = struct AUX1SW(bool);
        /// AUX2 switch state.
        AUX2SW: 3 = struct AUX2SW(bool);
    }
    /// Auxiliary Supply Control register 1
    rw AUXCTL1 @ 0x02: u16 = 0_0 {
        /// DVCC okay flag.
        AUX0OK: 0 = struct AUX0OK(bool);
        /// AUX 1 supply okay flag.
        AUX1OK: 1 = struct AUX1OK(bool);
        /// AUX 2 supply okay flag.
        AUX2OK: 2 = struct AUX2OK(bool);
        /// Auxiliary supply AUX2 priority.
        AUX2PRIO: 3 = struct AUX2PRIO(bool);
        /// DVCC supply mode.
        AUX0MD: 8 = struct AUX0MD(bool);
        /// AUX1 supply mode.
        AUX1MD: 9 = struct AUX1MD(bool);
        /// AUX2 supply mode.
        AUX2MD: 10 = struct AUX2MD(bool);
    }
    /// Auxiliary Supply Control register 2
    rw AUXCTL2 @ 0x04: u16 = 0_0 {
        /// DVCC supply threshold level Bit: 0
        AUX0LVL: 0..2 = enum AUX0LVL {
            /// DVCC supply threshold level: 0
            AUX0LVL_0 = 0b000,
            /// DVCC supply threshold level: 1
            AUX0LVL_1 = 0b001,
            /// DVCC supply threshold level: 2
            AUX0LVL_2 = 0b010,
            /// DVCC supply threshold level: 3
            AUX0LVL_3 = 0b011,
            /// DVCC supply threshold level: 4
            AUX0LVL_4 = 0b100,
            /// DVCC supply threshold level: 5
            AUX0LVL_5 = 0b101,
            /// DVCC supply threshold level: 6
            AUX0LVL_6 = 0b110,
            /// DVCC supply threshold level: 7
            AUX0LVL_7 = 0b111,
        }
        /// AUX1 supply threshold level Bit: 0
        AUX1LVL: 4..6 = enum AUX1LVL {
            /// AUX1 supply threshold level: 0
            AUX1LVL_0 = 0b000,
            /// AUX1 supply threshold level: 1
            AUX1LVL_1 = 0b001,
            /// AUX1 supply threshold level: 2
            AUX1LVL_2 = 0b010,
            /// AUX1 supply threshold level: 3
            AUX1LVL_3 = 0b011,
            /// AUX1 supply threshold level: 4
            AUX1LVL_4 = 0b100,
            /// AUX1 supply threshold level: 5
            AUX1LVL_5 = 0b101,
            /// AUX1 supply threshold level: 6
            AUX1LVL_6 = 0b110,
            /// AUX1 supply threshold level: 7
            AUX1LVL_7 = 0b111,
        }
        /// AUX2 supply threshold level Bit: 0
        AUX2LVL: 8..10 = enum AUX2LVL {
            /// AUX2 supply threshold level: 0
            AUX2LVL_0 = 0b000,
            /// AUX2 supply threshold level: 1
            AUX2LVL_1 = 0b001,
            /// AUX2 supply threshold level: 2
            AUX2LVL_2 = 0b010,
            /// AUX2 supply threshold level: 3
            AUX2LVL_3 = 0b011,
            /// AUX2 supply threshold level: 4
            AUX2LVL_4 = 0b100,
            /// AUX2 supply threshold level: 5
            AUX2LVL_5 = 0b101,
            /// AUX2 supply threshold level: 6
            AUX2LVL_6 = 0b110,
            /// AUX2 supply threshold level: 7
            AUX2LVL_7 = 0b111,
        }
        /// Auxiliary supply monitoring rate Bit: 0
        AUXMR: 12..13 = enum AUXMR {
            /// Auxiliary supply monitoring rate: 0
            AUXMR_0 = 0b00,
            /// Auxiliary supply monitoring rate: 1
            AUXMR_1 = 0b01,
            /// Auxiliary supply monitoring rate: 2
            AUXMR_2 = 0b10,
            /// Auxiliary supply monitoring rate: 3
            AUXMR_3 = 0b11,
        }
    }
    /// AUX2 Charger Control register
    rw AUX2CHCTL @ 0x12: u16 = 0_0 {
        /// Lock auxiliary supply system.
        AUX2CHCTL_AUXCHEN: 0 = struct AUX2CHCTL_AUXCHEN(bool);
        /// Charger charge current Bit: 0
        AUX2CHCTL_AUXCHC: 1..2 = enum AUX2CHCTL_AUXCHC {
            /// Charger charge current: 0
            AUXCHC_0 = 0b00,
            /// Charger charge current: 1
            AUXCHC_1 = 0b01,
            /// Charger charge current: 2
            AUXCHC_2 = 0b10,
            /// Charger charge current: 3
            AUXCHC_3 = 0b11,
        }
        /// Charger end voltage Bit: 0
        AUX2CHCTL_AUXCHV: 4..5 = enum AUX2CHCTL_AUXCHV {
            /// Charger end voltage: 0
            AUXCHV_0 = 0b00,
            /// Charger end voltage: 1
            AUXCHV_1 = 0b01,
            /// Charger end voltage: 2
            AUXCHV_2 = 0b10,
            /// Charger end voltage: 3
            AUXCHV_3 = 0b11,
        }
    }
    /// AUX3 Charger Control register
    rw AUX3CHCTL @ 0x14: u16 = 0_0 {
        /// Lock auxiliary supply system.
        AUX3CHCTL_AUXCHEN: 0 = struct AUX3CHCTL_AUXCHEN(bool);
        /// Charger charge current Bit: 0
        AUX3CHCTL_AUXCHC: 1..2 = enum AUX3CHCTL_AUXCHC {
            /// Charger charge current: 0
            AUXCHC_0 = 0b00,
            /// Charger charge current: 1
            AUXCHC_1 = 0b01,
            /// Charger charge current: 2
            AUXCHC_2 = 0b10,
            /// Charger charge current: 3
            AUXCHC_3 = 0b11,
        }
        /// Charger end voltage Bit: 0
        AUX3CHCTL_AUXCHV: 4..5 = enum AUX3CHCTL_AUXCHV {
            /// Charger end voltage: 0
            AUXCHV_0 = 0b00,
            /// Charger end voltage: 1
            AUXCHV_1 = 0b01,
            /// Charger end voltage: 2
            AUXCHV_2 = 0b10,
            /// Charger end voltage: 3
            AUXCHV_3 = 0b11,
        }
    }
    /// AUX ADC Control
    rw AUXADCCTL @ 0x16: u16 = 0_0 {
        /// Auxiliary supplies to ADC
        AUXADC: 0 = struct AUXADC(bool);
        /// Select supply to be measured with ADC Bit: 0
        AUXADCSEL: 1..2 = enum AUXADCSEL {
            /// Select supply to be measured with ADC: DVCC
            AUXADCSEL_0 = 0b00,
            /// Select supply to be measured with ADC: AUXVCC1
            AUXADCSEL_1 = 0b01,
            /// Select supply to be measured with ADC: AUXVCC2
            AUXADCSEL_2 = 0b10,
            /// Select supply to be measured with ADC: AUXVCC3
            AUXADCSEL_3 = 0b11,
        }
        /// Load resistance Bit: 0
        AUXADCR: 4..5 = enum AUXADCR {
            /// Load resistance: 0
            AUXADCR_0 = 0b00,
            /// Load resistance: 1
            AUXADCR_1 = 0b01,
            /// Load resistance: 2
            AUXADCR_2 = 0b10,
            /// Load resistance: 3
            AUXADCR_3 = 0b11,
        }
    }
    /// AUX Interrupt Flag
    rw AUXIFG @ 0x1a: u16 = 0_0 {
        /// Switched to DVCC interrupt flag
        AUX0SWIFG: 0 = struct AUX0SWIFG(bool);
        /// Switched to AUX1 interrupt flag
        AUX1SWIFG: 1 = struct AUX1SWIFG(bool);
        /// Switched to AUX2 interrupt flag
        AUX2SWIFG: 2 = struct AUX2SWIFG(bool);
        /// DVCC dropped below its threshold interrupt flag
        AUX0DRPIFG: 4 = struct AUX0DRPIFG(bool);
        /// AUX1 dropped below its threshold interrupt flag
        AUX1DRPIFG: 5 = struct AUX1DRPIFG(bool);
        /// AUX2 dropped below its threshold interrupt flag
        AUX2DRPIFG: 6 = struct AUX2DRPIFG(bool);
        /// Supply monitor interrupt flag
        AUXMONIFG: 7 = struct AUXMONIFG(bool);
        /// Supplies switched (non-)maskable interrupt flag
        AUXSWNMIFG: 8 = struct AUXSWNMIFG(bool);
    }
    /// AUX Interrupt Enable
    rw AUXIE @ 0x1c: u16 = 0_0 {
        /// Switched to DVCC interrupt enable
        AUX0SWIE: 0 = struct AUX0SWIE(bool);
        /// Switched to AUX1 interrupt enable
        AUX1SWIE: 1 = struct AUX1SWIE(bool);
        /// Switched to AUX2 interrupt enable
        AUX2SWIE: 2 = struct AUX2SWIE(bool);
        /// Global supply switched interrupt enable.
        AUXSWGIE: 3 = struct AUXSWGIE(bool);
        /// DVCC dropped below its threshold interrupt enable
        AUX0DRPIE: 4 = struct AUX0DRPIE(bool);
        /// AUX1 dropped below its threshold interrupt enable
        AUX1DRPIE: 5 = struct AUX1DRPIE(bool);
        /// AUX2 dropped below its threshold interrupt enable
        AUX2DRPIE: 6 = struct AUX2DRPIE(bool);
        /// Supply monitor interrupt enable
        AUXMONIE: 7 = struct AUXMONIE(bool);
        /// Supplies switched (non-)maskable interrupt enable
        AUXSWNMIE: 8 = struct AUXSWNMIE(bool);
    }
    /// AUX Interrupt Vector Word
    rw AUXIV @ 0x1e: u16 = 0_0 {
        /// AUX Interrupt Vector Word
        AUXIV: 0..15 = struct AUXIVField(u16);
    }
}
