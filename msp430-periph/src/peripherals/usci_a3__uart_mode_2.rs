//! USCI_A3  UART Mode

utils::periph! {
    /// USCI_A3  UART Mode
    USCI_A3UARTMode;
    /// USCI A3 Control Word Register 0
    rw UCA3CTLW0 @ 0x00: u16 = 0_0 {
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
        /// Sync-Mode  0:UART-Mode / 1:SPI-Mode
        UCSYNC: 8 = struct UCSYNC(bool);
        /// Async. Mode: USCI Mode 1
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
        /// Async. Mode: Stop Bits  0:one / 1: two
        UCSPB: 11 = struct UCSPB(bool);
        /// Async. Mode: Data Bits  0:8-bits / 1:7-bits
        UC7BIT: 12 = struct UC7BIT(bool);
        /// Async. Mode: MSB first  0:LSB / 1:MSB
        UCMSB: 13 = struct UCMSB(bool);
        /// Async. Mode: Parity     0:odd / 1:even
        UCPAR: 14 = struct UCPAR(bool);
        /// Async. Mode: Parity enable
        UCPEN: 15 = struct UCPEN(bool);
    }
    /// USCI A3 Control Register 0
    rw UCA3CTL0 @ 0x01: u8 = 0_0 {
        /// USCI A3 Control Register 0
        UCA3CTL0: 0..7 = struct UCA3CTL0Field(u8);
    }
    /// USCI A3 Control Register 1
    rw UCA3CTL1 @ 0x00: u8 = 0_0 {
        /// USCI A3 Control Register 1
        UCA3CTL1: 0..7 = struct UCA3CTL1Field(u8);
    }
    /// USCI A3 Control Word Register 1
    rw UCA3CTLW1 @ 0x02: u16 = 0_0 {
        /// USCI Deglitch Time Bit 1
        UCGLIT: 0..1 = enum UCGLIT {
            /// USCI Deglitch time: 0
            UCGLIT_0 = 0b00,
            /// USCI Deglitch time: 1
            UCGLIT_1 = 0b01,
            /// USCI Deglitch time: 2
            UCGLIT_2 = 0b10,
            /// USCI Deglitch time: 3
            UCGLIT_3 = 0b11,
        }
    }
    /// USCI A3 Baud Word Rate 0
    rw UCA3BRW @ 0x06: u16 = 0_0 {
        /// USCI A3 Baud Word Rate 0
        UCA3BRW: 0..15 = struct UCA3BRWField(u16);
    }
    /// USCI A3 Baud Rate 0
    rw UCA3BR0 @ 0x06: u8 = 0_0 {
        /// USCI A3 Baud Rate 0
        UCA3BR0: 0..7 = struct UCA3BR0Field(u8);
    }
    /// USCI A3 Baud Rate 1
    rw UCA3BR1 @ 0x07: u8 = 0_0 {
        /// USCI A3 Baud Rate 1
        UCA3BR1: 0..7 = struct UCA3BR1Field(u8);
    }
    /// USCI A3 Modulation Control
    rw UCA3MCTLW @ 0x08: u16 = 0_0 {
        /// USCI 16-times Oversampling enable
        UCOS16: 0 = struct UCOS16(bool);
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
        /// USCI Second Stage Modulation Select 0
        UCBRS0: 8 = struct UCBRS0(bool);
        /// USCI Second Stage Modulation Select 1
        UCBRS1: 9 = struct UCBRS1(bool);
        /// USCI Second Stage Modulation Select 2
        UCBRS2: 10 = struct UCBRS2(bool);
        /// USCI Second Stage Modulation Select 3
        UCBRS3: 11 = struct UCBRS3(bool);
        /// USCI Second Stage Modulation Select 4
        UCBRS4: 12 = struct UCBRS4(bool);
        /// USCI Second Stage Modulation Select 5
        UCBRS5: 13 = struct UCBRS5(bool);
        /// USCI Second Stage Modulation Select 6
        UCBRS6: 14 = struct UCBRS6(bool);
        /// USCI Second Stage Modulation Select 7
        UCBRS7: 15 = struct UCBRS7(bool);
    }
    /// USCI A3 Status Register
    rw UCA3STATW @ 0x0a: u8 = 0_0 {
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
    /// USCI A3 Receive Buffer
    rw UCA3RXBUF @ 0x0c: u16 = 0_0 {
        /// USCI A3 Receive Buffer
        UCA3RXBUF: 0..15 = struct UCA3RXBUFField(u16);
    }
    /// USCI A3 Transmit Buffer
    rw UCA3TXBUF @ 0x0e: u16 = 0_0 {
        /// USCI A3 Transmit Buffer
        UCA3TXBUF: 0..15 = struct UCA3TXBUFField(u16);
    }
    /// USCI A3 LIN Control
    rw UCA3ABCTL @ 0x10: u8 = 0_0 {
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
    /// USCI A3 IrDA Transmit Control
    rw UCA3IRCTL @ 0x12: u16 = 0_0 {
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
        /// IRDA Receive Filter enable
        UCIRRXFE: 8 = struct UCIRRXFE(bool);
        /// IRDA Receive Input Polarity
        UCIRRXPL: 9 = struct UCIRRXPL(bool);
        /// IRDA Receive Filter Length 0
        UCIRRXFL0: 10 = struct UCIRRXFL0(bool);
        /// IRDA Receive Filter Length 1
        UCIRRXFL1: 11 = struct UCIRRXFL1(bool);
        /// IRDA Receive Filter Length 2
        UCIRRXFL2: 12 = struct UCIRRXFL2(bool);
        /// IRDA Receive Filter Length 3
        UCIRRXFL3: 13 = struct UCIRRXFL3(bool);
        /// IRDA Receive Filter Length 4
        UCIRRXFL4: 14 = struct UCIRRXFL4(bool);
        /// IRDA Receive Filter Length 5
        UCIRRXFL5: 15 = struct UCIRRXFL5(bool);
    }
    /// USCI A3 IrDA Transmit Control
    rw UCA3IRTCTL @ 0x12: u8 = 0_0 {
        /// USCI A3 IrDA Transmit Control
        UCA3IRTCTL: 0..7 = struct UCA3IRTCTLField(u8);
    }
    /// USCI A3 IrDA Receive Control
    rw UCA3IRRCTL @ 0x13: u8 = 0_0 {
        /// USCI A3 IrDA Receive Control
        UCA3IRRCTL: 0..7 = struct UCA3IRRCTLField(u8);
    }
    /// USCI A3 Interrupt Enable Register
    rw UCA3IE @ 0x1a: u16 = 0_0 {
        /// USCI A3 Interrupt Enable Register
        UCA3IE: 0..15 = struct UCA3IEField(u16);
    }
    /// USCI A3 Interrupt Flags Register
    rw UCA3IFG @ 0x1c: u16 = 0_0 {
        /// USCI A3 Interrupt Flags Register
        UCA3IFG: 0..15 = struct UCA3IFGField(u16);
    }
    /// USCI A3 Interrupt Enable Register
    rw UCA3IE__UART @ 0x1a: u16 = 0_0 {
        /// UART Receive Interrupt Enable
        UCRXIE: 0 = struct UCRXIE(bool);
        /// UART Transmit Interrupt Enable
        UCTXIE: 1 = struct UCTXIE(bool);
        /// UART Start Bit Interrupt Enalble
        UCSTTIE: 2 = struct UCSTTIE(bool);
        /// UART Transmit Complete Interrupt Enable
        UCTXCPTIE: 3 = struct UCTXCPTIE(bool);
    }
    /// USCI A3 Interrupt Flags Register
    rw UCA3IFG__UART @ 0x1c: u16 = 0_0 {
        /// UART Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// UART Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
        /// UART Start Bit Interrupt Flag
        UCSTTIFG: 2 = struct UCSTTIFG(bool);
        /// UART Transmit Complete Interrupt Flag
        UCTXCPTIFG: 3 = struct UCTXCPTIFG(bool);
    }
    /// USCI A3 Interrupt Vector Register
    rw UCA3IV @ 0x1e: u16 = 0_0 {
        /// USCI A3 Interrupt Vector Register
        UCA3IV: 0..15 = struct UCA3IVField(u16);
    }
}
