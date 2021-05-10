//! ICC

utils::periph! {
    /// ICC
    ICC;
    /// ICCSC
    rw ICCSC @ 0x00: u16 = 0_0 {
        /// Current Interrupt Compare Mask of virtual stack specifies the current ICM at the top of virtual stack If ICM[1:0] is less than the priority level (ILSRx[1:0]) of the new interrupt, the corresponding source is sent to the CPU. Note that the ICMC is the element stack that the stack pointer is pointing to.
        ICMC: 0..1 = struct ICMC(u16);
        /// Virtual stack full flag This bit indicates whether or not the virtual stack is full. It is automatically updated when the stack is pushed or popped.
        VSFFLG: 4 = enum VSFFLG {
            /// ICCMVS register is not full
            VSFFLG_0 = 0b0,
            /// ICCMVS register is full
            VSFFLG_1 = 0b1,
        }
        /// Virtual stack empty flag.This bit indicates whether or not the virtual stack is empty. It is automatically updated when the stack is pushed or popped.
        VSEFLG: 5 = enum VSEFLG {
            /// Stack has valid data
            VSEFLG_0 = 0b0,
            /// Stack has no valid data
            VSEFLG_1 = 0b1,
        }
        /// ICC enable
        ICCEN: 7 = enum ICCEN {
            /// ICC module disabled
            ICCEN_0 = 0b0,
            /// ICC module enabled
            ICCEN_1 = 0b1,
        }
    }
    /// ICCMVS
    r ICCMVS @ 0x02: u16 = 0_0 {
        /// Interrupt compare mask virtual stack position 0 This field is the virtual stack register for ICM0.
        ICM0: 0..1 = struct ICM0(u16);
        /// Interrupt compare mask virtual stack position 1 This field is the virtual stack register for ICM1.
        ICM1: 2..3 = struct ICM1(u16);
        /// Interrupt compare mask virtual stack position 3 This field is the virtual stack register for ICM3.
        ICM3: 6..7 = struct ICM3(u16);
        /// MVS stack pointer indicate register
        MVSSP: 8..10 = enum MVSSP {
            /// 000b = Stack empty
            MVSSP_0 = 0b000,
            /// 001b = ICM0 affected
            MVSSP_1 = 0b001,
            /// 010b = ICM0 and ICM1 affected
            MVSSP_2 = 0b010,
            /// 011b = ICM0, ICM1, and ICM2 affected
            MVSSP_3 = 0b011,
            /// 100b = ICM0, ICM1, ICM2, and ICM3 affected. Also means the stack is full.
            MVSSP_4 = 0b100,
        }
        /// Interrupt compare mask virtual stack position 2 This field is the virtual stack register for ICM2.
        ICM2: 4..5 = struct ICM2(u16);
    }
    /// ICCILSR0
    rw ICCILSR0 @ 0x04: u16 = 0_0 {
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.
        ILSR0: 0..1 = struct ILSR0(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.
        ILSR1: 2..3 = struct ILSR1(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.
        ILSR2: 4..5 = struct ILSR2(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.
        ILSR3: 6..7 = struct ILSR3(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.
        ILSR4: 8..9 = struct ILSR4(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.
        ILSR5: 10..11 = struct ILSR5(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.
        ILSR6: 12..13 = struct ILSR6(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.
        ILSR7: 14..15 = struct ILSR7(u16);
    }
    /// ICCILSR1
    rw ICCILSR1 @ 0x06: u16 = 0_0 {
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR8: 0..1 = struct ILSR8(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR9: 2..3 = struct ILSR9(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR10: 4..5 = struct ILSR10(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit
        ILSR11: 6..7 = struct ILSR11(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR12: 8..9 = struct ILSR12(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR13: 10..11 = struct ILSR13(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR14: 12..13 = struct ILSR14(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR15: 14..15 = struct ILSR15(u16);
    }
    /// ICCILSR2
    rw ICCILSR2 @ 0x08: u16 = 0_0 {
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR16: 0..1 = struct ILSR16(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit
        ILSR17: 2..3 = struct ILSR17(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR18: 4..5 = struct ILSR18(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR19: 6..7 = struct ILSR19(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR20: 8..9 = struct ILSR20(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR21: 10..11 = struct ILSR21(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each
        ILSR22: 12..13 = struct ILSR22(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each
        ILSR23: 14..15 = struct ILSR23(u16);
    }
    /// ICCILSR3
    rw ICCILSR3 @ 0x0a: u16 = 0_0 {
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR24: 0..1 = struct ILSR24(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR25: 2..3 = struct ILSR25(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR26: 4..5 = struct ILSR26(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR27: 6..7 = struct ILSR27(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR28: 8..9 = struct ILSR28(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR29: 10..11 = struct ILSR29(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR30: 12..13 = struct ILSR30(u16);
        /// Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.
        ILSR31: 14..15 = struct ILSR31(u16);
    }
}
