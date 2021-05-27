//! SYS

utils::periph! {
    /// SYS
    SYS;
    /// System Control
    rw CTL @ 0x00: u16 = 0_0 {
        /// RAM-based interrupt vectors
        RIVECT: 0 = enum RIVECT {
            /// Interrupt vectors generated with end address TOP of lower 64K FRAM FFFFh
            FRAM = 0b0,
            /// Interrupt vectors generated with end address TOP of RAM, when RAM available
            RAM = 0b1,
        }
        /// PMM access protect
        PMMPE: 2 = enum PMMPE {
            /// Access from anywhere in memory
            PMMPE_0 = 0b0,
            /// Access only from the BSL segments
            PMMPE_1 = 0b1,
        }
        /// BSL entry indication
        BSLIND: 4 = enum BSLIND {
            /// No BSL entry sequence detected
            BSLIND_0 = 0b0,
            /// BSL entry sequence detected
            BSLIND_1 = 0b1,
        }
        /// Dedicated JTAG pins enable
        JTAGPIN: 5 = enum JTAGPIN {
            /// Shared JTAG pins (JTAG mode selectable using SBW sequence)
            SHARED = 0b0,
            /// Dedicated JTAG pins (explicit 4-wire JTAG mode selection)
            DEDICATED = 0b1,
        }
    }
    /// JTAG Mailbox Control
    rw JMBC @ 0x06: u16 = 0_0 {
        /// Incoming JTAG Mailbox 0 flag
        JMBIN0FG: 0 = enum JMBIN0FG {
            /// JMBI0 has no new data
            JMBIN0FG_0 = 0b0,
            /// JMBI0 has new data available
            JMBIN0FG_1 = 0b1,
        }
        /// Incoming JTAG Mailbox 1 flag
        JMBIN1FG: 1 = enum JMBIN1FG {
            /// JMBI1 has no new data
            JMBIN1FG_0 = 0b0,
            /// JMBI1 has new data available
            JMBIN1FG_1 = 0b1,
        }
        /// Outgoing JTAG Mailbox 0 flag
        JMBOUT0FG: 2 = enum JMBOUT0FG {
            /// JMBO0 is not ready to receive new data
            JMBOUT0FG_0 = 0b0,
            /// JMBO0 is ready to receive new data
            JMBOUT0FG_1 = 0b1,
        }
        /// Outgoing JTAG Mailbox 1 flag
        JMBOUT1FG: 3 = enum JMBOUT1FG {
            /// JMBO1 is not ready to receive new data
            JMBOUT1FG_0 = 0b0,
            /// JMBO1 is ready to receive new data
            JMBOUT1FG_1 = 0b1,
        }
        /// Operation mode of JMB
        JMBMODE: 4 = enum JMBMODE {
            /// 16-bit transfers using JMBO0 and JMBI0 only
            _16BIT = 0b0,
            /// 32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1
            _32BIT = 0b1,
        }
        /// Incoming JTAG Mailbox 0 flag auto-clear disable
        JMBCLR0OFF: 6 = enum JMBCLR0OFF {
            /// JMBIN0FG cleared on read of JMB0IN register
            JMBCLR0OFF_0 = 0b0,
            /// JMBIN0FG cleared by software
            JMBCLR0OFF_1 = 0b1,
        }
        /// Incoming JTAG Mailbox 1 flag auto-clear disable
        JMBCLR1OFF: 7 = enum JMBCLR1OFF {
            /// JMBIN1FG cleared on read of JMB1IN register
            JMBCLR1OFF_0 = 0b0,
            /// JMBIN1FG cleared by software
            JMBCLR1OFF_1 = 0b1,
        }
    }
    /// JTAG Mailbox Input
    rw JMBI0 @ 0x08: u16 = 0_0 {
        /// JTAG mailbox incoming message low byte
        JMBI0_MSGLO: 0..7 = struct JMBI0_MSGLO(u16);
        /// JTAG mailbox incoming message high byte
        JMBI0_MSGHI: 8..15 = struct JMBI0_MSGHI(u16);
    }
    /// JTAG Mailbox Input
    rw JMBI1 @ 0x0a: u16 = 0_0 {
        /// JTAG mailbox incoming message low byte
        JMBI1_MSGLO: 0..7 = struct JMBI1_MSGLO(u16);
        /// JTAG mailbox incoming message high byte
        JMBI1_MSGHI: 8..15 = struct JMBI1_MSGHI(u16);
    }
    /// JTAG Mailbox Output
    rw JMBO0 @ 0x0c: u16 = 0_0 {
        /// JTAG mailbox outgoing message low byte
        JMBO0_MSGLO: 0..7 = struct JMBO0_MSGLO(u16);
        /// JTAG mailbox outgoing message high byte
        JMBO0_MSGHI: 8..15 = struct JMBO0_MSGHI(u16);
    }
    /// JTAG Mailbox Output
    rw JMBO1 @ 0x0e: u16 = 0_0 {
        /// JTAG mailbox outgoing message low byte
        JMBO1_MSGLO: 0..7 = struct JMBO1_MSGLO(u16);
        /// JTAG mailbox outgoing message high byte
        JMBO1_MSGHI: 8..15 = struct JMBO1_MSGHI(u16);
    }
    /// User NMI Vector Generator
    r UNIV @ 0x1a: u16 = 0_0 {
        /// User NMI vector
        UNIV: 0..15 = enum UNIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// NMIIFG NMI pin
            NMIIFG = 0b0000000000000010,
            /// OFIFG oscillator fault
            OFIFG = 0b0000000000000100,
        }
    }
    /// System NMI Vector Generator
    r SNIV @ 0x1c: u16 = 0_0 {
        /// System NMI vector
        SNIV: 0..15 = enum SNIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Reserved
            SNIV_2 = 0b0000000000000010,
            /// Uncorrectable FRAM bit error detection
            UBDIFG = 0b0000000000000100,
            /// FRAM Access Time Error
            ACCTEIFG = 0b0000000000000110,
            /// MPUSEGPIFG encapsulated IP memory segment violation
            MPUSEGPIFG = 0b0000000000001000,
            /// MPUSEGIIFG information memory segment violation
            MPUSEGIIFG = 0b0000000000001010,
            /// MPUSEG1IFG segment 1 memory violation
            MPUSEG1IFG = 0b0000000000001100,
            /// MPUSEG2IFG segment 2 memory violation
            MPUSEG2IFG = 0b0000000000001110,
            /// MPUSEG3IFG segment 3 memory violation
            MPUSEG3IFG = 0b0000000000010000,
            /// VMAIFG Vacant memory access
            VMAIFG = 0b0000000000010010,
            /// JMBINIFG JTAG mailbox input
            JMBINIFG = 0b0000000000010100,
            /// JMBOUTIFG JTAG mailbox output
            JMBOUTIFG = 0b0000000000010110,
            /// Correctable FRAM bit error detection
            CBDIFG = 0b0000000000011000,
            /// FRAM write protection detection
            WPROT = 0b0000000000011010,
            /// LEA time-out fault
            LEATO = 0b0000000000011100,
            /// LEA command fault
            LEACMD = 0b0000000000011110,
        }
    }
    /// Reset Vector Generator
    r RSTIV @ 0x1e: u16 = 0_0 {
        /// Reset interrupt vector
        RSTIV: 0..15 = enum RSTIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Brownout
            BOR = 0b0000000000000010,
            /// RSTIFG RST/NMI
            RSTNMI = 0b0000000000000100,
            /// PMMSWBOR software BOR
            PMMSWBOR = 0b0000000000000110,
            /// LPMx.5 wakeup
            LPM5WU = 0b0000000000001000,
            /// Security violation
            SECYV = 0b0000000000001010,
            /// Reserved
            RSTIV_12 = 0b0000000000001100,
            /// SVSHIFG SVSH event
            SVSHIFG = 0b0000000000001110,
            /// Reserved
            RSTIV_16 = 0b0000000000010000,
            /// Reserved
            RSTIV_18 = 0b0000000000010010,
            /// PMMSWPOR software POR
            PMMSWPOR = 0b0000000000010100,
            /// WDTIFG watchdog timeout
            WDTIFG = 0b0000000000010110,
            /// WDTPW watchdog password violation
            WDTPW = 0b0000000000011000,
            /// FRCTLPW password violation
            FRCTLPW = 0b0000000000011010,
            /// Uncorrectable FRAM bit error detection
            UBDIFG = 0b0000000000011100,
            /// Peripheral area fetch
            PERF = 0b0000000000011110,
            /// PMM password violation
            PMMPW = 0b0000000000100000,
            /// MPU password violation
            MPUPW = 0b0000000000100010,
            /// CS password violation
            CSPW = 0b0000000000100100,
            /// MPUSEGPIFG encapsulated IP memory segment violation
            MPUSEGPIFG = 0b0000000000100110,
            /// MPUSEGIIFG information memory segment violation
            MPUSEGIIFG = 0b0000000000101000,
            /// MPUSEG1IFG segment 1 memory violation
            MPUSEG1IFG = 0b0000000000101010,
            /// MPUSEG2IFG segment 2 memory violation
            MPUSEG2IFG = 0b0000000000101100,
            /// MPUSEG3IFG segment 3 memory violation
            MPUSEG3IFG = 0b0000000000101110,
        }
    }
}
