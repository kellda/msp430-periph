//! USCI_A  SPI Mode

utils::periph! {
    /// USCI_A  SPI Mode
    USCI_A_SPI;
    /// USCI A Control Word Register 0
    rw UCACTLW0__SPI @ 0x00: u16 = 0_0 {
        /// USCI A Control Word Register 0
        UCACTLW0__SPI: 0..15 = struct UCA0CTLW0__SPIField(u16);
    }
    /// USCI A Control Register 0
    rw UCACTL0__SPI @ 0x01: u8 = 0_0 {
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
    /// USCI A Control Register 1
    rw UCACTL1__SPI @ 0x00: u8 = 0_0 {
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
    /// USCI A Baud Word Rate 0
    rw UCABRW__SPI @ 0x06: u16 = 0_0 {
        /// USCI A Baud Word Rate 0
        UCABRW__SPI: 0..15 = struct UCA0BRW__SPIField(u16);
    }
    /// USCI A Baud Rate 0
    rw UCABR0__SPI @ 0x06: u8 = 0_0 {
        /// USCI A Baud Rate 0
        UCABR0__SPI: 0..7 = struct UCA0BR0__SPIField(u8);
    }
    /// USCI A Baud Rate 1
    rw UCABR1__SPI @ 0x07: u8 = 0_0 {
        /// USCI A Baud Rate 1
        UCABR1__SPI: 0..7 = struct UCA0BR1__SPIField(u8);
    }
    /// USCI A Modulation Control
    rw UCAMCTL__SPI @ 0x08: u8 = 0_0 {
        /// USCI A Modulation Control
        UCAMCTL__SPI: 0..7 = struct UCA0MCTL__SPIField(u8);
    }
    /// USCI A Status Register
    rw UCASTAT__SPI @ 0x0a: u8 = 0_0 {
        /// USCI Busy Flag
        UCBUSY: 0 = struct UCBUSY(bool);
        /// USCI Overrun Error Flag
        UCOE: 5 = struct UCOE(bool);
        /// USCI Frame Error Flag
        UCFE: 6 = struct UCFE(bool);
        /// USCI Listen mode
        UCLISTEN: 7 = struct UCLISTEN(bool);
    }
    /// USCI A Receive Buffer
    rw UCARXBUF__SPI @ 0x0c: u8 = 0_0 {
        /// USCI A Receive Buffer
        UCARXBUF__SPI: 0..7 = struct UCA0RXBUF__SPIField(u8);
    }
    /// USCI A Transmit Buffer
    rw UCATXBUF__SPI @ 0x0e: u8 = 0_0 {
        /// USCI A Transmit Buffer
        UCATXBUF__SPI: 0..7 = struct UCA0TXBUF__SPIField(u8);
    }
    /// USCI A Interrupt Enable Register
    rw UCAICTL__SPI @ 0x1c: u16 = 0_0 {
        /// USCI A Interrupt Enable Register
        UCAICTL__SPI: 0..15 = struct UCA0ICTL__SPIField(u16);
    }
    /// USCI A Interrupt Enable Register
    rw UCAIE__SPI @ 0x1c: u8 = 0_0 {
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
    /// USCI A Interrupt Flags Register
    rw UCAIFG__SPI @ 0x1d: u8 = 0_0 {
        /// USCI Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// USCI Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
    }
    /// USCI A Interrupt Vector Register
    rw UCAIV__SPI @ 0x1e: u16 = 0_0 {
        /// USCI A Interrupt Vector Register
        UCAIV__SPI: 0..15 = struct UCA0IV__SPIField(u16);
    }
}