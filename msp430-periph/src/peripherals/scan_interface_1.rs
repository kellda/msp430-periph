//! Scan Interface

utils::periph! {
    /// Scan Interface
    ScanInterface;
    /// SIF
    rw SIFDEBUG @ 0x00: u16 = 0_0 {
        /// SIF
        SIFDEBUG: 0..15 = struct SIFDEBUGField(u16);
    }
    /// SIF
    rw SIFCNT @ 0x02: u16 = 0_0 {
        /// SIF
        SIFCNT: 0..15 = struct SIFCNTField(u16);
    }
    /// SIF
    rw SIFPSMV @ 0x04: u16 = 0_0 {
        /// SIF
        SIFPSMV: 0..15 = struct SIFPSMVField(u16);
    }
    /// SIF
    rw SIFCTL1 @ 0x06: u16 = 0_0 {
        /// SIF Enable
        SIFEN: 0 = struct SIFEN(bool);
        /// SIF 0:Normal / 1:Test Mode
        SIFTESTD: 1 = struct SIFTESTD(bool);
        /// SIF Interrupt Flag 0
        SIFIFG0: 2 = struct SIFIFG0(bool);
        /// SIF Interrupt Flag 1
        SIFIFG1: 3 = struct SIFIFG1(bool);
        /// SIF Interrupt Flag 2
        SIFIFG2: 4 = struct SIFIFG2(bool);
        /// SIF Interrupt Flag 3
        SIFIFG3: 5 = struct SIFIFG3(bool);
        /// SIF Interrupt Flag 4
        SIFIFG4: 6 = struct SIFIFG4(bool);
        /// SIF Interrupt Flag 5
        SIFIFG5: 7 = struct SIFIFG5(bool);
        /// SIF Interrupt Flag 6
        SIFIFG6: 8 = struct SIFIFG6(bool);
        /// SIF Interrupt Enable 0
        SIFIE0: 9 = struct SIFIE0(bool);
        /// SIF Interrupt Enable 1
        SIFIE1: 10 = struct SIFIE1(bool);
        /// SIF Interrupt Enable 2
        SIFIE2: 11 = struct SIFIE2(bool);
        /// SIF Interrupt Enable 3
        SIFIE3: 12 = struct SIFIE3(bool);
        /// SIF Interrupt Enable 4
        SIFIE4: 13 = struct SIFIE4(bool);
        /// SIF Interrupt Enable 5
        SIFIE5: 14 = struct SIFIE5(bool);
        /// SIF Interrupt Enable 6
        SIFIE6: 15 = struct SIFIE6(bool);
    }
    /// SIF
    rw SIFCTL2 @ 0x08: u16 = 0_0 {
        /// SIF TCH0 result
        SIFTCH0OUT: 0 = struct SIFTCH0OUT(bool);
        /// SIF TCH1 result
        SIFTCH1OUT: 1 = struct SIFTCH1OUT(bool);
        /// SIF 1. Channel select 0
        SIFTCH00: 2 = struct SIFTCH00(bool);
        /// SIF 1. Channel select 1
        SIFTCH01: 3 = struct SIFTCH01(bool);
        /// SIF 2. Channel select 0
        SIFTCH10: 4 = struct SIFTCH10(bool);
        /// SIF 2. Channel select 1
        SIFTCH11: 5 = struct SIFTCH11(bool);
        /// SIF Enable Transistors
        SIFTEN: 6 = struct SIFTEN(bool);
        /// SIF Sample on/off
        SIFSH: 7 = struct SIFSH(bool);
        /// SIF VCC/2 Generator off/on
        SIFVCC2: 8 = struct SIFVCC2(bool);
        /// SIF Select Terminal for sample Cap.
        SIFVSS: 9 = struct SIFVSS(bool);
        /// SIF Selection of SIFCI3
        SIFCACI3: 10 = struct SIFCACI3(bool);
        /// SIF Comparator Input Select
        SIFCISEL: 11 = struct SIFCISEL(bool);
        /// SIF Select CA Source
        SIFCAX: 12 = struct SIFCAX(bool);
        /// SIF Invert CA Output 0:off/1:on
        SIFCAINV: 13 = struct SIFCAINV(bool);
        /// SIF Switch CA on
        SIFCAON: 14 = struct SIFCAON(bool);
        /// SIF Switch DAC on
        SIFDACON: 15 = struct SIFDACON(bool);
    }
    /// SIF
    rw SIFCTL3 @ 0x0a: u16 = 0_0 {
        /// SIF Sensor 0 Out
        SIF0OUT: 0 = struct SIF0OUT(bool);
        /// SIF Sensor 1 Out
        SIF1OUT: 1 = struct SIF1OUT(bool);
        /// SIF Sensor 2 Out
        SIF2OUT: 2 = struct SIF2OUT(bool);
        /// SIF Sensor 3 Out
        SIF3OUT: 3 = struct SIF3OUT(bool);
        /// SIF SIFIFG0 level select
        SIFIFGSET0: 4 = struct SIFIFGSET0(bool);
        /// SIF SIFIFG1 level select
        SIFIFGSET1: 5 = struct SIFIFGSET1(bool);
        /// SIF SIFIFG2 level select
        SIFIFGSET2: 6 = struct SIFIFGSET2(bool);
        /// SIF Capture Select
        SIFCS: 7 = struct SIFCS(bool);
        /// SIF SIFIFG3 Int.Flag Source 0
        SIFIS1: 8..9 = enum SIFIS1 {
            /// SIF SIFIFG3 set with each count of SIFCNT1
            SIFIS1_0 = 0b00,
            /// SIF SIFIFG3 set if (SIFCNT1 mod  4)=0
            SIFIS1_1 = 0b01,
            /// SIF SIFIFG3 set if (SIFCNT1 mod 64)=0
            SIFIS1_2 = 0b10,
            /// SIF SIFIFG3 set if SIFCNT1 rolls over
            SIFIS1_3 = 0b11,
        }
        /// SIF SIFIFG4 Int.Flag Source 0
        SIFIS2: 10..11 = enum SIFIS2 {
            /// SIF SIFIFG4 set with each count of SIFCNT2
            SIFIS2_0 = 0b00,
            /// SIF SIFIFG4 set if (SIFCNT2 mod  4)=0
            SIFIS2_1 = 0b01,
            /// SIF SIFIFG4 set if (SIFCNT2 mod 64)=0
            SIFIS2_2 = 0b10,
            /// SIF SIFIFG4 set if SIFCNT2 rolls over
            SIFIS2_3 = 0b11,
        }
        /// SIF S1 Source Select 0
        SIFS1: 12..13 = enum SIFS1 {
            /// SIF S1 Source : SIF0OUT
            SIFS1_0 = 0b00,
            /// SIF S1 Source : SIF1OUT
            SIFS1_1 = 0b01,
            /// SIF S1 Source : SIF2OUT
            SIFS1_2 = 0b10,
            /// SIF S1 Source : SIF3OUT
            SIFS1_3 = 0b11,
        }
        /// SIF S2 Source Select 0
        SIFS2: 14..15 = enum SIFS2 {
            /// SIF S2 Source : SIF0OUT
            SIFS2_0 = 0b00,
            /// SIF S2 Source : SIF1OUT
            SIFS2_1 = 0b01,
            /// SIF S2 Source : SIF2OUT
            SIFS2_2 = 0b10,
            /// SIF S2 Source : SIF3OUT
            SIFS2_3 = 0b11,
        }
    }
    /// SIF
    rw SIFCTL4 @ 0x0c: u16 = 0_0 {
        /// SIF Clock Divider 1.0
        SIFDIV1: 0..1 = enum SIFDIV1 {
            /// SIF Clock Divider 1: /1
            SIFDIV1_1 = 0b00,
            /// SIF Clock Divider 1: /2
            SIFDIV1_2 = 0b01,
            /// SIF Clock Divider 1: /4
            SIFDIV1_4 = 0b10,
            /// SIF Clock Divider 1: /8
            SIFDIV1_8 = 0b11,
        }
        /// SIF Clock Divider 2.0
        SIFDIV2: 2..3 = enum SIFDIV2 {
            /// SIF Clock Divider 2: /1
            SIFDIV2_1 = 0b00,
            /// SIF Clock Divider 2: /2
            SIFDIV2_2 = 0b01,
            /// SIF Clock Divider 2: /4
            SIFDIV2_4 = 0b10,
            /// SIF Clock Divider 2: /8
            SIFDIV2_8 = 0b11,
        }
        /// SIF Clock Divider 3.0
        SIFDIV3A0: 4 = struct SIFDIV3A0(bool);
        /// SIF Clock Divider 3.1
        SIFDIV3A1: 5 = struct SIFDIV3A1(bool);
        /// SIF Clock Divider 3.2
        SIFDIV3A2: 6 = struct SIFDIV3A2(bool);
        /// SIF Clock Divider 3.3
        SIFDIV3B0: 7 = struct SIFDIV3B0(bool);
        /// SIF Clock Divider 3.4
        SIFDIV3B1: 8 = struct SIFDIV3B1(bool);
        /// SIF Clock Divider 3.5
        SIFDIV3B2: 9 = struct SIFDIV3B2(bool);
        /// SIF Feedback 6 Enable
        SIFQ6EN: 10 = struct SIFQ6EN(bool);
        /// SIF Feedback 7 Enable
        SIFQ7EN: 11 = struct SIFQ7EN(bool);
        /// SIF Enable SIFCNT1 up count
        SIFCNT1ENP: 12 = struct SIFCNT1ENP(bool);
        /// SIF Enable SIFCNT1 down count
        SIFCNT1ENM: 13 = struct SIFCNT1ENM(bool);
        /// SIF Enable SIFCNT2 count
        SIFCNT2EN: 14 = struct SIFCNT2EN(bool);
        /// SIF Enable Counter Reset on Read
        SIFCNTRST: 15 = struct SIFCNTRST(bool);
    }
    /// SIF
    rw SIFCTL5 @ 0x0e: u16 = 0_0 {
        /// SIF 0:SMCLK for SIFCLK / 1:SIFCLKG for SIFCLK
        SIFCLKEN: 0 = struct SIFCLKEN(bool);
        /// SIF Switch SIFCLKG on
        SIFCLKGON: 1 = struct SIFCLKGON(bool);
        /// SIF Select Nominal Frequ. 0:4MHz / 1:1MHz
        SIFFNOM: 2 = struct SIFFNOM(bool);
        /// SIF Clock Generator frequency adjust 0
        SIFCLKFQ0: 3 = struct SIFCLKFQ0(bool);
        /// SIF Clock Generator frequency adjust 1
        SIFCLKFQ1: 4 = struct SIFCLKFQ1(bool);
        /// SIF Clock Generator frequency adjust 2
        SIFCLKFQ2: 5 = struct SIFCLKFQ2(bool);
        /// SIF Clock Generator frequency adjust 3
        SIFCLKFQ3: 6 = struct SIFCLKFQ3(bool);
        /// SIF Timing State Machine Repeat mode
        SIFTSMRP: 7 = struct SIFTSMRP(bool);
        /// SIF Counter 3.0
        SIFCNT30: 8 = struct SIFCNT30(bool);
        /// SIF Counter 3.1
        SIFCNT31: 9 = struct SIFCNT31(bool);
        /// SIF Counter 3.2
        SIFCNT32: 10 = struct SIFCNT32(bool);
        /// SIF Counter 3.3
        SIFCNT33: 11 = struct SIFCNT33(bool);
        /// SIF Counter 3.4
        SIFCNT34: 12 = struct SIFCNT34(bool);
        /// SIF Counter 3.5
        SIFCNT35: 13 = struct SIFCNT35(bool);
        /// SIF Counter 3.6
        SIFCNT36: 14 = struct SIFCNT36(bool);
        /// SIF Counter 3.7
        SIFCNT37: 15 = struct SIFCNT37(bool);
    }
    /// SIF
    rw SIFDACR0 @ 0x10: u16 = 0_0 {
        /// SIF
        SIFDACR0: 0..15 = struct SIFDACR0Field(u16);
    }
    /// SIF
    rw SIFDACR1 @ 0x12: u16 = 0_0 {
        /// SIF
        SIFDACR1: 0..15 = struct SIFDACR1Field(u16);
    }
    /// SIF
    rw SIFDACR2 @ 0x14: u16 = 0_0 {
        /// SIF
        SIFDACR2: 0..15 = struct SIFDACR2Field(u16);
    }
    /// SIF
    rw SIFDACR3 @ 0x16: u16 = 0_0 {
        /// SIF
        SIFDACR3: 0..15 = struct SIFDACR3Field(u16);
    }
    /// SIF
    rw SIFDACR4 @ 0x18: u16 = 0_0 {
        /// SIF
        SIFDACR4: 0..15 = struct SIFDACR4Field(u16);
    }
    /// SIF
    rw SIFDACR5 @ 0x1a: u16 = 0_0 {
        /// SIF
        SIFDACR5: 0..15 = struct SIFDACR5Field(u16);
    }
    /// SIF
    rw SIFDACR6 @ 0x1c: u16 = 0_0 {
        /// SIF
        SIFDACR6: 0..15 = struct SIFDACR6Field(u16);
    }
    /// SIF
    rw SIFDACR7 @ 0x1e: u16 = 0_0 {
        /// SIF
        SIFDACR7: 0..15 = struct SIFDACR7Field(u16);
    }
    /// SIF
    rw SIFTSM0 @ 0x20: u16 = 0_0 {
        /// SIF
        SIFTSM0: 0..15 = struct SIFTSM0Field(u16);
    }
    /// SIF
    rw SIFTSM1 @ 0x22: u16 = 0_0 {
        /// SIF
        SIFTSM1: 0..15 = struct SIFTSM1Field(u16);
    }
    /// SIF
    rw SIFTSM2 @ 0x24: u16 = 0_0 {
        /// SIF
        SIFTSM2: 0..15 = struct SIFTSM2Field(u16);
    }
    /// SIF
    rw SIFTSM3 @ 0x26: u16 = 0_0 {
        /// SIF
        SIFTSM3: 0..15 = struct SIFTSM3Field(u16);
    }
    /// SIF
    rw SIFTSM4 @ 0x28: u16 = 0_0 {
        /// SIF
        SIFTSM4: 0..15 = struct SIFTSM4Field(u16);
    }
    /// SIF
    rw SIFTSM5 @ 0x2a: u16 = 0_0 {
        /// SIF
        SIFTSM5: 0..15 = struct SIFTSM5Field(u16);
    }
    /// SIF
    rw SIFTSM6 @ 0x2c: u16 = 0_0 {
        /// SIF
        SIFTSM6: 0..15 = struct SIFTSM6Field(u16);
    }
    /// SIF
    rw SIFTSM7 @ 0x2e: u16 = 0_0 {
        /// SIF
        SIFTSM7: 0..15 = struct SIFTSM7Field(u16);
    }
    /// SIF
    rw SIFTSM8 @ 0x30: u16 = 0_0 {
        /// SIF
        SIFTSM8: 0..15 = struct SIFTSM8Field(u16);
    }
    /// SIF
    rw SIFTSM9 @ 0x32: u16 = 0_0 {
        /// SIF
        SIFTSM9: 0..15 = struct SIFTSM9Field(u16);
    }
    /// SIF
    rw SIFTSM10 @ 0x34: u16 = 0_0 {
        /// SIF
        SIFTSM10: 0..15 = struct SIFTSM10Field(u16);
    }
    /// SIF
    rw SIFTSM11 @ 0x36: u16 = 0_0 {
        /// SIF
        SIFTSM11: 0..15 = struct SIFTSM11Field(u16);
    }
    /// SIF
    rw SIFTSM12 @ 0x38: u16 = 0_0 {
        /// SIF
        SIFTSM12: 0..15 = struct SIFTSM12Field(u16);
    }
    /// SIF
    rw SIFTSM13 @ 0x3a: u16 = 0_0 {
        /// SIF
        SIFTSM13: 0..15 = struct SIFTSM13Field(u16);
    }
    /// SIF
    rw SIFTSM14 @ 0x3c: u16 = 0_0 {
        /// SIF
        SIFTSM14: 0..15 = struct SIFTSM14Field(u16);
    }
    /// SIF
    rw SIFTSM15 @ 0x3e: u16 = 0_0 {
        /// SIF
        SIFTSM15: 0..15 = struct SIFTSM15Field(u16);
    }
    /// SIF
    rw SIFTSM16 @ 0x40: u16 = 0_0 {
        /// SIF
        SIFTSM16: 0..15 = struct SIFTSM16Field(u16);
    }
    /// SIF
    rw SIFTSM17 @ 0x42: u16 = 0_0 {
        /// SIF
        SIFTSM17: 0..15 = struct SIFTSM17Field(u16);
    }
    /// SIF
    rw SIFTSM18 @ 0x44: u16 = 0_0 {
        /// SIF
        SIFTSM18: 0..15 = struct SIFTSM18Field(u16);
    }
    /// SIF
    rw SIFTSM19 @ 0x46: u16 = 0_0 {
        /// SIF
        SIFTSM19: 0..15 = struct SIFTSM19Field(u16);
    }
    /// SIF
    rw SIFTSM20 @ 0x48: u16 = 0_0 {
        /// SIF
        SIFTSM20: 0..15 = struct SIFTSM20Field(u16);
    }
    /// SIF
    rw SIFTSM21 @ 0x4a: u16 = 0_0 {
        /// SIF
        SIFTSM21: 0..15 = struct SIFTSM21Field(u16);
    }
    /// SIF
    rw SIFTSM22 @ 0x4c: u16 = 0_0 {
        /// SIF
        SIFTSM22: 0..15 = struct SIFTSM22Field(u16);
    }
    /// SIF
    rw SIFTSM23 @ 0x4e: u16 = 0_0 {
        /// SIF
        SIFTSM23: 0..15 = struct SIFTSM23Field(u16);
    }
}
