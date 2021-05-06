//! USCI_B1  SPI Mode

utils::periph! {
    /// USCI_B1  SPI Mode
    USCI_B1SPIMode;
    /// USCI B1 Control Word Register 0
    rw UCB1CTLW0__SPI @ 0x00: u16 = 0_0 {
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
    /// USCI B1 Control Register 0
    rw UCB1CTL0__SPI @ 0x01: u8 = 0_0 {
        /// USCI B1 Control Register 0
        UCB1CTL0__SPI: 0..7 = struct UCB1CTL0__SPIField(u8);
    }
    /// USCI B1 Control Register 1
    rw UCB1CTL1__SPI @ 0x00: u8 = 0_0 {
        /// USCI B1 Control Register 1
        UCB1CTL1__SPI: 0..7 = struct UCB1CTL1__SPIField(u8);
    }
    /// USCI B1 Baud Word Rate 0
    rw UCB1BRW__SPI @ 0x06: u16 = 0_0 {
        /// USCI B1 Baud Word Rate 0
        UCB1BRW__SPI: 0..15 = struct UCB1BRW__SPIField(u16);
    }
    /// USCI B1 Baud Rate 0
    rw UCB1BR0__SPI @ 0x06: u8 = 0_0 {
        /// USCI B1 Baud Rate 0
        UCB1BR0__SPI: 0..7 = struct UCB1BR0__SPIField(u8);
    }
    /// USCI B1 Baud Rate 1
    rw UCB1BR1__SPI @ 0x07: u8 = 0_0 {
        /// USCI B1 Baud Rate 1
        UCB1BR1__SPI: 0..7 = struct UCB1BR1__SPIField(u8);
    }
    /// USCI B1 Status Register
    rw UCB1STATW__SPI @ 0x08: u16 = 0_0 {
        /// USCI Busy Flag
        UCBUSY: 0 = struct UCBUSY(bool);
        /// USCI Overrun Error Flag
        UCOE: 5 = struct UCOE(bool);
        /// USCI Frame Error Flag
        UCFE: 6 = struct UCFE(bool);
        /// USCI Listen mode
        UCLISTEN: 7 = struct UCLISTEN(bool);
    }
    /// USCI B1 Receive Buffer
    rw UCB1RXBUF__SPI @ 0x0c: u16 = 0_0 {
        /// USCI B1 Receive Buffer
        UCB1RXBUF__SPI: 0..15 = struct UCB1RXBUF__SPIField(u16);
    }
    /// USCI B1 Transmit Buffer
    rw UCB1TXBUF__SPI @ 0x0e: u16 = 0_0 {
        /// USCI B1 Transmit Buffer
        UCB1TXBUF__SPI: 0..15 = struct UCB1TXBUF__SPIField(u16);
    }
    /// USCI B1 Interrupt Enable Register
    rw UCB1IE__SPI @ 0x2a: u16 = 0_0 {
        /// USCI Receive Interrupt Enable
        UCRXIE: 0 = struct UCRXIE(bool);
        /// USCI Transmit Interrupt Enable
        UCTXIE: 1 = struct UCTXIE(bool);
    }
    /// USCI B1 Interrupt Flags Register
    rw UCB1IFG__SPI @ 0x2c: u16 = 0_0 {
        /// SPI Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// SPI Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
    }
    /// USCI B1 Interrupt Vector Register
    rw UCB1IV__SPI @ 0x2e: u16 = 0_0 {
        /// USCI B1 Interrupt Vector Register
        UCB1IV__SPI: 0..15 = struct UCB1IV__SPIField(u16);
    }
}
