//! eUSCI_A

utils::periph! {
    /// eUSCI_A
    eUSCI_A;
    /// eUSCI_Ax Control Word Register 0
    rw UCACTLW0 @ 0x00: u16 = 0_0 {
        /// Software reset enable
        UCACTLW0_UCSWRST: 0 = enum UCACTLW0_UCSWRST {
            /// Disabled. eUSCI_A reset released for operation
            DISABLE = 0b0,
            /// Enabled. eUSCI_A logic held in reset state
            ENABLE = 0b1,
        }
        /// Transmit break
        UCTXBRK: 1 = enum UCTXBRK {
            /// Next frame transmitted is not a break
            UCTXBRK_0 = 0b0,
            /// Next frame transmitted is a break or a break/synch
            UCTXBRK_1 = 0b1,
        }
        /// Transmit address
        UCTXADDR: 2 = enum UCTXADDR {
            /// Next frame transmitted is data
            UCTXADDR_0 = 0b0,
            /// Next frame transmitted is an address
            UCTXADDR_1 = 0b1,
        }
        /// Dormant
        UCDORM: 3 = enum UCDORM {
            /// Not dormant. All received characters set UCRXIFG.
            UCDORM_0 = 0b0,
            /// Dormant. Only characters that are preceded by an idle-line or with address bit set UCRXIFG. In UART mode with automatic baud-rate detection, only the combination of a break and synch field sets UCRXIFG.
            UCDORM_1 = 0b1,
        }
        /// Receive break character interrupt enable
        UCBRKIE: 4 = enum UCBRKIE {
            /// Received break characters do not set UCRXIFG
            UCBRKIE_0 = 0b0,
            /// Received break characters set UCRXIFG
            UCBRKIE_1 = 0b1,
        }
        /// Receive erroneous-character interrupt enable
        UCRXEIE: 5 = enum UCRXEIE {
            /// Erroneous characters rejected and UCRXIFG is not set
            UCRXEIE_0 = 0b0,
            /// Erroneous characters received set UCRXIFG
            UCRXEIE_1 = 0b1,
        }
        /// eUSCI_A clock source select
        UCACTLW0_UCSSEL: 6..7 = enum UCACTLW0_UCSSEL {
            /// UCLK
            UCLK = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// SMCLK
            SMCLK = 0b10,
            /// SMCLK
            UCSSEL_3 = 0b11,
        }
        /// Synchronous mode enable
        UCACTLW0_UCSYNC: 8 = enum UCACTLW0_UCSYNC {
            /// Asynchronous mode
            ASYNC = 0b0,
            /// Synchronous mode
            SYNC = 0b1,
        }
        /// eUSCI_A mode
        UCACTLW0_UCMODE: 9..10 = enum UCACTLW0_UCMODE {
            /// UART mode
            UCMODE_0 = 0b00,
            /// Idle-line multiprocessor mode
            UCMODE_1 = 0b01,
            /// Address-bit multiprocessor mode
            UCMODE_2 = 0b10,
            /// UART mode with automatic baud-rate detection
            UCMODE_3 = 0b11,
        }
        /// Stop bit select
        UCSPB: 11 = enum UCSPB {
            /// One stop bit
            UCSPB_0 = 0b0,
            /// Two stop bits
            UCSPB_1 = 0b1,
        }
        /// Character length
        UCACTLW0_UC7BIT: 12 = enum UCACTLW0_UC7BIT {
            /// 8-bit data
            _8BIT = 0b0,
            /// 7-bit data
            _7BIT = 0b1,
        }
        /// MSB first select
        UCACTLW0_UCMSB: 13 = enum UCACTLW0_UCMSB {
            /// LSB first
            UCMSB_0 = 0b0,
            /// MSB first
            UCMSB_1 = 0b1,
        }
        /// Parity select
        UCPAR: 14 = enum UCPAR {
            /// Odd parity
            ODD = 0b0,
            /// Even parity
            EVEN = 0b1,
        }
        /// Parity enable
        UCPEN: 15 = enum UCPEN {
            /// Parity disabled
            UCPEN_0 = 0b0,
            /// Parity enabled. Parity bit is generated (UCAxTXD) and expected (UCAxRXD). In address-bit multiprocessor mode, the address bit is included in the parity calculation.
            UCPEN_1 = 0b1,
        }
    }
    /// eUSCI_Ax Control Word Register 0
    rw UCACTLW0_SPI @ 0x00: u16 = 0_0 {
        /// Software reset enable
        UCACTLW0_SPI_UCSWRST: 0 = enum UCACTLW0_SPI_UCSWRST {
            /// Disabled. eUSCI_A reset released for operation
            DISABLE = 0b0,
            /// Enabled. eUSCI_A logic held in reset state
            ENABLE = 0b1,
        }
        /// STE mode select in master mode.
        UCSTEM: 1 = enum UCSTEM {
            /// STE pin is used to prevent conflicts with other masters
            UCSTEM_0 = 0b0,
            /// STE pin is used to generate the enable signal for a 4-wire slave
            UCSTEM_1 = 0b1,
        }
        /// eUSCI_A clock source select
        UCACTLW0_SPI_UCSSEL: 6..7 = enum UCACTLW0_SPI_UCSSEL {
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
        UCACTLW0_SPI_UCSYNC: 8 = enum UCACTLW0_SPI_UCSYNC {
            /// Asynchronous mode
            ASYNC = 0b0,
            /// Synchronous mode
            SYNC = 0b1,
        }
        /// eUSCI mode
        UCACTLW0_SPI_UCMODE: 9..10 = enum UCACTLW0_SPI_UCMODE {
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
        UCMST: 11 = enum UCMST {
            /// Slave mode
            SLAVE = 0b0,
            /// Master mode
            MASTER = 0b1,
        }
        /// Character length
        UCACTLW0_SPI_UC7BIT: 12 = enum UCACTLW0_SPI_UC7BIT {
            /// 8-bit data
            _8BIT = 0b0,
            /// 7-bit data
            _7BIT = 0b1,
        }
        /// MSB first select
        UCACTLW0_SPI_UCMSB: 13 = enum UCACTLW0_SPI_UCMSB {
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
    /// eUSCI_Ax Control Word Register 1
    rw UCACTLW1 @ 0x02: u16 = 0_0 {
        /// Deglitch time
        UCGLIT: 0..1 = enum UCGLIT {
            /// Approximately 2 ns (equivalent of 1 delay element)
            UCGLIT_0 = 0b00,
            /// Approximately 50 ns
            UCGLIT_1 = 0b01,
            /// Approximately 100 ns
            UCGLIT_2 = 0b10,
            /// Approximately 200 ns
            UCGLIT_3 = 0b11,
        }
    }
    /// eUSCI_Ax Baud Rate Control Word Register
    rw UCABRW @ 0x06: u16 = 0_0 {
        /// eUSCI_Ax Baud Rate Control Word Register
        UCABRW: 0..15 = struct UCABRWField(u16);
    }
    /// eUSCI_Ax Bit Rate Control Register 1
    rw UCABRW_SPI @ 0x06: u16 = 0_0 {
        /// eUSCI_Ax Bit Rate Control Register 1
        UCABRW_SPI: 0..15 = struct UCABRW_SPIField(u16);
    }
    /// eUSCI_Ax Modulation Control Word Register
    rw UCAMCTLW @ 0x08: u16 = 0_0 {
        /// Oversampling mode enabled
        UCOS16: 0 = enum UCOS16 {
            /// Disabled
            UCOS16_0 = 0b0,
            /// Enabled
            UCOS16_1 = 0b1,
        }
        /// First modulation stage select
        UCBRF: 4..7 = struct UCBRF(u16);
        /// Second modulation stage select
        UCBRS: 8..15 = struct UCBRS(u16);
    }
    /// eUSCI_Ax Status Register
    rw UCASTATW @ 0x0a: u16 = 0_0 {
        /// eUSCI_A busy
        UCBUSY: 0 = enum UCBUSY {
            /// eUSCI_A inactive
            IDLE = 0b0,
            /// eUSCI_A transmitting or receiving
            BUSY = 0b1,
        }
        /// Address received / Idle line detected
        UCADDR_UCIDLE: 1 = enum UCADDR_UCIDLE {
            /// UCADDR: Received character is data. UCIDLE: No idle line detected
            UCADDR_UCIDLE_0 = 0b0,
            /// UCADDR: Received character is an address. UCIDLE: Idle line detected
            UCADDR_UCIDLE_1 = 0b1,
        }
        /// Receive error flag
        UCRXERR: 2 = enum UCRXERR {
            /// No receive errors detected
            UCRXERR_0 = 0b0,
            /// Receive error detected
            UCRXERR_1 = 0b1,
        }
        /// Break detect flag
        UCBRK: 3 = enum UCBRK {
            /// No break condition
            UCBRK_0 = 0b0,
            /// Break condition occurred
            UCBRK_1 = 0b1,
        }
        /// Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read.
        UCPE: 4 = enum UCPE {
            /// No error
            UCPE_0 = 0b0,
            /// Character received with parity error
            UCPE_1 = 0b1,
        }
        /// Overrun error flag
        UCASTATW_UCOE: 5 = enum UCASTATW_UCOE {
            /// No error
            UCOE_0 = 0b0,
            /// Overrun error occurred
            UCOE_1 = 0b1,
        }
        /// Framing error flag
        UCASTATW_UCFE: 6 = enum UCASTATW_UCFE {
            /// No error
            UCFE_0 = 0b0,
            /// Character received with low stop bit
            UCFE_1 = 0b1,
        }
        /// Listen enable
        UCASTATW_UCLISTEN: 7 = enum UCASTATW_UCLISTEN {
            /// Disabled
            UCLISTEN_0 = 0b0,
            /// Enabled. UCAxTXD is internally fed back to the receiver
            UCLISTEN_1 = 0b1,
        }
    }
    /// UCASTATW_SPI
    rw UCASTATW_SPI @ 0x0a: u16 = 0_0 {
        /// Overrun error flag
        UCASTATW_SPI_UCOE: 5 = enum UCASTATW_SPI_UCOE {
            /// No error
            UCOE_0 = 0b0,
            /// Overrun error occurred
            UCOE_1 = 0b1,
        }
        /// Framing error flag
        UCASTATW_SPI_UCFE: 6 = enum UCASTATW_SPI_UCFE {
            /// No error
            UCFE_0 = 0b0,
            /// Bus conflict occurred
            UCFE_1 = 0b1,
        }
        /// Listen enable
        UCASTATW_SPI_UCLISTEN: 7 = enum UCASTATW_SPI_UCLISTEN {
            /// Disabled
            UCLISTEN_0 = 0b0,
            /// Enabled. UCAxTXD is internally fed back to the receiver
            UCLISTEN_1 = 0b1,
        }
    }
    /// eUSCI_Ax Receive Buffer Register
    r UCARXBUF @ 0x0c: u16 = 0_0 {
        /// Receive data buffer
        UCARXBUF_UCRXBUF: 0..7 = struct UCARXBUF_UCRXBUF(u16);
    }
    /// eUSCI_Ax Receive Buffer Register
    r UCARXBUF_SPI @ 0x0c: u16 = 0_0 {
        /// Receive data buffer
        UCARXBUF_SPI_UCRXBUF: 0..7 = struct UCARXBUF_SPI_UCRXBUF(u16);
    }
    /// eUSCI_Ax Transmit Buffer Register
    rw UCATXBUF @ 0x0e: u16 = 0_0 {
        /// Transmit data buffer
        UCATXBUF_UCTXBUF: 0..7 = struct UCATXBUF_UCTXBUF(u16);
    }
    /// eUSCI_Ax Transmit Buffer Register
    rw UCATXBUF_SPI @ 0x0e: u16 = 0_0 {
        /// Transmit data buffer
        UCATXBUF_SPI_UCTXBUF: 0..7 = struct UCATXBUF_SPI_UCTXBUF(u16);
    }
    /// eUSCI_Ax Auto Baud Rate Control Register
    rw UCAABCTL @ 0x10: u16 = 0_0 {
        /// Automatic baud-rate detect enable
        UCABDEN: 0 = enum UCABDEN {
            /// Baud-rate detection disabled. Length of break and synch field is not measured.
            UCABDEN_0 = 0b0,
            /// Baud-rate detection enabled. Length of break and synch field is measured and baud-rate settings are changed accordingly.
            UCABDEN_1 = 0b1,
        }
        /// Break time out error
        UCBTOE: 2 = enum UCBTOE {
            /// No error
            UCBTOE_0 = 0b0,
            /// Length of break field exceeded 22 bit times
            UCBTOE_1 = 0b1,
        }
        /// Synch field time out error
        UCSTOE: 3 = enum UCSTOE {
            /// No error
            UCSTOE_0 = 0b0,
            /// Length of synch field exceeded measurable time
            UCSTOE_1 = 0b1,
        }
        /// Break/synch delimiter length
        UCDELIM: 4..5 = enum UCDELIM {
            /// 1 bit time
            UCDELIM_0 = 0b00,
            /// 2 bit times
            UCDELIM_1 = 0b01,
            /// 3 bit times
            UCDELIM_2 = 0b10,
            /// 4 bit times
            UCDELIM_3 = 0b11,
        }
    }
    /// eUSCI_Ax IrDA Control Word Register
    rw UCAIRCTL @ 0x12: u16 = 0_0 {
        /// IrDA encoder/decoder enable
        UCIREN: 0 = enum UCIREN {
            /// IrDA encoder/decoder disabled
            UCIREN_0 = 0b0,
            /// IrDA encoder/decoder enabled
            UCIREN_1 = 0b1,
        }
        /// IrDA transmit pulse clock select
        UCIRTXCLK: 1 = enum UCIRTXCLK {
            /// BRCLK
            UCIRTXCLK_0 = 0b0,
            /// BITCLK16 when UCOS16 = 1. Otherwise, BRCLK.
            UCIRTXCLK_1 = 0b1,
        }
        /// Transmit pulse length
        UCIRTXPL: 2..7 = struct UCIRTXPL(u16);
        /// IrDA receive filter enabled
        UCIRRXFE: 8 = enum UCIRRXFE {
            /// Receive filter disabled
            UCIRRXFE_0 = 0b0,
            /// Receive filter enabled
            UCIRRXFE_1 = 0b1,
        }
        /// IrDA receive input UCAxRXD polarity
        UCIRRXPL: 9 = enum UCIRRXPL {
            /// IrDA transceiver delivers a high pulse when a light pulse is seen
            HIGH = 0b0,
            /// IrDA transceiver delivers a low pulse when a light pulse is seen
            LOW = 0b1,
        }
        /// Receive filter length
        UCIRRXFL: 10..15 = struct UCIRRXFL(u16);
    }
    /// eUSCI_Ax Interrupt Enable Register
    rw UCAIE @ 0x1a: u16 = 0_0 {
        /// Receive interrupt enable
        UCAIE_UCRXIE: 0 = enum UCAIE_UCRXIE {
            /// Interrupt disabled
            UCRXIE_0 = 0b0,
            /// Interrupt enabled
            UCRXIE_1 = 0b1,
        }
        /// Transmit interrupt enable
        UCAIE_UCTXIE: 1 = enum UCAIE_UCTXIE {
            /// Interrupt disabled
            UCTXIE_0 = 0b0,
            /// Interrupt enabled
            UCTXIE_1 = 0b1,
        }
        /// Start bit interrupt enable
        UCSTTIE: 2 = enum UCSTTIE {
            /// Interrupt disabled
            UCSTTIE_0 = 0b0,
            /// Interrupt enabled
            UCSTTIE_1 = 0b1,
        }
        /// Transmit complete interrupt enable
        UCTXCPTIE: 3 = enum UCTXCPTIE {
            /// Interrupt disabled
            UCTXCPTIE_0 = 0b0,
            /// Interrupt enabled
            UCTXCPTIE_1 = 0b1,
        }
    }
    /// eUSCI_Ax Interrupt Enable Register
    rw UCAIE_SPI @ 0x1a: u16 = 0_0 {
        /// Receive interrupt enable
        UCAIE_SPI_UCRXIE: 0 = enum UCAIE_SPI_UCRXIE {
            /// Interrupt disabled
            UCRXIE_0 = 0b0,
            /// Interrupt enabled
            UCRXIE_1 = 0b1,
        }
        /// Transmit interrupt enable
        UCAIE_SPI_UCTXIE: 1 = enum UCAIE_SPI_UCTXIE {
            /// Interrupt disabled
            UCTXIE_0 = 0b0,
            /// Interrupt enabled
            UCTXIE_1 = 0b1,
        }
    }
    /// eUSCI_Ax Interrupt Flag Register
    rw UCAIFG @ 0x1c: u16 = 0_0 {
        /// Receive interrupt flag
        UCAIFG_UCRXIFG: 0 = enum UCAIFG_UCRXIFG {
            /// No interrupt pending
            UCRXIFG_0 = 0b0,
            /// Interrupt pending
            UCRXIFG_1 = 0b1,
        }
        /// Transmit interrupt flag
        UCAIFG_UCTXIFG: 1 = enum UCAIFG_UCTXIFG {
            /// No interrupt pending
            UCTXIFG_0 = 0b0,
            /// Interrupt pending
            UCTXIFG_1 = 0b1,
        }
        /// Start bit interrupt flag
        UCSTTIFG: 2 = enum UCSTTIFG {
            /// No interrupt pending
            UCSTTIFG_0 = 0b0,
            /// Interrupt pending
            UCSTTIFG_1 = 0b1,
        }
        /// Transmit ready interrupt enable
        UCTXCPTIFG: 3 = enum UCTXCPTIFG {
            /// No interrupt pending
            UCTXCPTIFG_0 = 0b0,
            /// Interrupt pending
            UCTXCPTIFG_1 = 0b1,
        }
    }
    /// eUSCI_Ax Interrupt Flag Register
    rw UCAIFG_SPI @ 0x1c: u16 = 0_0 {
        /// Receive interrupt flag
        UCAIFG_SPI_UCRXIFG: 0 = enum UCAIFG_SPI_UCRXIFG {
            /// No interrupt pending
            UCRXIFG_0 = 0b0,
            /// Interrupt pending
            UCRXIFG_1 = 0b1,
        }
        /// Transmit interrupt flag
        UCAIFG_SPI_UCTXIFG: 1 = enum UCAIFG_SPI_UCTXIFG {
            /// No interrupt pending
            UCTXIFG_0 = 0b0,
            /// Interrupt pending
            UCTXIFG_1 = 0b1,
        }
    }
    /// eUSCI_Ax Interrupt Vector Register
    r UCAIV @ 0x1e: u16 = 0_0 {
        /// eUSCI_A interrupt vector value
        UCAIV_UCIV: 0..15 = enum UCAIV_UCIV {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest
            UCRXIFG = 0b0000000000000010,
            /// Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG
            UCTXIFG = 0b0000000000000100,
            /// Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG
            UCSTTIFG = 0b0000000000000110,
            /// Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest
            UCTXCPTIFG = 0b0000000000001000,
        }
    }
    /// eUSCI_Ax Interrupt Vector Register
    r UCAIV_SPI @ 0x1e: u16 = 0_0 {
        /// eUSCI_A interrupt vector value
        UCAIV_SPI_UCIV: 0..15 = enum UCAIV_SPI_UCIV {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest
            UCRXIFG = 0b0000000000000010,
            /// Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest
            UCTXIFG = 0b0000000000000100,
        }
    }
}
