//! RTC_D  Real Time Clock

utils::periph! {
    /// RTC_D  Real Time Clock
    RTC_D;
    /// Real Timer Control 0/1
    rw RTCCTL01 @ 0x00: u16 = 0_0 {
        /// RTC Ready Interrupt Flag
        RTCRDYIFG: 0 = struct RTCRDYIFG(bool);
        /// RTC Alarm Interrupt Flag
        RTCAIFG: 1 = struct RTCAIFG(bool);
        /// RTC Time Event Interrupt Flag
        RTCTEVIFG: 2 = struct RTCTEVIFG(bool);
        /// RTC 32kHz cyrstal oscillator fault interrupt flag
        RTCOFIFG: 3 = struct RTCOFIFG(bool);
        /// RTC Ready Interrupt Enable Flag
        RTCRDYIE: 4 = struct RTCRDYIE(bool);
        /// RTC Alarm Interrupt Enable Flag
        RTCAIE: 5 = struct RTCAIE(bool);
        /// RTC Time Event Interrupt Enable Flag
        RTCTEVIE: 6 = struct RTCTEVIE(bool);
        /// RTC 32kHz cyrstal oscillator fault interrupt enable
        RTCOFIE: 7 = struct RTCOFIE(bool);
        /// RTC Time Event 1
        RTCTEV: 8..9 = enum RTCTEV {
            /// RTC Time Event: 0 (Min. changed)
            RTCTEV_0 = 0b00,
            /// RTC Time Event: 1 (Hour changed)
            RTCTEV_1 = 0b01,
            /// RTC Time Event: 2 (12:00 changed)
            RTCTEV_2 = 0b10,
            /// RTC Time Event: 3 (00:00 changed)
            RTCTEV_3 = 0b11,
        }
        /// RTC Source Select 1
        RTCSSEL: 10..11 = enum RTCSSEL {
            /// RTC Source Select ACLK
            RTCSSEL_0 = 0b00,
            /// RTC Source Select SMCLK
            RTCSSEL_1 = 0b01,
            /// RTC Source Select RT1PS
            RTCSSEL_2 = 0b10,
            /// RTC Source Select RT1PS
            RTCSSEL_3 = 0b11,
        }
        /// RTC Ready
        RTCRDY: 12 = struct RTCRDY(bool);
        /// RTC Mode 0:Counter / 1: Calendar
        RTCMODE: 13 = struct RTCMODE(bool);
        /// RTC Hold
        RTCHOLD: 14 = struct RTCHOLD(bool);
        /// RTC BCD  0:Binary / 1:BCD
        RTCBCD: 15 = struct RTCBCD(bool);
    }
    /// Real Timer Control 2/3
    rw RTCCTL23 @ 0x02: u16 = 0_0 {
        /// RTC Calibration Bit 0
        RTCCAL0: 0 = struct RTCCAL0(bool);
        /// RTC Calibration Bit 1
        RTCCAL1: 1 = struct RTCCAL1(bool);
        /// RTC Calibration Bit 2
        RTCCAL2: 2 = struct RTCCAL2(bool);
        /// RTC Calibration Bit 3
        RTCCAL3: 3 = struct RTCCAL3(bool);
        /// RTC Calibration Bit 4
        RTCCAL4: 4 = struct RTCCAL4(bool);
        /// RTC Calibration Bit 5
        RTCCAL5: 5 = struct RTCCAL5(bool);
        /// RTC Calibration Sign
        RTCCALS: 7 = struct RTCCALS(bool);
        /// RTC Calibration Frequency Bit 1
        RTCCALF: 8..9 = enum RTCCALF {
            /// RTC Calibration Frequency: No Output
            RTCCALF_0 = 0b00,
            /// RTC Calibration Frequency: 512 Hz
            RTCCALF_1 = 0b01,
            /// RTC Calibration Frequency: 256 Hz
            RTCCALF_2 = 0b10,
            /// RTC Calibration Frequency: 1 Hz
            RTCCALF_3 = 0b11,
        }
    }
    /// Real Timer Prescale Timer 0 Control
    rw RTCPS0CTL @ 0x08: u16 = 0_0 {
        /// RTC Prescale Timer 0 Interrupt Flag
        RT0PSIFG: 0 = struct RT0PSIFG(bool);
        /// RTC Prescale Timer 0 Interrupt Enable Flag
        RT0PSIE: 1 = struct RT0PSIE(bool);
        /// RTC Prescale Timer 0 Interrupt Interval Bit: 2
        RT0IP: 2..4 = enum RT0IP {
            /// RTC Prescale Timer 0 Interrupt Interval /2
            RT0IP_0 = 0b000,
            /// RTC Prescale Timer 0 Interrupt Interval /4
            RT0IP_1 = 0b001,
            /// RTC Prescale Timer 0 Interrupt Interval /8
            RT0IP_2 = 0b010,
            /// RTC Prescale Timer 0 Interrupt Interval /16
            RT0IP_3 = 0b011,
            /// RTC Prescale Timer 0 Interrupt Interval /32
            RT0IP_4 = 0b100,
            /// RTC Prescale Timer 0 Interrupt Interval /64
            RT0IP_5 = 0b101,
            /// RTC Prescale Timer 0 Interrupt Interval /128
            RT0IP_6 = 0b110,
            /// RTC Prescale Timer 0 Interrupt Interval /256
            RT0IP_7 = 0b111,
        }
        /// RTC Prescale Timer 0 Hold
        RT0PSHOLD: 8 = struct RT0PSHOLD(bool);
        /// RTC Prescale Timer 0 Clock Divide Bit: 2
        RT0PSDIV: 11..13 = enum RT0PSDIV {
            /// RTC Prescale Timer 0 Clock Divide /2
            RT0PSDIV_0 = 0b000,
            /// RTC Prescale Timer 0 Clock Divide /4
            RT0PSDIV_1 = 0b001,
            /// RTC Prescale Timer 0 Clock Divide /8
            RT0PSDIV_2 = 0b010,
            /// RTC Prescale Timer 0 Clock Divide /16
            RT0PSDIV_3 = 0b011,
            /// RTC Prescale Timer 0 Clock Divide /32
            RT0PSDIV_4 = 0b100,
            /// RTC Prescale Timer 0 Clock Divide /64
            RT0PSDIV_5 = 0b101,
            /// RTC Prescale Timer 0 Clock Divide /128
            RT0PSDIV_6 = 0b110,
            /// RTC Prescale Timer 0 Clock Divide /256
            RT0PSDIV_7 = 0b111,
        }
    }
    /// Real Timer Prescale Timer 1 Control
    rw RTCPS1CTL @ 0x0a: u16 = 0_0 {
        /// RTC Prescale Timer 1 Interrupt Flag
        RT1PSIFG: 0 = struct RT1PSIFG(bool);
        /// RTC Prescale Timer 1 Interrupt Enable Flag
        RT1PSIE: 1 = struct RT1PSIE(bool);
        /// RTC Prescale Timer 1 Interrupt Interval Bit: 2
        RT1IP: 2..4 = enum RT1IP {
            /// RTC Prescale Timer 1 Interrupt Interval /2
            RT1IP_0 = 0b000,
            /// RTC Prescale Timer 1 Interrupt Interval /4
            RT1IP_1 = 0b001,
            /// RTC Prescale Timer 1 Interrupt Interval /8
            RT1IP_2 = 0b010,
            /// RTC Prescale Timer 1 Interrupt Interval /16
            RT1IP_3 = 0b011,
            /// RTC Prescale Timer 1 Interrupt Interval /32
            RT1IP_4 = 0b100,
            /// RTC Prescale Timer 1 Interrupt Interval /64
            RT1IP_5 = 0b101,
            /// RTC Prescale Timer 1 Interrupt Interval /128
            RT1IP_6 = 0b110,
            /// RTC Prescale Timer 1 Interrupt Interval /256
            RT1IP_7 = 0b111,
        }
        /// RTC Prescale Timer 1 Hold
        RT1PSHOLD: 8 = struct RT1PSHOLD(bool);
        /// RTC Prescale Timer 1 Clock Divide Bit: 2
        RT1PSDIV: 11..13 = enum RT1PSDIV {
            /// RTC Prescale Timer 1 Clock Divide /2
            RT1PSDIV_0 = 0b000,
            /// RTC Prescale Timer 1 Clock Divide /4
            RT1PSDIV_1 = 0b001,
            /// RTC Prescale Timer 1 Clock Divide /8
            RT1PSDIV_2 = 0b010,
            /// RTC Prescale Timer 1 Clock Divide /16
            RT1PSDIV_3 = 0b011,
            /// RTC Prescale Timer 1 Clock Divide /32
            RT1PSDIV_4 = 0b100,
            /// RTC Prescale Timer 1 Clock Divide /64
            RT1PSDIV_5 = 0b101,
            /// RTC Prescale Timer 1 Clock Divide /128
            RT1PSDIV_6 = 0b110,
            /// RTC Prescale Timer 1 Clock Divide /256
            RT1PSDIV_7 = 0b111,
        }
        /// RTC Prescale Timer 1 Source Select Bit 1
        RT1SSEL: 14..15 = enum RT1SSEL {
            /// RTC Prescale Timer Source Select ACLK
            RT1SSEL_0 = 0b00,
            /// RTC Prescale Timer Source Select SMCLK
            RT1SSEL_1 = 0b01,
            /// RTC Prescale Timer Source Select RT0PS
            RT1SSEL_2 = 0b10,
            /// RTC Prescale Timer Source Select RT0PS
            RT1SSEL_3 = 0b11,
        }
    }
    /// Real Timer Prescale Timer Control
    rw RTCPS @ 0x0c: u16 = 0_0 {
        /// Real Timer Prescale Timer Control
        RTCPS: 0..15 = struct RTCPSField(u16);
    }
    /// Real Time Clock Interrupt Vector
    rw RTCIV @ 0x0e: u16 = 0_0 {
        /// Real Time Clock Interrupt Vector
        RTCIV: 0..15 = struct RTCIVField(u16);
    }
    /// Real Time Clock Time 0
    rw RTCTIM0 @ 0x10: u16 = 0_0 {
        /// Real Time Clock Time 0
        RTCTIM0: 0..15 = struct RTCTIM0Field(u16);
    }
    /// Real Time Clock Time 1
    rw RTCTIM1 @ 0x12: u16 = 0_0 {
        /// Real Time Clock Time 1
        RTCTIM1: 0..15 = struct RTCTIM1Field(u16);
    }
    /// Real Time Clock Date
    rw RTCDATE @ 0x14: u16 = 0_0 {
        /// Real Time Clock Date
        RTCDATE: 0..15 = struct RTCDATEField(u16);
    }
    /// Real Time Clock Year
    rw RTCYEAR @ 0x16: u16 = 0_0 {
        /// Real Time Clock Year
        RTCYEAR: 0..15 = struct RTCYEARField(u16);
    }
    /// Real Time Clock Alarm Min/Hour
    rw RTCAMINHR @ 0x18: u16 = 0_0 {
        /// Real Time Clock Alarm Min/Hour
        RTCAMINHR: 0..15 = struct RTCAMINHRField(u16);
    }
    /// Real Time Clock Alarm day of week/day
    rw RTCADOWDAY @ 0x1a: u16 = 0_0 {
        /// Real Time Clock Alarm day of week/day
        RTCADOWDAY: 0..15 = struct RTCADOWDAYField(u16);
    }
    /// Real Time Binary-to-BCD conversion register
    rw BIN2BCD @ 0x1c: u16 = 0_0 {
        /// Real Time Binary-to-BCD conversion register
        BIN2BCD: 0..15 = struct BIN2BCDField(u16);
    }
    /// Real Time BCD-to-binary conversion register
    rw BCD2BIN @ 0x1e: u16 = 0_0 {
        /// Real Time BCD-to-binary conversion register
        BCD2BIN: 0..15 = struct BCD2BINField(u16);
    }
    /// Real Time Clock Seconds
    rw RTCSEC @ 0x10: u8 = 0_0 {
        /// Real Time Clock Seconds Bit: 0
        SECONDS0: 0 = struct SECONDS0(bool);
        /// Real Time Clock Seconds Bit: 1
        SECONDS1: 1 = struct SECONDS1(bool);
        /// Real Time Clock Seconds Bit: 2
        SECONDS2: 2 = struct SECONDS2(bool);
        /// Real Time Clock Seconds Bit: 3
        SECONDS3: 3 = struct SECONDS3(bool);
        /// Real Time Clock Seconds Bit: 4
        SECONDS4: 4 = struct SECONDS4(bool);
        /// Real Time Clock Seconds Bit: 5
        SECONDS5: 5 = struct SECONDS5(bool);
        /// Real Time Clock Seconds Bit: 6
        SECONDS6: 6 = struct SECONDS6(bool);
    }
    /// Real Time Clock Minutes
    rw RTCMIN @ 0x11: u8 = 0_0 {
        /// Real Time Clock Minutes Bit: 0
        RTCMIN_MINUTES0: 0 = struct RTCMIN_MINUTES0(bool);
        /// Real Time Clock Minutes Bit: 1
        RTCMIN_MINUTES1: 1 = struct RTCMIN_MINUTES1(bool);
        /// Real Time Clock Minutes Bit: 2
        RTCMIN_MINUTES2: 2 = struct RTCMIN_MINUTES2(bool);
        /// Real Time Clock Minutes Bit: 3
        RTCMIN_MINUTES3: 3 = struct RTCMIN_MINUTES3(bool);
        /// Real Time Clock Minutes Bit: 4
        RTCMIN_MINUTES4: 4 = struct RTCMIN_MINUTES4(bool);
        /// Real Time Clock Minutes Bit: 5
        RTCMIN_MINUTES5: 5 = struct RTCMIN_MINUTES5(bool);
        /// Real Time Clock Minutes Bit: 6
        RTCMIN_MINUTES6: 6 = struct RTCMIN_MINUTES6(bool);
    }
    /// Real Time Clock Hour
    rw RTCHOUR @ 0x12: u8 = 0_0 {
        /// Real Time Clock Hour Bit: 0
        RTCHOUR_HOUR0: 0 = struct RTCHOUR_HOUR0(bool);
        /// Real Time Clock Hour Bit: 1
        RTCHOUR_HOUR1: 1 = struct RTCHOUR_HOUR1(bool);
        /// Real Time Clock Hour Bit: 2
        RTCHOUR_HOUR2: 2 = struct RTCHOUR_HOUR2(bool);
        /// Real Time Clock Hour Bit: 3
        RTCHOUR_HOUR3: 3 = struct RTCHOUR_HOUR3(bool);
        /// Real Time Clock Hour Bit: 4
        RTCHOUR_HOUR4: 4 = struct RTCHOUR_HOUR4(bool);
        /// Real Time Clock Hour Bit: 5
        RTCHOUR_HOUR5: 5 = struct RTCHOUR_HOUR5(bool);
        /// Real Time Clock Hour Bit: 6
        RTCHOUR_HOUR6: 6 = struct RTCHOUR_HOUR6(bool);
    }
    /// Real Time Clock Day of week
    rw RTCDOW @ 0x13: u8 = 0_0 {
        /// Real Time Clock DOW Bit: 0
        RTCDOW_DOW0: 0 = struct RTCDOW_DOW0(bool);
        /// Real Time Clock DOW Bit: 1
        RTCDOW_DOW1: 1 = struct RTCDOW_DOW1(bool);
        /// Real Time Clock DOW Bit: 2
        RTCDOW_DOW2: 2 = struct RTCDOW_DOW2(bool);
        /// Real Time Clock DOW Bit: 3
        RTCDOW_DOW3: 3 = struct RTCDOW_DOW3(bool);
        /// Real Time Clock DOW Bit: 4
        RTCDOW_DOW4: 4 = struct RTCDOW_DOW4(bool);
        /// Real Time Clock DOW Bit: 5
        RTCDOW_DOW5: 5 = struct RTCDOW_DOW5(bool);
        /// Real Time Clock DOW Bit: 6
        RTCDOW_DOW6: 6 = struct RTCDOW_DOW6(bool);
    }
    /// Real Time Clock Day
    rw RTCDAY @ 0x14: u8 = 0_0 {
        /// Real Time Clock Day Bit: 0
        RTCDAY_DAY0: 0 = struct RTCDAY_DAY0(bool);
        /// Real Time Clock Day Bit: 1
        RTCDAY_DAY1: 1 = struct RTCDAY_DAY1(bool);
        /// Real Time Clock Day Bit: 2
        RTCDAY_DAY2: 2 = struct RTCDAY_DAY2(bool);
        /// Real Time Clock Day Bit: 3
        RTCDAY_DAY3: 3 = struct RTCDAY_DAY3(bool);
        /// Real Time Clock Day Bit: 4
        RTCDAY_DAY4: 4 = struct RTCDAY_DAY4(bool);
        /// Real Time Clock Day Bit: 5
        RTCDAY_DAY5: 5 = struct RTCDAY_DAY5(bool);
        /// Real Time Clock Day Bit: 6
        RTCDAY_DAY6: 6 = struct RTCDAY_DAY6(bool);
    }
    /// Real Time Clock Month
    rw RTCMON @ 0x15: u8 = 0_0 {
        /// Real Time Clock Month Bit: 0
        MONTH0: 0 = struct MONTH0(bool);
        /// Real Time Clock Month Bit: 1
        MONTH1: 1 = struct MONTH1(bool);
        /// Real Time Clock Month Bit: 2
        MONTH2: 2 = struct MONTH2(bool);
        /// Real Time Clock Month Bit: 3
        MONTH3: 3 = struct MONTH3(bool);
        /// Real Time Clock Month Bit: 4
        MONTH4: 4 = struct MONTH4(bool);
        /// Real Time Clock Month Bit: 5
        MONTH5: 5 = struct MONTH5(bool);
        /// Real Time Clock Month Bit: 6
        MONTH6: 6 = struct MONTH6(bool);
    }
    /// Real Time Clock Alarm Min
    rw RTCAMIN @ 0x18: u8 = 0_0 {
        /// Real Time Clock Minutes Bit: 0
        RTCAMIN_MINUTES0: 0 = struct RTCAMIN_MINUTES0(bool);
        /// Real Time Clock Minutes Bit: 1
        RTCAMIN_MINUTES1: 1 = struct RTCAMIN_MINUTES1(bool);
        /// Real Time Clock Minutes Bit: 2
        RTCAMIN_MINUTES2: 2 = struct RTCAMIN_MINUTES2(bool);
        /// Real Time Clock Minutes Bit: 3
        RTCAMIN_MINUTES3: 3 = struct RTCAMIN_MINUTES3(bool);
        /// Real Time Clock Minutes Bit: 4
        RTCAMIN_MINUTES4: 4 = struct RTCAMIN_MINUTES4(bool);
        /// Real Time Clock Minutes Bit: 5
        RTCAMIN_MINUTES5: 5 = struct RTCAMIN_MINUTES5(bool);
        /// Real Time Clock Minutes Bit: 6
        RTCAMIN_MINUTES6: 6 = struct RTCAMIN_MINUTES6(bool);
        /// Real Time Clock Alarm enable
        RTCAMIN_RTCAE: 7 = struct RTCAMIN_RTCAE(bool);
    }
    /// Real Time Clock Alarm Hour
    rw RTCAHOUR @ 0x19: u8 = 0_0 {
        /// Real Time Clock Hour Bit: 0
        RTCAHOUR_HOUR0: 0 = struct RTCAHOUR_HOUR0(bool);
        /// Real Time Clock Hour Bit: 1
        RTCAHOUR_HOUR1: 1 = struct RTCAHOUR_HOUR1(bool);
        /// Real Time Clock Hour Bit: 2
        RTCAHOUR_HOUR2: 2 = struct RTCAHOUR_HOUR2(bool);
        /// Real Time Clock Hour Bit: 3
        RTCAHOUR_HOUR3: 3 = struct RTCAHOUR_HOUR3(bool);
        /// Real Time Clock Hour Bit: 4
        RTCAHOUR_HOUR4: 4 = struct RTCAHOUR_HOUR4(bool);
        /// Real Time Clock Hour Bit: 5
        RTCAHOUR_HOUR5: 5 = struct RTCAHOUR_HOUR5(bool);
        /// Real Time Clock Hour Bit: 6
        RTCAHOUR_HOUR6: 6 = struct RTCAHOUR_HOUR6(bool);
        /// Real Time Clock Alarm enable
        RTCAHOUR_RTCAE: 7 = struct RTCAHOUR_RTCAE(bool);
    }
    /// Real Time Clock Alarm Day of week
    rw RTCADOW @ 0x1a: u8 = 0_0 {
        /// Real Time Clock DOW Bit: 0
        RTCADOW_DOW0: 0 = struct RTCADOW_DOW0(bool);
        /// Real Time Clock DOW Bit: 1
        RTCADOW_DOW1: 1 = struct RTCADOW_DOW1(bool);
        /// Real Time Clock DOW Bit: 2
        RTCADOW_DOW2: 2 = struct RTCADOW_DOW2(bool);
        /// Real Time Clock DOW Bit: 3
        RTCADOW_DOW3: 3 = struct RTCADOW_DOW3(bool);
        /// Real Time Clock DOW Bit: 4
        RTCADOW_DOW4: 4 = struct RTCADOW_DOW4(bool);
        /// Real Time Clock DOW Bit: 5
        RTCADOW_DOW5: 5 = struct RTCADOW_DOW5(bool);
        /// Real Time Clock DOW Bit: 6
        RTCADOW_DOW6: 6 = struct RTCADOW_DOW6(bool);
        /// Real Time Clock Alarm enable
        RTCADOW_RTCAE: 7 = struct RTCADOW_RTCAE(bool);
    }
    /// Real Time Clock Alarm Day
    rw RTCADAY @ 0x1b: u8 = 0_0 {
        /// Real Time Clock Day Bit: 0
        RTCADAY_DAY0: 0 = struct RTCADAY_DAY0(bool);
        /// Real Time Clock Day Bit: 1
        RTCADAY_DAY1: 1 = struct RTCADAY_DAY1(bool);
        /// Real Time Clock Day Bit: 2
        RTCADAY_DAY2: 2 = struct RTCADAY_DAY2(bool);
        /// Real Time Clock Day Bit: 3
        RTCADAY_DAY3: 3 = struct RTCADAY_DAY3(bool);
        /// Real Time Clock Day Bit: 4
        RTCADAY_DAY4: 4 = struct RTCADAY_DAY4(bool);
        /// Real Time Clock Day Bit: 5
        RTCADAY_DAY5: 5 = struct RTCADAY_DAY5(bool);
        /// Real Time Clock Day Bit: 6
        RTCADAY_DAY6: 6 = struct RTCADAY_DAY6(bool);
        /// Real Time Clock Alarm enable
        RTCADAY_RTCAE: 7 = struct RTCADAY_RTCAE(bool);
    }
    /// Real Time Clock Time 0
    rw RTCNT12 @ 0x10: u16 = 0_0 {
        /// Real Time Clock Time 0
        RTCNT12: 0..15 = struct RTCNT12Field(u16);
    }
    /// Real Time Clock Time 1
    rw RTCNT34 @ 0x12: u16 = 0_0 {
        /// Real Time Clock Time 1
        RTCNT34: 0..15 = struct RTCNT34Field(u16);
    }
}
