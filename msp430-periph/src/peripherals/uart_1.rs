//! UART

utils::periph! {
    /// UART
    UART;
    /// USART Control
    rw UCTL @ 0x00: u8 = 0_0 {
        /// SWRST
        SWRST: 0 = struct SWRST(bool);
        /// MM
        MM: 1 = struct MM(bool);
        /// SYNC
        SYNC: 2 = struct SYNC(bool);
        /// LISTEN
        LISTEN: 3 = struct LISTEN(bool);
        /// CHAR
        CHAR: 4 = struct CHAR(bool);
        /// to distinguish from stackpointer SP
        SPB: 5 = struct SPB(bool);
        /// PEV
        PEV: 6 = struct PEV(bool);
        /// UCTL
        PENA: 7 = struct PENA(bool);
    }
    /// USART Transmit Control
    rw UTCTL @ 0x01: u8 = 0_0 {
        /// TXEPT
        TXEPT: 0 = struct TXEPT(bool);
        /// STC
        STC: 1 = struct STC(bool);
        /// TXWAKE
        TXWAKE: 2 = struct TXWAKE(bool);
        /// URXSE
        URXSE: 3 = struct URXSE(bool);
        /// SSEL0
        SSEL0: 4 = struct SSEL0(bool);
        /// SSEL1
        SSEL1: 5 = struct SSEL1(bool);
        /// CKPL
        CKPL: 6 = struct CKPL(bool);
        /// UTCTL
        CKPH: 7 = struct CKPH(bool);
    }
    /// USART Receive Control
    rw URCTL @ 0x02: u8 = 0_0 {
        /// RXERR
        RXERR: 0 = struct RXERR(bool);
        /// RXWAKE
        RXWAKE: 1 = struct RXWAKE(bool);
        /// URXWIE
        URXWIE: 2 = struct URXWIE(bool);
        /// URXEIE
        URXEIE: 3 = struct URXEIE(bool);
        /// BRK
        BRK: 4 = struct BRK(bool);
        /// OE
        OE: 5 = struct OE(bool);
        /// PE
        PE: 6 = struct PE(bool);
        /// URCTL
        FE: 7 = struct FE(bool);
    }
    /// USART Modulation Control
    rw UMCTL @ 0x03: u8 = 0_0 {
        /// USART Modulation Control
        UMCTL: 0..7 = struct UMCTLField(u8);
    }
    /// USART Baud Rate 0
    rw UBR0 @ 0x04: u8 = 0_0 {
        /// USART Baud Rate 0
        UBR0: 0..7 = struct UBR0Field(u8);
    }
    /// USART Buad Rate 1
    rw UBR1 @ 0x05: u8 = 0_0 {
        /// USART Buad Rate 1
        UBR1: 0..7 = struct UBR1Field(u8);
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
