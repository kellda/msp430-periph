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
            /// DMA channel 0 transfer select 5:  Timer2_A (TA2CCR0.IFG)
            DMA0TSEL_5 = 0b00101,
            /// DMA channel 0 transfer select 6:  Timer2_A (TA2CCR2.IFG)
            DMA0TSEL_6 = 0b00110,
            /// DMA channel 0 transfer select 7:  TimerB0 (TB0CCR0.IFG)
            DMA0TSEL_7 = 0b00111,
            /// DMA channel 0 transfer select 8:  TimerB0 (TB0CCR2.IFG)
            DMA0TSEL_8 = 0b01000,
            /// DMA channel 0 transfer select 9:  Reserved
            DMA0TSEL_9 = 0b01001,
            /// DMA channel 0 transfer select 10: Reserved
            DMA0TSEL_10 = 0b01010,
            /// DMA channel 0 transfer select 11: Reserved
            DMA0TSEL_11 = 0b01011,
            /// DMA channel 0 transfer select 12: USCIA2 receive
            DMA0TSEL_12 = 0b01100,
            /// DMA channel 0 transfer select 13: USCIA2 transmit
            DMA0TSEL_13 = 0b01101,
            /// DMA channel 0 transfer select 14: USCIB2 receive
            DMA0TSEL_14 = 0b01110,
            /// DMA channel 0 transfer select 15: USCIB2 transmit
            DMA0TSEL_15 = 0b01111,
            /// DMA channel 0 transfer select 16: USCIA0 receive
            DMA0TSEL_16 = 0b10000,
            /// DMA channel 0 transfer select 17: USCIA0 transmit
            DMA0TSEL_17 = 0b10001,
            /// DMA channel 0 transfer select 18: USCIB0 receive
            DMA0TSEL_18 = 0b10010,
            /// DMA channel 0 transfer select 19: USCIB0 transmit
            DMA0TSEL_19 = 0b10011,
            /// DMA channel 0 transfer select 20: USCIA1 receive
            DMA0TSEL_20 = 0b10100,
            /// DMA channel 0 transfer select 21: USCIA1 transmit
            DMA0TSEL_21 = 0b10101,
            /// DMA channel 0 transfer select 22: USCIB1 receive
            DMA0TSEL_22 = 0b10110,
            /// DMA channel 0 transfer select 23: USCIB1 transmit
            DMA0TSEL_23 = 0b10111,
            /// DMA channel 0 transfer select 24: ADC12IFGx
            DMA0TSEL_24 = 0b11000,
            /// DMA channel 0 transfer select 25: DAC12_0IFG
            DMA0TSEL_25 = 0b11001,
            /// DMA channel 0 transfer select 26: DAC12_1IFG
            DMA0TSEL_26 = 0b11010,
            /// DMA channel 0 transfer select 29: Multiplier ready
            DMA0TSEL_29 = 0b11101,
            /// DMA channel 0 transfer select 30: previous DMA channel DMA5IFG
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
            /// DMA channel 1 transfer select 5:  Timer2_A (TA2CCR0.IFG)
            DMA1TSEL_5 = 0b00101,
            /// DMA channel 1 transfer select 6:  Timer2_A (TA2CCR2.IFG)
            DMA1TSEL_6 = 0b00110,
            /// DMA channel 1 transfer select 7:  TimerB0 (TB0CCR0.IFG)
            DMA1TSEL_7 = 0b00111,
            /// DMA channel 1 transfer select 8:  TimerB0 (TB0CCR2.IFG)
            DMA1TSEL_8 = 0b01000,
            /// DMA channel 1 transfer select 9:  Reserved
            DMA1TSEL_9 = 0b01001,
            /// DMA channel 1 transfer select 10: Reserved
            DMA1TSEL_10 = 0b01010,
            /// DMA channel 1 transfer select 11: Reserved
            DMA1TSEL_11 = 0b01011,
            /// DMA channel 1 transfer select 12: USCIA2 receive
            DMA1TSEL_12 = 0b01100,
            /// DMA channel 1 transfer select 13: USCIA2 transmit
            DMA1TSEL_13 = 0b01101,
            /// DMA channel 1 transfer select 14: USCIB2 receive
            DMA1TSEL_14 = 0b01110,
            /// DMA channel 1 transfer select 15: USCIB2 transmit
            DMA1TSEL_15 = 0b01111,
            /// DMA channel 1 transfer select 16: USCIA0 receive
            DMA1TSEL_16 = 0b10000,
            /// DMA channel 1 transfer select 17: USCIA0 transmit
            DMA1TSEL_17 = 0b10001,
            /// DMA channel 1 transfer select 18: USCIB0 receive
            DMA1TSEL_18 = 0b10010,
            /// DMA channel 1 transfer select 19: USCIB0 transmit
            DMA1TSEL_19 = 0b10011,
            /// DMA channel 1 transfer select 20: USCIA1 receive
            DMA1TSEL_20 = 0b10100,
            /// DMA channel 1 transfer select 21: USCIA1 transmit
            DMA1TSEL_21 = 0b10101,
            /// DMA channel 1 transfer select 22: USCIB1 receive
            DMA1TSEL_22 = 0b10110,
            /// DMA channel 1 transfer select 23: USCIB1 transmit
            DMA1TSEL_23 = 0b10111,
            /// DMA channel 1 transfer select 24: ADC12IFGx
            DMA1TSEL_24 = 0b11000,
            /// DMA channel 1 transfer select 25: DAC12_0IFG
            DMA1TSEL_25 = 0b11001,
            /// DMA channel 1 transfer select 26: DAC12_1IFG
            DMA1TSEL_26 = 0b11010,
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
            /// DMA channel 2 transfer select 5:  Timer2_A (TA2CCR0.IFG)
            DMA2TSEL_5 = 0b00101,
            /// DMA channel 2 transfer select 6:  Timer2_A (TA2CCR2.IFG)
            DMA2TSEL_6 = 0b00110,
            /// DMA channel 2 transfer select 7:  TimerB0 (TB0CCR0.IFG)
            DMA2TSEL_7 = 0b00111,
            /// DMA channel 2 transfer select 8:  TimerB0 (TB0CCR2.IFG)
            DMA2TSEL_8 = 0b01000,
            /// DMA channel 2 transfer select 9:  Reserved
            DMA2TSEL_9 = 0b01001,
            /// DMA channel 2 transfer select 10: Reserved
            DMA2TSEL_10 = 0b01010,
            /// DMA channel 2 transfer select 11: Reserved
            DMA2TSEL_11 = 0b01011,
            /// DMA channel 2 transfer select 12: USCIA2 receive
            DMA2TSEL_12 = 0b01100,
            /// DMA channel 2 transfer select 13: USCIA2 transmit
            DMA2TSEL_13 = 0b01101,
            /// DMA channel 2 transfer select 14: USCIB2 receive
            DMA2TSEL_14 = 0b01110,
            /// DMA channel 2 transfer select 15: USCIB2 transmit
            DMA2TSEL_15 = 0b01111,
            /// DMA channel 2 transfer select 16: USCIA0 receive
            DMA2TSEL_16 = 0b10000,
            /// DMA channel 2 transfer select 17: USCIA0 transmit
            DMA2TSEL_17 = 0b10001,
            /// DMA channel 2 transfer select 18: USCIB0 receive
            DMA2TSEL_18 = 0b10010,
            /// DMA channel 2 transfer select 19: USCIB0 transmit
            DMA2TSEL_19 = 0b10011,
            /// DMA channel 2 transfer select 20: USCIA1 receive
            DMA2TSEL_20 = 0b10100,
            /// DMA channel 2 transfer select 21: USCIA1 transmit
            DMA2TSEL_21 = 0b10101,
            /// DMA channel 2 transfer select 22: USCIB1 receive
            DMA2TSEL_22 = 0b10110,
            /// DMA channel 2 transfer select 23: USCIB1 transmit
            DMA2TSEL_23 = 0b10111,
            /// DMA channel 2 transfer select 24: ADC12IFGx
            DMA2TSEL_24 = 0b11000,
            /// DMA channel 2 transfer select 25: DAC12_0IFG
            DMA2TSEL_25 = 0b11001,
            /// DMA channel 2 transfer select 26: DAC12_1IFG
            DMA2TSEL_26 = 0b11010,
            /// DMA channel 2 transfer select 29: Multiplier ready
            DMA2TSEL_29 = 0b11101,
            /// DMA channel 2 transfer select 30: previous DMA channel DMA1IFG
            DMA2TSEL_30 = 0b11110,
            /// DMA channel 2 transfer select 31: ext. Trigger (DMAE0)
            DMA2TSEL_31 = 0b11111,
        }
        /// DMA channel 3 transfer select bit 0
        DMA3TSEL: 8..12 = enum DMA3TSEL {
            /// DMA channel 3 transfer select 0:  DMA_REQ (sw)
            DMA3TSEL_0 = 0b00000,
            /// DMA channel 3 transfer select 1:  Timer0_A (TA0CCR0.IFG)
            DMA3TSEL_1 = 0b00001,
            /// DMA channel 3 transfer select 2:  Timer0_A (TA0CCR2.IFG)
            DMA3TSEL_2 = 0b00010,
            /// DMA channel 3 transfer select 3:  Timer1_A (TA1CCR0.IFG)
            DMA3TSEL_3 = 0b00011,
            /// DMA channel 3 transfer select 4:  Timer1_A (TA1CCR2.IFG)
            DMA3TSEL_4 = 0b00100,
            /// DMA channel 3 transfer select 5:  Timer2_A (TA2CCR0.IFG)
            DMA3TSEL_5 = 0b00101,
            /// DMA channel 3 transfer select 6:  Timer2_A (TA2CCR2.IFG)
            DMA3TSEL_6 = 0b00110,
            /// DMA channel 3 transfer select 7:  TimerB0 (TB0CCR0.IFG)
            DMA3TSEL_7 = 0b00111,
            /// DMA channel 3 transfer select 8:  TimerB0 (TB0CCR2.IFG)
            DMA3TSEL_8 = 0b01000,
            /// DMA channel 3 transfer select 9:  Reserved
            DMA3TSEL_9 = 0b01001,
            /// DMA channel 3 transfer select 10: Reserved
            DMA3TSEL_10 = 0b01010,
            /// DMA channel 3 transfer select 11: Reserved
            DMA3TSEL_11 = 0b01011,
            /// DMA channel 3 transfer select 12: USCIA2 receive
            DMA3TSEL_12 = 0b01100,
            /// DMA channel 3 transfer select 13: USCIA2 transmit
            DMA3TSEL_13 = 0b01101,
            /// DMA channel 3 transfer select 14: USCIB2 receive
            DMA3TSEL_14 = 0b01110,
            /// DMA channel 3 transfer select 15: USCIB2 transmit
            DMA3TSEL_15 = 0b01111,
            /// DMA channel 3 transfer select 16: USCIA0 receive
            DMA3TSEL_16 = 0b10000,
            /// DMA channel 3 transfer select 17: USCIA0 transmit
            DMA3TSEL_17 = 0b10001,
            /// DMA channel 3 transfer select 18: USCIB0 receive
            DMA3TSEL_18 = 0b10010,
            /// DMA channel 3 transfer select 19: USCIB0 transmit
            DMA3TSEL_19 = 0b10011,
            /// DMA channel 3 transfer select 20: USCIA1 receive
            DMA3TSEL_20 = 0b10100,
            /// DMA channel 3 transfer select 21: USCIA1 transmit
            DMA3TSEL_21 = 0b10101,
            /// DMA channel 3 transfer select 22: USCIB1 receive
            DMA3TSEL_22 = 0b10110,
            /// DMA channel 3 transfer select 23: USCIB1 transmit
            DMA3TSEL_23 = 0b10111,
            /// DMA channel 3 transfer select 24: ADC12IFGx
            DMA3TSEL_24 = 0b11000,
            /// DMA channel 3 transfer select 25: DAC12_0IFG
            DMA3TSEL_25 = 0b11001,
            /// DMA channel 3 transfer select 26: DAC12_1IFG
            DMA3TSEL_26 = 0b11010,
            /// DMA channel 3 transfer select 29: Multiplier ready
            DMA3TSEL_29 = 0b11101,
            /// DMA channel 3 transfer select 30: previous DMA channel DMA2IFG
            DMA3TSEL_30 = 0b11110,
            /// DMA channel 3 transfer select 31: ext. Trigger (DMAE0)
            DMA3TSEL_31 = 0b11111,
        }
    }
    /// DMA Module Control 2
    rw DMACTL2 @ 0x04: u16 = 0_0 {
        /// DMA channel 4 transfer select bit 0
        DMA4TSEL: 0..4 = enum DMA4TSEL {
            /// DMA channel 4 transfer select 0:  DMA_REQ (sw)
            DMA4TSEL_0 = 0b00000,
            /// DMA channel 4 transfer select 1:  Timer0_A (TA0CCR0.IFG)
            DMA4TSEL_1 = 0b00001,
            /// DMA channel 4 transfer select 2:  Timer0_A (TA0CCR2.IFG)
            DMA4TSEL_2 = 0b00010,
            /// DMA channel 4 transfer select 3:  Timer1_A (TA1CCR0.IFG)
            DMA4TSEL_3 = 0b00011,
            /// DMA channel 4 transfer select 4:  Timer1_A (TA1CCR2.IFG)
            DMA4TSEL_4 = 0b00100,
            /// DMA channel 4 transfer select 5:  Timer2_A (TA2CCR0.IFG)
            DMA4TSEL_5 = 0b00101,
            /// DMA channel 4 transfer select 6:  Timer2_A (TA2CCR2.IFG)
            DMA4TSEL_6 = 0b00110,
            /// DMA channel 4 transfer select 7:  TimerB0 (TB0CCR0.IFG)
            DMA4TSEL_7 = 0b00111,
            /// DMA channel 4 transfer select 8:  TimerB0 (TB0CCR2.IFG)
            DMA4TSEL_8 = 0b01000,
            /// DMA channel 4 transfer select 9:  Reserved
            DMA4TSEL_9 = 0b01001,
            /// DMA channel 4 transfer select 10: Reserved
            DMA4TSEL_10 = 0b01010,
            /// DMA channel 4 transfer select 11: Reserved
            DMA4TSEL_11 = 0b01011,
            /// DMA channel 4 transfer select 12: USCIA2 receive
            DMA4TSEL_12 = 0b01100,
            /// DMA channel 4 transfer select 13: USCIA2 transmit
            DMA4TSEL_13 = 0b01101,
            /// DMA channel 4 transfer select 14: USCIB2 receive
            DMA4TSEL_14 = 0b01110,
            /// DMA channel 4 transfer select 15: USCIB2 transmit
            DMA4TSEL_15 = 0b01111,
            /// DMA channel 4 transfer select 16: USCIA0 receive
            DMA4TSEL_16 = 0b10000,
            /// DMA channel 4 transfer select 17: USCIA0 transmit
            DMA4TSEL_17 = 0b10001,
            /// DMA channel 4 transfer select 18: USCIB0 receive
            DMA4TSEL_18 = 0b10010,
            /// DMA channel 4 transfer select 19: USCIB0 transmit
            DMA4TSEL_19 = 0b10011,
            /// DMA channel 4 transfer select 20: USCIA1 receive
            DMA4TSEL_20 = 0b10100,
            /// DMA channel 4 transfer select 21: USCIA1 transmit
            DMA4TSEL_21 = 0b10101,
            /// DMA channel 4 transfer select 22: USCIB1 receive
            DMA4TSEL_22 = 0b10110,
            /// DMA channel 4 transfer select 23: USCIB1 transmit
            DMA4TSEL_23 = 0b10111,
            /// DMA channel 4 transfer select 24: ADC12IFGx
            DMA4TSEL_24 = 0b11000,
            /// DMA channel 4 transfer select 25: DAC12_0IFG
            DMA4TSEL_25 = 0b11001,
            /// DMA channel 4 transfer select 26: DAC12_1IFG
            DMA4TSEL_26 = 0b11010,
            /// DMA channel 4 transfer select 29: Multiplier ready
            DMA4TSEL_29 = 0b11101,
            /// DMA channel 4 transfer select 30: previous DMA channel DMA3IFG
            DMA4TSEL_30 = 0b11110,
            /// DMA channel 4 transfer select 31: ext. Trigger (DMAE0)
            DMA4TSEL_31 = 0b11111,
        }
        /// DMA channel 5 transfer select bit 0
        DMA5TSEL: 8..12 = enum DMA5TSEL {
            /// DMA channel 5 transfer select 0:  DMA_REQ (sw)
            DMA5TSEL_0 = 0b00000,
            /// DMA channel 5 transfer select 1:  Timer0_A (TA0CCR0.IFG)
            DMA5TSEL_1 = 0b00001,
            /// DMA channel 5 transfer select 2:  Timer0_A (TA0CCR2.IFG)
            DMA5TSEL_2 = 0b00010,
            /// DMA channel 5 transfer select 3:  Timer1_A (TA1CCR0.IFG)
            DMA5TSEL_3 = 0b00011,
            /// DMA channel 5 transfer select 4:  Timer1_A (TA1CCR2.IFG)
            DMA5TSEL_4 = 0b00100,
            /// DMA channel 5 transfer select 5:  Timer2_A (TA2CCR0.IFG)
            DMA5TSEL_5 = 0b00101,
            /// DMA channel 5 transfer select 6:  Timer2_A (TA2CCR2.IFG)
            DMA5TSEL_6 = 0b00110,
            /// DMA channel 5 transfer select 7:  TimerB0 (TB0CCR0.IFG)
            DMA5TSEL_7 = 0b00111,
            /// DMA channel 5 transfer select 8:  TimerB0 (TB0CCR2.IFG)
            DMA5TSEL_8 = 0b01000,
            /// DMA channel 5 transfer select 9:  Reserved
            DMA5TSEL_9 = 0b01001,
            /// DMA channel 5 transfer select 10: Reserved
            DMA5TSEL_10 = 0b01010,
            /// DMA channel 5 transfer select 11: Reserved
            DMA5TSEL_11 = 0b01011,
            /// DMA channel 5 transfer select 12: USCIA2 receive
            DMA5TSEL_12 = 0b01100,
            /// DMA channel 5 transfer select 13: USCIA2 transmit
            DMA5TSEL_13 = 0b01101,
            /// DMA channel 5 transfer select 14: USCIB2 receive
            DMA5TSEL_14 = 0b01110,
            /// DMA channel 5 transfer select 15: USCIB2 transmit
            DMA5TSEL_15 = 0b01111,
            /// DMA channel 5 transfer select 16: USCIA0 receive
            DMA5TSEL_16 = 0b10000,
            /// DMA channel 5 transfer select 17: USCIA0 transmit
            DMA5TSEL_17 = 0b10001,
            /// DMA channel 5 transfer select 18: USCIB0 receive
            DMA5TSEL_18 = 0b10010,
            /// DMA channel 5 transfer select 19: USCIB0 transmit
            DMA5TSEL_19 = 0b10011,
            /// DMA channel 5 transfer select 20: USCIA1 receive
            DMA5TSEL_20 = 0b10100,
            /// DMA channel 5 transfer select 21: USCIA1 transmit
            DMA5TSEL_21 = 0b10101,
            /// DMA channel 5 transfer select 22: USCIB1 receive
            DMA5TSEL_22 = 0b10110,
            /// DMA channel 5 transfer select 23: USCIB1 transmit
            DMA5TSEL_23 = 0b10111,
            /// DMA channel 5 transfer select 24: ADC12IFGx
            DMA5TSEL_24 = 0b11000,
            /// DMA channel 5 transfer select 25: DAC12_0IFG
            DMA5TSEL_25 = 0b11001,
            /// DMA channel 5 transfer select 26: DAC12_1IFG
            DMA5TSEL_26 = 0b11010,
            /// DMA channel 5 transfer select 29: Multiplier ready
            DMA5TSEL_29 = 0b11101,
            /// DMA channel 5 transfer select 30: previous DMA channel DMA4IFG
            DMA5TSEL_30 = 0b11110,
            /// DMA channel 5 transfer select 31: ext. Trigger (DMAE0)
            DMA5TSEL_31 = 0b11111,
        }
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
    /// DMA Channel 3 Control
    rw DMA3CTL @ 0x40: u16 = 0_0 {
        /// Initiate DMA transfer with DMATSEL
        DMA3CTL_DMAREQ: 0 = struct DMA3CTL_DMAREQ(bool);
        /// DMA transfer aborted by NMI
        DMA3CTL_DMAABORT: 1 = struct DMA3CTL_DMAABORT(bool);
        /// DMA interrupt enable
        DMA3CTL_DMAIE: 2 = struct DMA3CTL_DMAIE(bool);
        /// DMA interrupt flag
        DMA3CTL_DMAIFG: 3 = struct DMA3CTL_DMAIFG(bool);
        /// DMA enable
        DMA3CTL_DMAEN: 4 = struct DMA3CTL_DMAEN(bool);
        /// DMA level sensitive trigger select
        DMA3CTL_DMALEVEL: 5 = struct DMA3CTL_DMALEVEL(bool);
        /// DMA source byte
        DMA3CTL_DMASRCBYTE: 6 = struct DMA3CTL_DMASRCBYTE(bool);
        /// DMA destination byte
        DMA3CTL_DMADSTBYTE: 7 = struct DMA3CTL_DMADSTBYTE(bool);
        /// DMA source increment bit 0
        DMA3CTL_DMASRCINCR: 8..9 = enum DMA3CTL_DMASRCINCR {
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
        DMA3CTL_DMADSTINCR: 10..11 = enum DMA3CTL_DMADSTINCR {
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
        DMA3CTL_DMADT: 12..14 = enum DMA3CTL_DMADT {
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
    /// DMA Channel 3 Source Address
    rw DMA3SA @ 0x42: u32 = 0_0 {
        /// DMA Channel 3 Source Address
        DMA3SA: 0..31 = struct DMA3SAField(u32);
    }
    /// DMA Channel 3 Destination Address
    rw DMA3DA @ 0x46: u32 = 0_0 {
        /// DMA Channel 3 Destination Address
        DMA3DA: 0..31 = struct DMA3DAField(u32);
    }
    /// DMA Channel 3 Transfer Size
    rw DMA3SZ @ 0x4a: u16 = 0_0 {
        /// DMA Channel 3 Transfer Size
        DMA3SZ: 0..15 = struct DMA3SZField(u16);
    }
    /// DMA Channel 4 Control
    rw DMA4CTL @ 0x50: u16 = 0_0 {
        /// Initiate DMA transfer with DMATSEL
        DMA4CTL_DMAREQ: 0 = struct DMA4CTL_DMAREQ(bool);
        /// DMA transfer aborted by NMI
        DMA4CTL_DMAABORT: 1 = struct DMA4CTL_DMAABORT(bool);
        /// DMA interrupt enable
        DMA4CTL_DMAIE: 2 = struct DMA4CTL_DMAIE(bool);
        /// DMA interrupt flag
        DMA4CTL_DMAIFG: 3 = struct DMA4CTL_DMAIFG(bool);
        /// DMA enable
        DMA4CTL_DMAEN: 4 = struct DMA4CTL_DMAEN(bool);
        /// DMA level sensitive trigger select
        DMA4CTL_DMALEVEL: 5 = struct DMA4CTL_DMALEVEL(bool);
        /// DMA source byte
        DMA4CTL_DMASRCBYTE: 6 = struct DMA4CTL_DMASRCBYTE(bool);
        /// DMA destination byte
        DMA4CTL_DMADSTBYTE: 7 = struct DMA4CTL_DMADSTBYTE(bool);
        /// DMA source increment bit 0
        DMA4CTL_DMASRCINCR: 8..9 = enum DMA4CTL_DMASRCINCR {
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
        DMA4CTL_DMADSTINCR: 10..11 = enum DMA4CTL_DMADSTINCR {
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
        DMA4CTL_DMADT: 12..14 = enum DMA4CTL_DMADT {
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
    /// DMA Channel 4 Source Address
    rw DMA4SA @ 0x52: u32 = 0_0 {
        /// DMA Channel 4 Source Address
        DMA4SA: 0..31 = struct DMA4SAField(u32);
    }
    /// DMA Channel 4 Destination Address
    rw DMA4DA @ 0x56: u32 = 0_0 {
        /// DMA Channel 4 Destination Address
        DMA4DA: 0..31 = struct DMA4DAField(u32);
    }
    /// DMA Channel 4 Transfer Size
    rw DMA4SZ @ 0x5a: u16 = 0_0 {
        /// DMA Channel 4 Transfer Size
        DMA4SZ: 0..15 = struct DMA4SZField(u16);
    }
    /// DMA Channel 5 Control
    rw DMA5CTL @ 0x60: u16 = 0_0 {
        /// Initiate DMA transfer with DMATSEL
        DMA5CTL_DMAREQ: 0 = struct DMA5CTL_DMAREQ(bool);
        /// DMA transfer aborted by NMI
        DMA5CTL_DMAABORT: 1 = struct DMA5CTL_DMAABORT(bool);
        /// DMA interrupt enable
        DMA5CTL_DMAIE: 2 = struct DMA5CTL_DMAIE(bool);
        /// DMA interrupt flag
        DMA5CTL_DMAIFG: 3 = struct DMA5CTL_DMAIFG(bool);
        /// DMA enable
        DMA5CTL_DMAEN: 4 = struct DMA5CTL_DMAEN(bool);
        /// DMA level sensitive trigger select
        DMA5CTL_DMALEVEL: 5 = struct DMA5CTL_DMALEVEL(bool);
        /// DMA source byte
        DMA5CTL_DMASRCBYTE: 6 = struct DMA5CTL_DMASRCBYTE(bool);
        /// DMA destination byte
        DMA5CTL_DMADSTBYTE: 7 = struct DMA5CTL_DMADSTBYTE(bool);
        /// DMA source increment bit 0
        DMA5CTL_DMASRCINCR: 8..9 = enum DMA5CTL_DMASRCINCR {
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
        DMA5CTL_DMADSTINCR: 10..11 = enum DMA5CTL_DMADSTINCR {
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
        DMA5CTL_DMADT: 12..14 = enum DMA5CTL_DMADT {
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
    /// DMA Channel 5 Source Address
    rw DMA5SA @ 0x62: u32 = 0_0 {
        /// DMA Channel 5 Source Address
        DMA5SA: 0..31 = struct DMA5SAField(u32);
    }
    /// DMA Channel 5 Destination Address
    rw DMA5DA @ 0x66: u32 = 0_0 {
        /// DMA Channel 5 Destination Address
        DMA5DA: 0..31 = struct DMA5DAField(u32);
    }
    /// DMA Channel 5 Transfer Size
    rw DMA5SZ @ 0x6a: u16 = 0_0 {
        /// DMA Channel 5 Transfer Size
        DMA5SZ: 0..15 = struct DMA5SZField(u16);
    }
}
