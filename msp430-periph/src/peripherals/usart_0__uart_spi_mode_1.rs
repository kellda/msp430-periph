//! USART 0  UART/SPI Mode

utils::periph! {
    /// USART 0  UART/SPI Mode
    USART0UARTSPIMode;
    /// USART 0 Control
    rw U0CTL @ 0x00: u8 = 0_0 {
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
    /// USART 0 Transmit Control
    rw U0TCTL @ 0x01: u8 = 0_0 {
        /// TX Buffer empty
        TXEPT: 0 = struct TXEPT(bool);
        /// SPI: STC enable 0:on / 1:off
        STC: 1 = struct STC(bool);
        /// TX Wake up mode
        TXWAKE: 2 = struct TXWAKE(bool);
        /// Receive Start edge select
        URXSE: 3 = struct URXSE(bool);
        /// Clock Source Select 0
        SSEL0: 4 = struct SSEL0(bool);
        /// Clock Source Select 1
        SSEL1: 5 = struct SSEL1(bool);
        /// Clock Polarity
        CKPL: 6 = struct CKPL(bool);
        /// SPI: Clock Phase
        CKPH: 7 = struct CKPH(bool);
    }
    /// USART 0 Receive Control
    rw U0RCTL @ 0x02: u8 = 0_0 {
        /// RX Error Error
        RXERR: 0 = struct RXERR(bool);
        /// RX Wake up detect
        RXWAKE: 1 = struct RXWAKE(bool);
        /// RX Wake up interrupt enable
        URXWIE: 2 = struct URXWIE(bool);
        /// RX Error interrupt enable
        URXEIE: 3 = struct URXEIE(bool);
        /// Break detected
        BRK: 4 = struct BRK(bool);
        /// Overrun Error
        OE: 5 = struct OE(bool);
        /// Parity Error
        PE: 6 = struct PE(bool);
        /// Frame Error
        FE: 7 = struct FE(bool);
    }
    /// USART 0 Modulation Control
    rw U0MCTL @ 0x03: u8 = 0_0 {
        /// USART 0 Modulation Control
        U0MCTL: 0..7 = struct U0MCTLField(u8);
    }
    /// USART 0 Baud Rate 0
    rw U0BR0 @ 0x04: u8 = 0_0 {
        /// USART 0 Baud Rate 0
        U0BR0: 0..7 = struct U0BR0Field(u8);
    }
    /// USART 0 Baud Rate 1
    rw U0BR1 @ 0x05: u8 = 0_0 {
        /// USART 0 Baud Rate 1
        U0BR1: 0..7 = struct U0BR1Field(u8);
    }
    /// USART 0 Receive Buffer
    rw U0RXBUF @ 0x06: u8 = 0_0 {
        /// USART 0 Receive Buffer
        U0RXBUF: 0..7 = struct U0RXBUFField(u8);
    }
    /// USART 0 Transmit Buffer
    rw U0TXBUF @ 0x07: u8 = 0_0 {
        /// USART 0 Transmit Buffer
        U0TXBUF: 0..7 = struct U0TXBUFField(u8);
    }
}
