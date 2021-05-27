//! USCI_B  SPI Mode

utils::periph! {
    /// USCI_B  SPI Mode
    USCI_B_SPI;
    /// USCI B Control Register 0
    rw CTL0 @ 0x00: u8 = 0_0 {
        /// Sync-Mode  0:UART-Mode / 1:SPI-Mode
        SYNC: 0 = struct SYNC(bool);
        /// Sync. Mode: USCI Mode 1
        MODE: 1..2 = enum MODE {
            /// Sync. Mode: USCI Mode: 0
            MODE_0 = 0b00,
            /// Sync. Mode: USCI Mode: 1
            MODE_1 = 0b01,
            /// Sync. Mode: USCI Mode: 2
            MODE_2 = 0b10,
            /// Sync. Mode: USCI Mode: 3
            MODE_3 = 0b11,
        }
        /// Sync. Mode: Master Select
        MST: 3 = struct MST(bool);
        /// Sync. Mode: Data Bits 0:8-bits / 1:7-bits
        _7BIT: 4 = struct _7BIT(bool);
        /// Sync. Mode: MSB first 0:LSB / 1:MSB
        MSB: 5 = struct MSB(bool);
        /// Sync. Mode: Clock Polarity
        CKPL: 6 = struct CKPL(bool);
        /// Sync. Mode: Clock Phase
        CKPH: 7 = struct CKPH(bool);
    }
    /// USCI B Control Register 1
    rw CTL1 @ 0x01: u8 = 0_0 {
        /// USCI Software Reset
        SWRST: 0 = struct SWRST(bool);
        /// USCI 1 Clock Source Select 1
        SSEL: 6..7 = enum SSEL {
            /// USCI 0 Clock Source: 0
            SSEL_0 = 0b00,
            /// USCI 0 Clock Source: 1
            SSEL_1 = 0b01,
            /// USCI 0 Clock Source: 2
            SSEL_2 = 0b10,
            /// USCI 0 Clock Source: 3
            SSEL_3 = 0b11,
        }
    }
    /// USCI B Baud Rate 0
    rw BR0 @ 0x02: u8 = 0_0 {
        /// USCI B Baud Rate 0
        BR0: 0..7 = struct BR0Field(u8);
    }
    /// USCI B Baud Rate 1
    rw BR1 @ 0x03: u8 = 0_0 {
        /// USCI B Baud Rate 1
        BR1: 0..7 = struct BR1Field(u8);
    }
    /// USCI B Status Register
    rw STAT @ 0x05: u8 = 0_0 {
        /// USCI Busy Flag
        BUSY: 0 = struct BUSY(bool);
        /// USCI Overrun Error Flag
        OE: 5 = struct OE(bool);
        /// USCI Frame Error Flag
        FE: 6 = struct FE(bool);
        /// USCI Listen mode
        LISTEN: 7 = struct LISTEN(bool);
    }
    /// USCI B Receive Buffer
    rw RXBUF @ 0x06: u8 = 0_0 {
        /// USCI B Receive Buffer
        RXBUF: 0..7 = struct RXBUFField(u8);
    }
    /// USCI B Transmit Buffer
    rw TXBUF @ 0x07: u8 = 0_0 {
        /// USCI B Transmit Buffer
        TXBUF: 0..7 = struct TXBUFField(u8);
    }
}
