//! USI

utils::periph! {
    /// USI
    USI;
    /// USI  Control Register 0
    rw USICTL0 @ 0x00: u8 = 0_0 {
        /// USI  Software Reset
        USISWRST: 0 = struct USISWRST(bool);
        /// USI  Output Enable
        USIOE: 1 = struct USIOE(bool);
        /// USI  General Output Enable Latch
        USIGE: 2 = struct USIGE(bool);
        /// USI  Master Select  0:Slave / 1:Master
        USIMST: 3 = struct USIMST(bool);
        /// USI  LSB first  1:LSB / 0:MSB
        USILSB: 4 = struct USILSB(bool);
        /// USI  Port Enable Px.5
        USIPE5: 5 = struct USIPE5(bool);
        /// USI  Port Enable Px.6
        USIPE6: 6 = struct USIPE6(bool);
        /// USI  Port Enable Px.7
        USIPE7: 7 = struct USIPE7(bool);
    }
    /// USI  Control Register 1
    rw USICTL1 @ 0x01: u8 = 0_0 {
        /// USI  Counter Interrupt Flag
        USIIFG: 0 = struct USIIFG(bool);
        /// USI  START Condition interrupt Flag
        USISTTIFG: 1 = struct USISTTIFG(bool);
        /// USI  STOP Condition received
        USISTP: 2 = struct USISTP(bool);
        /// USI  Arbitration Lost
        USIAL: 3 = struct USIAL(bool);
        /// USI  Counter Interrupt enable
        USIIE: 4 = struct USIIE(bool);
        /// USI  START Condition interrupt enable
        USISTTIE: 5 = struct USISTTIE(bool);
        /// USI  I2C Mode
        USII2C: 6 = struct USII2C(bool);
        /// USI  Sync. Mode: Clock Phase
        USICKPH: 7 = struct USICKPH(bool);
    }
    /// USI  Clock Control Register
    rw USICKCTL @ 0x02: u8 = 0_0 {
        /// USI  Software Clock
        USISWCLK: 0 = struct USISWCLK(bool);
        /// USI  Clock Polarity 0:Inactive=Low / 1:Inactive=High
        USICKPL: 1 = struct USICKPL(bool);
        /// USI  Clock Source Select 2
        USISSEL: 2..4 = enum USISSEL {
            /// USI  Clock Source: 0
            USISSEL_0 = 0b000,
            /// USI  Clock Source: 1
            USISSEL_1 = 0b001,
            /// USI  Clock Source: 2
            USISSEL_2 = 0b010,
            /// USI  Clock Source: 3
            USISSEL_3 = 0b011,
            /// USI  Clock Source: 4
            USISSEL_4 = 0b100,
            /// USI  Clock Source: 5
            USISSEL_5 = 0b101,
            /// USI  Clock Source: 6
            USISSEL_6 = 0b110,
            /// USI  Clock Source: 7
            USISSEL_7 = 0b111,
        }
        /// USI  Clock Divider 2
        USIDIV: 5..7 = enum USIDIV {
            /// USI  Clock Divider: 0
            USIDIV_0 = 0b000,
            /// USI  Clock Divider: 1
            USIDIV_1 = 0b001,
            /// USI  Clock Divider: 2
            USIDIV_2 = 0b010,
            /// USI  Clock Divider: 3
            USIDIV_3 = 0b011,
            /// USI  Clock Divider: 4
            USIDIV_4 = 0b100,
            /// USI  Clock Divider: 5
            USIDIV_5 = 0b101,
            /// USI  Clock Divider: 6
            USIDIV_6 = 0b110,
            /// USI  Clock Divider: 7
            USIDIV_7 = 0b111,
        }
    }
    /// USI  Bit Counter Register
    rw USICNT @ 0x03: u8 = 0_0 {
        /// USI  Bit Count 0
        USICNT0: 0 = struct USICNT0(bool);
        /// USI  Bit Count 1
        USICNT1: 1 = struct USICNT1(bool);
        /// USI  Bit Count 2
        USICNT2: 2 = struct USICNT2(bool);
        /// USI  Bit Count 3
        USICNT3: 3 = struct USICNT3(bool);
        /// USI  Bit Count 4
        USICNT4: 4 = struct USICNT4(bool);
        /// USI  Interrupt Flag Clear Control
        USIIFGCC: 5 = struct USIIFGCC(bool);
        /// USI  16 Bit Shift Register Enable
        USI16B: 6 = struct USI16B(bool);
        /// USI  SCL Released
        USISCLREL: 7 = struct USISCLREL(bool);
    }
    /// USI  Low Byte Shift Register
    rw USISRL @ 0x04: u8 = 0_0 {
        /// USI  Low Byte Shift Register
        USISRL: 0..7 = struct USISRLField(u8);
    }
    /// USI  High Byte Shift Register
    rw USISRH @ 0x05: u8 = 0_0 {
        /// USI  High Byte Shift Register
        USISRH: 0..7 = struct USISRHField(u8);
    }
    /// USI  Control Register
    rw USICTL @ 0x00: u16 = 0_0 {
        /// USI  Control Register
        USICTL: 0..15 = struct USICTLField(u16);
    }
    /// USI  Clock and Counter Control Register
    rw USICCTL @ 0x02: u16 = 0_0 {
        /// USI  Clock and Counter Control Register
        USICCTL: 0..15 = struct USICCTLField(u16);
    }
    /// USI  Shift Register
    rw USISR @ 0x04: u16 = 0_0 {
        /// USI  Shift Register
        USISR: 0..15 = struct USISRField(u16);
    }
}
