//! USCI_B0  SPI Mode

utils::periph! {
    /// USCI_B0  SPI Mode
    USCI_B0SPIMode;
    /// USCI B0 Control Word Register 0
    rw UCB0CTLW0__SPI @ 0x00: u16 = 0_0 {
        /// USCI Software Reset
        UCSWRST: 0 = struct UCSWRST(bool);
        /// USCI STE Mode
        UCSTEM: 1 = struct UCSTEM(bool);
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
        /// Sync-Mode  0:UART-Mode / 1:SPI-Mode
        UCSYNC: 8 = struct UCSYNC(bool);
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
        /// Sync. Mode: Data Bits 0:8-bits / 1:7-bits
        UC7BIT: 12 = struct UC7BIT(bool);
        /// Sync. Mode: MSB first 0:LSB / 1:MSB
        UCMSB: 13 = struct UCMSB(bool);
        /// Sync. Mode: Clock Polarity
        UCCKPL: 14 = struct UCCKPL(bool);
        /// Sync. Mode: Clock Phase
        UCCKPH: 15 = struct UCCKPH(bool);
    }
    /// USCI B0 Control Register 0
    rw UCB0CTL0__SPI @ 0x01: u8 = 0_0 {
        /// USCI B0 Control Register 0
        UCB0CTL0__SPI: 0..7 = struct UCB0CTL0__SPIField(u8);
    }
    /// USCI B0 Control Register 1
    rw UCB0CTL1__SPI @ 0x00: u8 = 0_0 {
        /// USCI B0 Control Register 1
        UCB0CTL1__SPI: 0..7 = struct UCB0CTL1__SPIField(u8);
    }
    /// USCI B0 Baud Word Rate 0
    rw UCB0BRW__SPI @ 0x06: u16 = 0_0 {
        /// USCI B0 Baud Word Rate 0
        UCB0BRW__SPI: 0..15 = struct UCB0BRW__SPIField(u16);
    }
    /// USCI B0 Baud Rate 0
    rw UCB0BR0__SPI @ 0x06: u8 = 0_0 {
        /// USCI B0 Baud Rate 0
        UCB0BR0__SPI: 0..7 = struct UCB0BR0__SPIField(u8);
    }
    /// USCI B0 Baud Rate 1
    rw UCB0BR1__SPI @ 0x07: u8 = 0_0 {
        /// USCI B0 Baud Rate 1
        UCB0BR1__SPI: 0..7 = struct UCB0BR1__SPIField(u8);
    }
    /// USCI B0 Status Register
    rw UCB0STATW__SPI @ 0x08: u16 = 0_0 {
        /// USCI Busy Flag
        UCBUSY: 0 = struct UCBUSY(bool);
        /// USCI Overrun Error Flag
        UCOE: 5 = struct UCOE(bool);
        /// USCI Frame Error Flag
        UCFE: 6 = struct UCFE(bool);
        /// USCI Listen mode
        UCLISTEN: 7 = struct UCLISTEN(bool);
    }
    /// USCI B0 Receive Buffer
    rw UCB0RXBUF__SPI @ 0x0c: u16 = 0_0 {
        /// USCI B0 Receive Buffer
        UCB0RXBUF__SPI: 0..15 = struct UCB0RXBUF__SPIField(u16);
    }
    /// USCI B0 Transmit Buffer
    rw UCB0TXBUF__SPI @ 0x0e: u16 = 0_0 {
        /// USCI B0 Transmit Buffer
        UCB0TXBUF__SPI: 0..15 = struct UCB0TXBUF__SPIField(u16);
    }
    /// USCI B0 Interrupt Enable Register
    rw UCB0IE__SPI @ 0x2a: u16 = 0_0 {
        /// USCI Receive Interrupt Enable
        UCRXIE: 0 = struct UCRXIE(bool);
        /// USCI Transmit Interrupt Enable
        UCTXIE: 1 = struct UCTXIE(bool);
    }
    /// USCI B0 Interrupt Flags Register
    rw UCB0IFG__SPI @ 0x2c: u16 = 0_0 {
        /// SPI Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// SPI Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
    }
    /// USCI B0 Interrupt Vector Register
    rw UCB0IV__SPI @ 0x2e: u16 = 0_0 {
        /// USCI B0 Interrupt Vector Register
        UCB0IV__SPI: 0..15 = struct UCB0IV__SPIField(u16);
    }
}
