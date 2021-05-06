//! USB Operation

utils::periph! {
    /// USB Operation
    USBOperation;
    /// Input Endpoint_7: X/Y-buffer size
    rw USBIEPSIZXY_7 @ 0x7ff: u8 = 0_0 {
        /// Input Endpoint_7: X/Y-buffer size
        USBIEPSIZXY_7: 0..7 = struct USBIEPSIZXY_7Field(u8);
    }
    /// Input Endpoint_7: Y-byte count
    rw USBIEPBCTY_7 @ 0x7fe: u8 = 0_0 {
        /// Input Endpoint_7: Y-byte count
        USBIEPBCTY_7: 0..7 = struct USBIEPBCTY_7Field(u8);
    }
    /// Input Endpoint_7: Y-buffer base addr.
    rw USBIEPBBAY_7 @ 0x7fd: u8 = 0_0 {
        /// Input Endpoint_7: Y-buffer base addr.
        USBIEPBBAY_7: 0..7 = struct USBIEPBBAY_7Field(u8);
    }
    /// Input Endpoint_7: X-byte count
    rw USBIEPBCTX_7 @ 0x7fa: u8 = 0_0 {
        /// Input Endpoint_7: X-byte count
        USBIEPBCTX_7: 0..7 = struct USBIEPBCTX_7Field(u8);
    }
    /// Input Endpoint_7: X-buffer base addr.
    rw USBIEPBBAX_7 @ 0x7f9: u8 = 0_0 {
        /// Input Endpoint_7: X-buffer base addr.
        USBIEPBBAX_7: 0..7 = struct USBIEPBBAX_7Field(u8);
    }
    /// Input Endpoint_7: Configuration
    rw USBIEPCNF_7 @ 0x7f8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBIEPCNF_7_USBIIE: 2 = struct USBIEPCNF_7_USBIIE(bool);
        /// USB - Stall Condition
        USBIEPCNF_7_STALL: 3 = struct USBIEPCNF_7_STALL(bool);
        /// USB - Double Buffer Enable
        USBIEPCNF_7_DBUF: 4 = struct USBIEPCNF_7_DBUF(bool);
        /// USB - Toggle Bit
        USBIEPCNF_7_TOGGLE: 5 = struct USBIEPCNF_7_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBIEPCNF_7_UBME: 7 = struct USBIEPCNF_7_UBME(bool);
    }
    /// Input Endpoint_6: X/Y-buffer size
    rw USBIEPSIZXY_6 @ 0x7f7: u8 = 0_0 {
        /// Input Endpoint_6: X/Y-buffer size
        USBIEPSIZXY_6: 0..7 = struct USBIEPSIZXY_6Field(u8);
    }
    /// Input Endpoint_6: Y-byte count
    rw USBIEPBCTY_6 @ 0x7f6: u8 = 0_0 {
        /// Input Endpoint_6: Y-byte count
        USBIEPBCTY_6: 0..7 = struct USBIEPBCTY_6Field(u8);
    }
    /// Input Endpoint_6: Y-buffer base addr.
    rw USBIEPBBAY_6 @ 0x7f5: u8 = 0_0 {
        /// Input Endpoint_6: Y-buffer base addr.
        USBIEPBBAY_6: 0..7 = struct USBIEPBBAY_6Field(u8);
    }
    /// Input Endpoint_6: X-byte count
    rw USBIEPBCTX_6 @ 0x7f2: u8 = 0_0 {
        /// Input Endpoint_6: X-byte count
        USBIEPBCTX_6: 0..7 = struct USBIEPBCTX_6Field(u8);
    }
    /// Input Endpoint_6: X-buffer base addr.
    rw USBIEPBBAX_6 @ 0x7f1: u8 = 0_0 {
        /// Input Endpoint_6: X-buffer base addr.
        USBIEPBBAX_6: 0..7 = struct USBIEPBBAX_6Field(u8);
    }
    /// Input Endpoint_6: Configuration
    rw USBIEPCNF_6 @ 0x7f0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBIEPCNF_6_USBIIE: 2 = struct USBIEPCNF_6_USBIIE(bool);
        /// USB - Stall Condition
        USBIEPCNF_6_STALL: 3 = struct USBIEPCNF_6_STALL(bool);
        /// USB - Double Buffer Enable
        USBIEPCNF_6_DBUF: 4 = struct USBIEPCNF_6_DBUF(bool);
        /// USB - Toggle Bit
        USBIEPCNF_6_TOGGLE: 5 = struct USBIEPCNF_6_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBIEPCNF_6_UBME: 7 = struct USBIEPCNF_6_UBME(bool);
    }
    /// Input Endpoint_5: X/Y-buffer size
    rw USBIEPSIZXY_5 @ 0x7ef: u8 = 0_0 {
        /// Input Endpoint_5: X/Y-buffer size
        USBIEPSIZXY_5: 0..7 = struct USBIEPSIZXY_5Field(u8);
    }
    /// Input Endpoint_5: Y-byte count
    rw USBIEPBCTY_5 @ 0x7ee: u8 = 0_0 {
        /// Input Endpoint_5: Y-byte count
        USBIEPBCTY_5: 0..7 = struct USBIEPBCTY_5Field(u8);
    }
    /// Input Endpoint_5: Y-buffer base addr.
    rw USBIEPBBAY_5 @ 0x7ed: u8 = 0_0 {
        /// Input Endpoint_5: Y-buffer base addr.
        USBIEPBBAY_5: 0..7 = struct USBIEPBBAY_5Field(u8);
    }
    /// Input Endpoint_5: X-byte count
    rw USBIEPBCTX_5 @ 0x7ea: u8 = 0_0 {
        /// Input Endpoint_5: X-byte count
        USBIEPBCTX_5: 0..7 = struct USBIEPBCTX_5Field(u8);
    }
    /// Input Endpoint_5: X-buffer base addr.
    rw USBIEPBBAX_5 @ 0x7e9: u8 = 0_0 {
        /// Input Endpoint_5: X-buffer base addr.
        USBIEPBBAX_5: 0..7 = struct USBIEPBBAX_5Field(u8);
    }
    /// Input Endpoint_5: Configuration
    rw USBIEPCNF_5 @ 0x7e8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBIEPCNF_5_USBIIE: 2 = struct USBIEPCNF_5_USBIIE(bool);
        /// USB - Stall Condition
        USBIEPCNF_5_STALL: 3 = struct USBIEPCNF_5_STALL(bool);
        /// USB - Double Buffer Enable
        USBIEPCNF_5_DBUF: 4 = struct USBIEPCNF_5_DBUF(bool);
        /// USB - Toggle Bit
        USBIEPCNF_5_TOGGLE: 5 = struct USBIEPCNF_5_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBIEPCNF_5_UBME: 7 = struct USBIEPCNF_5_UBME(bool);
    }
    /// Input Endpoint_4: X/Y-buffer size
    rw USBIEPSIZXY_4 @ 0x7e7: u8 = 0_0 {
        /// Input Endpoint_4: X/Y-buffer size
        USBIEPSIZXY_4: 0..7 = struct USBIEPSIZXY_4Field(u8);
    }
    /// Input Endpoint_4: Y-byte count
    rw USBIEPBCTY_4 @ 0x7e6: u8 = 0_0 {
        /// Input Endpoint_4: Y-byte count
        USBIEPBCTY_4: 0..7 = struct USBIEPBCTY_4Field(u8);
    }
    /// Input Endpoint_4: Y-buffer base addr.
    rw USBIEPBBAY_4 @ 0x7e5: u8 = 0_0 {
        /// Input Endpoint_4: Y-buffer base addr.
        USBIEPBBAY_4: 0..7 = struct USBIEPBBAY_4Field(u8);
    }
    /// Input Endpoint_4: X-byte count
    rw USBIEPBCTX_4 @ 0x7e2: u8 = 0_0 {
        /// Input Endpoint_4: X-byte count
        USBIEPBCTX_4: 0..7 = struct USBIEPBCTX_4Field(u8);
    }
    /// Input Endpoint_4: X-buffer base addr.
    rw USBIEPBBAX_4 @ 0x7e1: u8 = 0_0 {
        /// Input Endpoint_4: X-buffer base addr.
        USBIEPBBAX_4: 0..7 = struct USBIEPBBAX_4Field(u8);
    }
    /// Input Endpoint_4: Configuration
    rw USBIEPCNF_4 @ 0x7e0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBIEPCNF_4_USBIIE: 2 = struct USBIEPCNF_4_USBIIE(bool);
        /// USB - Stall Condition
        USBIEPCNF_4_STALL: 3 = struct USBIEPCNF_4_STALL(bool);
        /// USB - Double Buffer Enable
        USBIEPCNF_4_DBUF: 4 = struct USBIEPCNF_4_DBUF(bool);
        /// USB - Toggle Bit
        USBIEPCNF_4_TOGGLE: 5 = struct USBIEPCNF_4_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBIEPCNF_4_UBME: 7 = struct USBIEPCNF_4_UBME(bool);
    }
    /// Input Endpoint_3: X/Y-buffer size
    rw USBIEPSIZXY_3 @ 0x7df: u8 = 0_0 {
        /// Input Endpoint_3: X/Y-buffer size
        USBIEPSIZXY_3: 0..7 = struct USBIEPSIZXY_3Field(u8);
    }
    /// Input Endpoint_3: Y-byte count
    rw USBIEPBCTY_3 @ 0x7de: u8 = 0_0 {
        /// Input Endpoint_3: Y-byte count
        USBIEPBCTY_3: 0..7 = struct USBIEPBCTY_3Field(u8);
    }
    /// Input Endpoint_3: Y-buffer base addr.
    rw USBIEPBBAY_3 @ 0x7dd: u8 = 0_0 {
        /// Input Endpoint_3: Y-buffer base addr.
        USBIEPBBAY_3: 0..7 = struct USBIEPBBAY_3Field(u8);
    }
    /// Input Endpoint_3: X-byte count
    rw USBIEPBCTX_3 @ 0x7da: u8 = 0_0 {
        /// Input Endpoint_3: X-byte count
        USBIEPBCTX_3: 0..7 = struct USBIEPBCTX_3Field(u8);
    }
    /// Input Endpoint_3: X-buffer base addr.
    rw USBIEPBBAX_3 @ 0x7d9: u8 = 0_0 {
        /// Input Endpoint_3: X-buffer base addr.
        USBIEPBBAX_3: 0..7 = struct USBIEPBBAX_3Field(u8);
    }
    /// Input Endpoint_3: Configuration
    rw USBIEPCNF_3 @ 0x7d8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBIEPCNF_3_USBIIE: 2 = struct USBIEPCNF_3_USBIIE(bool);
        /// USB - Stall Condition
        USBIEPCNF_3_STALL: 3 = struct USBIEPCNF_3_STALL(bool);
        /// USB - Double Buffer Enable
        USBIEPCNF_3_DBUF: 4 = struct USBIEPCNF_3_DBUF(bool);
        /// USB - Toggle Bit
        USBIEPCNF_3_TOGGLE: 5 = struct USBIEPCNF_3_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBIEPCNF_3_UBME: 7 = struct USBIEPCNF_3_UBME(bool);
    }
    /// Input Endpoint_2: X/Y-buffer size
    rw USBIEPSIZXY_2 @ 0x7d7: u8 = 0_0 {
        /// Input Endpoint_2: X/Y-buffer size
        USBIEPSIZXY_2: 0..7 = struct USBIEPSIZXY_2Field(u8);
    }
    /// Input Endpoint_2: Y-byte count
    rw USBIEPBCTY_2 @ 0x7d6: u8 = 0_0 {
        /// Input Endpoint_2: Y-byte count
        USBIEPBCTY_2: 0..7 = struct USBIEPBCTY_2Field(u8);
    }
    /// Input Endpoint_2: Y-buffer base addr.
    rw USBIEPBBAY_2 @ 0x7d5: u8 = 0_0 {
        /// Input Endpoint_2: Y-buffer base addr.
        USBIEPBBAY_2: 0..7 = struct USBIEPBBAY_2Field(u8);
    }
    /// Input Endpoint_2: X-byte count
    rw USBIEPBCTX_2 @ 0x7d2: u8 = 0_0 {
        /// Input Endpoint_2: X-byte count
        USBIEPBCTX_2: 0..7 = struct USBIEPBCTX_2Field(u8);
    }
    /// Input Endpoint_2: X-buffer base addr.
    rw USBIEPBBAX_2 @ 0x7d1: u8 = 0_0 {
        /// Input Endpoint_2: X-buffer base addr.
        USBIEPBBAX_2: 0..7 = struct USBIEPBBAX_2Field(u8);
    }
    /// Input Endpoint_2: Configuration
    rw USBIEPCNF_2 @ 0x7d0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBIEPCNF_2_USBIIE: 2 = struct USBIEPCNF_2_USBIIE(bool);
        /// USB - Stall Condition
        USBIEPCNF_2_STALL: 3 = struct USBIEPCNF_2_STALL(bool);
        /// USB - Double Buffer Enable
        USBIEPCNF_2_DBUF: 4 = struct USBIEPCNF_2_DBUF(bool);
        /// USB - Toggle Bit
        USBIEPCNF_2_TOGGLE: 5 = struct USBIEPCNF_2_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBIEPCNF_2_UBME: 7 = struct USBIEPCNF_2_UBME(bool);
    }
    /// Input Endpoint_1: X/Y-buffer size
    rw USBIEPSIZXY_1 @ 0x7cf: u8 = 0_0 {
        /// Input Endpoint_1: X/Y-buffer size
        USBIEPSIZXY_1: 0..7 = struct USBIEPSIZXY_1Field(u8);
    }
    /// Input Endpoint_1: Y-byte count
    rw USBIEPBCTY_1 @ 0x7ce: u8 = 0_0 {
        /// Input Endpoint_1: Y-byte count
        USBIEPBCTY_1: 0..7 = struct USBIEPBCTY_1Field(u8);
    }
    /// Input Endpoint_1: Y-buffer base addr.
    rw USBIEPBBAY_1 @ 0x7cd: u8 = 0_0 {
        /// Input Endpoint_1: Y-buffer base addr.
        USBIEPBBAY_1: 0..7 = struct USBIEPBBAY_1Field(u8);
    }
    /// Input Endpoint_1: X-byte count
    rw USBIEPBCTX_1 @ 0x7ca: u8 = 0_0 {
        /// Input Endpoint_1: X-byte count
        USBIEPBCTX_1: 0..7 = struct USBIEPBCTX_1Field(u8);
    }
    /// Input Endpoint_1: X-buffer base addr.
    rw USBIEPBBAX_1 @ 0x7c9: u8 = 0_0 {
        /// Input Endpoint_1: X-buffer base addr.
        USBIEPBBAX_1: 0..7 = struct USBIEPBBAX_1Field(u8);
    }
    /// Input Endpoint_1: Configuration
    rw USBIEPCNF_1 @ 0x7c8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBIEPCNF_1_USBIIE: 2 = struct USBIEPCNF_1_USBIIE(bool);
        /// USB - Stall Condition
        USBIEPCNF_1_STALL: 3 = struct USBIEPCNF_1_STALL(bool);
        /// USB - Double Buffer Enable
        USBIEPCNF_1_DBUF: 4 = struct USBIEPCNF_1_DBUF(bool);
        /// USB - Toggle Bit
        USBIEPCNF_1_TOGGLE: 5 = struct USBIEPCNF_1_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBIEPCNF_1_UBME: 7 = struct USBIEPCNF_1_UBME(bool);
    }
    /// Output Endpoint_7: X/Y-buffer size
    rw USBOEPSIZXY_7 @ 0x7bf: u8 = 0_0 {
        /// Output Endpoint_7: X/Y-buffer size
        USBOEPSIZXY_7: 0..7 = struct USBOEPSIZXY_7Field(u8);
    }
    /// Output Endpoint_7: Y-byte count
    rw USBOEPBCTY_7 @ 0x7be: u8 = 0_0 {
        /// Output Endpoint_7: Y-byte count
        USBOEPBCTY_7: 0..7 = struct USBOEPBCTY_7Field(u8);
    }
    /// Output Endpoint_7: Y-buffer base addr.
    rw USBOEPBBAY_7 @ 0x7bd: u8 = 0_0 {
        /// Output Endpoint_7: Y-buffer base addr.
        USBOEPBBAY_7: 0..7 = struct USBOEPBBAY_7Field(u8);
    }
    /// Output Endpoint_7: X-byte count
    rw USBOEPBCTX_7 @ 0x7ba: u8 = 0_0 {
        /// Output Endpoint_7: X-byte count
        USBOEPBCTX_7: 0..7 = struct USBOEPBCTX_7Field(u8);
    }
    /// Output Endpoint_7: X-buffer base addr.
    rw USBOEPBBAX_7 @ 0x7b9: u8 = 0_0 {
        /// Output Endpoint_7: X-buffer base addr.
        USBOEPBBAX_7: 0..7 = struct USBOEPBBAX_7Field(u8);
    }
    /// Output Endpoint_7: Configuration
    rw USBOEPCNF_7 @ 0x7b8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBOEPCNF_7_USBIIE: 2 = struct USBOEPCNF_7_USBIIE(bool);
        /// USB - Stall Condition
        USBOEPCNF_7_STALL: 3 = struct USBOEPCNF_7_STALL(bool);
        /// USB - Double Buffer Enable
        USBOEPCNF_7_DBUF: 4 = struct USBOEPCNF_7_DBUF(bool);
        /// USB - Toggle Bit
        USBOEPCNF_7_TOGGLE: 5 = struct USBOEPCNF_7_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBOEPCNF_7_UBME: 7 = struct USBOEPCNF_7_UBME(bool);
    }
    /// Output Endpoint_6: X/Y-buffer size
    rw USBOEPSIZXY_6 @ 0x7b7: u8 = 0_0 {
        /// Output Endpoint_6: X/Y-buffer size
        USBOEPSIZXY_6: 0..7 = struct USBOEPSIZXY_6Field(u8);
    }
    /// Output Endpoint_6: Y-byte count
    rw USBOEPBCTY_6 @ 0x7b6: u8 = 0_0 {
        /// Output Endpoint_6: Y-byte count
        USBOEPBCTY_6: 0..7 = struct USBOEPBCTY_6Field(u8);
    }
    /// Output Endpoint_6: Y-buffer base addr.
    rw USBOEPBBAY_6 @ 0x7b5: u8 = 0_0 {
        /// Output Endpoint_6: Y-buffer base addr.
        USBOEPBBAY_6: 0..7 = struct USBOEPBBAY_6Field(u8);
    }
    /// Output Endpoint_6: X-byte count
    rw USBOEPBCTX_6 @ 0x7b2: u8 = 0_0 {
        /// Output Endpoint_6: X-byte count
        USBOEPBCTX_6: 0..7 = struct USBOEPBCTX_6Field(u8);
    }
    /// Output Endpoint_6: X-buffer base addr.
    rw USBOEPBBAX_6 @ 0x7b1: u8 = 0_0 {
        /// Output Endpoint_6: X-buffer base addr.
        USBOEPBBAX_6: 0..7 = struct USBOEPBBAX_6Field(u8);
    }
    /// Output Endpoint_6: Configuration
    rw USBOEPCNF_6 @ 0x7b0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBOEPCNF_6_USBIIE: 2 = struct USBOEPCNF_6_USBIIE(bool);
        /// USB - Stall Condition
        USBOEPCNF_6_STALL: 3 = struct USBOEPCNF_6_STALL(bool);
        /// USB - Double Buffer Enable
        USBOEPCNF_6_DBUF: 4 = struct USBOEPCNF_6_DBUF(bool);
        /// USB - Toggle Bit
        USBOEPCNF_6_TOGGLE: 5 = struct USBOEPCNF_6_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBOEPCNF_6_UBME: 7 = struct USBOEPCNF_6_UBME(bool);
    }
    /// Output Endpoint_5: X/Y-buffer size
    rw USBOEPSIZXY_5 @ 0x7af: u8 = 0_0 {
        /// Output Endpoint_5: X/Y-buffer size
        USBOEPSIZXY_5: 0..7 = struct USBOEPSIZXY_5Field(u8);
    }
    /// Output Endpoint_5: Y-byte count
    rw USBOEPBCTY_5 @ 0x7ae: u8 = 0_0 {
        /// Output Endpoint_5: Y-byte count
        USBOEPBCTY_5: 0..7 = struct USBOEPBCTY_5Field(u8);
    }
    /// Output Endpoint_5: Y-buffer base addr.
    rw USBOEPBBAY_5 @ 0x7ad: u8 = 0_0 {
        /// Output Endpoint_5: Y-buffer base addr.
        USBOEPBBAY_5: 0..7 = struct USBOEPBBAY_5Field(u8);
    }
    /// Output Endpoint_5: X-byte count
    rw USBOEPBCTX_5 @ 0x7aa: u8 = 0_0 {
        /// Output Endpoint_5: X-byte count
        USBOEPBCTX_5: 0..7 = struct USBOEPBCTX_5Field(u8);
    }
    /// Output Endpoint_5: X-buffer base addr.
    rw USBOEPBBAX_5 @ 0x7a9: u8 = 0_0 {
        /// Output Endpoint_5: X-buffer base addr.
        USBOEPBBAX_5: 0..7 = struct USBOEPBBAX_5Field(u8);
    }
    /// Output Endpoint_5: Configuration
    rw USBOEPCNF_5 @ 0x7a8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBOEPCNF_5_USBIIE: 2 = struct USBOEPCNF_5_USBIIE(bool);
        /// USB - Stall Condition
        USBOEPCNF_5_STALL: 3 = struct USBOEPCNF_5_STALL(bool);
        /// USB - Double Buffer Enable
        USBOEPCNF_5_DBUF: 4 = struct USBOEPCNF_5_DBUF(bool);
        /// USB - Toggle Bit
        USBOEPCNF_5_TOGGLE: 5 = struct USBOEPCNF_5_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBOEPCNF_5_UBME: 7 = struct USBOEPCNF_5_UBME(bool);
    }
    /// Output Endpoint_4: X/Y-buffer size
    rw USBOEPSIZXY_4 @ 0x7a7: u8 = 0_0 {
        /// Output Endpoint_4: X/Y-buffer size
        USBOEPSIZXY_4: 0..7 = struct USBOEPSIZXY_4Field(u8);
    }
    /// Output Endpoint_4: Y-byte count
    rw USBOEPBCTY_4 @ 0x7a6: u8 = 0_0 {
        /// Output Endpoint_4: Y-byte count
        USBOEPBCTY_4: 0..7 = struct USBOEPBCTY_4Field(u8);
    }
    /// Output Endpoint_4: Y-buffer base addr.
    rw USBOEPBBAY_4 @ 0x7a5: u8 = 0_0 {
        /// Output Endpoint_4: Y-buffer base addr.
        USBOEPBBAY_4: 0..7 = struct USBOEPBBAY_4Field(u8);
    }
    /// Output Endpoint_4: X-byte count
    rw USBOEPBCTX_4 @ 0x7a2: u8 = 0_0 {
        /// Output Endpoint_4: X-byte count
        USBOEPBCTX_4: 0..7 = struct USBOEPBCTX_4Field(u8);
    }
    /// Output Endpoint_4: X-buffer base addr.
    rw USBOEPBBAX_4 @ 0x7a1: u8 = 0_0 {
        /// Output Endpoint_4: X-buffer base addr.
        USBOEPBBAX_4: 0..7 = struct USBOEPBBAX_4Field(u8);
    }
    /// Output Endpoint_4: Configuration
    rw USBOEPCNF_4 @ 0x7a0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBOEPCNF_4_USBIIE: 2 = struct USBOEPCNF_4_USBIIE(bool);
        /// USB - Stall Condition
        USBOEPCNF_4_STALL: 3 = struct USBOEPCNF_4_STALL(bool);
        /// USB - Double Buffer Enable
        USBOEPCNF_4_DBUF: 4 = struct USBOEPCNF_4_DBUF(bool);
        /// USB - Toggle Bit
        USBOEPCNF_4_TOGGLE: 5 = struct USBOEPCNF_4_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBOEPCNF_4_UBME: 7 = struct USBOEPCNF_4_UBME(bool);
    }
    /// Output Endpoint_3: X/Y-buffer size
    rw USBOEPSIZXY_3 @ 0x79f: u8 = 0_0 {
        /// Output Endpoint_3: X/Y-buffer size
        USBOEPSIZXY_3: 0..7 = struct USBOEPSIZXY_3Field(u8);
    }
    /// Output Endpoint_3: Y-byte count
    rw USBOEPBCTY_3 @ 0x79e: u8 = 0_0 {
        /// Output Endpoint_3: Y-byte count
        USBOEPBCTY_3: 0..7 = struct USBOEPBCTY_3Field(u8);
    }
    /// Output Endpoint_3: Y-buffer base addr.
    rw USBOEPBBAY_3 @ 0x79d: u8 = 0_0 {
        /// Output Endpoint_3: Y-buffer base addr.
        USBOEPBBAY_3: 0..7 = struct USBOEPBBAY_3Field(u8);
    }
    /// Output Endpoint_3: X-byte count
    rw USBOEPBCTX_3 @ 0x79a: u8 = 0_0 {
        /// Output Endpoint_3: X-byte count
        USBOEPBCTX_3: 0..7 = struct USBOEPBCTX_3Field(u8);
    }
    /// Output Endpoint_3: X-buffer base addr.
    rw USBOEPBBAX_3 @ 0x799: u8 = 0_0 {
        /// Output Endpoint_3: X-buffer base addr.
        USBOEPBBAX_3: 0..7 = struct USBOEPBBAX_3Field(u8);
    }
    /// Output Endpoint_3: Configuration
    rw USBOEPCNF_3 @ 0x798: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBOEPCNF_3_USBIIE: 2 = struct USBOEPCNF_3_USBIIE(bool);
        /// USB - Stall Condition
        USBOEPCNF_3_STALL: 3 = struct USBOEPCNF_3_STALL(bool);
        /// USB - Double Buffer Enable
        USBOEPCNF_3_DBUF: 4 = struct USBOEPCNF_3_DBUF(bool);
        /// USB - Toggle Bit
        USBOEPCNF_3_TOGGLE: 5 = struct USBOEPCNF_3_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBOEPCNF_3_UBME: 7 = struct USBOEPCNF_3_UBME(bool);
    }
    /// Output Endpoint_2: X/Y-buffer size
    rw USBOEPSIZXY_2 @ 0x797: u8 = 0_0 {
        /// Output Endpoint_2: X/Y-buffer size
        USBOEPSIZXY_2: 0..7 = struct USBOEPSIZXY_2Field(u8);
    }
    /// Output Endpoint_2: Y-byte count
    rw USBOEPBCTY_2 @ 0x796: u8 = 0_0 {
        /// Output Endpoint_2: Y-byte count
        USBOEPBCTY_2: 0..7 = struct USBOEPBCTY_2Field(u8);
    }
    /// Output Endpoint_2: Y-buffer base addr.
    rw USBOEPBBAY_2 @ 0x795: u8 = 0_0 {
        /// Output Endpoint_2: Y-buffer base addr.
        USBOEPBBAY_2: 0..7 = struct USBOEPBBAY_2Field(u8);
    }
    /// Output Endpoint_2: X-byte count
    rw USBOEPBCTX_2 @ 0x792: u8 = 0_0 {
        /// Output Endpoint_2: X-byte count
        USBOEPBCTX_2: 0..7 = struct USBOEPBCTX_2Field(u8);
    }
    /// Output Endpoint_2: X-buffer base addr.
    rw USBOEPBBAX_2 @ 0x791: u8 = 0_0 {
        /// Output Endpoint_2: X-buffer base addr.
        USBOEPBBAX_2: 0..7 = struct USBOEPBBAX_2Field(u8);
    }
    /// Output Endpoint_2: Configuration
    rw USBOEPCNF_2 @ 0x790: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBOEPCNF_2_USBIIE: 2 = struct USBOEPCNF_2_USBIIE(bool);
        /// USB - Stall Condition
        USBOEPCNF_2_STALL: 3 = struct USBOEPCNF_2_STALL(bool);
        /// USB - Double Buffer Enable
        USBOEPCNF_2_DBUF: 4 = struct USBOEPCNF_2_DBUF(bool);
        /// USB - Toggle Bit
        USBOEPCNF_2_TOGGLE: 5 = struct USBOEPCNF_2_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBOEPCNF_2_UBME: 7 = struct USBOEPCNF_2_UBME(bool);
    }
    /// Output Endpoint_1: X/Y-buffer size
    rw USBOEPSIZXY_1 @ 0x78f: u8 = 0_0 {
        /// Output Endpoint_1: X/Y-buffer size
        USBOEPSIZXY_1: 0..7 = struct USBOEPSIZXY_1Field(u8);
    }
    /// Output Endpoint_1: Y-byte count
    rw USBOEPBCTY_1 @ 0x78e: u8 = 0_0 {
        /// Output Endpoint_1: Y-byte count
        USBOEPBCTY_1: 0..7 = struct USBOEPBCTY_1Field(u8);
    }
    /// Output Endpoint_1: Y-buffer base addr.
    rw USBOEPBBAY_1 @ 0x78d: u8 = 0_0 {
        /// Output Endpoint_1: Y-buffer base addr.
        USBOEPBBAY_1: 0..7 = struct USBOEPBBAY_1Field(u8);
    }
    /// Output Endpoint_1: X-byte count
    rw USBOEPBCTX_1 @ 0x78a: u8 = 0_0 {
        /// Output Endpoint_1: X-byte count
        USBOEPBCTX_1: 0..7 = struct USBOEPBCTX_1Field(u8);
    }
    /// Output Endpoint_1: X-buffer base addr.
    rw USBOEPBBAX_1 @ 0x789: u8 = 0_0 {
        /// Output Endpoint_1: X-buffer base addr.
        USBOEPBBAX_1: 0..7 = struct USBOEPBBAX_1Field(u8);
    }
    /// Output Endpoint_1: Configuration
    rw USBOEPCNF_1 @ 0x788: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        USBOEPCNF_1_USBIIE: 2 = struct USBOEPCNF_1_USBIIE(bool);
        /// USB - Stall Condition
        USBOEPCNF_1_STALL: 3 = struct USBOEPCNF_1_STALL(bool);
        /// USB - Double Buffer Enable
        USBOEPCNF_1_DBUF: 4 = struct USBOEPCNF_1_DBUF(bool);
        /// USB - Toggle Bit
        USBOEPCNF_1_TOGGLE: 5 = struct USBOEPCNF_1_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        USBOEPCNF_1_UBME: 7 = struct USBOEPCNF_1_UBME(bool);
    }
    /// Setup Packet Block
    rw USBSUBLK @ 0x780: u8 = 0_0 {
        /// Setup Packet Block
        USBSUBLK: 0..7 = struct USBSUBLKField(u8);
    }
    /// Input endpoint_0 buffer
    rw USBIEP0BUF @ 0x778: u8 = 0_0 {
        /// Input endpoint_0 buffer
        USBIEP0BUF: 0..7 = struct USBIEP0BUFField(u8);
    }
    /// Output endpoint_0 buffer
    rw USBOEP0BUF @ 0x770: u8 = 0_0 {
        /// Output endpoint_0 buffer
        USBOEP0BUF: 0..7 = struct USBOEP0BUFField(u8);
    }
    /// Top of buffer space
    rw USBTOPBUFF @ 0x76f: u8 = 0_0 {
        /// Top of buffer space
        USBTOPBUFF: 0..7 = struct USBTOPBUFFField(u8);
    }
    /// Start of buffer space
    rw USBSTABUFF @ 0x00: u8 = 0_0 {
        /// Start of buffer space
        USBSTABUFF: 0..7 = struct USBSTABUFFField(u8);
    }
}
