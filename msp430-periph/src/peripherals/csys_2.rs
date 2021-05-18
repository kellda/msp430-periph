//! CSYS  Compact System Module

utils::periph! {
    /// CSYS  Compact System Module
    CSYS;
    /// System control
    rw SYSCTL @ 0x00: u16 = 0_0 {
        /// SYS - RAM based interrupt vectors
        SYSRIVECT: 0 = struct SYSRIVECT(bool);
        /// SYS - Dedicated JTAG pins enabled
        SYSJTAGPIN: 5 = struct SYSJTAGPIN(bool);
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
    /// System Configuration Register
    rw SYSCNF @ 0x10: u16 = 0_0 {
        /// SYS - FRAM Write protect area 0
        FRAMLOCK0: 8 = struct FRAMLOCK0(bool);
        /// SYS - FRAM Write protect area 1
        FRAMLOCK1: 9 = struct FRAMLOCK1(bool);
        /// SYS - FRAM Write protect area 2
        FRAMLOCK2: 10 = struct FRAMLOCK2(bool);
        /// SYS - Write protect Re-Mapped ROM development area
        DEVRAMLOCK: 11 = struct DEVRAMLOCK(bool);
        /// SYS - Enable ROM Developent mode
        ROMDEVMODE: 12 = struct ROMDEVMODE(bool);
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
}
