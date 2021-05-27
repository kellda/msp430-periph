//! Timer B7

utils::periph! {
    /// Timer B7
    TimerB7;
    /// Timer B7 Control
    rw CTL @ 0x00: u16 = 0_0 {
        /// Timer B7 interrupt flag
        IFG: 0 = struct IFG(bool);
        /// Timer B7 interrupt enable
        IE: 1 = struct IE(bool);
        /// Timer B7 counter clear
        CLR: 2 = struct CLR(bool);
        /// Timer B7 mode control 1
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
        /// Timer B7 clock input divider 1
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
            /// Clock Source: TBCLK
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
        /// Timer B7 Compare latch load group 1
        CLGRP: 13..14 = enum CLGRP {
            /// Timer B7 Group: 0 - individually
            CLGRP_0 = 0b00,
            /// Timer B7 Group: 1 - 3 groups (1-2
            CLGRP_1 = 0b01,
            /// Timer B7 Group: 2 - 2 groups (1-3
            CLGRP_2 = 0b10,
            /// Timer B7 Group: 3 - 1 group (all)
            CLGRP_3 = 0b11,
        }
    }
    /// Timer B7 Capture/Compare Control 0
    rw CCTL0 @ 0x02: u16 = 0_0 {
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
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
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
    /// Timer B7 Capture/Compare Control 1
    rw CCTL1 @ 0x04: u16 = 0_0 {
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
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
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
    /// Timer B7 Capture/Compare Control 2
    rw CCTL2 @ 0x06: u16 = 0_0 {
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
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
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
    /// Timer B7 Capture/Compare Control 3
    rw CCTL3 @ 0x08: u16 = 0_0 {
        /// Capture/compare interrupt flag
        CCIFG3: 0 = struct CCIFG3(bool);
        /// Capture/compare overflow flag
        COV3: 1 = struct COV3(bool);
        /// PWM Output signal if output mode 0
        OUT3: 2 = struct OUT3(bool);
        /// Capture input signal (read)
        CCI3: 3 = struct CCI3(bool);
        /// Capture/compare interrupt enable
        CCIE3: 4 = struct CCIE3(bool);
        /// Output mode 2
        OUTMOD3: 5..7 = enum OUTMOD3 {
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
        CAP3: 8 = struct CAP3(bool);
        /// Compare latch load source 1
        CLLD3: 9..10 = enum CLLD3 {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        SCS3: 11 = struct SCS3(bool);
        /// Capture input select 1
        CCIS3: 12..13 = enum CCIS3 {
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
        CM3: 14..15 = enum CM3 {
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
    /// Timer B7 Capture/Compare Control 4
    rw CCTL4 @ 0x0a: u16 = 0_0 {
        /// Capture/compare interrupt flag
        CCIFG4: 0 = struct CCIFG4(bool);
        /// Capture/compare overflow flag
        COV4: 1 = struct COV4(bool);
        /// PWM Output signal if output mode 0
        OUT4: 2 = struct OUT4(bool);
        /// Capture input signal (read)
        CCI4: 3 = struct CCI4(bool);
        /// Capture/compare interrupt enable
        CCIE4: 4 = struct CCIE4(bool);
        /// Output mode 2
        OUTMOD4: 5..7 = enum OUTMOD4 {
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
        CAP4: 8 = struct CAP4(bool);
        /// Compare latch load source 1
        CLLD4: 9..10 = enum CLLD4 {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        SCS4: 11 = struct SCS4(bool);
        /// Capture input select 1
        CCIS4: 12..13 = enum CCIS4 {
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
        CM4: 14..15 = enum CM4 {
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
    /// Timer B7 Capture/Compare Control 5
    rw CCTL5 @ 0x0c: u16 = 0_0 {
        /// Capture/compare interrupt flag
        CCIFG5: 0 = struct CCIFG5(bool);
        /// Capture/compare overflow flag
        COV5: 1 = struct COV5(bool);
        /// PWM Output signal if output mode 0
        OUT5: 2 = struct OUT5(bool);
        /// Capture input signal (read)
        CCI5: 3 = struct CCI5(bool);
        /// Capture/compare interrupt enable
        CCIE5: 4 = struct CCIE5(bool);
        /// Output mode 2
        OUTMOD5: 5..7 = enum OUTMOD5 {
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
        CAP5: 8 = struct CAP5(bool);
        /// Compare latch load source 1
        CLLD5: 9..10 = enum CLLD5 {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        SCS5: 11 = struct SCS5(bool);
        /// Capture input select 1
        CCIS5: 12..13 = enum CCIS5 {
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
        CM5: 14..15 = enum CM5 {
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
    /// Timer B7 Capture/Compare Control 6
    rw CCTL6 @ 0x0e: u16 = 0_0 {
        /// Capture/compare interrupt flag
        CCIFG6: 0 = struct CCIFG6(bool);
        /// Capture/compare overflow flag
        COV6: 1 = struct COV6(bool);
        /// PWM Output signal if output mode 0
        OUT6: 2 = struct OUT6(bool);
        /// Capture input signal (read)
        CCI6: 3 = struct CCI6(bool);
        /// Capture/compare interrupt enable
        CCIE6: 4 = struct CCIE6(bool);
        /// Output mode 2
        OUTMOD6: 5..7 = enum OUTMOD6 {
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
        CAP6: 8 = struct CAP6(bool);
        /// Compare latch load source 1
        CLLD6: 9..10 = enum CLLD6 {
            /// Compare latch load sourec : 0 - immediate
            CLLD_0 = 0b00,
            /// Compare latch load sourec : 1 - TBR counts to 0
            CLLD_1 = 0b01,
            /// Compare latch load sourec : 2 - up/down
            CLLD_2 = 0b10,
            /// Compare latch load sourec : 3 - TBR counts to TBCTL0
            CLLD_3 = 0b11,
        }
        /// Capture sychronize
        SCS6: 11 = struct SCS6(bool);
        /// Capture input select 1
        CCIS6: 12..13 = enum CCIS6 {
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
        CM6: 14..15 = enum CM6 {
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
    /// Timer B7
    rw R @ 0x10: u16 = 0_0 {
        /// Timer B7
        R: 0..15 = struct RField(u16);
    }
    /// Timer B7 Capture/Compare 0
    rw CCR0 @ 0x12: u16 = 0_0 {
        /// Timer B7 Capture/Compare 0
        CCR0: 0..15 = struct CCR0Field(u16);
    }
    /// Timer B7 Capture/Compare 1
    rw CCR1 @ 0x14: u16 = 0_0 {
        /// Timer B7 Capture/Compare 1
        CCR1: 0..15 = struct CCR1Field(u16);
    }
    /// Timer B7 Capture/Compare 2
    rw CCR2 @ 0x16: u16 = 0_0 {
        /// Timer B7 Capture/Compare 2
        CCR2: 0..15 = struct CCR2Field(u16);
    }
    /// Timer B7 Capture/Compare 3
    rw CCR3 @ 0x18: u16 = 0_0 {
        /// Timer B7 Capture/Compare 3
        CCR3: 0..15 = struct CCR3Field(u16);
    }
    /// Timer B7 Capture/Compare 4
    rw CCR4 @ 0x1a: u16 = 0_0 {
        /// Timer B7 Capture/Compare 4
        CCR4: 0..15 = struct CCR4Field(u16);
    }
    /// Timer B7 Capture/Compare 5
    rw CCR5 @ 0x1c: u16 = 0_0 {
        /// Timer B7 Capture/Compare 5
        CCR5: 0..15 = struct CCR5Field(u16);
    }
    /// Timer B7 Capture/Compare 6
    rw CCR6 @ 0x1e: u16 = 0_0 {
        /// Timer B7 Capture/Compare 6
        CCR6: 0..15 = struct CCR6Field(u16);
    }
    /// Timer B7 Expansion Register 0
    rw EX0 @ 0x20: u16 = 0_0 {
        /// Timer B7 Input divider expansion Bit: 0
        IDEX: 0..2 = enum IDEX {
            /// Timer B7 Input divider expansion : /1
            IDEX_0 = 0b000,
            /// Timer B7 Input divider expansion : /2
            IDEX_1 = 0b001,
            /// Timer B7 Input divider expansion : /3
            IDEX_2 = 0b010,
            /// Timer B7 Input divider expansion : /4
            IDEX_3 = 0b011,
            /// Timer B7 Input divider expansion : /5
            IDEX_4 = 0b100,
            /// Timer B7 Input divider expansion : /6
            IDEX_5 = 0b101,
            /// Timer B7 Input divider expansion : /7
            IDEX_6 = 0b110,
            /// Timer B7 Input divider expansion : /8
            IDEX_7 = 0b111,
        }
    }
    /// Timer B7 Interrupt Vector Word
    rw IV @ 0x2e: u16 = 0_0 {
        /// Timer B7 Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
}
