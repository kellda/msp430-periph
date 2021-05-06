//! PMM  Power Management System

utils::periph! {
    /// PMM  Power Management System
    PMMPowerManagementSystem;
    /// RF PMM Control 0
    rw RFPMMCTL0 @ 0x00: u16 = 0_0 {
        /// RF MM Software brownout reset. BOR. Bit: 2
        RFPMMSWBOR: 2 = struct RFPMMSWBOR(bool);
        /// RF PMM Software power-on reset. POR. Bit: 3
        RFPMMSWPOR: 3 = struct RFPMMSWPOR(bool);
        /// RF PMM Enable voltage regulator for FRAM vddf permanently
        RFPMM_EN_VF_REG: 4 = struct RFPMM_EN_VF_REG(bool);
        /// RF PMM Enable the battery switch
        RFPMM_EN_BATSWITCH: 5 = struct RFPMM_EN_BATSWITCH(bool);
        /// RF PMM Enable voltage regulator for digital core. VDD
        RFPMM_EN_V_DOUB: 6 = struct RFPMM_EN_V_DOUB(bool);
    }
    /// RF PMM Control 1
    rw RFPMMCTL1 @ 0x02: u16 = 0_0 {
        /// RF PMM Low active reset signal of VDDF Bit: 0
        RFPMM_NRESET_VF: 0 = struct RFPMM_NRESET_VF(bool);
        /// RF PMM Low active reset signal of VDDB Bit: 1
        RFPMM_NRESET_VB: 1 = struct RFPMM_NRESET_VB(bool);
        /// RF PMM Low active reset signal of VDDR Bit: 2
        RFPMM_NRESET_VR: 2 = struct RFPMM_NRESET_VR(bool);
        /// RF PMM Low active reset signal of VDDH Bit: 3
        RFPMM_NRESET_VH: 3 = struct RFPMM_NRESET_VH(bool);
        /// RF PMM Low active reset signal of VDD2X Bit: 4
        RFPMM_NRESET_V2X: 4 = struct RFPMM_NRESET_V2X(bool);
        /// RF PMM indicates position of the bulk switch Bit: 8
        RFPMM_BW_VR_VBN: 8 = struct RFPMM_BW_VR_VBN(bool);
    }
    /// RF PPM interrupt enable register
    rw RFMMIE @ 0x04: u16 = 0_0 {
        /// RF PMM Interrupt enable voltage regulator VDDF Bit: 0
        RFPMMIEVF: 0 = struct RFPMMIEVF(bool);
        /// RF PMM Interrupt enable voltage regulator VDDB Bit: 1
        RFPMMIEVB: 1 = struct RFPMMIEVB(bool);
        /// RF PMM Interrupt enable voltage regulator VDDR Bit: 2
        RFPMMIEVR: 2 = struct RFPMMIEVR(bool);
        /// RF PMM Interrupt enable voltage regulator VDDH Bit: 3
        RFPMMIEVH: 3 = struct RFPMMIEVH(bool);
        /// RF PMM Interrupt enable voltage regulator VDD2X Bit: 4
        RFPMMIEV2X: 4 = struct RFPMMIEV2X(bool);
    }
    /// RF PMM Interrupt Flag
    rw RFPMMIFG @ 0x06: u16 = 0_0 {
        /// RF PMM Interrupt flag voltage regulator VDDF Bit: 0
        RFPMMIFGVF: 0 = struct RFPMMIFGVF(bool);
        /// RF PMM Interrupt flag voltage regulator VDDB Bit: 1
        RFPMMIFGVB: 1 = struct RFPMMIFGVB(bool);
        /// RF PMM Interrupt flag voltage regulator VDDR Bit: 2
        RFPMMIFGVR: 2 = struct RFPMMIFGVR(bool);
        /// RF PMM Interrupt flag voltage regulator VDDH Bit: 3
        RFPMMIFGVH: 3 = struct RFPMMIFGVH(bool);
        /// RF PMM Interrupt flag voltage regulator VDD2X Bit: 4
        RFPMMIFGV2X: 4 = struct RFPMMIFGV2X(bool);
    }
    /// RF PMM Interrupt vector regiser
    rw RFPMMIV @ 0x08: u16 = 0_0 {
        /// RF PMM Interrupt vector regiser
        RFPMMIV: 0..15 = struct RFPMMIVField(u16);
    }
}
