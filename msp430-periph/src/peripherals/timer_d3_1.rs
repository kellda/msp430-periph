//! Timer D3

utils::periph! {
    /// Timer D3
    TimerD3;
    /// Timer D3 Control 0
    rw TDCTL0 @ 0x00: u16 = 0_0 {
        /// Timer D3 interrupt flag
        TDIFG: 0 = struct TDIFG(bool);
        /// Timer D3 interrupt enable
        TDIE: 1 = struct TDIE(bool);
        /// Timer D3 counter clear
        TDCLR: 2 = struct TDCLR(bool);
        /// Timer D3 mode control 1
        MC: 4..5 = enum MC {
            /// Timer A mode control: 0 - Stop
            MC_0 = 0b00,
            /// Timer A mode control: 1 - Up to CCR0
            MC_1 = 0b01,
            /// Timer A mode control: 2 - Continuous up
            MC_2 = 0b10,
            /// Timer A mode control: 3 - Up/Down
            MC_3 = 0b11,
        }
        /// Timer D3 clock input divider 1
        ID: 6..7 = enum ID {
            /// Timer A input divider: 0 - /1
            ID_0 = 0b00,
            /// Timer A input divider: 1 - /2
            ID_1 = 0b01,
            /// Timer A input divider: 2 - /4
            ID_2 = 0b10,
            /// Timer A input divider: 3 - /8
            ID_3 = 0b11,
        }
        /// Clock source 1
        TDSSEL: 8..9 = enum TDSSEL {
            /// Clock Source: TDCLK
            TDSSEL_0 = 0b00,
            /// Clock Source: ACLK
            TDSSEL_1 = 0b01,
            /// Clock Source: SMCLK
            TDSSEL_2 = 0b10,
            /// Clock Source: INCLK
            TDSSEL_3 = 0b11,
        }
        /// Counter lenght 1
        CNTL: 11..12 = enum CNTL {
            /// Counter lenght: 16 bit
            CNTL_0 = 0b00,
            /// Counter lenght: 12 bit
            CNTL_1 = 0b01,
            /// Counter lenght: 10 bit
            CNTL_2 = 0b10,
            /// Counter lenght:  8 bit
            CNTL_3 = 0b11,
        }
        /// Timer D3 Compare latch load group 1
        TDCLGRP: 13..14 = enum TDCLGRP {
            /// Timer D3 Group: 0 - individually
            TDCLGRP_0 = 0b00,
            /// Timer D3 Group: 1 - 3 groups (1-2
            TDCLGRP_1 = 0b01,
            /// Timer D3 Group: 2 - 2 groups (1-3
            TDCLGRP_2 = 0b10,
            /// Timer D3 Group: 3 - 1 group (all)
            TDCLGRP_3 = 0b11,
        }
    }
    /// Timer D3 Control 1
    rw TDCTL1 @ 0x02: u16 = 0_0 {
        /// Timer D3 Clocking Mode Bit: 0
        TDCLKM: 0..1 = enum TDCLKM {
            /// Timer D3 Clocking Mode: External
            TDCLKM_0 = 0b00,
            /// Timer D3 Clocking Mode: High-Res. local clock
            TDCLKM_1 = 0b01,
            /// Timer D3 Clocking Mode: Aux Clock
            TDCLKM_2 = 0b10,
        }
        /// Timer D3 TDCCR Combination in TD2
        TD2CMB: 4 = struct TD2CMB(bool);
        /// Timer D3 TDCCR Combination in TD4
        TD4CMB: 5 = struct TD4CMB(bool);
        /// Timer D3 TDCCR Combination in TD6
        TD6CMB: 6 = struct TD6CMB(bool);
        /// Timer D3 Input divider expansion Bit: 0
        TDIDEX: 8..10 = enum TDIDEX {
            /// Timer D3 Input divider expansion : /1
            TDIDEX_0 = 0b000,
            /// Timer D3 Input divider expansion : /2
            TDIDEX_1 = 0b001,
            /// Timer D3 Input divider expansion : /3
            TDIDEX_2 = 0b010,
            /// Timer D3 Input divider expansion : /4
            TDIDEX_3 = 0b011,
            /// Timer D3 Input divider expansion : /5
            TDIDEX_4 = 0b100,
            /// Timer D3 Input divider expansion : /6
            TDIDEX_5 = 0b101,
            /// Timer D3 Input divider expansion : /7
            TDIDEX_6 = 0b110,
            /// Timer D3 Input divider expansion : /8
            TDIDEX_7 = 0b111,
        }
    }
    /// Timer D3 Control 2
    rw TDCTL2 @ 0x04: u16 = 0_0 {
        /// Timer D3 Capture Mode of Channel 0
        TDCAPM0: 0 = struct TDCAPM0(bool);
        /// Timer D3 Capture Mode of Channel 1
        TDCAPM1: 1 = struct TDCAPM1(bool);
        /// Timer D3 Capture Mode of Channel 2
        TDCAPM2: 2 = struct TDCAPM2(bool);
        /// Timer D3 Capture Mode of Channel 3
        TDCAPM3: 3 = struct TDCAPM3(bool);
        /// Timer D3 Capture Mode of Channel 4
        TDCAPM4: 4 = struct TDCAPM4(bool);
        /// Timer D3 Capture Mode of Channel 5
        TDCAPM5: 5 = struct TDCAPM5(bool);
        /// Timer D3 Capture Mode of Channel 6
        TDCAPM6: 6 = struct TDCAPM6(bool);
    }
    /// Timer D3 Counter
    rw TDR @ 0x06: u16 = 0_0 {
        /// Timer D3 Counter
        TDR: 0..15 = struct TDRField(u16);
    }
    /// Timer D3 Capture/Compare Control 0
    rw TDCCTL0 @ 0x08: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TDCCTL0_CCIFG: 0 = struct TDCCTL0_CCIFG(bool);
        /// Capture/compare overflow flag
        TDCCTL0_COV: 1 = struct TDCCTL0_COV(bool);
        /// PWM Output signal if output mode 0
        TDCCTL0_OUT: 2 = struct TDCCTL0_OUT(bool);
        /// Capture input signal (read)
        TDCCTL0_CCI: 3 = struct TDCCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TDCCTL0_CCIE: 4 = struct TDCCTL0_CCIE(bool);
        /// Output mode 2
        TDCCTL0_OUTMOD: 5..7 = enum TDCCTL0_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TDCCTL0_CAP: 8 = struct TDCCTL0_CAP(bool);
        /// Compare latch load source 1
        TDCCTL0_CLLD: 9..10 = enum TDCCTL0_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TDR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TDR counts to TDCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TDCCTL0_SCS: 11 = struct TDCCTL0_SCS(bool);
        /// Capture input select 1
        TDCCTL0_CCIS: 12..13 = enum TDCCTL0_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TDCCTL0_CM: 14..15 = enum TDCCTL0_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer D3 Capture/Compare 0
    rw TDCCR0 @ 0x0a: u16 = 0_0 {
        /// Timer D3 Capture/Compare 0
        TDCCR0: 0..15 = struct TDCCR0Field(u16);
    }
    /// Timer D3 Capture/Compare Latch 0
    rw TDCL0 @ 0x0c: u16 = 0_0 {
        /// Timer D3 Capture/Compare Latch 0
        TDCL0: 0..15 = struct TDCL0Field(u16);
    }
    /// Timer D3 Capture/Compare Control 1
    rw TDCCTL1 @ 0x0e: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TDCCTL1_CCIFG: 0 = struct TDCCTL1_CCIFG(bool);
        /// Capture/compare overflow flag
        TDCCTL1_COV: 1 = struct TDCCTL1_COV(bool);
        /// PWM Output signal if output mode 0
        TDCCTL1_OUT: 2 = struct TDCCTL1_OUT(bool);
        /// Capture input signal (read)
        TDCCTL1_CCI: 3 = struct TDCCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TDCCTL1_CCIE: 4 = struct TDCCTL1_CCIE(bool);
        /// Output mode 2
        TDCCTL1_OUTMOD: 5..7 = enum TDCCTL1_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TDCCTL1_CAP: 8 = struct TDCCTL1_CAP(bool);
        /// Compare latch load source 1
        TDCCTL1_CLLD: 9..10 = enum TDCCTL1_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TDR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TDR counts to TDCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TDCCTL1_SCS: 11 = struct TDCCTL1_SCS(bool);
        /// Capture input select 1
        TDCCTL1_CCIS: 12..13 = enum TDCCTL1_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TDCCTL1_CM: 14..15 = enum TDCCTL1_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer D3 Capture/Compare 1
    rw TDCCR1 @ 0x10: u16 = 0_0 {
        /// Timer D3 Capture/Compare 1
        TDCCR1: 0..15 = struct TDCCR1Field(u16);
    }
    /// Timer D3 Capture/Compare Latch 1
    rw TDCL1 @ 0x12: u16 = 0_0 {
        /// Timer D3 Capture/Compare Latch 1
        TDCL1: 0..15 = struct TDCL1Field(u16);
    }
    /// Timer D3 Capture/Compare Control 2
    rw TDCCTL2 @ 0x14: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TDCCTL2_CCIFG: 0 = struct TDCCTL2_CCIFG(bool);
        /// Capture/compare overflow flag
        TDCCTL2_COV: 1 = struct TDCCTL2_COV(bool);
        /// PWM Output signal if output mode 0
        TDCCTL2_OUT: 2 = struct TDCCTL2_OUT(bool);
        /// Capture input signal (read)
        TDCCTL2_CCI: 3 = struct TDCCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TDCCTL2_CCIE: 4 = struct TDCCTL2_CCIE(bool);
        /// Output mode 2
        TDCCTL2_OUTMOD: 5..7 = enum TDCCTL2_OUTMOD {
            /// PWM output mode: 0 - output only
            OUTMOD_0 = 0b000,
            /// PWM output mode: 1 - set
            OUTMOD_1 = 0b001,
            /// PWM output mode: 2 - PWM toggle/reset
            OUTMOD_2 = 0b010,
            /// PWM output mode: 3 - PWM set/reset
            OUTMOD_3 = 0b011,
            /// PWM output mode: 4 - toggle
            OUTMOD_4 = 0b100,
            /// PWM output mode: 5 - Reset
            OUTMOD_5 = 0b101,
            /// PWM output mode: 6 - PWM toggle/set
            OUTMOD_6 = 0b110,
            /// PWM output mode: 7 - PWM reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode: 1 /Compare mode : 0
        TDCCTL2_CAP: 8 = struct TDCCTL2_CAP(bool);
        /// Compare latch load source 1
        TDCCTL2_CLLD: 9..10 = enum TDCCTL2_CLLD {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TDR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TDR counts to TDCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        TDCCTL2_SCS: 11 = struct TDCCTL2_SCS(bool);
        /// Capture input select 1
        TDCCTL2_CCIS: 12..13 = enum TDCCTL2_CCIS {
            /// Capture input select: 0 - CCIxA
            CCIS_0 = 0b00,
            /// Capture input select: 1 - CCIxB
            CCIS_1 = 0b01,
            /// Capture input select: 2 - GND
            CCIS_2 = 0b10,
            /// Capture input select: 3 - Vcc
            CCIS_3 = 0b11,
        }
        /// Capture mode 1
        TDCCTL2_CM: 14..15 = enum TDCCTL2_CM {
            /// Capture mode: 0 - disabled
            CM_0 = 0b00,
            /// Capture mode: 1 - pos. edge
            CM_1 = 0b01,
            /// Capture mode: 1 - neg. edge
            CM_2 = 0b10,
            /// Capture mode: 1 - both edges
            CM_3 = 0b11,
        }
    }
    /// Timer D3 Capture/Compare 2
    rw TDCCR2 @ 0x16: u16 = 0_0 {
        /// Timer D3 Capture/Compare 2
        TDCCR2: 0..15 = struct TDCCR2Field(u16);
    }
    /// Timer D3 Capture/Compare Latch 2
    rw TDCL2 @ 0x18: u16 = 0_0 {
        /// Timer D3 Capture/Compare Latch 2
        TDCL2: 0..15 = struct TDCL2Field(u16);
    }
    /// Timer D3 High-resolution Control Register 0
    rw TDHCTL0 @ 0x38: u16 = 0_0 {
        /// Timer D3 High-Resolution Enable
        TDHEN: 0 = struct TDHEN(bool);
        /// Timer D3 High-Resolution Regulated Mode
        TDHREGEN: 1 = struct TDHREGEN(bool);
        /// Timer D3 High-Resolution clock error accum. enable
        TDHEAEN: 2 = struct TDHEAEN(bool);
        /// Timer D3 High-Resolution Generator forced on
        TDHRON: 3 = struct TDHRON(bool);
        /// Timer D3 High-Resoltuion Clock Mult. Bit: 0
        TDHM: 4..5 = enum TDHM {
            /// Timer D3 High-Resoltuion Clock Mult.: 8x TimerD clock
            TDHM_0 = 0b00,
            /// Timer D3 High-Resoltuion Clock Mult.: 16x TimerD clock
            TDHM_1 = 0b01,
        }
        /// Timer D3 High-Resolution clock divider Bit: 0
        TDHD: 6..7 = enum TDHD {
            /// Timer D3 High-Resolution clock divider: /1
            TDHD_0 = 0b00,
            /// Timer D3 High-Resolution clock divider: /2
            TDHD_1 = 0b01,
            /// Timer D3 High-Resolution clock divider: /4
            TDHD_2 = 0b10,
            /// Timer D3 High-Resolution clock divider: /8
            TDHD_3 = 0b11,
        }
        /// Timer0_D7 High-resolution generator fast wakeup enable
        TDHFW: 8 = struct TDHFW(bool);
    }
    /// Timer D3 High-resolution Control Register 1
    rw TDHCTL1 @ 0x3a: u16 = 0_0 {
        /// Timer D3 High-Resolution Clock Trim Bit: 0
        TDHCLKTRIM0: 1 = struct TDHCLKTRIM0(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 1
        TDHCLKTRIM1: 2 = struct TDHCLKTRIM1(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 2
        TDHCLKTRIM2: 3 = struct TDHCLKTRIM2(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 3
        TDHCLKTRIM3: 4 = struct TDHCLKTRIM3(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 4
        TDHCLKTRIM4: 5 = struct TDHCLKTRIM4(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 5
        TDHCLKTRIM5: 6 = struct TDHCLKTRIM5(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 6
        TDHCLKTRIM6: 7 = struct TDHCLKTRIM6(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 0
        TDHCLKSR0: 8 = struct TDHCLKSR0(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 1
        TDHCLKSR1: 9 = struct TDHCLKSR1(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 2
        TDHCLKSR2: 10 = struct TDHCLKSR2(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 3
        TDHCLKSR3: 11 = struct TDHCLKSR3(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 4
        TDHCLKSR4: 12 = struct TDHCLKSR4(bool);
        /// Timer D3 High-Resolution Clock Range Bit: 0
        TDHCLKR0: 13 = struct TDHCLKR0(bool);
        /// Timer D3 High-Resolution Clock Range Bit: 1
        TDHCLKR1: 14 = struct TDHCLKR1(bool);
        /// Timer D3 High-Resolution Coarse Clock Range
        TDHCLKCR: 15 = struct TDHCLKCR(bool);
    }
    /// Timer D3 High-resolution Interrupt Register
    rw TDHINT @ 0x3c: u16 = 0_0 {
        /// Timer D3 High-Res. fail low Interrupt Flag
        TDHFLIFG: 0 = struct TDHFLIFG(bool);
        /// Timer D3 High-Res. fail high Interrupt Flag
        TDHFHIFG: 1 = struct TDHFHIFG(bool);
        /// Timer D3 High-Res. frequency lock Interrupt Flag
        TDHLKIFG: 2 = struct TDHLKIFG(bool);
        /// Timer D3 High-Res. frequency unlock Interrupt Flag
        TDHUNLKIFG: 3 = struct TDHUNLKIFG(bool);
        /// Timer D3 High-Res. fail low Interrupt Enable
        TDHFLIE: 8 = struct TDHFLIE(bool);
        /// Timer D3 High-Res. fail high Interrupt Enable
        TDHFHIE: 9 = struct TDHFHIE(bool);
        /// Timer D3 High-Res. frequency lock Interrupt Enable
        TDHLKIE: 10 = struct TDHLKIE(bool);
        /// Timer D3 High-Res. frequency unlock Interrupt Enable
        TDHUNLKIE: 11 = struct TDHUNLKIE(bool);
    }
    /// Timer D3 Interrupt Vector Word
    rw TDIV @ 0x3e: u16 = 0_0 {
        /// Timer D3 Interrupt Vector Word
        TDIV: 0..15 = struct TDIVField(u16);
    }
}
