//! USCI_A3  SPI Mode

utils::periph! {
    /// USCI_A3  SPI Mode
    USCI_A3SPIMode;
    /// USCI A3 Control Word Register 0
    rw UCA3CTLW0__SPI @ 0x00: u16 = 0_0 {
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
    /// USCI A3 Control Register 0
    rw UCA3CTL0__SPI @ 0x01: u8 = 0_0 {
        /// USCI A3 Control Register 0
        UCA3CTL0__SPI: 0..7 = struct UCA3CTL0__SPIField(u8);
    }
    /// USCI A3 Control Register 1
    rw UCA3CTL1__SPI @ 0x00: u8 = 0_0 {
        /// USCI A3 Control Register 1
        UCA3CTL1__SPI: 0..7 = struct UCA3CTL1__SPIField(u8);
    }
    /// USCI A3 Baud Word Rate 0
    rw UCA3BRW__SPI @ 0x06: u16 = 0_0 {
        /// USCI A3 Baud Word Rate 0
        UCA3BRW__SPI: 0..15 = struct UCA3BRW__SPIField(u16);
    }
    /// USCI A3 Baud Rate 0
    rw UCA3BR0__SPI @ 0x06: u8 = 0_0 {
        /// USCI A3 Baud Rate 0
        UCA3BR0__SPI: 0..7 = struct UCA3BR0__SPIField(u8);
    }
    /// USCI A3 Baud Rate 1
    rw UCA3BR1__SPI @ 0x07: u8 = 0_0 {
        /// USCI A3 Baud Rate 1
        UCA3BR1__SPI: 0..7 = struct UCA3BR1__SPIField(u8);
    }
    /// USCI A3 Status Register
    rw UCA3STATW__SPI @ 0x0a: u8 = 0_0 {
        /// USCI Busy Flag
        UCBUSY: 0 = struct UCBUSY(bool);
        /// USCI Overrun Error Flag
        UCOE: 5 = struct UCOE(bool);
        /// USCI Frame Error Flag
        UCFE: 6 = struct UCFE(bool);
        /// USCI Listen mode
        UCLISTEN: 7 = struct UCLISTEN(bool);
    }
    /// USCI A3 Receive Buffer
    rw UCA3RXBUF__SPI @ 0x0c: u16 = 0_0 {
        /// USCI A3 Receive Buffer
        UCA3RXBUF__SPI: 0..15 = struct UCA3RXBUF__SPIField(u16);
    }
    /// USCI A3 Transmit Buffer
    rw UCA3TXBUF__SPI @ 0x0e: u16 = 0_0 {
        /// USCI A3 Transmit Buffer
        UCA3TXBUF__SPI: 0..15 = struct UCA3TXBUF__SPIField(u16);
    }
    /// USCI A3 Interrupt Enable Register
    rw UCA3IE__SPI @ 0x1a: u8 = 0_0 {
        /// USCI Receive Interrupt Enable
        UCRXIE: 0 = struct UCRXIE(bool);
        /// USCI Transmit Interrupt Enable
        UCTXIE: 1 = struct UCTXIE(bool);
    }
    /// USCI A3 Interrupt Flags Register
    rw UCA3IFG__SPI @ 0x1c: u8 = 0_0 {
        /// SPI Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// SPI Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
    }
    /// USCI A3 Interrupt Vector Register
    rw UCA3IV__SPI @ 0x1e: u16 = 0_0 {
        /// USCI A3 Interrupt Vector Register
        UCA3IV__SPI: 0..15 = struct UCA3IV__SPIField(u16);
    }
}
