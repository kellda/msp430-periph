//! MPU

utils::periph! {
    /// MPU
    MPU;
    /// Memory Protection Unit Control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// MPU Enable
        ENA: 0 = enum ENA {
            /// Disabled
            DISABLE = 0b0,
            /// Enabled
            ENABLE = 0b1,
        }
        /// MPU Lock
        LOCK: 1 = enum LOCK {
            /// Open
            OPEN = 0b0,
            /// Locked
            LOCK = 0b1,
        }
        /// Enable NMI Event if a Segment violation
        SEGIE: 4 = enum SEGIE {
            /// Segment violation interrupt disabled
            DISABLE = 0b0,
            /// Segment violation interrupt enabled
            ENABLE = 0b1,
        }
        /// MPU Password
        PW: 8..15 = struct PW(u16);
    }
    /// Memory Protection Unit Control 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// Main Memory Segment 1 Violation Interrupt Flag
        SEG1IFG: 0 = enum SEG1IFG {
            /// No interrupt pending
            SEG1IFG_0 = 0b0,
            /// Interrupt pending
            SEG1IFG_1 = 0b1,
        }
        /// Main Memory Segment 2 Violation Interrupt Flag
        SEG2IFG: 1 = enum SEG2IFG {
            /// No interrupt pending
            SEG2IFG_0 = 0b0,
            /// Interrupt pending
            SEG2IFG_1 = 0b1,
        }
        /// Main Memory Segment 3 Violation Interrupt Flag
        SEG3IFG: 2 = enum SEG3IFG {
            /// No interrupt pending
            SEG1IFG_0 = 0b0,
            /// Interrupt pending
            SEG1IFG_1 = 0b1,
        }
        /// User Information Memory Violation Interrupt Flag
        SEGIIFG: 3 = enum SEGIIFG {
            /// No interrupt pending
            SEGIIFG_0 = 0b0,
            /// Interrupt pending
            SEGIIFG_1 = 0b1,
        }
        /// IP Encapsulation Access Violation Interrupt Flag
        SEGIPIFG: 4 = enum SEGIPIFG {
            /// No interrupt pending
            SEG1IFG_0 = 0b0,
            /// Interrupt pending
            SEG1IFG_1 = 0b1,
        }
    }
    /// Memory Protection Unit Segmentation Border 2 Register
    rw SEGB2 @ 0x04: u16 = 0_0 {
        /// Memory Protection Unit Segmentation Border 2 Register
        SEGB2: 0..15 = struct SEGB2Field(u16);
    }
    /// Memory Protection Unit Segmentation Border 1 Register
    rw SEGB1 @ 0x06: u16 = 0_0 {
        /// Memory Protection Unit Segmentation Border 1 Register
        SEGB1: 0..15 = struct SEGB1Field(u16);
    }
    /// Memory Protection Unit Segmentation Access Management Register
    rw SAM @ 0x08: u16 = 0_0 {
        /// MPU Main Memory Segment 1 Read Enable
        SEG1RE: 0 = enum SEG1RE {
            /// Read on Main Memory Segment 1 causes a violation if MPUSEG1WE = MPUSEG1XE = 0
            DISABLE = 0b0,
            /// Read on Main Memory Segment 1 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 1 Write Enable
        SEG1WE: 1 = enum SEG1WE {
            /// Write on Main Memory Segment 1 causes a violation
            DISABLE = 0b0,
            /// Write on Main Memory Segment 1 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 1 Execute Enable
        SEG1XE: 2 = enum SEG1XE {
            /// Execute code on Main Memory Segment 1 causes a violation
            DISABLE = 0b0,
            /// Execute code on Main Memory Segment 1 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 1 Violation Select
        SEG1VS: 3 = enum SEG1VS {
            /// Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a SNMI if enabled by MPUSEGIE = 1
            SEG1VS_0 = 0b0,
            /// Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a PUC
            SEG1VS_1 = 0b1,
        }
        /// MPU Main Memory Segment 2 Read Enable
        SEG2RE: 4 = enum SEG2RE {
            /// Read on Main Memory Segment 2 causes a violation if MPUSEG2WE = MPUSEG2XE = 0
            DISABLE = 0b0,
            /// Read on Main Memory Segment 2 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 2 Write Enable
        SEG2WE: 5 = enum SEG2WE {
            /// Write on Main Memory Segment 2 causes a violation
            DISABLE = 0b0,
            /// Write on Main Memory Segment 2 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 2 Execute Enable
        SEG2XE: 6 = enum SEG2XE {
            /// Execute code on Main Memory Segment 2 causes a violation
            DISABLE = 0b0,
            /// Execute code on Main Memory Segment 2 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 2 Violation Select
        SEG2VS: 7 = enum SEG2VS {
            /// Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a SNMI if enabled by MPUSEGIE = 1
            SEG2VS_0 = 0b0,
            /// Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a PUC
            SEG2VS_1 = 0b1,
        }
        /// MPU Main Memory Segment 3 Read Enable
        SEG3RE: 8 = enum SEG3RE {
            /// Read on Main Memory Segment 3 causes a violation if MPUSEG3WE = MPUSEG3XE = 0
            DISABLE = 0b0,
            /// Read on Main Memory Segment 3 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 3 Write Enable
        SEG3WE: 9 = enum SEG3WE {
            /// Write on Main Memory Segment 3 causes a violation
            DISABLE = 0b0,
            /// Write on Main Memory Segment 3 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 3 Execute Enable
        SEG3XE: 10 = enum SEG3XE {
            /// Execute code on Main Memory Segment 3 causes a violation
            DISABLE = 0b0,
            /// Execute code on Main Memory Segment 3 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 3 Violation Select
        SEG3VS: 11 = enum SEG3VS {
            /// Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a SNMI if enabled by MPUSEGIE = 1
            SEG3VS_0 = 0b0,
            /// Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a PUC
            SEG3VS_1 = 0b1,
        }
        /// MPU User Information Memory Segment Read Enable
        SEGIRE: 12 = enum SEGIRE {
            /// Read on User Information Memory causes a violation if MPUSEGIWE=MPUSEGIXE=0
            DISABLE = 0b0,
            /// Read on User Information Memory is allowed
            ENABLE = 0b1,
        }
        /// MPU User Information Memory Segment Write Enable.
        SEGIWE: 13 = enum SEGIWE {
            /// Write on User Information Memory causes a violation
            DISABLE = 0b0,
            /// Write on User Information Memory is allowed
            ENABLE = 0b1,
        }
        /// MPU User Information Memory Segment Execute Enable
        SEGIXE: 14 = enum SEGIXE {
            /// Execute code on User Information Memory causes a violation
            DISABLE = 0b0,
            /// Execute code on User Information Memory is allowed
            ENABLE = 0b1,
        }
        /// MPU User Information Memory Segment Violation Select
        SEGIVS: 15 = enum SEGIVS {
            /// Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a SNMI if enabled by MPUSEGIE =1
            SEGIVS_0 = 0b0,
            /// Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a PUC
            SEGIVS_1 = 0b1,
        }
    }
    /// Memory Protection Unit IP Control 0 Register
    rw IPC0 @ 0x0a: u16 = 0_0 {
        /// MPU IP Encapsulation segment Violation Select
        IPVS: 5 = enum IPVS {
            /// Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a SNMI if enabled by MPUSEGIE = 1
            IPVS_0 = 0b0,
            /// Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a PUC
            IPVS_1 = 0b1,
        }
        /// MPU IP Encapsulation Enable
        IPENA: 6 = enum IPENA {
            /// Disabled
            DISABLE = 0b0,
            /// Enabled
            ENABLE = 0b1,
        }
        /// MPU IP Encapsulation Lock
        IPLOCK: 7 = enum IPLOCK {
            /// Open
            OPEN = 0b0,
            /// Locked
            LOCK = 0b1,
        }
    }
    /// Memory Protection Unit IP Encapsulation Segment Border 2 Register
    rw IPSEGB2 @ 0x0c: u16 = 0_0 {
        /// Memory Protection Unit IP Encapsulation Segment Border 2 Register
        IPSEGB2: 0..15 = struct IPSEGB2Field(u16);
    }
    /// Memory Protection Unit IP Encapsulation Segment Border 1 Register
    rw IPSEGB1 @ 0x0e: u16 = 0_0 {
        /// Memory Protection Unit IP Encapsulation Segment Border 1 Register
        IPSEGB1: 0..15 = struct IPSEGB1Field(u16);
    }
}
