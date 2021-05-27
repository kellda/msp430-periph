//! USCI_A  UART Mode

utils::periph! {
    /// USCI_A  UART Mode
    USCI_A_UART;
    /// USCI A Control Word Register 0
    rw CTLW0 @ 0x00: u16 = 0_0 {
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
        /// Sync-Mode  0:UART-Mode / 1:SPI-Mode
        SYNC: 8 = struct SYNC(bool);
        /// Async. Mode: USCI Mode 1
        MODE: 9..10 = enum MODE {
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
        SPB: 11 = struct SPB(bool);
        /// Async. Mode: Data Bits  0:8-bits / 1:7-bits
        _7BIT: 12 = struct _7BIT(bool);
        /// Async. Mode: MSB first  0:LSB / 1:MSB
        MSB: 13 = struct MSB(bool);
        /// Async. Mode: Parity     0:odd / 1:even
        PAR: 14 = struct PAR(bool);
        /// Async. Mode: Parity enable
        PEN: 15 = struct PEN(bool);
    }
    /// USCI A Control Register 0
    rw CTL0 @ 0x01: u8 = 0_0 {
        /// USCI A Control Register 0
        CTL0: 0..7 = struct CTL0Field(u8);
    }
    /// USCI A Control Register 1
    rw CTL1 @ 0x00: u8 = 0_0 {
        /// USCI A Control Register 1
        CTL1: 0..7 = struct CTL1Field(u8);
    }
    /// USCI A Control Word Register 1
    rw CTLW1 @ 0x02: u16 = 0_0 {
        /// USCI Deglitch Time Bit 1
        GLIT: 0..1 = enum GLIT {
            /// USCI Deglitch time: 0
            GLIT_0 = 0b00,
            /// USCI Deglitch time: 1
            GLIT_1 = 0b01,
            /// USCI Deglitch time: 2
            GLIT_2 = 0b10,
            /// USCI Deglitch time: 3
            GLIT_3 = 0b11,
        }
    }
    /// USCI A Baud Word Rate 0
    rw BRW @ 0x06: u16 = 0_0 {
        /// USCI A Baud Word Rate 0
        BRW: 0..15 = struct BRWField(u16);
    }
    /// USCI A Baud Rate 0
    rw BR0 @ 0x06: u8 = 0_0 {
        /// USCI A Baud Rate 0
        BR0: 0..7 = struct BR0Field(u8);
    }
    /// USCI A Baud Rate 1
    rw BR1 @ 0x07: u8 = 0_0 {
        /// USCI A Baud Rate 1
        BR1: 0..7 = struct BR1Field(u8);
    }
    /// USCI A Modulation Control
    rw MCTLW @ 0x08: u16 = 0_0 {
        /// USCI 16-times Oversampling enable
        OS16: 0 = struct OS16(bool);
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
        /// USCI Second Stage Modulation Select 0
        BRS0: 8 = struct BRS0(bool);
        /// USCI Second Stage Modulation Select 1
        BRS1: 9 = struct BRS1(bool);
        /// USCI Second Stage Modulation Select 2
        BRS2: 10 = struct BRS2(bool);
        /// USCI Second Stage Modulation Select 3
        BRS3: 11 = struct BRS3(bool);
        /// USCI Second Stage Modulation Select 4
        BRS4: 12 = struct BRS4(bool);
        /// USCI Second Stage Modulation Select 5
        BRS5: 13 = struct BRS5(bool);
        /// USCI Second Stage Modulation Select 6
        BRS6: 14 = struct BRS6(bool);
        /// USCI Second Stage Modulation Select 7
        BRS7: 15 = struct BRS7(bool);
    }
    /// USCI A Status Register
    rw STATW @ 0x0a: u8 = 0_0 {
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
    rw RXBUF @ 0x0c: u16 = 0_0 {
        /// USCI A Receive Buffer
        RXBUF: 0..15 = struct RXBUFField(u16);
    }
    /// USCI A Transmit Buffer
    rw TXBUF @ 0x0e: u16 = 0_0 {
        /// USCI A Transmit Buffer
        TXBUF: 0..15 = struct TXBUFField(u16);
    }
    /// USCI A LIN Control
    rw ABCTL @ 0x10: u8 = 0_0 {
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
    rw IRCTL @ 0x12: u16 = 0_0 {
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
        /// IRDA Receive Filter enable
        IRRXFE: 8 = struct IRRXFE(bool);
        /// IRDA Receive Input Polarity
        IRRXPL: 9 = struct IRRXPL(bool);
        /// IRDA Receive Filter Length 0
        IRRXFL0: 10 = struct IRRXFL0(bool);
        /// IRDA Receive Filter Length 1
        IRRXFL1: 11 = struct IRRXFL1(bool);
        /// IRDA Receive Filter Length 2
        IRRXFL2: 12 = struct IRRXFL2(bool);
        /// IRDA Receive Filter Length 3
        IRRXFL3: 13 = struct IRRXFL3(bool);
        /// IRDA Receive Filter Length 4
        IRRXFL4: 14 = struct IRRXFL4(bool);
        /// IRDA Receive Filter Length 5
        IRRXFL5: 15 = struct IRRXFL5(bool);
    }
    /// USCI A IrDA Transmit Control
    rw IRTCTL @ 0x12: u8 = 0_0 {
        /// USCI A IrDA Transmit Control
        IRTCTL: 0..7 = struct IRTCTLField(u8);
    }
    /// USCI A IrDA Receive Control
    rw IRRCTL @ 0x13: u8 = 0_0 {
        /// USCI A IrDA Receive Control
        IRRCTL: 0..7 = struct IRRCTLField(u8);
    }
    /// USCI A Interrupt Enable Register
    rw IE @ 0x1a: u16 = 0_0 {
        /// USCI A Interrupt Enable Register
        IE: 0..15 = struct IEField(u16);
    }
    /// USCI A Interrupt Flags Register
    rw IFG @ 0x1c: u16 = 0_0 {
        /// USCI A Interrupt Flags Register
        IFG: 0..15 = struct IFGField(u16);
    }
    /// USCI A Interrupt Enable Register
    rw IE__UART @ 0x1a: u16 = 0_0 {
        /// UART Receive Interrupt Enable
        RXIE: 0 = struct RXIE(bool);
        /// UART Transmit Interrupt Enable
        TXIE: 1 = struct TXIE(bool);
        /// UART Start Bit Interrupt Enalble
        STTIE: 2 = struct STTIE(bool);
        /// UART Transmit Complete Interrupt Enable
        TXCPTIE: 3 = struct TXCPTIE(bool);
    }
    /// USCI A Interrupt Flags Register
    rw IFG__UART @ 0x1c: u16 = 0_0 {
        /// UART Receive Interrupt Flag
        RXIFG: 0 = struct RXIFG(bool);
        /// UART Transmit Interrupt Flag
        TXIFG: 1 = struct TXIFG(bool);
        /// UART Start Bit Interrupt Flag
        STTIFG: 2 = struct STTIFG(bool);
        /// UART Transmit Complete Interrupt Flag
        TXCPTIFG: 3 = struct TXCPTIFG(bool);
    }
    /// USCI A Interrupt Vector Register
    rw IV @ 0x1e: u16 = 0_0 {
        /// USCI A Interrupt Vector Register
        IV: 0..15 = struct IVField(u16);
    }
}
