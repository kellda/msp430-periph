//! AES256

utils::periph! {
    /// AES256
    AES256;
    /// AES Accelerator Control Register 0
    rw AESACTL0 @ 0x00: u16 = 0_0 {
        /// AES operation
        AESOP: 0..1 = enum AESOP {
            /// Encryption
            AESOP_0 = 0b00,
            /// Decryption. The provided key is the same key used for encryption
            AESOP_1 = 0b01,
            /// Generate first round key required for decryption
            AESOP_2 = 0b10,
            /// Decryption. The provided key is the first round key required for decryption
            AESOP_3 = 0b11,
        }
        /// AES key length
        AESKL: 2..3 = enum AESKL {
            /// AES128. The key size is 128 bit
            _128 = 0b00,
            /// AES192. The key size is 192 bit.
            _192 = 0b01,
            /// AES256. The key size is 256 bit
            _256 = 0b10,
        }
        /// AES cipher mode select
        AESCM: 5..6 = enum AESCM {
            /// ECB
            ECB = 0b00,
            /// CBC
            CBC = 0b01,
            /// OFB
            OFB = 0b10,
            /// CFB
            CFB = 0b11,
        }
        /// AES software reset
        AESSWRST: 7 = enum AESSWRST {
            /// No reset
            AESSWRST_0 = 0b0,
            /// Reset AES accelerator module
            RESET = 0b1,
        }
        /// AES ready interrupt flag
        AESRDYIFG: 8 = enum AESRDYIFG {
            /// No interrupt pending
            AESRDYIFG_0 = 0b0,
            /// Interrupt pending
            AESRDYIFG_1 = 0b1,
        }
        /// AES error flag
        AESERRFG: 11 = enum AESERRFG {
            /// No error
            AESERRFG_0 = 0b0,
            /// Error occurred
            AESERRFG_1 = 0b1,
        }
        /// AES ready interrupt enable
        AESRDYIE: 12 = enum AESRDYIE {
            /// Interrupt disabled
            DISABLE = 0b0,
            /// Interrupt enabled
            ENABLE = 0b1,
        }
        /// AES cipher mode enable
        AESCMEN: 15 = enum AESCMEN {
            /// No DMA triggers are generated
            DISABLE = 0b0,
            /// DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated
            ENABLE = 0b1,
        }
    }
    /// AES Accelerator Control Register 1
    rw AESACTL1 @ 0x02: u16 = 0_0 {
        /// Cipher Block Counter
        AESBLKCNT: 0..7 = struct AESBLKCNT(u16);
    }
    /// AES Accelerator Status Register
    rw AESASTAT @ 0x04: u16 = 0_0 {
        /// AES accelerator module busy
        AESBUSY: 0 = enum AESBUSY {
            /// Not busy
            IDLE = 0b0,
            /// Busy
            BUSY = 0b1,
        }
        /// All 16 bytes written to AESAKEY
        AESKEYWR: 1 = enum AESKEYWR {
            /// Not all bytes written
            AESKEYWR_0 = 0b0,
            /// All bytes written
            AESKEYWR_1 = 0b1,
        }
        /// All 16 bytes written to AESADIN, AESAXDIN or AESAXIN
        AESDINWR: 2 = enum AESDINWR {
            /// Not all bytes written
            AESDINWR_0 = 0b0,
            /// All bytes written
            AESDINWR_1 = 0b1,
        }
        /// All 16 bytes read from AESADOUT
        AESDOUTRD: 3 = enum AESDOUTRD {
            /// Not all bytes read
            AESDOUTRD_0 = 0b0,
            /// All bytes read
            AESDOUTRD_1 = 0b1,
        }
        /// Bytes written via AESAKEY for AESKL=00, half-words written via AESAKEY
        AESKEYCNT: 4..7 = struct AESKEYCNT(u16);
        /// Bytes written via AESADIN, AESAXDIN or AESAXIN
        AESDINCNT: 8..11 = struct AESDINCNT(u16);
        /// Bytes read via AESADOUT
        AESDOUTCNT: 12..15 = struct AESDOUTCNT(u16);
    }
    /// AES Accelerator Key Register
    rw AESAKEY @ 0x06: u16 = 0_0 {
        /// AES key byte n when AESAKEY is written as half-word
        AESKEY0: 0..7 = struct AESKEY0(u16);
        /// AES key byte n+1 when AESAKEY is written as half-word
        AESKEY1: 8..15 = struct AESKEY1(u16);
    }
    /// AES Accelerator Data In Register
    rw AESADIN @ 0x08: u16 = 0_0 {
        /// AES data in byte n when AESADIN is written as half-word
        AESDIN0: 0..7 = struct AESDIN0(u16);
        /// AES data in byte n+1 when AESADIN is written as half-word
        AESDIN1: 8..15 = struct AESDIN1(u16);
    }
    /// AES Accelerator Data Out Register
    rw AESADOUT @ 0x0a: u16 = 0_0 {
        /// AES data out byte n when AESADOUT is read as half-word
        AESDOUT0: 0..7 = struct AESDOUT0(u16);
        /// AES data out byte n+1 when AESADOUT is read as half-word
        AESDOUT1: 8..15 = struct AESDOUT1(u16);
    }
    /// AES Accelerator XORed Data In Register
    rw AESAXDIN @ 0x0c: u16 = 0_0 {
        /// AES data in byte n when AESAXDIN is written as half-word
        AESXDIN0: 0..7 = struct AESXDIN0(u16);
        /// AES data in byte n+1 when AESAXDIN is written as half-word
        AESXDIN1: 8..15 = struct AESXDIN1(u16);
    }
    /// AES Accelerator XORed Data In Register
    rw AESAXIN @ 0x0e: u16 = 0_0 {
        /// AES data in byte n when AESAXIN is written as half-word
        AESXIN0: 0..7 = struct AESXIN0(u16);
        /// AES data in byte n+1 when AESAXIN is written as half-word
        AESXIN1: 8..15 = struct AESXIN1(u16);
    }
}
