//! USCI_B  I2C Mode

utils::periph! {
    /// USCI_B  I2C Mode
    USCI_B_I2C;
    /// USCI B Control Word Register 0
    rw UCBCTLW0 @ 0x00: u16 = 0_0 {
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
        /// Transmit ACK
        UCTXACK: 5 = struct UCTXACK(bool);
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
        /// Sync. Mode: USCI Mode 1
        UCMODE: 9..10 = enum UCMODE {
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
        UCMST: 11 = struct UCMST(bool);
        /// Multi-Master Environment
        UCMM: 13 = struct UCMM(bool);
        /// 10-bit Slave Address Mode
        UCSLA10: 14 = struct UCSLA10(bool);
        /// 10-bit Address Mode
        UCA10: 15 = struct UCA10(bool);
    }
    /// USCI B Control Register 0
    rw UCBCTL0 @ 0x01: u8 = 0_0 {
        /// USCI B Control Register 0
        UCBCTL0: 0..7 = struct UCBCTL0Field(u8);
    }
    /// USCI B Control Register 1
    rw UCBCTL1 @ 0x00: u8 = 0_0 {
        /// USCI B Control Register 1
        UCBCTL1: 0..7 = struct UCBCTL1Field(u8);
    }
    /// USCI B Control Word Register 1
    rw UCBCTLW1 @ 0x02: u16 = 0_0 {
        /// USCI Deglitch time Bit: 1
        UCGLIT: 0..1 = enum UCGLIT {
            /// USCI Deglitch time: 0
            UCGLIT_0 = 0b00,
            /// USCI Deglitch time: 1
            UCGLIT_1 = 0b01,
            /// USCI Deglitch time: 2
            UCGLIT_2 = 0b10,
            /// USCI Deglitch time: 3
            UCGLIT_3 = 0b11,
        }
        /// USCI Automatic Stop condition generation Bit: 1
        UCASTP: 2..3 = enum UCASTP {
            /// USCI Automatic Stop condition generation: 0
            UCASTP_0 = 0b00,
            /// USCI Automatic Stop condition generation: 1
            UCASTP_1 = 0b01,
            /// USCI Automatic Stop condition generation: 2
            UCASTP_2 = 0b10,
            /// USCI Automatic Stop condition generation: 3
            UCASTP_3 = 0b11,
        }
        /// USCI Software controlled ACK
        UCSWACK: 4 = struct UCSWACK(bool);
        /// USCI Acknowledge Stop last byte
        UCSTPNACK: 5 = struct UCSTPNACK(bool);
        /// USCI Clock low timeout Bit: 1
        UCCLTO: 6..7 = enum UCCLTO {
            /// USCI Clock low timeout: 0
            UCCLTO_0 = 0b00,
            /// USCI Clock low timeout: 1
            UCCLTO_1 = 0b01,
            /// USCI Clock low timeout: 2
            UCCLTO_2 = 0b10,
            /// USCI Clock low timeout: 3
            UCCLTO_3 = 0b11,
        }
        /// USCI Early UCTXIFG0
        UCETXINT: 8 = struct UCETXINT(bool);
    }
    /// USCI B Baud Word Rate 0
    rw UCBBRW @ 0x06: u16 = 0_0 {
        /// USCI B Baud Word Rate 0
        UCBBRW: 0..15 = struct UCBBRWField(u16);
    }
    /// USCI B Baud Rate 0
    rw UCBBR0 @ 0x06: u8 = 0_0 {
        /// USCI B Baud Rate 0
        UCBBR0: 0..7 = struct UCBBR0Field(u8);
    }
    /// USCI B Baud Rate 1
    rw UCBBR1 @ 0x07: u8 = 0_0 {
        /// USCI B Baud Rate 1
        UCBBR1: 0..7 = struct UCBBR1Field(u8);
    }
    /// USCI B Status Word Register
    rw UCBSTATW @ 0x08: u16 = 0_0 {
        /// USCI B Status Word Register
        UCBSTATW: 0..15 = struct UCBSTATWField(u16);
    }
    /// USCI B Status Register
    rw UCBSTATW__I2C @ 0x08: u16 = 0_0 {
        /// Bus Busy Flag
        UCBSTATW__I2C_UCBBUSY: 4 = struct UCBSTATW__I2C_UCBBUSY(bool);
        /// General Call address received Flag
        UCBSTATW__I2C_UCGC: 5 = struct UCBSTATW__I2C_UCGC(bool);
        /// SCL low
        UCBSTATW__I2C_UCSCLLOW: 6 = struct UCBSTATW__I2C_UCSCLLOW(bool);
        /// USCI Byte Counter Bit 0
        UCBSTATW__I2C_UCBCNT0: 8 = struct UCBSTATW__I2C_UCBCNT0(bool);
        /// USCI Byte Counter Bit 1
        UCBSTATW__I2C_UCBCNT1: 9 = struct UCBSTATW__I2C_UCBCNT1(bool);
        /// USCI Byte Counter Bit 2
        UCBSTATW__I2C_UCBCNT2: 10 = struct UCBSTATW__I2C_UCBCNT2(bool);
        /// USCI Byte Counter Bit 3
        UCBSTATW__I2C_UCBCNT3: 11 = struct UCBSTATW__I2C_UCBCNT3(bool);
        /// USCI Byte Counter Bit 4
        UCBSTATW__I2C_UCBCNT4: 12 = struct UCBSTATW__I2C_UCBCNT4(bool);
        /// USCI Byte Counter Bit 5
        UCBSTATW__I2C_UCBCNT5: 13 = struct UCBSTATW__I2C_UCBCNT5(bool);
        /// USCI Byte Counter Bit 6
        UCBSTATW__I2C_UCBCNT6: 14 = struct UCBSTATW__I2C_UCBCNT6(bool);
        /// USCI Byte Counter Bit 7
        UCBSTATW__I2C_UCBCNT7: 15 = struct UCBSTATW__I2C_UCBCNT7(bool);
    }
    /// USCI B Status Register
    rw UCBSTAT__I2C @ 0x08: u8 = 0_0 {
        /// Bus Busy Flag
        UCBSTAT__I2C_UCBBUSY: 4 = struct UCBSTAT__I2C_UCBBUSY(bool);
        /// General Call address received Flag
        UCBSTAT__I2C_UCGC: 5 = struct UCBSTAT__I2C_UCGC(bool);
        /// SCL low
        UCBSTAT__I2C_UCSCLLOW: 6 = struct UCBSTAT__I2C_UCSCLLOW(bool);
    }
    /// USCI B Byte Counter Register
    rw UCBBCNT__I2C @ 0x09: u8 = 0_0 {
        /// USCI Byte Counter Bit 0
        UCBBCNT__I2C_UCBCNT0: 0 = struct UCBBCNT__I2C_UCBCNT0(bool);
        /// USCI Byte Counter Bit 1
        UCBBCNT__I2C_UCBCNT1: 1 = struct UCBBCNT__I2C_UCBCNT1(bool);
        /// USCI Byte Counter Bit 2
        UCBBCNT__I2C_UCBCNT2: 2 = struct UCBBCNT__I2C_UCBCNT2(bool);
        /// USCI Byte Counter Bit 3
        UCBBCNT__I2C_UCBCNT3: 3 = struct UCBBCNT__I2C_UCBCNT3(bool);
        /// USCI Byte Counter Bit 4
        UCBBCNT__I2C_UCBCNT4: 4 = struct UCBBCNT__I2C_UCBCNT4(bool);
        /// USCI Byte Counter Bit 5
        UCBBCNT__I2C_UCBCNT5: 5 = struct UCBBCNT__I2C_UCBCNT5(bool);
        /// USCI Byte Counter Bit 6
        UCBBCNT__I2C_UCBCNT6: 6 = struct UCBBCNT__I2C_UCBCNT6(bool);
        /// USCI Byte Counter Bit 7
        UCBBCNT__I2C_UCBCNT7: 7 = struct UCBBCNT__I2C_UCBCNT7(bool);
    }
    /// USCI B Byte Counter Threshold Register
    rw UCBTBCNT @ 0x0a: u16 = 0_0 {
        /// USCI B Byte Counter Threshold Register
        UCBTBCNT: 0..15 = struct UCBTBCNTField(u16);
    }
    /// USCI B Receive Buffer
    rw UCBRXBUF @ 0x0c: u16 = 0_0 {
        /// USCI B Receive Buffer
        UCBRXBUF: 0..15 = struct UCBRXBUFField(u16);
    }
    /// USCI B Transmit Buffer
    rw UCBTXBUF @ 0x0e: u16 = 0_0 {
        /// USCI B Transmit Buffer
        UCBTXBUF: 0..15 = struct UCBTXBUFField(u16);
    }
    /// USCI B I2C Own Address 0
    rw UCBI2COA0 @ 0x14: u16 = 0_0 {
        /// I2C Own Address Bit 0
        UCBI2COA0_UCOA0: 0 = struct UCBI2COA0_UCOA0(bool);
        /// I2C Own Address Bit 1
        UCBI2COA0_UCOA1: 1 = struct UCBI2COA0_UCOA1(bool);
        /// I2C Own Address Bit 2
        UCBI2COA0_UCOA2: 2 = struct UCBI2COA0_UCOA2(bool);
        /// I2C Own Address Bit 3
        UCBI2COA0_UCOA3: 3 = struct UCBI2COA0_UCOA3(bool);
        /// I2C Own Address Bit 4
        UCBI2COA0_UCOA4: 4 = struct UCBI2COA0_UCOA4(bool);
        /// I2C Own Address Bit 5
        UCBI2COA0_UCOA5: 5 = struct UCBI2COA0_UCOA5(bool);
        /// I2C Own Address Bit 6
        UCBI2COA0_UCOA6: 6 = struct UCBI2COA0_UCOA6(bool);
        /// I2C Own Address Bit 7
        UCBI2COA0_UCOA7: 7 = struct UCBI2COA0_UCOA7(bool);
        /// I2C Own Address Bit 8
        UCBI2COA0_UCOA8: 8 = struct UCBI2COA0_UCOA8(bool);
        /// I2C Own Address Bit 9
        UCBI2COA0_UCOA9: 9 = struct UCBI2COA0_UCOA9(bool);
        /// I2C Own Address enable
        UCBI2COA0_UCOAEN: 10 = struct UCBI2COA0_UCOAEN(bool);
        /// I2C General Call enable
        UCGCEN: 15 = struct UCGCEN(bool);
    }
    /// USCI B I2C Own Address 1
    rw UCBI2COA1 @ 0x16: u16 = 0_0 {
        /// I2C Own Address Bit 0
        UCBI2COA1_UCOA0: 0 = struct UCBI2COA1_UCOA0(bool);
        /// I2C Own Address Bit 1
        UCBI2COA1_UCOA1: 1 = struct UCBI2COA1_UCOA1(bool);
        /// I2C Own Address Bit 2
        UCBI2COA1_UCOA2: 2 = struct UCBI2COA1_UCOA2(bool);
        /// I2C Own Address Bit 3
        UCBI2COA1_UCOA3: 3 = struct UCBI2COA1_UCOA3(bool);
        /// I2C Own Address Bit 4
        UCBI2COA1_UCOA4: 4 = struct UCBI2COA1_UCOA4(bool);
        /// I2C Own Address Bit 5
        UCBI2COA1_UCOA5: 5 = struct UCBI2COA1_UCOA5(bool);
        /// I2C Own Address Bit 6
        UCBI2COA1_UCOA6: 6 = struct UCBI2COA1_UCOA6(bool);
        /// I2C Own Address Bit 7
        UCBI2COA1_UCOA7: 7 = struct UCBI2COA1_UCOA7(bool);
        /// I2C Own Address Bit 8
        UCBI2COA1_UCOA8: 8 = struct UCBI2COA1_UCOA8(bool);
        /// I2C Own Address Bit 9
        UCBI2COA1_UCOA9: 9 = struct UCBI2COA1_UCOA9(bool);
        /// I2C Own Address enable
        UCBI2COA1_UCOAEN: 10 = struct UCBI2COA1_UCOAEN(bool);
    }
    /// USCI B I2C Own Address 2
    rw UCBI2COA2 @ 0x18: u16 = 0_0 {
        /// I2C Own Address Bit 0
        UCBI2COA2_UCOA0: 0 = struct UCBI2COA2_UCOA0(bool);
        /// I2C Own Address Bit 1
        UCBI2COA2_UCOA1: 1 = struct UCBI2COA2_UCOA1(bool);
        /// I2C Own Address Bit 2
        UCBI2COA2_UCOA2: 2 = struct UCBI2COA2_UCOA2(bool);
        /// I2C Own Address Bit 3
        UCBI2COA2_UCOA3: 3 = struct UCBI2COA2_UCOA3(bool);
        /// I2C Own Address Bit 4
        UCBI2COA2_UCOA4: 4 = struct UCBI2COA2_UCOA4(bool);
        /// I2C Own Address Bit 5
        UCBI2COA2_UCOA5: 5 = struct UCBI2COA2_UCOA5(bool);
        /// I2C Own Address Bit 6
        UCBI2COA2_UCOA6: 6 = struct UCBI2COA2_UCOA6(bool);
        /// I2C Own Address Bit 7
        UCBI2COA2_UCOA7: 7 = struct UCBI2COA2_UCOA7(bool);
        /// I2C Own Address Bit 8
        UCBI2COA2_UCOA8: 8 = struct UCBI2COA2_UCOA8(bool);
        /// I2C Own Address Bit 9
        UCBI2COA2_UCOA9: 9 = struct UCBI2COA2_UCOA9(bool);
        /// I2C Own Address enable
        UCBI2COA2_UCOAEN: 10 = struct UCBI2COA2_UCOAEN(bool);
    }
    /// USCI B I2C Own Address 3
    rw UCBI2COA3 @ 0x1a: u16 = 0_0 {
        /// I2C Own Address Bit 0
        UCBI2COA3_UCOA0: 0 = struct UCBI2COA3_UCOA0(bool);
        /// I2C Own Address Bit 1
        UCBI2COA3_UCOA1: 1 = struct UCBI2COA3_UCOA1(bool);
        /// I2C Own Address Bit 2
        UCBI2COA3_UCOA2: 2 = struct UCBI2COA3_UCOA2(bool);
        /// I2C Own Address Bit 3
        UCBI2COA3_UCOA3: 3 = struct UCBI2COA3_UCOA3(bool);
        /// I2C Own Address Bit 4
        UCBI2COA3_UCOA4: 4 = struct UCBI2COA3_UCOA4(bool);
        /// I2C Own Address Bit 5
        UCBI2COA3_UCOA5: 5 = struct UCBI2COA3_UCOA5(bool);
        /// I2C Own Address Bit 6
        UCBI2COA3_UCOA6: 6 = struct UCBI2COA3_UCOA6(bool);
        /// I2C Own Address Bit 7
        UCBI2COA3_UCOA7: 7 = struct UCBI2COA3_UCOA7(bool);
        /// I2C Own Address Bit 8
        UCBI2COA3_UCOA8: 8 = struct UCBI2COA3_UCOA8(bool);
        /// I2C Own Address Bit 9
        UCBI2COA3_UCOA9: 9 = struct UCBI2COA3_UCOA9(bool);
        /// I2C Own Address enable
        UCBI2COA3_UCOAEN: 10 = struct UCBI2COA3_UCOAEN(bool);
    }
    /// USCI B Received Address Register
    rw UCBADDRX @ 0x1c: u16 = 0_0 {
        /// I2C Receive Address Bit 0
        UCADDRX0: 0 = struct UCADDRX0(bool);
        /// I2C Receive Address Bit 1
        UCADDRX1: 1 = struct UCADDRX1(bool);
        /// I2C Receive Address Bit 2
        UCADDRX2: 2 = struct UCADDRX2(bool);
        /// I2C Receive Address Bit 3
        UCADDRX3: 3 = struct UCADDRX3(bool);
        /// I2C Receive Address Bit 4
        UCADDRX4: 4 = struct UCADDRX4(bool);
        /// I2C Receive Address Bit 5
        UCADDRX5: 5 = struct UCADDRX5(bool);
        /// I2C Receive Address Bit 6
        UCADDRX6: 6 = struct UCADDRX6(bool);
        /// I2C Receive Address Bit 7
        UCADDRX7: 7 = struct UCADDRX7(bool);
        /// I2C Receive Address Bit 8
        UCADDRX8: 8 = struct UCADDRX8(bool);
        /// I2C Receive Address Bit 9
        UCADDRX9: 9 = struct UCADDRX9(bool);
    }
    /// USCI B Address Mask Register
    rw UCBADDMASK @ 0x1e: u16 = 0_0 {
        /// I2C Address Mask Bit 0
        UCADDMASK0: 0 = struct UCADDMASK0(bool);
        /// I2C Address Mask Bit 1
        UCADDMASK1: 1 = struct UCADDMASK1(bool);
        /// I2C Address Mask Bit 2
        UCADDMASK2: 2 = struct UCADDMASK2(bool);
        /// I2C Address Mask Bit 3
        UCADDMASK3: 3 = struct UCADDMASK3(bool);
        /// I2C Address Mask Bit 4
        UCADDMASK4: 4 = struct UCADDMASK4(bool);
        /// I2C Address Mask Bit 5
        UCADDMASK5: 5 = struct UCADDMASK5(bool);
        /// I2C Address Mask Bit 6
        UCADDMASK6: 6 = struct UCADDMASK6(bool);
        /// I2C Address Mask Bit 7
        UCADDMASK7: 7 = struct UCADDMASK7(bool);
        /// I2C Address Mask Bit 8
        UCADDMASK8: 8 = struct UCADDMASK8(bool);
        /// I2C Address Mask Bit 9
        UCADDMASK9: 9 = struct UCADDMASK9(bool);
    }
    /// USCI B I2C Slave Address
    rw UCBI2CSA @ 0x20: u16 = 0_0 {
        /// I2C Slave Address Bit 0
        UCSA0: 0 = struct UCSA0(bool);
        /// I2C Slave Address Bit 1
        UCSA1: 1 = struct UCSA1(bool);
        /// I2C Slave Address Bit 2
        UCSA2: 2 = struct UCSA2(bool);
        /// I2C Slave Address Bit 3
        UCSA3: 3 = struct UCSA3(bool);
        /// I2C Slave Address Bit 4
        UCSA4: 4 = struct UCSA4(bool);
        /// I2C Slave Address Bit 5
        UCSA5: 5 = struct UCSA5(bool);
        /// I2C Slave Address Bit 6
        UCSA6: 6 = struct UCSA6(bool);
        /// I2C Slave Address Bit 7
        UCSA7: 7 = struct UCSA7(bool);
        /// I2C Slave Address Bit 8
        UCSA8: 8 = struct UCSA8(bool);
        /// I2C Slave Address Bit 9
        UCSA9: 9 = struct UCSA9(bool);
    }
    /// USCI B Interrupt Enable Register
    rw UCBIE @ 0x2a: u16 = 0_0 {
        /// USCI B Interrupt Enable Register
        UCBIE: 0..15 = struct UCBIEField(u16);
    }
    /// USCI B Interrupt Flags Register
    rw UCBIFG @ 0x2c: u16 = 0_0 {
        /// USCI B Interrupt Flags Register
        UCBIFG: 0..15 = struct UCBIFGField(u16);
    }
    /// USCI B Interrupt Enable Register
    rw UCBIE__I2C @ 0x2a: u16 = 0_0 {
        /// I2C Receive Interrupt Enable 0
        UCRXIE0: 0 = struct UCRXIE0(bool);
        /// I2C Transmit Interrupt Enable 0
        UCTXIE0: 1 = struct UCTXIE0(bool);
        /// I2C START Condition interrupt enable
        UCSTTIE: 2 = struct UCSTTIE(bool);
        /// I2C STOP Condition interrupt enable
        UCSTPIE: 3 = struct UCSTPIE(bool);
        /// I2C Arbitration Lost interrupt enable
        UCALIE: 4 = struct UCALIE(bool);
        /// I2C NACK Condition interrupt enable
        UCNACKIE: 5 = struct UCNACKIE(bool);
        /// I2C Automatic stop assertion interrupt enable
        UCBCNTIE: 6 = struct UCBCNTIE(bool);
        /// I2C Clock Low Timeout interrupt enable
        UCCLTOIE: 7 = struct UCCLTOIE(bool);
        /// I2C Receive Interrupt Enable 1
        UCRXIE1: 8 = struct UCRXIE1(bool);
        /// I2C Transmit Interrupt Enable 1
        UCTXIE1: 9 = struct UCTXIE1(bool);
        /// I2C Receive Interrupt Enable 2
        UCRXIE2: 10 = struct UCRXIE2(bool);
        /// I2C Transmit Interrupt Enable 2
        UCTXIE2: 11 = struct UCTXIE2(bool);
        /// I2C Receive Interrupt Enable 3
        UCRXIE3: 12 = struct UCRXIE3(bool);
        /// I2C Transmit Interrupt Enable 3
        UCTXIE3: 13 = struct UCTXIE3(bool);
        /// I2C Bit 9 Position Interrupt Enable 3
        UCBIT9IE: 14 = struct UCBIT9IE(bool);
    }
    /// USCI B Interrupt Flags Register
    rw UCBIFG__I2C @ 0x2c: u16 = 0_0 {
        /// I2C Receive Interrupt Flag 0
        UCRXIFG0: 0 = struct UCRXIFG0(bool);
        /// I2C Transmit Interrupt Flag 0
        UCTXIFG0: 1 = struct UCTXIFG0(bool);
        /// I2C START Condition interrupt Flag
        UCSTTIFG: 2 = struct UCSTTIFG(bool);
        /// I2C STOP Condition interrupt Flag
        UCSTPIFG: 3 = struct UCSTPIFG(bool);
        /// I2C Arbitration Lost interrupt Flag
        UCALIFG: 4 = struct UCALIFG(bool);
        /// I2C NACK Condition interrupt Flag
        UCNACKIFG: 5 = struct UCNACKIFG(bool);
        /// I2C Byte counter interrupt flag
        UCBCNTIFG: 6 = struct UCBCNTIFG(bool);
        /// I2C Clock low Timeout interrupt Flag
        UCCLTOIFG: 7 = struct UCCLTOIFG(bool);
        /// I2C Receive Interrupt Flag 1
        UCRXIFG1: 8 = struct UCRXIFG1(bool);
        /// I2C Transmit Interrupt Flag 1
        UCTXIFG1: 9 = struct UCTXIFG1(bool);
        /// I2C Receive Interrupt Flag 2
        UCRXIFG2: 10 = struct UCRXIFG2(bool);
        /// I2C Transmit Interrupt Flag 2
        UCTXIFG2: 11 = struct UCTXIFG2(bool);
        /// I2C Receive Interrupt Flag 3
        UCRXIFG3: 12 = struct UCRXIFG3(bool);
        /// I2C Transmit Interrupt Flag 3
        UCTXIFG3: 13 = struct UCTXIFG3(bool);
        /// I2C Bit 9 Possition Interrupt Flag 3
        UCBIT9IFG: 14 = struct UCBIT9IFG(bool);
    }
    /// USCI B Interrupt Vector Register
    rw UCBIV @ 0x2e: u16 = 0_0 {
        /// USCI B Interrupt Vector Register
        UCBIV: 0..15 = struct UCBIVField(u16);
    }
}
