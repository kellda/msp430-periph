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
            /// DMA channel 0 transfer select 3:  UART0/I2C receive
            DMA0TSEL_3 = 0b0011,
            /// DMA channel 0 transfer select 4:  UART0/I2C transmit
            DMA0TSEL_4 = 0b0100,
            /// DMA channel 0 transfer select 5:  DAC12_0CTL.DAC12IFG
            DMA0TSEL_5 = 0b0101,
            /// DMA channel 0 transfer select 6:  ADC12 (ADC12IFG)
            DMA0TSEL_6 = 0b0110,
            /// DMA channel 0 transfer select 7:  Timer_A (TACCR0.IFG)
            DMA0TSEL_7 = 0b0111,
            /// DMA channel 0 transfer select 8:  Timer_B (TBCCR0.IFG)
            DMA0TSEL_8 = 0b1000,
            /// DMA channel 0 transfer select 9:  UART1 receive
            DMA0TSEL_9 = 0b1001,
            /// DMA channel 0 transfer select 10: UART1 transmit
            DMA0TSEL_10 = 0b1010,
            /// DMA channel 0 transfer select 11: Multiplier ready
            DMA0TSEL_11 = 0b1011,
            /// DMA channel 0 transfer select 14: previous DMA channel DMA2IFG
            DMA0TSEL_14 = 0b1110,
            /// DMA channel 0 transfer select 15: ext. Trigger (DMAE0)
            DMA0TSEL_15 = 0b1111,
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
    /// DMA Channel 0 Control
    rw DMA0CTL @ 0xbe: u16 = 0_0 {
        /// Initiate DMA transfer with DMATSEL
        DMAREQ: 0 = struct DMAREQ(bool);
        /// DMA transfer aborted by NMI
        DMAABORT: 1 = struct DMAABORT(bool);
        /// DMA interrupt enable
        DMAIE: 2 = struct DMAIE(bool);
        /// DMA interrupt flag
        DMAIFG: 3 = struct DMAIFG(bool);
        /// DMA enable
        DMAEN: 4 = struct DMAEN(bool);
        /// DMA level sensitive trigger select
        DMALEVEL: 5 = struct DMALEVEL(bool);
        /// DMA source byte
        DMASRCBYTE: 6 = struct DMASRCBYTE(bool);
        /// DMA destination byte
        DMADSTBYTE: 7 = struct DMADSTBYTE(bool);
        /// DMA source increment bit 0
        DMASRCINCR: 8..9 = enum DMASRCINCR {
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
        DMADSTINCR: 10..11 = enum DMADSTINCR {
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
        DMADT: 12..14 = enum DMADT {
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
    rw DMA0SA @ 0xc0: u16 = 0_0 {
        /// DMA Channel 0 Source Address
        DMA0SA: 0..15 = struct DMA0SAField(u16);
    }
    /// DMA Channel 0 Destination Address
    rw DMA0DA @ 0xc2: u16 = 0_0 {
        /// DMA Channel 0 Destination Address
        DMA0DA: 0..15 = struct DMA0DAField(u16);
    }
    /// DMA Channel 0 Transfer Size
    rw DMA0SZ @ 0xc4: u16 = 0_0 {
        /// DMA Channel 0 Transfer Size
        DMA0SZ: 0..15 = struct DMA0SZField(u16);
    }
}
