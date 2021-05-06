//! USCI_B3  SPI Mode

utils::periph! {
    /// USCI_B3  SPI Mode
    USCI_B3SPIMode;
    /// USCI B3 Control Word Register 0
    rw UCB3CTLW0__SPI @ 0x00: u16 = 0_0 {
        /// USCI B3 Control Word Register 0
        UCB3CTLW0__SPI: 0..15 = struct UCB3CTLW0__SPIField(u16);
    }
    /// USCI B3 Control Register 0
    rw UCB3CTL0__SPI @ 0x01: u8 = 0_0 {
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
        /// Sync. Mode: Data Bits 0:8-bits / 1:7-bits
        UC7BIT: 4 = struct UC7BIT(bool);
        /// Sync. Mode: MSB first 0:LSB / 1:MSB
        UCMSB: 5 = struct UCMSB(bool);
        /// Sync. Mode: Clock Polarity
        UCCKPL: 6 = struct UCCKPL(bool);
        /// Sync. Mode: Clock Phase
        UCCKPH: 7 = struct UCCKPH(bool);
    }
    /// USCI B3 Control Register 1
    rw UCB3CTL1__SPI @ 0x00: u8 = 0_0 {
        /// USCI Software Reset
        UCSWRST: 0 = struct UCSWRST(bool);
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
    /// USCI B3 Baud Word Rate 0
    rw UCB3BRW__SPI @ 0x06: u16 = 0_0 {
        /// USCI B3 Baud Word Rate 0
        UCB3BRW__SPI: 0..15 = struct UCB3BRW__SPIField(u16);
    }
    /// USCI B3 Baud Rate 0
    rw UCB3BR0__SPI @ 0x06: u8 = 0_0 {
        /// USCI B3 Baud Rate 0
        UCB3BR0__SPI: 0..7 = struct UCB3BR0__SPIField(u8);
    }
    /// USCI B3 Baud Rate 1
    rw UCB3BR1__SPI @ 0x07: u8 = 0_0 {
        /// USCI B3 Baud Rate 1
        UCB3BR1__SPI: 0..7 = struct UCB3BR1__SPIField(u8);
    }
    /// USCI B3 Status Register
    rw UCB3STAT__SPI @ 0x0a: u8 = 0_0 {
        /// USCI Busy Flag
        UCBUSY: 0 = struct UCBUSY(bool);
        /// USCI Overrun Error Flag
        UCOE: 5 = struct UCOE(bool);
        /// USCI Frame Error Flag
        UCFE: 6 = struct UCFE(bool);
        /// USCI Listen mode
        UCLISTEN: 7 = struct UCLISTEN(bool);
    }
    /// USCI B3 Receive Buffer
    rw UCB3RXBUF__SPI @ 0x0c: u8 = 0_0 {
        /// USCI B3 Receive Buffer
        UCB3RXBUF__SPI: 0..7 = struct UCB3RXBUF__SPIField(u8);
    }
    /// USCI B3 Transmit Buffer
    rw UCB3TXBUF__SPI @ 0x0e: u8 = 0_0 {
        /// USCI B3 Transmit Buffer
        UCB3TXBUF__SPI: 0..7 = struct UCB3TXBUF__SPIField(u8);
    }
    /// USCI B3 Interrupt Enable Register
    rw UCB3ICTL__SPI @ 0x1c: u16 = 0_0 {
        /// USCI B3 Interrupt Enable Register
        UCB3ICTL__SPI: 0..15 = struct UCB3ICTL__SPIField(u16);
    }
    /// USCI B3 Interrupt Enable Register
    rw UCB3IE__SPI @ 0x1c: u8 = 0_0 {
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
    /// USCI B3 Interrupt Flags Register
    rw UCB3IFG__SPI @ 0x1d: u8 = 0_0 {
        /// USCI Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// USCI Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
    }
    /// USCI B3 Interrupt Vector Register
    rw UCB3IV__SPI @ 0x1e: u16 = 0_0 {
        /// USCI B3 Interrupt Vector Register
        UCB3IV__SPI: 0..15 = struct UCB3IV__SPIField(u16);
    }
}
