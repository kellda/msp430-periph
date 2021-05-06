//! DMA

utils::periph! {
    /// DMA
    DMA;
    /// DMA Module Control 0
    rw DMACTL0 @ 0x00: u16 = 0_0 {
        /// DMA channel 0 transfer select bit 0
        DMA0TSEL: 0..4 = enum DMA0TSEL {
            /// DMA channel 0 transfer select 0:  DMA_REQ (sw)
            DMA0TSEL_0 = 0b00000,
            /// DMA channel 0 transfer select 1:  Timer0_A (TA0CCR0.IFG)
            DMA0TSEL_1 = 0b00001,
            /// DMA channel 0 transfer select 2:  Timer0_A (TA0CCR2.IFG)
            DMA0TSEL_2 = 0b00010,
            /// DMA channel 0 transfer select 3:  Timer1_A (TA1CCR0.IFG)
            DMA0TSEL_3 = 0b00011,
            /// DMA channel 0 transfer select 4:  Timer1_A (TA1CCR2.IFG)
            DMA0TSEL_4 = 0b00100,
            /// DMA channel 0 transfer select 5:  TimerB (TB0CCR0.IFG)
            DMA0TSEL_5 = 0b00101,
            /// DMA channel 0 transfer select 6:  TimerB (TB0CCR2.IFG)
            DMA0TSEL_6 = 0b00110,
            /// DMA channel 0 transfer select 7:  Reserved
            DMA0TSEL_7 = 0b00111,
            /// DMA channel 0 transfer select 8:  Reserved
            DMA0TSEL_8 = 0b01000,
            /// DMA channel 0 transfer select 9:  Reserved
            DMA0TSEL_9 = 0b01001,
            /// DMA channel 0 transfer select 10: Reserved
            DMA0TSEL_10 = 0b01010,
            /// DMA channel 0 transfer select 11: Reserved
            DMA0TSEL_11 = 0b01011,
            /// DMA channel 0 transfer select 12: Reserved
            DMA0TSEL_12 = 0b01100,
            /// DMA channel 0 transfer select 13: Reserved
            DMA0TSEL_13 = 0b01101,
            /// DMA channel 0 transfer select 14: RFRXIFG
            DMA0TSEL_14 = 0b01110,
            /// DMA channel 0 transfer select 15: RFTXIFG
            DMA0TSEL_15 = 0b01111,
            /// DMA channel 0 transfer select 16: USCIA0 receive
            DMA0TSEL_16 = 0b10000,
            /// DMA channel 0 transfer select 17: USCIA0 transmit
            DMA0TSEL_17 = 0b10001,
            /// DMA channel 0 transfer select 18: USCIB0 receive
            DMA0TSEL_18 = 0b10010,
            /// DMA channel 0 transfer select 19: USCIB0 transmit
            DMA0TSEL_19 = 0b10011,
            /// DMA channel 0 transfer select 20: Reserved
            DMA0TSEL_20 = 0b10100,
            /// DMA channel 0 transfer select 21: Reserved
            DMA0TSEL_21 = 0b10101,
            /// DMA channel 0 transfer select 22: Reserved
            DMA0TSEL_22 = 0b10110,
            /// DMA channel 0 transfer select 23: Reserved
            DMA0TSEL_23 = 0b10111,
            /// DMA channel 0 transfer select 24: ADC12IFGx
            DMA0TSEL_24 = 0b11000,
            /// DMA channel 0 transfer select 25: Reserved
            DMA0TSEL_25 = 0b11001,
            /// DMA channel 0 transfer select 26: Reserved
            DMA0TSEL_26 = 0b11010,
            /// DMA channel 0 transfer select 27: Reserved
            DMA0TSEL_27 = 0b11011,
            /// DMA channel 0 transfer select 28: Reserved
            DMA0TSEL_28 = 0b11100,
            /// DMA channel 0 transfer select 29: Multiplier ready
            DMA0TSEL_29 = 0b11101,
            /// DMA channel 0 transfer select 30: previous DMA channel DMA2IFG
            DMA0TSEL_30 = 0b11110,
            /// DMA channel 0 transfer select 31: ext. Trigger (DMAE0)
            DMA0TSEL_31 = 0b11111,
        }
        /// DMA channel 1 transfer select bit 0
        DMA1TSEL: 8..12 = enum DMA1TSEL {
            /// DMA channel 1 transfer select 0:  DMA_REQ (sw)
            DMA1TSEL_0 = 0b00000,
            /// DMA channel 1 transfer select 1:  Timer0_A (TA0CCR0.IFG)
            DMA1TSEL_1 = 0b00001,
            /// DMA channel 1 transfer select 2:  Timer0_A (TA0CCR2.IFG)
            DMA1TSEL_2 = 0b00010,
            /// DMA channel 1 transfer select 3:  Timer1_A (TA1CCR0.IFG)
            DMA1TSEL_3 = 0b00011,
            /// DMA channel 1 transfer select 4:  Timer1_A (TA1CCR2.IFG)
            DMA1TSEL_4 = 0b00100,
            /// DMA channel 1 transfer select 5:  TimerB (TB0CCR0.IFG)
            DMA1TSEL_5 = 0b00101,
            /// DMA channel 1 transfer select 6:  TimerB (TB0CCR2.IFG)
            DMA1TSEL_6 = 0b00110,
            /// DMA channel 1 transfer select 7:  Reserved
            DMA1TSEL_7 = 0b00111,
            /// DMA channel 1 transfer select 8:  Reserved
            DMA1TSEL_8 = 0b01000,
            /// DMA channel 1 transfer select 9:  Reserved
            DMA1TSEL_9 = 0b01001,
            /// DMA channel 1 transfer select 10: Reserved
            DMA1TSEL_10 = 0b01010,
            /// DMA channel 1 transfer select 11: Reserved
            DMA1TSEL_11 = 0b01011,
            /// DMA channel 1 transfer select 12: Reserved
            DMA1TSEL_12 = 0b01100,
            /// DMA channel 1 transfer select 13: Reserved
            DMA1TSEL_13 = 0b01101,
            /// DMA channel 1 transfer select 14: RFRXIFG
            DMA1TSEL_14 = 0b01110,
            /// DMA channel 1 transfer select 15: RFTXIFG
            DMA1TSEL_15 = 0b01111,
            /// DMA channel 1 transfer select 16: USCIA0 receive
            DMA1TSEL_16 = 0b10000,
            /// DMA channel 1 transfer select 17: USCIA0 transmit
            DMA1TSEL_17 = 0b10001,
            /// DMA channel 1 transfer select 18: USCIB0 receive
            DMA1TSEL_18 = 0b10010,
            /// DMA channel 1 transfer select 19: USCIB0 transmit
            DMA1TSEL_19 = 0b10011,
            /// DMA channel 1 transfer select 20: Reserved
            DMA1TSEL_20 = 0b10100,
            /// DMA channel 1 transfer select 21: Reserved
            DMA1TSEL_21 = 0b10101,
            /// DMA channel 1 transfer select 22: Reserved
            DMA1TSEL_22 = 0b10110,
            /// DMA channel 1 transfer select 23: Reserved
            DMA1TSEL_23 = 0b10111,
            /// DMA channel 1 transfer select 24: ADC12IFGx
            DMA1TSEL_24 = 0b11000,
            /// DMA channel 1 transfer select 25: Reserved
            DMA1TSEL_25 = 0b11001,
            /// DMA channel 1 transfer select 26: Reserved
            DMA1TSEL_26 = 0b11010,
            /// DMA channel 1 transfer select 27: Reserved
            DMA1TSEL_27 = 0b11011,
            /// DMA channel 1 transfer select 28: Reserved
            DMA1TSEL_28 = 0b11100,
            /// DMA channel 1 transfer select 29: Multiplier ready
            DMA1TSEL_29 = 0b11101,
            /// DMA channel 1 transfer select 30: previous DMA channel DMA0IFG
            DMA1TSEL_30 = 0b11110,
            /// DMA channel 1 transfer select 31: ext. Trigger (DMAE0)
            DMA1TSEL_31 = 0b11111,
        }
    }
    /// DMA Module Control 1
    rw DMACTL1 @ 0x02: u16 = 0_0 {
        /// DMA channel 2 transfer select bit 0
        DMA2TSEL: 0..4 = enum DMA2TSEL {
            /// DMA channel 2 transfer select 0:  DMA_REQ (sw)
            DMA2TSEL_0 = 0b00000,
            /// DMA channel 2 transfer select 1:  Timer0_A (TA0CCR0.IFG)
            DMA2TSEL_1 = 0b00001,
            /// DMA channel 2 transfer select 2:  Timer0_A (TA0CCR2.IFG)
            DMA2TSEL_2 = 0b00010,
            /// DMA channel 2 transfer select 3:  Timer1_A (TA1CCR0.IFG)
            DMA2TSEL_3 = 0b00011,
            /// DMA channel 2 transfer select 4:  Timer1_A (TA1CCR2.IFG)
            DMA2TSEL_4 = 0b00100,
            /// DMA channel 2 transfer select 5:  TimerB (TB0CCR0.IFG)
            DMA2TSEL_5 = 0b00101,
            /// DMA channel 2 transfer select 6:  TimerB (TB0CCR2.IFG)
            DMA2TSEL_6 = 0b00110,
            /// DMA channel 2 transfer select 7:  Reserved
            DMA2TSEL_7 = 0b00111,
            /// DMA channel 2 transfer select 8:  Reserved
            DMA2TSEL_8 = 0b01000,
            /// DMA channel 2 transfer select 9:  Reserved
            DMA2TSEL_9 = 0b01001,
            /// DMA channel 2 transfer select 10: Reserved
            DMA2TSEL_10 = 0b01010,
            /// DMA channel 2 transfer select 11: Reserved
            DMA2TSEL_11 = 0b01011,
            /// DMA channel 2 transfer select 12: Reserved
            DMA2TSEL_12 = 0b01100,
            /// DMA channel 2 transfer select 13: Reserved
            DMA2TSEL_13 = 0b01101,
            /// DMA channel 2 transfer select 14: RFRXIFG
            DMA2TSEL_14 = 0b01110,
            /// DMA channel 2 transfer select 15: RFTXIFG
            DMA2TSEL_15 = 0b01111,
            /// DMA channel 2 transfer select 16: USCIA0 receive
            DMA2TSEL_16 = 0b10000,
            /// DMA channel 2 transfer select 17: USCIA0 transmit
            DMA2TSEL_17 = 0b10001,
            /// DMA channel 2 transfer select 18: USCIB0 receive
            DMA2TSEL_18 = 0b10010,
            /// DMA channel 2 transfer select 19: USCIB0 transmit
            DMA2TSEL_19 = 0b10011,
            /// DMA channel 2 transfer select 20: Reserved
            DMA2TSEL_20 = 0b10100,
            /// DMA channel 2 transfer select 21: Reserved
            DMA2TSEL_21 = 0b10101,
            /// DMA channel 2 transfer select 22: Reserved
            DMA2TSEL_22 = 0b10110,
            /// DMA channel 2 transfer select 23: Reserved
            DMA2TSEL_23 = 0b10111,
            /// DMA channel 2 transfer select 24: ADC12IFGx
            DMA2TSEL_24 = 0b11000,
            /// DMA channel 2 transfer select 25: Reserved
            DMA2TSEL_25 = 0b11001,
            /// DMA channel 2 transfer select 26: Reserved
            DMA2TSEL_26 = 0b11010,
            /// DMA channel 2 transfer select 27: Reserved
            DMA2TSEL_27 = 0b11011,
            /// DMA channel 2 transfer select 28: Reserved
            DMA2TSEL_28 = 0b11100,
            /// DMA channel 2 transfer select 29: Multiplier ready
            DMA2TSEL_29 = 0b11101,
            /// DMA channel 2 transfer select 30: previous DMA channel DMA1IFG
            DMA2TSEL_30 = 0b11110,
            /// DMA channel 2 transfer select 31: ext. Trigger (DMAE0)
            DMA2TSEL_31 = 0b11111,
        }
    }
    /// DMA Module Control 2
    rw DMACTL2 @ 0x04: u16 = 0_0 {
        /// DMA Module Control 2
        DMACTL2: 0..15 = struct DMACTL2Field(u16);
    }
    /// DMA Module Control 3
    rw DMACTL3 @ 0x06: u16 = 0_0 {
        /// DMA Module Control 3
        DMACTL3: 0..15 = struct DMACTL3Field(u16);
    }
    /// DMA Module Control 4
    rw DMACTL4 @ 0x08: u16 = 0_0 {
        /// Enable NMI interruption of DMA
        ENNMI: 0 = struct ENNMI(bool);
        /// Round-Robin DMA channel priorities
        ROUNDROBIN: 1 = struct ROUNDROBIN(bool);
        /// Inhibited DMA transfers during read-modify-write CPU operations
        DMARMWDIS: 2 = struct DMARMWDIS(bool);
    }
    /// DMA Interrupt Vector Word
    rw DMAIV @ 0x0e: u16 = 0_0 {
        /// DMA Interrupt Vector Word
        DMAIV: 0..15 = struct DMAIVField(u16);
    }
    /// DMA Channel 0 Control
    rw DMA0CTL @ 0x10: u16 = 0_0 {
        /// Initiate DMA transfer with DMATSEL
        DMA0CTL_DMAREQ: 0 = struct DMA0CTL_DMAREQ(bool);
        /// DMA transfer aborted by NMI
        DMA0CTL_DMAABORT: 1 = struct DMA0CTL_DMAABORT(bool);
        /// DMA interrupt enable
        DMA0CTL_DMAIE: 2 = struct DMA0CTL_DMAIE(bool);
        /// DMA interrupt flag
        DMA0CTL_DMAIFG: 3 = struct DMA0CTL_DMAIFG(bool);
        /// DMA enable
        DMA0CTL_DMAEN: 4 = struct DMA0CTL_DMAEN(bool);
        /// DMA level sensitive trigger select
        DMA0CTL_DMALEVEL: 5 = struct DMA0CTL_DMALEVEL(bool);
        /// DMA source byte
        DMA0CTL_DMASRCBYTE: 6 = struct DMA0CTL_DMASRCBYTE(bool);
        /// DMA destination byte
        DMA0CTL_DMADSTBYTE: 7 = struct DMA0CTL_DMADSTBYTE(bool);
        /// DMA source increment bit 0
        DMA0CTL_DMASRCINCR: 8..9 = enum DMA0CTL_DMASRCINCR {
            /// DMA source increment 0: source address unchanged
            DMASRCINCR_0 = 0b00,
            /// DMA source increment 1: source address unchanged
            DMASRCINCR_1 = 0b01,
            /// DMA source increment 2: source address decremented
            DMASRCINCR_2 = 0b10,
            /// DMA source increment 3: source address incremented
            DMASRCINCR_3 = 0b11,
        }
        /// DMA destination increment bit 0
        DMA0CTL_DMADSTINCR: 10..11 = enum DMA0CTL_DMADSTINCR {
            /// DMA destination increment 0: destination address unchanged
            DMADSTINCR_0 = 0b00,
            /// DMA destination increment 1: destination address unchanged
            DMADSTINCR_1 = 0b01,
            /// DMA destination increment 2: destination address decremented
            DMADSTINCR_2 = 0b10,
            /// DMA destination increment 3: destination address incremented
            DMADSTINCR_3 = 0b11,
        }
        /// DMA transfer mode bit 0
        DMA0CTL_DMADT: 12..14 = enum DMA0CTL_DMADT {
            /// DMA transfer mode 0: Single transfer
            DMADT_0 = 0b000,
            /// DMA transfer mode 1: Block transfer
            DMADT_1 = 0b001,
            /// DMA transfer mode 2: Burst-Block transfer
            DMADT_2 = 0b010,
            /// DMA transfer mode 3: Burst-Block transfer
            DMADT_3 = 0b011,
            /// DMA transfer mode 4: Repeated Single transfer
            DMADT_4 = 0b100,
            /// DMA transfer mode 5: Repeated Block transfer
            DMADT_5 = 0b101,
            /// DMA transfer mode 6: Repeated Burst-Block transfer
            DMADT_6 = 0b110,
            /// DMA transfer mode 7: Repeated Burst-Block transfer
            DMADT_7 = 0b111,
        }
    }
    /// DMA Channel 0 Source Address
    rw DMA0SA @ 0x12: u32 = 0_0 {
        /// DMA Channel 0 Source Address
        DMA0SA: 0..31 = struct DMA0SAField(u32);
    }
    /// DMA Channel 0 Destination Address
    rw DMA0DA @ 0x16: u32 = 0_0 {
        /// DMA Channel 0 Destination Address
        DMA0DA: 0..31 = struct DMA0DAField(u32);
    }
    /// DMA Channel 0 Transfer Size
    rw DMA0SZ @ 0x1a: u16 = 0_0 {
        /// DMA Channel 0 Transfer Size
        DMA0SZ: 0..15 = struct DMA0SZField(u16);
    }
    /// DMA Channel 1 Control
    rw DMA1CTL @ 0x20: u16 = 0_0 {
        /// Initiate DMA transfer with DMATSEL
        DMA1CTL_DMAREQ: 0 = struct DMA1CTL_DMAREQ(bool);
        /// DMA transfer aborted by NMI
        DMA1CTL_DMAABORT: 1 = struct DMA1CTL_DMAABORT(bool);
        /// DMA interrupt enable
        DMA1CTL_DMAIE: 2 = struct DMA1CTL_DMAIE(bool);
        /// DMA interrupt flag
        DMA1CTL_DMAIFG: 3 = struct DMA1CTL_DMAIFG(bool);
        /// DMA enable
        DMA1CTL_DMAEN: 4 = struct DMA1CTL_DMAEN(bool);
        /// DMA level sensitive trigger select
        DMA1CTL_DMALEVEL: 5 = struct DMA1CTL_DMALEVEL(bool);
        /// DMA source byte
        DMA1CTL_DMASRCBYTE: 6 = struct DMA1CTL_DMASRCBYTE(bool);
        /// DMA destination byte
        DMA1CTL_DMADSTBYTE: 7 = struct DMA1CTL_DMADSTBYTE(bool);
        /// DMA source increment bit 0
        DMA1CTL_DMASRCINCR: 8..9 = enum DMA1CTL_DMASRCINCR {
            /// DMA source increment 0: source address unchanged
            DMASRCINCR_0 = 0b00,
            /// DMA source increment 1: source address unchanged
            DMASRCINCR_1 = 0b01,
            /// DMA source increment 2: source address decremented
            DMASRCINCR_2 = 0b10,
            /// DMA source increment 3: source address incremented
            DMASRCINCR_3 = 0b11,
        }
        /// DMA destination increment bit 0
        DMA1CTL_DMADSTINCR: 10..11 = enum DMA1CTL_DMADSTINCR {
            /// DMA destination increment 0: destination address unchanged
            DMADSTINCR_0 = 0b00,
            /// DMA destination increment 1: destination address unchanged
            DMADSTINCR_1 = 0b01,
            /// DMA destination increment 2: destination address decremented
            DMADSTINCR_2 = 0b10,
            /// DMA destination increment 3: destination address incremented
            DMADSTINCR_3 = 0b11,
        }
        /// DMA transfer mode bit 0
        DMA1CTL_DMADT: 12..14 = enum DMA1CTL_DMADT {
            /// DMA transfer mode 0: Single transfer
            DMADT_0 = 0b000,
            /// DMA transfer mode 1: Block transfer
            DMADT_1 = 0b001,
            /// DMA transfer mode 2: Burst-Block transfer
            DMADT_2 = 0b010,
            /// DMA transfer mode 3: Burst-Block transfer
            DMADT_3 = 0b011,
            /// DMA transfer mode 4: Repeated Single transfer
            DMADT_4 = 0b100,
            /// DMA transfer mode 5: Repeated Block transfer
            DMADT_5 = 0b101,
            /// DMA transfer mode 6: Repeated Burst-Block transfer
            DMADT_6 = 0b110,
            /// DMA transfer mode 7: Repeated Burst-Block transfer
            DMADT_7 = 0b111,
        }
    }
    /// DMA Channel 1 Source Address
    rw DMA1SA @ 0x22: u32 = 0_0 {
        /// DMA Channel 1 Source Address
        DMA1SA: 0..31 = struct DMA1SAField(u32);
    }
    /// DMA Channel 1 Destination Address
    rw DMA1DA @ 0x26: u32 = 0_0 {
        /// DMA Channel 1 Destination Address
        DMA1DA: 0..31 = struct DMA1DAField(u32);
    }
    /// DMA Channel 1 Transfer Size
    rw DMA1SZ @ 0x2a: u16 = 0_0 {
        /// DMA Channel 1 Transfer Size
        DMA1SZ: 0..15 = struct DMA1SZField(u16);
    }
    /// DMA Channel 2 Control
    rw DMA2CTL @ 0x30: u16 = 0_0 {
        /// Initiate DMA transfer with DMATSEL
        DMA2CTL_DMAREQ: 0 = struct DMA2CTL_DMAREQ(bool);
        /// DMA transfer aborted by NMI
        DMA2CTL_DMAABORT: 1 = struct DMA2CTL_DMAABORT(bool);
        /// DMA interrupt enable
        DMA2CTL_DMAIE: 2 = struct DMA2CTL_DMAIE(bool);
        /// DMA interrupt flag
        DMA2CTL_DMAIFG: 3 = struct DMA2CTL_DMAIFG(bool);
        /// DMA enable
        DMA2CTL_DMAEN: 4 = struct DMA2CTL_DMAEN(bool);
        /// DMA level sensitive trigger select
        DMA2CTL_DMALEVEL: 5 = struct DMA2CTL_DMALEVEL(bool);
        /// DMA source byte
        DMA2CTL_DMASRCBYTE: 6 = struct DMA2CTL_DMASRCBYTE(bool);
        /// DMA destination byte
        DMA2CTL_DMADSTBYTE: 7 = struct DMA2CTL_DMADSTBYTE(bool);
        /// DMA source increment bit 0
        DMA2CTL_DMASRCINCR: 8..9 = enum DMA2CTL_DMASRCINCR {
            /// DMA source increment 0: source address unchanged
            DMASRCINCR_0 = 0b00,
            /// DMA source increment 1: source address unchanged
            DMASRCINCR_1 = 0b01,
            /// DMA source increment 2: source address decremented
            DMASRCINCR_2 = 0b10,
            /// DMA source increment 3: source address incremented
            DMASRCINCR_3 = 0b11,
        }
        /// DMA destination increment bit 0
        DMA2CTL_DMADSTINCR: 10..11 = enum DMA2CTL_DMADSTINCR {
            /// DMA destination increment 0: destination address unchanged
            DMADSTINCR_0 = 0b00,
            /// DMA destination increment 1: destination address unchanged
            DMADSTINCR_1 = 0b01,
            /// DMA destination increment 2: destination address decremented
            DMADSTINCR_2 = 0b10,
            /// DMA destination increment 3: destination address incremented
            DMADSTINCR_3 = 0b11,
        }
        /// DMA transfer mode bit 0
        DMA2CTL_DMADT: 12..14 = enum DMA2CTL_DMADT {
            /// DMA transfer mode 0: Single transfer
            DMADT_0 = 0b000,
            /// DMA transfer mode 1: Block transfer
            DMADT_1 = 0b001,
            /// DMA transfer mode 2: Burst-Block transfer
            DMADT_2 = 0b010,
            /// DMA transfer mode 3: Burst-Block transfer
            DMADT_3 = 0b011,
            /// DMA transfer mode 4: Repeated Single transfer
            DMADT_4 = 0b100,
            /// DMA transfer mode 5: Repeated Block transfer
            DMADT_5 = 0b101,
            /// DMA transfer mode 6: Repeated Burst-Block transfer
            DMADT_6 = 0b110,
            /// DMA transfer mode 7: Repeated Burst-Block transfer
            DMADT_7 = 0b111,
        }
    }
    /// DMA Channel 2 Source Address
    rw DMA2SA @ 0x32: u32 = 0_0 {
        /// DMA Channel 2 Source Address
        DMA2SA: 0..31 = struct DMA2SAField(u32);
    }
    /// DMA Channel 2 Destination Address
    rw DMA2DA @ 0x36: u32 = 0_0 {
        /// DMA Channel 2 Destination Address
        DMA2DA: 0..31 = struct DMA2DAField(u32);
    }
    /// DMA Channel 2 Transfer Size
    rw DMA2SZ @ 0x3a: u16 = 0_0 {
        /// DMA Channel 2 Transfer Size
        DMA2SZ: 0..15 = struct DMA2SZField(u16);
    }
}
