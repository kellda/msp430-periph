//! RTC_C

utils::periph! {
    /// RTC_C
    RTC_C;
    /// RTCCTL0 Register
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// Real-time clock ready interrupt flag
        RDYIFG: 0 = struct RDYIFG(bool);
        /// Real-time clock alarm interrupt flag
        AIFG: 1 = struct AIFG(bool);
        /// Real-time clock time event interrupt flag
        TEVIFG: 2 = struct TEVIFG(bool);
        /// 32-kHz crystal oscillator fault interrupt flag
        OFIFG: 3 = struct OFIFG(bool);
        /// Real-time clock ready interrupt enable
        RDYIE: 4 = struct RDYIE(bool);
        /// Real-time clock alarm interrupt enable
        AIE: 5 = struct AIE(bool);
        /// Real-time clock time event interrupt enable
        TEVIE: 6 = struct TEVIE(bool);
        /// 32-kHz crystal oscillator fault interrupt enable
        OFIE: 7 = struct OFIE(bool);
        /// Real-time clock key
        KEY: 8..15 = struct KEY(u16);
    }
    /// RTCCTL13 Register
    rw CTL13 @ 0x02: u16 = 0_0 {
        /// Real-time clock time event
        TEV: 0..1 = enum TEV {
            /// Minute changed
            MIN = 0b00,
            /// Hour changed
            HOUR = 0b01,
            /// Every day at midnight (00:00)
            _0000 = 0b10,
            /// Every day at noon (12:00)
            _1200 = 0b11,
        }
        /// Real-time clock source select
        SSEL: 2..3 = enum SSEL {
            /// 32-kHz crystal oscillator clock
            LFXT = 0b00,
            /// 32-kHz crystal oscillator clock
            LFXT1 = 0b01,
            /// Output from RT1PS
            RT1PS = 0b10,
            /// Output from RT1PS
            RT1PS1 = 0b11,
        }
        /// Real-time clock ready
        RDY: 4 = enum RDY {
            /// RTC time values in transition
            RDY_0 = 0b0,
            /// RTC time values safe for reading. This bit indicates when the real-time clock time values are safe for reading.
            RDY_1 = 0b1,
        }
        /// RTCMODE
        MODE: 5 = enum MODE {
            /// Calendar mode. Always reads a value of 1.
            MODE_1 = 0b1,
        }
        /// Real-time clock hold
        HOLD: 6 = enum HOLD {
            /// Real-time clock is operational
            HOLD_0 = 0b0,
            /// When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care
            HOLD_1 = 0b1,
        }
        /// Real-time clock BCD select
        BCD: 7 = enum BCD {
            /// Binary (hexadecimal) code selected
            HEX = 0b0,
            /// Binary coded decimal (BCD) code selected
            BCD = 0b1,
        }
        /// Real-time clock calibration frequency
        CALF: 8..9 = enum CALF {
            /// No frequency output to RTCCLK pin
            NONE = 0b00,
            /// 512 Hz
            _512 = 0b01,
            /// 256 Hz
            _256 = 0b10,
            /// 1 Hz
            _1 = 0b11,
        }
    }
    /// OCAL Register
    rw OCAL @ 0x04: u16 = 0_0 {
        /// Real-time clock offset error calibration
        OCAL: 0..7 = struct OCALField(u16);
        /// Real-time clock offset error calibration sign
        OCALS: 15 = enum OCALS {
            /// Down calibration. Frequency adjusted down.
            DOWN = 0b0,
            /// Up calibration. Frequency adjusted up.
            UP = 0b1,
        }
    }
    /// RTCTCMP Register
    rw TCMP @ 0x06: u16 = 0_0 {
        /// Real-time clock temperature compensation
        TCMP: 0..7 = struct TCMPField(u16);
        /// Real-time clock temperature compensation write OK
        TCOK: 13 = enum TCOK {
            /// Write to RTCTCMPx is unsuccessful
            TCOK_0 = 0b0,
            /// Write to RTCTCMPx is successful
            TCOK_1 = 0b1,
        }
        /// Real-time clock temperature compensation ready
        TCRDY: 14 = struct TCRDY(bool);
        /// Real-time clock temperature compensation sign
        TCMPS: 15 = enum TCMPS {
            /// Down calibration. Frequency adjusted down
            DOWN = 0b0,
            /// Up calibration. Frequency adjusted up
            UP = 0b1,
        }
    }
    /// Real-Time Clock Prescale Timer 0 Control Register
    rw PS0CTL @ 0x08: u16 = 0_0 {
        /// Prescale timer 0 interrupt flag
        RT0PSIFG: 0 = enum RT0PSIFG {
            /// No time event occurred
            RT0PSIFG_0 = 0b0,
            /// Time event occurred
            RT0PSIFG_1 = 0b1,
        }
        /// Prescale timer 0 interrupt enable
        RT0PSIE: 1 = enum RT0PSIE {
            /// Interrupt not enabled
            DISABLE = 0b0,
            /// Interrupt enabled
            ENABLE = 0b1,
        }
        /// Prescale timer 0 interrupt interval
        RT0IP: 2..4 = enum RT0IP {
            /// Divide by 2
            _2 = 0b000,
            /// Divide by 4
            _4 = 0b001,
            /// Divide by 8
            _8 = 0b010,
            /// Divide by 16
            _16 = 0b011,
            /// Divide by 32
            _32 = 0b100,
            /// Divide by 64
            _64 = 0b101,
            /// Divide by 128
            _128 = 0b110,
            /// Divide by 256
            _256 = 0b111,
        }
        /// Prescale timer 0 hold
        RT0PSHOLD: 8 = enum RT0PSHOLD {
            /// RT0PS is operational
            RT0PSHOLD_0 = 0b0,
            /// RT0PS is held
            RT0PSHOLD_1 = 0b1,
        }
        /// Prescale timer 0 clock divide
        RT0PSDIV: 11..13 = enum RT0PSDIV {
            /// Divide by 2
            _2 = 0b000,
            /// Divide by 4
            _4 = 0b001,
            /// Divide by 8
            _8 = 0b010,
            /// Divide by 16
            _16 = 0b011,
            /// Divide by 32
            _32 = 0b100,
            /// Divide by 64
            _64 = 0b101,
            /// Divide by 128
            _128 = 0b110,
            /// Divide by 256
            _256 = 0b111,
        }
    }
    /// Real-Time Clock Prescale Timer 1 Control Register
    rw PS1CTL @ 0x0a: u16 = 0_0 {
        /// Prescale timer 1 interrupt flag
        RT1PSIFG: 0 = enum RT1PSIFG {
            /// No time event occurred
            RT1PSIFG_0 = 0b0,
            /// Time event occurred
            RT1PSIFG_1 = 0b1,
        }
        /// Prescale timer 1 interrupt enable
        RT1PSIE: 1 = enum RT1PSIE {
            /// Interrupt not enabled
            DISABLE = 0b0,
            /// Interrupt enabled (LPM3/LPM3.5 wake-up enabled)
            ENABLE = 0b1,
        }
        /// Prescale timer 1 interrupt interval
        RT1IP: 2..4 = enum RT1IP {
            /// Divide by 2
            _2 = 0b000,
            /// Divide by 4
            _4 = 0b001,
            /// Divide by 8
            _8 = 0b010,
            /// Divide by 16
            _16 = 0b011,
            /// Divide by 32
            _32 = 0b100,
            /// Divide by 64
            _64 = 0b101,
            /// Divide by 128
            _128 = 0b110,
            /// Divide by 256
            _256 = 0b111,
        }
        /// Prescale timer 1 hold
        RT1PSHOLD: 8 = enum RT1PSHOLD {
            /// RT1PS is operational
            RT1PSHOLD_0 = 0b0,
            /// RT1PS is held
            RT1PSHOLD_1 = 0b1,
        }
        /// Prescale timer 1 clock divide
        RT1PSDIV: 11..13 = enum RT1PSDIV {
            /// Divide by 2
            _2 = 0b000,
            /// Divide by 4
            _4 = 0b001,
            /// Divide by 8
            _8 = 0b010,
            /// Divide by 16
            _16 = 0b011,
            /// Divide by 32
            _32 = 0b100,
            /// Divide by 64
            _64 = 0b101,
            /// Divide by 128
            _128 = 0b110,
            /// Divide by 256
            _256 = 0b111,
        }
        /// Prescale timer 1 clock source select
        RT1SSEL: 14..15 = enum RT1SSEL {
            /// 32-kHz crystal oscillator clock
            RT1SSEL_0 = 0b00,
            /// 32-kHz crystal oscillator clock
            RT1SSEL_1 = 0b01,
            /// Output from RT0PS
            RT0PS = 0b10,
            /// Output from RT0PS
            RT0PS1 = 0b11,
        }
    }
    /// Real-Time Clock Prescale Timer Counter Register
    rw PS @ 0x0c: u16 = 0_0 {
        /// Real-Time Clock Prescale Timer Counter Register
        PS: 0..15 = struct PSField(u16);
    }
    /// Real-Time Clock Interrupt Vector Register
    r IV @ 0x0e: u16 = 0_0 {
        /// Real-time clock interrupt vector value
        IV: 0..15 = enum IVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest
            OFIFG = 0b0000000000000010,
            /// Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG
            RDYIFG = 0b0000000000000100,
            /// Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG
            TEVIFG = 0b0000000000000110,
            /// Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG
            AIFG = 0b0000000000001000,
            /// Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG
            RT0PSIFG = 0b0000000000001010,
            /// Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG
            RT1PSIFG = 0b0000000000001100,
        }
    }
    /// RTCTIM0 Register  Hexadecimal Format
    rw TIM0 @ 0x10: u16 = 0_0 {
        /// Seconds (0 to 59)
        SECONDS: 0..5 = struct SECONDS(u16);
        /// Minutes (0 to 59)
        MINUTES: 8..13 = struct MINUTES(u16);
    }
    /// Real-Time Clock Seconds, Minutes Register - BCD Format
    rw TIM0_BCD @ 0x10: u16 = 0_0 {
        /// Seconds  low digit (0 to 9)
        SECONDSLOWDIGIT: 0..3 = struct SECONDSLOWDIGIT(u16);
        /// Seconds  high digit (0 to 5)
        SECONDSHIGHDIGIT: 4..6 = struct SECONDSHIGHDIGIT(u16);
        /// Minutes  low digit (0 to 9)
        TIM0_BCD_MINUTESLOWDIGIT: 8..11 = struct TIM0_BCD_MINUTESLOWDIGIT(u16);
        /// Minutes  high digit (0 to 5)
        TIM0_BCD_MINUTESHIGHDIGIT: 12..14 = struct TIM0_BCD_MINUTESHIGHDIGIT(u16);
    }
    /// Real-Time Clock Counter 1 and 2  Register  Counter Mode
    rw CNT12 @ 0x10: u16 = 0_0 {
        /// Real-Time Clock Counter 1 and 2  Register  Counter Mode
        CNT12: 0..15 = struct CNT12Field(u16);
    }
    /// Real-Time Clock Hour, Day of Week
    rw TIM1 @ 0x12: u16 = 0_0 {
        /// Hours (0 to 23)
        HOURS: 0..4 = struct HOURS(u16);
        /// Day of week (0 to 6)
        TIM1_DAYOFWEEK: 8..10 = struct TIM1_DAYOFWEEK(u16);
    }
    /// Real-Time Clock Hour, Day of Week - BCD Format
    rw TIM1_BCD @ 0x12: u16 = 0_0 {
        /// Hours  low digit (0 to 9)
        TIM1_BCD_HOURSLOWDIGIT: 0..3 = struct TIM1_BCD_HOURSLOWDIGIT(u16);
        /// Hours  high digit (0 to 2)
        TIM1_BCD_HOURSHIGHDIGIT: 4..5 = struct TIM1_BCD_HOURSHIGHDIGIT(u16);
        /// Day of week (0 to 6)
        TIM1_BCD_DAYOFWEEK: 8..10 = struct TIM1_BCD_DAYOFWEEK(u16);
    }
    /// Real-Time Clock Counter 3 and 4  Register  Counter Mode
    rw CNT34 @ 0x12: u16 = 0_0 {
        /// Real-Time Clock Counter 3 and 4  Register  Counter Mode
        CNT34: 0..15 = struct CNT34Field(u16);
    }
    /// RTCDATE - Hexadecimal Format
    rw DATE @ 0x14: u16 = 0_0 {
        /// Day of month (1 to 28, 29, 30, 31)
        DATE_DAY: 0..4 = struct DATE_DAY(u16);
        /// Month (1 to 12)
        MONTH: 8..11 = struct MONTH(u16);
    }
    /// Real-Time Clock Date - BCD Format
    rw DATE_BCD @ 0x14: u16 = 0_0 {
        /// Day of month  low digit (0 to 9)
        DAYLOWDIGIT: 0..3 = struct DAYLOWDIGIT(u16);
        /// Day of month  high digit (0 to 3)
        DAYHIGHDIGIT: 4..5 = struct DAYHIGHDIGIT(u16);
        /// Month  low digit (0 to 9)
        MONTHLOWDIGIT: 8..11 = struct MONTHLOWDIGIT(u16);
        /// Month  high digit (0 or 1)
        MONTHHIGHDIGIT: 12 = struct MONTHHIGHDIGIT(bool);
    }
    /// YEAR Register  Hexadecimal Format
    rw YEAR @ 0x16: u16 = 0_0 {
        /// Year  low byte. Valid values for Year are 0 to 4095.
        YEARLOWBYTE: 0..7 = struct YEARLOWBYTE(u16);
        /// Year  high byte. Valid values for Year are 0 to 4095.
        YEARHIGHBYTE: 8..11 = struct YEARHIGHBYTE(u16);
    }
    /// Real-Time Clock Year Register - BCD Format
    rw YEAR_BCD @ 0x16: u16 = 0_0 {
        /// Year  lowest digit (0 to 9)
        YEAR: 0..3 = struct YEARField(u16);
        /// Decade (0 to 9)
        DECADE: 4..7 = struct DECADE(u16);
        /// Century  low digit (0 to 9)
        CENTURYLOWDIGIT: 8..11 = struct CENTURYLOWDIGIT(u16);
        /// Century  high digit (0 to 4)
        CENTURYHIGHDIGIT: 12..14 = struct CENTURYHIGHDIGIT(u16);
    }
    /// MINHR - Hexadecimal Format
    rw AMINHR @ 0x18: u16 = 0_0 {
        /// Minutes (0 to 59)
        MIN: 0..5 = struct MIN(u16);
        /// Alarm enable
        AMINHR_MINAE: 7 = struct AMINHR_MINAE(bool);
        /// Hours (0 to 23)
        HOUR: 8..12 = struct HOUR(u16);
        /// Alarm enable
        AMINHR_HOURAE: 15 = struct AMINHR_HOURAE(bool);
    }
    /// Real-Time Clock Minutes, Hour Alarm - BCD Format
    rw AMINHR_BCD @ 0x18: u16 = 0_0 {
        /// Minutes  low digit (0 to 9)
        AMINHR_BCD_MINUTESLOWDIGIT: 0..3 = struct AMINHR_BCD_MINUTESLOWDIGIT(u16);
        /// Minutes  high digit (0 to 5)
        AMINHR_BCD_MINUTESHIGHDIGIT: 4..6 = struct AMINHR_BCD_MINUTESHIGHDIGIT(u16);
        /// Alarm enable
        AMINHR_BCD_MINAE: 7 = struct AMINHR_BCD_MINAE(bool);
        /// Hours  low digit (0 to 9)
        AMINHR_BCD_HOURSLOWDIGIT: 8..11 = struct AMINHR_BCD_HOURSLOWDIGIT(u16);
        /// Hours  high digit (0 to 2)
        AMINHR_BCD_HOURSHIGHDIGIT: 12..13 = struct AMINHR_BCD_HOURSHIGHDIGIT(u16);
        /// Alarm enable
        AMINHR_BCD_HOURAE: 15 = struct AMINHR_BCD_HOURAE(bool);
    }
    /// ADOWDAY - Hexadecimal Format
    rw ADOWDAY @ 0x1a: u16 = 0_0 {
        /// Day of week (0 to 6)
        ADOWDAY_DOW: 0..2 = struct ADOWDAY_DOW(u16);
        /// Alarm enable
        ADOWDAY_DOWAE: 7 = struct ADOWDAY_DOWAE(bool);
        /// Day of month (1 to 28, 29, 30, 31)
        ADOWDAY_DAY: 8..12 = struct ADOWDAY_DAY(u16);
        /// Alarm enable
        ADOWDAY_DAYAE: 15 = struct ADOWDAY_DAYAE(bool);
    }
    /// Real-Time Clock Day of Week, Day of Month Alarm - BCD Format
    rw ADOWDAY_BCD @ 0x1a: u16 = 0_0 {
        /// Day of week (0 to 6)
        ADOWDAY_BCD_DOW: 0..2 = struct ADOWDAY_BCD_DOW(u16);
        /// Alarm enable
        ADOWDAY_BCD_DOWAE: 7 = struct ADOWDAY_BCD_DOWAE(bool);
        /// Day of month  low digit (0 to 9)
        DAY_LD: 8..11 = struct DAY_LD(u16);
        /// Day of month  high digit (0 to 3)
        DAY_HD: 12..13 = struct DAY_HD(u16);
        /// Alarm enable
        ADOWDAY_BCD_DAYAE: 15 = struct ADOWDAY_BCD_DAYAE(bool);
    }
    /// Binary-to-BCD Conversion Register
    rw BIN2BCD @ 0x1c: u16 = 0_0 {
        /// Binary-to-BCD Conversion Register
        BIN2BCD: 0..15 = struct BIN2BCDField(u16);
    }
    /// BCD-to-Binary Conversion Register
    rw BCD2BIN @ 0x1e: u16 = 0_0 {
        /// BCD-to-Binary Conversion Register
        BCD2BIN: 0..15 = struct BCD2BINField(u16);
    }
    /// Prescale timer 0 counter value
    rw RT0PS @ 0x0c: u8 = 0_0 {
        /// Prescale timer 0 counter value
        RT0PS: 0..7 = struct RT0PSField(u8);
    }
    /// Prescale timer 1 counter value
    rw RT1PS @ 0x0d: u8 = 0_0 {
        /// Prescale timer 1 counter value
        RT1PS: 0..7 = struct RT1PSField(u8);
    }
    /// The RTCCNT1 register is the count of RTCCNT1
    rw CNT1 @ 0x10: u8 = 0_0 {
        /// The RTCCNT1 register is the count of RTCCNT1
        CNT1: 0..7 = struct CNT1Field(u8);
    }
    /// The RTCCNT2 register is the count of RTCCNT2
    rw CNT2 @ 0x11: u8 = 0_0 {
        /// The RTCCNT2 register is the count of RTCCNT2
        CNT2: 0..7 = struct CNT2Field(u8);
    }
    /// The RTCCNT3 register is the count of RTCCNT3
    rw CNT3 @ 0x12: u8 = 0_0 {
        /// The RTCCNT3 register is the count of RTCCNT3
        CNT3: 0..7 = struct CNT3Field(u8);
    }
    /// The RTCCNT4 register is the count of RTCCNT4
    rw CNT4 @ 0x13: u8 = 0_0 {
        /// The RTCCNT4 register is the count of RTCCNT4
        CNT4: 0..7 = struct CNT4Field(u8);
    }
}
