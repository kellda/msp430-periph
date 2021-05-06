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
        /// AES Key length Bit: 0
        AESKL: 2..3 = enum AESKL {
            /// AES Key length: AES128
            AESKL_0 = 0b00,
            /// AES Key length: AES192
            AESKL_1 = 0b01,
            /// AES Key length: AES256
            AESKL_2 = 0b10,
        }
        /// AES Trigger Select
        AESTRIG: 4 = struct AESTRIG(bool);
        /// AES Cipher mode select Bit: 0
        AESCM: 5..6 = enum AESCM {
            /// AES Cipher mode select: ECB
            AESCM_0 = 0b00,
            /// AES Cipher mode select: CBC
            AESCM_1 = 0b01,
            /// AES Cipher mode select: OFB
            AESCM_2 = 0b10,
            /// AES Cipher mode select: CFB
            AESCM_3 = 0b11,
        }
        /// AES Software Reset
        AESSWRST: 7 = struct AESSWRST(bool);
        /// AES ready interrupt flag
        AESRDYIFG: 8 = struct AESRDYIFG(bool);
        /// AES Error Flag
        AESERRFG: 11 = struct AESERRFG(bool);
        /// AES ready interrupt enable
        AESRDYIE: 12 = struct AESRDYIE(bool);
        /// AES DMA cipher mode enable
        AESCMEN: 15 = struct AESCMEN(bool);
    }
    /// AES accelerator control register 1
    rw AESACTL1 @ 0x02: u16 = 0_0 {
        /// AES Cipher Block Counter Bit: 0
        AESBLKCNT0: 0 = struct AESBLKCNT0(bool);
        /// AES Cipher Block Counter Bit: 1
        AESBLKCNT1: 1 = struct AESBLKCNT1(bool);
        /// AES Cipher Block Counter Bit: 2
        AESBLKCNT2: 2 = struct AESBLKCNT2(bool);
        /// AES Cipher Block Counter Bit: 3
        AESBLKCNT3: 3 = struct AESBLKCNT3(bool);
        /// AES Cipher Block Counter Bit: 4
        AESBLKCNT4: 4 = struct AESBLKCNT4(bool);
        /// AES Cipher Block Counter Bit: 5
        AESBLKCNT5: 5 = struct AESBLKCNT5(bool);
        /// AES Cipher Block Counter Bit: 6
        AESBLKCNT6: 6 = struct AESBLKCNT6(bool);
        /// AES Cipher Block Counter Bit: 7
        AESBLKCNT7: 7 = struct AESBLKCNT7(bool);
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
    /// AES accelerator XORed data in register
    rw AESAXDIN @ 0x0c: u16 = 0_0 {
        /// AES accelerator XORed data in register
        AESAXDIN: 0..15 = struct AESAXDINField(u16);
    }
    /// AES accelerator XORed data in register (no trigger)
    rw AESAXIN @ 0x0e: u16 = 0_0 {
        /// AES accelerator XORed data in register (no trigger)
        AESAXIN: 0..15 = struct AESAXINField(u16);
    }
}
