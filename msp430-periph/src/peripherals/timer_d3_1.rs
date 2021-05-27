//! Timer D3

utils::periph! {
    /// Timer D3
    TimerD3;
    /// Timer D3 Control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// Timer D3 interrupt flag
        IFG: 0 = struct IFG(bool);
        /// Timer D3 interrupt enable
        IE: 1 = struct IE(bool);
        /// Timer D3 counter clear
        CLR: 2 = struct CLR(bool);
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
        SSEL: 8..9 = enum SSEL {
            /// Clock Source: TDCLK
            SSEL_0 = 0b00,
            /// Clock Source: ACLK
            SSEL_1 = 0b01,
            /// Clock Source: SMCLK
            SSEL_2 = 0b10,
            /// Clock Source: INCLK
            SSEL_3 = 0b11,
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
        CLGRP: 13..14 = enum CLGRP {
            /// Timer D3 Group: 0 - individually
            CLGRP_0 = 0b00,
            /// Timer D3 Group: 1 - 3 groups (1-2
            CLGRP_1 = 0b01,
            /// Timer D3 Group: 2 - 2 groups (1-3
            CLGRP_2 = 0b10,
            /// Timer D3 Group: 3 - 1 group (all)
            CLGRP_3 = 0b11,
        }
    }
    /// Timer D3 Control 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// Timer D3 Clocking Mode Bit: 0
        CLKM: 0..1 = enum CLKM {
            /// Timer D3 Clocking Mode: External
            CLKM_0 = 0b00,
            /// Timer D3 Clocking Mode: High-Res. local clock
            CLKM_1 = 0b01,
            /// Timer D3 Clocking Mode: Aux Clock
            CLKM_2 = 0b10,
        }
        /// Timer D3 TDCCR Combination in TD2
        TD2CMB: 4 = struct TD2CMB(bool);
        /// Timer D3 TDCCR Combination in TD4
        TD4CMB: 5 = struct TD4CMB(bool);
        /// Timer D3 TDCCR Combination in TD6
        TD6CMB: 6 = struct TD6CMB(bool);
        /// Timer D3 Input divider expansion Bit: 0
        IDEX: 8..10 = enum IDEX {
            /// Timer D3 Input divider expansion : /1
            IDEX_0 = 0b000,
            /// Timer D3 Input divider expansion : /2
            IDEX_1 = 0b001,
            /// Timer D3 Input divider expansion : /3
            IDEX_2 = 0b010,
            /// Timer D3 Input divider expansion : /4
            IDEX_3 = 0b011,
            /// Timer D3 Input divider expansion : /5
            IDEX_4 = 0b100,
            /// Timer D3 Input divider expansion : /6
            IDEX_5 = 0b101,
            /// Timer D3 Input divider expansion : /7
            IDEX_6 = 0b110,
            /// Timer D3 Input divider expansion : /8
            IDEX_7 = 0b111,
        }
    }
    /// Timer D3 Control 2
    rw CTL2 @ 0x04: u16 = 0_0 {
        /// Timer D3 Capture Mode of Channel 0
        CAPM0: 0 = struct CAPM0(bool);
        /// Timer D3 Capture Mode of Channel 1
        CAPM1: 1 = struct CAPM1(bool);
        /// Timer D3 Capture Mode of Channel 2
        CAPM2: 2 = struct CAPM2(bool);
        /// Timer D3 Capture Mode of Channel 3
        CAPM3: 3 = struct CAPM3(bool);
        /// Timer D3 Capture Mode of Channel 4
        CAPM4: 4 = struct CAPM4(bool);
        /// Timer D3 Capture Mode of Channel 5
        CAPM5: 5 = struct CAPM5(bool);
        /// Timer D3 Capture Mode of Channel 6
        CAPM6: 6 = struct CAPM6(bool);
    }
    /// Timer D3 Counter
    rw R @ 0x06: u16 = 0_0 {
        /// Timer D3 Counter
        R: 0..15 = struct RField(u16);
    }
    /// Timer D3 Capture/Compare Control 0
    rw CCTL0 @ 0x08: u16 = 0_0 {
        /// Capture/compare interrupt flag
        CCIFG0: 0 = struct CCIFG0(bool);
        /// Capture/compare overflow flag
        COV0: 1 = struct COV0(bool);
        /// PWM Output signal if output mode 0
        OUT0: 2 = struct OUT0(bool);
        /// Capture input signal (read)
        CCI0: 3 = struct CCI0(bool);
        /// Capture/compare interrupt enable
        CCIE0: 4 = struct CCIE0(bool);
        /// Output mode 2
        OUTMOD0: 5..7 = enum OUTMOD0 {
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
        CAP0: 8 = struct CAP0(bool);
        /// Compare latch load source 1
        CLLD0: 9..10 = enum CLLD0 {
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
        SCS0: 11 = struct SCS0(bool);
        /// Capture input select 1
        CCIS0: 12..13 = enum CCIS0 {
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
        CM0: 14..15 = enum CM0 {
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
    rw CCR0 @ 0x0a: u16 = 0_0 {
        /// Timer D3 Capture/Compare 0
        CCR0: 0..15 = struct CCR0Field(u16);
    }
    /// Timer D3 Capture/Compare Latch 0
    rw CL0 @ 0x0c: u16 = 0_0 {
        /// Timer D3 Capture/Compare Latch 0
        CL0: 0..15 = struct CL0Field(u16);
    }
    /// Timer D3 Capture/Compare Control 1
    rw CCTL1 @ 0x0e: u16 = 0_0 {
        /// Capture/compare interrupt flag
        CCIFG1: 0 = struct CCIFG1(bool);
        /// Capture/compare overflow flag
        COV1: 1 = struct COV1(bool);
        /// PWM Output signal if output mode 0
        OUT1: 2 = struct OUT1(bool);
        /// Capture input signal (read)
        CCI1: 3 = struct CCI1(bool);
        /// Capture/compare interrupt enable
        CCIE1: 4 = struct CCIE1(bool);
        /// Output mode 2
        OUTMOD1: 5..7 = enum OUTMOD1 {
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
        CAP1: 8 = struct CAP1(bool);
        /// Compare latch load source 1
        CLLD1: 9..10 = enum CLLD1 {
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
        SCS1: 11 = struct SCS1(bool);
        /// Capture input select 1
        CCIS1: 12..13 = enum CCIS1 {
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
        CM1: 14..15 = enum CM1 {
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
    rw CCR1 @ 0x10: u16 = 0_0 {
        /// Timer D3 Capture/Compare 1
        CCR1: 0..15 = struct CCR1Field(u16);
    }
    /// Timer D3 Capture/Compare Latch 1
    rw CL1 @ 0x12: u16 = 0_0 {
        /// Timer D3 Capture/Compare Latch 1
        CL1: 0..15 = struct CL1Field(u16);
    }
    /// Timer D3 Capture/Compare Control 2
    rw CCTL2 @ 0x14: u16 = 0_0 {
        /// Capture/compare interrupt flag
        CCIFG2: 0 = struct CCIFG2(bool);
        /// Capture/compare overflow flag
        COV2: 1 = struct COV2(bool);
        /// PWM Output signal if output mode 0
        OUT2: 2 = struct OUT2(bool);
        /// Capture input signal (read)
        CCI2: 3 = struct CCI2(bool);
        /// Capture/compare interrupt enable
        CCIE2: 4 = struct CCIE2(bool);
        /// Output mode 2
        OUTMOD2: 5..7 = enum OUTMOD2 {
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
        CAP2: 8 = struct CAP2(bool);
        /// Compare latch load source 1
        CLLD2: 9..10 = enum CLLD2 {
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
        SCS2: 11 = struct SCS2(bool);
        /// Capture input select 1
        CCIS2: 12..13 = enum CCIS2 {
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
        CM2: 14..15 = enum CM2 {
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
    rw CCR2 @ 0x16: u16 = 0_0 {
        /// Timer D3 Capture/Compare 2
        CCR2: 0..15 = struct CCR2Field(u16);
    }
    /// Timer D3 Capture/Compare Latch 2
    rw CL2 @ 0x18: u16 = 0_0 {
        /// Timer D3 Capture/Compare Latch 2
        CL2: 0..15 = struct CL2Field(u16);
    }
    /// Timer D3 High-resolution Control Register 0
    rw HCTL0 @ 0x38: u16 = 0_0 {
        /// Timer D3 High-Resolution Enable
        HEN: 0 = struct HEN(bool);
        /// Timer D3 High-Resolution Regulated Mode
        HREGEN: 1 = struct HREGEN(bool);
        /// Timer D3 High-Resolution clock error accum. enable
        HEAEN: 2 = struct HEAEN(bool);
        /// Timer D3 High-Resolution Generator forced on
        HRON: 3 = struct HRON(bool);
        /// Timer D3 High-Resoltuion Clock Mult. Bit: 0
        HM: 4..5 = enum HM {
            /// Timer D3 High-Resoltuion Clock Mult.: 8x TimerD clock
            HM_0 = 0b00,
            /// Timer D3 High-Resoltuion Clock Mult.: 16x TimerD clock
            HM_1 = 0b01,
        }
        /// Timer D3 High-Resolution clock divider Bit: 0
        HD: 6..7 = enum HD {
            /// Timer D3 High-Resolution clock divider: /1
            HD_0 = 0b00,
            /// Timer D3 High-Resolution clock divider: /2
            HD_1 = 0b01,
            /// Timer D3 High-Resolution clock divider: /4
            HD_2 = 0b10,
            /// Timer D3 High-Resolution clock divider: /8
            HD_3 = 0b11,
        }
        /// Timer0_D7 High-resolution generator fast wakeup enable
        HFW: 8 = struct HFW(bool);
    }
    /// Timer D3 High-resolution Control Register 1
    rw HCTL1 @ 0x3a: u16 = 0_0 {
        /// Timer D3 High-Resolution Clock Trim Bit: 0
        HCLKTRIM0: 1 = struct HCLKTRIM0(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 1
        HCLKTRIM1: 2 = struct HCLKTRIM1(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 2
        HCLKTRIM2: 3 = struct HCLKTRIM2(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 3
        HCLKTRIM3: 4 = struct HCLKTRIM3(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 4
        HCLKTRIM4: 5 = struct HCLKTRIM4(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 5
        HCLKTRIM5: 6 = struct HCLKTRIM5(bool);
        /// Timer D3 High-Resolution Clock Trim Bit: 6
        HCLKTRIM6: 7 = struct HCLKTRIM6(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 0
        HCLKSR0: 8 = struct HCLKSR0(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 1
        HCLKSR1: 9 = struct HCLKSR1(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 2
        HCLKSR2: 10 = struct HCLKSR2(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 3
        HCLKSR3: 11 = struct HCLKSR3(bool);
        /// Timer D3 High-Resolution Clock Sub-Range Bit: 4
        HCLKSR4: 12 = struct HCLKSR4(bool);
        /// Timer D3 High-Resolution Clock Range Bit: 0
        HCLKR0: 13 = struct HCLKR0(bool);
        /// Timer D3 High-Resolution Clock Range Bit: 1
        HCLKR1: 14 = struct HCLKR1(bool);
        /// Timer D3 High-Resolution Coarse Clock Range
        HCLKCR: 15 = struct HCLKCR(bool);
    }
    /// Timer D3 High-resolution Interrupt Register
    rw HINT @ 0x3c: u16 = 0_0 {
        /// Timer D3 High-Res. fail low Interrupt Flag
        HFLIFG: 0 = struct HFLIFG(bool);
        /// Timer D3 High-Res. fail high Interrupt Flag
        HFHIFG: 1 = struct HFHIFG(bool);
        /// Timer D3 High-Res. frequency lock Interrupt Flag
        HLKIFG: 2 = struct HLKIFG(bool);
        /// Timer D3 High-Res. frequency unlock Interrupt Flag
        HUNLKIFG: 3 = struct HUNLKIFG(bool);
        /// Timer D3 High-Res. fail low Interrupt Enable
        HFLIE: 8 = struct HFLIE(bool);
        /// Timer D3 High-Res. fail high Interrupt Enable
        HFHIE: 9 = struct HFHIE(bool);
        /// Timer D3 High-Res. frequency lock Interrupt Enable
        HLKIE: 10 = struct HLKIE(bool);
        /// Timer D3 High-Res. frequency unlock Interrupt Enable
        HUNLKIE: 11 = struct HUNLKIE(bool);
    }
    /// Timer D3 Interrupt Vector Word
    rw IV @ 0x3e: u16 = 0_0 {
        /// Timer D3 Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
}
