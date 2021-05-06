//! AES Accelerator

utils::periph! {
    /// AES Accelerator
    AESAccelerator;
    /// AES accelerator control register 0
    rw AESACTL0 @ 0x00: u16 = 0_0 {
        /// AES Operation Bit: 0
        AESOP: 0..1 = enum AESOP {
            /// AES Operation: Encrypt
            AESOP_0 = 0b00,
            /// AES Operation: Decrypt (same Key)
            AESOP_1 = 0b01,
            /// AES Operation: Generate first round Key
            AESOP_2 = 0b10,
            /// AES Operation: Decrypt (first round Key)
            AESOP_3 = 0b11,
        }
        /// AES Software Reset
        AESSWRST: 7 = struct AESSWRST(bool);
        /// AES ready interrupt flag
        AESRDYIFG: 8 = struct AESRDYIFG(bool);
        /// AES Error Flag
        AESERRFG: 11 = struct AESERRFG(bool);
        /// AES ready interrupt enable
        AESRDYIE: 12 = struct AESRDYIE(bool);
    }
    /// AES accelerator status register
    rw AESASTAT @ 0x04: u16 = 0_0 {
        /// AES Busy
        AESBUSY: 0 = struct AESBUSY(bool);
        /// AES All 16 bytes written to AESAKEY
        AESKEYWR: 1 = struct AESKEYWR(bool);
        /// AES All 16 bytes written to AESADIN
        AESDINWR: 2 = struct AESDINWR(bool);
        /// AES All 16 bytes read from AESADOUT
        AESDOUTRD: 3 = struct AESDOUTRD(bool);
        /// AES Bytes written via AESAKEY Bit: 0
        AESKEYCNT0: 4 = struct AESKEYCNT0(bool);
        /// AES Bytes written via AESAKEY Bit: 1
        AESKEYCNT1: 5 = struct AESKEYCNT1(bool);
        /// AES Bytes written via AESAKEY Bit: 2
        AESKEYCNT2: 6 = struct AESKEYCNT2(bool);
        /// AES Bytes written via AESAKEY Bit: 3
        AESKEYCNT3: 7 = struct AESKEYCNT3(bool);
        /// AES Bytes written via AESADIN Bit: 0
        AESDINCNT0: 8 = struct AESDINCNT0(bool);
        /// AES Bytes written via AESADIN Bit: 1
        AESDINCNT1: 9 = struct AESDINCNT1(bool);
        /// AES Bytes written via AESADIN Bit: 2
        AESDINCNT2: 10 = struct AESDINCNT2(bool);
        /// AES Bytes written via AESADIN Bit: 3
        AESDINCNT3: 11 = struct AESDINCNT3(bool);
        /// AES Bytes read via AESADOUT Bit: 0
        AESDOUTCNT0: 12 = struct AESDOUTCNT0(bool);
        /// AES Bytes read via AESADOUT Bit: 1
        AESDOUTCNT1: 13 = struct AESDOUTCNT1(bool);
        /// AES Bytes read via AESADOUT Bit: 2
        AESDOUTCNT2: 14 = struct AESDOUTCNT2(bool);
        /// AES Bytes read via AESADOUT Bit: 3
        AESDOUTCNT3: 15 = struct AESDOUTCNT3(bool);
    }
    /// AES accelerator key register
    rw AESAKEY @ 0x06: u16 = 0_0 {
        /// AES accelerator key register
        AESAKEY: 0..15 = struct AESAKEYField(u16);
    }
    /// AES accelerator data in register
    rw AESADIN @ 0x08: u16 = 0_0 {
        /// AES accelerator data in register
        AESADIN: 0..15 = struct AESADINField(u16);
    }
    /// AES accelerator data out register
    rw AESADOUT @ 0x0a: u16 = 0_0 {
        /// AES accelerator data out register
        AESADOUT: 0..15 = struct AESADOUTField(u16);
    }
}
