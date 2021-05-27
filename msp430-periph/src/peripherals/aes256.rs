//! AES256

utils::periph! {
    /// AES256
    AES256;
    /// AES Accelerator Control Register 0
    rw CTL0 @ 0x00: u16 = 0_0 {
        /// AES operation
        OP: 0..1 = enum OP {
            /// Encryption
            OP_0 = 0b00,
            /// Decryption. The provided key is the same key used for encryption
            OP_1 = 0b01,
            /// Generate first round key required for decryption
            OP_2 = 0b10,
            /// Decryption. The provided key is the first round key required for decryption
            OP_3 = 0b11,
        }
        /// AES key length
        KL: 2..3 = enum KL {
            /// AES128. The key size is 128 bit
            _128 = 0b00,
            /// AES192. The key size is 192 bit.
            _192 = 0b01,
            /// AES256. The key size is 256 bit
            _256 = 0b10,
        }
        /// AES cipher mode select
        CM: 5..6 = enum CM {
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
        SWRST: 7 = enum SWRST {
            /// No reset
            SWRST_0 = 0b0,
            /// Reset AES accelerator module
            RESET = 0b1,
        }
        /// AES ready interrupt flag
        RDYIFG: 8 = enum RDYIFG {
            /// No interrupt pending
            RDYIFG_0 = 0b0,
            /// Interrupt pending
            RDYIFG_1 = 0b1,
        }
        /// AES error flag
        ERRFG: 11 = enum ERRFG {
            /// No error
            ERRFG_0 = 0b0,
            /// Error occurred
            ERRFG_1 = 0b1,
        }
        /// AES ready interrupt enable
        RDYIE: 12 = enum RDYIE {
            /// Interrupt disabled
            DISABLE = 0b0,
            /// Interrupt enabled
            ENABLE = 0b1,
        }
        /// AES cipher mode enable
        CMEN: 15 = enum CMEN {
            /// No DMA triggers are generated
            DISABLE = 0b0,
            /// DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated
            ENABLE = 0b1,
        }
    }
    /// AES Accelerator Control Register 1
    rw CTL1 @ 0x02: u16 = 0_0 {
        /// Cipher Block Counter
        BLKCNT: 0..7 = struct BLKCNT(u16);
    }
    /// AES Accelerator Status Register
    rw STAT @ 0x04: u16 = 0_0 {
        /// AES accelerator module busy
        BUSY: 0 = enum BUSY {
            /// Not busy
            IDLE = 0b0,
            /// Busy
            BUSY = 0b1,
        }
        /// All 16 bytes written to AESAKEY
        KEYWR: 1 = enum KEYWR {
            /// Not all bytes written
            KEYWR_0 = 0b0,
            /// All bytes written
            KEYWR_1 = 0b1,
        }
        /// All 16 bytes written to AESADIN, AESAXDIN or AESAXIN
        DINWR: 2 = enum DINWR {
            /// Not all bytes written
            DINWR_0 = 0b0,
            /// All bytes written
            DINWR_1 = 0b1,
        }
        /// All 16 bytes read from AESADOUT
        DOUTRD: 3 = enum DOUTRD {
            /// Not all bytes read
            DOUTRD_0 = 0b0,
            /// All bytes read
            DOUTRD_1 = 0b1,
        }
        /// Bytes written via AESAKEY for AESKL=00, half-words written via AESAKEY
        KEYCNT: 4..7 = struct KEYCNT(u16);
        /// Bytes written via AESADIN, AESAXDIN or AESAXIN
        DINCNT: 8..11 = struct DINCNT(u16);
        /// Bytes read via AESADOUT
        DOUTCNT: 12..15 = struct DOUTCNT(u16);
    }
    /// AES Accelerator Key Register
    rw KEY @ 0x06: u16 = 0_0 {
        /// AES key byte n when AESAKEY is written as half-word
        KEY0: 0..7 = struct KEY0(u16);
        /// AES key byte n+1 when AESAKEY is written as half-word
        KEY1: 8..15 = struct KEY1(u16);
    }
    /// AES Accelerator Data In Register
    rw DIN @ 0x08: u16 = 0_0 {
        /// AES data in byte n when AESADIN is written as half-word
        DIN0: 0..7 = struct DIN0(u16);
        /// AES data in byte n+1 when AESADIN is written as half-word
        DIN1: 8..15 = struct DIN1(u16);
    }
    /// AES Accelerator Data Out Register
    rw DOUT @ 0x0a: u16 = 0_0 {
        /// AES data out byte n when AESADOUT is read as half-word
        DOUT0: 0..7 = struct DOUT0(u16);
        /// AES data out byte n+1 when AESADOUT is read as half-word
        DOUT1: 8..15 = struct DOUT1(u16);
    }
    /// AES Accelerator XORed Data In Register
    rw XDIN @ 0x0c: u16 = 0_0 {
        /// AES data in byte n when AESAXDIN is written as half-word
        XDIN0: 0..7 = struct XDIN0(u16);
        /// AES data in byte n+1 when AESAXDIN is written as half-word
        XDIN1: 8..15 = struct XDIN1(u16);
    }
    /// AES Accelerator XORed Data In Register
    rw XIN @ 0x0e: u16 = 0_0 {
        /// AES data in byte n when AESAXIN is written as half-word
        XIN0: 0..7 = struct XIN0(u16);
        /// AES data in byte n+1 when AESAXIN is written as half-word
        XIN1: 8..15 = struct XIN1(u16);
    }
}
