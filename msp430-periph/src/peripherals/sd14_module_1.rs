//! SD14 Module

utils::periph! {
    /// SD14 Module
    SD14Module;
    /// SD14 Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// SD14 Module enable
        EN: 0 = struct EN(bool);
        /// SD14 Enable transmission of uplink
        SC: 2 = struct SC(bool);
        /// SD14 Conversion mode select
        SGL: 3 = struct SGL(bool);
        /// SD14 Sigma-Delta Clock source Bit: 0
        SSEL: 4..5 = enum SSEL {
            /// SD14 Sigma-Delta Clock select: 0
            SSEL_0 = 0b00,
            /// SD14 Sigma-Delta Clock select: 1
            SSEL_1 = 0b01,
            /// SD14 Sigma-Delta Clock select: 2
            SSEL_2 = 0b10,
            /// SD14 Sigma-Delta Clock select: 3
            SSEL_3 = 0b11,
        }
        /// SD14 Sigma-Delta Clock divider Bit: 0
        DIV: 6..7 = enum DIV {
            /// SD14 Sigma-Delta Clock divider select: 0
            DIV_0 = 0b00,
            /// SD14 Sigma-Delta Clock divider select: 1
            DIV_1 = 0b01,
            /// SD14 Sigma-Delta Clock divider select: 2
            DIV_2 = 0b10,
            /// SD14 Sigma-Delta Clock divider select: 3
            DIV_3 = 0b11,
        }
        /// SD14 interrupt enable
        IE: 8 = struct IE(bool);
        /// SD14 interrupt flag
        IFG: 9 = struct IFG(bool);
        /// SD14 overflow interrupt enable
        OVIE: 10 = struct OVIE(bool);
        /// SD14 overflow interrupt flag
        OVIFG: 11 = struct OVIFG(bool);
        /// Defines virtual ground
        VIRTGND: 12 = struct VIRTGND(bool);
    }
    /// SD14 Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// SD14 channel input selection Bit: 0
        INCH: 0..2 = enum INCH {
            /// SD14 channel input select: 0
            INCH_0 = 0b000,
            /// SD14 channel input select: 1
            INCH_1 = 0b001,
            /// SD14 channel input select: 2
            INCH_2 = 0b010,
            /// SD14 channel input select: 3
            INCH_3 = 0b011,
            /// SD14 channel input select: 4
            INCH_4 = 0b100,
            /// SD14 channel input select: 5
            INCH_5 = 0b101,
            /// SD14 channel input select: 6
            INCH_6 = 0b110,
            /// SD14 channel input select: 7
            INCH_7 = 0b111,
        }
        /// SD14 preamplifier gain Bit: 0
        GAIN: 3..4 = enum GAIN {
            /// SD14 preamplifier gain select: 0
            GAIN_0 = 0b00,
            /// SD14 preamplifier gain select: 1
            GAIN_1 = 0b01,
            /// SD14 preamplifier gain select: 2
            GAIN_2 = 0b10,
            /// SD14 preamplifier gain select: 3
            GAIN_3 = 0b11,
        }
        /// SD14 Interrupt delay Bit: 0
        INTDLY: 6..7 = enum INTDLY {
            /// SD14 Interrupt delay select: 0
            INTDLY_0 = 0b00,
            /// SD14 Interrupt delay select: 1
            INTDLY_1 = 0b01,
            /// SD14 Interrupt delay select: 2
            INTDLY_2 = 0b10,
            /// SD14 Interrupt delay select: 3
            INTDLY_3 = 0b11,
        }
        /// SD14 digital filter selection
        FILT: 8 = struct FILT(bool);
        /// SD14 rate change factor selection Bit: 0
        RATE: 9..11 = enum RATE {
            /// SD14 rate change factor: 0
            RATE_0 = 0b000,
            /// SD14 rate change factor: 1
            RATE_1 = 0b001,
            /// SD14 rate change factor: 2
            RATE_2 = 0b010,
            /// SD14 rate change factor: 3
            RATE_3 = 0b011,
            /// SD14 rate change factor: 4
            RATE_4 = 0b100,
            /// SD14 rate change factor: 5
            RATE_5 = 0b101,
            /// SD14 rate change factor: 6
            RATE_6 = 0b110,
            /// SD14 rate change factor: 7
            RATE_7 = 0b111,
        }
        /// SD14 Unipolar format enable Bit: 0
        UNI: 12 = struct UNI(bool);
        /// SD14 Two's complement format enable Bit: 0
        TC: 13 = struct TC(bool);
        /// SD14 Resistive bias on TEMP1 enable
        RBEN0: 14 = struct RBEN0(bool);
        /// SD14 Resistive bias on TEMP2 enable
        RBEN1: 15 = struct RBEN1(bool);
    }
    /// SD14 Conversion Result
    rw MEM0 @ 0x04: u16 = 0_0 {
        /// SD14 Conversion Result
        MEM0: 0..15 = struct MEM0Field(u16);
    }
    /// SD14 Intermediate Conversion Result
    rw MEM1 @ 0x06: u16 = 0_0 {
        /// SD14 Intermediate Conversion Result
        MEM1: 0..15 = struct MEM1Field(u16);
    }
    /// SD14 Intermediate Conversion Result
    rw MEM2 @ 0x08: u16 = 0_0 {
        /// SD14 Intermediate Conversion Result
        MEM2: 0..15 = struct MEM2Field(u16);
    }
    /// SD14 Intermediate Conversion Result
    rw MEM3 @ 0x0a: u16 = 0_0 {
        /// SD14 Intermediate Conversion Result
        MEM3: 0..15 = struct MEM3Field(u16);
    }
    /// SD14 Interrupt Vector
    rw IV @ 0x0c: u16 = 0_0 {
        /// SD14IV Interrupt vector value bit: 0
        IV0: 1 = struct IV0(bool);
        /// SD14IV Interrupt vector value bit: 1
        IV1: 2 = struct IV1(bool);
    }
}
