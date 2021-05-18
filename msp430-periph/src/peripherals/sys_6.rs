//! SYS  System Module

utils::periph! {
    /// SYS  System Module
    SYS;
    /// System control
    rw SYSCTL @ 0x00: u16 = 0_0 {
        /// SYS - RAM based interrupt vectors
        SYSRIVECT: 0 = struct SYSRIVECT(bool);
        /// SYS - PMM access protect
        SYSPMMPE: 2 = struct SYSPMMPE(bool);
        /// SYS - TCK/RST indication detected
        SYSBSLIND: 4 = struct SYSBSLIND(bool);
        /// SYS - Dedicated JTAG pins enabled
        SYSJTAGPIN: 5 = struct SYSJTAGPIN(bool);
    }
    /// Boot strap configuration area
    rw SYSBSLC @ 0x02: u16 = 0_0 {
        /// SYS - RAM assigned to BSL
        SYSBSLR: 2 = struct SYSBSLR(bool);
        /// SYS - BSL Memory disabled
        SYSBSLOFF: 14 = struct SYSBSLOFF(bool);
        /// SYS - BSL Memory protection enabled
        SYSBSLPE: 15 = struct SYSBSLPE(bool);
    }
    /// JTAG mailbox control
    rw SYSJMBC @ 0x06: u16 = 0_0 {
        /// SYS - Incoming JTAG Mailbox 0 Flag
        JMBIN0FG: 0 = struct JMBIN0FG(bool);
        /// SYS - Incoming JTAG Mailbox 1 Flag
        JMBIN1FG: 1 = struct JMBIN1FG(bool);
        /// SYS - Outgoing JTAG Mailbox 0 Flag
        JMBOUT0FG: 2 = struct JMBOUT0FG(bool);
        /// SYS - Outgoing JTAG Mailbox 1 Flag
        JMBOUT1FG: 3 = struct JMBOUT1FG(bool);
        /// SYS - JMB 16/32 Bit Mode
        JMBMODE: 4 = struct JMBMODE(bool);
        /// SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe
        JMBCLR0OFF: 6 = struct JMBCLR0OFF(bool);
        /// SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe
        JMBCLR1OFF: 7 = struct JMBCLR1OFF(bool);
    }
    /// JTAG mailbox input 0
    rw SYSJMBI0 @ 0x08: u16 = 0_0 {
        /// JTAG mailbox input 0
        SYSJMBI0: 0..15 = struct SYSJMBI0Field(u16);
    }
    /// JTAG mailbox input 1
    rw SYSJMBI1 @ 0x0a: u16 = 0_0 {
        /// JTAG mailbox input 1
        SYSJMBI1: 0..15 = struct SYSJMBI1Field(u16);
    }
    /// JTAG mailbox output 0
    rw SYSJMBO0 @ 0x0c: u16 = 0_0 {
        /// JTAG mailbox output 0
        SYSJMBO0: 0..15 = struct SYSJMBO0Field(u16);
    }
    /// JTAG mailbox output 1
    rw SYSJMBO1 @ 0x0e: u16 = 0_0 {
        /// JTAG mailbox output 1
        SYSJMBO1: 0..15 = struct SYSJMBO1Field(u16);
    }
    /// Bus Error vector generator
    rw SYSBERRIV @ 0x18: u16 = 0_0 {
        /// Bus Error vector generator
        SYSBERRIV: 0..15 = struct SYSBERRIVField(u16);
    }
    /// User NMI vector generator
    rw SYSUNIV @ 0x1a: u16 = 0_0 {
        /// User NMI vector generator
        SYSUNIV: 0..15 = struct SYSUNIVField(u16);
    }
    /// System NMI vector generator
    rw SYSSNIV @ 0x1c: u16 = 0_0 {
        /// System NMI vector generator
        SYSSNIV: 0..15 = struct SYSSNIVField(u16);
    }
    /// Reset vector generator
    rw SYSRSTIV @ 0x1e: u16 = 0_0 {
        /// Reset vector generator
        SYSRSTIV: 0..15 = struct SYSRSTIVField(u16);
    }
    /// System Configuration 0
    rw SYSCFG0 @ 0x20: u16 = 0_0 {
        /// Program FRAM Write Protection
        PFWP: 0 = struct PFWP(bool);
        /// Data FRAM Write Protection
        DFWP: 1 = struct DFWP(bool);
    }
    /// System Configuration 1
    rw SYSCFG1 @ 0x22: u16 = 0_0 {
        /// Infrared enable
        IREN: 0 = struct IREN(bool);
        /// Infrared polarity select
        IRPSEL: 1 = struct IRPSEL(bool);
        /// Infrared mode select
        IRMSEL: 2 = struct IRMSEL(bool);
        /// Infrared data source select
        IRDSSEL: 3 = struct IRDSSEL(bool);
        /// Infrared enable
        IRDATA: 4 = struct IRDATA(bool);
        /// Captivate conversion triggered source select Bit: 0
        SYNCSEL: 6..7 = enum SYNCSEL {
            /// Captivate conversion triggered source select: External
            SYNCSEL_0 = 0b00,
            /// Captivate conversion triggered source select: ADC
            SYNCSEL_1 = 0b01,
            /// Captivate conversion triggered source select: Comparator
            SYNCSEL_2 = 0b10,
            /// Captivate conversion triggered source select: Res.
            SYNCSEL_3 = 0b11,
        }
    }
    /// System Configuration 2
    rw SYSCFG2 @ 0x24: u16 = 0_0 {
        /// ADC input A0 pin select
        ADCPCTL0: 0 = struct ADCPCTL0(bool);
        /// ADC input A1 pin select
        ADCPCTL1: 1 = struct ADCPCTL1(bool);
        /// ADC input A2 pin select
        ADCPCTL2: 2 = struct ADCPCTL2(bool);
        /// ADC input A3 pin select
        ADCPCTL3: 3 = struct ADCPCTL3(bool);
        /// ADC input A4 pin select
        ADCPCTL4: 4 = struct ADCPCTL4(bool);
        /// ADC input A5 pin select
        ADCPCTL5: 5 = struct ADCPCTL5(bool);
        /// ADC input A6 pin select
        ADCPCTL6: 6 = struct ADCPCTL6(bool);
        /// ADC input A7 pin select
        ADCPCTL7: 7 = struct ADCPCTL7(bool);
    }
}
