//! USCI_A0  SPI Mode

utils::periph! {
    /// USCI_A0  SPI Mode
    USCI_A0SPIMode;
    /// USCI A0 Control Register 0
    rw UCA0CTL0__SPI @ 0x00: u8 = 0_0 {
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
    /// USCI A0 Control Register 1
    rw UCA0CTL1__SPI @ 0x01: u8 = 0_0 {
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
    /// USCI A0 Baud Rate 0
    rw UCA0BR0__SPI @ 0x02: u8 = 0_0 {
        /// USCI A0 Baud Rate 0
        UCA0BR0__SPI: 0..7 = struct UCA0BR0__SPIField(u8);
    }
    /// USCI A0 Baud Rate 1
    rw UCA0BR1__SPI @ 0x03: u8 = 0_0 {
        /// USCI A0 Baud Rate 1
        UCA0BR1__SPI: 0..7 = struct UCA0BR1__SPIField(u8);
    }
    /// USCI A0 Modulation Control
    rw UCA0MCTL__SPI @ 0x04: u8 = 0_0 {
        /// USCI A0 Modulation Control
        UCA0MCTL__SPI: 0..7 = struct UCA0MCTL__SPIField(u8);
    }
    /// USCI A0 Status Register
    rw UCA0STAT__SPI @ 0x05: u8 = 0_0 {
        /// USCI Busy Flag
        UCBUSY: 0 = struct UCBUSY(bool);
        /// USCI Overrun Error Flag
        UCOE: 5 = struct UCOE(bool);
        /// USCI Frame Error Flag
        UCFE: 6 = struct UCFE(bool);
        /// USCI Listen mode
        UCLISTEN: 7 = struct UCLISTEN(bool);
    }
    /// USCI A0 Receive Buffer
    rw UCA0RXBUF__SPI @ 0x06: u8 = 0_0 {
        /// USCI A0 Receive Buffer
        UCA0RXBUF__SPI: 0..7 = struct UCA0RXBUF__SPIField(u8);
    }
    /// USCI A0 Transmit Buffer
    rw UCA0TXBUF__SPI @ 0x07: u8 = 0_0 {
        /// USCI A0 Transmit Buffer
        UCA0TXBUF__SPI: 0..7 = struct UCA0TXBUF__SPIField(u8);
    }
}
