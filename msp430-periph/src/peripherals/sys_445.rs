//! SYS

utils::periph! {
    /// SYS
    SYS;
    /// System Control
    rw SYSCTL @ 0x00: u16 = 0_0 {
        /// RAM-based interrupt vectors
        SYSRIVECT: 0 = enum SYSRIVECT {
            /// Interrupt vectors generated with end address TOP of lower 64K FRAM FFFFh
            FRAM = 0b0,
            /// Interrupt vectors generated with end address TOP of RAM, when RAM available
            RAM = 0b1,
        }
        /// PMM access protect
        SYSPMMPE: 2 = enum SYSPMMPE {
            /// Access from anywhere in memory
            SYSPMMPE_0 = 0b0,
            /// Access only from the BSL segments
            SYSPMMPE_1 = 0b1,
        }
        /// BSL entry indication
        SYSBSLIND: 4 = enum SYSBSLIND {
            /// No BSL entry sequence detected
            SYSBSLIND_0 = 0b0,
            /// BSL entry sequence detected
            SYSBSLIND_1 = 0b1,
        }
        /// Dedicated JTAG pins enable
        SYSJTAGPIN: 5 = enum SYSJTAGPIN {
            /// Shared JTAG pins (JTAG mode selectable using SBW sequence)
            SHARED = 0b0,
            /// Dedicated JTAG pins (explicit 4-wire JTAG mode selection)
            DEDICATED = 0b1,
        }
    }
    /// Bootstrap Loader Configuration Register
    rw SYSBSLC @ 0x02: u16 = 0_0 {
        /// RAM assigned to BSL
        SYSBSLR: 2 = enum SYSBSLR {
            /// No RAM assigned to BSL area
            SYSBSLR_0 = 0b0,
            /// Lowest 16 bytes of RAM assigned to BSL
            SYSBSLR_1 = 0b1,
        }
        /// Bootstrap loader memory disable for the size covered in SYSBSLSIZE
        SYSBSLOFF: 14 = enum SYSBSLOFF {
            /// BSL memory is addressed when this area is read.
            SYSBSLOFF_0 = 0b0,
            /// BSL memory behaves like vacant memory. Reads cause 3FFFh to be read. Fetches cause JMP $ to be executed.
            SYSBSLOFF_1 = 0b1,
        }
        /// Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit.
        SYSBSLPE: 15 = enum SYSBSLPE {
            /// Area not protected. Read, program, and erase of BSL memory is possible.
            SYSBSLPE_0 = 0b0,
            /// Area protected
            SYSBSLPE_1 = 0b1,
        }
    }
    /// JTAG Mailbox Control
    rw SYSJMBC @ 0x06: u16 = 0_0 {
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
    rw SYSJMBI0 @ 0x08: u16 = 0_0 {
        /// JTAG mailbox incoming message low byte
        SYSJMBI0_MSGLO: 0..7 = struct SYSJMBI0_MSGLO(u16);
        /// JTAG mailbox incoming message high byte
        SYSJMBI0_MSGHI: 8..15 = struct SYSJMBI0_MSGHI(u16);
    }
    /// JTAG Mailbox Input 1 Register
    rw SYSJMBI1 @ 0x0a: u16 = 0_0 {
        /// JTAG mailbox incoming message low byte
        SYSJMBI1_MSGLO: 0..7 = struct SYSJMBI1_MSGLO(u16);
        /// JTAG mailbox incoming message high byte
        SYSJMBI1_MSGHI: 8..15 = struct SYSJMBI1_MSGHI(u16);
    }
    /// JTAG Mailbox Output
    rw SYSJMBO0 @ 0x0c: u16 = 0_0 {
        /// JTAG mailbox outgoing message low byte
        SYSJMBO0_MSGLO: 0..7 = struct SYSJMBO0_MSGLO(u16);
        /// JTAG mailbox outgoing message high byte
        SYSJMBO0_MSGHI: 8..15 = struct SYSJMBO0_MSGHI(u16);
    }
    /// JTAG Mailbox Output 1 Register
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
            /// NMIIFG NMI pin
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
            /// FRAM Access Time Error
            ACCTEIFG = 0b0000000000000110,
            /// Reserved
            SYSSNIV_8 = 0b0000000000001000,
            /// Reserved
            SYSSNIV_10 = 0b0000000000001010,
            /// Reserved
            SYSSNIV_12 = 0b0000000000001100,
            /// Reserved
            SYSSNIV_14 = 0b0000000000001110,
            /// Reserved
            SYSSNIV_16 = 0b0000000000010000,
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
    /// System Configuration Register 0
    rw SYSCFG0 @ 0x20: u16 = 0_0 {
        /// Program FRAM write protection
        PFWP: 0 = enum PFWP {
            /// Program FRAM write enable
            PFWP_0 = 0b0,
            /// Program FRAM write protected (not writable)
            PFWP_1 = 0b1,
        }
        /// Data FRAM write protection
        DFWP: 1 = enum DFWP {
            /// Data FRAM write enable
            DFWP_0 = 0b0,
            /// Data FRAM write protected (not writable)
            DFWP_1 = 0b1,
        }
        /// FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h
        FRWPPW: 8..15 = struct FRWPPW(u16);
        /// Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution
        FRWPOA: 2..7 = struct FRWPOA(u16);
    }
    /// System Configuration Register 1
    rw SYSCFG1 @ 0x22: u16 = 0_0 {
        /// Infrared enable
        IREN: 0 = enum IREN {
            /// Infrared function disabled
            IREN_0 = 0b0,
            /// Infrared function enabled
            IREN_1 = 0b1,
        }
        /// Infrared polarity select
        IRPSEL: 1 = enum IRPSEL {
            /// Normal polarity
            IRPSEL_0 = 0b0,
            /// Inverted polarity
            IRPSEL_1 = 0b1,
        }
        /// Infrared mode select
        IRMSEL: 2 = enum IRMSEL {
            /// FSK mode
            IRMSEL_0 = 0b0,
            /// ASK mode
            IRMSEL_1 = 0b1,
        }
        /// Infrared data source select
        IRDSSEL: 3 = enum IRDSSEL {
            /// From hardware peripherals upon device configuration
            IRDSSEL_0 = 0b0,
            /// From IRDATA bit
            IRDSSEL_1 = 0b1,
        }
        /// Infrared data
        IRDATA: 4 = enum IRDATA {
            /// Infrared data logic 0
            IRDATA_0 = 0b0,
            /// Infrared data logic 1
            IRDATA_1 = 0b1,
        }
        /// Captivate Conversion triggered Source Selection
        SYNCSEL: 6..7 = enum SYNCSEL {
            /// External source is selected
            SYNCSEL_0 = 0b00,
            /// ADC as the source is selected
            SYNCSEL_1 = 0b01,
            /// internal source is selected
            SYNCSEL_2 = 0b10,
            /// Reserved
            SYNCSEL_3 = 0b11,
        }
    }
    /// System Configuration Register 2
    rw SYSCFG2 @ 0x24: u16 = 0_0 {
        /// ADC input A0 pin select
        ADCPCTL0: 0 = enum ADCPCTL0 {
            /// ADC input A0 disabled
            ADCPCTL0_0 = 0b0,
            /// ADC input A0 enabled
            ADCPCTL0_1 = 0b1,
        }
        /// ADC input A1 pin select
        ADCPCTL1: 1 = enum ADCPCTL1 {
            /// ADC input A1 disabled
            ADCPCTL1_0 = 0b0,
            /// ADC input A1 enabled
            ADCPCTL1_1 = 0b1,
        }
        /// ADC input A2 pin select
        ADCPCTL2: 2 = enum ADCPCTL2 {
            /// ADC input A2 disabled
            ADCPCTL2_0 = 0b0,
            /// ADC input A2 enabled
            ADCPCTL2_1 = 0b1,
        }
        /// ADC input A3 pin select
        ADCPCTL3: 3 = enum ADCPCTL3 {
            /// ADC input A3 disabled
            ADCPCTL3_0 = 0b0,
            /// ADC input A3 enabled
            ADCPCTL3_1 = 0b1,
        }
        /// ADC input A4 pin select
        ADCPCTL4: 4 = enum ADCPCTL4 {
            /// ADC input A4 disabled
            ADCPCTL4_0 = 0b0,
            /// ADC input A4 enabled
            ADCPCTL4_1 = 0b1,
        }
        /// ADC input A5 pin select
        ADCPCTL5: 5 = enum ADCPCTL5 {
            /// ADC input A5 disabled
            ADCPCTL5_0 = 0b0,
            /// ADC input A5 enabled
            ADCPCTL5_1 = 0b1,
        }
        /// ADC input A6 pin select
        ADCPCTL6: 6 = enum ADCPCTL6 {
            /// ADC input A6 disabled
            ADCPCTL6_0 = 0b0,
            /// ADC input A6 enabled
            ADCPCTL6_1 = 0b1,
        }
        /// ADC input A7 pin select
        ADCPCTL7: 7 = enum ADCPCTL7 {
            /// ADC input A7 disabled
            ADCPCTL7_0 = 0b0,
            /// ADC input A7 enabled
            ADCPCTL7_1 = 0b1,
        }
        /// ADC input A8 pin select
        ADCPCTL8: 8 = enum ADCPCTL8 {
            /// ADC input A8 disabled
            ADCPCTL8_0 = 0b0,
            /// ADC input A8 enabled
            ADCPCTL8_1 = 0b1,
        }
        /// ADC input A9 pin select
        ADCPCTL9: 9 = enum ADCPCTL9 {
            /// ADC input A9 disabled
            ADCPCTL9_0 = 0b0,
            /// ADC input A9 enabled
            ADCPCTL9_1 = 0b1,
        }
        /// eUSCIB Remapping source selection , please refer to device specific for details
        USCIBRMP: 11 = enum USCIBRMP {
            /// P1.x is selected, please refer to device specific for details
            USCIBRMP_0 = 0b0,
            /// other port is selected, please refer to device specific for details
            USCIBRMP_1 = 0b1,
        }
        /// RTC clock selection
        RTCCKSEL: 10 = enum RTCCKSEL {
            /// SMCLK is selected
            RTC_SMCLK = 0b0,
            /// ACLK is selected
            RTC_ACLK = 0b1,
        }
    }
    /// System Configuration Register 3
    rw SYSCFG3 @ 0x26: u16 = 0_0 {
        /// eUSCIA remapping source selection, please refer to device specific for details
        USCIARMP: 0 = enum USCIARMP {
            /// P1.x is selected, please refer to device specific for details
            USCIARMP_0 = 0b0,
            /// other port is selected, please refer to device specific for details
            USCIARMP_1 = 0b1,
        }
    }
}
