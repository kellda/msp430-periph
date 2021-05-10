//! eUSCI_B2

utils::periph! {
    /// eUSCI_B2
    eUSCI_B2;
    /// eUSCI_Bx Control Word Register 0
    rw UCB2CTLW0 @ 0x00: u16 = 0_0 {
        /// Software reset enable
        UCB2CTLW0_UCSWRST: 0 = enum UCB2CTLW0_UCSWRST {
            /// Disabled. eUSCI_B reset released for operation
            DISABLE = 0b0,
            /// Enabled. eUSCI_B logic held in reset state
            ENABLE = 0b1,
        }
        /// Transmit START condition in master mode
        UCTXSTT: 1 = enum UCTXSTT {
            /// Do not generate START condition
            UCTXSTT_0 = 0b0,
            /// Generate START condition
            UCTXSTT_1 = 0b1,
        }
        /// Transmit STOP condition in master mode
        UCTXSTP: 2 = enum UCTXSTP {
            /// No STOP generated
            UCTXSTP_0 = 0b0,
            /// Generate STOP
            UCTXSTP_1 = 0b1,
        }
        /// Transmit a NACK
        UCTXNACK: 3 = enum UCTXNACK {
            /// Acknowledge normally
            UCTXNACK_0 = 0b0,
            /// Generate NACK
            UCTXNACK_1 = 0b1,
        }
        /// Transmitter/receiver
        UCTR: 4 = enum UCTR {
            /// Receiver
            RX = 0b0,
            /// Transmitter
            TX = 0b1,
        }
        /// Transmit ACK condition in slave mode
        UCTXACK: 5 = enum UCTXACK {
            /// Do not acknowledge the slave address
            UCTXACK_0 = 0b0,
            /// Acknowledge the slave address
            UCTXACK_1 = 0b1,
        }
        /// eUSCI_B clock source select
        UCB2CTLW0_UCSSEL: 6..7 = enum UCB2CTLW0_UCSSEL {
            /// UCLKI
            UCLKI = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// SMCLK
            SMCLK = 0b10,
            /// SMCLK
            UCSSEL_3 = 0b11,
        }
        /// Synchronous mode enable
        UCB2CTLW0_UCSYNC: 8 = enum UCB2CTLW0_UCSYNC {
            /// Asynchronous mode
            ASYNC = 0b0,
            /// Synchronous mode
            SYNC = 0b1,
        }
        /// eUSCI_B mode
        UCB2CTLW0_UCMODE: 9..10 = enum UCB2CTLW0_UCMODE {
            /// 3-pin SPI
            UCMODE_0 = 0b00,
            /// 4-pin SPI (master or slave enabled if STE = 1)
            UCMODE_1 = 0b01,
            /// 4-pin SPI (master or slave enabled if STE = 0)
            UCMODE_2 = 0b10,
            /// I2C mode
            UCMODE_3 = 0b11,
        }
        /// Master mode select
        UCB2CTLW0_UCMST: 11 = enum UCB2CTLW0_UCMST {
            /// Slave mode
            SLAVE = 0b0,
            /// Master mode
            MASTER = 0b1,
        }
        /// Multi-master environment select
        UCMM: 13 = enum UCMM {
            /// Single master environment. There is no other master in the system. The address compare unit is disabled.
            SINGLE = 0b0,
            /// Multi-master environment
            MULTI = 0b1,
        }
        /// Slave addressing mode select
        UCSLA10: 14 = enum UCSLA10 {
            /// Address slave with 7-bit address
            _7BIT = 0b0,
            /// Address slave with 10-bit address
            _10BIT = 0b1,
        }
        /// Own addressing mode select
        UCA10: 15 = enum UCA10 {
            /// Own address is a 7-bit address
            UCA10_0 = 0b0,
            /// Own address is a 10-bit address
            UCA10_1 = 0b1,
        }
    }
    /// eUSCI_Bx Control Word Register 0
    rw UCB2CTLW0_SPI @ 0x00: u16 = 0_0 {
        /// Software reset enable
        UCB2CTLW0_SPI_UCSWRST: 0 = enum UCB2CTLW0_SPI_UCSWRST {
            /// Disabled. eUSCI_B reset released for operation
            DISABLE = 0b0,
            /// Enabled. eUSCI_B logic held in reset state
            ENABLE = 0b1,
        }
        /// STE mode select in master mode.
        UCSTEM: 1 = enum UCSTEM {
            /// STE pin is used to prevent conflicts with other masters
            UCSTEM_0 = 0b0,
            /// STE pin is used to generate the enable signal for a 4-wire slave
            UCSTEM_1 = 0b1,
        }
        /// eUSCI_B clock source select
        UCB2CTLW0_SPI_UCSSEL: 6..7 = enum UCB2CTLW0_SPI_UCSSEL {
            /// Reserved
            UCSSEL_0 = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// SMCLK
            SMCLK = 0b10,
            /// SMCLK
            UCSSEL_3 = 0b11,
        }
        /// Synchronous mode enable
        UCB2CTLW0_SPI_UCSYNC: 8 = enum UCB2CTLW0_SPI_UCSYNC {
            /// Asynchronous mode
            ASYNC = 0b0,
            /// Synchronous mode
            SYNC = 0b1,
        }
        /// eUSCI mode
        UCB2CTLW0_SPI_UCMODE: 9..10 = enum UCB2CTLW0_SPI_UCMODE {
            /// 3-pin SPI
            UCMODE_0 = 0b00,
            /// 4-pin SPI with UCxSTE active high: Slave enabled when UCxSTE = 1
            UCMODE_1 = 0b01,
            /// 4-pin SPI with UCxSTE active low: Slave enabled when UCxSTE = 0
            UCMODE_2 = 0b10,
            /// I2C mode
            UCMODE_3 = 0b11,
        }
        /// Master mode select
        UCB2CTLW0_SPI_UCMST: 11 = enum UCB2CTLW0_SPI_UCMST {
            /// Slave mode
            SLAVE = 0b0,
            /// Master mode
            MASTER = 0b1,
        }
        /// Character length
        UC7BIT: 12 = enum UC7BIT {
            /// 8-bit data
            _8BIT = 0b0,
            /// 7-bit data
            _7BIT = 0b1,
        }
        /// MSB first select
        UCMSB: 13 = enum UCMSB {
            /// LSB first
            UCMSB_0 = 0b0,
            /// MSB first
            UCMSB_1 = 0b1,
        }
        /// Clock polarity select
        UCCKPL: 14 = enum UCCKPL {
            /// The inactive state is low
            LOW = 0b0,
            /// The inactive state is high
            HIGH = 0b1,
        }
        /// Clock phase select
        UCCKPH: 15 = enum UCCKPH {
            /// Data is changed on the first UCLK edge and captured on the following edge.
            UCCKPH_0 = 0b0,
            /// Data is captured on the first UCLK edge and changed on the following edge.
            UCCKPH_1 = 0b1,
        }
    }
    /// eUSCI_Bx Control Word Register 1
    rw UCB2CTLW1 @ 0x02: u16 = 0_0 {
        /// Deglitch time
        UCGLIT: 0..1 = enum UCGLIT {
            /// 50 ns
            UCGLIT_0 = 0b00,
            /// 25 ns
            UCGLIT_1 = 0b01,
            /// 12.5 ns
            UCGLIT_2 = 0b10,
            /// 6.25 ns
            UCGLIT_3 = 0b11,
        }
        /// Automatic STOP condition generation
        UCASTP: 2..3 = enum UCASTP {
            /// No automatic STOP generation. The STOP condition is generated after the user sets the UCTXSTP bit. The value in UCBxTBCNT is a don't care.
            UCASTP_0 = 0b00,
            /// UCBCNTIFG is set with the byte counter reaches the threshold defined in UCBxTBCNT
            UCASTP_1 = 0b01,
            /// A STOP condition is generated automatically after the byte counter value reached UCBxTBCNT. UCBCNTIFG is set with the byte counter reaching the threshold
            UCASTP_2 = 0b10,
        }
        /// SW or HW ACK control
        UCSWACK: 4 = enum UCSWACK {
            /// The address acknowledge of the slave is controlled by the eUSCI_B module
            UCSWACK_0 = 0b0,
            /// The user needs to trigger the sending of the address ACK by issuing UCTXACK
            UCSWACK_1 = 0b1,
        }
        /// ACK all master bytes
        UCSTPNACK: 5 = enum UCSTPNACK {
            /// Send a non-acknowledge before the STOP condition as a master receiver (conform to I2C standard)
            UCSTPNACK_0 = 0b0,
            /// All bytes are acknowledged by the eUSCI_B when configured as master receiver
            UCSTPNACK_1 = 0b1,
        }
        /// Clock low timeout select
        UCCLTO: 6..7 = enum UCCLTO {
            /// Disable clock low timeout counter
            UCCLTO_0 = 0b00,
            /// 135 000 SYSCLK cycles (approximately 28 ms)
            UCCLTO_1 = 0b01,
            /// 150 000 SYSCLK cycles (approximately 31 ms)
            UCCLTO_2 = 0b10,
            /// 165 000 SYSCLK cycles (approximately 34 ms)
            UCCLTO_3 = 0b11,
        }
        /// Early UCTXIFG0
        UCETXINT: 8 = enum UCETXINT {
            /// UCTXIFGx is set after an address match with UCxI2COAx and the direction bit indicating slave transmit
            UCETXINT_0 = 0b0,
            /// UCTXIFG0 is set for each START condition
            UCETXINT_1 = 0b1,
        }
    }
    /// eUSCI_Bx Baud Rate Control Word Register
    rw UCB2BRW @ 0x06: u16 = 0_0 {
        /// eUSCI_Bx Baud Rate Control Word Register
        UCB2BRW: 0..15 = struct UCB2BRWField(u16);
    }
    /// eUSCI_Bx Bit Rate Control Register 1
    rw UCB2BRW_SPI @ 0x06: u16 = 0_0 {
        /// eUSCI_Bx Bit Rate Control Register 1
        UCB2BRW_SPI: 0..15 = struct UCB2BRW_SPIField(u16);
    }
    /// eUSCI_Bx Status Register
    r UCB2STATW @ 0x08: u16 = 0_0 {
        /// Bus busy
        UCBBUSY: 4 = enum UCBBUSY {
            /// Bus inactive
            IDLE = 0b0,
            /// Bus busy
            BUSY = 0b1,
        }
        /// General call address received
        UCGC: 5 = enum UCGC {
            /// No general call address received
            UCGC_0 = 0b0,
            /// General call address received
            UCGC_1 = 0b1,
        }
        /// SCL low
        UCSCLLOW: 6 = enum UCSCLLOW {
            /// SCL is not held low
            UCSCLLOW_0 = 0b0,
            /// SCL is held low
            UCSCLLOW_1 = 0b1,
        }
        /// Hardware byte counter value
        UCBCNT: 8..15 = struct UCBCNT(u16);
    }
    /// UCB2STATW_SPI
    rw UCB2STATW_SPI @ 0x08: u16 = 0_0 {
        /// Overrun error flag
        UCOE: 5 = enum UCOE {
            /// No error
            UCOE_0 = 0b0,
            /// Overrun error occurred
            UCOE_1 = 0b1,
        }
        /// Framing error flag
        UCFE: 6 = enum UCFE {
            /// No error
            UCFE_0 = 0b0,
            /// Bus conflict occurred
            UCFE_1 = 0b1,
        }
        /// Listen enable
        UCLISTEN: 7 = enum UCLISTEN {
            /// Disabled
            UCLISTEN_0 = 0b0,
            /// Enabled. UCBxTXD is internally fed back to the receiver
            UCLISTEN_1 = 0b1,
        }
    }
    /// eUSCI_Bx Byte Counter Threshold Register
    rw UCB2TBCNT @ 0x0a: u16 = 0_0 {
        /// Byte counter threshold value
        UCTBCNT: 0..7 = struct UCTBCNT(u16);
    }
    /// eUSCI_Bx Receive Buffer Register
    r UCB2RXBUF @ 0x0c: u16 = 0_0 {
        /// Receive data buffer
        UCB2RXBUF_UCRXBUF: 0..7 = struct UCB2RXBUF_UCRXBUF(u16);
    }
    /// eUSCI_Bx Receive Buffer Register
    r UCB2RXBUF_SPI @ 0x0c: u16 = 0_0 {
        /// Receive data buffer
        UCB2RXBUF_SPI_UCRXBUF: 0..7 = struct UCB2RXBUF_SPI_UCRXBUF(u16);
    }
    /// eUSCI_Bx Transmit Buffer Register
    rw UCB2TXBUF @ 0x0e: u16 = 0_0 {
        /// Transmit data buffer
        UCB2TXBUF_UCTXBUF: 0..7 = struct UCB2TXBUF_UCTXBUF(u16);
    }
    /// eUSCI_Bx Transmit Buffer Register
    rw UCB2TXBUF_SPI @ 0x0e: u16 = 0_0 {
        /// Transmit data buffer
        UCB2TXBUF_SPI_UCTXBUF: 0..7 = struct UCB2TXBUF_SPI_UCTXBUF(u16);
    }
    /// eUSCI_Bx I2C Own Address 0 Register
    rw UCB2I2COA0 @ 0x14: u16 = 0_0 {
        /// I2C own address
        I2COA0: 0..9 = struct I2COA0(u16);
        /// Own Address enable register
        UCB2I2COA0_UCOAEN: 10 = enum UCB2I2COA0_UCOAEN {
            /// The slave address defined in I2COA0 is disabled
            DISABLE = 0b0,
            /// The slave address defined in I2COA0 is enabled
            ENABLE = 0b1,
        }
        /// General call response enable
        UCGCEN: 15 = enum UCGCEN {
            /// Do not respond to a general call
            UCGCEN_0 = 0b0,
            /// Respond to a general call
            UCGCEN_1 = 0b1,
        }
    }
    /// eUSCI_Bx I2C Own Address 1 Register
    rw UCB2I2COA1 @ 0x16: u16 = 0_0 {
        /// I2C own address
        I2COA1: 0..9 = struct I2COA1(u16);
        /// Own Address enable register
        UCB2I2COA1_UCOAEN: 10 = enum UCB2I2COA1_UCOAEN {
            /// The slave address defined in I2COA1 is disabled
            DISABLE = 0b0,
            /// The slave address defined in I2COA1 is enabled
            ENABLE = 0b1,
        }
    }
    /// eUSCI_Bx I2C Own Address 2 Register
    rw UCB2I2COA2 @ 0x18: u16 = 0_0 {
        /// I2C own address
        I2COA2: 0..9 = struct I2COA2(u16);
        /// Own Address enable register
        UCB2I2COA2_UCOAEN: 10 = enum UCB2I2COA2_UCOAEN {
            /// The slave address defined in I2COA2 is disabled
            DISABLE = 0b0,
            /// The slave address defined in I2COA2 is enabled
            ENABLE = 0b1,
        }
    }
    /// eUSCI_Bx I2C Own Address 3 Register
    rw UCB2I2COA3 @ 0x1a: u16 = 0_0 {
        /// I2C own address
        I2COA3: 0..9 = struct I2COA3(u16);
        /// Own Address enable register
        UCB2I2COA3_UCOAEN: 10 = enum UCB2I2COA3_UCOAEN {
            /// The slave address defined in I2COA3 is disabled
            DISABLE = 0b0,
            /// The slave address defined in I2COA3 is enabled
            ENABLE = 0b1,
        }
    }
    /// eUSCI_Bx I2C Received Address Register
    r UCB2ADDRX @ 0x1c: u16 = 0_0 {
        /// Received Address Register
        ADDRX: 0..9 = struct ADDRX(u16);
    }
    /// eUSCI_Bx I2C Address Mask Register
    rw UCB2ADDMASK @ 0x1e: u16 = 0_0 {
        /// Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1.
        ADDMASK: 0..9 = struct ADDMASK(u16);
    }
    /// eUSCI_Bx I2C Slave Address Register
    rw UCB2I2CSA @ 0x20: u16 = 0_0 {
        /// I2C slave address
        I2CSA: 0..9 = struct I2CSA(u16);
    }
    /// eUSCI_Bx Interrupt Enable Register
    rw UCB2IE @ 0x2a: u16 = 0_0 {
        /// Receive interrupt enable 0
        UCRXIE0: 0 = enum UCRXIE0 {
            /// Interrupt disabled
            UCRXIE0_0 = 0b0,
            /// Interrupt enabled
            UCRXIE0_1 = 0b1,
        }
        /// Transmit interrupt enable 0
        UCTXIE0: 1 = enum UCTXIE0 {
            /// Interrupt disabled
            UCTXIE0_0 = 0b0,
            /// Interrupt enabled
            UCTXIE0_1 = 0b1,
        }
        /// START condition interrupt enable
        UCSTTIE: 2 = enum UCSTTIE {
            /// Interrupt disabled
            UCSTTIE_0 = 0b0,
            /// Interrupt enabled
            UCSTTIE_1 = 0b1,
        }
        /// STOP condition interrupt enable
        UCSTPIE: 3 = enum UCSTPIE {
            /// Interrupt disabled
            UCSTPIE_0 = 0b0,
            /// Interrupt enabled
            UCSTPIE_1 = 0b1,
        }
        /// Arbitration lost interrupt enable
        UCALIE: 4 = enum UCALIE {
            /// Interrupt disabled
            UCALIE_0 = 0b0,
            /// Interrupt enabled
            UCALIE_1 = 0b1,
        }
        /// Not-acknowledge interrupt enable
        UCNACKIE: 5 = enum UCNACKIE {
            /// Interrupt disabled
            UCNACKIE_0 = 0b0,
            /// Interrupt enabled
            UCNACKIE_1 = 0b1,
        }
        /// Byte counter interrupt enable
        UCBCNTIE: 6 = enum UCBCNTIE {
            /// Interrupt disabled
            UCBCNTIE_0 = 0b0,
            /// Interrupt enabled
            UCBCNTIE_1 = 0b1,
        }
        /// Clock low timeout interrupt enable
        UCCLTOIE: 7 = enum UCCLTOIE {
            /// Interrupt disabled
            UCCLTOIE_0 = 0b0,
            /// Interrupt enabled
            UCCLTOIE_1 = 0b1,
        }
        /// Receive interrupt enable 1
        UCRXIE1: 8 = enum UCRXIE1 {
            /// Interrupt disabled
            UCRXIE1_0 = 0b0,
            /// Interrupt enabled
            UCRXIE1_1 = 0b1,
        }
        /// Transmit interrupt enable 1
        UCTXIE1: 9 = enum UCTXIE1 {
            /// Interrupt disabled
            UCTXIE1_0 = 0b0,
            /// Interrupt enabled
            UCTXIE1_1 = 0b1,
        }
        /// Receive interrupt enable 2
        UCRXIE2: 10 = enum UCRXIE2 {
            /// Interrupt disabled
            UCRXIE2_0 = 0b0,
            /// Interrupt enabled
            UCRXIE2_1 = 0b1,
        }
        /// Transmit interrupt enable 2
        UCTXIE2: 11 = enum UCTXIE2 {
            /// Interrupt disabled
            UCTXIE2_0 = 0b0,
            /// Interrupt enabled
            UCTXIE2_1 = 0b1,
        }
        /// Receive interrupt enable 3
        UCRXIE3: 12 = enum UCRXIE3 {
            /// Interrupt disabled
            UCRXIE3_0 = 0b0,
            /// Interrupt enabled
            UCRXIE3_1 = 0b1,
        }
        /// Transmit interrupt enable 3
        UCTXIE3: 13 = enum UCTXIE3 {
            /// Interrupt disabled
            UCTXIE3_0 = 0b0,
            /// Interrupt enabled
            UCTXIE3_1 = 0b1,
        }
        /// Bit position 9 interrupt enable
        UCBIT9IE: 14 = enum UCBIT9IE {
            /// Interrupt disabled
            UCBIT9IE_0 = 0b0,
            /// Interrupt enabled
            UCBIT9IE_1 = 0b1,
        }
    }
    /// eUSCI_Bx Interrupt Enable Register
    rw UCB2IE_SPI @ 0x2a: u16 = 0_0 {
        /// Receive interrupt enable
        UCRXIE: 0 = enum UCRXIE {
            /// Interrupt disabled
            UCRXIE_0 = 0b0,
            /// Interrupt enabled
            UCRXIE_1 = 0b1,
        }
        /// Transmit interrupt enable
        UCTXIE: 1 = enum UCTXIE {
            /// Interrupt disabled
            UCTXIE_0 = 0b0,
            /// Interrupt enabled
            UCTXIE_1 = 0b1,
        }
    }
    /// eUSCI_Bx Interrupt Flag Register
    rw UCB2IFG @ 0x2c: u16 = 0_0 {
        /// eUSCI_B receive interrupt flag 0
        UCRXIFG0: 0 = enum UCRXIFG0 {
            /// No interrupt pending
            UCRXIFG0_0 = 0b0,
            /// Interrupt pending
            UCRXIFG0_1 = 0b1,
        }
        /// eUSCI_B transmit interrupt flag 0
        UCTXIFG0: 1 = enum UCTXIFG0 {
            /// No interrupt pending
            UCTXIFG0_0 = 0b0,
            /// Interrupt pending
            UCTXIFG0_1 = 0b1,
        }
        /// START condition interrupt flag
        UCSTTIFG: 2 = enum UCSTTIFG {
            /// No interrupt pending
            UCSTTIFG_0 = 0b0,
            /// Interrupt pending
            UCSTTIFG_1 = 0b1,
        }
        /// STOP condition interrupt flag
        UCSTPIFG: 3 = enum UCSTPIFG {
            /// No interrupt pending
            UCSTPIFG_0 = 0b0,
            /// Interrupt pending
            UCSTPIFG_1 = 0b1,
        }
        /// Arbitration lost interrupt flag
        UCALIFG: 4 = enum UCALIFG {
            /// No interrupt pending
            UCALIFG_0 = 0b0,
            /// Interrupt pending
            UCALIFG_1 = 0b1,
        }
        /// Not-acknowledge received interrupt flag
        UCNACKIFG: 5 = enum UCNACKIFG {
            /// No interrupt pending
            UCNACKIFG_0 = 0b0,
            /// Interrupt pending
            UCNACKIFG_1 = 0b1,
        }
        /// Byte counter interrupt flag
        UCBCNTIFG: 6 = enum UCBCNTIFG {
            /// No interrupt pending
            UCBCNTIFG_0 = 0b0,
            /// Interrupt pending
            UCBCNTIFG_1 = 0b1,
        }
        /// Clock low timeout interrupt flag
        UCCLTOIFG: 7 = enum UCCLTOIFG {
            /// No interrupt pending
            UCCLTOIFG_0 = 0b0,
            /// Interrupt pending
            UCCLTOIFG_1 = 0b1,
        }
        /// eUSCI_B receive interrupt flag 1
        UCRXIFG1: 8 = enum UCRXIFG1 {
            /// No interrupt pending
            UCRXIFG1_0 = 0b0,
            /// Interrupt pending
            UCRXIFG1_1 = 0b1,
        }
        /// eUSCI_B transmit interrupt flag 1
        UCTXIFG1: 9 = enum UCTXIFG1 {
            /// No interrupt pending
            UCTXIFG1_0 = 0b0,
            /// Interrupt pending
            UCTXIFG1_1 = 0b1,
        }
        /// eUSCI_B receive interrupt flag 2
        UCRXIFG2: 10 = enum UCRXIFG2 {
            /// No interrupt pending
            UCRXIFG2_0 = 0b0,
            /// Interrupt pending
            UCRXIFG2_1 = 0b1,
        }
        /// eUSCI_B transmit interrupt flag 2
        UCTXIFG2: 11 = enum UCTXIFG2 {
            /// No interrupt pending
            UCTXIFG2_0 = 0b0,
            /// Interrupt pending
            UCTXIFG2_1 = 0b1,
        }
        /// eUSCI_B receive interrupt flag 3
        UCRXIFG3: 12 = enum UCRXIFG3 {
            /// No interrupt pending
            UCRXIFG3_0 = 0b0,
            /// Interrupt pending
            UCRXIFG3_1 = 0b1,
        }
        /// eUSCI_B transmit interrupt flag 3
        UCTXIFG3: 13 = enum UCTXIFG3 {
            /// No interrupt pending
            UCTXIFG3_0 = 0b0,
            /// Interrupt pending
            UCTXIFG3_1 = 0b1,
        }
        /// Bit position 9 interrupt flag
        UCBIT9IFG: 14 = enum UCBIT9IFG {
            /// No interrupt pending
            UCBIT9IFG_0 = 0b0,
            /// Interrupt pending
            UCBIT9IFG_1 = 0b1,
        }
    }
    /// eUSCI_Bx Interrupt Flag Register
    rw UCB2IFG_SPI @ 0x2c: u16 = 0_0 {
        /// Receive interrupt flag
        UCRXIFG: 0 = enum UCRXIFG {
            /// No interrupt pending
            UCRXIFG_0 = 0b0,
            /// Interrupt pending
            UCRXIFG_1 = 0b1,
        }
        /// Transmit interrupt flag
        UCTXIFG: 1 = enum UCTXIFG {
            /// No interrupt pending
            UCTXIFG_0 = 0b0,
            /// Interrupt pending
            UCTXIFG_1 = 0b1,
        }
    }
    /// eUSCI_Bx Interrupt Vector Register
    r UCB2IV @ 0x2e: u16 = 0_0 {
        /// eUSCI_B interrupt vector value
        UCB2IV_UCIV: 0..15 = enum UCB2IV_UCIV {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest
            UCALIFG = 0b0000000000000010,
            /// Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG
            UCNACKIFG = 0b0000000000000100,
            /// Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG
            UCSTTIFG = 0b0000000000000110,
            /// Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG
            UCSTPIFG = 0b0000000000001000,
            /// Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3
            UCRXIFG3 = 0b0000000000001010,
            /// Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3
            UCTXIFG3 = 0b0000000000001100,
            /// Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2
            UCRXIFG2 = 0b0000000000001110,
            /// Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2
            UCTXIFG2 = 0b0000000000010000,
            /// Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1
            UCRXIFG1 = 0b0000000000010010,
            /// Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1
            UCTXIFG1 = 0b0000000000010100,
            /// Interrupt Source: Data received; Interrupt Flag: UCRXIFG0
            UCRXIFG0 = 0b0000000000010110,
            /// Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0
            UCTXIFG0 = 0b0000000000011000,
            /// Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG
            UCBCNTIFG = 0b0000000000011010,
            /// Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG
            UCCLTOIFG = 0b0000000000011100,
            /// Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest
            UCBIT9IFG = 0b0000000000011110,
        }
    }
    /// eUSCI_Bx Interrupt Vector Register
    r UCB2IV_SPI @ 0x2e: u16 = 0_0 {
        /// eUSCI_B interrupt vector value
        UCB2IV_SPI_UCIV: 0..15 = enum UCB2IV_SPI_UCIV {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest
            UCRXIFG = 0b0000000000000010,
            /// Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest
            UCTXIFG = 0b0000000000000100,
        }
    }
}
