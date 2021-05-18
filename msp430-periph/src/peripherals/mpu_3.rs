//! MPU

utils::periph! {
    /// MPU
    MPU;
    /// Memory Protection Unit Control 0
    rw MPUCTL0 @ 0x00: u16 = 0_0 {
        /// MPU Enable
        MPUENA: 0 = enum MPUENA {
            /// Disabled
            DISABLE = 0b0,
            /// Enabled
            ENABLE = 0b1,
        }
        /// MPU Lock
        MPULOCK: 1 = enum MPULOCK {
            /// Open
            OPEN = 0b0,
            /// Locked
            LOCK = 0b1,
        }
        /// Enable NMI Event if a Segment violation
        MPUSEGIE: 4 = enum MPUSEGIE {
            /// Segment violation interrupt disabled
            DISABLE = 0b0,
            /// Segment violation interrupt enabled
            ENABLE = 0b1,
        }
        /// MPU Password
        MPUPW: 8..15 = struct MPUPW(u16);
    }
    /// Memory Protection Unit Control 1
    rw MPUCTL1 @ 0x02: u16 = 0_0 {
        /// Main Memory Segment 1 Violation Interrupt Flag
        MPUSEG1IFG: 0 = enum MPUSEG1IFG {
            /// No interrupt pending
            MPUSEG1IFG_0 = 0b0,
            /// Interrupt pending
            MPUSEG1IFG_1 = 0b1,
        }
        /// Main Memory Segment 2 Violation Interrupt Flag
        MPUSEG2IFG: 1 = enum MPUSEG2IFG {
            /// No interrupt pending
            MPUSEG2IFG_0 = 0b0,
            /// Interrupt pending
            MPUSEG2IFG_1 = 0b1,
        }
        /// Main Memory Segment 3 Violation Interrupt Flag
        MPUSEG3IFG: 2 = enum MPUSEG3IFG {
            /// No interrupt pending
            MPUSEG1IFG_0 = 0b0,
            /// Interrupt pending
            MPUSEG1IFG_1 = 0b1,
        }
        /// User Information Memory Violation Interrupt Flag
        MPUSEGIIFG: 3 = enum MPUSEGIIFG {
            /// No interrupt pending
            MPUSEGIIFG_0 = 0b0,
            /// Interrupt pending
            MPUSEGIIFG_1 = 0b1,
        }
        /// IP Encapsulation Access Violation Interrupt Flag
        MPUSEGIPIFG: 4 = enum MPUSEGIPIFG {
            /// No interrupt pending
            MPUSEG1IFG_0 = 0b0,
            /// Interrupt pending
            MPUSEG1IFG_1 = 0b1,
        }
    }
    /// Memory Protection Unit Segmentation Border 2 Register
    rw MPUSEGB2 @ 0x04: u16 = 0_0 {
        /// Memory Protection Unit Segmentation Border 2 Register
        MPUSEGB2: 0..15 = struct MPUSEGB2Field(u16);
    }
    /// Memory Protection Unit Segmentation Border 1 Register
    rw MPUSEGB1 @ 0x06: u16 = 0_0 {
        /// Memory Protection Unit Segmentation Border 1 Register
        MPUSEGB1: 0..15 = struct MPUSEGB1Field(u16);
    }
    /// Memory Protection Unit Segmentation Access Management Register
    rw MPUSAM @ 0x08: u16 = 0_0 {
        /// MPU Main Memory Segment 1 Read Enable
        MPUSEG1RE: 0 = enum MPUSEG1RE {
            /// Read on Main Memory Segment 1 causes a violation if MPUSEG1WE = MPUSEG1XE = 0
            DISABLE = 0b0,
            /// Read on Main Memory Segment 1 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 1 Write Enable
        MPUSEG1WE: 1 = enum MPUSEG1WE {
            /// Write on Main Memory Segment 1 causes a violation
            DISABLE = 0b0,
            /// Write on Main Memory Segment 1 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 1 Execute Enable
        MPUSEG1XE: 2 = enum MPUSEG1XE {
            /// Execute code on Main Memory Segment 1 causes a violation
            DISABLE = 0b0,
            /// Execute code on Main Memory Segment 1 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 1 Violation Select
        MPUSEG1VS: 3 = enum MPUSEG1VS {
            /// Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a SNMI if enabled by MPUSEGIE = 1
            MPUSEG1VS_0 = 0b0,
            /// Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a PUC
            MPUSEG1VS_1 = 0b1,
        }
        /// MPU Main Memory Segment 2 Read Enable
        MPUSEG2RE: 4 = enum MPUSEG2RE {
            /// Read on Main Memory Segment 2 causes a violation if MPUSEG2WE = MPUSEG2XE = 0
            DISABLE = 0b0,
            /// Read on Main Memory Segment 2 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 2 Write Enable
        MPUSEG2WE: 5 = enum MPUSEG2WE {
            /// Write on Main Memory Segment 2 causes a violation
            DISABLE = 0b0,
            /// Write on Main Memory Segment 2 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 2 Execute Enable
        MPUSEG2XE: 6 = enum MPUSEG2XE {
            /// Execute code on Main Memory Segment 2 causes a violation
            DISABLE = 0b0,
            /// Execute code on Main Memory Segment 2 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 2 Violation Select
        MPUSEG2VS: 7 = enum MPUSEG2VS {
            /// Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a SNMI if enabled by MPUSEGIE = 1
            MPUSEG2VS_0 = 0b0,
            /// Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a PUC
            MPUSEG2VS_1 = 0b1,
        }
        /// MPU Main Memory Segment 3 Read Enable
        MPUSEG3RE: 8 = enum MPUSEG3RE {
            /// Read on Main Memory Segment 3 causes a violation if MPUSEG3WE = MPUSEG3XE = 0
            DISABLE = 0b0,
            /// Read on Main Memory Segment 3 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 3 Write Enable
        MPUSEG3WE: 9 = enum MPUSEG3WE {
            /// Write on Main Memory Segment 3 causes a violation
            DISABLE = 0b0,
            /// Write on Main Memory Segment 3 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 3 Execute Enable
        MPUSEG3XE: 10 = enum MPUSEG3XE {
            /// Execute code on Main Memory Segment 3 causes a violation
            DISABLE = 0b0,
            /// Execute code on Main Memory Segment 3 is allowed
            ENABLE = 0b1,
        }
        /// MPU Main Memory Segment 3 Violation Select
        MPUSEG3VS: 11 = enum MPUSEG3VS {
            /// Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a SNMI if enabled by MPUSEGIE = 1
            MPUSEG3VS_0 = 0b0,
            /// Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a PUC
            MPUSEG3VS_1 = 0b1,
        }
        /// MPU User Information Memory Segment Read Enable
        MPUSEGIRE: 12 = enum MPUSEGIRE {
            /// Read on User Information Memory causes a violation if MPUSEGIWE=MPUSEGIXE=0
            DISABLE = 0b0,
            /// Read on User Information Memory is allowed
            ENABLE = 0b1,
        }
        /// MPU User Information Memory Segment Write Enable.
        MPUSEGIWE: 13 = enum MPUSEGIWE {
            /// Write on User Information Memory causes a violation
            DISABLE = 0b0,
            /// Write on User Information Memory is allowed
            ENABLE = 0b1,
        }
        /// MPU User Information Memory Segment Execute Enable
        MPUSEGIXE: 14 = enum MPUSEGIXE {
            /// Execute code on User Information Memory causes a violation
            DISABLE = 0b0,
            /// Execute code on User Information Memory is allowed
            ENABLE = 0b1,
        }
        /// MPU User Information Memory Segment Violation Select
        MPUSEGIVS: 15 = enum MPUSEGIVS {
            /// Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a SNMI if enabled by MPUSEGIE =1
            MPUSEGIVS_0 = 0b0,
            /// Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a PUC
            MPUSEGIVS_1 = 0b1,
        }
    }
    /// Memory Protection Unit IP Control 0 Register
    rw MPUIPC0 @ 0x0a: u16 = 0_0 {
        /// MPU IP Encapsulation segment Violation Select
        MPUIPVS: 5 = enum MPUIPVS {
            /// Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a SNMI if enabled by MPUSEGIE = 1
            MPUIPVS_0 = 0b0,
            /// Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a PUC
            MPUIPVS_1 = 0b1,
        }
        /// MPU IP Encapsulation Enable
        MPUIPENA: 6 = enum MPUIPENA {
            /// Disabled
            DISABLE = 0b0,
            /// Enabled
            ENABLE = 0b1,
        }
        /// MPU IP Encapsulation Lock
        MPUIPLOCK: 7 = enum MPUIPLOCK {
            /// Open
            OPEN = 0b0,
            /// Locked
            LOCK = 0b1,
        }
    }
    /// Memory Protection Unit IP Encapsulation Segment Border 2 Register
    rw MPUIPSEGB2 @ 0x0c: u16 = 0_0 {
        /// Memory Protection Unit IP Encapsulation Segment Border 2 Register
        MPUIPSEGB2: 0..15 = struct MPUIPSEGB2Field(u16);
    }
    /// Memory Protection Unit IP Encapsulation Segment Border 1 Register
    rw MPUIPSEGB1 @ 0x0e: u16 = 0_0 {
        /// Memory Protection Unit IP Encapsulation Segment Border 1 Register
        MPUIPSEGB1: 0..15 = struct MPUIPSEGB1Field(u16);
    }
}
