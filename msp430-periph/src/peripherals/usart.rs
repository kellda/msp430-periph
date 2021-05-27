//! USART

utils::periph! {
    /// USART
    USART;
    /// USART Control
    rw CTL @ 0x00: u8 = 0_0 {
        /// USART Software Reset
        SWRST: 0 = struct SWRST(bool);
        /// Master Mode off/on
        MM: 1 = struct MM(bool);
        /// UART / SPI mode
        SYNC: 2 = struct SYNC(bool);
        /// Listen mode
        LISTEN: 3 = struct LISTEN(bool);
        /// Data 0:7-bits / 1:8-bits
        CHAR: 4 = struct CHAR(bool);
        /// Stop Bits 0:one / 1: two
        SPB: 5 = struct SPB(bool);
        /// Parity 0:odd / 1:even
        PEV: 6 = struct PEV(bool);
        /// Parity enable
        PENA: 7 = struct PENA(bool);
    }
    /// USART Transmit Control
    rw TCTL @ 0x01: u8 = 0_0 {
        /// TX Buffer empty
        TXEPT: 0 = struct TXEPT(bool);
        /// SPI: STC enable 0:on / 1:off
        STC: 1 = struct STC(bool);
        /// TX Wake up mode
        TXWAKE: 2 = struct TXWAKE(bool);
        /// Receive Start edge select
        RXSE: 3 = struct RXSE(bool);
        /// Clock Source Select 0
        SSEL0: 4 = struct SSEL0(bool);
        /// Clock Source Select 1
        SSEL1: 5 = struct SSEL1(bool);
        /// Clock Polarity
        CKPL: 6 = struct CKPL(bool);
        /// SPI: Clock Phase
        CKPH: 7 = struct CKPH(bool);
    }
    /// USART Receive Control
    rw RCTL @ 0x02: u8 = 0_0 {
        /// RX Error Error
        RXERR: 0 = struct RXERR(bool);
        /// RX Wake up detect
        RXWAKE: 1 = struct RXWAKE(bool);
        /// RX Wake up interrupt enable
        RXWIE: 2 = struct RXWIE(bool);
        /// RX Error interrupt enable
        RXEIE: 3 = struct RXEIE(bool);
        /// Break detected
        BRK: 4 = struct BRK(bool);
        /// Overrun Error
        OE: 5 = struct OE(bool);
        /// Parity Error
        PE: 6 = struct PE(bool);
        /// Frame Error
        FE: 7 = struct FE(bool);
    }
    /// USART Modulation Control
    rw MCTL @ 0x03: u8 = 0_0 {
        /// USART Modulation Control
        MCTL: 0..7 = struct MCTLField(u8);
    }
    /// USART Baud Rate 0
    rw BR0 @ 0x04: u8 = 0_0 {
        /// USART Baud Rate 0
        BR0: 0..7 = struct BR0Field(u8);
    }
    /// USART Baud Rate 1
    rw BR1 @ 0x05: u8 = 0_0 {
        /// USART Baud Rate 1
        BR1: 0..7 = struct BR1Field(u8);
    }
    /// USART Receive Buffer
    rw RXBUF @ 0x06: u8 = 0_0 {
        /// USART Receive Buffer
        RXBUF: 0..7 = struct RXBUFField(u8);
    }
    /// USART Transmit Buffer
    rw TXBUF @ 0x07: u8 = 0_0 {
        /// USART Transmit Buffer
        TXBUF: 0..7 = struct TXBUFField(u8);
    }
}
