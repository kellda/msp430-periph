//! Scan Interface

utils::periph! {
    /// Scan Interface
    ScanInterface;
    /// SIF
    rw DEBUG @ 0x00: u16 = 0_0 {
        /// SIF
        DEBUG: 0..15 = struct DEBUGField(u16);
    }
    /// SIF
    rw CNT @ 0x02: u16 = 0_0 {
        /// SIF
        CNT: 0..15 = struct CNTField(u16);
    }
    /// SIF
    rw PSMV @ 0x04: u16 = 0_0 {
        /// SIF
        PSMV: 0..15 = struct PSMVField(u16);
    }
    /// SIF
    rw CTL1 @ 0x06: u16 = 0_0 {
        /// SIF Enable
        EN: 0 = struct EN(bool);
        /// SIF 0:Normal / 1:Test Mode
        TESTD: 1 = struct TESTD(bool);
        /// SIF Interrupt Flag 0
        IFG0: 2 = struct IFG0(bool);
        /// SIF Interrupt Flag 1
        IFG1: 3 = struct IFG1(bool);
        /// SIF Interrupt Flag 2
        IFG2: 4 = struct IFG2(bool);
        /// SIF Interrupt Flag 3
        IFG3: 5 = struct IFG3(bool);
        /// SIF Interrupt Flag 4
        IFG4: 6 = struct IFG4(bool);
        /// SIF Interrupt Flag 5
        IFG5: 7 = struct IFG5(bool);
        /// SIF Interrupt Flag 6
        IFG6: 8 = struct IFG6(bool);
        /// SIF Interrupt Enable 0
        IE0: 9 = struct IE0(bool);
        /// SIF Interrupt Enable 1
        IE1: 10 = struct IE1(bool);
        /// SIF Interrupt Enable 2
        IE2: 11 = struct IE2(bool);
        /// SIF Interrupt Enable 3
        IE3: 12 = struct IE3(bool);
        /// SIF Interrupt Enable 4
        IE4: 13 = struct IE4(bool);
        /// SIF Interrupt Enable 5
        IE5: 14 = struct IE5(bool);
        /// SIF Interrupt Enable 6
        IE6: 15 = struct IE6(bool);
    }
    /// SIF
    rw CTL2 @ 0x08: u16 = 0_0 {
        /// SIF TCH0 result
        TCH0OUT: 0 = struct TCH0OUT(bool);
        /// SIF TCH1 result
        TCH1OUT: 1 = struct TCH1OUT(bool);
        /// SIF 1. Channel select 0
        TCH00: 2 = struct TCH00(bool);
        /// SIF 1. Channel select 1
        TCH01: 3 = struct TCH01(bool);
        /// SIF 2. Channel select 0
        TCH10: 4 = struct TCH10(bool);
        /// SIF 2. Channel select 1
        TCH11: 5 = struct TCH11(bool);
        /// SIF Enable Transistors
        TEN: 6 = struct TEN(bool);
        /// SIF Sample on/off
        SH: 7 = struct SH(bool);
        /// SIF VCC/2 Generator off/on
        VCC2: 8 = struct VCC2(bool);
        /// SIF Select Terminal for sample Cap.
        VSS: 9 = struct VSS(bool);
        /// SIF Selection of SIFCI3
        CACI3: 10 = struct CACI3(bool);
        /// SIF Comparator Input Select
        CISEL: 11 = struct CISEL(bool);
        /// SIF Select CA Source
        CAX: 12 = struct CAX(bool);
        /// SIF Invert CA Output 0:off/1:on
        CAINV: 13 = struct CAINV(bool);
        /// SIF Switch CA on
        CAON: 14 = struct CAON(bool);
        /// SIF Switch DAC on
        DACON: 15 = struct DACON(bool);
    }
    /// SIF
    rw CTL3 @ 0x0a: u16 = 0_0 {
        /// SIF Sensor 0 Out
        SIF0OUT: 0 = struct SIF0OUT(bool);
        /// SIF Sensor 1 Out
        SIF1OUT: 1 = struct SIF1OUT(bool);
        /// SIF Sensor 2 Out
        SIF2OUT: 2 = struct SIF2OUT(bool);
        /// SIF Sensor 3 Out
        SIF3OUT: 3 = struct SIF3OUT(bool);
        /// SIF SIFIFG0 level select
        IFGSET0: 4 = struct IFGSET0(bool);
        /// SIF SIFIFG1 level select
        IFGSET1: 5 = struct IFGSET1(bool);
        /// SIF SIFIFG2 level select
        IFGSET2: 6 = struct IFGSET2(bool);
        /// SIF Capture Select
        CS: 7 = struct CS(bool);
        /// SIF SIFIFG3 Int.Flag Source 0
        IS1: 8..9 = enum IS1 {
            /// SIF SIFIFG3 set with each count of SIFCNT1
            IS1_0 = 0b00,
            /// SIF SIFIFG3 set if (SIFCNT1 mod  4)=0
            IS1_1 = 0b01,
            /// SIF SIFIFG3 set if (SIFCNT1 mod 64)=0
            IS1_2 = 0b10,
            /// SIF SIFIFG3 set if SIFCNT1 rolls over
            IS1_3 = 0b11,
        }
        /// SIF SIFIFG4 Int.Flag Source 0
        IS2: 10..11 = enum IS2 {
            /// SIF SIFIFG4 set with each count of SIFCNT2
            IS2_0 = 0b00,
            /// SIF SIFIFG4 set if (SIFCNT2 mod  4)=0
            IS2_1 = 0b01,
            /// SIF SIFIFG4 set if (SIFCNT2 mod 64)=0
            IS2_2 = 0b10,
            /// SIF SIFIFG4 set if SIFCNT2 rolls over
            IS2_3 = 0b11,
        }
        /// SIF S1 Source Select 0
        S1: 12..13 = enum S1 {
            /// SIF S1 Source : SIF0OUT
            S1_0 = 0b00,
            /// SIF S1 Source : SIF1OUT
            S1_1 = 0b01,
            /// SIF S1 Source : SIF2OUT
            S1_2 = 0b10,
            /// SIF S1 Source : SIF3OUT
            S1_3 = 0b11,
        }
        /// SIF S2 Source Select 0
        S2: 14..15 = enum S2 {
            /// SIF S2 Source : SIF0OUT
            S2_0 = 0b00,
            /// SIF S2 Source : SIF1OUT
            S2_1 = 0b01,
            /// SIF S2 Source : SIF2OUT
            S2_2 = 0b10,
            /// SIF S2 Source : SIF3OUT
            S2_3 = 0b11,
        }
    }
    /// SIF
    rw CTL4 @ 0x0c: u16 = 0_0 {
        /// SIF Clock Divider 1.0
        DIV1: 0..1 = enum DIV1 {
            /// SIF Clock Divider 1: /1
            DIV1_1 = 0b00,
            /// SIF Clock Divider 1: /2
            DIV1_2 = 0b01,
            /// SIF Clock Divider 1: /4
            DIV1_4 = 0b10,
            /// SIF Clock Divider 1: /8
            DIV1_8 = 0b11,
        }
        /// SIF Clock Divider 2.0
        DIV2: 2..3 = enum DIV2 {
            /// SIF Clock Divider 2: /1
            DIV2_1 = 0b00,
            /// SIF Clock Divider 2: /2
            DIV2_2 = 0b01,
            /// SIF Clock Divider 2: /4
            DIV2_4 = 0b10,
            /// SIF Clock Divider 2: /8
            DIV2_8 = 0b11,
        }
        /// SIF Clock Divider 3.0
        DIV3A0: 4 = struct DIV3A0(bool);
        /// SIF Clock Divider 3.1
        DIV3A1: 5 = struct DIV3A1(bool);
        /// SIF Clock Divider 3.2
        DIV3A2: 6 = struct DIV3A2(bool);
        /// SIF Clock Divider 3.3
        DIV3B0: 7 = struct DIV3B0(bool);
        /// SIF Clock Divider 3.4
        DIV3B1: 8 = struct DIV3B1(bool);
        /// SIF Clock Divider 3.5
        DIV3B2: 9 = struct DIV3B2(bool);
        /// SIF Feedback 6 Enable
        Q6EN: 10 = struct Q6EN(bool);
        /// SIF Feedback 7 Enable
        Q7EN: 11 = struct Q7EN(bool);
        /// SIF Enable SIFCNT1 up count
        CNT1ENP: 12 = struct CNT1ENP(bool);
        /// SIF Enable SIFCNT1 down count
        CNT1ENM: 13 = struct CNT1ENM(bool);
        /// SIF Enable SIFCNT2 count
        CNT2EN: 14 = struct CNT2EN(bool);
        /// SIF Enable Counter Reset on Read
        CNTRST: 15 = struct CNTRST(bool);
    }
    /// SIF
    rw CTL5 @ 0x0e: u16 = 0_0 {
        /// SIF 0:SMCLK for SIFCLK / 1:SIFCLKG for SIFCLK
        CLKEN: 0 = struct CLKEN(bool);
        /// SIF Switch SIFCLKG on
        CLKGON: 1 = struct CLKGON(bool);
        /// SIF Select Nominal Frequ. 0:4MHz / 1:1MHz
        FNOM: 2 = struct FNOM(bool);
        /// SIF Clock Generator frequency adjust 0
        CLKFQ0: 3 = struct CLKFQ0(bool);
        /// SIF Clock Generator frequency adjust 1
        CLKFQ1: 4 = struct CLKFQ1(bool);
        /// SIF Clock Generator frequency adjust 2
        CLKFQ2: 5 = struct CLKFQ2(bool);
        /// SIF Clock Generator frequency adjust 3
        CLKFQ3: 6 = struct CLKFQ3(bool);
        /// SIF Timing State Machine Repeat mode
        TSMRP: 7 = struct TSMRP(bool);
        /// SIF Counter 3.0
        CNT30: 8 = struct CNT30(bool);
        /// SIF Counter 3.1
        CNT31: 9 = struct CNT31(bool);
        /// SIF Counter 3.2
        CNT32: 10 = struct CNT32(bool);
        /// SIF Counter 3.3
        CNT33: 11 = struct CNT33(bool);
        /// SIF Counter 3.4
        CNT34: 12 = struct CNT34(bool);
        /// SIF Counter 3.5
        CNT35: 13 = struct CNT35(bool);
        /// SIF Counter 3.6
        CNT36: 14 = struct CNT36(bool);
        /// SIF Counter 3.7
        CNT37: 15 = struct CNT37(bool);
    }
    /// SIF
    rw DACR0 @ 0x10: u16 = 0_0 {
        /// SIF
        DACR0: 0..15 = struct DACR0Field(u16);
    }
    /// SIF
    rw DACR1 @ 0x12: u16 = 0_0 {
        /// SIF
        DACR1: 0..15 = struct DACR1Field(u16);
    }
    /// SIF
    rw DACR2 @ 0x14: u16 = 0_0 {
        /// SIF
        DACR2: 0..15 = struct DACR2Field(u16);
    }
    /// SIF
    rw DACR3 @ 0x16: u16 = 0_0 {
        /// SIF
        DACR3: 0..15 = struct DACR3Field(u16);
    }
    /// SIF
    rw DACR4 @ 0x18: u16 = 0_0 {
        /// SIF
        DACR4: 0..15 = struct DACR4Field(u16);
    }
    /// SIF
    rw DACR5 @ 0x1a: u16 = 0_0 {
        /// SIF
        DACR5: 0..15 = struct DACR5Field(u16);
    }
    /// SIF
    rw DACR6 @ 0x1c: u16 = 0_0 {
        /// SIF
        DACR6: 0..15 = struct DACR6Field(u16);
    }
    /// SIF
    rw DACR7 @ 0x1e: u16 = 0_0 {
        /// SIF
        DACR7: 0..15 = struct DACR7Field(u16);
    }
    /// SIF
    rw TSM0 @ 0x20: u16 = 0_0 {
        /// SIF
        TSM0: 0..15 = struct TSM0Field(u16);
    }
    /// SIF
    rw TSM1 @ 0x22: u16 = 0_0 {
        /// SIF
        TSM1: 0..15 = struct TSM1Field(u16);
    }
    /// SIF
    rw TSM2 @ 0x24: u16 = 0_0 {
        /// SIF
        TSM2: 0..15 = struct TSM2Field(u16);
    }
    /// SIF
    rw TSM3 @ 0x26: u16 = 0_0 {
        /// SIF
        TSM3: 0..15 = struct TSM3Field(u16);
    }
    /// SIF
    rw TSM4 @ 0x28: u16 = 0_0 {
        /// SIF
        TSM4: 0..15 = struct TSM4Field(u16);
    }
    /// SIF
    rw TSM5 @ 0x2a: u16 = 0_0 {
        /// SIF
        TSM5: 0..15 = struct TSM5Field(u16);
    }
    /// SIF
    rw TSM6 @ 0x2c: u16 = 0_0 {
        /// SIF
        TSM6: 0..15 = struct TSM6Field(u16);
    }
    /// SIF
    rw TSM7 @ 0x2e: u16 = 0_0 {
        /// SIF
        TSM7: 0..15 = struct TSM7Field(u16);
    }
    /// SIF
    rw TSM8 @ 0x30: u16 = 0_0 {
        /// SIF
        TSM8: 0..15 = struct TSM8Field(u16);
    }
    /// SIF
    rw TSM9 @ 0x32: u16 = 0_0 {
        /// SIF
        TSM9: 0..15 = struct TSM9Field(u16);
    }
    /// SIF
    rw TSM10 @ 0x34: u16 = 0_0 {
        /// SIF
        TSM10: 0..15 = struct TSM10Field(u16);
    }
    /// SIF
    rw TSM11 @ 0x36: u16 = 0_0 {
        /// SIF
        TSM11: 0..15 = struct TSM11Field(u16);
    }
    /// SIF
    rw TSM12 @ 0x38: u16 = 0_0 {
        /// SIF
        TSM12: 0..15 = struct TSM12Field(u16);
    }
    /// SIF
    rw TSM13 @ 0x3a: u16 = 0_0 {
        /// SIF
        TSM13: 0..15 = struct TSM13Field(u16);
    }
    /// SIF
    rw TSM14 @ 0x3c: u16 = 0_0 {
        /// SIF
        TSM14: 0..15 = struct TSM14Field(u16);
    }
    /// SIF
    rw TSM15 @ 0x3e: u16 = 0_0 {
        /// SIF
        TSM15: 0..15 = struct TSM15Field(u16);
    }
    /// SIF
    rw TSM16 @ 0x40: u16 = 0_0 {
        /// SIF
        TSM16: 0..15 = struct TSM16Field(u16);
    }
    /// SIF
    rw TSM17 @ 0x42: u16 = 0_0 {
        /// SIF
        TSM17: 0..15 = struct TSM17Field(u16);
    }
    /// SIF
    rw TSM18 @ 0x44: u16 = 0_0 {
        /// SIF
        TSM18: 0..15 = struct TSM18Field(u16);
    }
    /// SIF
    rw TSM19 @ 0x46: u16 = 0_0 {
        /// SIF
        TSM19: 0..15 = struct TSM19Field(u16);
    }
    /// SIF
    rw TSM20 @ 0x48: u16 = 0_0 {
        /// SIF
        TSM20: 0..15 = struct TSM20Field(u16);
    }
    /// SIF
    rw TSM21 @ 0x4a: u16 = 0_0 {
        /// SIF
        TSM21: 0..15 = struct TSM21Field(u16);
    }
    /// SIF
    rw TSM22 @ 0x4c: u16 = 0_0 {
        /// SIF
        TSM22: 0..15 = struct TSM22Field(u16);
    }
    /// SIF
    rw TSM23 @ 0x4e: u16 = 0_0 {
        /// SIF
        TSM23: 0..15 = struct TSM23Field(u16);
    }
}
