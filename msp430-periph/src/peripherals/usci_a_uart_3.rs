//! USCI_A  UART Mode

utils::periph! {
    /// USCI_A  UART Mode
    USCI_A_UART;
    /// USCI A Control Word Register 0
    rw UCACTLW0 @ 0x00: u16 = 0_0 {
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
    /// USCI A Control Register 0
    rw UCACTL0 @ 0x01: u8 = 0_0 {
        /// USCI A Control Register 0
        UCACTL0: 0..7 = struct UCA0CTL0Field(u8);
    }
    /// USCI A Control Register 1
    rw UCACTL1 @ 0x00: u8 = 0_0 {
        /// USCI A Control Register 1
        UCACTL1: 0..7 = struct UCA0CTL1Field(u8);
    }
    /// USCI A Control Word Register 1
    rw UCACTLW1 @ 0x02: u16 = 0_0 {
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
    /// USCI A Baud Word Rate 0
    rw UCABRW @ 0x06: u16 = 0_0 {
        /// USCI A Baud Word Rate 0
        UCABRW: 0..15 = struct UCA0BRWField(u16);
    }
    /// USCI A Baud Rate 0
    rw UCABR0 @ 0x06: u8 = 0_0 {
        /// USCI A Baud Rate 0
        UCABR0: 0..7 = struct UCA0BR0Field(u8);
    }
    /// USCI A Baud Rate 1
    rw UCABR1 @ 0x07: u8 = 0_0 {
        /// USCI A Baud Rate 1
        UCABR1: 0..7 = struct UCA0BR1Field(u8);
    }
    /// USCI A Modulation Control
    rw UCAMCTLW @ 0x08: u16 = 0_0 {
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
    /// USCI A Status Register
    rw UCASTATW @ 0x0a: u8 = 0_0 {
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
    /// USCI A Receive Buffer
    rw UCARXBUF @ 0x0c: u16 = 0_0 {
        /// USCI A Receive Buffer
        UCARXBUF: 0..15 = struct UCA0RXBUFField(u16);
    }
    /// USCI A Transmit Buffer
    rw UCATXBUF @ 0x0e: u16 = 0_0 {
        /// USCI A Transmit Buffer
        UCATXBUF: 0..15 = struct UCA0TXBUFField(u16);
    }
    /// USCI A LIN Control
    rw UCAABCTL @ 0x10: u8 = 0_0 {
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
    /// USCI A IrDA Transmit Control
    rw UCAIRCTL @ 0x12: u16 = 0_0 {
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
    /// USCI A IrDA Transmit Control
    rw UCAIRTCTL @ 0x12: u8 = 0_0 {
        /// USCI A IrDA Transmit Control
        UCAIRTCTL: 0..7 = struct UCA0IRTCTLField(u8);
    }
    /// USCI A IrDA Receive Control
    rw UCAIRRCTL @ 0x13: u8 = 0_0 {
        /// USCI A IrDA Receive Control
        UCAIRRCTL: 0..7 = struct UCA0IRRCTLField(u8);
    }
    /// USCI A Interrupt Enable Register
    rw UCAIE @ 0x1a: u16 = 0_0 {
        /// USCI A Interrupt Enable Register
        UCAIE: 0..15 = struct UCA0IEField(u16);
    }
    /// USCI A Interrupt Flags Register
    rw UCAIFG @ 0x1c: u16 = 0_0 {
        /// USCI A Interrupt Flags Register
        UCAIFG: 0..15 = struct UCA0IFGField(u16);
    }
    /// USCI A Interrupt Enable Register
    rw UCAIE__UART @ 0x1a: u16 = 0_0 {
        /// UART Receive Interrupt Enable
        UCRXIE: 0 = struct UCRXIE(bool);
        /// UART Transmit Interrupt Enable
        UCTXIE: 1 = struct UCTXIE(bool);
        /// UART Start Bit Interrupt Enalble
        UCSTTIE: 2 = struct UCSTTIE(bool);
        /// UART Transmit Complete Interrupt Enable
        UCTXCPTIE: 3 = struct UCTXCPTIE(bool);
    }
    /// USCI A Interrupt Flags Register
    rw UCAIFG__UART @ 0x1c: u16 = 0_0 {
        /// UART Receive Interrupt Flag
        UCRXIFG: 0 = struct UCRXIFG(bool);
        /// UART Transmit Interrupt Flag
        UCTXIFG: 1 = struct UCTXIFG(bool);
        /// UART Start Bit Interrupt Flag
        UCSTTIFG: 2 = struct UCSTTIFG(bool);
        /// UART Transmit Complete Interrupt Flag
        UCTXCPTIFG: 3 = struct UCTXCPTIFG(bool);
    }
    /// USCI A Interrupt Vector Register
    rw UCAIV @ 0x1e: u16 = 0_0 {
        /// USCI A Interrupt Vector Register
        UCAIV: 0..15 = struct UCA0IVField(u16);
    }
}