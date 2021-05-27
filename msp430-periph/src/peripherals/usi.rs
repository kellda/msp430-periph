//! USI

utils::periph! {
    /// USI
    USI;
    /// USI  Control Register 0
    rw CTL0 @ 0x00: u8 = 0_0 {
        /// USI  Software Reset
        SWRST: 0 = struct SWRST(bool);
        /// USI  Output Enable
        OE: 1 = struct OE(bool);
        /// USI  General Output Enable Latch
        GE: 2 = struct GE(bool);
        /// USI  Master Select  0:Slave / 1:Master
        MST: 3 = struct MST(bool);
        /// USI  LSB first  1:LSB / 0:MSB
        LSB: 4 = struct LSB(bool);
        /// USI  Port Enable Px.5
        PE5: 5 = struct PE5(bool);
        /// USI  Port Enable Px.6
        PE6: 6 = struct PE6(bool);
        /// USI  Port Enable Px.7
        PE7: 7 = struct PE7(bool);
    }
    /// USI  Control Register 1
    rw CTL1 @ 0x01: u8 = 0_0 {
        /// USI  Counter Interrupt Flag
        IFG: 0 = struct IFG(bool);
        /// USI  START Condition interrupt Flag
        STTIFG: 1 = struct STTIFG(bool);
        /// USI  STOP Condition received
        STP: 2 = struct STP(bool);
        /// USI  Arbitration Lost
        AL: 3 = struct AL(bool);
        /// USI  Counter Interrupt enable
        IE: 4 = struct IE(bool);
        /// USI  START Condition interrupt enable
        STTIE: 5 = struct STTIE(bool);
        /// USI  I2C Mode
        I2C: 6 = struct I2C(bool);
        /// USI  Sync. Mode: Clock Phase
        CKPH: 7 = struct CKPH(bool);
    }
    /// USI  Clock Control Register
    rw CKCTL @ 0x02: u8 = 0_0 {
        /// USI  Software Clock
        SWCLK: 0 = struct SWCLK(bool);
        /// USI  Clock Polarity 0:Inactive=Low / 1:Inactive=High
        CKPL: 1 = struct CKPL(bool);
        /// USI  Clock Source Select 2
        SSEL: 2..4 = enum SSEL {
            /// USI  Clock Source: 0
            SSEL_0 = 0b000,
            /// USI  Clock Source: 1
            SSEL_1 = 0b001,
            /// USI  Clock Source: 2
            SSEL_2 = 0b010,
            /// USI  Clock Source: 3
            SSEL_3 = 0b011,
            /// USI  Clock Source: 4
            SSEL_4 = 0b100,
            /// USI  Clock Source: 5
            SSEL_5 = 0b101,
            /// USI  Clock Source: 6
            SSEL_6 = 0b110,
            /// USI  Clock Source: 7
            SSEL_7 = 0b111,
        }
        /// USI  Clock Divider 2
        DIV: 5..7 = enum DIV {
            /// USI  Clock Divider: 0
            DIV_0 = 0b000,
            /// USI  Clock Divider: 1
            DIV_1 = 0b001,
            /// USI  Clock Divider: 2
            DIV_2 = 0b010,
            /// USI  Clock Divider: 3
            DIV_3 = 0b011,
            /// USI  Clock Divider: 4
            DIV_4 = 0b100,
            /// USI  Clock Divider: 5
            DIV_5 = 0b101,
            /// USI  Clock Divider: 6
            DIV_6 = 0b110,
            /// USI  Clock Divider: 7
            DIV_7 = 0b111,
        }
    }
    /// USI  Bit Counter Register
    rw CNT @ 0x03: u8 = 0_0 {
        /// USI  Bit Count 0
        CNT0: 0 = struct CNT0(bool);
        /// USI  Bit Count 1
        CNT1: 1 = struct CNT1(bool);
        /// USI  Bit Count 2
        CNT2: 2 = struct CNT2(bool);
        /// USI  Bit Count 3
        CNT3: 3 = struct CNT3(bool);
        /// USI  Bit Count 4
        CNT4: 4 = struct CNT4(bool);
        /// USI  Interrupt Flag Clear Control
        IFGCC: 5 = struct IFGCC(bool);
        /// USI  16 Bit Shift Register Enable
        _16B: 6 = struct _16B(bool);
        /// USI  SCL Released
        SCLREL: 7 = struct SCLREL(bool);
    }
    /// USI  Low Byte Shift Register
    rw SRL @ 0x04: u8 = 0_0 {
        /// USI  Low Byte Shift Register
        SRL: 0..7 = struct SRLField(u8);
    }
    /// USI  High Byte Shift Register
    rw SRH @ 0x05: u8 = 0_0 {
        /// USI  High Byte Shift Register
        SRH: 0..7 = struct SRHField(u8);
    }
    /// USI  Control Register
    rw CTL @ 0x00: u16 = 0_0 {
        /// USI  Control Register
        CTL: 0..15 = struct CTLField(u16);
    }
    /// USI  Clock and Counter Control Register
    rw CCTL @ 0x02: u16 = 0_0 {
        /// USI  Clock and Counter Control Register
        CCTL: 0..15 = struct CCTLField(u16);
    }
    /// USI  Shift Register
    rw SR @ 0x04: u16 = 0_0 {
        /// USI  Shift Register
        SR: 0..15 = struct SRField(u16);
    }
}
