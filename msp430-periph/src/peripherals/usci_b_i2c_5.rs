//! USCI_B  I2C Mode

utils::periph! {
    /// USCI_B  I2C Mode
    USCI_B_I2C;
    /// USCI B Control Word Register 0
    rw CTLW0 @ 0x00: u16 = 0_0 {
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
        /// Transmit ACK
        TXACK: 5 = struct TXACK(bool);
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
        /// Sync. Mode: USCI Mode 1
        MODE: 9..10 = enum MODE {
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
        MST: 11 = struct MST(bool);
        /// Multi-Master Environment
        MM: 13 = struct MM(bool);
        /// 10-bit Slave Address Mode
        SLA10: 14 = struct SLA10(bool);
        /// 10-bit Address Mode
        A10: 15 = struct A10(bool);
    }
    /// USCI B Control Register 0
    rw CTL0 @ 0x01: u8 = 0_0 {
        /// USCI B Control Register 0
        CTL0: 0..7 = struct CTL0Field(u8);
    }
    /// USCI B Control Register 1
    rw CTL1 @ 0x00: u8 = 0_0 {
        /// USCI B Control Register 1
        CTL1: 0..7 = struct CTL1Field(u8);
    }
    /// USCI B Control Word Register 1
    rw CTLW1 @ 0x02: u16 = 0_0 {
        /// USCI Deglitch time Bit: 1
        GLIT: 0..1 = enum GLIT {
            /// USCI Deglitch time: 0
            GLIT_0 = 0b00,
            /// USCI Deglitch time: 1
            GLIT_1 = 0b01,
            /// USCI Deglitch time: 2
            GLIT_2 = 0b10,
            /// USCI Deglitch time: 3
            GLIT_3 = 0b11,
        }
        /// USCI Automatic Stop condition generation Bit: 1
        ASTP: 2..3 = enum ASTP {
            /// USCI Automatic Stop condition generation: 0
            ASTP_0 = 0b00,
            /// USCI Automatic Stop condition generation: 1
            ASTP_1 = 0b01,
            /// USCI Automatic Stop condition generation: 2
            ASTP_2 = 0b10,
            /// USCI Automatic Stop condition generation: 3
            ASTP_3 = 0b11,
        }
        /// USCI Software controlled ACK
        SWACK: 4 = struct SWACK(bool);
        /// USCI Acknowledge Stop last byte
        STPNACK: 5 = struct STPNACK(bool);
        /// USCI Clock low timeout Bit: 1
        CLTO: 6..7 = enum CLTO {
            /// USCI Clock low timeout: 0
            CLTO_0 = 0b00,
            /// USCI Clock low timeout: 1
            CLTO_1 = 0b01,
            /// USCI Clock low timeout: 2
            CLTO_2 = 0b10,
            /// USCI Clock low timeout: 3
            CLTO_3 = 0b11,
        }
        /// USCI Early TXIFG0
        ETXINT: 8 = struct ETXINT(bool);
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
    /// USCI B Status Word Register
    rw STATW @ 0x08: u16 = 0_0 {
        /// USCI B Status Word Register
        STATW: 0..15 = struct STATWField(u16);
    }
    /// USCI B Status Register
    rw STATW_I2C @ 0x08: u16 = 0_0 {
        /// Bus Busy Flag
        BBUSY: 4 = struct WBBUSY(bool);
        /// General Call address received Flag
        GC: 5 = struct GC(bool);
        /// SCL low
        SCLLOW: 6 = struct WSCLLOW(bool);
        /// USCI Byte Counter Bit 0
        BCNT0: 8 = struct WBCNT0(bool);
        /// USCI Byte Counter Bit 1
        BCNT1: 9 = struct WBCNT1(bool);
        /// USCI Byte Counter Bit 2
        BCNT2: 10 = struct WBCNT2(bool);
        /// USCI Byte Counter Bit 3
        BCNT3: 11 = struct WBCNT3(bool);
        /// USCI Byte Counter Bit 4
        BCNT4: 12 = struct WBCNT4(bool);
        /// USCI Byte Counter Bit 5
        BCNT5: 13 = struct WBCNT5(bool);
        /// USCI Byte Counter Bit 6
        BCNT6: 14 = struct WBCNT6(bool);
        /// USCI Byte Counter Bit 7
        BCNT7: 15 = struct WBCNT7(bool);
    }
    /// USCI B Status Register
    rw STAT_I2C @ 0x08: u8 = 0_0 {
        /// Bus Busy Flag
        BBUSY: 4 = struct BBUSY(bool);
        /// General Call address received Flag
        GC_I2C: 5 = struct GC_I2C(bool);
        /// SCL low
        SCLLOW: 6 = struct SCLLOW(bool);
    }
    /// USCI B Byte Counter Register
    rw BCNT_I2C @ 0x09: u8 = 0_0 {
        /// USCI Byte Counter Bit 0
        BCNT0: 0 = struct BCNT0(bool);
        /// USCI Byte Counter Bit 1
        BCNT1: 1 = struct BCNT1(bool);
        /// USCI Byte Counter Bit 2
        BCNT2: 2 = struct BCNT2(bool);
        /// USCI Byte Counter Bit 3
        BCNT3: 3 = struct BCNT3(bool);
        /// USCI Byte Counter Bit 4
        BCNT4: 4 = struct BCNT4(bool);
        /// USCI Byte Counter Bit 5
        BCNT5: 5 = struct BCNT5(bool);
        /// USCI Byte Counter Bit 6
        BCNT6: 6 = struct BCNT6(bool);
        /// USCI Byte Counter Bit 7
        BCNT7: 7 = struct BCNT7(bool);
    }
    /// USCI B Byte Counter Threshold Register
    rw TBCNT @ 0x0a: u16 = 0_0 {
        /// USCI B Byte Counter Threshold Register
        TBCNT: 0..15 = struct TBCNTField(u16);
    }
    /// USCI B Receive Buffer
    rw RXBUF @ 0x0c: u16 = 0_0 {
        /// USCI B Receive Buffer
        RXBUF: 0..15 = struct RXBUFField(u16);
    }
    /// USCI B Transmit Buffer
    rw TXBUF @ 0x0e: u16 = 0_0 {
        /// USCI B Transmit Buffer
        TXBUF: 0..15 = struct TXBUFField(u16);
    }
    /// USCI B I2C Own Address 0
    rw I2COA0 @ 0x14: u16 = 0_0 {
        /// I2C Own Address Bit 0
        I2COA0_UCOA0: 0 = struct I2COA0_UCOA0(bool);
        /// I2C Own Address Bit 1
        I2COA0_UCOA1: 1 = struct I2COA0_UCOA1(bool);
        /// I2C Own Address Bit 2
        I2COA0_UCOA2: 2 = struct I2COA0_UCOA2(bool);
        /// I2C Own Address Bit 3
        I2COA0_UCOA3: 3 = struct I2COA0_UCOA3(bool);
        /// I2C Own Address Bit 4
        I2COA0_UCOA4: 4 = struct I2COA0_UCOA4(bool);
        /// I2C Own Address Bit 5
        I2COA0_UCOA5: 5 = struct I2COA0_UCOA5(bool);
        /// I2C Own Address Bit 6
        I2COA0_UCOA6: 6 = struct I2COA0_UCOA6(bool);
        /// I2C Own Address Bit 7
        I2COA0_UCOA7: 7 = struct I2COA0_UCOA7(bool);
        /// I2C Own Address Bit 8
        I2COA0_UCOA8: 8 = struct I2COA0_UCOA8(bool);
        /// I2C Own Address Bit 9
        I2COA0_UCOA9: 9 = struct I2COA0_UCOA9(bool);
        /// I2C Own Address enable
        I2COA0_UCOAEN: 10 = struct I2COA0_UCOAEN(bool);
        /// I2C General Call enable
        GCEN: 15 = struct GCEN(bool);
    }
    /// USCI B I2C Own Address 1
    rw I2COA1 @ 0x16: u16 = 0_0 {
        /// I2C Own Address Bit 0
        I2COA1_UCOA0: 0 = struct I2COA1_UCOA0(bool);
        /// I2C Own Address Bit 1
        I2COA1_UCOA1: 1 = struct I2COA1_UCOA1(bool);
        /// I2C Own Address Bit 2
        I2COA1_UCOA2: 2 = struct I2COA1_UCOA2(bool);
        /// I2C Own Address Bit 3
        I2COA1_UCOA3: 3 = struct I2COA1_UCOA3(bool);
        /// I2C Own Address Bit 4
        I2COA1_UCOA4: 4 = struct I2COA1_UCOA4(bool);
        /// I2C Own Address Bit 5
        I2COA1_UCOA5: 5 = struct I2COA1_UCOA5(bool);
        /// I2C Own Address Bit 6
        I2COA1_UCOA6: 6 = struct I2COA1_UCOA6(bool);
        /// I2C Own Address Bit 7
        I2COA1_UCOA7: 7 = struct I2COA1_UCOA7(bool);
        /// I2C Own Address Bit 8
        I2COA1_UCOA8: 8 = struct I2COA1_UCOA8(bool);
        /// I2C Own Address Bit 9
        I2COA1_UCOA9: 9 = struct I2COA1_UCOA9(bool);
        /// I2C Own Address enable
        I2COA1_UCOAEN: 10 = struct I2COA1_UCOAEN(bool);
    }
    /// USCI B I2C Own Address 2
    rw I2COA2 @ 0x18: u16 = 0_0 {
        /// I2C Own Address Bit 0
        I2COA2_UCOA0: 0 = struct I2COA2_UCOA0(bool);
        /// I2C Own Address Bit 1
        I2COA2_UCOA1: 1 = struct I2COA2_UCOA1(bool);
        /// I2C Own Address Bit 2
        I2COA2_UCOA2: 2 = struct I2COA2_UCOA2(bool);
        /// I2C Own Address Bit 3
        I2COA2_UCOA3: 3 = struct I2COA2_UCOA3(bool);
        /// I2C Own Address Bit 4
        I2COA2_UCOA4: 4 = struct I2COA2_UCOA4(bool);
        /// I2C Own Address Bit 5
        I2COA2_UCOA5: 5 = struct I2COA2_UCOA5(bool);
        /// I2C Own Address Bit 6
        I2COA2_UCOA6: 6 = struct I2COA2_UCOA6(bool);
        /// I2C Own Address Bit 7
        I2COA2_UCOA7: 7 = struct I2COA2_UCOA7(bool);
        /// I2C Own Address Bit 8
        I2COA2_UCOA8: 8 = struct I2COA2_UCOA8(bool);
        /// I2C Own Address Bit 9
        I2COA2_UCOA9: 9 = struct I2COA2_UCOA9(bool);
        /// I2C Own Address enable
        I2COA2_UCOAEN: 10 = struct I2COA2_UCOAEN(bool);
    }
    /// USCI B I2C Own Address 3
    rw I2COA3 @ 0x1a: u16 = 0_0 {
        /// I2C Own Address Bit 0
        I2COA3_UCOA0: 0 = struct I2COA3_UCOA0(bool);
        /// I2C Own Address Bit 1
        I2COA3_UCOA1: 1 = struct I2COA3_UCOA1(bool);
        /// I2C Own Address Bit 2
        I2COA3_UCOA2: 2 = struct I2COA3_UCOA2(bool);
        /// I2C Own Address Bit 3
        I2COA3_UCOA3: 3 = struct I2COA3_UCOA3(bool);
        /// I2C Own Address Bit 4
        I2COA3_UCOA4: 4 = struct I2COA3_UCOA4(bool);
        /// I2C Own Address Bit 5
        I2COA3_UCOA5: 5 = struct I2COA3_UCOA5(bool);
        /// I2C Own Address Bit 6
        I2COA3_UCOA6: 6 = struct I2COA3_UCOA6(bool);
        /// I2C Own Address Bit 7
        I2COA3_UCOA7: 7 = struct I2COA3_UCOA7(bool);
        /// I2C Own Address Bit 8
        I2COA3_UCOA8: 8 = struct I2COA3_UCOA8(bool);
        /// I2C Own Address Bit 9
        I2COA3_UCOA9: 9 = struct I2COA3_UCOA9(bool);
        /// I2C Own Address enable
        I2COA3_UCOAEN: 10 = struct I2COA3_UCOAEN(bool);
    }
    /// USCI B Received Address Register
    rw ADDRX @ 0x1c: u16 = 0_0 {
        /// I2C Receive Address Bit 0
        ADDRX0: 0 = struct ADDRX0(bool);
        /// I2C Receive Address Bit 1
        ADDRX1: 1 = struct ADDRX1(bool);
        /// I2C Receive Address Bit 2
        ADDRX2: 2 = struct ADDRX2(bool);
        /// I2C Receive Address Bit 3
        ADDRX3: 3 = struct ADDRX3(bool);
        /// I2C Receive Address Bit 4
        ADDRX4: 4 = struct ADDRX4(bool);
        /// I2C Receive Address Bit 5
        ADDRX5: 5 = struct ADDRX5(bool);
        /// I2C Receive Address Bit 6
        ADDRX6: 6 = struct ADDRX6(bool);
        /// I2C Receive Address Bit 7
        ADDRX7: 7 = struct ADDRX7(bool);
        /// I2C Receive Address Bit 8
        ADDRX8: 8 = struct ADDRX8(bool);
        /// I2C Receive Address Bit 9
        ADDRX9: 9 = struct ADDRX9(bool);
    }
    /// USCI B Address Mask Register
    rw ADDMASK @ 0x1e: u16 = 0_0 {
        /// I2C Address Mask Bit 0
        ADDMASK0: 0 = struct ADDMASK0(bool);
        /// I2C Address Mask Bit 1
        ADDMASK1: 1 = struct ADDMASK1(bool);
        /// I2C Address Mask Bit 2
        ADDMASK2: 2 = struct ADDMASK2(bool);
        /// I2C Address Mask Bit 3
        ADDMASK3: 3 = struct ADDMASK3(bool);
        /// I2C Address Mask Bit 4
        ADDMASK4: 4 = struct ADDMASK4(bool);
        /// I2C Address Mask Bit 5
        ADDMASK5: 5 = struct ADDMASK5(bool);
        /// I2C Address Mask Bit 6
        ADDMASK6: 6 = struct ADDMASK6(bool);
        /// I2C Address Mask Bit 7
        ADDMASK7: 7 = struct ADDMASK7(bool);
        /// I2C Address Mask Bit 8
        ADDMASK8: 8 = struct ADDMASK8(bool);
        /// I2C Address Mask Bit 9
        ADDMASK9: 9 = struct ADDMASK9(bool);
    }
    /// USCI B I2C Slave Address
    rw I2CSA @ 0x20: u16 = 0_0 {
        /// I2C Slave Address Bit 0
        SA0: 0 = struct SA0(bool);
        /// I2C Slave Address Bit 1
        SA1: 1 = struct SA1(bool);
        /// I2C Slave Address Bit 2
        SA2: 2 = struct SA2(bool);
        /// I2C Slave Address Bit 3
        SA3: 3 = struct SA3(bool);
        /// I2C Slave Address Bit 4
        SA4: 4 = struct SA4(bool);
        /// I2C Slave Address Bit 5
        SA5: 5 = struct SA5(bool);
        /// I2C Slave Address Bit 6
        SA6: 6 = struct SA6(bool);
        /// I2C Slave Address Bit 7
        SA7: 7 = struct SA7(bool);
        /// I2C Slave Address Bit 8
        SA8: 8 = struct SA8(bool);
        /// I2C Slave Address Bit 9
        SA9: 9 = struct SA9(bool);
    }
    /// USCI B Interrupt Enable Register
    rw IE @ 0x2a: u16 = 0_0 {
        /// USCI B Interrupt Enable Register
        IE: 0..15 = struct IEField(u16);
    }
    /// USCI B Interrupt Flags Register
    rw IFG @ 0x2c: u16 = 0_0 {
        /// USCI B Interrupt Flags Register
        IFG: 0..15 = struct IFGField(u16);
    }
    /// USCI B Interrupt Enable Register
    rw IE_I2C @ 0x2a: u16 = 0_0 {
        /// I2C Receive Interrupt Enable 0
        RXIE0: 0 = struct RXIE0(bool);
        /// I2C Transmit Interrupt Enable 0
        TXIE0: 1 = struct TXIE0(bool);
        /// I2C START Condition interrupt enable
        STTIE: 2 = struct STTIE(bool);
        /// I2C STOP Condition interrupt enable
        STPIE: 3 = struct STPIE(bool);
        /// I2C Arbitration Lost interrupt enable
        ALIE: 4 = struct ALIE(bool);
        /// I2C NACK Condition interrupt enable
        NACKIE: 5 = struct NACKIE(bool);
        /// I2C Automatic stop assertion interrupt enable
        CNTIE: 6 = struct CNTIE(bool);
        /// I2C Clock Low Timeout interrupt enable
        CLTOIE: 7 = struct CLTOIE(bool);
        /// I2C Receive Interrupt Enable 1
        RXIE1: 8 = struct RXIE1(bool);
        /// I2C Transmit Interrupt Enable 1
        TXIE1: 9 = struct TXIE1(bool);
        /// I2C Receive Interrupt Enable 2
        RXIE2: 10 = struct RXIE2(bool);
        /// I2C Transmit Interrupt Enable 2
        TXIE2: 11 = struct TXIE2(bool);
        /// I2C Receive Interrupt Enable 3
        RXIE3: 12 = struct RXIE3(bool);
        /// I2C Transmit Interrupt Enable 3
        TXIE3: 13 = struct TXIE3(bool);
        /// I2C Bit 9 Position Interrupt Enable 3
        IT9IE: 14 = struct IT9IE(bool);
    }
    /// USCI B Interrupt Flags Register
    rw IFG_I2C @ 0x2c: u16 = 0_0 {
        /// I2C Receive Interrupt Flag 0
        RXIFG0: 0 = struct RXIFG0(bool);
        /// I2C Transmit Interrupt Flag 0
        TXIFG0: 1 = struct TXIFG0(bool);
        /// I2C START Condition interrupt Flag
        STTIFG: 2 = struct STTIFG(bool);
        /// I2C STOP Condition interrupt Flag
        STPIFG: 3 = struct STPIFG(bool);
        /// I2C Arbitration Lost interrupt Flag
        ALIFG: 4 = struct ALIFG(bool);
        /// I2C NACK Condition interrupt Flag
        NACKIFG: 5 = struct NACKIFG(bool);
        /// I2C Byte counter interrupt flag
        CNTIFG: 6 = struct CNTIFG(bool);
        /// I2C Clock low Timeout interrupt Flag
        CLTOIFG: 7 = struct CLTOIFG(bool);
        /// I2C Receive Interrupt Flag 1
        RXIFG1: 8 = struct RXIFG1(bool);
        /// I2C Transmit Interrupt Flag 1
        TXIFG1: 9 = struct TXIFG1(bool);
        /// I2C Receive Interrupt Flag 2
        RXIFG2: 10 = struct RXIFG2(bool);
        /// I2C Transmit Interrupt Flag 2
        TXIFG2: 11 = struct TXIFG2(bool);
        /// I2C Receive Interrupt Flag 3
        RXIFG3: 12 = struct RXIFG3(bool);
        /// I2C Transmit Interrupt Flag 3
        TXIFG3: 13 = struct TXIFG3(bool);
        /// I2C Bit 9 Possition Interrupt Flag 3
        IT9IFG: 14 = struct IT9IFG(bool);
    }
    /// USCI B Interrupt Vector Register
    rw IV @ 0x2e: u16 = 0_0 {
        /// USCI B Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
}
