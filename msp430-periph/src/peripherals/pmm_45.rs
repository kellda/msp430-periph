//! PMM

utils::periph! {
    /// PMM
    PMM;
    /// PMM control register 0
    rw PMMCTL0 @ 0x00: u16 = 0_0 {
        /// Software brownout reset.
        PMMSWBOR: 2 = enum PMMSWBOR {
            /// Normal operation
            PMMSWBOR_0 = 0b0,
            /// Set to 1 to trigger a BOR
            PMMSWBOR_1 = 0b1,
        }
        /// Software POR.
        PMMSWPOR: 3 = enum PMMSWPOR {
            /// Normal operation
            PMMSWPOR_0 = 0b0,
            /// Set to 1 to trigger a POR
            PMMSWPOR_1 = 0b1,
        }
        /// Regulator off
        PMMREGOFF: 4 = enum PMMREGOFF {
            /// Regulator remains on when going into LPM3 or LPM4
            PMMREGOFF_0 = 0b0,
            /// Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively.
            PMMREGOFF_1 = 0b1,
        }
        /// High-side SVS enable.
        SVSHE: 6 = enum SVSHE {
            /// High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1.
            SVSHE_0 = 0b0,
            /// SVSH is always enabled.
            SVSHE_1 = 0b1,
        }
        /// PMM password.
        PMMPW: 8..15 = struct PMMPW(u16);
    }
    /// Power Management Module Control Register 2
    rw PMMCTL2 @ 0x04: u16 = 0_0 {
        /// Internal reference enable
        INTREFEN: 0 = enum INTREFEN {
            /// Disable internal reference
            INTREFEN_0 = 0b0,
            /// Enable internal reference
            INTREFEN_1 = 0b1,
        }
        /// External reference output enable
        EXTREFEN: 1 = enum EXTREFEN {
            /// Disable external reference output
            EXTREFEN_0 = 0b0,
            /// Enable internal reference output
            EXTREFEN_1 = 0b1,
        }
        /// Temperature sensor enable
        TSENSOREN: 3 = enum TSENSOREN {
            /// Disable temperature sensor
            TSENSOREN_0 = 0b0,
            /// Enable temperature sensor
            TSENSOREN_1 = 0b1,
        }
        /// Reference generator active. Read only.
        REFGENACT: 8 = enum REFGENACT {
            /// Reference generator not active
            REFGENACT_0 = 0b0,
            /// Reference generator active
            REFGENACT_1 = 0b1,
        }
        /// Reference bandgap active. Ready only.
        REFBGACT: 9 = enum REFBGACT {
            /// Reference bandgap buffer not active
            REFBGACT_0 = 0b0,
            /// Reference bandgap buffer active
            REFBGACT_1 = 0b1,
        }
        /// Bandgap mode. Ready only.
        BGMODE: 11 = enum BGMODE {
            /// Static mode (higher precision)
            BGMODE_0 = 0b0,
            /// Sampled mode (lower power consumption)
            BGMODE_1 = 0b1,
        }
        /// Variable reference voltage ready status.
        REFGENRDY: 12 = enum REFGENRDY {
            /// Reference voltage output is not ready to be used.
            REFGENRDY_0 = 0b0,
            /// Reference voltage output is ready to be used
            REFGENRDY_1 = 0b1,
        }
        /// Buffered bandgap voltage ready status.
        REFBGRDY: 13 = enum REFBGRDY {
            /// Buffered bandgap voltage is not ready to be used
            REFBGRDY_0 = 0b0,
            /// Buffered bandgap voltage is ready to be used
            REFBGRDY_1 = 0b1,
        }
    }
    /// PMM interrupt flag register
    rw PMMIFG @ 0x0a: u16 = 0_0 {
        /// PMM software brownout reset interrupt flag.
        PMMBORIFG: 8 = enum PMMBORIFG {
            /// Reset not due to PMMSWBOR
            PMMBORIFG_0 = 0b0,
            /// Reset due to PMMSWBOR
            PMMBORIFG_1 = 0b1,
        }
        /// PMM reset pin interrupt flag.
        PMMRSTIFG: 9 = enum PMMRSTIFG {
            /// Reset not due to reset pin
            PMMRSTIFG_0 = 0b0,
            /// Reset due to reset pin
            PMMRSTIFG_1 = 0b1,
        }
        /// PMM software POR interrupt flag.
        PMMPORIFG: 10 = enum PMMPORIFG {
            /// Reset not due to PMMSWPOR
            PMMPORIFG_0 = 0b0,
            /// Reset due to PMMSWPOR
            PMMPORIFG_1 = 0b1,
        }
        /// High-side SVS interrupt flag.
        SVSHIFG: 13 = enum SVSHIFG {
            /// Reset not due to SVSH
            SVSHIFG_0 = 0b0,
            /// Reset due to SVSH
            SVSHIFG_1 = 0b1,
        }
        /// LPMx.5 flag.
        PMMLPM5IFG: 15 = enum PMMLPM5IFG {
            /// Reset not due to wake-up from LPMx.5
            PMMLPM5IFG_0 = 0b0,
            /// Reset due to wake-up from LPMx.5
            PMMLPM5IFG_1 = 0b1,
        }
    }
    /// Power mode 5 control register 0
    rw PM5CTL0 @ 0x10: u16 = 0_0 {
        /// LPMx.5 Lock Bit
        LOCKLPM5: 0 = enum LOCKLPM5 {
            /// LPMx.5 configuration is not locked and defaults to its reset condition.
            LOCKLPM5_0 = 0b0,
            /// LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit.
            LOCKLPM5_1 = 0b1,
        }
        /// Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected.
        LPM5SW: 4 = enum LPM5SW {
            /// LPMx.5 switch disconnected
            LPM5SW_0 = 0b0,
            /// LPMx.5 switch connected
            LPM5SW_1 = 0b1,
        }
        /// Specifies the operation mode of the LPM3.5 switch.
        LPM5SM: 5 = enum LPM5SM {
            /// Automatic mode for LPM3.5 switch that the switch is fully handled by the circuitry during mode switch.
            LPM5SM_0 = 0b0,
            /// Manual mode for LPM3.5 switch that the switch is specified by LPM5SW bit setting in software.
            LPM5SM_1 = 0b1,
        }
    }
}
