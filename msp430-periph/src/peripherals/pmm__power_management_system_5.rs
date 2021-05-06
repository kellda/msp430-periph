//! PMM  Power Management System

utils::periph! {
    /// PMM  Power Management System
    PMMPowerManagementSystem;
    /// PMM Power Mode 4.5 Control Register
    rw LPM45CTL @ 0x00: u8 = 0_0 {
        /// Lock I/O pin configuration upon entry/exit to/from LPM4.5
        LOCKLPM45: 0 = struct LOCKLPM45(bool);
        /// LPM4.5 interrupt flag
        LPM45IFG: 1 = struct LPM45IFG(bool);
        /// PMM Turn Regulator off
        PMMREGOFF: 4 = struct PMMREGOFF(bool);
    }
    /// PMM Voltage Monitor Control Register
    rw VMONCTL @ 0x01: u8 = 0_0 {
        /// Voltage monitor level selection Bit 0
        VMONLVL: 0..2 = enum VMONLVL {
            /// Voltage monitor level selection: 0
            VMONLVL_0 = 0b000,
            /// Voltage monitor level selection: 1
            VMONLVL_1 = 0b001,
            /// Voltage monitor level selection: 2
            VMONLVL_2 = 0b010,
            /// Voltage monitor level selection: 3
            VMONLVL_3 = 0b011,
            /// Voltage monitor level selection: 4
            VMONLVL_4 = 0b100,
            /// Voltage monitor level selection: 5
            VMONLVL_5 = 0b101,
            /// Voltage monitor level selection: 6
            VMONLVL_6 = 0b110,
            /// Voltage monitor level selection: 7
            VMONLVL_7 = 0b111,
        }
        /// Voltage monitor interrupt enable
        VMONIE: 4 = struct VMONIE(bool);
        /// Voltage monitor interrupt flag
        VMONIFG: 5 = struct VMONIFG(bool);
    }
    /// PMM Reference Calibration Register 0
    rw REFCAL0 @ 0x02: u8 = 0_0 {
        /// Bandgap voltage fine calibration Bit 0
        BGFINE0: 0 = struct BGFINE0(bool);
        /// Bandgap voltage fine calibration Bit 1
        BGFINE1: 1 = struct BGFINE1(bool);
        /// Bandgap voltage fine calibration Bit 2
        BGFINE2: 2 = struct BGFINE2(bool);
        /// Bandgap voltage fine calibration Bit 3
        BGFINE3: 3 = struct BGFINE3(bool);
        /// Bandgap voltage coarse calibration Bit 0
        BGCOARSE0: 4 = struct BGCOARSE0(bool);
        /// Bandgap voltage coarse calibration Bit 1
        BGCOARSE1: 5 = struct BGCOARSE1(bool);
    }
    /// PMM Reference Calibration Register 1
    rw REFCAL1 @ 0x03: u8 = 0_0 {
        /// Curvature compensation Bit 0
        BGCURVE0: 0 = struct BGCURVE0(bool);
        /// Curvature compensation Bit 1
        BGCURVE1: 1 = struct BGCURVE1(bool);
        /// Curvature compensation Bit 2
        BGCURVE2: 2 = struct BGCURVE2(bool);
        /// Curvature compensation Bit 3
        BGCURVE3: 3 = struct BGCURVE3(bool);
        /// Bandgap bias current calibration Bit 0
        BIASCURRENT0: 4 = struct BIASCURRENT0(bool);
        /// Bandgap bias current calibration Bit 1
        BIASCURRENT1: 5 = struct BIASCURRENT1(bool);
        /// Bandgap bias current calibration Bit 2
        BIASCURRENT2: 6 = struct BIASCURRENT2(bool);
        /// Bandgap bias current calibration Bit 3
        BIASCURRENT3: 7 = struct BIASCURRENT3(bool);
    }
}
