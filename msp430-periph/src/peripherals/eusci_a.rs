//! eUSCI_A

utils::periph! {
    /// eUSCI_A
    eUSCI_A;
    /// eUSCI_Ax Control Word Register 0
    rw CTLW0 @ 0x00: u16 = 0_0 {
        /// Software reset enable
        SWRST: 0 = enum SWRST {
            /// Disabled. eUSCI_A reset released for operation
            DISABLE = 0b0,
            /// Enabled. eUSCI_A logic held in reset state
            ENABLE = 0b1,
        }
        /// Transmit break
        TXBRK: 1 = enum TXBRK {
            /// Next frame transmitted is not a break
            TXBRK_0 = 0b0,
            /// Next frame transmitted is a break or a break/synch
            TXBRK_1 = 0b1,
        }
        /// Transmit address
        TXADDR: 2 = enum TXADDR {
            /// Next frame transmitted is data
            TXADDR_0 = 0b0,
            /// Next frame transmitted is an address
            TXADDR_1 = 0b1,
        }
        /// Dormant
        DORM: 3 = enum DORM {
            /// Not dormant. All received characters set UCRXIFG.
            DORM_0 = 0b0,
            /// Dormant. Only characters that are preceded by an idle-line or with address bit set UCRXIFG. In UART mode with automatic baud-rate detection, only the combination of a break and synch field sets UCRXIFG.
            DORM_1 = 0b1,
        }
        /// Receive break character interrupt enable
        BRKIE: 4 = enum BRKIE {
            /// Received break characters do not set UCRXIFG
            BRKIE_0 = 0b0,
            /// Received break characters set UCRXIFG
            BRKIE_1 = 0b1,
        }
        /// Receive erroneous-character interrupt enable
        RXEIE: 5 = enum RXEIE {
            /// Erroneous characters rejected and UCRXIFG is not set
            RXEIE_0 = 0b0,
            /// Erroneous characters received set UCRXIFG
            RXEIE_1 = 0b1,
        }
        /// eUSCI_A clock source select
        SSEL: 6..7 = enum SSEL {
            /// UCLK
            UCLK = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// SMCLK
            SMCLK = 0b10,
            /// SMCLK
            SSEL_3 = 0b11,
        }
        /// Synchronous mode enable
        SYNC: 8 = enum SYNC {
            /// Asynchronous mode
            ASYNC = 0b0,
            /// Synchronous mode
            SYNC = 0b1,
        }
        /// eUSCI_A mode
        MODE: 9..10 = enum MODE {
            /// UART mode
            MODE_0 = 0b00,
            /// Idle-line multiprocessor mode
            MODE_1 = 0b01,
            /// Address-bit multiprocessor mode
            MODE_2 = 0b10,
            /// UART mode with automatic baud-rate detection
            MODE_3 = 0b11,
        }
        /// Stop bit select
        SPB: 11 = enum SPB {
            /// One stop bit
            SPB_0 = 0b0,
            /// Two stop bits
            SPB_1 = 0b1,
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
        /// Parity select
        PAR: 14 = enum PAR {
            /// Odd parity
            ODD = 0b0,
            /// Even parity
            EVEN = 0b1,
        }
        /// Parity enable
        PEN: 15 = enum PEN {
            /// Parity disabled
            PEN_0 = 0b0,
            /// Parity enabled. Parity bit is generated (UCAxTXD) and expected (UCAxRXD). In address-bit multiprocessor mode, the address bit is included in the parity calculation.
            PEN_1 = 0b1,
        }
    }
    /// eUSCI_Ax Control Word Register 0
    rw CTLW0_SPI @ 0x00: u16 = 0_0 {
        /// Software reset enable
        SWRST_SPI: 0 = enum SWRST_SPI {
            /// Disabled. eUSCI_A reset released for operation
            DISABLE = 0b0,
            /// Enabled. eUSCI_A logic held in reset state
            ENABLE = 0b1,
        }
        /// STE mode select in master mode.
        STEM: 1 = enum STEM {
            /// STE pin is used to prevent conflicts with other masters
            STEM_0 = 0b0,
            /// STE pin is used to generate the enable signal for a 4-wire slave
            STEM_1 = 0b1,
        }
        /// eUSCI_A clock source select
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
        MST: 11 = enum MST {
            /// Slave mode
            SLAVE = 0b0,
            /// Master mode
            MASTER = 0b1,
        }
        /// Character length
        _7BIT_SPI: 12 = enum _7BIT_SPI {
            /// 8-bit data
            _8BIT = 0b0,
            /// 7-bit data
            _7BIT = 0b1,
        }
        /// MSB first select
        MSB_SPI: 13 = enum MSB_SPI {
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
    /// eUSCI_Ax Control Word Register 1
    rw CTLW1 @ 0x02: u16 = 0_0 {
        /// Deglitch time
        GLIT: 0..1 = enum GLIT {
            /// Approximately 2 ns (equivalent of 1 delay element)
            GLIT_0 = 0b00,
            /// Approximately 50 ns
            GLIT_1 = 0b01,
            /// Approximately 100 ns
            GLIT_2 = 0b10,
            /// Approximately 200 ns
            GLIT_3 = 0b11,
        }
    }
    /// eUSCI_Ax Baud Rate Control Word Register
    rw BRW @ 0x06: u16 = 0_0 {
        /// eUSCI_Ax Baud Rate Control Word Register
        BRW: 0..15 = struct BRWField(u16);
    }
    /// eUSCI_Ax Bit Rate Control Register 1
    rw BRW_SPI @ 0x06: u16 = 0_0 {
        /// eUSCI_Ax Bit Rate Control Register 1
        BRW_SPI: 0..15 = struct BRW_SPIField(u16);
    }
    /// eUSCI_Ax Modulation Control Word Register
    rw MCTLW @ 0x08: u16 = 0_0 {
        /// Oversampling mode enabled
        OS16: 0 = enum OS16 {
            /// Disabled
            OS16_0 = 0b0,
            /// Enabled
            OS16_1 = 0b1,
        }
        /// First modulation stage select
        BRF: 4..7 = struct BRF(u16);
        /// Second modulation stage select
        BRS: 8..15 = struct BRS(u16);
    }
    /// eUSCI_Ax Status Register
    rw STATW @ 0x0a: u16 = 0_0 {
        /// eUSCI_A busy
        BUSY: 0 = enum BUSY {
            /// eUSCI_A inactive
            IDLE = 0b0,
            /// eUSCI_A transmitting or receiving
            BUSY = 0b1,
        }
        /// Address received / Idle line detected
        ADDR_IDLE: 1 = enum ADDR_IDLE {
            /// UCADDR: Received character is data. UCIDLE: No idle line detected
            ADDR_IDLE_0 = 0b0,
            /// UCADDR: Received character is an address. UCIDLE: Idle line detected
            ADDR_IDLE_1 = 0b1,
        }
        /// Receive error flag
        RXERR: 2 = enum RXERR {
            /// No receive errors detected
            RXERR_0 = 0b0,
            /// Receive error detected
            RXERR_1 = 0b1,
        }
        /// Break detect flag
        BRK: 3 = enum BRK {
            /// No break condition
            BRK_0 = 0b0,
            /// Break condition occurred
            BRK_1 = 0b1,
        }
        /// Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read.
        PE: 4 = enum PE {
            /// No error
            PE_0 = 0b0,
            /// Character received with parity error
            PE_1 = 0b1,
        }
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
            /// Character received with low stop bit
            FE_1 = 0b1,
        }
        /// Listen enable
        LISTEN: 7 = enum LISTEN {
            /// Disabled
            LISTEN_0 = 0b0,
            /// Enabled. UCAxTXD is internally fed back to the receiver
            LISTEN_1 = 0b1,
        }
    }
    /// STATW_SPI
    rw STATW_SPI @ 0x0a: u16 = 0_0 {
        /// Overrun error flag
        OE_SPI: 5 = enum OE_SPI {
            /// No error
            OE_0 = 0b0,
            /// Overrun error occurred
            OE_1 = 0b1,
        }
        /// Framing error flag
        FE_SPI: 6 = enum FE_SPI {
            /// No error
            FE_0 = 0b0,
            /// Bus conflict occurred
            FE_1 = 0b1,
        }
        /// Listen enable
        LISTEN_SPI: 7 = enum LISTEN_SPI {
            /// Disabled
            LISTEN_0 = 0b0,
            /// Enabled. UCAxTXD is internally fed back to the receiver
            LISTEN_1 = 0b1,
        }
    }
    /// eUSCI_Ax Receive Buffer Register
    r RXBUF @ 0x0c: u16 = 0_0 {
        /// Receive data buffer
        RXBUF: 0..7 = struct RXBUFField(u16);
    }
    /// eUSCI_Ax Receive Buffer Register
    r RXBUF_SPI @ 0x0c: u16 = 0_0 {
        /// Receive data buffer
        RXBUF_SPI: 0..7 = struct RXBUF_SPIField(u16);
    }
    /// eUSCI_Ax Transmit Buffer Register
    rw TXBUF @ 0x0e: u16 = 0_0 {
        /// Transmit data buffer
        TXBUF: 0..7 = struct TXBUFField(u16);
    }
    /// eUSCI_Ax Transmit Buffer Register
    rw TXBUF_SPI @ 0x0e: u16 = 0_0 {
        /// Transmit data buffer
        TXBUF_SPIField: 0..7 = struct TXBUF_SPIField(u16);
    }
    /// eUSCI_Ax Auto Baud Rate Control Register
    rw ABCTL @ 0x10: u16 = 0_0 {
        /// Automatic baud-rate detect enable
        ABDEN: 0 = enum ABDEN {
            /// Baud-rate detection disabled. Length of break and synch field is not measured.
            ABDEN_0 = 0b0,
            /// Baud-rate detection enabled. Length of break and synch field is measured and baud-rate settings are changed accordingly.
            ABDEN_1 = 0b1,
        }
        /// Break time out error
        BTOE: 2 = enum BTOE {
            /// No error
            BTOE_0 = 0b0,
            /// Length of break field exceeded 22 bit times
            BTOE_1 = 0b1,
        }
        /// Synch field time out error
        STOE: 3 = enum STOE {
            /// No error
            STOE_0 = 0b0,
            /// Length of synch field exceeded measurable time
            STOE_1 = 0b1,
        }
        /// Break/synch delimiter length
        DELIM: 4..5 = enum DELIM {
            /// 1 bit time
            DELIM_0 = 0b00,
            /// 2 bit times
            DELIM_1 = 0b01,
            /// 3 bit times
            DELIM_2 = 0b10,
            /// 4 bit times
            DELIM_3 = 0b11,
        }
    }
    /// eUSCI_Ax IrDA Control Word Register
    rw IRCTL @ 0x12: u16 = 0_0 {
        /// IrDA encoder/decoder enable
        IREN: 0 = enum IREN {
            /// IrDA encoder/decoder disabled
            IREN_0 = 0b0,
            /// IrDA encoder/decoder enabled
            IREN_1 = 0b1,
        }
        /// IrDA transmit pulse clock select
        IRTXCLK: 1 = enum IRTXCLK {
            /// BRCLK
            IRTXCLK_0 = 0b0,
            /// BITCLK16 when UCOS16 = 1. Otherwise, BRCLK.
            IRTXCLK_1 = 0b1,
        }
        /// Transmit pulse length
        IRTXPL: 2..7 = struct IRTXPL(u16);
        /// IrDA receive filter enabled
        IRRXFE: 8 = enum IRRXFE {
            /// Receive filter disabled
            IRRXFE_0 = 0b0,
            /// Receive filter enabled
            IRRXFE_1 = 0b1,
        }
        /// IrDA receive input UCAxRXD polarity
        IRRXPL: 9 = enum IRRXPL {
            /// IrDA transceiver delivers a high pulse when a light pulse is seen
            HIGH = 0b0,
            /// IrDA transceiver delivers a low pulse when a light pulse is seen
            LOW = 0b1,
        }
        /// Receive filter length
        IRRXFL: 10..15 = struct IRRXFL(u16);
    }
    /// eUSCI_Ax Interrupt Enable Register
    rw IE @ 0x1a: u16 = 0_0 {
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
        /// Start bit interrupt enable
        STTIE: 2 = enum STTIE {
            /// Interrupt disabled
            STTIE_0 = 0b0,
            /// Interrupt enabled
            STTIE_1 = 0b1,
        }
        /// Transmit complete interrupt enable
        TXCPTIE: 3 = enum TXCPTIE {
            /// Interrupt disabled
            TXCPTIE_0 = 0b0,
            /// Interrupt enabled
            TXCPTIE_1 = 0b1,
        }
    }
    /// eUSCI_Ax Interrupt Enable Register
    rw IE_SPI @ 0x1a: u16 = 0_0 {
        /// Receive interrupt enable
        RXIE_SPI: 0 = enum RXIE_SPI {
            /// Interrupt disabled
            RXIE_0 = 0b0,
            /// Interrupt enabled
            RXIE_1 = 0b1,
        }
        /// Transmit interrupt enable
        TXIE_SPI: 1 = enum TXIE_SPI {
            /// Interrupt disabled
            TXIE_0 = 0b0,
            /// Interrupt enabled
            TXIE_1 = 0b1,
        }
    }
    /// eUSCI_Ax Interrupt Flag Register
    rw IFG @ 0x1c: u16 = 0_0 {
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
        /// Start bit interrupt flag
        STTIFG: 2 = enum STTIFG {
            /// No interrupt pending
            STTIFG_0 = 0b0,
            /// Interrupt pending
            STTIFG_1 = 0b1,
        }
        /// Transmit ready interrupt enable
        TXCPTIFG: 3 = enum TXCPTIFG {
            /// No interrupt pending
            TXCPTIFG_0 = 0b0,
            /// Interrupt pending
            TXCPTIFG_1 = 0b1,
        }
    }
    /// eUSCI_Ax Interrupt Flag Register
    rw IFG_SPI @ 0x1c: u16 = 0_0 {
        /// Receive interrupt flag
        RXIFG_SPI: 0 = enum RXIFG_SPI {
            /// No interrupt pending
            RXIFG_0 = 0b0,
            /// Interrupt pending
            RXIFG_1 = 0b1,
        }
        /// Transmit interrupt flag
        TXIFG_SPI: 1 = enum TXIFG_SPI {
            /// No interrupt pending
            TXIFG_0 = 0b0,
            /// Interrupt pending
            TXIFG_1 = 0b1,
        }
    }
    /// eUSCI_Ax Interrupt Vector Register
    r IV @ 0x1e: u16 = 0_0 {
        /// eUSCI_A interrupt vector value
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest
            RXIFG = 0b0000000000000010,
            /// Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG
            TXIFG = 0b0000000000000100,
            /// Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG
            STTIFG = 0b0000000000000110,
            /// Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest
            TXCPTIFG = 0b0000000000001000,
        }
    }
    /// eUSCI_Ax Interrupt Vector Register
    r IV_SPI @ 0x1e: u16 = 0_0 {
        /// eUSCI_A interrupt vector value
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
