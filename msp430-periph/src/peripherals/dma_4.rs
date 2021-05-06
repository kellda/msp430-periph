//! DMA

utils::periph! {
    /// DMA
    DMA;
    /// DMA Module Control 0
    rw DMACTL0 @ 0x00: u16 = 0_0 {
        /// DMA channel 0 transfer select bit 0
        DMA0TSEL: 0..3 = enum DMA0TSEL {
            /// DMA channel 0 transfer select 0:  DMA_REQ (sw)
            DMA0TSEL_0 = 0b0000,
            /// DMA channel 0 transfer select 1:  Timer_A (TACCR2.IFG)
            DMA0TSEL_1 = 0b0001,
            /// DMA channel 0 transfer select 2:  Timer_B (TBCCR2.IFG)
            DMA0TSEL_2 = 0b0010,
            /// DMA channel 0 transfer select 3:  USCIA0 receive
            DMA0TSEL_3 = 0b0011,
            /// DMA channel 0 transfer select 4:  USCIA0 transmit
            DMA0TSEL_4 = 0b0100,
            /// DMA channel 0 transfer select 5:  Reserved
            DMA0TSEL_5 = 0b0101,
            /// DMA channel 0 transfer select 6:  SD16 (SD16IFG)
            DMA0TSEL_6 = 0b0110,
            /// DMA channel 0 transfer select 7:  Timer_A (TACCR0.IFG)
            DMA0TSEL_7 = 0b0111,
            /// DMA channel 0 transfer select 8:  Timer_B (TBCCR0.IFG)
            DMA0TSEL_8 = 0b1000,
            /// DMA channel 0 transfer select 9:  USCIA1 receive
            DMA0TSEL_9 = 0b1001,
            /// DMA channel 0 transfer select 10: USCIA1 transmit
            DMA0TSEL_10 = 0b1010,
            /// DMA channel 0 transfer select 11: Multiplier ready
            DMA0TSEL_11 = 0b1011,
            /// DMA channel 0 transfer select 12: USCIB0 receive
            DMA0TSEL_12 = 0b1100,
            /// DMA channel 0 transfer select 13: USCIB0 transmit
            DMA0TSEL_13 = 0b1101,
            /// DMA channel 0 transfer select 14: previous DMA channel DMA2IFG
            DMA0TSEL_14 = 0b1110,
            /// DMA channel 0 transfer select 15: ext. Trigger (DMAE0)
            DMA0TSEL_15 = 0b1111,
        }
        /// DMA channel 1 transfer select bit 0
        DMA1TSEL: 4..7 = enum DMA1TSEL {
            /// DMA channel 1 transfer select 0:  DMA_REQ
            DMA1TSEL_0 = 0b0000,
            /// DMA channel 1 transfer select 1:  Timer_A CCRIFG.2
            DMA1TSEL_1 = 0b0001,
            /// DMA channel 1 transfer select 2:  Timer_B CCRIFG.2
            DMA1TSEL_2 = 0b0010,
            /// DMA channel 1 transfer select 3:  USCIA0 receive
            DMA1TSEL_3 = 0b0011,
            /// DMA channel 1 transfer select 4:  USCIA0 transmit
            DMA1TSEL_4 = 0b0100,
            /// DMA channel 1 transfer select 5:  Reserved
            DMA1TSEL_5 = 0b0101,
            /// DMA channel 1 transfer select 6:  SD16 (SD16IFG)
            DMA1TSEL_6 = 0b0110,
            /// DMA channel 1 transfer select 7:  Timer_A (TACCR0.IFG)
            DMA1TSEL_7 = 0b0111,
            /// DMA channel 1 transfer select 8:  Timer_B (TBCCR0.IFG)
            DMA1TSEL_8 = 0b1000,
            /// DMA channel 1 transfer select 9:  USCIA1 receive
            DMA1TSEL_9 = 0b1001,
            /// DMA channel 1 transfer select 10: USCIA1 transmit
            DMA1TSEL_10 = 0b1010,
            /// DMA channel 1 transfer select 11: Multiplier ready
            DMA1TSEL_11 = 0b1011,
            /// DMA channel 1 transfer select 12: USCIB0 receive
            DMA1TSEL_12 = 0b1100,
            /// DMA channel 1 transfer select 13: USCIB0 transmit
            DMA1TSEL_13 = 0b1101,
            /// DMA channel 1 transfer select 14: previous DMA channel DMA0IFG
            DMA1TSEL_14 = 0b1110,
            /// DMA channel 1 transfer select 15: ext. Trigger (DMAE0)
            DMA1TSEL_15 = 0b1111,
        }
        /// DMA channel 2 transfer select bit 0
        DMA2TSEL: 8..11 = enum DMA2TSEL {
            /// DMA channel 2 transfer select 0:  DMA_REQ
            DMA2TSEL_0 = 0b0000,
            /// DMA channel 2 transfer select 1:  Timer_A CCRIFG.2
            DMA2TSEL_1 = 0b0001,
            /// DMA channel 2 transfer select 2:  Timer_B CCRIFG.2
            DMA2TSEL_2 = 0b0010,
            /// DMA channel 2 transfer select 3:  USCIA0 receive
            DMA2TSEL_3 = 0b0011,
            /// DMA channel 2 transfer select 4:  USCIA0 transmit
            DMA2TSEL_4 = 0b0100,
            /// DMA channel 2 transfer select 5:  Reserved
            DMA2TSEL_5 = 0b0101,
            /// DMA channel 2 transfer select 6:  SD16 (SD16IFG)
            DMA2TSEL_6 = 0b0110,
            /// DMA channel 2 transfer select 7:  Timer_A (TACCR0.IFG)
            DMA2TSEL_7 = 0b0111,
            /// DMA channel 2 transfer select 8:  Timer_B (TBCCR0.IFG)
            DMA2TSEL_8 = 0b1000,
            /// DMA channel 2 transfer select 9:  USCIA1 receive
            DMA2TSEL_9 = 0b1001,
            /// DMA channel 2 transfer select 10: USCIA1 transmit
            DMA2TSEL_10 = 0b1010,
            /// DMA channel 2 transfer select 11: Multiplier ready
            DMA2TSEL_11 = 0b1011,
            /// DMA channel 2 transfer select 12: USCIB0 receive
            DMA2TSEL_12 = 0b1100,
            /// DMA channel 2 transfer select 13: USCIB0 transmit
            DMA2TSEL_13 = 0b1101,
            /// DMA channel 2 transfer select 14: previous DMA channel DMA1IFG
            DMA2TSEL_14 = 0b1110,
            /// DMA channel 2 transfer select 15: ext. Trigger (DMAE0)
            DMA2TSEL_15 = 0b1111,
        }
    }
    /// DMA Module Control 1
    rw DMACTL1 @ 0x02: u16 = 0_0 {
        /// Enable NMI interruption of DMA
        ENNMI: 0 = struct ENNMI(bool);
        /// Round-Robin DMA channel priorities
        ROUNDROBIN: 1 = struct ROUNDROBIN(bool);
        /// DMA transfer on instruction fetch
        DMAONFETCH: 2 = struct DMAONFETCH(bool);
    }
    /// DMA Interrupt Vector Word
    rw DMAIV @ 0x04: u16 = 0_0 {
        /// DMA Interrupt Vector Word
        DMAIV: 0..15 = struct DMAIVField(u16);
    }
    /// DMA Channel 0 Control
    rw DMA0CTL @ 0xae: u16 = 0_0 {
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
            /// DMA transfer mode 0: single
            DMADT_0 = 0b000,
            /// DMA transfer mode 1: block
            DMADT_1 = 0b001,
            /// DMA transfer mode 2: interleaved
            DMADT_2 = 0b010,
            /// DMA transfer mode 3: interleaved
            DMADT_3 = 0b011,
            /// DMA transfer mode 4: single
            DMADT_4 = 0b100,
            /// DMA transfer mode 5: block
            DMADT_5 = 0b101,
            /// DMA transfer mode 6: interleaved
            DMADT_6 = 0b110,
            /// DMA transfer mode 7: interleaved
            DMADT_7 = 0b111,
        }
    }
    /// DMA Channel 1 Control
    rw DMA1CTL @ 0xba: u16 = 0_0 {
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
            /// DMA transfer mode 0: single
            DMADT_0 = 0b000,
            /// DMA transfer mode 1: block
            DMADT_1 = 0b001,
            /// DMA transfer mode 2: interleaved
            DMADT_2 = 0b010,
            /// DMA transfer mode 3: interleaved
            DMADT_3 = 0b011,
            /// DMA transfer mode 4: single
            DMADT_4 = 0b100,
            /// DMA transfer mode 5: block
            DMADT_5 = 0b101,
            /// DMA transfer mode 6: interleaved
            DMADT_6 = 0b110,
            /// DMA transfer mode 7: interleaved
            DMADT_7 = 0b111,
        }
    }
    /// DMA Channel 2 Control
    rw DMA2CTL @ 0xc6: u16 = 0_0 {
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
            /// DMA transfer mode 0: single
            DMADT_0 = 0b000,
            /// DMA transfer mode 1: block
            DMADT_1 = 0b001,
            /// DMA transfer mode 2: interleaved
            DMADT_2 = 0b010,
            /// DMA transfer mode 3: interleaved
            DMADT_3 = 0b011,
            /// DMA transfer mode 4: single
            DMADT_4 = 0b100,
            /// DMA transfer mode 5: block
            DMADT_5 = 0b101,
            /// DMA transfer mode 6: interleaved
            DMADT_6 = 0b110,
            /// DMA transfer mode 7: interleaved
            DMADT_7 = 0b111,
        }
    }
    /// DMA Channel 0 Source Address
    rw DMA0SA @ 0xb0: u32 = 0_0 {
        /// DMA Channel 0 Source Address
        DMA0SA: 0..31 = struct DMA0SAField(u32);
    }
    /// DMA Channel 0 Source Address
    rw DMA0SAL @ 0xb0: u16 = 0_0 {
        /// DMA Channel 0 Source Address
        DMA0SAL: 0..15 = struct DMA0SALField(u16);
    }
    /// DMA Channel 0 Destination Address
    rw DMA0DA @ 0xb4: u32 = 0_0 {
        /// DMA Channel 0 Destination Address
        DMA0DA: 0..31 = struct DMA0DAField(u32);
    }
    /// DMA Channel 0 Destination Address
    rw DMA0DAL @ 0xb4: u16 = 0_0 {
        /// DMA Channel 0 Destination Address
        DMA0DAL: 0..15 = struct DMA0DALField(u16);
    }
    /// DMA Channel 0 Transfer Size
    rw DMA0SZ @ 0xb8: u16 = 0_0 {
        /// DMA Channel 0 Transfer Size
        DMA0SZ: 0..15 = struct DMA0SZField(u16);
    }
    /// DMA Channel 1 Source Address
    rw DMA1SA @ 0xbc: u32 = 0_0 {
        /// DMA Channel 1 Source Address
        DMA1SA: 0..31 = struct DMA1SAField(u32);
    }
    /// DMA Channel 1 Source Address
    rw DMA1SAL @ 0xbc: u16 = 0_0 {
        /// DMA Channel 1 Source Address
        DMA1SAL: 0..15 = struct DMA1SALField(u16);
    }
    /// DMA Channel 1 Destination Address
    rw DMA1DA @ 0xc0: u32 = 0_0 {
        /// DMA Channel 1 Destination Address
        DMA1DA: 0..31 = struct DMA1DAField(u32);
    }
    /// DMA Channel 1 Destination Address
    rw DMA1DAL @ 0xc0: u16 = 0_0 {
        /// DMA Channel 1 Destination Address
        DMA1DAL: 0..15 = struct DMA1DALField(u16);
    }
    /// DMA Channel 1 Transfer Size
    rw DMA1SZ @ 0xc4: u16 = 0_0 {
        /// DMA Channel 1 Transfer Size
        DMA1SZ: 0..15 = struct DMA1SZField(u16);
    }
    /// DMA Channel 2 Source Address
    rw DMA2SA @ 0xc8: u32 = 0_0 {
        /// DMA Channel 2 Source Address
        DMA2SA: 0..31 = struct DMA2SAField(u32);
    }
    /// DMA Channel 2 Source Address
    rw DMA2SAL @ 0xc8: u16 = 0_0 {
        /// DMA Channel 2 Source Address
        DMA2SAL: 0..15 = struct DMA2SALField(u16);
    }
    /// DMA Channel 2 Destination Address
    rw DMA2DA @ 0xcc: u32 = 0_0 {
        /// DMA Channel 2 Destination Address
        DMA2DA: 0..31 = struct DMA2DAField(u32);
    }
    /// DMA Channel 2 Destination Address
    rw DMA2DAL @ 0xcc: u16 = 0_0 {
        /// DMA Channel 2 Destination Address
        DMA2DAL: 0..15 = struct DMA2DALField(u16);
    }
    /// DMA Channel 2 Transfer Size
    rw DMA2SZ @ 0xd0: u16 = 0_0 {
        /// DMA Channel 2 Transfer Size
        DMA2SZ: 0..15 = struct DMA2SZField(u16);
    }
}
