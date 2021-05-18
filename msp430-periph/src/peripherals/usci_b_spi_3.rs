//! USCI_B  SPI Mode

utils::periph! {
    /// USCI_B  SPI Mode
    USCI_B_SPI;
    /// USCI B Control Word Register 0
    rw UCBCTLW0__SPI @ 0x00: u16 = 0_0 {
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
    /// USCI B Control Register 0
    rw UCBCTL0__SPI @ 0x01: u8 = 0_0 {
        /// USCI B Control Register 0
        UCBCTL0__SPI: 0..7 = struct UCBCTL0__SPIField(u8);
    }
    /// USCI B Control Register 1
    rw UCBCTL1__SPI @ 0x00: u8 = 0_0 {
        /// USCI B Control Register 1
        UCBCTL1__SPI: 0..7 = struct UCBCTL1__SPIField(u8);
    }
    /// USCI B Baud Word Rate 0
    rw UCBBRW__SPI @ 0x06: u16 = 0_0 {
        /// USCI B Baud Word Rate 0
        UCBBRW__SPI: 0..15 = struct UCBBRW__SPIField(u16);
    }
    /// USCI B Baud Rate 0
    rw UCBBR0__SPI @ 0x06: u8 = 0_0 {
        /// USCI B Baud Rate 0
        UCBBR0__SPI: 0..7 = struct UCBBR0__SPIField(u8);
    }
    /// USCI B Baud Rate 1
    rw UCBBR1__SPI @ 0x07: u8 = 0_0 {
        /// USCI B Baud Rate 1
        UCBBR1__SPI: 0..7 = struct UCBBR1__SPIField(u8);
    }
    /// USCI B Status Register
    rw UCBSTATW__SPI @ 0x08: u16 = 0_0 {
        /// USCI Busy Flag
        UCBUSY: 0 = struct UCBUSY(bool);
        /// USCI Overrun Error Flag
        UCOE: 5 = struct UCOE(bool);
        /// USCI Frame Error Flag
        UCFE: 6 = struct UCFE(bool);
        /// USCI Listen mode
        UCLISTEN: 7 = struct UCLISTEN(bool);
    }
    /// USCI B Receive Buffer
    rw UCBRXBUF__SPI @ 0x0c: u16 = 0_0 {
        /// USCI B Receive Buffer
        UCBRXBUF__SPI: 0..15 = struct UCBRXBUF__SPIField(u16);
    }
    /// USCI B Transmit Buffer
    rw UCBTXBUF__SPI @ 0x0e: u16 = 0_0 {
        /// USCI B Transmit Buffer
        UCBTXBUF__SPI: 0..15 = struct UCBTXBUF__SPIField(u16);
    }
    /// USCI B Interrupt Enable Register
    rw UCBIE__SPI @ 0x2a: u16 = 0_0 {
        /// USCI Receive Interrupt Enable
        UCRXIE: 0 = struct UCRXIE(bool);
        /// USCI Transmit Interrupt Enable
        UCTXIE: 1 = struct UCTXIE(bool);
    }
    /// USCI B Interrupt Flags Register
    rw UCBIFG__SPI @ 0x2c: u16 = 0_0 {
        /// SPI Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// SPI Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
    }
    /// USCI B Interrupt Vector Register
    rw UCBIV__SPI @ 0x2e: u16 = 0_0 {
        /// USCI B Interrupt Vector Register
        UCBIV__SPI: 0..15 = struct UCBIV__SPIField(u16);
    }
}
