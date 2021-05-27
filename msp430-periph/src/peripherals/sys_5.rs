//! SYS  System Module

utils::periph! {
    /// SYS  System Module
    SYS;
    /// System control
    rw CTL @ 0x00: u16 = 0_0 {
        /// SYS - RAM based interrupt vectors
        RIVECT: 0 = struct RIVECT(bool);
        /// SYS - PMM access protect
        PMMPE: 2 = struct PMMPE(bool);
        /// SYS - TCK/RST indication detected
        BSLIND: 4 = struct BSLIND(bool);
        /// SYS - Dedicated JTAG pins enabled
        JTAGPIN: 5 = struct JTAGPIN(bool);
    }
    /// Boot strap configuration area
    rw BSLC @ 0x02: u16 = 0_0 {
        /// SYS - RAM assigned to BSL
        BSLR: 2 = struct BSLR(bool);
        /// SYS - BSL Memory disabled
        BSLOFF: 14 = struct BSLOFF(bool);
        /// SYS - BSL Memory protection enabled
        BSLPE: 15 = struct BSLPE(bool);
    }
    /// JTAG mailbox control
    rw JMBC @ 0x06: u16 = 0_0 {
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
    rw JMBI0 @ 0x08: u16 = 0_0 {
        /// JTAG mailbox input 0
        JMBI0: 0..15 = struct JMBI0Field(u16);
    }
    /// JTAG mailbox input 1
    rw JMBI1 @ 0x0a: u16 = 0_0 {
        /// JTAG mailbox input 1
        JMBI1: 0..15 = struct JMBI1Field(u16);
    }
    /// JTAG mailbox output 0
    rw JMBO0 @ 0x0c: u16 = 0_0 {
        /// JTAG mailbox output 0
        JMBO0: 0..15 = struct JMBO0Field(u16);
    }
    /// JTAG mailbox output 1
    rw JMBO1 @ 0x0e: u16 = 0_0 {
        /// JTAG mailbox output 1
        JMBO1: 0..15 = struct JMBO1Field(u16);
    }
    /// Bus Error vector generator
    rw BERRIV @ 0x18: u16 = 0_0 {
        /// Bus Error vector generator
        BERRIV: 0..15 = struct BERRIVField(u16);
    }
    /// User NMI vector generator
    rw UNIV @ 0x1a: u16 = 0_0 {
        /// User NMI vector generator
        UNIV: 0..15 = struct UNIVField(u16);
    }
    /// System NMI vector generator
    rw SNIV @ 0x1c: u16 = 0_0 {
        /// System NMI vector generator
        SNIV: 0..15 = struct SNIVField(u16);
    }
    /// Reset vector generator
    rw RSTIV @ 0x1e: u16 = 0_0 {
        /// Reset vector generator
        RSTIV: 0..15 = struct RSTIVField(u16);
    }
    /// System Configuration 0
    rw CFG0 @ 0x20: u16 = 0_0 {
        /// Program FRAM Write Protection
        PFWP: 0 = struct PFWP(bool);
        /// Data FRAM Write Protection
        DFWP: 1 = struct DFWP(bool);
    }
    /// System Configuration 1
    rw CFG1 @ 0x22: u16 = 0_0 {
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
    }
    /// System Configuration 2
    rw CFG2 @ 0x24: u16 = 0_0 {
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
