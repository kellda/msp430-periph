//! Analog Pool

utils::periph! {
    /// Analog Pool
    AnalogPool;
    /// A-POOL Configuration Register
    rw CNF @ 0x00: u16 = 0_0 {
        /// A-POOL TimerA0 trigger enable
        TA0EN: 0 = struct TA0EN(bool);
        /// A-POOL TimerA1 trigger enable
        TA1EN: 1 = struct TA1EN(bool);
        /// A-POOL Deglitch Filter Bit: 0
        DFSET: 2..3 = enum DFSET {
            /// A-POOL Deglitch Filter select: 0
            DFSET_0 = 0b00,
            /// A-POOL Deglitch Filter select: 1
            DFSET_1 = 0b01,
            /// A-POOL Deglitch Filter select: 2
            DFSET_2 = 0b10,
            /// A-POOL Deglitch Filter select: 3
            DFSET_3 = 0b11,
        }
        /// A-POOL Latch comparator
        LCMP: 4 = struct LCMP(bool);
        /// A-POOL Comparator enable
        CMPON: 5 = struct CMPON(bool);
        /// A-POOL DAC buffer enable signal
        DBON: 6 = struct DBON(bool);
        /// A-POOL Enable for convertergnal
        CONVON: 7 = struct CONVON(bool);
        /// A-POOL Conversion clock select Bit: 0
        CLKSEL: 8..9 = enum CLKSEL {
            /// A-POOL Conversion clock select: 0
            CLKSEL_0 = 0b00,
            /// A-POOL Conversion clock select: 1
            CLKSEL_1 = 0b01,
            /// A-POOL Conversion clock select: 2
            CLKSEL_2 = 0b10,
        }
        /// A-POOL Enable bit for loading conversion buffer
        EOCBU: 10 = struct EOCBU(bool);
        /// A-POOL Automatic update of conversion register
        ATBU: 11 = struct ATBU(bool);
        /// A-POOL Analog input A3 access Bit
        A3PSEL: 12 = struct A3PSEL(bool);
        /// A-POOL Internal voltage reference enable
        APREFON: 13 = struct APREFON(bool);
        /// A-POOL Reference voltage pin enable
        VREFEN: 14 = struct VREFEN(bool);
    }
    /// A-POOL Control Register 1
    rw CTL @ 0x02: u16 = 0_0 {
        /// A-POOL Output driver enable
        ODEN: 0 = struct ODEN(bool);
        /// A-POOL Output swap
        OSWP: 1 = struct OSWP(bool);
        /// A-POOL Output buffer select
        OSEL: 2 = struct OSEL(bool);
        /// A-POOL Slope select of converter
        SLOPE: 3 = struct SLOPE(bool);
        /// A-POOL Neg. Channel Input Select 0
        NSEL: 4..7 = enum NSEL {
            /// A-POOL V- terminal Input Select: Channel 0
            NSEL_0 = 0b0000,
            /// A-POOL V- terminal Input Select: Channel 1
            NSEL_1 = 0b0001,
            /// A-POOL V- terminal Input Select: Channel 2
            NSEL_2 = 0b0010,
            /// A-POOL V- terminal Input Select: Channel 3
            NSEL_3 = 0b0011,
            /// A-POOL V- terminal Input Select: Channel 4
            NSEL_4 = 0b0100,
            /// A-POOL V- terminal Input Select: Channel 5
            NSEL_5 = 0b0101,
            /// A-POOL V- terminal Input Select: Channel 6
            NSEL_6 = 0b0110,
            /// A-POOL V- terminal Input Select: Channel 7
            NSEL_7 = 0b0111,
        }
        /// A-POOL Converteral Input Select: Channel 7
        RUNSTOP: 8 = struct RUNSTOP(bool);
        /// A-POOL Saturation based conversion stop enable
        SBSTP: 9 = struct SBSTP(bool);
        /// A-POOL Comparator based conversion stop enable
        CBSTP: 10 = struct CBSTP(bool);
        /// A-POOL Timer based conversion stop enable for TimerA0
        TBSTP: 11 = struct TBSTP(bool);
        /// A-POOL Pos. Channel Input Select 0
        PSEL: 12..15 = enum PSEL {
            /// A-POOL V+ Terminal Input Select: Channel 0
            PSEL_0 = 0b0000,
            /// A-POOL V+ Terminal Input Select: Channel 1
            PSEL_1 = 0b0001,
            /// A-POOL V+ Terminal Input Select: Channel 2
            PSEL_2 = 0b0010,
            /// A-POOL V+ Terminal Input Select: Channel 3
            PSEL_3 = 0b0011,
            /// A-POOL V+ Terminal Input Select: Channel 4
            PSEL_4 = 0b0100,
            /// A-POOL V+ Terminal Input Select: Channel 5
            PSEL_5 = 0b0101,
            /// A-POOL V+ Terminal Input Select: Channel 6
            PSEL_6 = 0b0110,
            /// A-POOL V+ Terminal Input Select: Channel 7
            PSEL_7 = 0b0111,
            /// A-POOL V+ Terminal Input Select: Channel 8
            PSEL_8 = 0b1000,
        }
    }
    /// A-POOL Operation Mode Register 2
    rw OMR @ 0x04: u16 = 0_0 {
        /// A-POOL Prescaler Control Bit: 0
        CLKDIV: 0..2 = enum CLKDIV {
            /// A-POOL Prescaler Control 0 : /1
            CLKDIV_0 = 0b000,
            /// A-POOL Prescaler Control 1 : /2
            CLKDIV_1 = 0b001,
            /// A-POOL Prescaler Control 2 : /4
            CLKDIV_2 = 0b010,
            /// A-POOL Prescaler Control 3 : /8
            CLKDIV_3 = 0b011,
            /// A-POOL Prescaler Control 4 : /16
            CLKDIV_4 = 0b100,
            /// A-POOL Prescaler Control 5 : /32
            CLKDIV_5 = 0b101,
        }
        /// A-POOL SAR conversion enable
        SAREN: 3 = struct SAREN(bool);
        /// A-POOL Continuous time mode of comparator
        CTEN: 8 = struct CTEN(bool);
        /// A-POOL Clocked zero compensated long term comparison
        AZCMP: 9 = struct AZCMP(bool);
        /// A-POOL SW request for Auto Zero Phase
        AZSWREQ: 10 = struct AZSWREQ(bool);
        /// A-POOL Suppress the generation of an SVM interrupt event.
        SVMINH: 11 = struct SVMINH(bool);
    }
    /// A-POOL Voltage Divider Register 3
    rw VDIV @ 0x06: u16 = 0_0 {
        /// A-POOL Analog channel #0 voltage divider control
        A0DIV: 0 = struct A0DIV(bool);
        /// A-POOL Analog channel #1 voltage divider control
        A1DIV: 1 = struct A1DIV(bool);
        /// A-POOL Analog channel #2 voltage divider control Bit : 0
        A2DIV: 2..3 = enum A2DIV {
            /// A-POOL Analog channel #2 voltage divider control: 0
            A2DIV_0 = 0b00,
            /// A-POOL Analog channel #2 voltage divider control: 1
            A2DIV_1 = 0b01,
            /// A-POOL Analog channel #2 voltage divider control: 2
            A2DIV_2 = 0b10,
            /// A-POOL Analog channel #2 voltage divider control: 3
            A2DIV_3 = 0b11,
        }
        /// A-POOL Analog channel #3 voltage divider control Bit : 0
        A3DIV: 4..5 = enum A3DIV {
            /// A-POOL Analog channel #3 voltage divider control: 0
            A3DIV_0 = 0b00,
            /// A-POOL Analog channel #3 voltage divider control: 1
            A3DIV_1 = 0b01,
            /// A-POOL Analog channel #3 voltage divider control: 2
            A3DIV_2 = 0b10,
            /// A-POOL Analog channel #3 voltage divider control: 3
            A3DIV_3 = 0b11,
        }
        /// A-POOL Temperature sensor enable
        TMPSEN: 6 = struct TMPSEN(bool);
        /// A-POOL VCC voltage divider enable
        VCCDIVEN: 7 = struct VCCDIVEN(bool);
        /// A-POOL Clock trimming Bit : 0
        CLKTRIM: 10..11 = enum CLKTRIM {
            /// A-POOL Clock trimming: 0
            CLKTRIM_0 = 0b00,
            /// A-POOL Clock trimming: 1
            CLKTRIM_1 = 0b01,
            /// A-POOL Clock trimming: 2
            CLKTRIM_2 = 0b10,
            /// A-POOL Clock trimming: 3
            CLKTRIM_3 = 0b11,
        }
    }
    /// A-POOL trimming register
    rw TRIM @ 0x08: u16 = 0_0 {
        /// A-POOL Register bank used for the reference trimming
        REFTSEL: 0 = struct REFTSEL(bool);
        /// A-POOL Reference trimming bit: 0
        REFTRIM: 12..15 = enum REFTRIM {
            /// A-POOL Reference trimming: 0
            REFTRIM_0 = 0b0000,
            /// A-POOL Reference trimming: 1
            REFTRIM_1 = 0b0001,
            /// A-POOL Reference trimming: 2
            REFTRIM_2 = 0b0010,
            /// A-POOL Reference trimming: 3
            REFTRIM_3 = 0b0011,
            /// A-POOL Reference trimming: 4
            REFTRIM_4 = 0b0100,
            /// A-POOL Reference trimming: 5
            REFTRIM_5 = 0b0101,
            /// A-POOL Reference trimming: 6
            REFTRIM_6 = 0b0110,
            /// A-POOL Reference trimming: 7
            REFTRIM_7 = 0b0111,
        }
    }
    /// A-POOL Integer Conversion Register
    rw INT @ 0x10: u16 = 0_0 {
        /// A-POOL Integer Conversion Register
        INT: 0..15 = struct INTField(u16);
    }
    /// A-POOL Integer Conversion Buffer Register
    rw INTB @ 0x12: u16 = 0_0 {
        /// A-POOL Integer Conversion Buffer Register
        INTB: 0..15 = struct INTBField(u16);
    }
    /// A-POOL Fractional Conversion Register
    rw FRACT @ 0x14: u16 = 0_0 {
        /// A-POOL Fractional Conversion Register
        FRACT: 0..15 = struct FRACTField(u16);
    }
    /// A-POOL Fractional Conversion Buffer Register
    rw FRACTB @ 0x16: u16 = 0_0 {
        /// A-POOL Fractional Conversion Buffer Register
        FRACTB: 0..15 = struct FRACTBField(u16);
    }
    /// A-POOL Interrupt Flag Register
    rw IFG @ 0x1a: u16 = 0_0 {
        /// A-POOL End of conversion interrupt flag
        EOCIFG: 0 = struct EOCIFG(bool);
        /// A-POOL Comparator falling edge interrupt flag
        CFIFG: 1 = struct CFIFG(bool);
        /// A-POOL Comparator rising edge interrupt flag
        CRIFG: 2 = struct CRIFG(bool);
        /// A-POOL Reference voltage ready interrupt flag
        REFOKIFG: 3 = struct REFOKIFG(bool);
    }
    /// A-POOL Interrupt Enable Register
    rw IE @ 0x1c: u16 = 0_0 {
        /// A-POOL End of conversion interrupt enable
        EOCIE: 0 = struct EOCIE(bool);
        /// A-POOL Comparator falling edge interrupt enable
        CFIE: 1 = struct CFIE(bool);
        /// A-POOL Comparator rising edge interrupt enable
        CRIE: 2 = struct CRIE(bool);
        /// A-POOL Reference voltage ready interrupt enable
        REFIKIE: 3 = struct REFIKIE(bool);
    }
    /// A-POOL Interrupt Vector Word
    rw IV @ 0x1e: u16 = 0_0 {
        /// A-POOL Interrupt Vector Word
        IV: 0..15 = struct IVField(u16);
    }
}
