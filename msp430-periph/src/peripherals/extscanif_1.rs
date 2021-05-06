//! ExtScanIF

utils::periph! {
    /// ExtScanIF
    ExtScanIF;
    /// ESI debug register 1
    rw ESIDEBUG1 @ 0x00: u16 = 0_0 {
        /// ESI debug register 1
        ESIDEBUG1: 0..15 = struct ESIDEBUG1Field(u16);
    }
    /// ESI debug register 2
    rw ESIDEBUG2 @ 0x02: u16 = 0_0 {
        /// ESI debug register 2
        ESIDEBUG2: 0..15 = struct ESIDEBUG2Field(u16);
    }
    /// ESI debug register 3
    rw ESIDEBUG3 @ 0x04: u16 = 0_0 {
        /// ESI debug register 3
        ESIDEBUG3: 0..15 = struct ESIDEBUG3Field(u16);
    }
    /// ESI debug register 4
    rw ESIDEBUG4 @ 0x06: u16 = 0_0 {
        /// ESI debug register 4
        ESIDEBUG4: 0..15 = struct ESIDEBUG4Field(u16);
    }
    /// ESI debug register 5
    rw ESIDEBUG5 @ 0x08: u16 = 0_0 {
        /// ESI debug register 5
        ESIDEBUG5: 0..15 = struct ESIDEBUG5Field(u16);
    }
    /// ESI PSM counter 0
    rw ESICNT0 @ 0x10: u16 = 0_0 {
        /// ESI PSM counter 0
        ESICNT0: 0..15 = struct ESICNT0Field(u16);
    }
    /// ESI PSM counter 1
    rw ESICNT1 @ 0x12: u16 = 0_0 {
        /// ESI PSM counter 1
        ESICNT1: 0..15 = struct ESICNT1Field(u16);
    }
    /// ESI PSM counter 2
    rw ESICNT2 @ 0x14: u16 = 0_0 {
        /// ESI PSM counter 2
        ESICNT2: 0..15 = struct ESICNT2Field(u16);
    }
    /// ESI oscillator counter register
    rw ESICNT3 @ 0x16: u16 = 0_0 {
        /// ESI oscillator counter register
        ESICNT3: 0..15 = struct ESICNT3Field(u16);
    }
    /// ESI interrupt vector
    rw ESIIV @ 0x1a: u16 = 0_0 {
        /// ESI interrupt vector
        ESIIV: 0..15 = struct ESIIVField(u16);
    }
    /// ESI interrupt register 1
    rw ESIINT1 @ 0x1c: u16 = 0_0 {
        /// Interrupt enable
        ESIIE0: 0 = struct ESIIE0(bool);
        /// Interrupt enable
        ESIIE1: 1 = struct ESIIE1(bool);
        /// Interrupt enable
        ESIIE2: 2 = struct ESIIE2(bool);
        /// Interrupt enable
        ESIIE3: 3 = struct ESIIE3(bool);
        /// Interrupt enable
        ESIIE4: 4 = struct ESIIE4(bool);
        /// Interrupt enable
        ESIIE5: 5 = struct ESIIE5(bool);
        /// Interrupt enable
        ESIIE6: 6 = struct ESIIE6(bool);
        /// Interrupt enable
        ESIIE7: 7 = struct ESIIE7(bool);
        /// Interrupt enable
        ESIIE8: 8 = struct ESIIE8(bool);
        /// ESIIFG0 interrupt flag source
        ESIIFGSET1: 10..12 = enum ESIIFGSET1 {
            /// ESIIFG0 is set when ESIOUT0 is set
            ESIIFGSET1_0 = 0b000,
            /// ESIIFG0 is set when ESIOUT0 is reset
            ESIIFGSET1_1 = 0b001,
            /// ESIIFG0 is set when ESIOUT1 is set
            ESIIFGSET1_2 = 0b010,
            /// ESIIFG0 is set when ESIOUT1 is reset
            ESIIFGSET1_3 = 0b011,
            /// ESIIFG0 is set when ESIOUT2 is set
            ESIIFGSET1_4 = 0b100,
            /// ESIIFG0 is set when ESIOUT2 is reset
            ESIIFGSET1_5 = 0b101,
            /// ESIIFG0 is set when ESIOUT3 is set
            ESIIFGSET1_6 = 0b110,
            /// ESIIFG0 is set when ESIOUT3 is reset
            ESIIFGSET1_7 = 0b111,
        }
        /// ESIIFG8 interrupt flag source
        ESIIFGSET2: 13..15 = enum ESIIFGSET2 {
            /// ESIIFG8 is set when ESIOUT4 is set
            ESIIFGSET2_0 = 0b000,
            /// ESIIFG8 is set when ESIOUT4 is reset
            ESIIFGSET2_1 = 0b001,
            /// ESIIFG8 is set when ESIOUT5 is set
            ESIIFGSET2_2 = 0b010,
            /// ESIIFG8 is set when ESIOUT5 is reset
            ESIIFGSET2_3 = 0b011,
            /// ESIIFG8 is set when ESIOUT6 is set
            ESIIFGSET2_4 = 0b100,
            /// ESIIFG8 is set when ESIOUT6 is reset
            ESIIFGSET2_5 = 0b101,
            /// ESIIFG8 is set when ESIOUT7 is set
            ESIIFGSET2_6 = 0b110,
            /// ESIIFG8 is set when ESIOUT7 is reset
            ESIIFGSET2_7 = 0b111,
        }
    }
    /// ESI interrupt register 2
    rw ESIINT2 @ 0x1e: u16 = 0_0 {
        /// ESIIFG0 interrupt pending
        ESIIFG0: 0 = struct ESIIFG0(bool);
        /// ESIIFG1 interrupt pending
        ESIIFG1: 1 = struct ESIIFG1(bool);
        /// ESIIFG2 interrupt pending
        ESIIFG2: 2 = struct ESIIFG2(bool);
        /// ESIIFG3 interrupt pending
        ESIIFG3: 3 = struct ESIIFG3(bool);
        /// ESIIFG4 interrupt pending
        ESIIFG4: 4 = struct ESIIFG4(bool);
        /// ESIIFG5 interrupt pending
        ESIIFG5: 5 = struct ESIIFG5(bool);
        /// ESIIFG6 interrupt pending
        ESIIFG6: 6 = struct ESIIFG6(bool);
        /// ESIIFG7 interrupt pending
        ESIIFG7: 7 = struct ESIIFG7(bool);
        /// ESIIFG8 interrupt pending
        ESIIFG8: 8 = struct ESIIFG8(bool);
        /// SIFIFG7 interrupt flag source
        ESIIS0: 10..11 = enum ESIIS0 {
            /// SIFIFG7 interrupt flag source: SIFCNT0
            ESIIS0_0 = 0b00,
            /// SIFIFG7 interrupt flag source: SIFCNT0 MOD 4
            ESIIS0_1 = 0b01,
            /// SIFIFG7 interrupt flag source: SIFCNT0 MOD 256
            ESIIS0_2 = 0b10,
            /// SIFIFG7 interrupt flag source: SIFCNT0 increments from FFFFh to 00h
            ESIIS0_3 = 0b11,
        }
        /// SIFIFG4 interrupt flag source
        ESIIS2: 13..14 = enum ESIIS2 {
            /// SIFIFG4 interrupt flag source: SIFCNT2
            ESIIS2_0 = 0b00,
            /// SIFIFG4 interrupt flag source: SIFCNT2 MOD 4
            ESIIS2_1 = 0b01,
            /// SIFIFG4 interrupt flag source: SIFCNT2 MOD 256
            ESIIS2_2 = 0b10,
            /// SIFIFG4 interrupt flag source: SIFCNT2 decrements from 01h to 00h
            ESIIS2_3 = 0b11,
        }
    }
    /// ESI AFE control register
    rw ESIAFE @ 0x20: u16 = 0_0 {
        /// Excitation enable
        ESITEN: 0 = struct ESITEN(bool);
        /// Sample-and-hold enable
        ESISH: 1 = struct ESISH(bool);
        /// Mid-voltage generator
        ESIVMIDEN: 2 = struct ESIVMIDEN(bool);
        /// Sample-and-hold ESIVSS select
        ESISHTSM: 3 = struct ESISHTSM(bool);
        /// Comparator input select for AFE1 only
        ESICACI3: 4 = struct ESICACI3(bool);
        /// Comparator input select for AFE1 only
        ESICISEL: 5 = struct ESICISEL(bool);
        /// AFE1's comparator input select
        ESICA1X: 6 = struct ESICA1X(bool);
        /// AFE2's comparator input select
        ESICA2X: 7 = struct ESICA2X(bool);
        /// Invert AFE1's comparator output
        ESICA1INV: 8 = struct ESICA1INV(bool);
        /// Invert AFE2's comparator output
        ESICA2INV: 9 = struct ESICA2INV(bool);
        /// Enable ESICA(tsm) control for comparator in AFE2
        ESICA2EN: 10 = struct ESICA2EN(bool);
        /// Enable ESIDAC(tsm) control for DAC in AFE2
        ESIDAC2EN: 11 = struct ESIDAC2EN(bool);
    }
    /// ESI PPU control register
    rw ESIPPU @ 0x22: u16 = 0_0 {
        /// Latched AFE1 comparator output when ESICH0 input is selected
        ESIOUT0: 0 = struct ESIOUT0(bool);
        /// Latched AFE1 comparator output when ESICH1 input is selected
        ESIOUT1: 1 = struct ESIOUT1(bool);
        /// Latched AFE1 comparator output when ESICH2 input is selected
        ESIOUT2: 2 = struct ESIOUT2(bool);
        /// Latched AFE1 comparator output when ESICH3 input is selected
        ESIOUT3: 3 = struct ESIOUT3(bool);
        /// Latched AFE2 comparator output when ESICH0 input is selected
        ESIOUT4: 4 = struct ESIOUT4(bool);
        /// Latched AFE2 comparator output when ESICH1 input is selected
        ESIOUT5: 5 = struct ESIOUT5(bool);
        /// Latched AFE2 comparator output when ESICH2 input is selected
        ESIOUT6: 6 = struct ESIOUT6(bool);
        /// Latched AFE2 comparator output when ESICH3 input is selected
        ESIOUT7: 7 = struct ESIOUT7(bool);
        /// Lachted AFE1 comparator output for test channel 0
        ESITCHOUT0: 8 = struct ESITCHOUT0(bool);
        /// Latched AFE1 comparator output for test channel 1
        ESITCHOUT1: 9 = struct ESITCHOUT1(bool);
    }
    /// ESI TSM control register
    rw ESITSM @ 0x24: u16 = 0_0 {
        /// TSM SMCLK divider
        ESIDIV1: 0..1 = enum ESIDIV1 {
            /// TSM SMCLK/ESIOSC divider mode: 0
            ESIDIV1_0 = 0b00,
            /// TSM SMCLK/ESIOSC divider mode: 1
            ESIDIV1_1 = 0b01,
            /// TSM SMCLK/ESIOSC divider mode: 2
            ESIDIV1_2 = 0b10,
            /// TSM SMCLK/ESIOSC divider mode: 3
            ESIDIV1_3 = 0b11,
        }
        /// ACLK divider
        ESIDIV2: 2..3 = enum ESIDIV2 {
            /// ACLK divider mode: 0
            ESIDIV2_0 = 0b00,
            /// ACLK divider mode: 1
            ESIDIV2_1 = 0b01,
            /// ACLK divider mode: 2
            ESIDIV2_2 = 0b10,
            /// ACLK divider mode: 3
            ESIDIV2_3 = 0b11,
        }
        /// TSM start trigger ACLK divider
        ESIDIV3A: 4..6 = enum ESIDIV3A {
            /// TSM start trigger ACLK divider
            ESIDIV3A_0 = 0b000,
            /// TSM start trigger ACLK divider
            ESIDIV3A_1 = 0b001,
            /// TSM start trigger ACLK divider
            ESIDIV3A_2 = 0b010,
            /// TSM start trigger ACLK divider
            ESIDIV3A_3 = 0b011,
            /// TSM start trigger ACLK divider
            ESIDIV3A_4 = 0b100,
            /// TSM start trigger ACLK divider
            ESIDIV3A_5 = 0b101,
            /// TSM start trigger ACLK divider
            ESIDIV3A_6 = 0b110,
            /// TSM start trigger ACLK divider
            ESIDIV3A_7 = 0b111,
        }
        /// TSM start trigger ACLK divider
        ESIDIV3B: 7..9 = enum ESIDIV3B {
            /// TSM start trigger ACLK divider
            ESIDIV3B_0 = 0b000,
            /// TSM start trigger ACLK divider
            ESIDIV3B_1 = 0b001,
            /// TSM start trigger ACLK divider
            ESIDIV3B_2 = 0b010,
            /// TSM start trigger ACLK divider
            ESIDIV3B_3 = 0b011,
            /// TSM start trigger ACLK divider
            ESIDIV3B_4 = 0b100,
            /// TSM start trigger ACLK divider
            ESIDIV3B_5 = 0b101,
            /// TSM start trigger ACLK divider
            ESIDIV3B_6 = 0b110,
            /// TSM start trigger ACLK divider
            ESIDIV3B_7 = 0b111,
        }
        /// TSM repeat modee
        ESITSMRP: 10 = struct ESITSMRP(bool);
        /// TSM software start trigger
        ESISTART: 11 = struct ESISTART(bool);
        /// TSM start trigger selection
        ESITSMTRG: 12..13 = enum ESITSMTRG {
            /// Halt mode
            ESITSMTRG_0 = 0b00,
            /// TSM start trigger ACLK divider
            ESITSMTRG_1 = 0b01,
            /// Software trigger for TSM
            ESITSMTRG_2 = 0b10,
            /// Either the ACLK divider or the ESISTART biT
            ESITSMTRG_3 = 0b11,
        }
        /// Functionality selection of ESITSMx bit5
        ESICLKAZSEL: 14 = struct ESICLKAZSEL(bool);
    }
    /// ESI PSM control register
    rw ESIPSM @ 0x26: u16 = 0_0 {
        /// Q6 enable
        ESIQ6EN: 0 = struct ESIQ6EN(bool);
        /// Enabling to use Q7 as trigger for a TSM sequence
        ESIQ7TRG: 2 = struct ESIQ7TRG(bool);
        /// ESICNT0 enable (up counter)
        ESICNT0EN: 3 = struct ESICNT0EN(bool);
        /// ESICNT1 enable (up/down counter)
        ESICNT1EN: 4 = struct ESICNT1EN(bool);
        /// ESICNT2 enable (down counter)
        ESICNT2EN: 5 = struct ESICNT2EN(bool);
        /// Source Selection for V2 bit
        ESIV2SEL: 7 = struct ESIV2SEL(bool);
        /// Output signal selection for SIFTEST4 pin
        ESITEST4SEL: 8..9 = enum ESITEST4SEL {
            /// Q1 signal from PSM table
            ESITEST4SEL_0 = 0b00,
            /// Q2 signal from PSM table
            ESITEST4SEL_1 = 0b01,
            /// TSM clock signal from Timing State Machine
            ESITEST4SEL_2 = 0b10,
            /// AFE1's comparator output signal Comp1Out
            ESITEST4SEL_3 = 0b11,
        }
        /// ESI Counter 0 reset
        ESICNT0RST: 13 = struct ESICNT0RST(bool);
        /// ESI Counter 1 reset
        ESICNT1RST: 14 = struct ESICNT1RST(bool);
        /// ESI Counter 2 reset
        ESICNT2RST: 15 = struct ESICNT2RST(bool);
    }
    /// ESI oscillator control register
    rw ESIOSC @ 0x28: u16 = 0_0 {
        /// Internal oscillator enable
        ESIHFSEL: 0 = struct ESIHFSEL(bool);
        /// Internal oscillator control
        ESICLKGON: 1 = struct ESICLKGON(bool);
        /// Internal oscillator frequency adjust
        ESICLKFQ0: 8 = struct ESICLKFQ0(bool);
        /// Internal oscillator frequency adjust
        ESICLKFQ1: 9 = struct ESICLKFQ1(bool);
        /// Internal oscillator frequency adjust
        ESICLKFQ2: 10 = struct ESICLKFQ2(bool);
        /// Internal oscillator frequency adjust
        ESICLKFQ3: 11 = struct ESICLKFQ3(bool);
        /// Internal oscillator frequency adjust
        ESICLKFQ4: 12 = struct ESICLKFQ4(bool);
        /// Internal oscillator frequency adjust
        ESICLKFQ5: 13 = struct ESICLKFQ5(bool);
    }
    /// ESI control register
    rw ESICTL @ 0x2a: u16 = 0_0 {
        /// Extended Scan interface enable
        ESIEN: 0 = struct ESIEN(bool);
        /// Test cycle insertion
        ESITESTD: 1 = struct ESITESTD(bool);
        /// Comparator output/Timer_A input selection
        ESICS: 2 = struct ESICS(bool);
        /// select the comparator input for test channel 0
        ESITCH0: 3..4 = enum ESITCH0 {
            /// Comparator input is ESICH0 when ESICAX = 0; Comparator input is ESICI0 when ESICAX = 1
            ESITCH0_0 = 0b00,
            /// Comparator input is ESICH1 when ESICAX = 0; Comparator input is ESICI1 when ESICAX = 1
            ESITCH0_1 = 0b01,
            /// Comparator input is ESICH2 when ESICAX = 0; Comparator input is ESICI2 when ESICAX = 1
            ESITCH0_2 = 0b10,
            /// Comparator input is ESICH3 when ESICAX = 0; Comparator input is ESICI3 when ESICAX = 1
            ESITCH0_3 = 0b11,
        }
        /// select the comparator input for test channel 1
        ESITCH1: 5..6 = enum ESITCH1 {
            /// Comparator input is ESICH0 when ESICAX = 0; Comparator input is ESICI0 when ESICAX = 1
            ESITCH1_0 = 0b00,
            /// Comparator input is ESICH1 when ESICAX = 0; Comparator input is ESICI1 when ESICAX = 1
            ESITCH1_1 = 0b01,
            /// Comparator input is ESICH2 when ESICAX = 0; Comparator input is ESICI2 when ESICAX = 1
            ESITCH1_2 = 0b10,
            /// Comparator input is ESICH3 when ESICAX = 0; Comparator input is ESICI3 when ESICAX = 1
            ESITCH1_3 = 0b11,
        }
        /// PPUS1 source select
        ESIS1SEL: 7..9 = enum ESIS1SEL {
            /// ESIOUT0 is the PPUS1 source
            ESIS1SEL_0 = 0b000,
            /// ESIOUT1 is the PPUS1 source
            ESIS1SEL_1 = 0b001,
            /// ESIOUT2 is the PPUS1 source
            ESIS1SEL_2 = 0b010,
            /// ESIOUT3 is the PPUS1 source
            ESIS1SEL_3 = 0b011,
            /// ESIOUT4 is the PPUS1 source
            ESIS1SEL_4 = 0b100,
            /// ESIOUT5 is the PPUS1 source
            ESIS1SEL_5 = 0b101,
            /// ESIOUT6 is the PPUS1 source
            ESIS1SEL_6 = 0b110,
            /// ESIOUT7 is the PPUS1 source
            ESIS1SEL_7 = 0b111,
        }
        /// PPUS2 source select
        ESIS2SEL: 10..12 = enum ESIS2SEL {
            /// ESIOUT0 is the PPUS2 source
            ESIS2SEL_0 = 0b000,
            /// ESIOUT1 is the PPUS2 source
            ESIS2SEL_1 = 0b001,
            /// ESIOUT2 is the PPUS2 source
            ESIS2SEL_2 = 0b010,
            /// ESIOUT3 is the PPUS2 source
            ESIS2SEL_3 = 0b011,
            /// ESIOUT4 is the PPUS2 source
            ESIS2SEL_4 = 0b100,
            /// ESIOUT5 is the PPUS2 source
            ESIS2SEL_5 = 0b101,
            /// ESIOUT6 is the PPUS2 source
            ESIS2SEL_6 = 0b110,
            /// ESIOUT7 is the PPUS2 source
            ESIS2SEL_7 = 0b111,
        }
        /// PPUS3 source select
        ESIS3SEL: 13..15 = enum ESIS3SEL {
            /// ESIOUT0 is the PPUS3 source
            ESIS3SEL_0 = 0b000,
            /// ESIOUT1 is the PPUS3 source
            ESIS3SEL_1 = 0b001,
            /// ESIOUT2 is the PPUS3 source
            ESIS3SEL_2 = 0b010,
            /// ESIOUT3 is the PPUS3 source
            ESIS3SEL_3 = 0b011,
            /// ESIOUT4 is the PPUS3 source
            ESIS3SEL_4 = 0b100,
            /// ESIOUT5 is the PPUS3 source
            ESIS3SEL_5 = 0b101,
            /// ESIOUT6 is the PPUS3 source
            ESIS3SEL_6 = 0b110,
            /// ESIOUT7 is the PPUS3 source
            ESIS3SEL_7 = 0b111,
        }
    }
    /// ESI PSM Counter Threshold 1 register
    rw ESITHR1 @ 0x2c: u16 = 0_0 {
        /// ESI PSM Counter Threshold 1 register
        ESITHR1: 0..15 = struct ESITHR1Field(u16);
    }
    /// ESI PSM Counter Threshold 2 register
    rw ESITHR2 @ 0x2e: u16 = 0_0 {
        /// ESI PSM Counter Threshold 2 register
        ESITHR2: 0..15 = struct ESITHR2Field(u16);
    }
    /// ESI DAC1 register 0
    rw ESIDAC1R0 @ 0x40: u16 = 0_0 {
        /// ESI DAC1 register 0
        ESIDAC1R0: 0..15 = struct ESIDAC1R0Field(u16);
    }
    /// ESI DAC1 register 1
    rw ESIDAC1R1 @ 0x42: u16 = 0_0 {
        /// ESI DAC1 register 1
        ESIDAC1R1: 0..15 = struct ESIDAC1R1Field(u16);
    }
    /// ESI DAC1 register 2
    rw ESIDAC1R2 @ 0x44: u16 = 0_0 {
        /// ESI DAC1 register 2
        ESIDAC1R2: 0..15 = struct ESIDAC1R2Field(u16);
    }
    /// ESI DAC1 register 3
    rw ESIDAC1R3 @ 0x46: u16 = 0_0 {
        /// ESI DAC1 register 3
        ESIDAC1R3: 0..15 = struct ESIDAC1R3Field(u16);
    }
    /// ESI DAC1 register 4
    rw ESIDAC1R4 @ 0x48: u16 = 0_0 {
        /// ESI DAC1 register 4
        ESIDAC1R4: 0..15 = struct ESIDAC1R4Field(u16);
    }
    /// ESI DAC1 register 5
    rw ESIDAC1R5 @ 0x4a: u16 = 0_0 {
        /// ESI DAC1 register 5
        ESIDAC1R5: 0..15 = struct ESIDAC1R5Field(u16);
    }
    /// ESI DAC1 register 6
    rw ESIDAC1R6 @ 0x4c: u16 = 0_0 {
        /// ESI DAC1 register 6
        ESIDAC1R6: 0..15 = struct ESIDAC1R6Field(u16);
    }
    /// ESI DAC1 register 7
    rw ESIDAC1R7 @ 0x4e: u16 = 0_0 {
        /// ESI DAC1 register 7
        ESIDAC1R7: 0..15 = struct ESIDAC1R7Field(u16);
    }
    /// ESI DAC2 register 0
    rw ESIDAC2R0 @ 0x50: u16 = 0_0 {
        /// ESI DAC2 register 0
        ESIDAC2R0: 0..15 = struct ESIDAC2R0Field(u16);
    }
    /// ESI DAC2 register 1
    rw ESIDAC2R1 @ 0x52: u16 = 0_0 {
        /// ESI DAC2 register 1
        ESIDAC2R1: 0..15 = struct ESIDAC2R1Field(u16);
    }
    /// ESI DAC2 register 2
    rw ESIDAC2R2 @ 0x54: u16 = 0_0 {
        /// ESI DAC2 register 2
        ESIDAC2R2: 0..15 = struct ESIDAC2R2Field(u16);
    }
    /// ESI DAC2 register 3
    rw ESIDAC2R3 @ 0x56: u16 = 0_0 {
        /// ESI DAC2 register 3
        ESIDAC2R3: 0..15 = struct ESIDAC2R3Field(u16);
    }
    /// ESI DAC2 register 4
    rw ESIDAC2R4 @ 0x58: u16 = 0_0 {
        /// ESI DAC2 register 4
        ESIDAC2R4: 0..15 = struct ESIDAC2R4Field(u16);
    }
    /// ESI DAC2 register 5
    rw ESIDAC2R5 @ 0x5a: u16 = 0_0 {
        /// ESI DAC2 register 5
        ESIDAC2R5: 0..15 = struct ESIDAC2R5Field(u16);
    }
    /// ESI DAC2 register 6
    rw ESIDAC2R6 @ 0x5c: u16 = 0_0 {
        /// ESI DAC2 register 6
        ESIDAC2R6: 0..15 = struct ESIDAC2R6Field(u16);
    }
    /// ESI DAC2 register 7
    rw ESIDAC2R7 @ 0x5e: u16 = 0_0 {
        /// ESI DAC2 register 7
        ESIDAC2R7: 0..15 = struct ESIDAC2R7Field(u16);
    }
    /// ESI TSM 0
    rw ESITSM0 @ 0x60: u16 = 0_0 {
        /// Input channel select
        ESITSM0_ESICH: 0..1 = enum ESITSM0_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM0_ESILCEN: 2 = struct ESITSM0_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM0_ESIEX: 3 = struct ESITSM0_ESIEX(bool);
        /// TSM comparator on
        ESITSM0_ESICA: 4 = struct ESITSM0_ESICA(bool);
        /// High-frequency clock on
        ESITSM0_ESICLKON: 5 = struct ESITSM0_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM0_ESIRSON: 6 = struct ESITSM0_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM0_ESITESTS1: 7 = struct ESITSM0_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM0_ESIDAC: 8 = struct ESITSM0_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM0_ESISTOP: 9 = struct ESITSM0_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM0_ESICLK: 10 = struct ESITSM0_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM0_ESIREPEAT: 11..15 = enum ESITSM0_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 1
    rw ESITSM1 @ 0x62: u16 = 0_0 {
        /// Input channel select
        ESITSM1_ESICH: 0..1 = enum ESITSM1_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM1_ESILCEN: 2 = struct ESITSM1_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM1_ESIEX: 3 = struct ESITSM1_ESIEX(bool);
        /// TSM comparator on
        ESITSM1_ESICA: 4 = struct ESITSM1_ESICA(bool);
        /// High-frequency clock on
        ESITSM1_ESICLKON: 5 = struct ESITSM1_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM1_ESIRSON: 6 = struct ESITSM1_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM1_ESITESTS1: 7 = struct ESITSM1_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM1_ESIDAC: 8 = struct ESITSM1_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM1_ESISTOP: 9 = struct ESITSM1_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM1_ESICLK: 10 = struct ESITSM1_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM1_ESIREPEAT: 11..15 = enum ESITSM1_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 2
    rw ESITSM2 @ 0x64: u16 = 0_0 {
        /// Input channel select
        ESITSM2_ESICH: 0..1 = enum ESITSM2_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM2_ESILCEN: 2 = struct ESITSM2_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM2_ESIEX: 3 = struct ESITSM2_ESIEX(bool);
        /// TSM comparator on
        ESITSM2_ESICA: 4 = struct ESITSM2_ESICA(bool);
        /// High-frequency clock on
        ESITSM2_ESICLKON: 5 = struct ESITSM2_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM2_ESIRSON: 6 = struct ESITSM2_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM2_ESITESTS1: 7 = struct ESITSM2_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM2_ESIDAC: 8 = struct ESITSM2_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM2_ESISTOP: 9 = struct ESITSM2_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM2_ESICLK: 10 = struct ESITSM2_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM2_ESIREPEAT: 11..15 = enum ESITSM2_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 3
    rw ESITSM3 @ 0x66: u16 = 0_0 {
        /// Input channel select
        ESITSM3_ESICH: 0..1 = enum ESITSM3_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM3_ESILCEN: 2 = struct ESITSM3_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM3_ESIEX: 3 = struct ESITSM3_ESIEX(bool);
        /// TSM comparator on
        ESITSM3_ESICA: 4 = struct ESITSM3_ESICA(bool);
        /// High-frequency clock on
        ESITSM3_ESICLKON: 5 = struct ESITSM3_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM3_ESIRSON: 6 = struct ESITSM3_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM3_ESITESTS1: 7 = struct ESITSM3_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM3_ESIDAC: 8 = struct ESITSM3_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM3_ESISTOP: 9 = struct ESITSM3_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM3_ESICLK: 10 = struct ESITSM3_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM3_ESIREPEAT: 11..15 = enum ESITSM3_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 4
    rw ESITSM4 @ 0x68: u16 = 0_0 {
        /// Input channel select
        ESITSM4_ESICH: 0..1 = enum ESITSM4_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM4_ESILCEN: 2 = struct ESITSM4_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM4_ESIEX: 3 = struct ESITSM4_ESIEX(bool);
        /// TSM comparator on
        ESITSM4_ESICA: 4 = struct ESITSM4_ESICA(bool);
        /// High-frequency clock on
        ESITSM4_ESICLKON: 5 = struct ESITSM4_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM4_ESIRSON: 6 = struct ESITSM4_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM4_ESITESTS1: 7 = struct ESITSM4_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM4_ESIDAC: 8 = struct ESITSM4_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM4_ESISTOP: 9 = struct ESITSM4_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM4_ESICLK: 10 = struct ESITSM4_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM4_ESIREPEAT: 11..15 = enum ESITSM4_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 5
    rw ESITSM5 @ 0x6a: u16 = 0_0 {
        /// Input channel select
        ESITSM5_ESICH: 0..1 = enum ESITSM5_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM5_ESILCEN: 2 = struct ESITSM5_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM5_ESIEX: 3 = struct ESITSM5_ESIEX(bool);
        /// TSM comparator on
        ESITSM5_ESICA: 4 = struct ESITSM5_ESICA(bool);
        /// High-frequency clock on
        ESITSM5_ESICLKON: 5 = struct ESITSM5_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM5_ESIRSON: 6 = struct ESITSM5_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM5_ESITESTS1: 7 = struct ESITSM5_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM5_ESIDAC: 8 = struct ESITSM5_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM5_ESISTOP: 9 = struct ESITSM5_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM5_ESICLK: 10 = struct ESITSM5_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM5_ESIREPEAT: 11..15 = enum ESITSM5_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 6
    rw ESITSM6 @ 0x6c: u16 = 0_0 {
        /// Input channel select
        ESITSM6_ESICH: 0..1 = enum ESITSM6_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM6_ESILCEN: 2 = struct ESITSM6_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM6_ESIEX: 3 = struct ESITSM6_ESIEX(bool);
        /// TSM comparator on
        ESITSM6_ESICA: 4 = struct ESITSM6_ESICA(bool);
        /// High-frequency clock on
        ESITSM6_ESICLKON: 5 = struct ESITSM6_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM6_ESIRSON: 6 = struct ESITSM6_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM6_ESITESTS1: 7 = struct ESITSM6_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM6_ESIDAC: 8 = struct ESITSM6_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM6_ESISTOP: 9 = struct ESITSM6_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM6_ESICLK: 10 = struct ESITSM6_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM6_ESIREPEAT: 11..15 = enum ESITSM6_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 7
    rw ESITSM7 @ 0x6e: u16 = 0_0 {
        /// Input channel select
        ESITSM7_ESICH: 0..1 = enum ESITSM7_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM7_ESILCEN: 2 = struct ESITSM7_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM7_ESIEX: 3 = struct ESITSM7_ESIEX(bool);
        /// TSM comparator on
        ESITSM7_ESICA: 4 = struct ESITSM7_ESICA(bool);
        /// High-frequency clock on
        ESITSM7_ESICLKON: 5 = struct ESITSM7_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM7_ESIRSON: 6 = struct ESITSM7_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM7_ESITESTS1: 7 = struct ESITSM7_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM7_ESIDAC: 8 = struct ESITSM7_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM7_ESISTOP: 9 = struct ESITSM7_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM7_ESICLK: 10 = struct ESITSM7_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM7_ESIREPEAT: 11..15 = enum ESITSM7_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 8
    rw ESITSM8 @ 0x70: u16 = 0_0 {
        /// Input channel select
        ESITSM8_ESICH: 0..1 = enum ESITSM8_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM8_ESILCEN: 2 = struct ESITSM8_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM8_ESIEX: 3 = struct ESITSM8_ESIEX(bool);
        /// TSM comparator on
        ESITSM8_ESICA: 4 = struct ESITSM8_ESICA(bool);
        /// High-frequency clock on
        ESITSM8_ESICLKON: 5 = struct ESITSM8_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM8_ESIRSON: 6 = struct ESITSM8_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM8_ESITESTS1: 7 = struct ESITSM8_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM8_ESIDAC: 8 = struct ESITSM8_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM8_ESISTOP: 9 = struct ESITSM8_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM8_ESICLK: 10 = struct ESITSM8_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM8_ESIREPEAT: 11..15 = enum ESITSM8_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 9
    rw ESITSM9 @ 0x72: u16 = 0_0 {
        /// Input channel select
        ESITSM9_ESICH: 0..1 = enum ESITSM9_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
        }
        /// LC enable
        ESITSM9_ESILCEN: 2 = struct ESITSM9_ESILCEN(bool);
        /// Excitation and sample-and-hold
        ESITSM9_ESIEX: 3 = struct ESITSM9_ESIEX(bool);
        /// TSM comparator on
        ESITSM9_ESICA: 4 = struct ESITSM9_ESICA(bool);
        /// High-frequency clock on
        ESITSM9_ESICLKON: 5 = struct ESITSM9_ESICLKON(bool);
        /// Internal output latches enabled
        ESITSM9_ESIRSON: 6 = struct ESITSM9_ESIRSON(bool);
        /// TSM test cycle control
        ESITSM9_ESITESTS1: 7 = struct ESITSM9_ESITESTS1(bool);
        /// TSM DAC on
        ESITSM9_ESIDAC: 8 = struct ESITSM9_ESIDAC(bool);
        /// This bit indicates the end of the TSM sequence
        ESITSM9_ESISTOP: 9 = struct ESITSM9_ESISTOP(bool);
        /// This bit selects the clock source for the TSM
        ESITSM9_ESICLK: 10 = struct ESITSM9_ESICLK(bool);
        /// These bits together with the ESICLK bit configure the duration of this state
        ESITSM9_ESIREPEAT: 11..15 = enum ESITSM9_ESIREPEAT {
            /// These bits configure the duration of this state
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 10
    rw ESITSM10 @ 0x74: u16 = 0_0 {
        /// Input channel select
        ESITSM10_ESICH: 0..1 = enum ESITSM10_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 11
    rw ESITSM11 @ 0x76: u16 = 0_0 {
        /// Input channel select
        ESITSM11_ESICH: 0..1 = enum ESITSM11_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 12
    rw ESITSM12 @ 0x78: u16 = 0_0 {
        /// Input channel select
        ESITSM12_ESICH: 0..1 = enum ESITSM12_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 13
    rw ESITSM13 @ 0x7a: u16 = 0_0 {
        /// Input channel select
        ESITSM13_ESICH: 0..1 = enum ESITSM13_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 14
    rw ESITSM14 @ 0x7c: u16 = 0_0 {
        /// Input channel select
        ESITSM14_ESICH: 0..1 = enum ESITSM14_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 15
    rw ESITSM15 @ 0x7e: u16 = 0_0 {
        /// Input channel select
        ESITSM15_ESICH: 0..1 = enum ESITSM15_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 16
    rw ESITSM16 @ 0x80: u16 = 0_0 {
        /// Input channel select
        ESITSM16_ESICH: 0..1 = enum ESITSM16_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 17
    rw ESITSM17 @ 0x82: u16 = 0_0 {
        /// Input channel select
        ESITSM17_ESICH: 0..1 = enum ESITSM17_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 18
    rw ESITSM18 @ 0x84: u16 = 0_0 {
        /// Input channel select
        ESITSM18_ESICH: 0..1 = enum ESITSM18_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 19
    rw ESITSM19 @ 0x86: u16 = 0_0 {
        /// Input channel select
        ESITSM19_ESICH: 0..1 = enum ESITSM19_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 20
    rw ESITSM20 @ 0x88: u16 = 0_0 {
        /// Input channel select
        ESITSM20_ESICH: 0..1 = enum ESITSM20_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 21
    rw ESITSM21 @ 0x8a: u16 = 0_0 {
        /// Input channel select
        ESITSM21_ESICH: 0..1 = enum ESITSM21_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 22
    rw ESITSM22 @ 0x8c: u16 = 0_0 {
        /// Input channel select
        ESITSM22_ESICH: 0..1 = enum ESITSM22_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 23
    rw ESITSM23 @ 0x8e: u16 = 0_0 {
        /// Input channel select
        ESITSM23_ESICH: 0..1 = enum ESITSM23_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 24
    rw ESITSM24 @ 0x90: u16 = 0_0 {
        /// Input channel select
        ESITSM24_ESICH: 0..1 = enum ESITSM24_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 25
    rw ESITSM25 @ 0x92: u16 = 0_0 {
        /// Input channel select
        ESITSM25_ESICH: 0..1 = enum ESITSM25_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 26
    rw ESITSM26 @ 0x94: u16 = 0_0 {
        /// Input channel select
        ESITSM26_ESICH: 0..1 = enum ESITSM26_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 27
    rw ESITSM27 @ 0x96: u16 = 0_0 {
        /// Input channel select
        ESITSM27_ESICH: 0..1 = enum ESITSM27_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 28
    rw ESITSM28 @ 0x98: u16 = 0_0 {
        /// Input channel select
        ESITSM28_ESICH: 0..1 = enum ESITSM28_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 29
    rw ESITSM29 @ 0x9a: u16 = 0_0 {
        /// Input channel select
        ESITSM29_ESICH: 0..1 = enum ESITSM29_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 30
    rw ESITSM30 @ 0x9c: u16 = 0_0 {
        /// Input channel select
        ESITSM30_ESICH: 0..1 = enum ESITSM30_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
    /// ESI TSM 31
    rw ESITSM31 @ 0x9e: u16 = 0_0 {
        /// Input channel select
        ESITSM31_ESICH: 0..1 = enum ESITSM31_ESICH {
            /// Input channel select: ESICH0
            ESICH_0 = 0b00,
            /// Input channel select: ESICH1
            ESICH_1 = 0b01,
            /// Input channel select: ESICH2
            ESICH_2 = 0b10,
            /// Input channel select: ESICH3
            ESICH_3 = 0b11,
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
            ESIREPEAT_0 = 0b00000,
            /// ESIREPEATx selects the number of clock cycles for this state. The number of clock cycles = ESIREPEATx + 1
            ESIREPEAT_1 = 0b00001,
            /// ESIREPEAT_2
            ESIREPEAT_2 = 0b00010,
            /// ESIREPEAT_3
            ESIREPEAT_3 = 0b00011,
            /// ESIREPEAT_4
            ESIREPEAT_4 = 0b00100,
            /// ESIREPEAT_5
            ESIREPEAT_5 = 0b00101,
            /// ESIREPEAT_6
            ESIREPEAT_6 = 0b00110,
            /// ESIREPEAT_7
            ESIREPEAT_7 = 0b00111,
            /// ESIREPEAT_8
            ESIREPEAT_8 = 0b01000,
            /// ESIREPEAT_9
            ESIREPEAT_9 = 0b01001,
            /// ESIREPEAT_10
            ESIREPEAT_10 = 0b01010,
            /// ESIREPEAT_11
            ESIREPEAT_11 = 0b01011,
            /// ESIREPEAT_12
            ESIREPEAT_12 = 0b01100,
            /// ESIREPEAT_13
            ESIREPEAT_13 = 0b01101,
            /// ESIREPEAT_14
            ESIREPEAT_14 = 0b01110,
            /// ESIREPEAT_15
            ESIREPEAT_15 = 0b01111,
            /// ESIREPEAT_16
            ESIREPEAT_16 = 0b10000,
            /// ESIREPEAT_17
            ESIREPEAT_17 = 0b10001,
            /// ESIREPEAT_18
            ESIREPEAT_18 = 0b10010,
            /// ESIREPEAT_19
            ESIREPEAT_19 = 0b10011,
            /// ESIREPEAT_20
            ESIREPEAT_20 = 0b10100,
            /// ESIREPEAT_21
            ESIREPEAT_21 = 0b10101,
            /// ESIREPEAT_22
            ESIREPEAT_22 = 0b10110,
            /// ESIREPEAT_23
            ESIREPEAT_23 = 0b10111,
            /// ESIREPEAT_24
            ESIREPEAT_24 = 0b11000,
            /// ESIREPEAT_25
            ESIREPEAT_25 = 0b11001,
            /// ESIREPEAT_26
            ESIREPEAT_26 = 0b11010,
            /// ESIREPEAT_27
            ESIREPEAT_27 = 0b11011,
            /// ESIREPEAT_28
            ESIREPEAT_28 = 0b11100,
            /// ESIREPEAT_29
            ESIREPEAT_29 = 0b11101,
            /// ESIREPEAT_30
            ESIREPEAT_30 = 0b11110,
            /// ESIREPEAT_31
            ESIREPEAT_31 = 0b11111,
        }
    }
}
