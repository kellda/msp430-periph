//! Timer1_D3

utils::periph! {
    /// Timer1_D3
    Timer1_D3;
    /// Timer1_D3 Control 0
    rw TD1CTL0 @ 0x00: u16 = 0_0 {
        /// Timer0_D3 interrupt flag
        TDIFG: 0 = struct TDIFG(bool);
        /// Timer0_D3 interrupt enable
        TDIE: 1 = struct TDIE(bool);
        /// Timer0_D3 counter clear
        TDCLR: 2 = struct TDCLR(bool);
        /// Timer0_D3 mode control 1
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
        /// Timer0_D3 clock input divider 1
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
        /// Timer0_D3 Compare latch load group 1
        TDCLGRP: 13..14 = enum TDCLGRP {
            /// Timer0_D3 Group: 0 - individually
            TDCLGRP_0 = 0b00,
            /// Timer0_D3 Group: 1 - 3 groups (1-2
            TDCLGRP_1 = 0b01,
            /// Timer0_D3 Group: 2 - 2 groups (1-3
            TDCLGRP_2 = 0b10,
            /// Timer0_D3 Group: 3 - 1 group (all)
            TDCLGRP_3 = 0b11,
        }
    }
    /// Timer1_D3 Control 1
    rw TD1CTL1 @ 0x02: u16 = 0_0 {
        /// Timer0_D3 Clocking Mode Bit: 0
        TDCLKM: 0..1 = enum TDCLKM {
            /// Timer0_D3 Clocking Mode: External
            TDCLKM_0 = 0b00,
            /// Timer0_D3 Clocking Mode: High-Res. local clock
            TDCLKM_1 = 0b01,
            /// Timer0_D3 Clocking Mode: Aux Clock
            TDCLKM_2 = 0b10,
        }
        /// Timer0_D3 TD0CCR Combination in TD2
        TD2CMB: 4 = struct TD2CMB(bool);
        /// Timer0_D3 TD0CCR Combination in TD4
        TD4CMB: 5 = struct TD4CMB(bool);
        /// Timer0_D3 TD0CCR Combination in TD6
        TD6CMB: 6 = struct TD6CMB(bool);
        /// Timer0_D3 Input divider expansion Bit: 0
        TDIDEX: 8..10 = enum TDIDEX {
            /// Timer0_D3 Input divider expansion : /1
            TDIDEX_0 = 0b000,
            /// Timer0_D3 Input divider expansion : /2
            TDIDEX_1 = 0b001,
            /// Timer0_D3 Input divider expansion : /3
            TDIDEX_2 = 0b010,
            /// Timer0_D3 Input divider expansion : /4
            TDIDEX_3 = 0b011,
            /// Timer0_D3 Input divider expansion : /5
            TDIDEX_4 = 0b100,
            /// Timer0_D3 Input divider expansion : /6
            TDIDEX_5 = 0b101,
            /// Timer0_D3 Input divider expansion : /7
            TDIDEX_6 = 0b110,
            /// Timer0_D3 Input divider expansion : /8
            TDIDEX_7 = 0b111,
        }
    }
    /// Timer1_D3 Control 2
    rw TD1CTL2 @ 0x04: u16 = 0_0 {
        /// Timer0_D3 Capture Mode of Channel 0
        TDCAPM0: 0 = struct TDCAPM0(bool);
        /// Timer0_D3 Capture Mode of Channel 1
        TDCAPM1: 1 = struct TDCAPM1(bool);
        /// Timer0_D3 Capture Mode of Channel 2
        TDCAPM2: 2 = struct TDCAPM2(bool);
        /// Timer0_D3 Capture Mode of Channel 3
        TDCAPM3: 3 = struct TDCAPM3(bool);
        /// Timer0_D3 Capture Mode of Channel 4
        TDCAPM4: 4 = struct TDCAPM4(bool);
        /// Timer0_D3 Capture Mode of Channel 5
        TDCAPM5: 5 = struct TDCAPM5(bool);
        /// Timer0_D3 Capture Mode of Channel 6
        TDCAPM6: 6 = struct TDCAPM6(bool);
    }
    /// Timer1_D3 Counter
    rw TD1R @ 0x06: u16 = 0_0 {
        /// Timer1_D3 Counter
        TD1R: 0..15 = struct TD1RField(u16);
    }
    /// Timer1_D3 Capture/Compare Control 0
    rw TD1CCTL0 @ 0x08: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TD1CCTL0_CCIFG: 0 = struct TD1CCTL0_CCIFG(bool);
        /// Capture/compare overflow flag
        TD1CCTL0_COV: 1 = struct TD1CCTL0_COV(bool);
        /// PWM Output signal if output mode 0
        TD1CCTL0_OUT: 2 = struct TD1CCTL0_OUT(bool);
        /// Capture input signal (read)
        TD1CCTL0_CCI: 3 = struct TD1CCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TD1CCTL0_CCIE: 4 = struct TD1CCTL0_CCIE(bool);
        /// Output mode 2
        TD1CCTL0_OUTMOD: 5..7 = enum TD1CCTL0_OUTMOD {
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
        TD1CCTL0_CAP: 8 = struct TD1CCTL0_CAP(bool);
        /// Compare latch load source 1
        TD1CCTL0_CLLD: 9..10 = enum TD1CCTL0_CLLD {
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
        TD1CCTL0_SCS: 11 = struct TD1CCTL0_SCS(bool);
        /// Capture input select 1
        TD1CCTL0_CCIS: 12..13 = enum TD1CCTL0_CCIS {
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
        TD1CCTL0_CM: 14..15 = enum TD1CCTL0_CM {
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
    /// Timer1_D3 Capture/Compare 0
    rw TD1CCR0 @ 0x0a: u16 = 0_0 {
        /// Timer1_D3 Capture/Compare 0
        TD1CCR0: 0..15 = struct TD1CCR0Field(u16);
    }
    /// Timer1_D3 Capture/Compare Latch 0
    rw TD1CL0 @ 0x0c: u16 = 0_0 {
        /// Timer1_D3 Capture/Compare Latch 0
        TD1CL0: 0..15 = struct TD1CL0Field(u16);
    }
    /// Timer1_D3 Capture/Compare Control 1
    rw TD1CCTL1 @ 0x0e: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TD1CCTL1_CCIFG: 0 = struct TD1CCTL1_CCIFG(bool);
        /// Capture/compare overflow flag
        TD1CCTL1_COV: 1 = struct TD1CCTL1_COV(bool);
        /// PWM Output signal if output mode 0
        TD1CCTL1_OUT: 2 = struct TD1CCTL1_OUT(bool);
        /// Capture input signal (read)
        TD1CCTL1_CCI: 3 = struct TD1CCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TD1CCTL1_CCIE: 4 = struct TD1CCTL1_CCIE(bool);
        /// Output mode 2
        TD1CCTL1_OUTMOD: 5..7 = enum TD1CCTL1_OUTMOD {
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
        TD1CCTL1_CAP: 8 = struct TD1CCTL1_CAP(bool);
        /// Compare latch load source 1
        TD1CCTL1_CLLD: 9..10 = enum TD1CCTL1_CLLD {
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
        TD1CCTL1_SCS: 11 = struct TD1CCTL1_SCS(bool);
        /// Capture input select 1
        TD1CCTL1_CCIS: 12..13 = enum TD1CCTL1_CCIS {
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
        TD1CCTL1_CM: 14..15 = enum TD1CCTL1_CM {
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
    /// Timer1_D3 Capture/Compare 1
    rw TD1CCR1 @ 0x10: u16 = 0_0 {
        /// Timer1_D3 Capture/Compare 1
        TD1CCR1: 0..15 = struct TD1CCR1Field(u16);
    }
    /// Timer1_D3 Capture/Compare Latch 1
    rw TD1CL1 @ 0x12: u16 = 0_0 {
        /// Timer1_D3 Capture/Compare Latch 1
        TD1CL1: 0..15 = struct TD1CL1Field(u16);
    }
    /// Timer1_D3 Capture/Compare Control 2
    rw TD1CCTL2 @ 0x14: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TD1CCTL2_CCIFG: 0 = struct TD1CCTL2_CCIFG(bool);
        /// Capture/compare overflow flag
        TD1CCTL2_COV: 1 = struct TD1CCTL2_COV(bool);
        /// PWM Output signal if output mode 0
        TD1CCTL2_OUT: 2 = struct TD1CCTL2_OUT(bool);
        /// Capture input signal (read)
        TD1CCTL2_CCI: 3 = struct TD1CCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TD1CCTL2_CCIE: 4 = struct TD1CCTL2_CCIE(bool);
        /// Output mode 2
        TD1CCTL2_OUTMOD: 5..7 = enum TD1CCTL2_OUTMOD {
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
        TD1CCTL2_CAP: 8 = struct TD1CCTL2_CAP(bool);
        /// Compare latch load source 1
        TD1CCTL2_CLLD: 9..10 = enum TD1CCTL2_CLLD {
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
        TD1CCTL2_SCS: 11 = struct TD1CCTL2_SCS(bool);
        /// Capture input select 1
        TD1CCTL2_CCIS: 12..13 = enum TD1CCTL2_CCIS {
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
        TD1CCTL2_CM: 14..15 = enum TD1CCTL2_CM {
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
    /// Timer1_D3 Capture/Compare 2
    rw TD1CCR2 @ 0x16: u16 = 0_0 {
        /// Timer1_D3 Capture/Compare 2
        TD1CCR2: 0..15 = struct TD1CCR2Field(u16);
    }
    /// Timer1_D3 Capture/Compare Latch 2
    rw TD1CL2 @ 0x18: u16 = 0_0 {
        /// Timer1_D3 Capture/Compare Latch 2
        TD1CL2: 0..15 = struct TD1CL2Field(u16);
    }
    /// Timer1_D3 High-resolution Control Register 0
    rw TD1HCTL0 @ 0x38: u16 = 0_0 {
        /// Timer0_D3 High-Resolution Enable
        TDHEN: 0 = struct TDHEN(bool);
        /// Timer0_D3 High-Resolution Regulated Mode
        TDHREGEN: 1 = struct TDHREGEN(bool);
        /// Timer0_D3 High-Resolution clock error accum. enable
        TDHEAEN: 2 = struct TDHEAEN(bool);
        /// Timer0_D3 High-Resolution Generator forced on
        TDHRON: 3 = struct TDHRON(bool);
        /// Timer0_D3 High-Resoltuion Clock Mult. Bit: 0
        TDHM: 4..5 = enum TDHM {
            /// Timer0_D3 High-Resoltuion Clock Mult.: 8x TimerD clock
            TDHM_0 = 0b00,
            /// Timer0_D3 High-Resoltuion Clock Mult.: 16x TimerD clock
            TDHM_1 = 0b01,
        }
        /// Timer0_D3 High-Resolution clock divider Bit: 0
        TDHD: 6..7 = enum TDHD {
            /// Timer0_D3 High-Resolution clock divider: /1
            TDHD_0 = 0b00,
            /// Timer0_D3 High-Resolution clock divider: /2
            TDHD_1 = 0b01,
            /// Timer0_D3 High-Resolution clock divider: /4
            TDHD_2 = 0b10,
            /// Timer0_D3 High-Resolution clock divider: /8
            TDHD_3 = 0b11,
        }
        /// Timer0_D7 High-resolution generator fast wakeup enable
        TDHFW: 8 = struct TDHFW(bool);
    }
    /// Timer1_D3 High-resolution Control Register 1
    rw TD1HCTL1 @ 0x3a: u16 = 0_0 {
        /// Timer0_D3 High-Resolution Clock Trim Bit: 0
        TDHCLKTRIM0: 1 = struct TDHCLKTRIM0(bool);
        /// Timer0_D3 High-Resolution Clock Trim Bit: 1
        TDHCLKTRIM1: 2 = struct TDHCLKTRIM1(bool);
        /// Timer0_D3 High-Resolution Clock Trim Bit: 2
        TDHCLKTRIM2: 3 = struct TDHCLKTRIM2(bool);
        /// Timer0_D3 High-Resolution Clock Trim Bit: 3
        TDHCLKTRIM3: 4 = struct TDHCLKTRIM3(bool);
        /// Timer0_D3 High-Resolution Clock Trim Bit: 4
        TDHCLKTRIM4: 5 = struct TDHCLKTRIM4(bool);
        /// Timer0_D3 High-Resolution Clock Trim Bit: 5
        TDHCLKTRIM5: 6 = struct TDHCLKTRIM5(bool);
        /// Timer0_D3 High-Resolution Clock Trim Bit: 6
        TDHCLKTRIM6: 7 = struct TDHCLKTRIM6(bool);
        /// Timer0_D3 High-Resolution Clock Sub-Range Bit: 0
        TDHCLKSR0: 8 = struct TDHCLKSR0(bool);
        /// Timer0_D3 High-Resolution Clock Sub-Range Bit: 1
        TDHCLKSR1: 9 = struct TDHCLKSR1(bool);
        /// Timer0_D3 High-Resolution Clock Sub-Range Bit: 2
        TDHCLKSR2: 10 = struct TDHCLKSR2(bool);
        /// Timer0_D3 High-Resolution Clock Sub-Range Bit: 3
        TDHCLKSR3: 11 = struct TDHCLKSR3(bool);
        /// Timer0_D3 High-Resolution Clock Sub-Range Bit: 4
        TDHCLKSR4: 12 = struct TDHCLKSR4(bool);
        /// Timer0_D3 High-Resolution Clock Range Bit: 0
        TDHCLKR0: 13 = struct TDHCLKR0(bool);
        /// Timer0_D3 High-Resolution Clock Range Bit: 1
        TDHCLKR1: 14 = struct TDHCLKR1(bool);
        /// Timer0_D3 High-Resolution Coarse Clock Range
        TDHCLKCR: 15 = struct TDHCLKCR(bool);
    }
    /// Timer1_D3 High-resolution Interrupt Register
    rw TD1HINT @ 0x3c: u16 = 0_0 {
        /// Timer0_D3 High-Res. fail low Interrupt Flag
        TDHFLIFG: 0 = struct TDHFLIFG(bool);
        /// Timer0_D3 High-Res. fail high Interrupt Flag
        TDHFHIFG: 1 = struct TDHFHIFG(bool);
        /// Timer0_D3 High-Res. frequency lock Interrupt Flag
        TDHLKIFG: 2 = struct TDHLKIFG(bool);
        /// Timer0_D3 High-Res. frequency unlock Interrupt Flag
        TDHUNLKIFG: 3 = struct TDHUNLKIFG(bool);
        /// Timer0_D3 High-Res. fail low Interrupt Enable
        TDHFLIE: 8 = struct TDHFLIE(bool);
        /// Timer0_D3 High-Res. fail high Interrupt Enable
        TDHFHIE: 9 = struct TDHFHIE(bool);
        /// Timer0_D3 High-Res. frequency lock Interrupt Enable
        TDHLKIE: 10 = struct TDHLKIE(bool);
        /// Timer0_D3 High-Res. frequency unlock Interrupt Enable
        TDHUNLKIE: 11 = struct TDHUNLKIE(bool);
    }
    /// Timer1_D3 Interrupt Vector Word
    rw TD1IV @ 0x3e: u16 = 0_0 {
        /// Timer1_D3 Interrupt Vector Word
        TD1IV: 0..15 = struct TD1IVField(u16);
    }
}
