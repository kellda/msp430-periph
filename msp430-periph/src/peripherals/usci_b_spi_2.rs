//! USCI_B  SPI Mode

utils::periph! {
    /// USCI_B  SPI Mode
    USCI_B_SPI;
    /// USCI B Control Word Register 0
    rw CTLW0 @ 0x00: u16 = 0_0 {
        /// USCI B Control Word Register 0
        CTLW0: 0..15 = struct CTLW0Field(u16);
    }
    /// USCI B Control Register 0
    rw CTL0 @ 0x01: u8 = 0_0 {
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
    rw CTL1 @ 0x00: u8 = 0_0 {
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
    /// USCI B Baud Word Rate 0
    rw BRW @ 0x06: u16 = 0_0 {
        /// USCI B Baud Word Rate 0
        BRW: 0..15 = struct BRWField(u16);
    }
    /// USCI B Baud Rate 0
    rw BR0 @ 0x06: u8 = 0_0 {
        /// USCI B Baud Rate 0
        BR0: 0..7 = struct BR0Field(u8);
    }
    /// USCI B Baud Rate 1
    rw BR1 @ 0x07: u8 = 0_0 {
        /// USCI B Baud Rate 1
        BR1: 0..7 = struct BR1Field(u8);
    }
    /// USCI B Status Register
    rw STAT @ 0x0a: u8 = 0_0 {
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
    rw RXBUF @ 0x0c: u8 = 0_0 {
        /// USCI B Receive Buffer
        RXBUF: 0..7 = struct RXBUFField(u8);
    }
    /// USCI B Transmit Buffer
    rw TXBUF @ 0x0e: u8 = 0_0 {
        /// USCI B Transmit Buffer
        TXBUF: 0..7 = struct TXBUFField(u8);
    }
    /// USCI B Interrupt Enable Register
    rw ICTL @ 0x1c: u16 = 0_0 {
        /// USCI B Interrupt Enable Register
        ICTL: 0..15 = struct ICTLField(u16);
    }
    /// USCI B Interrupt Enable Register
    rw IE @ 0x1c: u8 = 0_0 {
        /// USCI Receive Interrupt Enable
        RXIE: 0 = struct RXIE(bool);
        /// USCI Transmit Interrupt Enable
        TXIE: 1 = struct TXIE(bool);
        /// START Condition interrupt enable
        STTIE: 2 = struct STTIE(bool);
        /// STOP Condition interrupt enable
        STPIE: 3 = struct STPIE(bool);
        /// Arbitration Lost interrupt enable
        ALIE: 4 = struct ALIE(bool);
        /// NACK Condition interrupt enable
        NACKIE: 5 = struct NACKIE(bool);
    }
    /// USCI B Interrupt Flags Register
    rw IFG @ 0x1d: u8 = 0_0 {
        /// USCI Receive Interrupt Flag
        RXIFG: 0 = struct RXIFG(bool);
        /// USCI Transmit Interrupt Flag
        TXIFG: 1 = struct TXIFG(bool);
    }
    /// USCI B Interrupt Vector Register
    rw IV @ 0x1e: u16 = 0_0 {
        /// USCI B Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
}
