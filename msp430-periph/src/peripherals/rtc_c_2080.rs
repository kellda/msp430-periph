//! RTC_C

utils::periph! {
    /// RTC_C
    RTC_C;
    /// RTCCTL0 Register
    rw RTCCTL0 @ 0x00: u16 = 0_0 {
        /// Real-time clock ready interrupt flag
        RTCRDYIFG: 0 = struct RTCRDYIFG(bool);
        /// Real-time clock alarm interrupt flag
        RTCAIFG: 1 = struct RTCAIFG(bool);
        /// Real-time clock time event interrupt flag
        RTCTEVIFG: 2 = struct RTCTEVIFG(bool);
        /// 32-kHz crystal oscillator fault interrupt flag
        RTCOFIFG: 3 = struct RTCOFIFG(bool);
        /// Real-time clock ready interrupt enable
        RTCRDYIE: 4 = struct RTCRDYIE(bool);
        /// Real-time clock alarm interrupt enable
        RTCAIE: 5 = struct RTCAIE(bool);
        /// Real-time clock time event interrupt enable
        RTCTEVIE: 6 = struct RTCTEVIE(bool);
        /// 32-kHz crystal oscillator fault interrupt enable
        RTCOFIE: 7 = struct RTCOFIE(bool);
        /// Real-time clock key
        RTCKEY: 8..15 = struct RTCKEY(u16);
    }
    /// RTCCTL13 Register
    rw RTCCTL13 @ 0x02: u16 = 0_0 {
        /// Real-time clock time event
        RTCTEV: 0..1 = enum RTCTEV {
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
        RTCSSEL: 2..3 = enum RTCSSEL {
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
        RTCRDY: 4 = enum RTCRDY {
            /// RTC time values in transition
            RTCRDY_0 = 0b0,
            /// RTC time values safe for reading. This bit indicates when the real-time clock time values are safe for reading.
            RTCRDY_1 = 0b1,
        }
        /// RTCMODE
        RTCMODE: 5 = enum RTCMODE {
            /// Calendar mode. Always reads a value of 1.
            RTCMODE_1 = 0b1,
        }
        /// Real-time clock hold
        RTCHOLD: 6 = enum RTCHOLD {
            /// Real-time clock is operational
            RTCHOLD_0 = 0b0,
            /// When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care
            RTCHOLD_1 = 0b1,
        }
        /// Real-time clock BCD select
        RTCBCD: 7 = enum RTCBCD {
            /// Binary (hexadecimal) code selected
            HEX = 0b0,
            /// Binary coded decimal (BCD) code selected
            BCD = 0b1,
        }
        /// Real-time clock calibration frequency
        RTCCALF: 8..9 = enum RTCCALF {
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
    /// RTCOCAL Register
    rw RTCOCAL @ 0x04: u16 = 0_0 {
        /// Real-time clock offset error calibration
        RTCOCAL: 0..7 = struct RTCOCALField(u16);
        /// Real-time clock offset error calibration sign
        RTCOCALS: 15 = enum RTCOCALS {
            /// Down calibration. Frequency adjusted down.
            DOWN = 0b0,
            /// Up calibration. Frequency adjusted up.
            UP = 0b1,
        }
    }
    /// RTCTCMP Register
    rw RTCTCMP @ 0x06: u16 = 0_0 {
        /// Real-time clock temperature compensation
        RTCTCMP: 0..7 = struct RTCTCMPField(u16);
        /// Real-time clock temperature compensation write OK
        RTCTCOK: 13 = enum RTCTCOK {
            /// Write to RTCTCMPx is unsuccessful
            RTCTCOK_0 = 0b0,
            /// Write to RTCTCMPx is successful
            RTCTCOK_1 = 0b1,
        }
        /// Real-time clock temperature compensation ready
        RTCTCRDY: 14 = struct RTCTCRDY(bool);
        /// Real-time clock temperature compensation sign
        RTCTCMPS: 15 = enum RTCTCMPS {
            /// Down calibration. Frequency adjusted down
            DOWN = 0b0,
            /// Up calibration. Frequency adjusted up
            UP = 0b1,
        }
    }
    /// Real-Time Clock Prescale Timer 0 Control Register
    rw RTCPS0CTL @ 0x08: u16 = 0_0 {
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
    rw RTCPS1CTL @ 0x0a: u16 = 0_0 {
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
    rw RTCPS @ 0x0c: u16 = 0_0 {
        /// Real-Time Clock Prescale Timer Counter Register
        RTCPS: 0..15 = struct RTCPSField(u16);
    }
    /// Real-Time Clock Interrupt Vector Register
    r RTCIV @ 0x0e: u16 = 0_0 {
        /// Real-time clock interrupt vector value
        RTCIV: 0..15 = enum RTCIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest
            RTCOFIFG = 0b0000000000000010,
            /// Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG
            RTCRDYIFG = 0b0000000000000100,
            /// Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG
            RTCTEVIFG = 0b0000000000000110,
            /// Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG
            RTCAIFG = 0b0000000000001000,
            /// Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG
            RT0PSIFG = 0b0000000000001010,
            /// Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG
            RT1PSIFG = 0b0000000000001100,
        }
    }
    /// RTCTIM0 Register  Hexadecimal Format
    rw RTCTIM0 @ 0x10: u16 = 0_0 {
        /// Seconds (0 to 59)
        SECONDS: 0..5 = struct SECONDS(u16);
        /// Minutes (0 to 59)
        MINUTES: 8..13 = struct MINUTES(u16);
    }
    /// Real-Time Clock Seconds, Minutes Register - BCD Format
    rw RTCTIM0_BCD @ 0x10: u16 = 0_0 {
        /// Seconds  low digit (0 to 9)
        SECONDSLOWDIGIT: 0..3 = struct SECONDSLOWDIGIT(u16);
        /// Seconds  high digit (0 to 5)
        SECONDSHIGHDIGIT: 4..6 = struct SECONDSHIGHDIGIT(u16);
        /// Minutes  low digit (0 to 9)
        RTCTIM0_BCD_MINUTESLOWDIGIT: 8..11 = struct RTCTIM0_BCD_MINUTESLOWDIGIT(u16);
        /// Minutes  high digit (0 to 5)
        RTCTIM0_BCD_MINUTESHIGHDIGIT: 12..14 = struct RTCTIM0_BCD_MINUTESHIGHDIGIT(u16);
    }
    /// Real-Time Clock Counter 1 and 2  Register  Counter Mode
    rw RTCCNT12 @ 0x10: u16 = 0_0 {
        /// Real-Time Clock Counter 1 and 2  Register  Counter Mode
        RTCCNT12: 0..15 = struct RTCCNT12Field(u16);
    }
    /// Real-Time Clock Hour, Day of Week
    rw RTCTIM1 @ 0x12: u16 = 0_0 {
        /// Hours (0 to 23)
        HOURS: 0..4 = struct HOURS(u16);
        /// Day of week (0 to 6)
        RTCTIM1_DAYOFWEEK: 8..10 = struct RTCTIM1_DAYOFWEEK(u16);
    }
    /// Real-Time Clock Hour, Day of Week - BCD Format
    rw RTCTIM1_BCD @ 0x12: u16 = 0_0 {
        /// Hours  low digit (0 to 9)
        RTCTIM1_BCD_HOURSLOWDIGIT: 0..3 = struct RTCTIM1_BCD_HOURSLOWDIGIT(u16);
        /// Hours  high digit (0 to 2)
        RTCTIM1_BCD_HOURSHIGHDIGIT: 4..5 = struct RTCTIM1_BCD_HOURSHIGHDIGIT(u16);
        /// Day of week (0 to 6)
        RTCTIM1_BCD_DAYOFWEEK: 8..10 = struct RTCTIM1_BCD_DAYOFWEEK(u16);
    }
    /// Real-Time Clock Counter 3 and 4  Register  Counter Mode
    rw RTCCNT34 @ 0x12: u16 = 0_0 {
        /// Real-Time Clock Counter 3 and 4  Register  Counter Mode
        RTCCNT34: 0..15 = struct RTCCNT34Field(u16);
    }
    /// RTCDATE - Hexadecimal Format
    rw RTCDATE @ 0x14: u16 = 0_0 {
        /// Day of month (1 to 28, 29, 30, 31)
        RTCDATE_DAY: 0..4 = struct RTCDATE_DAY(u16);
        /// Month (1 to 12)
        MONTH: 8..11 = struct MONTH(u16);
    }
    /// Real-Time Clock Date - BCD Format
    rw RTCDATE_BCD @ 0x14: u16 = 0_0 {
        /// Day of month  low digit (0 to 9)
        DAYLOWDIGIT: 0..3 = struct DAYLOWDIGIT(u16);
        /// Day of month  high digit (0 to 3)
        DAYHIGHDIGIT: 4..5 = struct DAYHIGHDIGIT(u16);
        /// Month  low digit (0 to 9)
        MONTHLOWDIGIT: 8..11 = struct MONTHLOWDIGIT(u16);
        /// Month  high digit (0 or 1)
        MONTHHIGHDIGIT: 12 = struct MONTHHIGHDIGIT(bool);
    }
    /// RTCYEAR Register  Hexadecimal Format
    rw RTCYEAR @ 0x16: u16 = 0_0 {
        /// Year  low byte. Valid values for Year are 0 to 4095.
        YEARLOWBYTE: 0..7 = struct YEARLOWBYTE(u16);
        /// Year  high byte. Valid values for Year are 0 to 4095.
        YEARHIGHBYTE: 8..11 = struct YEARHIGHBYTE(u16);
    }
    /// Real-Time Clock Year Register - BCD Format
    rw RTCYEAR_BCD @ 0x16: u16 = 0_0 {
        /// Year  lowest digit (0 to 9)
        YEAR: 0..3 = struct YEAR(u16);
        /// Decade (0 to 9)
        DECADE: 4..7 = struct DECADE(u16);
        /// Century  low digit (0 to 9)
        CENTURYLOWDIGIT: 8..11 = struct CENTURYLOWDIGIT(u16);
        /// Century  high digit (0 to 4)
        CENTURYHIGHDIGIT: 12..14 = struct CENTURYHIGHDIGIT(u16);
    }
    /// RTCMINHR - Hexadecimal Format
    rw RTCAMINHR @ 0x18: u16 = 0_0 {
        /// Minutes (0 to 59)
        MIN: 0..5 = struct MIN(u16);
        /// Alarm enable
        RTCAMINHR_MINAE: 7 = struct RTCAMINHR_MINAE(bool);
        /// Hours (0 to 23)
        HOUR: 8..12 = struct HOUR(u16);
        /// Alarm enable
        RTCAMINHR_HOURAE: 15 = struct RTCAMINHR_HOURAE(bool);
    }
    /// Real-Time Clock Minutes, Hour Alarm - BCD Format
    rw RTCAMINHR_BCD @ 0x18: u16 = 0_0 {
        /// Minutes  low digit (0 to 9)
        RTCAMINHR_BCD_MINUTESLOWDIGIT: 0..3 = struct RTCAMINHR_BCD_MINUTESLOWDIGIT(u16);
        /// Minutes  high digit (0 to 5)
        RTCAMINHR_BCD_MINUTESHIGHDIGIT: 4..6 = struct RTCAMINHR_BCD_MINUTESHIGHDIGIT(u16);
        /// Alarm enable
        RTCAMINHR_BCD_MINAE: 7 = struct RTCAMINHR_BCD_MINAE(bool);
        /// Hours  low digit (0 to 9)
        RTCAMINHR_BCD_HOURSLOWDIGIT: 8..11 = struct RTCAMINHR_BCD_HOURSLOWDIGIT(u16);
        /// Hours  high digit (0 to 2)
        RTCAMINHR_BCD_HOURSHIGHDIGIT: 12..13 = struct RTCAMINHR_BCD_HOURSHIGHDIGIT(u16);
        /// Alarm enable
        RTCAMINHR_BCD_HOURAE: 15 = struct RTCAMINHR_BCD_HOURAE(bool);
    }
    /// RTCADOWDAY - Hexadecimal Format
    rw RTCADOWDAY @ 0x1a: u16 = 0_0 {
        /// Day of week (0 to 6)
        RTCADOWDAY_DOW: 0..2 = struct RTCADOWDAY_DOW(u16);
        /// Alarm enable
        RTCADOWDAY_DOWAE: 7 = struct RTCADOWDAY_DOWAE(bool);
        /// Day of month (1 to 28, 29, 30, 31)
        RTCADOWDAY_DAY: 8..12 = struct RTCADOWDAY_DAY(u16);
        /// Alarm enable
        RTCADOWDAY_DAYAE: 15 = struct RTCADOWDAY_DAYAE(bool);
    }
    /// Real-Time Clock Day of Week, Day of Month Alarm - BCD Format
    rw RTCADOWDAY_BCD @ 0x1a: u16 = 0_0 {
        /// Day of week (0 to 6)
        RTCADOWDAY_BCD_DOW: 0..2 = struct RTCADOWDAY_BCD_DOW(u16);
        /// Alarm enable
        RTCADOWDAY_BCD_DOWAE: 7 = struct RTCADOWDAY_BCD_DOWAE(bool);
        /// Day of month  low digit (0 to 9)
        DAY_LD: 8..11 = struct DAY_LD(u16);
        /// Day of month  high digit (0 to 3)
        DAY_HD: 12..13 = struct DAY_HD(u16);
        /// Alarm enable
        RTCADOWDAY_BCD_DAYAE: 15 = struct RTCADOWDAY_BCD_DAYAE(bool);
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
    rw RTCCNT1 @ 0x10: u8 = 0_0 {
        /// The RTCCNT1 register is the count of RTCCNT1
        RTCCNT1: 0..7 = struct RTCCNT1Field(u8);
    }
    /// The RTCCNT2 register is the count of RTCCNT2
    rw RTCCNT2 @ 0x11: u8 = 0_0 {
        /// The RTCCNT2 register is the count of RTCCNT2
        RTCCNT2: 0..7 = struct RTCCNT2Field(u8);
    }
    /// The RTCCNT3 register is the count of RTCCNT3
    rw RTCCNT3 @ 0x12: u8 = 0_0 {
        /// The RTCCNT3 register is the count of RTCCNT3
        RTCCNT3: 0..7 = struct RTCCNT3Field(u8);
    }
    /// The RTCCNT4 register is the count of RTCCNT4
    rw RTCCNT4 @ 0x13: u8 = 0_0 {
        /// The RTCCNT4 register is the count of RTCCNT4
        RTCCNT4: 0..7 = struct RTCCNT4Field(u8);
    }
}
