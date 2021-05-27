//! eUSCI_B

utils::periph! {
    /// eUSCI_B
    eUSCI_B;
    /// eUSCI_Bx Control Word Register 0
    rw CTLW0 @ 0x00: u16 = 0_0 {
        /// Software reset enable
        SWRST: 0 = enum SWRST {
            /// Disabled. eUSCI_B reset released for operation
            DISABLE = 0b0,
            /// Enabled. eUSCI_B logic held in reset state
            ENABLE = 0b1,
        }
        /// Transmit START condition in master mode
        TXSTT: 1 = enum TXSTT {
            /// Do not generate START condition
            TXSTT_0 = 0b0,
            /// Generate START condition
            TXSTT_1 = 0b1,
        }
        /// Transmit STOP condition in master mode
        TXSTP: 2 = enum TXSTP {
            /// No STOP generated
            TXSTP_0 = 0b0,
            /// Generate STOP
            TXSTP_1 = 0b1,
        }
        /// Transmit a NACK
        TXNACK: 3 = enum TXNACK {
            /// Acknowledge normally
            TXNACK_0 = 0b0,
            /// Generate NACK
            TXNACK_1 = 0b1,
        }
        /// Transmitter/receiver
        TR: 4 = enum TR {
            /// Receiver
            RX = 0b0,
            /// Transmitter
            TX = 0b1,
        }
        /// Transmit ACK condition in slave mode
        TXACK: 5 = enum TXACK {
            /// Do not acknowledge the slave address
            TXACK_0 = 0b0,
            /// Acknowledge the slave address
            TXACK_1 = 0b1,
        }
        /// eUSCI_B clock source select
        SSEL: 6..7 = enum SSEL {
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
        SYNC: 8 = enum SYNC {
            /// Asynchronous mode
            ASYNC = 0b0,
            /// Synchronous mode
            SYNC = 0b1,
        }
        /// eUSCI_B mode
        MODE: 9..10 = enum MODE {
            /// 3-pin SPI
            MODE_0 = 0b00,
            /// 4-pin SPI (master or slave enabled if STE = 1)
            MODE_1 = 0b01,
            /// 4-pin SPI (master or slave enabled if STE = 0)
            MODE_2 = 0b10,
            /// I2C mode
            MODE_3 = 0b11,
        }
        /// Master mode select
        MST: 11 = enum MST {
            /// Slave mode
            SLAVE = 0b0,
            /// Master mode
            MASTER = 0b1,
        }
        /// Multi-master environment select
        MM: 13 = enum MM {
            /// Single master environment. There is no other master in the system. The address compare unit is disabled.
            SINGLE = 0b0,
            /// Multi-master environment
            MULTI = 0b1,
        }
        /// Slave addressing mode select
        SLA10: 14 = enum SLA10 {
            /// Address slave with 7-bit address
            _7BIT = 0b0,
            /// Address slave with 10-bit address
            _10BIT = 0b1,
        }
        /// Own addressing mode select
        A10: 15 = enum A10 {
            /// Own address is a 7-bit address
            A10_0 = 0b0,
            /// Own address is a 10-bit address
            A10_1 = 0b1,
        }
    }
    /// eUSCI_Bx Control Word Register 0
    rw BCTLW0_SPI @ 0x00: u16 = 0_0 {
        /// Software reset enable
        SWRST_SPI: 0 = enum SWRST_SPI {
            /// Disabled. eUSCI_B reset released for operation
            DISABLE = 0b0,
            /// Enabled. eUSCI_B logic held in reset state
            ENABLE = 0b1,
        }
        /// STE mode select in master mode.
        STEM: 1 = enum STEM {
            /// STE pin is used to prevent conflicts with other masters
            STEM_0 = 0b0,
            /// STE pin is used to generate the enable signal for a 4-wire slave
            STEM_1 = 0b1,
        }
        /// eUSCI_B clock source select
        SSEL_SPI: 6..7 = enum SSEL_SPI {
            /// Reserved
            SSEL_0 = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// SMCLK
            SMCLK = 0b10,
            /// SMCLK
            SSEL_3 = 0b11,
        }
        /// Synchronous mode enable
        SYNC_SPI: 8 = enum SYNC_SPI {
            /// Asynchronous mode
            ASYNC = 0b0,
            /// Synchronous mode
            SYNC = 0b1,
        }
        /// eUSCI mode
        MODE_SPI: 9..10 = enum MODE_SPI {
            /// 3-pin SPI
            MODE_0 = 0b00,
            /// 4-pin SPI with UCxSTE active high: Slave enabled when UCxSTE = 1
            MODE_1 = 0b01,
            /// 4-pin SPI with UCxSTE active low: Slave enabled when UCxSTE = 0
            MODE_2 = 0b10,
            /// I2C mode
            MODE_3 = 0b11,
        }
        /// Master mode select
        MST_SPI: 11 = enum MST_SPI {
            /// Slave mode
            SLAVE = 0b0,
            /// Master mode
            MASTER = 0b1,
        }
        /// Character length
        _7BIT: 12 = enum _7BIT {
            /// 8-bit data
            _8BIT = 0b0,
            /// 7-bit data
            _7BIT = 0b1,
        }
        /// MSB first select
        MSB: 13 = enum MSB {
            /// LSB first
            MSB_0 = 0b0,
            /// MSB first
            MSB_1 = 0b1,
        }
        /// Clock polarity select
        CKPL: 14 = enum CKPL {
            /// The inactive state is low
            LOW = 0b0,
            /// The inactive state is high
            HIGH = 0b1,
        }
        /// Clock phase select
        CKPH: 15 = enum CKPH {
            /// Data is changed on the first UCLK edge and captured on the following edge.
            CKPH_0 = 0b0,
            /// Data is captured on the first UCLK edge and changed on the following edge.
            CKPH_1 = 0b1,
        }
    }
    /// eUSCI_Bx Control Word Register 1
    rw CTLW1 @ 0x02: u16 = 0_0 {
        /// Deglitch time
        GLIT: 0..1 = enum GLIT {
            /// 50 ns
            GLIT_0 = 0b00,
            /// 25 ns
            GLIT_1 = 0b01,
            /// 12.5 ns
            GLIT_2 = 0b10,
            /// 6.25 ns
            GLIT_3 = 0b11,
        }
        /// Automatic STOP condition generation
        ASTP: 2..3 = enum ASTP {
            /// No automatic STOP generation. The STOP condition is generated after the user sets the UCTXSTP bit. The value in UCBxTBCNT is a don't care.
            ASTP_0 = 0b00,
            /// UCBCNTIFG is set with the byte counter reaches the threshold defined in UCBxTBCNT
            ASTP_1 = 0b01,
            /// A STOP condition is generated automatically after the byte counter value reached UCBxTBCNT. UCBCNTIFG is set with the byte counter reaching the threshold
            ASTP_2 = 0b10,
            /// Reserved
            ASTP_3 = 0b11,
        }
        /// SW or HW ACK control
        SWACK: 4 = enum SWACK {
            /// The address acknowledge of the slave is controlled by the eUSCI_B module
            SWACK_0 = 0b0,
            /// The user needs to trigger the sending of the address ACK by issuing UCTXACK
            SWACK_1 = 0b1,
        }
        /// ACK all master bytes
        STPNACK: 5 = enum STPNACK {
            /// Send a non-acknowledge before the STOP condition as a master receiver (conform to I2C standard)
            STPNACK_0 = 0b0,
            /// All bytes are acknowledged by the eUSCI_B when configured as master receiver
            STPNACK_1 = 0b1,
        }
        /// Clock low timeout select
        CLTO: 6..7 = enum CLTO {
            /// Disable clock low timeout counter
            CLTO_0 = 0b00,
            /// 135 000 SYSCLK cycles (approximately 28 ms)
            CLTO_1 = 0b01,
            /// 150 000 SYSCLK cycles (approximately 31 ms)
            CLTO_2 = 0b10,
            /// 165 000 SYSCLK cycles (approximately 34 ms)
            CLTO_3 = 0b11,
        }
        /// Early UCTXIFG0
        ETXINT: 8 = enum ETXINT {
            /// UCTXIFGx is set after an address match with UCxI2COAx and the direction bit indicating slave transmit
            ETXINT_0 = 0b0,
            /// UCTXIFG0 is set for each START condition
            ETXINT_1 = 0b1,
        }
    }
    /// eUSCI_Bx Baud Rate Control Word Register
    rw BRW @ 0x06: u16 = 0_0 {
        /// eUSCI_Bx Baud Rate Control Word Register
        BRW: 0..15 = struct BRWField(u16);
    }
    /// eUSCI_Bx Bit Rate Control Register 1
    rw BRW_SPI @ 0x06: u16 = 0_0 {
        /// eUSCI_Bx Bit Rate Control Register 1
        BRW_SPI: 0..15 = struct BRW_SPIField(u16);
    }
    /// eUSCI_Bx Status Register
    r STATW @ 0x08: u16 = 0_0 {
        /// Bus busy
        BUSY: 4 = enum BUSY {
            /// Bus inactive
            IDLE = 0b0,
            /// Bus busy
            BUSY = 0b1,
        }
        /// General call address received
        GC: 5 = enum GC {
            /// No general call address received
            GC_0 = 0b0,
            /// General call address received
            GC_1 = 0b1,
        }
        /// SCL low
        SCLLOW: 6 = enum SCLLOW {
            /// SCL is not held low
            SCLLOW_0 = 0b0,
            /// SCL is held low
            SCLLOW_1 = 0b1,
        }
        /// Hardware byte counter value
        CNT: 8..15 = struct CNT(u16);
    }
    /// UCBSTATW_SPI
    rw STATW_SPI @ 0x08: u16 = 0_0 {
        /// Overrun error flag
        OE: 5 = enum OE {
            /// No error
            OE_0 = 0b0,
            /// Overrun error occurred
            OE_1 = 0b1,
        }
        /// Framing error flag
        FE: 6 = enum FE {
            /// No error
            FE_0 = 0b0,
            /// Bus conflict occurred
            FE_1 = 0b1,
        }
        /// Listen enable
        LISTEN: 7 = enum LISTEN {
            /// Disabled
            LISTEN_0 = 0b0,
            /// Enabled. UCBxTXD is internally fed back to the receiver
            LISTEN_1 = 0b1,
        }
    }
    /// eUSCI_Bx Byte Counter Threshold Register
    rw TBCNT @ 0x0a: u16 = 0_0 {
        /// Byte counter threshold value
        TBCNT: 0..7 = struct TBCNTField(u16);
    }
    /// eUSCI_Bx Receive Buffer Register
    r RXBUF @ 0x0c: u16 = 0_0 {
        /// Receive data buffer
        RXBUF_UCRXBUF: 0..7 = struct RXBUF_UCRXBUF(u16);
    }
    /// eUSCI_Bx Receive Buffer Register
    r RXBUF_SPI @ 0x0c: u16 = 0_0 {
        /// Receive data buffer
        RXBUF_SPI_UCRXBUF: 0..7 = struct RXBUF_SPI_UCRXBUF(u16);
    }
    /// eUSCI_Bx Transmit Buffer Register
    rw TXBUF @ 0x0e: u16 = 0_0 {
        /// Transmit data buffer
        TXBUF_UCTXBUF: 0..7 = struct TXBUF_UCTXBUF(u16);
    }
    /// eUSCI_Bx Transmit Buffer Register
    rw TXBUF_SPI @ 0x0e: u16 = 0_0 {
        /// Transmit data buffer
        TXBUF_SPI_UCTXBUF: 0..7 = struct TXBUF_SPI_UCTXBUF(u16);
    }
    /// eUSCI_Bx I2C Own Address 0 Register
    rw I2COA0 @ 0x14: u16 = 0_0 {
        /// I2C own address
        I2COA0: 0..9 = struct I2COA0Field(u16);
        /// Own Address enable register
        I2COA0_UCOAEN: 10 = enum I2COA0_UCOAEN {
            /// The slave address defined in I2COA0 is disabled
            DISABLE = 0b0,
            /// The slave address defined in I2COA0 is enabled
            ENABLE = 0b1,
        }
        /// General call response enable
        GCEN: 15 = enum GCEN {
            /// Do not respond to a general call
            GCEN_0 = 0b0,
            /// Respond to a general call
            GCEN_1 = 0b1,
        }
    }
    /// eUSCI_Bx I2C Own Address 1 Register
    rw I2COA1 @ 0x16: u16 = 0_0 {
        /// I2C own address
        I2COA1: 0..9 = struct I2COA1Field(u16);
        /// Own Address enable register
        I2COA1_UCOAEN: 10 = enum I2COA1_UCOAEN {
            /// The slave address defined in I2COA1 is disabled
            DISABLE = 0b0,
            /// The slave address defined in I2COA1 is enabled
            ENABLE = 0b1,
        }
    }
    /// eUSCI_Bx I2C Own Address 2 Register
    rw I2COA2 @ 0x18: u16 = 0_0 {
        /// I2C own address
        I2COA2: 0..9 = struct I2COA2Field(u16);
        /// Own Address enable register
        I2COA2_UCOAEN: 10 = enum I2COA2_UCOAEN {
            /// The slave address defined in I2COA2 is disabled
            DISABLE = 0b0,
            /// The slave address defined in I2COA2 is enabled
            ENABLE = 0b1,
        }
    }
    /// eUSCI_Bx I2C Own Address 3 Register
    rw I2COA3 @ 0x1a: u16 = 0_0 {
        /// I2C own address
        I2COA3: 0..9 = struct I2COA3Field(u16);
        /// Own Address enable register
        I2COA3_UCOAEN: 10 = enum I2COA3_UCOAEN {
            /// The slave address defined in I2COA3 is disabled
            DISABLE = 0b0,
            /// The slave address defined in I2COA3 is enabled
            ENABLE = 0b1,
        }
    }
    /// eUSCI_Bx I2C Received Address Register
    r ADDRX @ 0x1c: u16 = 0_0 {
        /// Received Address Register
        ADDRX: 0..9 = struct ADDRXField(u16);
    }
    /// eUSCI_Bx I2C Address Mask Register
    rw ADDMASK @ 0x1e: u16 = 0_0 {
        /// Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1.
        ADDMASK: 0..9 = struct ADDMASKField(u16);
    }
    /// eUSCI_Bx I2C Slave Address Register
    rw I2CSA @ 0x20: u16 = 0_0 {
        /// I2C slave address
        I2CSA: 0..9 = struct I2CSAField(u16);
    }
    /// eUSCI_Bx Interrupt Enable Register
    rw IE @ 0x2a: u16 = 0_0 {
        /// Receive interrupt enable 0
        RXIE0: 0 = enum RXIE0 {
            /// Interrupt disabled
            RXIE0_0 = 0b0,
            /// Interrupt enabled
            RXIE0_1 = 0b1,
        }
        /// Transmit interrupt enable 0
        TXIE0: 1 = enum TXIE0 {
            /// Interrupt disabled
            TXIE0_0 = 0b0,
            /// Interrupt enabled
            TXIE0_1 = 0b1,
        }
        /// START condition interrupt enable
        STTIE: 2 = enum STTIE {
            /// Interrupt disabled
            STTIE_0 = 0b0,
            /// Interrupt enabled
            STTIE_1 = 0b1,
        }
        /// STOP condition interrupt enable
        STPIE: 3 = enum STPIE {
            /// Interrupt disabled
            STPIE_0 = 0b0,
            /// Interrupt enabled
            STPIE_1 = 0b1,
        }
        /// Arbitration lost interrupt enable
        ALIE: 4 = enum ALIE {
            /// Interrupt disabled
            ALIE_0 = 0b0,
            /// Interrupt enabled
            ALIE_1 = 0b1,
        }
        /// Not-acknowledge interrupt enable
        NACKIE: 5 = enum NACKIE {
            /// Interrupt disabled
            NACKIE_0 = 0b0,
            /// Interrupt enabled
            NACKIE_1 = 0b1,
        }
        /// Byte counter interrupt enable
        BCNTIE: 6 = enum BCNTIE {
            /// Interrupt disabled
            BCNTIE_0 = 0b0,
            /// Interrupt enabled
            BCNTIE_1 = 0b1,
        }
        /// Clock low timeout interrupt enable
        CLTOIE: 7 = enum CLTOIE {
            /// Interrupt disabled
            CLTOIE_0 = 0b0,
            /// Interrupt enabled
            CLTOIE_1 = 0b1,
        }
        /// Receive interrupt enable 1
        RXIE1: 8 = enum RXIE1 {
            /// Interrupt disabled
            RXIE1_0 = 0b0,
            /// Interrupt enabled
            RXIE1_1 = 0b1,
        }
        /// Transmit interrupt enable 1
        TXIE1: 9 = enum TXIE1 {
            /// Interrupt disabled
            TXIE1_0 = 0b0,
            /// Interrupt enabled
            TXIE1_1 = 0b1,
        }
        /// Receive interrupt enable 2
        RXIE2: 10 = enum RXIE2 {
            /// Interrupt disabled
            RXIE2_0 = 0b0,
            /// Interrupt enabled
            RXIE2_1 = 0b1,
        }
        /// Transmit interrupt enable 2
        TXIE2: 11 = enum TXIE2 {
            /// Interrupt disabled
            TXIE2_0 = 0b0,
            /// Interrupt enabled
            TXIE2_1 = 0b1,
        }
        /// Receive interrupt enable 3
        RXIE3: 12 = enum RXIE3 {
            /// Interrupt disabled
            RXIE3_0 = 0b0,
            /// Interrupt enabled
            RXIE3_1 = 0b1,
        }
        /// Transmit interrupt enable 3
        TXIE3: 13 = enum TXIE3 {
            /// Interrupt disabled
            TXIE3_0 = 0b0,
            /// Interrupt enabled
            TXIE3_1 = 0b1,
        }
        /// Bit position 9 interrupt enable
        IT9IE: 14 = enum IT9IE {
            /// Interrupt disabled
            IT9IE_0 = 0b0,
            /// Interrupt enabled
            IT9IE_1 = 0b1,
        }
    }
    /// eUSCI_Bx Interrupt Enable Register
    rw IE_SPI @ 0x2a: u16 = 0_0 {
        /// Receive interrupt enable
        RXIE: 0 = enum RXIE {
            /// Interrupt disabled
            RXIE_0 = 0b0,
            /// Interrupt enabled
            RXIE_1 = 0b1,
        }
        /// Transmit interrupt enable
        TXIE: 1 = enum TXIE {
            /// Interrupt disabled
            TXIE_0 = 0b0,
            /// Interrupt enabled
            TXIE_1 = 0b1,
        }
    }
    /// eUSCI_Bx Interrupt Flag Register
    rw IFG @ 0x2c: u16 = 0_0 {
        /// eUSCI_B receive interrupt flag 0
        RXIFG0: 0 = enum RXIFG0 {
            /// No interrupt pending
            RXIFG0_0 = 0b0,
            /// Interrupt pending
            RXIFG0_1 = 0b1,
        }
        /// eUSCI_B transmit interrupt flag 0
        TXIFG0: 1 = enum TXIFG0 {
            /// No interrupt pending
            TXIFG0_0 = 0b0,
            /// Interrupt pending
            TXIFG0_1 = 0b1,
        }
        /// START condition interrupt flag
        STTIFG: 2 = enum STTIFG {
            /// No interrupt pending
            STTIFG_0 = 0b0,
            /// Interrupt pending
            STTIFG_1 = 0b1,
        }
        /// STOP condition interrupt flag
        STPIFG: 3 = enum STPIFG {
            /// No interrupt pending
            STPIFG_0 = 0b0,
            /// Interrupt pending
            STPIFG_1 = 0b1,
        }
        /// Arbitration lost interrupt flag
        ALIFG: 4 = enum ALIFG {
            /// No interrupt pending
            ALIFG_0 = 0b0,
            /// Interrupt pending
            ALIFG_1 = 0b1,
        }
        /// Not-acknowledge received interrupt flag
        NACKIFG: 5 = enum NACKIFG {
            /// No interrupt pending
            NACKIFG_0 = 0b0,
            /// Interrupt pending
            NACKIFG_1 = 0b1,
        }
        /// Byte counter interrupt flag
        BCNTIFG: 6 = enum BCNTIFG {
            /// No interrupt pending
            BCNTIFG_0 = 0b0,
            /// Interrupt pending
            BCNTIFG_1 = 0b1,
        }
        /// Clock low timeout interrupt flag
        CLTOIFG: 7 = enum CLTOIFG {
            /// No interrupt pending
            CLTOIFG_0 = 0b0,
            /// Interrupt pending
            CLTOIFG_1 = 0b1,
        }
        /// eUSCI_B receive interrupt flag 1
        RXIFG1: 8 = enum RXIFG1 {
            /// No interrupt pending
            RXIFG1_0 = 0b0,
            /// Interrupt pending
            RXIFG1_1 = 0b1,
        }
        /// eUSCI_B transmit interrupt flag 1
        TXIFG1: 9 = enum TXIFG1 {
            /// No interrupt pending
            TXIFG1_0 = 0b0,
            /// Interrupt pending
            TXIFG1_1 = 0b1,
        }
        /// eUSCI_B receive interrupt flag 2
        RXIFG2: 10 = enum RXIFG2 {
            /// No interrupt pending
            RXIFG2_0 = 0b0,
            /// Interrupt pending
            RXIFG2_1 = 0b1,
        }
        /// eUSCI_B transmit interrupt flag 2
        TXIFG2: 11 = enum TXIFG2 {
            /// No interrupt pending
            TXIFG2_0 = 0b0,
            /// Interrupt pending
            TXIFG2_1 = 0b1,
        }
        /// eUSCI_B receive interrupt flag 3
        RXIFG3: 12 = enum RXIFG3 {
            /// No interrupt pending
            RXIFG3_0 = 0b0,
            /// Interrupt pending
            RXIFG3_1 = 0b1,
        }
        /// eUSCI_B transmit interrupt flag 3
        TXIFG3: 13 = enum TXIFG3 {
            /// No interrupt pending
            TXIFG3_0 = 0b0,
            /// Interrupt pending
            TXIFG3_1 = 0b1,
        }
        /// Bit position 9 interrupt flag
        IT9IFG: 14 = enum IT9IFG {
            /// No interrupt pending
            IT9IFG_0 = 0b0,
            /// Interrupt pending
            IT9IFG_1 = 0b1,
        }
    }
    /// eUSCI_Bx Interrupt Flag Register
    rw IFG_SPI @ 0x2c: u16 = 0_0 {
        /// Receive interrupt flag
        RXIFG: 0 = enum RXIFG {
            /// No interrupt pending
            RXIFG_0 = 0b0,
            /// Interrupt pending
            RXIFG_1 = 0b1,
        }
        /// Transmit interrupt flag
        TXIFG: 1 = enum TXIFG {
            /// No interrupt pending
            TXIFG_0 = 0b0,
            /// Interrupt pending
            TXIFG_1 = 0b1,
        }
    }
    /// eUSCI_Bx Interrupt Vector Register
    r IV @ 0x2e: u16 = 0_0 {
        /// eUSCI_B interrupt vector value
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest
            ALIFG = 0b0000000000000010,
            /// Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG
            NACKIFG = 0b0000000000000100,
            /// Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG
            STTIFG = 0b0000000000000110,
            /// Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG
            STPIFG = 0b0000000000001000,
            /// Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3
            RXIFG3 = 0b0000000000001010,
            /// Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3
            TXIFG3 = 0b0000000000001100,
            /// Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2
            RXIFG2 = 0b0000000000001110,
            /// Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2
            TXIFG2 = 0b0000000000010000,
            /// Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1
            RXIFG1 = 0b0000000000010010,
            /// Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1
            TXIFG1 = 0b0000000000010100,
            /// Interrupt Source: Data received; Interrupt Flag: UCRXIFG0
            RXIFG0 = 0b0000000000010110,
            /// Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0
            TXIFG0 = 0b0000000000011000,
            /// Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG
            BCNTIFG = 0b0000000000011010,
            /// Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG
            CLTOIFG = 0b0000000000011100,
            /// Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest
            BIT9IFG = 0b0000000000011110,
        }
    }
    /// eUSCI_Bx Interrupt Vector Register
    r IV_SPI @ 0x2e: u16 = 0_0 {
        /// eUSCI_B interrupt vector value
        IV_SPI: 0..15 = enum IV_SPIField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest
            RXIFG = 0b0000000000000010,
            /// Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest
            TXIFG = 0b0000000000000100,
        }
    }
}
