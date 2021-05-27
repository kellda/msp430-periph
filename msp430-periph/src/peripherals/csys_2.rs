//! CSYS  Compact System Module

utils::periph! {
    /// CSYS  Compact System Module
    CSYS;
    /// System control
    rw CTL @ 0x00: u16 = 0_0 {
        /// SYS - RAM based interrupt vectors
        RIVECT: 0 = struct RIVECT(bool);
        /// SYS - Dedicated JTAG pins enabled
        JTAGPIN: 5 = struct JTAGPIN(bool);
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
    /// System Configuration Register
    rw CNF @ 0x10: u16 = 0_0 {
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
}
