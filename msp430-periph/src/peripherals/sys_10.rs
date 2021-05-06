//! SYS

utils::periph! {
    /// SYS
    SYS;
    /// System Control
    rw SYSCTL @ 0x00: u16 = 0_0 {
        /// RAM-based interrupt vectors
        SYSRIVECT: 0..0 = enum SYSRIVECT {
            /// Interrupt vectors generated with end address TOP of lower 64K FRAM FFFFh
            FRAM = 0b0,
            /// Interrupt vectors generated with end address TOP of RAM, when RAM available
            RAM = 0b1,
        }
        /// PMM access protect
        SYSPMMPE: 2..2 = enum SYSPMMPE {
            /// Access from anywhere in memory
            DIS = 0b0,
            /// Access only from the BSL segments
            EN = 0b1,
        }
        /// BSL entry indication
        SYSBSLIND: 4..4 = enum SYSBSLIND {
            /// No BSL entry sequence detected
            CLR = 0b0,
            /// BSL entry sequence detected
            SET = 0b1,
        }
        /// Dedicated JTAG pins enable
        SYSJTAGPIN: 5..5 = enum SYSJTAGPIN {
            /// Shared JTAG pins (JTAG mode selectable using SBW sequence)
            SHARED = 0b0,
            /// Dedicated JTAG pins (explicit 4-wire JTAG mode selection)
            DEDICATED = 0b1,
        }
    }
    /// Bootloader Configuration
    rw SYSBSLC @ 0x02: u16 = 0_0 {
        /// RAM assigned to BSL
        SYSBSLR: 2..2 = enum SYSBSLR {
            /// No RAM assigned to BSL area
            NORAM = 0b0,
            /// Lowest 16 bytes of RAM assigned to BSL
            RAM = 0b1,
        }
        /// Bootstrap loader memory disable for the size covered in SYSBSLSIZE
        SYSBSLOFF: 14..14 = enum SYSBSLOFF {
            /// BSL memory is addressed when this area is read.
            ON = 0b0,
            /// BSL memory behaves like vacant memory. Reads cause 3FFFh to be read. Fetches cause JMP $ to be executed.
            OFF = 0b1,
        }
        /// Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit.
        SYSBSLPE: 15..15 = enum SYSBSLPE {
            /// Area not protected. Read, program, and erase of BSL memory is possible.
            NOTPROT = 0b0,
            /// Area protected
            PROT = 0b1,
        }
    }
    /// JTAG Mailbox Control
    rw SYSJMBC @ 0x06: u16 = 0_0 {
        /// Incoming JTAG Mailbox 0 flag
        JMBIN0FG: 0..0 = enum JMBIN0FG {
            /// JMBI0 has no new data
            NODAT = 0b0,
            /// JMBI0 has new data available
            NEWDAT = 0b1,
        }
        /// Incoming JTAG Mailbox 1 flag
        JMBIN1FG: 1..1 = enum JMBIN1FG {
            /// JMBI1 has no new data
            NODAT = 0b0,
            /// JMBI1 has new data available
            NEWDAT = 0b1,
        }
        /// Outgoing JTAG Mailbox 0 flag
        JMBOUT0FG: 2..2 = enum JMBOUT0FG {
            /// JMBO0 is not ready to receive new data
            BUSY = 0b0,
            /// JMBO0 is ready to receive new data
            READY = 0b1,
        }
        /// Outgoing JTAG Mailbox 1 flag
        JMBOUT1FG: 3..3 = enum JMBOUT1FG {
            /// JMBO1 is not ready to receive new data
            BUSY = 0b0,
            /// JMBO1 is ready to receive new data
            READY = 0b1,
        }
        /// Operation mode of JMB
        JMBMODE: 4..4 = enum JMBMODE {
            /// 16-bit transfers using JMBO0 and JMBI0 only
            _16BIT = 0b0,
            /// 32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1
            _32BIT = 0b1,
        }
        /// Incoming JTAG Mailbox 0 flag auto-clear disable
        JMBCLR0OFF: 6..6 = enum JMBCLR0OFF {
            /// JMBIN0FG cleared on read of JMB0IN register
            CLRORD = 0b0,
            /// JMBIN0FG cleared by software
            CLRBSW = 0b1,
        }
        /// Incoming JTAG Mailbox 1 flag auto-clear disable
        JMBCLR1OFF: 7..7 = enum JMBCLR1OFF {
            /// JMBIN1FG cleared on read of JMB1IN register
            CLRORD = 0b0,
            /// JMBIN1FG cleared by software
            CLRBSW = 0b1,
        }
    }
    /// JTAG Mailbox Input 0
    r SYSJMBI0 @ 0x08: u16 = 0_0 {
        /// JTAG mailbox incoming message low byte
        SYSJMBI0_MSGLO: 0..7 = struct SYSJMBI0_MSGLO(u16);
        /// JTAG mailbox incoming message high byte
        SYSJMBI0_MSGHI: 8..15 = struct SYSJMBI0_MSGHI(u16);
    }
    /// JTAG Mailbox Input 1
    r SYSJMBI1 @ 0x0a: u16 = 0_0 {
        /// JTAG mailbox incoming message low byte
        SYSJMBI1_MSGLO: 0..7 = struct SYSJMBI1_MSGLO(u16);
        /// JTAG mailbox incoming message high byte
        SYSJMBI1_MSGHI: 8..15 = struct SYSJMBI1_MSGHI(u16);
    }
    /// JTAG Mailbox Output 0
    rw SYSJMBO0 @ 0x0c: u16 = 0_0 {
        /// JTAG mailbox outgoing message low byte
        SYSJMBO0_MSGLO: 0..7 = struct SYSJMBO0_MSGLO(u16);
        /// JTAG mailbox outgoing message high byte
        SYSJMBO0_MSGHI: 8..15 = struct SYSJMBO0_MSGHI(u16);
    }
    /// JTAG Mailbox Output 1
    rw SYSJMBO1 @ 0x0e: u16 = 0_0 {
        /// JTAG mailbox outgoing message low byte
        SYSJMBO1_MSGLO: 0..7 = struct SYSJMBO1_MSGLO(u16);
        /// JTAG mailbox outgoing message high byte
        SYSJMBO1_MSGHI: 8..15 = struct SYSJMBO1_MSGHI(u16);
    }
    /// User NMI Vector Generator
    r SYSUNIV @ 0x1a: u16 = 0_0 {
        /// User NMI vector
        SYSUNIV: 0..15 = enum SYSUNIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// NMIFG NMI pin or SVSH event
            NMIIFG = 0b0000000000000010,
            /// OFIFG oscillator fault
            OFIFG = 0b0000000000000100,
        }
    }
    /// System NMI Vector Generator
    r SYSSNIV @ 0x1c: u16 = 0_0 {
        /// System NMI vector
        SYSSNIV: 0..15 = enum SYSSNIVField {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// SVS low-power reset entry
            SVSLIFG = 0b0000000000000010,
            /// Uncorrectable FRAM bit error detection
            UBDIFG = 0b0000000000000100,
            /// Reserved
            NONE1 = 0b0000000000000110,
            /// Reserved
            NONE2 = 0b0000000000001000,
            /// Reserved
            NONE3 = 0b0000000000001010,
            /// Reserved
            NONE4 = 0b0000000000001100,
            /// Reserved
            NONE5 = 0b0000000000001110,
            /// Reserved
            NONE6 = 0b0000000000010000,
            /// VMAIFG Vacant memory access
            VMAIFG = 0b0000000000010010,
            /// JMBINIFG JTAG mailbox input
            JMBINIFG = 0b0000000000010100,
            /// JMBOUTIFG JTAG mailbox output
            JMBOUTIFG = 0b0000000000010110,
            /// Correctable FRAM bit error detection
            CBDIFG = 0b0000000000011000,
        }
    }
    /// Reset Vector Generator
    r SYSRSTIV @ 0x1e: u16 = 0_0 {
        /// Reset interrupt vector
        SYSRSTIV: 0..15 = enum SYSRSTIVField {
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
            SYSRSTIV_12 = 0b0000000000001100,
            /// SVSHIFG SVSH event
            SVSHIFG = 0b0000000000001110,
            /// Reserved
            SYSRSTIV_16 = 0b0000000000010000,
            /// Reserved
            SYSRSTIV_18 = 0b0000000000010010,
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
            /// Reserved
            SYSRSTIV_34 = 0b0000000000100010,
            /// FLL unlock (PUC)
            FLLUL = 0b0000000000100100,
        }
    }
    /// System Configuration 0
    rw SYSCFG0 @ 0x20: u16 = 0_0 {
        /// Program FRAM write protection
        PFWP: 0..0 = enum PFWP {
            /// Program FRAM write enable
            WEN = 0b0,
            /// Program FRAM write protected (not writable)
            WPROT = 0b1,
        }
        /// FRWPPW password.
        FRWPPW: 8..15 = struct FRWPPW(u16);
        /// Data FRAM write protection
        DFWP: 1..1 = enum DFWP {
            /// Data FRAM write enable
            WEN = 0b0,
            /// Data FRAM write protected (not writable)
            WPROT = 0b1,
        }
        /// Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution
        FRWPOA: 2..7 = struct FRWPOA(u16);
    }
    /// System Configuration 1
    rw SYSCFG1 @ 0x22: u16 = 0_0 {
        /// Infrared enable
        IREN: 0..0 = enum IREN {
            /// Infrared function disabled
            DIS = 0b0,
            /// Infrared function enabled
            EN = 0b1,
        }
        /// Infrared polarity select
        IRPSEL: 1..1 = enum IRPSEL {
            /// Normal polarity
            NORM = 0b0,
            /// Inverted polarity
            INV = 0b1,
        }
        /// Infrared mode select
        IRMSEL: 2..2 = enum IRMSEL {
            /// ASK mode
            ASK = 0b0,
            /// FSK mode
            FSK = 0b1,
        }
        /// Infrared data source select
        IRDSSEL: 3..3 = enum IRDSSEL {
            /// From hardware peripherals upon device configuration
            HW = 0b0,
            /// From IRDATA bit
            IRDATA = 0b1,
        }
        /// Infrared data
        IRDATA: 4..4 = enum IRDATA {
            /// Infrared data logic 0
            LOW = 0b0,
            /// Infrared data logic 1
            HIGH = 0b1,
        }
        /// Captivate Conversion triggered Source Selection
        SYNCSEL: 6..7 = enum SYNCSEL {
            /// External source is selected
            SYNCSEL_0 = 0b00,
            /// ADC as the source is selected
            SYNCSEL_1 = 0b01,
            /// Comparator as the source is selected
            SYNCSEL_2 = 0b10,
            /// Reserved
            SYNCSEL_3 = 0b11,
        }
    }
    /// System Configuration 2
    rw SYSCFG2 @ 0x24: u16 = 0_0 {
        /// RTC clock selection
        RTCCKSEL: 10..10 = enum RTCCKSEL {
            /// SMCLK is selected
            RTCCKSEL_0 = 0b0,
            /// ACLK is selected
            RTCCKSEL_1 = 0b1,
        }
        /// eUSCI_B0 remapping source selection
        USCIB0RMP: 11..11 = enum USCIB0RMP {
            /// Default function. See the device-specific data sheet for details.
            USCIB0RMP_0 = 0b0,
            /// Remapped function. See the device-specific data sheet for details.
            USCIB0RMP_1 = 0b1,
        }
        /// TB0OUTH trigger source selection
        TB0TRGSEL: 15..15 = enum TB0TRGSEL {
            /// Internal source is selected
            TB0TRGSEL_0 = 0b0,
            /// External source is selected
            TB0TRGSEL_1 = 0b1,
        }
    }
    /// System Configuration 3
    rw SYSCFG3 @ 0x26: u16 = 0_0 {
        /// eUSCI_A0 remapping source selection
        USCIA0RMP: 0..0 = enum USCIA0RMP {
            /// Default function. See the device-specific data sheet for details.
            USCIA0RMP_0 = 0b0,
            /// Remapped function. See the device-specific data sheet for details.
            USCIA0RMP_1 = 0b1,
        }
        /// Timer2_A3 remapping source selection
        TA2RMP: 2..2 = enum TA2RMP {
            /// Default function. See the device-specific data sheet for details.
            TA2RMP_0 = 0b0,
            /// Remapped function. See the device-specific data sheet for details.
            TA2RMP_1 = 0b1,
        }
        /// Timer3_A3 remapping source selection
        TA3RMP: 3..3 = enum TA3RMP {
            /// Default function. See the device-specific data sheet for details.
            TA3RMP_0 = 0b0,
            /// Remapped function. See the device-specific data sheet for details.
            TA3RMP_1 = 0b1,
        }
        /// eUSCI_B1 remapping source selection
        USCIB1RMP: 4..4 = enum USCIB1RMP {
            /// Default function. See the device-specific data sheet for details.
            USCIB1RMP_0 = 0b0,
            /// Remapped function. See the device-specific data sheet for details.
            USCIB1RMP_1 = 0b1,
        }
    }
}
