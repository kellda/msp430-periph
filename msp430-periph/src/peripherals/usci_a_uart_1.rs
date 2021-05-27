//! USCI_A  UART Mode

utils::periph! {
    /// USCI_A  UART Mode
    USCI_A_UART;
    /// USCI A Control Register 0
    rw CTL0 @ 0x03: u8 = 0_0 {
        /// Sync-Mode  0:UART-Mode / 1:SPI-Mode
        SYNC: 0 = struct SYNC(bool);
        /// Async. Mode: USCI Mode 1
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
        /// Async. Mode: Stop Bits  0:one / 1: two
        SPB: 3 = struct SPB(bool);
        /// Async. Mode: Data Bits  0:8-bits / 1:7-bits
        _7BIT: 4 = struct _7BIT(bool);
        /// Async. Mode: MSB first  0:LSB / 1:MSB
        MSB: 5 = struct MSB(bool);
        /// Async. Mode: Parity     0:odd / 1:even
        PAR: 6 = struct PAR(bool);
        /// Async. Mode: Parity enable
        PEN: 7 = struct PEN(bool);
    }
    /// USCI A Control Register 1
    rw CTL1 @ 0x04: u8 = 0_0 {
        /// USCI Software Reset
        SWRST: 0 = struct SWRST(bool);
        /// Send next Data as Break
        TXBRK: 1 = struct TXBRK(bool);
        /// Send next Data as Address
        TXADDR: 2 = struct TXADDR(bool);
        /// Dormant (Sleep) Mode
        DORM: 3 = struct DORM(bool);
        /// Break interrupt enable
        BRKIE: 4 = struct BRKIE(bool);
        /// RX Error interrupt enable
        RXEIE: 5 = struct RXEIE(bool);
        /// USCI 0 Clock Source Select 1
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
    /// USCI A Baud Rate 0
    rw BR0 @ 0x05: u8 = 0_0 {
        /// USCI A Baud Rate 0
        BR0: 0..7 = struct BR0Field(u8);
    }
    /// USCI A Baud Rate 1
    rw BR1 @ 0x06: u8 = 0_0 {
        /// USCI A Baud Rate 1
        BR1: 0..7 = struct BR1Field(u8);
    }
    /// USCI A Modulation Control
    rw MCTL @ 0x07: u8 = 0_0 {
        /// USCI 16-times Oversampling enable
        OS16: 0 = struct OS16(bool);
        /// USCI Second Stage Modulation Select 2
        BRS: 1..3 = enum BRS {
            /// USCI Second Stage Modulation: 0
            BRS_0 = 0b000,
            /// USCI Second Stage Modulation: 1
            BRS_1 = 0b001,
            /// USCI Second Stage Modulation: 2
            BRS_2 = 0b010,
            /// USCI Second Stage Modulation: 3
            BRS_3 = 0b011,
            /// USCI Second Stage Modulation: 4
            BRS_4 = 0b100,
            /// USCI Second Stage Modulation: 5
            BRS_5 = 0b101,
            /// USCI Second Stage Modulation: 6
            BRS_6 = 0b110,
            /// USCI Second Stage Modulation: 7
            BRS_7 = 0b111,
        }
        /// USCI First Stage Modulation Select 3
        BRF: 4..7 = enum BRF {
            /// USCI First Stage Modulation: 0
            BRF_0 = 0b0000,
            /// USCI First Stage Modulation: 1
            BRF_1 = 0b0001,
            /// USCI First Stage Modulation: 2
            BRF_2 = 0b0010,
            /// USCI First Stage Modulation: 3
            BRF_3 = 0b0011,
            /// USCI First Stage Modulation: 4
            BRF_4 = 0b0100,
            /// USCI First Stage Modulation: 5
            BRF_5 = 0b0101,
            /// USCI First Stage Modulation: 6
            BRF_6 = 0b0110,
            /// USCI First Stage Modulation: 7
            BRF_7 = 0b0111,
            /// USCI First Stage Modulation: 8
            BRF_8 = 0b1000,
            /// USCI First Stage Modulation: 9
            BRF_9 = 0b1001,
            /// USCI First Stage Modulation: A
            BRF_10 = 0b1010,
            /// USCI First Stage Modulation: B
            BRF_11 = 0b1011,
            /// USCI First Stage Modulation: C
            BRF_12 = 0b1100,
            /// USCI First Stage Modulation: D
            BRF_13 = 0b1101,
            /// USCI First Stage Modulation: E
            BRF_14 = 0b1110,
            /// USCI First Stage Modulation: F
            BRF_15 = 0b1111,
        }
    }
    /// USCI A Status Register
    rw STAT @ 0x08: u8 = 0_0 {
        /// USCI Busy Flag
        BUSY: 0 = struct BUSY(bool);
        /// USCI Address received Flag
        DDR: 1 = struct DDR(bool);
        /// USCI RX Error Flag
        RXERR: 2 = struct RXERR(bool);
        /// USCI Break received
        BRK: 3 = struct BRK(bool);
        /// USCI Parity Error Flag
        PE: 4 = struct PE(bool);
        /// USCI Overrun Error Flag
        OE: 5 = struct OE(bool);
        /// USCI Frame Error Flag
        FE: 6 = struct FE(bool);
        /// USCI Listen mode
        LISTEN: 7 = struct LISTEN(bool);
    }
    /// USCI A Receive Buffer
    rw RXBUF @ 0x09: u8 = 0_0 {
        /// USCI A Receive Buffer
        RXBUF: 0..7 = struct RXBUFField(u8);
    }
    /// USCI A Transmit Buffer
    rw TXBUF @ 0x0a: u8 = 0_0 {
        /// USCI A Transmit Buffer
        TXBUF: 0..7 = struct TXBUFField(u8);
    }
    /// USCI A LIN Control
    rw ABCTL @ 0x00: u8 = 0_0 {
        /// Auto Baud Rate detect enable
        BDEN: 0 = struct BDEN(bool);
        /// Break Timeout error
        BTOE: 2 = struct BTOE(bool);
        /// Sync-Field Timeout error
        STOE: 3 = struct STOE(bool);
        /// Break Sync Delimiter 0
        DELIM0: 4 = struct DELIM0(bool);
        /// Break Sync Delimiter 1
        DELIM1: 5 = struct DELIM1(bool);
    }
    /// USCI A IrDA Transmit Control
    rw IRTCTL @ 0x01: u8 = 0_0 {
        /// IRDA Encoder/Decoder enable
        IREN: 0 = struct IREN(bool);
        /// IRDA Transmit Pulse Clock Select
        IRTXCLK: 1 = struct IRTXCLK(bool);
        /// IRDA Transmit Pulse Length 0
        IRTXPL0: 2 = struct IRTXPL0(bool);
        /// IRDA Transmit Pulse Length 1
        IRTXPL1: 3 = struct IRTXPL1(bool);
        /// IRDA Transmit Pulse Length 2
        IRTXPL2: 4 = struct IRTXPL2(bool);
        /// IRDA Transmit Pulse Length 3
        IRTXPL3: 5 = struct IRTXPL3(bool);
        /// IRDA Transmit Pulse Length 4
        IRTXPL4: 6 = struct IRTXPL4(bool);
        /// IRDA Transmit Pulse Length 5
        IRTXPL5: 7 = struct IRTXPL5(bool);
    }
    /// USCI A IrDA Receive Control
    rw IRRCTL @ 0x02: u8 = 0_0 {
        /// IRDA Receive Filter enable
        IRRXFE: 0 = struct IRRXFE(bool);
        /// IRDA Receive Input Polarity
        IRRXPL: 1 = struct IRRXPL(bool);
        /// IRDA Receive Filter Length 0
        IRRXFL0: 2 = struct IRRXFL0(bool);
        /// IRDA Receive Filter Length 1
        IRRXFL1: 3 = struct IRRXFL1(bool);
        /// IRDA Receive Filter Length 2
        IRRXFL2: 4 = struct IRRXFL2(bool);
        /// IRDA Receive Filter Length 3
        IRRXFL3: 5 = struct IRRXFL3(bool);
        /// IRDA Receive Filter Length 4
        IRRXFL4: 6 = struct IRRXFL4(bool);
        /// IRDA Receive Filter Length 5
        IRRXFL5: 7 = struct IRRXFL5(bool);
    }
}
