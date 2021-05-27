//! RTC_D  Real Time Clock

utils::periph! {
    /// RTC_D  Real Time Clock
    RTC_D;
    /// Real Timer Control 0/1
    rw CTL01 @ 0x00: u16 = 0_0 {
        /// RTC Ready Interrupt Flag
        RDYIFG: 0 = struct RDYIFG(bool);
        /// RTC Alarm Interrupt Flag
        AIFG: 1 = struct AIFG(bool);
        /// RTC Time Event Interrupt Flag
        TEVIFG: 2 = struct TEVIFG(bool);
        /// RTC 32kHz cyrstal oscillator fault interrupt flag
        OFIFG: 3 = struct OFIFG(bool);
        /// RTC Ready Interrupt Enable Flag
        RDYIE: 4 = struct RDYIE(bool);
        /// RTC Alarm Interrupt Enable Flag
        AIE: 5 = struct AIE(bool);
        /// RTC Time Event Interrupt Enable Flag
        TEVIE: 6 = struct TEVIE(bool);
        /// RTC 32kHz cyrstal oscillator fault interrupt enable
        OFIE: 7 = struct OFIE(bool);
        /// RTC Time Event 1
        TEV: 8..9 = enum TEV {
            /// RTC Time Event: 0 (Min. changed)
            TEV_0 = 0b00,
            /// RTC Time Event: 1 (Hour changed)
            TEV_1 = 0b01,
            /// RTC Time Event: 2 (12:00 changed)
            TEV_2 = 0b10,
            /// RTC Time Event: 3 (00:00 changed)
            TEV_3 = 0b11,
        }
        /// RTC Source Select 1
        SSEL: 10..11 = enum SSEL {
            /// RTC Source Select ACLK
            SSEL_0 = 0b00,
            /// RTC Source Select SMCLK
            SSEL_1 = 0b01,
            /// RTC Source Select RT1PS
            SSEL_2 = 0b10,
            /// RTC Source Select RT1PS
            SSEL_3 = 0b11,
        }
        /// RTC Ready
        RDY: 12 = struct RDY(bool);
        /// RTC Mode 0:Counter / 1: Calendar
        MODE: 13 = struct MODE(bool);
        /// RTC Hold
        HOLD: 14 = struct HOLD(bool);
        /// RTC BCD  0:Binary / 1:BCD
        BCD: 15 = struct BCD(bool);
    }
    /// Real Timer Control 2/3
    rw CTL23 @ 0x02: u16 = 0_0 {
        /// RTC Calibration Bit 0
        CAL0: 0 = struct CAL0(bool);
        /// RTC Calibration Bit 1
        CAL1: 1 = struct CAL1(bool);
        /// RTC Calibration Bit 2
        CAL2: 2 = struct CAL2(bool);
        /// RTC Calibration Bit 3
        CAL3: 3 = struct CAL3(bool);
        /// RTC Calibration Bit 4
        CAL4: 4 = struct CAL4(bool);
        /// RTC Calibration Bit 5
        CAL5: 5 = struct CAL5(bool);
        /// RTC Calibration Sign
        CALS: 7 = struct CALS(bool);
        /// RTC Calibration Frequency Bit 1
        CALF: 8..9 = enum CALF {
            /// RTC Calibration Frequency: No Output
            CALF_0 = 0b00,
            /// RTC Calibration Frequency: 512 Hz
            CALF_1 = 0b01,
            /// RTC Calibration Frequency: 256 Hz
            CALF_2 = 0b10,
            /// RTC Calibration Frequency: 1 Hz
            CALF_3 = 0b11,
        }
    }
    /// Real Timer Prescale Timer 0 Control
    rw PS0CTL @ 0x08: u16 = 0_0 {
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
    rw PS1CTL @ 0x0a: u16 = 0_0 {
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
    rw PS @ 0x0c: u16 = 0_0 {
        /// Real Timer Prescale Timer Control
        PS: 0..15 = struct PSField(u16);
    }
    /// Real Time Clock Interrupt Vector
    rw IV @ 0x0e: u16 = 0_0 {
        /// Real Time Clock Interrupt Vector
        IV: 0..15 = struct IVField(u16);
    }
    /// Real Time Clock Time 0
    rw TIM0 @ 0x10: u16 = 0_0 {
        /// Real Time Clock Time 0
        TIM0: 0..15 = struct TIM0Field(u16);
    }
    /// Real Time Clock Time 1
    rw TIM1 @ 0x12: u16 = 0_0 {
        /// Real Time Clock Time 1
        TIM1: 0..15 = struct TIM1Field(u16);
    }
    /// Real Time Clock Date
    rw DATE @ 0x14: u16 = 0_0 {
        /// Real Time Clock Date
        DATE: 0..15 = struct DATEField(u16);
    }
    /// Real Time Clock Year
    rw YEAR @ 0x16: u16 = 0_0 {
        /// Real Time Clock Year
        YEAR: 0..15 = struct YEARField(u16);
    }
    /// Real Time Clock Alarm Min/Hour
    rw AMINHR @ 0x18: u16 = 0_0 {
        /// Real Time Clock Alarm Min/Hour
        AMINHR: 0..15 = struct AMINHRField(u16);
    }
    /// Real Time Clock Alarm day of week/day
    rw ADOWDAY @ 0x1a: u16 = 0_0 {
        /// Real Time Clock Alarm day of week/day
        ADOWDAY: 0..15 = struct ADOWDAYField(u16);
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
    rw SEC @ 0x10: u8 = 0_0 {
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
    rw MIN @ 0x11: u8 = 0_0 {
        /// Real Time Clock Minutes Bit: 0
        MIN0: 0 = struct MIN0(bool);
        /// Real Time Clock Minutes Bit: 1
        MIN1: 1 = struct MIN1(bool);
        /// Real Time Clock Minutes Bit: 2
        MIN2: 2 = struct MIN2(bool);
        /// Real Time Clock Minutes Bit: 3
        MIN3: 3 = struct MIN3(bool);
        /// Real Time Clock Minutes Bit: 4
        MIN4: 4 = struct MIN4(bool);
        /// Real Time Clock Minutes Bit: 5
        MIN5: 5 = struct MIN5(bool);
        /// Real Time Clock Minutes Bit: 6
        MIN6: 6 = struct MIN6(bool);
    }
    /// Real Time Clock Hour
    rw HOUR @ 0x12: u8 = 0_0 {
        /// Real Time Clock Hour Bit: 0
        HOUR0: 0 = struct HOUR0(bool);
        /// Real Time Clock Hour Bit: 1
        HOUR1: 1 = struct HOUR1(bool);
        /// Real Time Clock Hour Bit: 2
        HOUR2: 2 = struct HOUR2(bool);
        /// Real Time Clock Hour Bit: 3
        HOUR3: 3 = struct HOUR3(bool);
        /// Real Time Clock Hour Bit: 4
        HOUR4: 4 = struct HOUR4(bool);
        /// Real Time Clock Hour Bit: 5
        HOUR5: 5 = struct HOUR5(bool);
        /// Real Time Clock Hour Bit: 6
        HOUR6: 6 = struct HOUR6(bool);
    }
    /// Real Time Clock Day of week
    rw DOW @ 0x13: u8 = 0_0 {
        /// Real Time Clock DOW Bit: 0
        DOW0: 0 = struct DOW0(bool);
        /// Real Time Clock DOW Bit: 1
        DOW1: 1 = struct DOW1(bool);
        /// Real Time Clock DOW Bit: 2
        DOW2: 2 = struct DOW2(bool);
        /// Real Time Clock DOW Bit: 3
        DOW3: 3 = struct DOW3(bool);
        /// Real Time Clock DOW Bit: 4
        DOW4: 4 = struct DOW4(bool);
        /// Real Time Clock DOW Bit: 5
        DOW5: 5 = struct DOW5(bool);
        /// Real Time Clock DOW Bit: 6
        DOW6: 6 = struct DOW6(bool);
    }
    /// Real Time Clock Day
    rw DAY @ 0x14: u8 = 0_0 {
        /// Real Time Clock Day Bit: 0
        DAY0: 0 = struct DAY0(bool);
        /// Real Time Clock Day Bit: 1
        DAY1: 1 = struct DAY1(bool);
        /// Real Time Clock Day Bit: 2
        DAY2: 2 = struct DAY2(bool);
        /// Real Time Clock Day Bit: 3
        DAY3: 3 = struct DAY3(bool);
        /// Real Time Clock Day Bit: 4
        DAY4: 4 = struct DAY4(bool);
        /// Real Time Clock Day Bit: 5
        DAY5: 5 = struct DAY5(bool);
        /// Real Time Clock Day Bit: 6
        DAY6: 6 = struct DAY6(bool);
    }
    /// Real Time Clock Month
    rw MON @ 0x15: u8 = 0_0 {
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
    rw AMIN @ 0x18: u8 = 0_0 {
        /// Real Time Clock Minutes Bit: 0
        AMIN0: 0 = struct AMIN0(bool);
        /// Real Time Clock Minutes Bit: 1
        AMIN1: 1 = struct AMIN1(bool);
        /// Real Time Clock Minutes Bit: 2
        AMIN2: 2 = struct AMIN2(bool);
        /// Real Time Clock Minutes Bit: 3
        AMIN3: 3 = struct AMIN3(bool);
        /// Real Time Clock Minutes Bit: 4
        AMIN4: 4 = struct AMIN4(bool);
        /// Real Time Clock Minutes Bit: 5
        AMIN5: 5 = struct AMIN5(bool);
        /// Real Time Clock Minutes Bit: 6
        AMIN6: 6 = struct AMIN6(bool);
        /// Real Time Clock Alarm enable
        AMIN_RTCAE: 7 = struct AMIN_RTCAE(bool);
    }
    /// Real Time Clock Alarm Hour
    rw AHOUR @ 0x19: u8 = 0_0 {
        /// Real Time Clock Hour Bit: 0
        AHOUR0: 0 = struct AHOUR0(bool);
        /// Real Time Clock Hour Bit: 1
        AHOUR1: 1 = struct AHOUR1(bool);
        /// Real Time Clock Hour Bit: 2
        AHOUR2: 2 = struct AHOUR2(bool);
        /// Real Time Clock Hour Bit: 3
        AHOUR3: 3 = struct AHOUR3(bool);
        /// Real Time Clock Hour Bit: 4
        AHOUR4: 4 = struct AHOUR4(bool);
        /// Real Time Clock Hour Bit: 5
        AHOUR5: 5 = struct AHOUR5(bool);
        /// Real Time Clock Hour Bit: 6
        AHOUR6: 6 = struct AHOUR6(bool);
        /// Real Time Clock Alarm enable
        AHOUR_RTCAE: 7 = struct AHOUR_RTCAE(bool);
    }
    /// Real Time Clock Alarm Day of week
    rw ADOW @ 0x1a: u8 = 0_0 {
        /// Real Time Clock DOW Bit: 0
        ADOW0: 0 = struct ADOW0(bool);
        /// Real Time Clock DOW Bit: 1
        ADOW1: 1 = struct ADOW1(bool);
        /// Real Time Clock DOW Bit: 2
        ADOW2: 2 = struct ADOW2(bool);
        /// Real Time Clock DOW Bit: 3
        ADOW3: 3 = struct ADOW3(bool);
        /// Real Time Clock DOW Bit: 4
        ADOW4: 4 = struct ADOW4(bool);
        /// Real Time Clock DOW Bit: 5
        ADOW5: 5 = struct ADOW5(bool);
        /// Real Time Clock DOW Bit: 6
        ADOW6: 6 = struct ADOW6(bool);
        /// Real Time Clock Alarm enable
        ADOW_RTCAE: 7 = struct ADOW_RTCAE(bool);
    }
    /// Real Time Clock Alarm Day
    rw ADAY @ 0x1b: u8 = 0_0 {
        /// Real Time Clock Day Bit: 0
        ADAY0: 0 = struct ADAY0(bool);
        /// Real Time Clock Day Bit: 1
        ADAY1: 1 = struct ADAY1(bool);
        /// Real Time Clock Day Bit: 2
        ADAY2: 2 = struct ADAY2(bool);
        /// Real Time Clock Day Bit: 3
        ADAY3: 3 = struct ADAY3(bool);
        /// Real Time Clock Day Bit: 4
        ADAY4: 4 = struct ADAY4(bool);
        /// Real Time Clock Day Bit: 5
        ADAY5: 5 = struct ADAY5(bool);
        /// Real Time Clock Day Bit: 6
        ADAY6: 6 = struct ADAY6(bool);
        /// Real Time Clock Alarm enable
        ADAY_RTCAE: 7 = struct ADAY_RTCAE(bool);
    }
    /// Real Time Clock Time 0
    rw NT12 @ 0x10: u16 = 0_0 {
        /// Real Time Clock Time 0
        NT12: 0..15 = struct NT12Field(u16);
    }
    /// Real Time Clock Time 1
    rw NT34 @ 0x12: u16 = 0_0 {
        /// Real Time Clock Time 1
        NT34: 0..15 = struct NT34Field(u16);
    }
}
