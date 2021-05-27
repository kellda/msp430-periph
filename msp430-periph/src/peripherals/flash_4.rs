//! Flash

utils::periph! {
    /// Flash
    Flash;
    /// FLASH Control 1
    rw CTL1 @ 0x00: u16 = 0_0 {
        /// Enable bit for Flash segment erase
        ERASE: 1 = struct ERASE(bool);
        /// Enable bit for Flash mass erase
        MERAS: 2 = struct MERAS(bool);
        /// Enable Erase Interrupts
        EEI: 3 = struct EEI(bool);
        /// Enable Emergency Interrupt Exit
        EEIEX: 4 = struct EEIEX(bool);
        /// Enable bit for Flash write
        WRT: 6 = struct WRT(bool);
        /// Enable bit for Flash segment write
        BLKWRT: 7 = struct BLKWRT(bool);
    }
    /// FLASH Control 2
    rw CTL2 @ 0x02: u16 = 0_0 {
        /// Divide Flash clock by 1 to 64 using FN0 to FN5 according to:
        N0: 0 = struct N0(bool);
        /// 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1
        N1: 1 = struct N1(bool);
        /// FN2
        N2: 2 = struct N2(bool);
        /// FN3
        N3: 3 = struct N3(bool);
        /// FN4
        N4: 4 = struct N4(bool);
        /// FN5
        N5: 5 = struct N5(bool);
        /// Flash clock select 0 */        /* to distinguish from USART SSELx
        SSEL: 6..7 = enum SSEL {
            /// Flash clock select: 0 - ACLK
            SSEL_0 = 0b00,
            /// Flash clock select: 1 - MCLK
            SSEL_1 = 0b01,
            /// Flash clock select: 2 - SMCLK
            SSEL_2 = 0b10,
            /// Flash clock select: 3 - SMCLK
            SSEL_3 = 0b11,
        }
    }
    /// FLASH Control 3
    rw CTL3 @ 0x04: u16 = 0_0 {
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
        /// Segment A Lock bit: read = 1 - Segment is locked (read only)
        LOCKA: 6 = struct LOCKA(bool);
        /// Last Program or Erase failed
        FAIL: 7 = struct FAIL(bool);
    }
}
