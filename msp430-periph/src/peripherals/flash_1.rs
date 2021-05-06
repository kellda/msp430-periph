//! Flash

utils::periph! {
    /// Flash
    Flash;
    /// FLASH Control 1
    rw FCTL1 @ 0x00: u16 = 0_0 {
        /// Enable bit for Flash segment erase
        ERASE: 1 = struct ERASE(bool);
        /// Enable bit for Flash mass erase
        MERAS: 2 = struct MERAS(bool);
        /// Enable bit for Flash write
        WRT: 6 = struct WRT(bool);
        /// Enable bit for Flash segment write
        BLKWRT: 7 = struct BLKWRT(bool);
    }
    /// FLASH Control 2
    rw FCTL2 @ 0x02: u16 = 0_0 {
        /// Divide Flash clock by 1 to 64 using FN0 to FN5 according to:
        FN0: 0 = struct FN0(bool);
        /// 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1
        FN1: 1 = struct FN1(bool);
        /// FN2
        FN2: 2 = struct FN2(bool);
        /// FN3
        FN3: 3 = struct FN3(bool);
        /// FN4
        FN4: 4 = struct FN4(bool);
        /// FN5
        FN5: 5 = struct FN5(bool);
        /// Flash clock select 0 */        /* to distinguish from USART SSELx
        FSSEL: 6..7 = enum FSSEL {
            /// Flash clock select: 0 - ACLK
            FSSEL_0 = 0b00,
            /// Flash clock select: 1 - MCLK
            FSSEL_1 = 0b01,
            /// Flash clock select: 2 - SMCLK
            FSSEL_2 = 0b10,
            /// Flash clock select: 3 - SMCLK
            FSSEL_3 = 0b11,
        }
    }
    /// FLASH Control 3
    rw FCTL3 @ 0x04: u16 = 0_0 {
        /// Flash busy: 1
        BUSY: 0 = struct BUSY(bool);
        /// Flash Key violation flag
        KEYV: 1 = struct KEYV(bool);
        /// Flash Access violation flag
        ACCVIFG: 2 = struct ACCVIFG(bool);
        /// Wait flag for segment write
        WAIT: 3 = struct WAIT(bool);
        /// Lock bit: 1 - Flash is locked (read only)
        LOCK: 4 = struct LOCK(bool);
        /// Flash Emergency Exit
        EMEX: 5 = struct EMEX(bool);
    }
}
