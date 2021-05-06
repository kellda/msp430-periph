//! CS  Clock System

utils::periph! {
    /// CS  Clock System
    CSClockSystem;
    /// CS Control Register 0
    rw CSCTL0 @ 0x00: u8 = 0_0 {
        /// DCO resistor select
        DCOR: 0 = struct DCOR(bool);
        /// DCO bypass mode
        DCOBYP: 1 = struct DCOBYP(bool);
        /// DCO fault flag
        DCOF: 7 = struct DCOF(bool);
    }
    /// CS Control Register 1
    rw CSCTL1 @ 0x01: u8 = 0_0 {
        /// MCLK Divider Bit: 0
        DIVM: 0..2 = enum DIVM {
            /// MCLK Source Divider 0
            DIVM_0 = 0b000,
            /// MCLK Source Divider 1
            DIVM_1 = 0b001,
            /// MCLK Source Divider 2
            DIVM_2 = 0b010,
            /// MCLK Source Divider 3
            DIVM_3 = 0b011,
            /// MCLK Source Divider 5
            DIVM_4 = 0b100,
            /// MCLK Source Divider 6
            DIVM_5 = 0b101,
            /// MCLK Source Divider 7
            DIVM_6 = 0b110,
            /// MCLK Source Divider 8
            DIVM_7 = 0b111,
        }
        /// SMCLK Divider Bit: 0
        DIVS: 4..6 = enum DIVS {
            /// SMCLK Source Divider 0
            DIVS_0 = 0b000,
            /// SMCLK Source Divider 1
            DIVS_1 = 0b001,
            /// SMCLK Source Divider 2
            DIVS_2 = 0b010,
            /// SMCLK Source Divider 3
            DIVS_3 = 0b011,
            /// SMCLK Source Divider 5
            DIVS_4 = 0b100,
            /// SMCLK Source Divider 6
            DIVS_5 = 0b101,
            /// SMCLK Source Divider 7
            DIVS_6 = 0b110,
            /// SMCLK Source Divider 8
            DIVS_7 = 0b111,
        }
    }
    /// CS Internal Resistor Frequency Calibration
    rw CSIRFCAL @ 0x02: u8 = 0_0 {
        /// DCO internal resistor frequency calibration value Bit 0
        IRFCAL0: 0 = struct IRFCAL0(bool);
        /// DCO internal resistor frequency calibration value Bit 1
        IRFCAL1: 1 = struct IRFCAL1(bool);
        /// DCO internal resistor frequency calibration value Bit 2
        IRFCAL2: 2 = struct IRFCAL2(bool);
        /// DCO internal resistor frequency calibration value Bit 3
        IRFCAL3: 3 = struct IRFCAL3(bool);
        /// DCO internal resistor frequency calibration value Bit 4
        IRFCAL4: 4 = struct IRFCAL4(bool);
        /// DCO internal resistor frequency calibration value Bit 5
        IRFCAL5: 5 = struct IRFCAL5(bool);
        /// DCO internal resistor frequency calibration value Bit 6
        IRFCAL6: 6 = struct IRFCAL6(bool);
        /// DCO internal resistor frequency calibration value Bit 7
        IRFCAL7: 7 = struct IRFCAL7(bool);
    }
    /// CS Internal Resistor Temperature Calibration
    rw CSIRTCAL @ 0x03: u8 = 0_0 {
        /// DCO internal resistor temperature calibration value Bit 0
        IRTCAL0: 0 = struct IRTCAL0(bool);
        /// DCO internal resistor temperature calibration value Bit 1
        IRTCAL1: 1 = struct IRTCAL1(bool);
        /// DCO internal resistor temperature calibration value Bit 2
        IRTCAL2: 2 = struct IRTCAL2(bool);
        /// DCO internal resistor temperature calibration value Bit 3
        IRTCAL3: 3 = struct IRTCAL3(bool);
        /// DCO internal resistor temperature calibration value Bit 4
        IRTCAL4: 4 = struct IRTCAL4(bool);
        /// DCO internal resistor temperature calibration value Bit 5
        IRTCAL5: 5 = struct IRTCAL5(bool);
        /// DCO internal resistor temperature calibration value Bit 6
        IRTCAL6: 6 = struct IRTCAL6(bool);
        /// DCO internal resistor temperature calibration value Bit 7
        IRTCAL7: 7 = struct IRTCAL7(bool);
    }
    /// CS External Resistor Frequency Calibration
    rw CSERFCAL @ 0x04: u8 = 0_0 {
        /// DCO external resistor frequency calibration value Bit 0
        ERFCAL0: 0 = struct ERFCAL0(bool);
        /// DCO external resistor frequency calibration value Bit 1
        ERFCAL1: 1 = struct ERFCAL1(bool);
        /// DCO external resistor frequency calibration value Bit 2
        ERFCAL2: 2 = struct ERFCAL2(bool);
        /// DCO external resistor frequency calibration value Bit 3
        ERFCAL3: 3 = struct ERFCAL3(bool);
        /// DCO external resistor frequency calibration value Bit 4
        ERFCAL4: 4 = struct ERFCAL4(bool);
        /// DCO external resistor frequency calibration value Bit 5
        ERFCAL5: 5 = struct ERFCAL5(bool);
        /// DCO external resistor frequency calibration value Bit 6
        ERFCAL6: 6 = struct ERFCAL6(bool);
        /// DCO external resistor frequency calibration value Bit 7
        ERFCAL7: 7 = struct ERFCAL7(bool);
    }
    /// CS External Resistor Temperature Calibration
    rw CSERTCAL @ 0x05: u8 = 0_0 {
        /// DCO external resistor temperature calibration value Bit 0
        ERTCAL0: 0 = struct ERTCAL0(bool);
        /// DCO external resistor temperature calibration value Bit 1
        ERTCAL1: 1 = struct ERTCAL1(bool);
        /// DCO external resistor temperature calibration value Bit 2
        ERTCAL2: 2 = struct ERTCAL2(bool);
        /// DCO external resistor temperature calibration value Bit 3
        ERTCAL3: 3 = struct ERTCAL3(bool);
        /// DCO external resistor temperature calibration value Bit 4
        ERTCAL4: 4 = struct ERTCAL4(bool);
        /// DCO external resistor temperature calibration value Bit 5
        ERTCAL5: 5 = struct ERTCAL5(bool);
        /// DCO external resistor temperature calibration value Bit 6
        ERTCAL6: 6 = struct ERTCAL6(bool);
        /// DCO external resistor temperature calibration value Bit 7
        ERTCAL7: 7 = struct ERTCAL7(bool);
    }
}
