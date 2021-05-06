//! SD14 Module

utils::periph! {
    /// SD14 Module
    SD14Module;
    /// SD14 Control Register 0
    rw SD14CTL0 @ 0x00: u16 = 0_0 {
        /// SD14 Module enable
        SD14EN: 0 = struct SD14EN(bool);
        /// SD14 Enable transmission of uplink
        SD14SC: 2 = struct SD14SC(bool);
        /// SD14 Conversion mode select
        SD14SGL: 3 = struct SD14SGL(bool);
        /// SD14 Sigma-Delta Clock source Bit: 0
        SD14SSEL: 4..5 = enum SD14SSEL {
            /// SD14 Sigma-Delta Clock select: 0
            SD14SSEL_0 = 0b00,
            /// SD14 Sigma-Delta Clock select: 1
            SD14SSEL_1 = 0b01,
            /// SD14 Sigma-Delta Clock select: 2
            SD14SSEL_2 = 0b10,
            /// SD14 Sigma-Delta Clock select: 3
            SD14SSEL_3 = 0b11,
        }
        /// SD14 Sigma-Delta Clock divider Bit: 0
        SD14DIV: 6..7 = enum SD14DIV {
            /// SD14 Sigma-Delta Clock divider select: 0
            SD14DIV_0 = 0b00,
            /// SD14 Sigma-Delta Clock divider select: 1
            SD14DIV_1 = 0b01,
            /// SD14 Sigma-Delta Clock divider select: 2
            SD14DIV_2 = 0b10,
            /// SD14 Sigma-Delta Clock divider select: 3
            SD14DIV_3 = 0b11,
        }
        /// SD14 interrupt enable
        SD14IE: 8 = struct SD14IE(bool);
        /// SD14 interrupt flag
        SD14IFG: 9 = struct SD14IFG(bool);
        /// SD14 overflow interrupt enable
        SD14OVIE: 10 = struct SD14OVIE(bool);
        /// SD14 overflow interrupt flag
        SD14OVIFG: 11 = struct SD14OVIFG(bool);
        /// Defines virtual ground
        VIRTGND: 12 = struct VIRTGND(bool);
    }
    /// SD14 Control Register 1
    rw SD14CTL1 @ 0x02: u16 = 0_0 {
        /// SD14 channel input selection Bit: 0
        SD14INCH: 0..2 = enum SD14INCH {
            /// SD14 channel input select: 0
            SD14INCH_0 = 0b000,
            /// SD14 channel input select: 1
            SD14INCH_1 = 0b001,
            /// SD14 channel input select: 2
            SD14INCH_2 = 0b010,
            /// SD14 channel input select: 3
            SD14INCH_3 = 0b011,
            /// SD14 channel input select: 4
            SD14INCH_4 = 0b100,
            /// SD14 channel input select: 5
            SD14INCH_5 = 0b101,
            /// SD14 channel input select: 6
            SD14INCH_6 = 0b110,
            /// SD14 channel input select: 7
            SD14INCH_7 = 0b111,
        }
        /// SD14 preamplifier gain Bit: 0
        SD14GAIN: 3..4 = enum SD14GAIN {
            /// SD14 preamplifier gain select: 0
            SD14GAIN_0 = 0b00,
            /// SD14 preamplifier gain select: 1
            SD14GAIN_1 = 0b01,
            /// SD14 preamplifier gain select: 2
            SD14GAIN_2 = 0b10,
            /// SD14 preamplifier gain select: 3
            SD14GAIN_3 = 0b11,
        }
        /// SD14 Interrupt delay Bit: 0
        SD14INTDLY: 6..7 = enum SD14INTDLY {
            /// SD14 Interrupt delay select: 0
            SD14INTDLY_0 = 0b00,
            /// SD14 Interrupt delay select: 1
            SD14INTDLY_1 = 0b01,
            /// SD14 Interrupt delay select: 2
            SD14INTDLY_2 = 0b10,
            /// SD14 Interrupt delay select: 3
            SD14INTDLY_3 = 0b11,
        }
        /// SD14 digital filter selection
        SD14FILT: 8 = struct SD14FILT(bool);
        /// SD14 rate change factor selection Bit: 0
        SD14RATE: 9..11 = enum SD14RATE {
            /// SD14 rate change factor: 0
            SD14RATE_0 = 0b000,
            /// SD14 rate change factor: 1
            SD14RATE_1 = 0b001,
            /// SD14 rate change factor: 2
            SD14RATE_2 = 0b010,
            /// SD14 rate change factor: 3
            SD14RATE_3 = 0b011,
            /// SD14 rate change factor: 4
            SD14RATE_4 = 0b100,
            /// SD14 rate change factor: 5
            SD14RATE_5 = 0b101,
            /// SD14 rate change factor: 6
            SD14RATE_6 = 0b110,
            /// SD14 rate change factor: 7
            SD14RATE_7 = 0b111,
        }
        /// SD14 Unipolar format enable Bit: 0
        SD14UNI: 12 = struct SD14UNI(bool);
        /// SD14 Two's complement format enable Bit: 0
        SD14TC: 13 = struct SD14TC(bool);
        /// SD14 Resistive bias on TEMP1 enable
        SD14RBEN0: 14 = struct SD14RBEN0(bool);
        /// SD14 Resistive bias on TEMP2 enable
        SD14RBEN1: 15 = struct SD14RBEN1(bool);
    }
    /// SD14 Conversion Result
    rw SD14MEM0 @ 0x04: u16 = 0_0 {
        /// SD14 Conversion Result
        SD14MEM0: 0..15 = struct SD14MEM0Field(u16);
    }
    /// SD14 Intermediate Conversion Result
    rw SD14MEM1 @ 0x06: u16 = 0_0 {
        /// SD14 Intermediate Conversion Result
        SD14MEM1: 0..15 = struct SD14MEM1Field(u16);
    }
    /// SD14 Intermediate Conversion Result
    rw SD14MEM2 @ 0x08: u16 = 0_0 {
        /// SD14 Intermediate Conversion Result
        SD14MEM2: 0..15 = struct SD14MEM2Field(u16);
    }
    /// SD14 Intermediate Conversion Result
    rw SD14MEM3 @ 0x0a: u16 = 0_0 {
        /// SD14 Intermediate Conversion Result
        SD14MEM3: 0..15 = struct SD14MEM3Field(u16);
    }
    /// SD14 Interrupt Vector
    rw SD14IV @ 0x0c: u16 = 0_0 {
        /// SD14IV Interrupt vector value bit: 0
        SD14IV0: 1 = struct SD14IV0(bool);
        /// SD14IV Interrupt vector value bit: 1
        SD14IV1: 2 = struct SD14IV1(bool);
    }
}
