//! USCI_B  I2C Mode

utils::periph! {
    /// USCI_B  I2C Mode
    USCI_B_I2C;
    /// USCI B Control Word Register 0
    rw CTLW0 @ 0x00: u16 = 0_0 {
        /// USCI B Control Word Register 0
        CTLW0: 0..15 = struct CTLW0Field(u16);
    }
    /// USCI B Control Register 0
    rw CTL0 @ 0x01: u8 = 0_0 {
        /// Sync-Mode  0:UART-Mode / 1:SPI-Mode
        SYNC: 0 = struct SYNC(bool);
        /// Sync. Mode: USCI Mode 1
        MODE: 1..2 = enum MODE {
            /// Sync. Mode: USCI Mode: 0
            MODE_0 = 0b00,
            /// Sync. Mode: USCI Mode: 1
            MODE_1 = 0b01,
            /// Sync. Mode: USCI Mode: 2
            MODE_2 = 0b10,
            /// Sync. Mode: USCI Mode: 3
            MODE_3 = 0b11,
        }
        /// Sync. Mode: Master Select
        MST: 3 = struct MST(bool);
        /// Multi-Master Environment
        MM: 5 = struct MM(bool);
        /// 10-bit Slave Address Mode
        SLA10: 6 = struct SLA10(bool);
        /// 10-bit Address Mode
        A10: 7 = struct A10(bool);
    }
    /// USCI B Control Register 1
    rw CTL1 @ 0x00: u8 = 0_0 {
        /// USCI Software Reset
        SWRST: 0 = struct SWRST(bool);
        /// Transmit START
        TXSTT: 1 = struct TXSTT(bool);
        /// Transmit STOP
        TXSTP: 2 = struct TXSTP(bool);
        /// Transmit NACK
        TXNACK: 3 = struct TXNACK(bool);
        /// Transmit/Receive Select/Flag
        TR: 4 = struct TR(bool);
        /// USCI 1 Clock Source Select 1
        SSEL: 6..7 = enum SSEL {
            /// USCI 0 Clock Source: 0
            SSEL_0 = 0b00,
            /// USCI 0 Clock Source: 1
            SSEL_1 = 0b01,
            /// USCI 0 Clock Source: 2
            SSEL_2 = 0b10,
            /// USCI 0 Clock Source: 3
            SSEL_3 = 0b11,
        }
    }
    /// USCI B Baud Word Rate 0
    rw BRW @ 0x06: u16 = 0_0 {
        /// USCI B Baud Word Rate 0
        BRW: 0..15 = struct BRWField(u16);
    }
    /// USCI B Baud Rate 0
    rw BR0 @ 0x06: u8 = 0_0 {
        /// USCI B Baud Rate 0
        BR0: 0..7 = struct BR0Field(u8);
    }
    /// USCI B Baud Rate 1
    rw BR1 @ 0x07: u8 = 0_0 {
        /// USCI B Baud Rate 1
        BR1: 0..7 = struct BR1Field(u8);
    }
    /// USCI B Status Register
    rw STAT @ 0x0a: u8 = 0_0 {
        /// Bus Busy Flag
        BUSY: 4 = struct BUSY(bool);
        /// General Call address received Flag
        GC: 5 = struct GC(bool);
        /// SCL low
        SCLLOW: 6 = struct SCLLOW(bool);
        /// USCI Listen mode
        LISTEN: 7 = struct LISTEN(bool);
    }
    /// USCI B Receive Buffer
    rw RXBUF @ 0x0c: u8 = 0_0 {
        /// USCI B Receive Buffer
        RXBUF: 0..7 = struct RXBUFField(u8);
    }
    /// USCI B Transmit Buffer
    rw TXBUF @ 0x0e: u8 = 0_0 {
        /// USCI B Transmit Buffer
        TXBUF: 0..7 = struct TXBUFField(u8);
    }
    /// USCI B I2C Own Address
    rw I2COA @ 0x10: u16 = 0_0 {
        /// I2C Own Address 0
        OA0: 0 = struct OA0(bool);
        /// I2C Own Address 1
        OA1: 1 = struct OA1(bool);
        /// I2C Own Address 2
        OA2: 2 = struct OA2(bool);
        /// I2C Own Address 3
        OA3: 3 = struct OA3(bool);
        /// I2C Own Address 4
        OA4: 4 = struct OA4(bool);
        /// I2C Own Address 5
        OA5: 5 = struct OA5(bool);
        /// I2C Own Address 6
        OA6: 6 = struct OA6(bool);
        /// I2C Own Address 7
        OA7: 7 = struct OA7(bool);
        /// I2C Own Address 8
        OA8: 8 = struct OA8(bool);
        /// I2C Own Address 9
        OA9: 9 = struct OA9(bool);
        /// I2C General Call enable
        GCEN: 15 = struct GCEN(bool);
    }
    /// USCI B I2C Slave Address
    rw I2CSA @ 0x12: u16 = 0_0 {
        /// I2C Slave Address 0
        SA0: 0 = struct SA0(bool);
        /// I2C Slave Address 1
        SA1: 1 = struct SA1(bool);
        /// I2C Slave Address 2
        SA2: 2 = struct SA2(bool);
        /// I2C Slave Address 3
        SA3: 3 = struct SA3(bool);
        /// I2C Slave Address 4
        SA4: 4 = struct SA4(bool);
        /// I2C Slave Address 5
        SA5: 5 = struct SA5(bool);
        /// I2C Slave Address 6
        SA6: 6 = struct SA6(bool);
        /// I2C Slave Address 7
        SA7: 7 = struct SA7(bool);
        /// I2C Slave Address 8
        SA8: 8 = struct SA8(bool);
        /// I2C Slave Address 9
        SA9: 9 = struct SA9(bool);
    }
    /// USCI B Interrupt Enable Register
    rw ICTL @ 0x1c: u16 = 0_0 {
        /// USCI B Interrupt Enable Register
        ICTL: 0..15 = struct ICTLField(u16);
    }
    /// USCI B Interrupt Enable Register
    rw IE @ 0x1c: u8 = 0_0 {
        /// USCI Receive Interrupt Enable
        RXIE: 0 = struct RXIE(bool);
        /// USCI Transmit Interrupt Enable
        TXIE: 1 = struct TXIE(bool);
        /// START Condition interrupt enable
        STTIE: 2 = struct STTIE(bool);
        /// STOP Condition interrupt enable
        STPIE: 3 = struct STPIE(bool);
        /// Arbitration Lost interrupt enable
        ALIE: 4 = struct ALIE(bool);
        /// NACK Condition interrupt enable
        NACKIE: 5 = struct NACKIE(bool);
    }
    /// USCI B Interrupt Flags Register
    rw IFG @ 0x1d: u8 = 0_0 {
        /// USCI Receive Interrupt Flag
        RXIFG: 0 = struct RXIFG(bool);
        /// USCI Transmit Interrupt Flag
        TXIFG: 1 = struct TXIFG(bool);
        /// START Condition interrupt Flag
        STTIFG: 2 = struct STTIFG(bool);
        /// STOP Condition interrupt Flag
        STPIFG: 3 = struct STPIFG(bool);
        /// Arbitration Lost interrupt Flag
        ALIFG: 4 = struct ALIFG(bool);
        /// NAK Condition interrupt Flag
        NACKIFG: 5 = struct NACKIFG(bool);
    }
    /// USCI B Interrupt Vector Register
    rw IV @ 0x1e: u16 = 0_0 {
        /// USCI B Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
}
