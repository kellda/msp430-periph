//! USCI_A0  UART Mode

utils::periph! {
    /// USCI_A0  UART Mode
    USCI_A0UARTMode;
    /// USCI A0 Control Word Register 0
    rw UCA0CTLW0 @ 0x00: u16 = 0_0 {
        /// USCI A0 Control Word Register 0
        UCA0CTLW0: 0..15 = struct UCA0CTLW0Field(u16);
    }
    /// USCI A0 Control Register 0
    rw UCA0CTL0 @ 0x01: u8 = 0_0 {
        /// Sync-Mode  0:UART-Mode / 1:SPI-Mode
        UCSYNC: 0 = struct UCSYNC(bool);
        /// Async. Mode: USCI Mode 1
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
        /// Async. Mode: Stop Bits  0:one / 1: two
        UCSPB: 3 = struct UCSPB(bool);
        /// Async. Mode: Data Bits  0:8-bits / 1:7-bits
        UC7BIT: 4 = struct UC7BIT(bool);
        /// Async. Mode: MSB first  0:LSB / 1:MSB
        UCMSB: 5 = struct UCMSB(bool);
        /// Async. Mode: Parity     0:odd / 1:even
        UCPAR: 6 = struct UCPAR(bool);
        /// Async. Mode: Parity enable
        UCPEN: 7 = struct UCPEN(bool);
    }
    /// USCI A0 Control Register 1
    rw UCA0CTL1 @ 0x00: u8 = 0_0 {
        /// USCI Software Reset
        UCSWRST: 0 = struct UCSWRST(bool);
        /// Send next Data as Break
        UCTXBRK: 1 = struct UCTXBRK(bool);
        /// Send next Data as Address
        UCTXADDR: 2 = struct UCTXADDR(bool);
        /// Dormant (Sleep) Mode
        UCDORM: 3 = struct UCDORM(bool);
        /// Break interrupt enable
        UCBRKIE: 4 = struct UCBRKIE(bool);
        /// RX Error interrupt enable
        UCRXEIE: 5 = struct UCRXEIE(bool);
        /// USCI 0 Clock Source Select 1
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
    /// USCI A0 Baud Word Rate 0
    rw UCA0BRW @ 0x06: u16 = 0_0 {
        /// USCI A0 Baud Word Rate 0
        UCA0BRW: 0..15 = struct UCA0BRWField(u16);
    }
    /// USCI A0 Baud Rate 0
    rw UCA0BR0 @ 0x06: u8 = 0_0 {
        /// USCI A0 Baud Rate 0
        UCA0BR0: 0..7 = struct UCA0BR0Field(u8);
    }
    /// USCI A0 Baud Rate 1
    rw UCA0BR1 @ 0x07: u8 = 0_0 {
        /// USCI A0 Baud Rate 1
        UCA0BR1: 0..7 = struct UCA0BR1Field(u8);
    }
    /// USCI A0 Modulation Control
    rw UCA0MCTL @ 0x08: u8 = 0_0 {
        /// USCI 16-times Oversampling enable
        UCOS16: 0 = struct UCOS16(bool);
        /// USCI Second Stage Modulation Select 2
        UCBRS: 1..3 = enum UCBRS {
            /// USCI Second Stage Modulation: 0
            UCBRS_0 = 0b000,
            /// USCI Second Stage Modulation: 1
            UCBRS_1 = 0b001,
            /// USCI Second Stage Modulation: 2
            UCBRS_2 = 0b010,
            /// USCI Second Stage Modulation: 3
            UCBRS_3 = 0b011,
            /// USCI Second Stage Modulation: 4
            UCBRS_4 = 0b100,
            /// USCI Second Stage Modulation: 5
            UCBRS_5 = 0b101,
            /// USCI Second Stage Modulation: 6
            UCBRS_6 = 0b110,
            /// USCI Second Stage Modulation: 7
            UCBRS_7 = 0b111,
        }
        /// USCI First Stage Modulation Select 3
        UCBRF: 4..7 = enum UCBRF {
            /// USCI First Stage Modulation: 0
            UCBRF_0 = 0b0000,
            /// USCI First Stage Modulation: 1
            UCBRF_1 = 0b0001,
            /// USCI First Stage Modulation: 2
            UCBRF_2 = 0b0010,
            /// USCI First Stage Modulation: 3
            UCBRF_3 = 0b0011,
            /// USCI First Stage Modulation: 4
            UCBRF_4 = 0b0100,
            /// USCI First Stage Modulation: 5
            UCBRF_5 = 0b0101,
            /// USCI First Stage Modulation: 6
            UCBRF_6 = 0b0110,
            /// USCI First Stage Modulation: 7
            UCBRF_7 = 0b0111,
            /// USCI First Stage Modulation: 8
            UCBRF_8 = 0b1000,
            /// USCI First Stage Modulation: 9
            UCBRF_9 = 0b1001,
            /// USCI First Stage Modulation: A
            UCBRF_10 = 0b1010,
            /// USCI First Stage Modulation: B
            UCBRF_11 = 0b1011,
            /// USCI First Stage Modulation: C
            UCBRF_12 = 0b1100,
            /// USCI First Stage Modulation: D
            UCBRF_13 = 0b1101,
            /// USCI First Stage Modulation: E
            UCBRF_14 = 0b1110,
            /// USCI First Stage Modulation: F
            UCBRF_15 = 0b1111,
        }
    }
    /// USCI A0 Status Register
    rw UCA0STAT @ 0x0a: u8 = 0_0 {
        /// USCI Busy Flag
        UCBUSY: 0 = struct UCBUSY(bool);
        /// USCI Address received Flag
        UCADDR: 1 = struct UCADDR(bool);
        /// USCI RX Error Flag
        UCRXERR: 2 = struct UCRXERR(bool);
        /// USCI Break received
        UCBRK: 3 = struct UCBRK(bool);
        /// USCI Parity Error Flag
        UCPE: 4 = struct UCPE(bool);
        /// USCI Overrun Error Flag
        UCOE: 5 = struct UCOE(bool);
        /// USCI Frame Error Flag
        UCFE: 6 = struct UCFE(bool);
        /// USCI Listen mode
        UCLISTEN: 7 = struct UCLISTEN(bool);
    }
    /// USCI A0 Receive Buffer
    rw UCA0RXBUF @ 0x0c: u8 = 0_0 {
        /// USCI A0 Receive Buffer
        UCA0RXBUF: 0..7 = struct UCA0RXBUFField(u8);
    }
    /// USCI A0 Transmit Buffer
    rw UCA0TXBUF @ 0x0e: u8 = 0_0 {
        /// USCI A0 Transmit Buffer
        UCA0TXBUF: 0..7 = struct UCA0TXBUFField(u8);
    }
    /// USCI A0 LIN Control
    rw UCA0ABCTL @ 0x10: u8 = 0_0 {
        /// Auto Baud Rate detect enable
        UCABDEN: 0 = struct UCABDEN(bool);
        /// Break Timeout error
        UCBTOE: 2 = struct UCBTOE(bool);
        /// Sync-Field Timeout error
        UCSTOE: 3 = struct UCSTOE(bool);
        /// Break Sync Delimiter 0
        UCDELIM0: 4 = struct UCDELIM0(bool);
        /// Break Sync Delimiter 1
        UCDELIM1: 5 = struct UCDELIM1(bool);
    }
    /// USCI A0 IrDA Transmit Control
    rw UCA0IRCTL @ 0x12: u16 = 0_0 {
        /// USCI A0 IrDA Transmit Control
        UCA0IRCTL: 0..15 = struct UCA0IRCTLField(u16);
    }
    /// USCI A0 IrDA Transmit Control
    rw UCA0IRTCTL @ 0x12: u8 = 0_0 {
        /// IRDA Encoder/Decoder enable
        UCIREN: 0 = struct UCIREN(bool);
        /// IRDA Transmit Pulse Clock Select
        UCIRTXCLK: 1 = struct UCIRTXCLK(bool);
        /// IRDA Transmit Pulse Length 0
        UCIRTXPL0: 2 = struct UCIRTXPL0(bool);
        /// IRDA Transmit Pulse Length 1
        UCIRTXPL1: 3 = struct UCIRTXPL1(bool);
        /// IRDA Transmit Pulse Length 2
        UCIRTXPL2: 4 = struct UCIRTXPL2(bool);
        /// IRDA Transmit Pulse Length 3
        UCIRTXPL3: 5 = struct UCIRTXPL3(bool);
        /// IRDA Transmit Pulse Length 4
        UCIRTXPL4: 6 = struct UCIRTXPL4(bool);
        /// IRDA Transmit Pulse Length 5
        UCIRTXPL5: 7 = struct UCIRTXPL5(bool);
    }
    /// USCI A0 IrDA Receive Control
    rw UCA0IRRCTL @ 0x13: u8 = 0_0 {
        /// IRDA Receive Filter enable
        UCIRRXFE: 0 = struct UCIRRXFE(bool);
        /// IRDA Receive Input Polarity
        UCIRRXPL: 1 = struct UCIRRXPL(bool);
        /// IRDA Receive Filter Length 0
        UCIRRXFL0: 2 = struct UCIRRXFL0(bool);
        /// IRDA Receive Filter Length 1
        UCIRRXFL1: 3 = struct UCIRRXFL1(bool);
        /// IRDA Receive Filter Length 2
        UCIRRXFL2: 4 = struct UCIRRXFL2(bool);
        /// IRDA Receive Filter Length 3
        UCIRRXFL3: 5 = struct UCIRRXFL3(bool);
        /// IRDA Receive Filter Length 4
        UCIRRXFL4: 6 = struct UCIRRXFL4(bool);
        /// IRDA Receive Filter Length 5
        UCIRRXFL5: 7 = struct UCIRRXFL5(bool);
    }
    /// USCI A0 Interrupt Enable Register
    rw UCA0ICTL @ 0x1c: u16 = 0_0 {
        /// USCI A0 Interrupt Enable Register
        UCA0ICTL: 0..15 = struct UCA0ICTLField(u16);
    }
    /// USCI A0 Interrupt Enable Register
    rw UCA0IE @ 0x1c: u8 = 0_0 {
        /// USCI Receive Interrupt Enable
        UCRXIE: 0 = struct UCRXIE(bool);
        /// USCI Transmit Interrupt Enable
        UCTXIE: 1 = struct UCTXIE(bool);
    }
    /// USCI A0 Interrupt Flags Register
    rw UCA0IFG @ 0x1d: u8 = 0_0 {
        /// USCI Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// USCI Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
    }
    /// USCI A0 Interrupt Vector Register
    rw UCA0IV @ 0x1e: u16 = 0_0 {
        /// USCI A0 Interrupt Vector Register
        UCA0IV: 0..15 = struct UCA0IVField(u16);
    }
}
