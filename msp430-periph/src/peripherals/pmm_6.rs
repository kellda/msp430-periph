//! PMM  Power Management System

utils::periph! {
    /// PMM  Power Management System
    PMM;
    /// RF PMM Control 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// RF MM Software brownout reset. BOR. Bit: 2
        SWBOR: 2 = struct SWBOR(bool);
        /// RF PMM Software power-on reset. POR. Bit: 3
        SWPOR: 3 = struct SWPOR(bool);
        /// RF PMM Enable voltage regulator for FRAM vddf permanently
        EN_VF_REG: 4 = struct EN_VF_REG(bool);
        /// RF PMM Enable the battery switch
        EN_BATSWITCH: 5 = struct EN_BATSWITCH(bool);
        /// RF PMM Enable voltage regulator for digital core. VDD
        EN_V_DOUB: 6 = struct EN_V_DOUB(bool);
    }
    /// RF PMM Control 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// RF PMM Low active reset signal of VDDF Bit: 0
        NRESET_VF: 0 = struct NRESET_VF(bool);
        /// RF PMM Low active reset signal of VDDB Bit: 1
        NRESET_VB: 1 = struct NRESET_VB(bool);
        /// RF PMM Low active reset signal of VDDR Bit: 2
        NRESET_VR: 2 = struct NRESET_VR(bool);
        /// RF PMM Low active reset signal of VDDH Bit: 3
        NRESET_VH: 3 = struct NRESET_VH(bool);
        /// RF PMM Low active reset signal of VDD2X Bit: 4
        NRESET_V2X: 4 = struct NRESET_V2X(bool);
        /// RF PMM indicates position of the bulk switch Bit: 8
        BW_VR_VBN: 8 = struct BW_VR_VBN(bool);
    }
    /// RF PPM interrupt enable register
    rw RFMMIE @ 0x04: u16 = 0_0 {
        /// RF PMM Interrupt enable voltage regulator VDDF Bit: 0
        IEVF: 0 = struct IEVF(bool);
        /// RF PMM Interrupt enable voltage regulator VDDB Bit: 1
        IEVB: 1 = struct IEVB(bool);
        /// RF PMM Interrupt enable voltage regulator VDDR Bit: 2
        IEVR: 2 = struct IEVR(bool);
        /// RF PMM Interrupt enable voltage regulator VDDH Bit: 3
        IEVH: 3 = struct IEVH(bool);
        /// RF PMM Interrupt enable voltage regulator VDD2X Bit: 4
        IEV2X: 4 = struct IEV2X(bool);
    }
    /// RF PMM Interrupt Flag
    rw IFG @ 0x06: u16 = 0_0 {
        /// RF PMM Interrupt flag voltage regulator VDDF Bit: 0
        IFGVF: 0 = struct IFGVF(bool);
        /// RF PMM Interrupt flag voltage regulator VDDB Bit: 1
        IFGVB: 1 = struct IFGVB(bool);
        /// RF PMM Interrupt flag voltage regulator VDDR Bit: 2
        IFGVR: 2 = struct IFGVR(bool);
        /// RF PMM Interrupt flag voltage regulator VDDH Bit: 3
        IFGVH: 3 = struct IFGVH(bool);
        /// RF PMM Interrupt flag voltage regulator VDD2X Bit: 4
        IFGV2X: 4 = struct IFGV2X(bool);
    }
    /// RF PMM Interrupt vector regiser
    rw IV @ 0x08: u16 = 0_0 {
        /// RF PMM Interrupt vector regiser
        IV: 0..15 = struct IVField(u16);
    }
}
