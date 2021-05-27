//! AES Accelerator

utils::periph! {
    /// AES Accelerator
    AESAccelerator;
    /// AES accelerator control register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// AES Operation Bit: 0
        OP: 0..1 = enum OP {
            /// AES Operation: Encrypt
            OP_0 = 0b00,
            /// AES Operation: Decrypt (same Key)
            OP_1 = 0b01,
            /// AES Operation: Generate first round Key
            OP_2 = 0b10,
            /// AES Operation: Decrypt (first round Key)
            OP_3 = 0b11,
        }
        /// AES Software Reset
        SWRST: 7 = struct SWRST(bool);
        /// AES ready interrupt flag
        RDYIFG: 8 = struct RDYIFG(bool);
        /// AES Error Flag
        ERRFG: 11 = struct ERRFG(bool);
        /// AES ready interrupt enable
        RDYIE: 12 = struct RDYIE(bool);
    }
    /// AES accelerator status register
    rw STAT @ 0x04: u16 = 0_0 {
        /// AES Busy
        BUSY: 0 = struct BUSY(bool);
        /// AES All 16 bytes written to AESAKEY
        KEYWR: 1 = struct KEYWR(bool);
        /// AES All 16 bytes written to AESADIN
        DINWR: 2 = struct DINWR(bool);
        /// AES All 16 bytes read from AESADOUT
        DOUTRD: 3 = struct DOUTRD(bool);
        /// AES Bytes written via AESAKEY Bit: 0
        KEYCNT0: 4 = struct KEYCNT0(bool);
        /// AES Bytes written via AESAKEY Bit: 1
        KEYCNT1: 5 = struct KEYCNT1(bool);
        /// AES Bytes written via AESAKEY Bit: 2
        KEYCNT2: 6 = struct KEYCNT2(bool);
        /// AES Bytes written via AESAKEY Bit: 3
        KEYCNT3: 7 = struct KEYCNT3(bool);
        /// AES Bytes written via AESADIN Bit: 0
        DINCNT0: 8 = struct DINCNT0(bool);
        /// AES Bytes written via AESADIN Bit: 1
        DINCNT1: 9 = struct DINCNT1(bool);
        /// AES Bytes written via AESADIN Bit: 2
        DINCNT2: 10 = struct DINCNT2(bool);
        /// AES Bytes written via AESADIN Bit: 3
        DINCNT3: 11 = struct DINCNT3(bool);
        /// AES Bytes read via AESADOUT Bit: 0
        DOUTCNT0: 12 = struct DOUTCNT0(bool);
        /// AES Bytes read via AESADOUT Bit: 1
        DOUTCNT1: 13 = struct DOUTCNT1(bool);
        /// AES Bytes read via AESADOUT Bit: 2
        DOUTCNT2: 14 = struct DOUTCNT2(bool);
        /// AES Bytes read via AESADOUT Bit: 3
        DOUTCNT3: 15 = struct DOUTCNT3(bool);
    }
    /// AES accelerator key register
    rw KEY @ 0x06: u16 = 0_0 {
        /// AES accelerator key register
        KEY: 0..15 = struct KEYField(u16);
    }
    /// AES accelerator data in register
    rw DIN @ 0x08: u16 = 0_0 {
        /// AES accelerator data in register
        DIN: 0..15 = struct DINField(u16);
    }
    /// AES accelerator data out register
    rw DOUT @ 0x0a: u16 = 0_0 {
        /// AES accelerator data out register
        DOUT: 0..15 = struct DOUTField(u16);
    }
}
