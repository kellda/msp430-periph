//! ExtScanIF

utils::periph! {
    /// ExtScanIF
    ExtScanIF;
    /// ESI debug register 1
    rw DEBUG1 @ 0x00: u16 = 0_0 {
        /// ESI debug register 1
        DEBUG1: 0..15 = struct DEBUG1Field(u16);
    }
    /// ESI debug register 2
    rw DEBUG2 @ 0x02: u16 = 0_0 {
        /// ESI debug register 2
        DEBUG2: 0..15 = struct DEBUG2Field(u16);
    }
    /// ESI debug register 3
    rw DEBUG3 @ 0x04: u16 = 0_0 {
        /// ESI debug register 3
        DEBUG3: 0..15 = struct DEBUG3Field(u16);
    }
    /// ESI debug register 4
    rw DEBUG4 @ 0x06: u16 = 0_0 {
        /// ESI debug register 4
        DEBUG4: 0..15 = struct DEBUG4Field(u16);
    }
    /// ESI debug register 5
    rw DEBUG5 @ 0x08: u16 = 0_0 {
        /// ESI debug register 5
        DEBUG5: 0..15 = struct DEBUG5Field(u16);
    }
    /// ESI PSM counter 0
    rw CNT0 @ 0x10: u16 = 0_0 {
        /// ESI PSM counter 0
        CNT0: 0..15 = struct CNT0Field(u16);
    }
    /// ESI PSM counter 1
    rw CNT1 @ 0x12: u16 = 0_0 {
        /// ESI PSM counter 1
        CNT1: 0..15 = struct CNT1Field(u16);
    }
    /// ESI PSM counter 2
    rw CNT2 @ 0x14: u16 = 0_0 {
        /// ESI PSM counter 2
        CNT2: 0..15 = struct CNT2Field(u16);
    }
    /// ESI oscillator counter register
    rw CNT3 @ 0x16: u16 = 0_0 {
        /// ESI oscillator counter register
        CNT3: 0..15 = struct CNT3Field(u16);
    }
    /// ESI interrupt vector
    rw IV @ 0x1a: u16 = 0_0 {
        /// ESI interrupt vector
        IV: 0..15 = struct IVField(u16);
    }
    /// ESI interrupt register 1
    rw INT1 @ 0x1c: u16 = 0_0 {
        /// Interrupt enable
        IE0: 0 = struct IE0(bool);
        /// Interrupt enable
        IE1: 1 = struct IE1(bool);
        /// Interrupt enable
        IE2: 2 = struct IE2(bool);
        /// Interrupt enable
        IE3: 3 = struct IE3(bool);
        /// Interrupt enable
        IE4: 4 = struct IE4(bool);
        /// Interrupt enable
        IE5: 5 = struct IE5(bool);
        /// Interrupt enable
        IE6: 6 = struct IE6(bool);
        /// Interrupt enable
        IE7: 7 = struct IE7(bool);
        /// Interrupt enable
        IE8: 8 = struct IE8(bool);
        /// IFG0 interrupt flag source
        IFGSET1: 10..12 = enum IFGSET1 {
            /// ESIIFG0 is set when ESIOUT0 is set
            IFGSET1_0 = 0b000,
            /// ESIIFG0 is set when ESIOUT0 is reset
            IFGSET1_1 = 0b001,
            /// ESIIFG0 is set when ESIOUT1 is set
            IFGSET1_2 = 0b010,
            /// ESIIFG0 is set when ESIOUT1 is reset
            IFGSET1_3 = 0b011,
            /// ESIIFG0 is set when ESIOUT2 is set
            IFGSET1_4 = 0b100,
            /// ESIIFG0 is set when ESIOUT2 is reset
            IFGSET1_5 = 0b101,
            /// ESIIFG0 is set when ESIOUT3 is set
            IFGSET1_6 = 0b110,
            /// ESIIFG0 is set when ESIOUT3 is reset
            IFGSET1_7 = 0b111,
        }
        /// ESIIFG8 interrupt flag source
        IFGSET2: 13..15 = enum IFGSET2 {
            /// ESIIFG8 is set when ESIOUT4 is set
            IFGSET2_0 = 0b000,
            /// ESIIFG8 is set when ESIOUT4 is reset
            IFGSET2_1 = 0b001,
            /// ESIIFG8 is set when ESIOUT5 is set
            IFGSET2_2 = 0b010,
            /// ESIIFG8 is set when ESIOUT5 is reset
            IFGSET2_3 = 0b011,
            /// ESIIFG8 is set when ESIOUT6 is set
            IFGSET2_4 = 0b100,
            /// ESIIFG8 is set when ESIOUT6 is reset
            IFGSET2_5 = 0b101,
            /// ESIIFG8 is set when ESIOUT7 is set
            IFGSET2_6 = 0b110,
            /// ESIIFG8 is set when ESIOUT7 is reset
            IFGSET2_7 = 0b111,
        }
    }
    /// ESI interrupt register 2
    rw INT2 @ 0x1e: u16 = 0_0 {
        /// ESIIFG0 interrupt pending
        IFG0: 0 = struct IFG0(bool);
        /// ESIIFG1 interrupt pending
        IFG1: 1 = struct IFG1(bool);
        /// ESIIFG2 interrupt pending
        IFG2: 2 = struct IFG2(bool);
        /// ESIIFG3 interrupt pending
        IFG3: 3 = struct IFG3(bool);
        /// ESIIFG4 interrupt pending
        IFG4: 4 = struct IFG4(bool);
        /// ESIIFG5 interrupt pending
        IFG5: 5 = struct IFG5(bool);
        /// ESIIFG6 interrupt pending
        IFG6: 6 = struct IFG6(bool);
        /// ESIIFG7 interrupt pending
        IFG7: 7 = struct IFG7(bool);
        /// ESIIFG8 interrupt pending
        IFG8: 8 = struct IFG8(bool);
        /// SIFIFG7 interrupt flag source
        IS0: 10..11 = enum IS0 {
            /// SIFIFG7 interrupt flag source: SIFCNT0
            IS0_0 = 0b00,
            /// SIFIFG7 interrupt flag source: SIFCNT0 MOD 4
            IS0_1 = 0b01,
            /// SIFIFG7 interrupt flag source: SIFCNT0 MOD 256
            IS0_2 = 0b10,
            /// SIFIFG7 interrupt flag source: SIFCNT0 increments from FFFFh to 00h
            IS0_3 = 0b11,
        }
        /// SIFIFG4 interrupt flag source
        IS2: 13..14 = enum IS2 {
            /// SIFIFG4 interrupt flag source: SIFCNT2
            IS2_0 = 0b00,
            /// SIFIFG4 interrupt flag source: SIFCNT2 MOD 4
            IS2_1 = 0b01,
            /// SIFIFG4 interrupt flag source: SIFCNT2 MOD 256
            IS2_2 = 0b10,
            /// SIFIFG4 interrupt flag source: SIFCNT2 decrements from 01h to 00h
            IS2_3 = 0b11,
        }
    }
    /// ESI AFE control register
    rw AFE @ 0x20: u16 = 0_0 {
        /// Excitation enable
        TEN: 0 = struct TEN(bool);
        /// Sample-and-hold enable
        SH: 1 = struct SH(bool);
        /// Mid-voltage generator
        VMIDEN: 2 = struct VMIDEN(bool);
        /// Sample-and-hold ESIVSS select
        SHTSM: 3 = struct SHTSM(bool);
        /// Comparator input select for AFE1 only
        CACI3: 4 = struct CACI3(bool);
        /// Comparator input select for AFE1 only
        CISEL: 5 = struct CISEL(bool);
        /// AFE1's comparator input select
        CA1X: 6 = struct CA1X(bool);
        /// AFE2's comparator input select
        CA2X: 7 = struct CA2X(bool);
        /// Invert AFE1's comparator output
        CA1INV: 8 = struct CA1INV(bool);
        /// Invert AFE2's comparator output
        CA2INV: 9 = struct CA2INV(bool);
        /// Enable ESICA(tsm) control for comparator in AFE2
        CA2EN: 10 = struct CA2EN(bool);
        /// Enable ESIDAC(tsm) control for DAC in AFE2
        DAC2EN: 11 = struct DAC2EN(bool);
    }
    /// ESI PPU control register
    rw PPU @ 0x22: u16 = 0_0 {
        /// Latched AFE1 comparator output when ESICH0 input is selected
        OUT0: 0 = struct OUT0(bool);
        /// Latched AFE1 comparator output when ESICH1 input is selected
        OUT1: 1 = struct OUT1(bool);
        /// Latched AFE1 comparator output when ESICH2 input is selected
        OUT2: 2 = struct OUT2(bool);
        /// Latched AFE1 comparator output when ESICH3 input is selected
        OUT3: 3 = struct OUT3(bool);
        /// Latched AFE2 comparator output when ESICH0 input is selected
        OUT4: 4 = struct OUT4(bool);
        /// Latched AFE2 comparator output when ESICH1 input is selected
        OUT5: 5 = struct OUT5(bool);
        /// Latched AFE2 comparator output when ESICH2 input is selected
        OUT6: 6 = struct OUT6(bool);
        /// Latched AFE2 comparator output when ESICH3 input is selected
        OUT7: 7 = struct OUT7(bool);
        /// Lachted AFE1 comparator output for test channel 0
        TCHOUT0: 8 = struct TCHOUT0(bool);
        /// Latched AFE1 comparator output for test channel 1
        TCHOUT1: 9 = struct TCHOUT1(bool);
    }
    /// ESI TSM control register
    rw TSM @ 0x24: u16 = 0_0 {
        /// TSM SMCLK divider
        DIV1: 0..1 = enum DIV1 {
            /// TSM SMCLK/ESIOSC divider mode: 0
            DIV1_0 = 0b00,
            /// TSM SMCLK/ESIOSC divider mode: 1
            DIV1_1 = 0b01,
            /// TSM SMCLK/ESIOSC divider mode: 2
            DIV1_2 = 0b10,
            /// TSM SMCLK/ESIOSC divider mode: 3
            DIV1_3 = 0b11,
        }
        /// ACLK divider
        DIV2: 2..3 = enum DIV2 {
            /// ACLK divider mode: 0
            DIV2_0 = 0b00,
            /// ACLK divider mode: 1
            DIV2_1 = 0b01,
            /// ACLK divider mode: 2
            DIV2_2 = 0b10,
            /// ACLK divider mode: 3
            DIV2_3 = 0b11,
        }
        /// TSM start trigger ACLK divider
        DIV3A: 4..6 = enum DIV3A {
            /// TSM start trigger ACLK divider
            DIV3A_0 = 0b000,
            /// TSM start trigger ACLK divider
            DIV3A_1 = 0b001,
            /// TSM start trigger ACLK divider
            DIV3A_2 = 0b010,
            /// TSM start trigger ACLK divider
            DIV3A_3 = 0b011,
            /// TSM start trigger ACLK divider
            DIV3A_4 = 0b100,
            /// TSM start trigger ACLK divider
            DIV3A_5 = 0b101,
            /// TSM start trigger ACLK divider
            DIV3A_6 = 0b110,
            /// TSM start trigger ACLK divider
            DIV3A_7 = 0b111,
        }
        /// TSM start trigger ACLK divider
        DIV3B: 7..9 = enum DIV3B {
            /// TSM start trigger ACLK divider
            DIV3B_0 = 0b000,
            /// TSM start trigger ACLK divider
            DIV3B_1 = 0b001,
            /// TSM start trigger ACLK divider
            DIV3B_2 = 0b010,
            /// TSM start trigger ACLK divider
            DIV3B_3 = 0b011,
            /// TSM start trigger ACLK divider
            DIV3B_4 = 0b100,
            /// TSM start trigger ACLK divider
            DIV3B_5 = 0b101,
            /// TSM start trigger ACLK divider
            DIV3B_6 = 0b110,
            /// TSM start trigger ACLK divider
            DIV3B_7 = 0b111,
        }
        /// TSM repeat modee
        TSMRP: 10 = struct TSMRP(bool);
        /// TSM software start trigger
        START: 11 = struct START(bool);
        /// TSM start trigger selection
        TSMTRG: 12..13 = enum TSMTRG {
            /// Halt mode
            TSMTRG_0 = 0b00,
            /// TSM start trigger ACLK divider
            TSMTRG_1 = 0b01,
            /// Software trigger for TSM
            TSMTRG_2 = 0b10,
            /// Either the ACLK divider or the ESISTART biT
            TSMTRG_3 = 0b11,
        }
        /// Functionality selection of ESITSMx bit5
        CLKAZSEL: 14 = struct CLKAZSEL(bool);
    }
    /// ESI PSM control register
    rw PSM @ 0x26: u16 = 0_0 {
        /// Q6 enable
        Q6EN: 0 = struct Q6EN(bool);
        /// Enabling to use Q7 as trigger for a TSM sequence
        Q7TRG: 2 = struct Q7TRG(bool);
        /// ESICNT0 enable (up counter)
        CNT0EN: 3 = struct CNT0EN(bool);
        /// ESICNT1 enable (up/down counter)
        CNT1EN: 4 = struct CNT1EN(bool);
        /// ESICNT2 enable (down counter)
        CNT2EN: 5 = struct CNT2EN(bool);
        /// Source Selection for V2 bit
        V2SEL: 7 = struct V2SEL(bool);
        /// Output signal selection for SIFTEST4 pin
        TEST4SEL: 8..9 = enum TEST4SEL {
            /// Q1 signal from PSM table
            TEST4SEL_0 = 0b00,
            /// Q2 signal from PSM table
            TEST4SEL_1 = 0b01,
            /// TSM clock signal from Timing State Machine
            TEST4SEL_2 = 0b10,
            /// AFE1's comparator output signal Comp1Out
            TEST4SEL_3 = 0b11,
        }
        /// ESI Counter 0 reset
        CNT0RST: 13 = struct CNT0RST(bool);
        /// ESI Counter 1 reset
        CNT1RST: 14 = struct CNT1RST(bool);
        /// ESI Counter 2 reset
        CNT2RST: 15 = struct CNT2RST(bool);
    }
    /// ESI oscillator control register
    rw OSC @ 0x28: u16 = 0_0 {
        /// Internal oscillator enable
        HFSEL: 0 = struct HFSEL(bool);
        /// Internal oscillator control
        CLKGON: 1 = struct CLKGON(bool);
        /// Internal oscillator frequency adjust
        CLKFQ0: 8 = struct CLKFQ0(bool);
        /// Internal oscillator frequency adjust
        CLKFQ1: 9 = struct CLKFQ1(bool);
        /// Internal oscillator frequency adjust
        CLKFQ2: 10 = struct CLKFQ2(bool);
        /// Internal oscillator frequency adjust
        CLKFQ3: 11 = struct CLKFQ3(bool);
        /// Internal oscillator frequency adjust
        CLKFQ4: 12 = struct CLKFQ4(bool);
        /// Internal oscillator frequency adjust
        CLKFQ5: 13 = struct CLKFQ5(bool);
    }
    /// ESI control register
    rw CTL @ 0x2a: u16 = 0_0 {
        /// Extended Scan interface enable
        EN: 0 = struct EN(bool);
        /// Test cycle insertion
        TESTD: 1 = struct TESTD(bool);
        /// Comparator output/Timer_A input selection
        CS: 2 = struct CS(bool);
        /// select the comparator input for test channel 0
        TCH0: 3..4 = enum TCH0 {
            /// Comparator input is ESICH0 when ESICAX = 0; Comparator input is ESICI0 when ESICAX = 1
            TCH0_0 = 0b00,
            /// Comparator input is ESICH1 when ESICAX = 0; Comparator input is ESICI1 when ESICAX = 1
            TCH0_1 = 0b01,
            /// Comparator input is ESICH2 when ESICAX = 0; Comparator input is ESICI2 when ESICAX = 1
            TCH0_2 = 0b10,
            /// Comparator input is ESICH3 when ESICAX = 0; Comparator input is ESICI3 when ESICAX = 1
            TCH0_3 = 0b11,
        }
        /// select the comparator input for test channel 1
        TCH1: 5..6 = enum TCH1 {
            /// Comparator input is ESICH0 when ESICAX = 0; Comparator input is ESICI0 when ESICAX = 1
            TCH1_0 = 0b00,
            /// Comparator input is ESICH1 when ESICAX = 0; Comparator input is ESICI1 when ESICAX = 1
            TCH1_1 = 0b01,
            /// Comparator input is ESICH2 when ESICAX = 0; Comparator input is ESICI2 when ESICAX = 1
            TCH1_2 = 0b10,
            /// Comparator input is ESICH3 when ESICAX = 0; Comparator input is ESICI3 when ESICAX = 1
            TCH1_3 = 0b11,
        }
        /// PPUS1 source select
        S1SEL: 7..9 = enum S1SEL {
            /// ESIOUT0 is the PPUS1 source
            S1SEL_0 = 0b000,
            /// ESIOUT1 is the PPUS1 source
            S1SEL_1 = 0b001,
            /// ESIOUT2 is the PPUS1 source
            S1SEL_2 = 0b010,
            /// ESIOUT3 is the PPUS1 source
            S1SEL_3 = 0b011,
            /// ESIOUT4 is the PPUS1 source
            S1SEL_4 = 0b100,
            /// ESIOUT5 is the PPUS1 source
            S1SEL_5 = 0b101,
            /// ESIOUT6 is the PPUS1 source
            S1SEL_6 = 0b110,
            /// ESIOUT7 is the PPUS1 source
            S1SEL_7 = 0b111,
        }
        /// PPUS2 source select
        S2SEL: 10..12 = enum S2SEL {
            /// ESIOUT0 is the PPUS2 source
            S2SEL_0 = 0b000,
            /// ESIOUT1 is the PPUS2 source
            S2SEL_1 = 0b001,
            /// ESIOUT2 is the PPUS2 source
            S2SEL_2 = 0b010,
            /// ESIOUT3 is the PPUS2 source
            S2SEL_3 = 0b011,
            /// ESIOUT4 is the PPUS2 source
            S2SEL_4 = 0b100,
            /// ESIOUT5 is the PPUS2 source
            S2SEL_5 = 0b101,
            /// ESIOUT6 is the PPUS2 source
            S2SEL_6 = 0b110,
            /// ESIOUT7 is the PPUS2 source
            S2SEL_7 = 0b111,
        }
        /// PPUS3 source select
        S3SEL: 13..15 = enum S3SEL {
            /// ESIOUT0 is the PPUS3 source
            S3SEL_0 = 0b000,
            /// ESIOUT1 is the PPUS3 source
            S3SEL_1 = 0b001,
            /// ESIOUT2 is the PPUS3 source
            S3SEL_2 = 0b010,
            /// ESIOUT3 is the PPUS3 source
            S3SEL_3 = 0b011,
            /// ESIOUT4 is the PPUS3 source
            S3SEL_4 = 0b100,
            /// ESIOUT5 is the PPUS3 source
            S3SEL_5 = 0b101,
            /// ESIOUT6 is the PPUS3 source
            S3SEL_6 = 0b110,
            /// ESIOUT7 is the PPUS3 source
            S3SEL_7 = 0b111,
        }
    }
    /// ESI PSM Counter Threshold 1 register
    rw THR1 @ 0x2c: u16 = 0_0 {
        /// ESI PSM Counter Threshold 1 register
        THR1: 0..15 = struct THR1Field(u16);
    }
    /// ESI PSM Counter Threshold 2 register
    rw THR2 @ 0x2e: u16 = 0_0 {
        /// ESI PSM Counter Threshold 2 register
        THR2: 0..15 = struct THR2Field(u16);
    }
    /// ESI DAC1 register 0
    rw DAC1R0 @ 0x40: u16 = 0_0 {
        /// ESI DAC1 register 0
        DAC1R0: 0..15 = struct DAC1R0Field(u16);
    }
    /// ESI DAC1 register 1
    rw DAC1R1 @ 0x42: u16 = 0_0 {
        /// ESI DAC1 register 1
        DAC1R1: 0..15 = struct DAC1R1Field(u16);
    }
    /// ESI DAC1 register 2
    rw DAC1R2 @ 0x44: u16 = 0_0 {
        /// ESI DAC1 register 2
        DAC1R2: 0..15 = struct DAC1R2Field(u16);
    }
    /// ESI DAC1 register 3
    rw DAC1R3 @ 0x46: u16 = 0_0 {
        /// ESI DAC1 register 3
        DAC1R3: 0..15 = struct DAC1R3Field(u16);
    }
    /// ESI DAC1 register 4
    rw DAC1R4 @ 0x48: u16 = 0_0 {
        /// ESI DAC1 register 4
        DAC1R4: 0..15 = struct DAC1R4Field(u16);
    }
    /// ESI DAC1 register 5
    rw DAC1R5 @ 0x4a: u16 = 0_0 {
        /// ESI DAC1 register 5
        DAC1R5: 0..15 = struct DAC1R5Field(u16);
    }
    /// ESI DAC1 register 6
    rw DAC1R6 @ 0x4c: u16 = 0_0 {
        /// ESI DAC1 register 6
        DAC1R6: 0..15 = struct DAC1R6Field(u16);
    }
    /// ESI DAC1 register 7
    rw DAC1R7 @ 0x4e: u16 = 0_0 {
        /// ESI DAC1 register 7
        DAC1R7: 0..15 = struct DAC1R7Field(u16);
    }
    /// ESI DAC2 register 0
    rw DAC2R0 @ 0x50: u16 = 0_0 {
        /// ESI DAC2 register 0
        DAC2R0: 0..15 = struct DAC2R0Field(u16);
    }
    /// ESI DAC2 register 1
    rw DAC2R1 @ 0x52: u16 = 0_0 {
        /// ESI DAC2 register 1
        DAC2R1: 0..15 = struct DAC2R1Field(u16);
    }
    /// ESI DAC2 register 2
    rw DAC2R2 @ 0x54: u16 = 0_0 {
        /// ESI DAC2 register 2
        DAC2R2: 0..15 = struct DAC2R2Field(u16);
    }
    /// ESI DAC2 register 3
    rw DAC2R3 @ 0x56: u16 = 0_0 {
        /// ESI DAC2 register 3
        DAC2R3: 0..15 = struct DAC2R3Field(u16);
    }
    /// ESI DAC2 register 4
    rw DAC2R4 @ 0x58: u16 = 0_0 {
        /// ESI DAC2 register 4
        DAC2R4: 0..15 = struct DAC2R4Field(u16);
    }
    /// ESI DAC2 register 5
    rw DAC2R5 @ 0x5a: u16 = 0_0 {
        /// ESI DAC2 register 5
        DAC2R5: 0..15 = struct DAC2R5Field(u16);
    }
    /// ESI DAC2 register 6
    rw DAC2R6 @ 0x5c: u16 = 0_0 {
        /// ESI DAC2 register 6
        DAC2R6: 0..15 = struct DAC2R6Field(u16);
    }
    /// ESI DAC2 register 7
    rw DAC2R7 @ 0x5e: u16 = 0_0 {
        /// ESI DAC2 register 7
        DAC2R7: 0..15 = struct DAC2R7Field(u16);
    }
    /// ESI TSM 0
    rw TSM0 @ 0x60: u16 = 0_0 {
        /// Input channel select
        TSM0CH: 0..1 = enum TSM0CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM0LCEN: 2 = struct TSM0LCEN(bool);
        /// Excitation and sample-and-hold
        TSM0IEX: 3 = struct TSM0EX(bool);
        /// TSM comparator on
        TSM0CA: 4 = struct TSM0ICA(bool);
        /// High-frequency clock on
        TSM0CLKON: 5 = struct TSM0CLKON(bool);
        /// Internal output latches enabled
        TSM0RSON: 6 = struct TSM0RSON(bool);
        /// TSM test cycle control
        TSM0TESTS1: 7 = struct TSM0TESTS1(bool);
        /// TSM DAC on
        TSM0DAC: 8 = struct TSM0DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM0STOP: 9 = struct TSM0STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM0CLK: 10 = struct TSM0CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM0REPEAT: 11..15 = enum TSM0REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 1
    rw TSM1 @ 0x62: u16 = 0_0 {
        /// Input channel select
        TSM1CH: 0..1 = enum TSM1CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM1LCEN: 2 = struct TSM1LCEN(bool);
        /// Excitation and sample-and-hold
        TSM1EX: 3 = struct TSM1EX(bool);
        /// TSM comparator on
        TSM1CA: 4 = struct TSM1CA(bool);
        /// High-frequency clock on
        TSM1CLKON: 5 = struct TSM1CLKON(bool);
        /// Internal output latches enabled
        TSM1RSON: 6 = struct TSM1RSON(bool);
        /// TSM test cycle control
        TSM1TESTS1: 7 = struct TSM1TESTS1(bool);
        /// TSM DAC on
        TSM1DAC: 8 = struct TSM1DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM1STOP: 9 = struct TSM1STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM1CLK: 10 = struct TSM1CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM1REPEAT: 11..15 = enum TSM1REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 2
    rw TSM2 @ 0x64: u16 = 0_0 {
        /// Input channel select
        TSM2CH: 0..1 = enum TSM2CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM2LCEN: 2 = struct TSM2LCEN(bool);
        /// Excitation and sample-and-hold
        TSM2EX: 3 = struct TSM2EX(bool);
        /// TSM comparator on
        TSM2CA: 4 = struct TSM2CA(bool);
        /// High-frequency clock on
        TSM2CLKON: 5 = struct TSM2CLKON(bool);
        /// Internal output latches enabled
        TSM2RSON: 6 = struct TSM2RSON(bool);
        /// TSM test cycle control
        TSM2TESTS1: 7 = struct TSM2TESTS1(bool);
        /// TSM DAC on
        TSM2DAC: 8 = struct TSM2DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM2STOP: 9 = struct TSM2STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM2CLK: 10 = struct TSM2CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM2REPEAT: 11..15 = enum TSM2REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 3
    rw TSM3 @ 0x66: u16 = 0_0 {
        /// Input channel select
        TSM3CH: 0..1 = enum TSM3CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM3LCEN: 2 = struct TSM3LCEN(bool);
        /// Excitation and sample-and-hold
        TSM3EX: 3 = struct TSM3EX(bool);
        /// TSM comparator on
        TSM3CA: 4 = struct TSM3CA(bool);
        /// High-frequency clock on
        TSM3CLKON: 5 = struct TSM3CLKON(bool);
        /// Internal output latches enabled
        TSM3RSON: 6 = struct TSM3RSON(bool);
        /// TSM test cycle control
        TSM3TESTS1: 7 = struct TSM3TESTS1(bool);
        /// TSM DAC on
        TSM3DAC: 8 = struct TSM3DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM3STOP: 9 = struct TSM3STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM3CLK: 10 = struct TSM3CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM3REPEAT: 11..15 = enum TSM3REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 4
    rw TSM4 @ 0x68: u16 = 0_0 {
        /// Input channel select
        TSM4CH: 0..1 = enum TSM4CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM4LCEN: 2 = struct TSM4LCEN(bool);
        /// Excitation and sample-and-hold
        TSM4EX: 3 = struct TSM4EX(bool);
        /// TSM comparator on
        TSM4CA: 4 = struct TSM4CA(bool);
        /// High-frequency clock on
        TSM4CLKON: 5 = struct TSM4CLKON(bool);
        /// Internal output latches enabled
        TSM4RSON: 6 = struct TSM4RSON(bool);
        /// TSM test cycle control
        TSM4TESTS1: 7 = struct TSM4TESTS1(bool);
        /// TSM DAC on
        TSM4DAC: 8 = struct TSM4DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM4STOP: 9 = struct TSM4STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM4CLK: 10 = struct TSM4CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM4REPEAT: 11..15 = enum TSM4REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 5
    rw TSM5 @ 0x6a: u16 = 0_0 {
        /// Input channel select
        TSM5CH: 0..1 = enum TSM5CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM5LCEN: 2 = struct TSM5LCEN(bool);
        /// Excitation and sample-and-hold
        TSM5EX: 3 = struct TSM5EX(bool);
        /// TSM comparator on
        TSM5CA: 4 = struct TSM5CA(bool);
        /// High-frequency clock on
        TSM5CLKON: 5 = struct TSM5CLKON(bool);
        /// Internal output latches enabled
        TSM5RSON: 6 = struct TSM5RSON(bool);
        /// TSM test cycle control
        TSM5TESTS1: 7 = struct TSM5TESTS1(bool);
        /// TSM DAC on
        TSM5DAC: 8 = struct TSM5DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM5STOP: 9 = struct TSM5STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM5CLK: 10 = struct TSM5CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM5REPEAT: 11..15 = enum TSM5REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 6
    rw TSM6 @ 0x6c: u16 = 0_0 {
        /// Input channel select
        TSM6CH: 0..1 = enum TSM6CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM6LCEN: 2 = struct TSM6LCEN(bool);
        /// Excitation and sample-and-hold
        TSM6EX: 3 = struct TSM6EX(bool);
        /// TSM comparator on
        TSM6CA: 4 = struct TSM6CA(bool);
        /// High-frequency clock on
        TSM6CLKON: 5 = struct TSM6CLKON(bool);
        /// Internal output latches enabled
        TSM6RSON: 6 = struct TSM6RSON(bool);
        /// TSM test cycle control
        TSM6TESTS1: 7 = struct TSM6TESTS1(bool);
        /// TSM DAC on
        TSM6DAC: 8 = struct TSM6DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM6STOP: 9 = struct TSM6STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM6CLK: 10 = struct TSM6CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM6REPEAT: 11..15 = enum TSM6REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 7
    rw TSM7 @ 0x6e: u16 = 0_0 {
        /// Input channel select
        TSM7CH: 0..1 = enum TSM7CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM7LCEN: 2 = struct TSM7LCEN(bool);
        /// Excitation and sample-and-hold
        TSM7EX: 3 = struct TSM7EX(bool);
        /// TSM comparator on
        TSM7CA: 4 = struct TSM7CA(bool);
        /// High-frequency clock on
        TSM7CLKON: 5 = struct TSM7CLKON(bool);
        /// Internal output latches enabled
        TSM7RSON: 6 = struct TSM7RSON(bool);
        /// TSM test cycle control
        TSM7TESTS1: 7 = struct TSM7TESTS1(bool);
        /// TSM DAC on
        TSM7DAC: 8 = struct TSM7DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM7STOP: 9 = struct TSM7STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM7CLK: 10 = struct TSM7CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM7REPEAT: 11..15 = enum TSM7REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 8
    rw TSM8 @ 0x70: u16 = 0_0 {
        /// Input channel select
        TSM8CH: 0..1 = enum TSM8CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM8LCEN: 2 = struct TSM8LCEN(bool);
        /// Excitation and sample-and-hold
        TSM8EX: 3 = struct TSM8EX(bool);
        /// TSM comparator on
        TSM8CA: 4 = struct TSM8CA(bool);
        /// High-frequency clock on
        TSM8CLKON: 5 = struct TSM8CLKON(bool);
        /// Internal output latches enabled
        TSM8RSON: 6 = struct TSM8RSON(bool);
        /// TSM test cycle control
        TSM8TESTS1: 7 = struct TSM8TESTS1(bool);
        /// TSM DAC on
        TSM8DAC: 8 = struct TSM8DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM8STOP: 9 = struct TSM8STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM8CLK: 10 = struct TSM8CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM8REPEAT: 11..15 = enum TSM8REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 9
    rw TSM9 @ 0x72: u16 = 0_0 {
        /// Input channel select
        TSM9CH: 0..1 = enum TSM9CH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        TSM9LCEN: 2 = struct TSM9LCEN(bool);
        /// Excitation and sample-and-hold
        TSM9EX: 3 = struct TSM9EX(bool);
        /// TSM comparator on
        TSM9CA: 4 = struct TSM9CA(bool);
        /// High-frequency clock on
        TSM9CLKON: 5 = struct TSM9CLKON(bool);
        /// Internal output latches enabled
        TSM9RSON: 6 = struct TSM9RSON(bool);
        /// TSM test cycle control
        TSM9TESTS1: 7 = struct TSM9TESTS1(bool);
        /// TSM DAC on
        TSM9DAC: 8 = struct TSM9DAC(bool);
        /// This bit indicates the end of the TSM sequence
        TSM9STOP: 9 = struct TSM9STOP(bool);
        /// This bit selects the clock source for the TSM
        TSM9CLK: 10 = struct TSM9CLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        TSM9REPEAT: 11..15 = enum TSM9REPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 10
    rw TSM10 @ 0x74: u16 = 0_0 {
        /// Input channel select
        ESITSM10_ESICH: 0..1 = enum ESITSM10_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM10_ESILCEN: 2 = struct ESITSM10_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM10_ESIEX: 3 = struct ESITSM10_ESIEX(bool);
        /// TSM comparator on
        ESITSM10_ESICA: 4 = struct ESITSM10_ESICA(bool);
        /// High-frequency clock on
        ESITSM10_ESICLKON: 5 = struct ESITSM10_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM10_ESIRSON: 6 = struct ESITSM10_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM10_ESITESTS1: 7 = struct ESITSM10_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM10_ESIDAC: 8 = struct ESITSM10_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM10_ESISTOP: 9 = struct ESITSM10_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM10_ESICLK: 10 = struct ESITSM10_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM10_ESIREPEAT: 11..15 = enum ESITSM10_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 11
    rw TSM11 @ 0x76: u16 = 0_0 {
        /// Input channel select
        ESITSM11_ESICH: 0..1 = enum ESITSM11_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM11_ESILCEN: 2 = struct ESITSM11_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM11_ESIEX: 3 = struct ESITSM11_ESIEX(bool);
        /// TSM comparator on
        ESITSM11_ESICA: 4 = struct ESITSM11_ESICA(bool);
        /// High-frequency clock on
        ESITSM11_ESICLKON: 5 = struct ESITSM11_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM11_ESIRSON: 6 = struct ESITSM11_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM11_ESITESTS1: 7 = struct ESITSM11_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM11_ESIDAC: 8 = struct ESITSM11_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM11_ESISTOP: 9 = struct ESITSM11_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM11_ESICLK: 10 = struct ESITSM11_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM11_ESIREPEAT: 11..15 = enum ESITSM11_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 12
    rw TSM12 @ 0x78: u16 = 0_0 {
        /// Input channel select
        ESITSM12_ESICH: 0..1 = enum ESITSM12_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM12_ESILCEN: 2 = struct ESITSM12_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM12_ESIEX: 3 = struct ESITSM12_ESIEX(bool);
        /// TSM comparator on
        ESITSM12_ESICA: 4 = struct ESITSM12_ESICA(bool);
        /// High-frequency clock on
        ESITSM12_ESICLKON: 5 = struct ESITSM12_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM12_ESIRSON: 6 = struct ESITSM12_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM12_ESITESTS1: 7 = struct ESITSM12_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM12_ESIDAC: 8 = struct ESITSM12_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM12_ESISTOP: 9 = struct ESITSM12_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM12_ESICLK: 10 = struct ESITSM12_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM12_ESIREPEAT: 11..15 = enum ESITSM12_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 13
    rw TSM13 @ 0x7a: u16 = 0_0 {
        /// Input channel select
        ESITSM13_ESICH: 0..1 = enum ESITSM13_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM13_ESILCEN: 2 = struct ESITSM13_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM13_ESIEX: 3 = struct ESITSM13_ESIEX(bool);
        /// TSM comparator on
        ESITSM13_ESICA: 4 = struct ESITSM13_ESICA(bool);
        /// High-frequency clock on
        ESITSM13_ESICLKON: 5 = struct ESITSM13_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM13_ESIRSON: 6 = struct ESITSM13_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM13_ESITESTS1: 7 = struct ESITSM13_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM13_ESIDAC: 8 = struct ESITSM13_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM13_ESISTOP: 9 = struct ESITSM13_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM13_ESICLK: 10 = struct ESITSM13_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM13_ESIREPEAT: 11..15 = enum ESITSM13_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 14
    rw TSM14 @ 0x7c: u16 = 0_0 {
        /// Input channel select
        ESITSM14_ESICH: 0..1 = enum ESITSM14_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM14_ESILCEN: 2 = struct ESITSM14_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM14_ESIEX: 3 = struct ESITSM14_ESIEX(bool);
        /// TSM comparator on
        ESITSM14_ESICA: 4 = struct ESITSM14_ESICA(bool);
        /// High-frequency clock on
        ESITSM14_ESICLKON: 5 = struct ESITSM14_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM14_ESIRSON: 6 = struct ESITSM14_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM14_ESITESTS1: 7 = struct ESITSM14_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM14_ESIDAC: 8 = struct ESITSM14_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM14_ESISTOP: 9 = struct ESITSM14_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM14_ESICLK: 10 = struct ESITSM14_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM14_ESIREPEAT: 11..15 = enum ESITSM14_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 15
    rw TSM15 @ 0x7e: u16 = 0_0 {
        /// Input channel select
        ESITSM15_ESICH: 0..1 = enum ESITSM15_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM15_ESILCEN: 2 = struct ESITSM15_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM15_ESIEX: 3 = struct ESITSM15_ESIEX(bool);
        /// TSM comparator on
        ESITSM15_ESICA: 4 = struct ESITSM15_ESICA(bool);
        /// High-frequency clock on
        ESITSM15_ESICLKON: 5 = struct ESITSM15_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM15_ESIRSON: 6 = struct ESITSM15_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM15_ESITESTS1: 7 = struct ESITSM15_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM15_ESIDAC: 8 = struct ESITSM15_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM15_ESISTOP: 9 = struct ESITSM15_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM15_ESICLK: 10 = struct ESITSM15_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM15_ESIREPEAT: 11..15 = enum ESITSM15_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 16
    rw TSM16 @ 0x80: u16 = 0_0 {
        /// Input channel select
        ESITSM16_ESICH: 0..1 = enum ESITSM16_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM16_ESILCEN: 2 = struct ESITSM16_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM16_ESIEX: 3 = struct ESITSM16_ESIEX(bool);
        /// TSM comparator on
        ESITSM16_ESICA: 4 = struct ESITSM16_ESICA(bool);
        /// High-frequency clock on
        ESITSM16_ESICLKON: 5 = struct ESITSM16_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM16_ESIRSON: 6 = struct ESITSM16_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM16_ESITESTS1: 7 = struct ESITSM16_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM16_ESIDAC: 8 = struct ESITSM16_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM16_ESISTOP: 9 = struct ESITSM16_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM16_ESICLK: 10 = struct ESITSM16_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM16_ESIREPEAT: 11..15 = enum ESITSM16_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 17
    rw TSM17 @ 0x82: u16 = 0_0 {
        /// Input channel select
        ESITSM17_ESICH: 0..1 = enum ESITSM17_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM17_ESILCEN: 2 = struct ESITSM17_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM17_ESIEX: 3 = struct ESITSM17_ESIEX(bool);
        /// TSM comparator on
        ESITSM17_ESICA: 4 = struct ESITSM17_ESICA(bool);
        /// High-frequency clock on
        ESITSM17_ESICLKON: 5 = struct ESITSM17_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM17_ESIRSON: 6 = struct ESITSM17_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM17_ESITESTS1: 7 = struct ESITSM17_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM17_ESIDAC: 8 = struct ESITSM17_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM17_ESISTOP: 9 = struct ESITSM17_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM17_ESICLK: 10 = struct ESITSM17_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM17_ESIREPEAT: 11..15 = enum ESITSM17_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 18
    rw TSM18 @ 0x84: u16 = 0_0 {
        /// Input channel select
        ESITSM18_ESICH: 0..1 = enum ESITSM18_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM18_ESILCEN: 2 = struct ESITSM18_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM18_ESIEX: 3 = struct ESITSM18_ESIEX(bool);
        /// TSM comparator on
        ESITSM18_ESICA: 4 = struct ESITSM18_ESICA(bool);
        /// High-frequency clock on
        ESITSM18_ESICLKON: 5 = struct ESITSM18_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM18_ESIRSON: 6 = struct ESITSM18_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM18_ESITESTS1: 7 = struct ESITSM18_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM18_ESIDAC: 8 = struct ESITSM18_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM18_ESISTOP: 9 = struct ESITSM18_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM18_ESICLK: 10 = struct ESITSM18_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM18_ESIREPEAT: 11..15 = enum ESITSM18_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 19
    rw TSM19 @ 0x86: u16 = 0_0 {
        /// Input channel select
        ESITSM19_ESICH: 0..1 = enum ESITSM19_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM19_ESILCEN: 2 = struct ESITSM19_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM19_ESIEX: 3 = struct ESITSM19_ESIEX(bool);
        /// TSM comparator on
        ESITSM19_ESICA: 4 = struct ESITSM19_ESICA(bool);
        /// High-frequency clock on
        ESITSM19_ESICLKON: 5 = struct ESITSM19_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM19_ESIRSON: 6 = struct ESITSM19_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM19_ESITESTS1: 7 = struct ESITSM19_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM19_ESIDAC: 8 = struct ESITSM19_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM19_ESISTOP: 9 = struct ESITSM19_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM19_ESICLK: 10 = struct ESITSM19_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM19_ESIREPEAT: 11..15 = enum ESITSM19_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 20
    rw TSM20 @ 0x88: u16 = 0_0 {
        /// Input channel select
        ESITSM20_ESICH: 0..1 = enum ESITSM20_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM20_ESILCEN: 2 = struct ESITSM20_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM20_ESIEX: 3 = struct ESITSM20_ESIEX(bool);
        /// TSM comparator on
        ESITSM20_ESICA: 4 = struct ESITSM20_ESICA(bool);
        /// High-frequency clock on
        ESITSM20_ESICLKON: 5 = struct ESITSM20_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM20_ESIRSON: 6 = struct ESITSM20_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM20_ESITESTS1: 7 = struct ESITSM20_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM20_ESIDAC: 8 = struct ESITSM20_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM20_ESISTOP: 9 = struct ESITSM20_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM20_ESICLK: 10 = struct ESITSM20_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM20_ESIREPEAT: 11..15 = enum ESITSM20_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 21
    rw TSM21 @ 0x8a: u16 = 0_0 {
        /// Input channel select
        ESITSM21_ESICH: 0..1 = enum ESITSM21_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM21_ESILCEN: 2 = struct ESITSM21_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM21_ESIEX: 3 = struct ESITSM21_ESIEX(bool);
        /// TSM comparator on
        ESITSM21_ESICA: 4 = struct ESITSM21_ESICA(bool);
        /// High-frequency clock on
        ESITSM21_ESICLKON: 5 = struct ESITSM21_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM21_ESIRSON: 6 = struct ESITSM21_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM21_ESITESTS1: 7 = struct ESITSM21_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM21_ESIDAC: 8 = struct ESITSM21_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM21_ESISTOP: 9 = struct ESITSM21_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM21_ESICLK: 10 = struct ESITSM21_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM21_ESIREPEAT: 11..15 = enum ESITSM21_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 22
    rw TSM22 @ 0x8c: u16 = 0_0 {
        /// Input channel select
        ESITSM22_ESICH: 0..1 = enum ESITSM22_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM22_ESILCEN: 2 = struct ESITSM22_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM22_ESIEX: 3 = struct ESITSM22_ESIEX(bool);
        /// TSM comparator on
        ESITSM22_ESICA: 4 = struct ESITSM22_ESICA(bool);
        /// High-frequency clock on
        ESITSM22_ESICLKON: 5 = struct ESITSM22_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM22_ESIRSON: 6 = struct ESITSM22_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM22_ESITESTS1: 7 = struct ESITSM22_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM22_ESIDAC: 8 = struct ESITSM22_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM22_ESISTOP: 9 = struct ESITSM22_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM22_ESICLK: 10 = struct ESITSM22_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM22_ESIREPEAT: 11..15 = enum ESITSM22_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 23
    rw TSM23 @ 0x8e: u16 = 0_0 {
        /// Input channel select
        ESITSM23_ESICH: 0..1 = enum ESITSM23_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM23_ESILCEN: 2 = struct ESITSM23_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM23_ESIEX: 3 = struct ESITSM23_ESIEX(bool);
        /// TSM comparator on
        ESITSM23_ESICA: 4 = struct ESITSM23_ESICA(bool);
        /// High-frequency clock on
        ESITSM23_ESICLKON: 5 = struct ESITSM23_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM23_ESIRSON: 6 = struct ESITSM23_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM23_ESITESTS1: 7 = struct ESITSM23_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM23_ESIDAC: 8 = struct ESITSM23_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM23_ESISTOP: 9 = struct ESITSM23_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM23_ESICLK: 10 = struct ESITSM23_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM23_ESIREPEAT: 11..15 = enum ESITSM23_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 24
    rw TSM24 @ 0x90: u16 = 0_0 {
        /// Input channel select
        ESITSM24_ESICH: 0..1 = enum ESITSM24_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM24_ESILCEN: 2 = struct ESITSM24_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM24_ESIEX: 3 = struct ESITSM24_ESIEX(bool);
        /// TSM comparator on
        ESITSM24_ESICA: 4 = struct ESITSM24_ESICA(bool);
        /// High-frequency clock on
        ESITSM24_ESICLKON: 5 = struct ESITSM24_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM24_ESIRSON: 6 = struct ESITSM24_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM24_ESITESTS1: 7 = struct ESITSM24_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM24_ESIDAC: 8 = struct ESITSM24_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM24_ESISTOP: 9 = struct ESITSM24_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM24_ESICLK: 10 = struct ESITSM24_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM24_ESIREPEAT: 11..15 = enum ESITSM24_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 25
    rw TSM25 @ 0x92: u16 = 0_0 {
        /// Input channel select
        ESITSM25_ESICH: 0..1 = enum ESITSM25_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM25_ESILCEN: 2 = struct ESITSM25_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM25_ESIEX: 3 = struct ESITSM25_ESIEX(bool);
        /// TSM comparator on
        ESITSM25_ESICA: 4 = struct ESITSM25_ESICA(bool);
        /// High-frequency clock on
        ESITSM25_ESICLKON: 5 = struct ESITSM25_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM25_ESIRSON: 6 = struct ESITSM25_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM25_ESITESTS1: 7 = struct ESITSM25_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM25_ESIDAC: 8 = struct ESITSM25_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM25_ESISTOP: 9 = struct ESITSM25_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM25_ESICLK: 10 = struct ESITSM25_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM25_ESIREPEAT: 11..15 = enum ESITSM25_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 26
    rw TSM26 @ 0x94: u16 = 0_0 {
        /// Input channel select
        ESITSM26_ESICH: 0..1 = enum ESITSM26_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM26_ESILCEN: 2 = struct ESITSM26_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM26_ESIEX: 3 = struct ESITSM26_ESIEX(bool);
        /// TSM comparator on
        ESITSM26_ESICA: 4 = struct ESITSM26_ESICA(bool);
        /// High-frequency clock on
        ESITSM26_ESICLKON: 5 = struct ESITSM26_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM26_ESIRSON: 6 = struct ESITSM26_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM26_ESITESTS1: 7 = struct ESITSM26_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM26_ESIDAC: 8 = struct ESITSM26_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM26_ESISTOP: 9 = struct ESITSM26_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM26_ESICLK: 10 = struct ESITSM26_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM26_ESIREPEAT: 11..15 = enum ESITSM26_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 27
    rw TSM27 @ 0x96: u16 = 0_0 {
        /// Input channel select
        ESITSM27_ESICH: 0..1 = enum ESITSM27_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM27_ESILCEN: 2 = struct ESITSM27_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM27_ESIEX: 3 = struct ESITSM27_ESIEX(bool);
        /// TSM comparator on
        ESITSM27_ESICA: 4 = struct ESITSM27_ESICA(bool);
        /// High-frequency clock on
        ESITSM27_ESICLKON: 5 = struct ESITSM27_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM27_ESIRSON: 6 = struct ESITSM27_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM27_ESITESTS1: 7 = struct ESITSM27_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM27_ESIDAC: 8 = struct ESITSM27_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM27_ESISTOP: 9 = struct ESITSM27_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM27_ESICLK: 10 = struct ESITSM27_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM27_ESIREPEAT: 11..15 = enum ESITSM27_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 28
    rw TSM28 @ 0x98: u16 = 0_0 {
        /// Input channel select
        ESITSM28_ESICH: 0..1 = enum ESITSM28_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM28_ESILCEN: 2 = struct ESITSM28_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM28_ESIEX: 3 = struct ESITSM28_ESIEX(bool);
        /// TSM comparator on
        ESITSM28_ESICA: 4 = struct ESITSM28_ESICA(bool);
        /// High-frequency clock on
        ESITSM28_ESICLKON: 5 = struct ESITSM28_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM28_ESIRSON: 6 = struct ESITSM28_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM28_ESITESTS1: 7 = struct ESITSM28_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM28_ESIDAC: 8 = struct ESITSM28_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM28_ESISTOP: 9 = struct ESITSM28_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM28_ESICLK: 10 = struct ESITSM28_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM28_ESIREPEAT: 11..15 = enum ESITSM28_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 29
    rw TSM29 @ 0x9a: u16 = 0_0 {
        /// Input channel select
        ESITSM29_ESICH: 0..1 = enum ESITSM29_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM29_ESILCEN: 2 = struct ESITSM29_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM29_ESIEX: 3 = struct ESITSM29_ESIEX(bool);
        /// TSM comparator on
        ESITSM29_ESICA: 4 = struct ESITSM29_ESICA(bool);
        /// High-frequency clock on
        ESITSM29_ESICLKON: 5 = struct ESITSM29_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM29_ESIRSON: 6 = struct ESITSM29_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM29_ESITESTS1: 7 = struct ESITSM29_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM29_ESIDAC: 8 = struct ESITSM29_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM29_ESISTOP: 9 = struct ESITSM29_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM29_ESICLK: 10 = struct ESITSM29_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM29_ESIREPEAT: 11..15 = enum ESITSM29_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 30
    rw TSM30 @ 0x9c: u16 = 0_0 {
        /// Input channel select
        ESITSM30_ESICH: 0..1 = enum ESITSM30_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM30_ESILCEN: 2 = struct ESITSM30_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM30_ESIEX: 3 = struct ESITSM30_ESIEX(bool);
        /// TSM comparator on
        ESITSM30_ESICA: 4 = struct ESITSM30_ESICA(bool);
        /// High-frequency clock on
        ESITSM30_ESICLKON: 5 = struct ESITSM30_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM30_ESIRSON: 6 = struct ESITSM30_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM30_ESITESTS1: 7 = struct ESITSM30_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM30_ESIDAC: 8 = struct ESITSM30_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM30_ESISTOP: 9 = struct ESITSM30_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM30_ESICLK: 10 = struct ESITSM30_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM30_ESIREPEAT: 11..15 = enum ESITSM30_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 31
    rw TSM31 @ 0x9e: u16 = 0_0 {
        /// Input channel select
        ESITSM31_ESICH: 0..1 = enum ESITSM31_ESICH {
            /// Input channel select: ESICH0
            CH_0 = 0b00,
            /// Input channel select: ESICH1
            CH_1 = 0b01,
            /// Input channel select: ESICH2
            CH_2 = 0b10,
            /// Input channel select: ESICH3
            CH_3 = 0b11,
        }
        /// LC enable
        ESITSM31_ESILCEN: 2 = struct ESITSM31_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM31_ESIEX: 3 = struct ESITSM31_ESIEX(bool);
        /// TSM comparator on
        ESITSM31_ESICA: 4 = struct ESITSM31_ESICA(bool);
        /// High-frequency clock on
        ESITSM31_ESICLKON: 5 = struct ESITSM31_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM31_ESIRSON: 6 = struct ESITSM31_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM31_ESITESTS1: 7 = struct ESITSM31_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM31_ESIDAC: 8 = struct ESITSM31_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM31_ESISTOP: 9 = struct ESITSM31_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM31_ESICLK: 10 = struct ESITSM31_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM31_ESIREPEAT: 11..15 = enum ESITSM31_ESIREPEAT {
            /// These bits configure the duration of this state
            REPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            REPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            REPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            REPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            REPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            REPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            REPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            REPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            REPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            REPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            REPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            REPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            REPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            REPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            REPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            REPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            REPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            REPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            REPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            REPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            REPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            REPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            REPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            REPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            REPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            REPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            REPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            REPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            REPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            REPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            REPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            REPEAT_31 = 0b11111,
        }
    }
}
