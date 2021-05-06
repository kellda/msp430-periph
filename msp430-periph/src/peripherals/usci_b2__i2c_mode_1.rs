//! USCI_B2  I2C Mode

utils::periph! {
    /// USCI_B2  I2C Mode
    USCI_B2I2CMode;
    /// USCI B2 Control Word Register 0
    rw UCB2CTLW0 @ 0x00: u16 = 0_0 {
        /// USCI B2 Control Word Register 0
        UCB2CTLW0: 0..15 = struct UCB2CTLW0Field(u16);
    }
    /// USCI B2 Control Register 0
    rw UCB2CTL0 @ 0x01: u8 = 0_0 {
        /// Sync-Mode  0:UART-Mode / 1:SPI-Mode
        UCSYNC: 0 = struct UCSYNC(bool);
        /// Sync. Mode: USCI Mode 1
        UCMODE: 1..2 = enum UCMODE {
            /// Sync. Mode: USCI Mode: 0
            UCMODE_0 = 0b00,
            /// Sync. Mode: USCI Mode: 1
            UCMODE_1 = 0b01,
            /// Sync. Mode: USCI Mode: 2
            UCMODE_2 = 0b10,
            /// Sync. Mode: USCI Mode: 3
            UCMODE_3 = 0b11,
        }
        /// Sync. Mode: Master Select
        UCMST: 3 = struct UCMST(bool);
        /// Multi-Master Environment
        UCMM: 5 = struct UCMM(bool);
        /// 10-bit Slave Address Mode
        UCSLA10: 6 = struct UCSLA10(bool);
        /// 10-bit Address Mode
        UCA10: 7 = struct UCA10(bool);
    }
    /// USCI B2 Control Register 1
    rw UCB2CTL1 @ 0x00: u8 = 0_0 {
        /// USCI Software Reset
        UCSWRST: 0 = struct UCSWRST(bool);
        /// Transmit START
        UCTXSTT: 1 = struct UCTXSTT(bool);
        /// Transmit STOP
        UCTXSTP: 2 = struct UCTXSTP(bool);
        /// Transmit NACK
        UCTXNACK: 3 = struct UCTXNACK(bool);
        /// Transmit/Receive Select/Flag
        UCTR: 4 = struct UCTR(bool);
        /// USCI 1 Clock Source Select 1
        UCSSEL: 6..7 = enum UCSSEL {
            /// USCI 0 Clock Source: 0
            UCSSEL_0 = 0b00,
            /// USCI 0 Clock Source: 1
            UCSSEL_1 = 0b01,
            /// USCI 0 Clock Source: 2
            UCSSEL_2 = 0b10,
            /// USCI 0 Clock Source: 3
            UCSSEL_3 = 0b11,
        }
    }
    /// USCI B2 Baud Word Rate 0
    rw UCB2BRW @ 0x06: u16 = 0_0 {
        /// USCI B2 Baud Word Rate 0
        UCB2BRW: 0..15 = struct UCB2BRWField(u16);
    }
    /// USCI B2 Baud Rate 0
    rw UCB2BR0 @ 0x06: u8 = 0_0 {
        /// USCI B2 Baud Rate 0
        UCB2BR0: 0..7 = struct UCB2BR0Field(u8);
    }
    /// USCI B2 Baud Rate 1
    rw UCB2BR1 @ 0x07: u8 = 0_0 {
        /// USCI B2 Baud Rate 1
        UCB2BR1: 0..7 = struct UCB2BR1Field(u8);
    }
    /// USCI B2 Status Register
    rw UCB2STAT @ 0x0a: u8 = 0_0 {
        /// Bus Busy Flag
        UCBBUSY: 4 = struct UCBBUSY(bool);
        /// General Call address received Flag
        UCGC: 5 = struct UCGC(bool);
        /// SCL low
        UCSCLLOW: 6 = struct UCSCLLOW(bool);
        /// USCI Listen mode
        UCLISTEN: 7 = struct UCLISTEN(bool);
    }
    /// USCI B2 Receive Buffer
    rw UCB2RXBUF @ 0x0c: u8 = 0_0 {
        /// USCI B2 Receive Buffer
        UCB2RXBUF: 0..7 = struct UCB2RXBUFField(u8);
    }
    /// USCI B2 Transmit Buffer
    rw UCB2TXBUF @ 0x0e: u8 = 0_0 {
        /// USCI B2 Transmit Buffer
        UCB2TXBUF: 0..7 = struct UCB2TXBUFField(u8);
    }
    /// USCI B2 I2C Own Address
    rw UCB2I2COA @ 0x10: u16 = 0_0 {
        /// I2C Own Address 0
        UCOA0: 0 = struct UCOA0(bool);
        /// I2C Own Address 1
        UCOA1: 1 = struct UCOA1(bool);
        /// I2C Own Address 2
        UCOA2: 2 = struct UCOA2(bool);
        /// I2C Own Address 3
        UCOA3: 3 = struct UCOA3(bool);
        /// I2C Own Address 4
        UCOA4: 4 = struct UCOA4(bool);
        /// I2C Own Address 5
        UCOA5: 5 = struct UCOA5(bool);
        /// I2C Own Address 6
        UCOA6: 6 = struct UCOA6(bool);
        /// I2C Own Address 7
        UCOA7: 7 = struct UCOA7(bool);
        /// I2C Own Address 8
        UCOA8: 8 = struct UCOA8(bool);
        /// I2C Own Address 9
        UCOA9: 9 = struct UCOA9(bool);
        /// I2C General Call enable
        UCGCEN: 15 = struct UCGCEN(bool);
    }
    /// USCI B2 I2C Slave Address
    rw UCB2I2CSA @ 0x12: u16 = 0_0 {
        /// I2C Slave Address 0
        UCSA0: 0 = struct UCSA0(bool);
        /// I2C Slave Address 1
        UCSA1: 1 = struct UCSA1(bool);
        /// I2C Slave Address 2
        UCSA2: 2 = struct UCSA2(bool);
        /// I2C Slave Address 3
        UCSA3: 3 = struct UCSA3(bool);
        /// I2C Slave Address 4
        UCSA4: 4 = struct UCSA4(bool);
        /// I2C Slave Address 5
        UCSA5: 5 = struct UCSA5(bool);
        /// I2C Slave Address 6
        UCSA6: 6 = struct UCSA6(bool);
        /// I2C Slave Address 7
        UCSA7: 7 = struct UCSA7(bool);
        /// I2C Slave Address 8
        UCSA8: 8 = struct UCSA8(bool);
        /// I2C Slave Address 9
        UCSA9: 9 = struct UCSA9(bool);
    }
    /// USCI B2 Interrupt Enable Register
    rw UCB2ICTL @ 0x1c: u16 = 0_0 {
        /// USCI B2 Interrupt Enable Register
        UCB2ICTL: 0..15 = struct UCB2ICTLField(u16);
    }
    /// USCI B2 Interrupt Enable Register
    rw UCB2IE @ 0x1c: u8 = 0_0 {
        /// USCI Receive Interrupt Enable
        UCRXIE: 0 = struct UCRXIE(bool);
        /// USCI Transmit Interrupt Enable
        UCTXIE: 1 = struct UCTXIE(bool);
        /// START Condition interrupt enable
        UCSTTIE: 2 = struct UCSTTIE(bool);
        /// STOP Condition interrupt enable
        UCSTPIE: 3 = struct UCSTPIE(bool);
        /// Arbitration Lost interrupt enable
        UCALIE: 4 = struct UCALIE(bool);
        /// NACK Condition interrupt enable
        UCNACKIE: 5 = struct UCNACKIE(bool);
    }
    /// USCI B2 Interrupt Flags Register
    rw UCB2IFG @ 0x1d: u8 = 0_0 {
        /// USCI Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// USCI Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
        /// START Condition interrupt Flag
        UCSTTIFG: 2 = struct UCSTTIFG(bool);
        /// STOP Condition interrupt Flag
        UCSTPIFG: 3 = struct UCSTPIFG(bool);
        /// Arbitration Lost interrupt Flag
        UCALIFG: 4 = struct UCALIFG(bool);
        /// NAK Condition interrupt Flag
        UCNACKIFG: 5 = struct UCNACKIFG(bool);
    }
    /// USCI B2 Interrupt Vector Register
    rw UCB2IV @ 0x1e: u16 = 0_0 {
        /// USCI B2 Interrupt Vector Register
        UCB2IV: 0..15 = struct UCB2IVField(u16);
    }
}
