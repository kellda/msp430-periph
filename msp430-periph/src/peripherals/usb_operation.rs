//! USB Operation

utils::periph! {
    /// USB Operation
    USBOperation;
    /// Input Endpoint_7: X/Y-buffer size
    rw IEPSIZXY_7 @ 0x7ff: u8 = 0_0 {
        /// Input Endpoint_7: X/Y-buffer size
        IEPSIZXY_7: 0..7 = struct IEPSIZXY_7Field(u8);
    }
    /// Input Endpoint_7: Y-byte count
    rw IEPBCTY_7 @ 0x7fe: u8 = 0_0 {
        /// Input Endpoint_7: Y-byte count
        IEPBCTY_7: 0..7 = struct IEPBCTY_7Field(u8);
    }
    /// Input Endpoint_7: Y-buffer base addr.
    rw IEPBBAY_7 @ 0x7fd: u8 = 0_0 {
        /// Input Endpoint_7: Y-buffer base addr.
        IEPBBAY_7: 0..7 = struct IEPBBAY_7Field(u8);
    }
    /// Input Endpoint_7: X-byte count
    rw IEPBCTX_7 @ 0x7fa: u8 = 0_0 {
        /// Input Endpoint_7: X-byte count
        IEPBCTX_7: 0..7 = struct IEPBCTX_7Field(u8);
    }
    /// Input Endpoint_7: X-buffer base addr.
    rw IEPBBAX_7 @ 0x7f9: u8 = 0_0 {
        /// Input Endpoint_7: X-buffer base addr.
        IEPBBAX_7: 0..7 = struct IEPBBAX_7Field(u8);
    }
    /// Input Endpoint_7: Configuration
    rw IEPCNF_7 @ 0x7f8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        IEPCNF_7_USBIIE: 2 = struct IEPCNF_7_USBIIE(bool);
        /// USB - Stall Condition
        IEPCNF_7_STALL: 3 = struct IEPCNF_7_STALL(bool);
        /// USB - Double Buffer Enable
        IEPCNF_7_DBUF: 4 = struct IEPCNF_7_DBUF(bool);
        /// USB - Toggle Bit
        IEPCNF_7_TOGGLE: 5 = struct IEPCNF_7_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        IEPCNF_7_UBME: 7 = struct IEPCNF_7_UBME(bool);
    }
    /// Input Endpoint_6: X/Y-buffer size
    rw IEPSIZXY_6 @ 0x7f7: u8 = 0_0 {
        /// Input Endpoint_6: X/Y-buffer size
        IEPSIZXY_6: 0..7 = struct IEPSIZXY_6Field(u8);
    }
    /// Input Endpoint_6: Y-byte count
    rw IEPBCTY_6 @ 0x7f6: u8 = 0_0 {
        /// Input Endpoint_6: Y-byte count
        IEPBCTY_6: 0..7 = struct IEPBCTY_6Field(u8);
    }
    /// Input Endpoint_6: Y-buffer base addr.
    rw IEPBBAY_6 @ 0x7f5: u8 = 0_0 {
        /// Input Endpoint_6: Y-buffer base addr.
        IEPBBAY_6: 0..7 = struct IEPBBAY_6Field(u8);
    }
    /// Input Endpoint_6: X-byte count
    rw IEPBCTX_6 @ 0x7f2: u8 = 0_0 {
        /// Input Endpoint_6: X-byte count
        IEPBCTX_6: 0..7 = struct IEPBCTX_6Field(u8);
    }
    /// Input Endpoint_6: X-buffer base addr.
    rw IEPBBAX_6 @ 0x7f1: u8 = 0_0 {
        /// Input Endpoint_6: X-buffer base addr.
        IEPBBAX_6: 0..7 = struct IEPBBAX_6Field(u8);
    }
    /// Input Endpoint_6: Configuration
    rw IEPCNF_6 @ 0x7f0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        IEPCNF_6_USBIIE: 2 = struct IEPCNF_6_USBIIE(bool);
        /// USB - Stall Condition
        IEPCNF_6_STALL: 3 = struct IEPCNF_6_STALL(bool);
        /// USB - Double Buffer Enable
        IEPCNF_6_DBUF: 4 = struct IEPCNF_6_DBUF(bool);
        /// USB - Toggle Bit
        IEPCNF_6_TOGGLE: 5 = struct IEPCNF_6_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        IEPCNF_6_UBME: 7 = struct IEPCNF_6_UBME(bool);
    }
    /// Input Endpoint_5: X/Y-buffer size
    rw IEPSIZXY_5 @ 0x7ef: u8 = 0_0 {
        /// Input Endpoint_5: X/Y-buffer size
        IEPSIZXY_5: 0..7 = struct IEPSIZXY_5Field(u8);
    }
    /// Input Endpoint_5: Y-byte count
    rw IEPBCTY_5 @ 0x7ee: u8 = 0_0 {
        /// Input Endpoint_5: Y-byte count
        IEPBCTY_5: 0..7 = struct IEPBCTY_5Field(u8);
    }
    /// Input Endpoint_5: Y-buffer base addr.
    rw IEPBBAY_5 @ 0x7ed: u8 = 0_0 {
        /// Input Endpoint_5: Y-buffer base addr.
        IEPBBAY_5: 0..7 = struct IEPBBAY_5Field(u8);
    }
    /// Input Endpoint_5: X-byte count
    rw IEPBCTX_5 @ 0x7ea: u8 = 0_0 {
        /// Input Endpoint_5: X-byte count
        IEPBCTX_5: 0..7 = struct IEPBCTX_5Field(u8);
    }
    /// Input Endpoint_5: X-buffer base addr.
    rw IEPBBAX_5 @ 0x7e9: u8 = 0_0 {
        /// Input Endpoint_5: X-buffer base addr.
        IEPBBAX_5: 0..7 = struct IEPBBAX_5Field(u8);
    }
    /// Input Endpoint_5: Configuration
    rw IEPCNF_5 @ 0x7e8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        IEPCNF_5_USBIIE: 2 = struct IEPCNF_5_USBIIE(bool);
        /// USB - Stall Condition
        IEPCNF_5_STALL: 3 = struct IEPCNF_5_STALL(bool);
        /// USB - Double Buffer Enable
        IEPCNF_5_DBUF: 4 = struct IEPCNF_5_DBUF(bool);
        /// USB - Toggle Bit
        IEPCNF_5_TOGGLE: 5 = struct IEPCNF_5_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        IEPCNF_5_UBME: 7 = struct IEPCNF_5_UBME(bool);
    }
    /// Input Endpoint_4: X/Y-buffer size
    rw IEPSIZXY_4 @ 0x7e7: u8 = 0_0 {
        /// Input Endpoint_4: X/Y-buffer size
        IEPSIZXY_4: 0..7 = struct IEPSIZXY_4Field(u8);
    }
    /// Input Endpoint_4: Y-byte count
    rw IEPBCTY_4 @ 0x7e6: u8 = 0_0 {
        /// Input Endpoint_4: Y-byte count
        IEPBCTY_4: 0..7 = struct IEPBCTY_4Field(u8);
    }
    /// Input Endpoint_4: Y-buffer base addr.
    rw IEPBBAY_4 @ 0x7e5: u8 = 0_0 {
        /// Input Endpoint_4: Y-buffer base addr.
        IEPBBAY_4: 0..7 = struct IEPBBAY_4Field(u8);
    }
    /// Input Endpoint_4: X-byte count
    rw IEPBCTX_4 @ 0x7e2: u8 = 0_0 {
        /// Input Endpoint_4: X-byte count
        IEPBCTX_4: 0..7 = struct IEPBCTX_4Field(u8);
    }
    /// Input Endpoint_4: X-buffer base addr.
    rw IEPBBAX_4 @ 0x7e1: u8 = 0_0 {
        /// Input Endpoint_4: X-buffer base addr.
        IEPBBAX_4: 0..7 = struct IEPBBAX_4Field(u8);
    }
    /// Input Endpoint_4: Configuration
    rw IEPCNF_4 @ 0x7e0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        IEPCNF_4_USBIIE: 2 = struct IEPCNF_4_USBIIE(bool);
        /// USB - Stall Condition
        IEPCNF_4_STALL: 3 = struct IEPCNF_4_STALL(bool);
        /// USB - Double Buffer Enable
        IEPCNF_4_DBUF: 4 = struct IEPCNF_4_DBUF(bool);
        /// USB - Toggle Bit
        IEPCNF_4_TOGGLE: 5 = struct IEPCNF_4_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        IEPCNF_4_UBME: 7 = struct IEPCNF_4_UBME(bool);
    }
    /// Input Endpoint_3: X/Y-buffer size
    rw IEPSIZXY_3 @ 0x7df: u8 = 0_0 {
        /// Input Endpoint_3: X/Y-buffer size
        IEPSIZXY_3: 0..7 = struct IEPSIZXY_3Field(u8);
    }
    /// Input Endpoint_3: Y-byte count
    rw IEPBCTY_3 @ 0x7de: u8 = 0_0 {
        /// Input Endpoint_3: Y-byte count
        IEPBCTY_3: 0..7 = struct IEPBCTY_3Field(u8);
    }
    /// Input Endpoint_3: Y-buffer base addr.
    rw IEPBBAY_3 @ 0x7dd: u8 = 0_0 {
        /// Input Endpoint_3: Y-buffer base addr.
        IEPBBAY_3: 0..7 = struct IEPBBAY_3Field(u8);
    }
    /// Input Endpoint_3: X-byte count
    rw IEPBCTX_3 @ 0x7da: u8 = 0_0 {
        /// Input Endpoint_3: X-byte count
        IEPBCTX_3: 0..7 = struct IEPBCTX_3Field(u8);
    }
    /// Input Endpoint_3: X-buffer base addr.
    rw IEPBBAX_3 @ 0x7d9: u8 = 0_0 {
        /// Input Endpoint_3: X-buffer base addr.
        IEPBBAX_3: 0..7 = struct IEPBBAX_3Field(u8);
    }
    /// Input Endpoint_3: Configuration
    rw IEPCNF_3 @ 0x7d8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        IEPCNF_3_USBIIE: 2 = struct IEPCNF_3_USBIIE(bool);
        /// USB - Stall Condition
        IEPCNF_3_STALL: 3 = struct IEPCNF_3_STALL(bool);
        /// USB - Double Buffer Enable
        IEPCNF_3_DBUF: 4 = struct IEPCNF_3_DBUF(bool);
        /// USB - Toggle Bit
        IEPCNF_3_TOGGLE: 5 = struct IEPCNF_3_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        IEPCNF_3_UBME: 7 = struct IEPCNF_3_UBME(bool);
    }
    /// Input Endpoint_2: X/Y-buffer size
    rw IEPSIZXY_2 @ 0x7d7: u8 = 0_0 {
        /// Input Endpoint_2: X/Y-buffer size
        IEPSIZXY_2: 0..7 = struct IEPSIZXY_2Field(u8);
    }
    /// Input Endpoint_2: Y-byte count
    rw IEPBCTY_2 @ 0x7d6: u8 = 0_0 {
        /// Input Endpoint_2: Y-byte count
        IEPBCTY_2: 0..7 = struct IEPBCTY_2Field(u8);
    }
    /// Input Endpoint_2: Y-buffer base addr.
    rw IEPBBAY_2 @ 0x7d5: u8 = 0_0 {
        /// Input Endpoint_2: Y-buffer base addr.
        IEPBBAY_2: 0..7 = struct IEPBBAY_2Field(u8);
    }
    /// Input Endpoint_2: X-byte count
    rw IEPBCTX_2 @ 0x7d2: u8 = 0_0 {
        /// Input Endpoint_2: X-byte count
        IEPBCTX_2: 0..7 = struct IEPBCTX_2Field(u8);
    }
    /// Input Endpoint_2: X-buffer base addr.
    rw IEPBBAX_2 @ 0x7d1: u8 = 0_0 {
        /// Input Endpoint_2: X-buffer base addr.
        IEPBBAX_2: 0..7 = struct IEPBBAX_2Field(u8);
    }
    /// Input Endpoint_2: Configuration
    rw IEPCNF_2 @ 0x7d0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        IEPCNF_2_USBIIE: 2 = struct IEPCNF_2_USBIIE(bool);
        /// USB - Stall Condition
        IEPCNF_2_STALL: 3 = struct IEPCNF_2_STALL(bool);
        /// USB - Double Buffer Enable
        IEPCNF_2_DBUF: 4 = struct IEPCNF_2_DBUF(bool);
        /// USB - Toggle Bit
        IEPCNF_2_TOGGLE: 5 = struct IEPCNF_2_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        IEPCNF_2_UBME: 7 = struct IEPCNF_2_UBME(bool);
    }
    /// Input Endpoint_1: X/Y-buffer size
    rw IEPSIZXY_1 @ 0x7cf: u8 = 0_0 {
        /// Input Endpoint_1: X/Y-buffer size
        IEPSIZXY_1: 0..7 = struct IEPSIZXY_1Field(u8);
    }
    /// Input Endpoint_1: Y-byte count
    rw IEPBCTY_1 @ 0x7ce: u8 = 0_0 {
        /// Input Endpoint_1: Y-byte count
        IEPBCTY_1: 0..7 = struct IEPBCTY_1Field(u8);
    }
    /// Input Endpoint_1: Y-buffer base addr.
    rw IEPBBAY_1 @ 0x7cd: u8 = 0_0 {
        /// Input Endpoint_1: Y-buffer base addr.
        IEPBBAY_1: 0..7 = struct IEPBBAY_1Field(u8);
    }
    /// Input Endpoint_1: X-byte count
    rw IEPBCTX_1 @ 0x7ca: u8 = 0_0 {
        /// Input Endpoint_1: X-byte count
        IEPBCTX_1: 0..7 = struct IEPBCTX_1Field(u8);
    }
    /// Input Endpoint_1: X-buffer base addr.
    rw IEPBBAX_1 @ 0x7c9: u8 = 0_0 {
        /// Input Endpoint_1: X-buffer base addr.
        IEPBBAX_1: 0..7 = struct IEPBBAX_1Field(u8);
    }
    /// Input Endpoint_1: Configuration
    rw IEPCNF_1 @ 0x7c8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        IEPCNF_1_USBIIE: 2 = struct IEPCNF_1_USBIIE(bool);
        /// USB - Stall Condition
        IEPCNF_1_STALL: 3 = struct IEPCNF_1_STALL(bool);
        /// USB - Double Buffer Enable
        IEPCNF_1_DBUF: 4 = struct IEPCNF_1_DBUF(bool);
        /// USB - Toggle Bit
        IEPCNF_1_TOGGLE: 5 = struct IEPCNF_1_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        IEPCNF_1_UBME: 7 = struct IEPCNF_1_UBME(bool);
    }
    /// Output Endpoint_7: X/Y-buffer size
    rw OEPSIZXY_7 @ 0x7bf: u8 = 0_0 {
        /// Output Endpoint_7: X/Y-buffer size
        OEPSIZXY_7: 0..7 = struct OEPSIZXY_7Field(u8);
    }
    /// Output Endpoint_7: Y-byte count
    rw OEPBCTY_7 @ 0x7be: u8 = 0_0 {
        /// Output Endpoint_7: Y-byte count
        OEPBCTY_7: 0..7 = struct OEPBCTY_7Field(u8);
    }
    /// Output Endpoint_7: Y-buffer base addr.
    rw OEPBBAY_7 @ 0x7bd: u8 = 0_0 {
        /// Output Endpoint_7: Y-buffer base addr.
        OEPBBAY_7: 0..7 = struct OEPBBAY_7Field(u8);
    }
    /// Output Endpoint_7: X-byte count
    rw OEPBCTX_7 @ 0x7ba: u8 = 0_0 {
        /// Output Endpoint_7: X-byte count
        OEPBCTX_7: 0..7 = struct OEPBCTX_7Field(u8);
    }
    /// Output Endpoint_7: X-buffer base addr.
    rw OEPBBAX_7 @ 0x7b9: u8 = 0_0 {
        /// Output Endpoint_7: X-buffer base addr.
        OEPBBAX_7: 0..7 = struct OEPBBAX_7Field(u8);
    }
    /// Output Endpoint_7: Configuration
    rw OEPCNF_7 @ 0x7b8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        OEPCNF_7_USBIIE: 2 = struct OEPCNF_7_USBIIE(bool);
        /// USB - Stall Condition
        OEPCNF_7_STALL: 3 = struct OEPCNF_7_STALL(bool);
        /// USB - Double Buffer Enable
        OEPCNF_7_DBUF: 4 = struct OEPCNF_7_DBUF(bool);
        /// USB - Toggle Bit
        OEPCNF_7_TOGGLE: 5 = struct OEPCNF_7_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        OEPCNF_7_UBME: 7 = struct OEPCNF_7_UBME(bool);
    }
    /// Output Endpoint_6: X/Y-buffer size
    rw OEPSIZXY_6 @ 0x7b7: u8 = 0_0 {
        /// Output Endpoint_6: X/Y-buffer size
        OEPSIZXY_6: 0..7 = struct OEPSIZXY_6Field(u8);
    }
    /// Output Endpoint_6: Y-byte count
    rw OEPBCTY_6 @ 0x7b6: u8 = 0_0 {
        /// Output Endpoint_6: Y-byte count
        OEPBCTY_6: 0..7 = struct OEPBCTY_6Field(u8);
    }
    /// Output Endpoint_6: Y-buffer base addr.
    rw OEPBBAY_6 @ 0x7b5: u8 = 0_0 {
        /// Output Endpoint_6: Y-buffer base addr.
        OEPBBAY_6: 0..7 = struct OEPBBAY_6Field(u8);
    }
    /// Output Endpoint_6: X-byte count
    rw OEPBCTX_6 @ 0x7b2: u8 = 0_0 {
        /// Output Endpoint_6: X-byte count
        OEPBCTX_6: 0..7 = struct OEPBCTX_6Field(u8);
    }
    /// Output Endpoint_6: X-buffer base addr.
    rw OEPBBAX_6 @ 0x7b1: u8 = 0_0 {
        /// Output Endpoint_6: X-buffer base addr.
        OEPBBAX_6: 0..7 = struct OEPBBAX_6Field(u8);
    }
    /// Output Endpoint_6: Configuration
    rw OEPCNF_6 @ 0x7b0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        OEPCNF_6_USBIIE: 2 = struct OEPCNF_6_USBIIE(bool);
        /// USB - Stall Condition
        OEPCNF_6_STALL: 3 = struct OEPCNF_6_STALL(bool);
        /// USB - Double Buffer Enable
        OEPCNF_6_DBUF: 4 = struct OEPCNF_6_DBUF(bool);
        /// USB - Toggle Bit
        OEPCNF_6_TOGGLE: 5 = struct OEPCNF_6_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        OEPCNF_6_UBME: 7 = struct OEPCNF_6_UBME(bool);
    }
    /// Output Endpoint_5: X/Y-buffer size
    rw OEPSIZXY_5 @ 0x7af: u8 = 0_0 {
        /// Output Endpoint_5: X/Y-buffer size
        OEPSIZXY_5: 0..7 = struct OEPSIZXY_5Field(u8);
    }
    /// Output Endpoint_5: Y-byte count
    rw OEPBCTY_5 @ 0x7ae: u8 = 0_0 {
        /// Output Endpoint_5: Y-byte count
        OEPBCTY_5: 0..7 = struct OEPBCTY_5Field(u8);
    }
    /// Output Endpoint_5: Y-buffer base addr.
    rw OEPBBAY_5 @ 0x7ad: u8 = 0_0 {
        /// Output Endpoint_5: Y-buffer base addr.
        OEPBBAY_5: 0..7 = struct OEPBBAY_5Field(u8);
    }
    /// Output Endpoint_5: X-byte count
    rw OEPBCTX_5 @ 0x7aa: u8 = 0_0 {
        /// Output Endpoint_5: X-byte count
        OEPBCTX_5: 0..7 = struct OEPBCTX_5Field(u8);
    }
    /// Output Endpoint_5: X-buffer base addr.
    rw OEPBBAX_5 @ 0x7a9: u8 = 0_0 {
        /// Output Endpoint_5: X-buffer base addr.
        OEPBBAX_5: 0..7 = struct OEPBBAX_5Field(u8);
    }
    /// Output Endpoint_5: Configuration
    rw OEPCNF_5 @ 0x7a8: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        OEPCNF_5_USBIIE: 2 = struct OEPCNF_5_USBIIE(bool);
        /// USB - Stall Condition
        OEPCNF_5_STALL: 3 = struct OEPCNF_5_STALL(bool);
        /// USB - Double Buffer Enable
        OEPCNF_5_DBUF: 4 = struct OEPCNF_5_DBUF(bool);
        /// USB - Toggle Bit
        OEPCNF_5_TOGGLE: 5 = struct OEPCNF_5_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        OEPCNF_5_UBME: 7 = struct OEPCNF_5_UBME(bool);
    }
    /// Output Endpoint_4: X/Y-buffer size
    rw OEPSIZXY_4 @ 0x7a7: u8 = 0_0 {
        /// Output Endpoint_4: X/Y-buffer size
        OEPSIZXY_4: 0..7 = struct OEPSIZXY_4Field(u8);
    }
    /// Output Endpoint_4: Y-byte count
    rw OEPBCTY_4 @ 0x7a6: u8 = 0_0 {
        /// Output Endpoint_4: Y-byte count
        OEPBCTY_4: 0..7 = struct OEPBCTY_4Field(u8);
    }
    /// Output Endpoint_4: Y-buffer base addr.
    rw OEPBBAY_4 @ 0x7a5: u8 = 0_0 {
        /// Output Endpoint_4: Y-buffer base addr.
        OEPBBAY_4: 0..7 = struct OEPBBAY_4Field(u8);
    }
    /// Output Endpoint_4: X-byte count
    rw OEPBCTX_4 @ 0x7a2: u8 = 0_0 {
        /// Output Endpoint_4: X-byte count
        OEPBCTX_4: 0..7 = struct OEPBCTX_4Field(u8);
    }
    /// Output Endpoint_4: X-buffer base addr.
    rw OEPBBAX_4 @ 0x7a1: u8 = 0_0 {
        /// Output Endpoint_4: X-buffer base addr.
        OEPBBAX_4: 0..7 = struct OEPBBAX_4Field(u8);
    }
    /// Output Endpoint_4: Configuration
    rw OEPCNF_4 @ 0x7a0: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        OEPCNF_4_USBIIE: 2 = struct OEPCNF_4_USBIIE(bool);
        /// USB - Stall Condition
        OEPCNF_4_STALL: 3 = struct OEPCNF_4_STALL(bool);
        /// USB - Double Buffer Enable
        OEPCNF_4_DBUF: 4 = struct OEPCNF_4_DBUF(bool);
        /// USB - Toggle Bit
        OEPCNF_4_TOGGLE: 5 = struct OEPCNF_4_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        OEPCNF_4_UBME: 7 = struct OEPCNF_4_UBME(bool);
    }
    /// Output Endpoint_3: X/Y-buffer size
    rw OEPSIZXY_3 @ 0x79f: u8 = 0_0 {
        /// Output Endpoint_3: X/Y-buffer size
        OEPSIZXY_3: 0..7 = struct OEPSIZXY_3Field(u8);
    }
    /// Output Endpoint_3: Y-byte count
    rw OEPBCTY_3 @ 0x79e: u8 = 0_0 {
        /// Output Endpoint_3: Y-byte count
        OEPBCTY_3: 0..7 = struct OEPBCTY_3Field(u8);
    }
    /// Output Endpoint_3: Y-buffer base addr.
    rw OEPBBAY_3 @ 0x79d: u8 = 0_0 {
        /// Output Endpoint_3: Y-buffer base addr.
        OEPBBAY_3: 0..7 = struct OEPBBAY_3Field(u8);
    }
    /// Output Endpoint_3: X-byte count
    rw OEPBCTX_3 @ 0x79a: u8 = 0_0 {
        /// Output Endpoint_3: X-byte count
        OEPBCTX_3: 0..7 = struct OEPBCTX_3Field(u8);
    }
    /// Output Endpoint_3: X-buffer base addr.
    rw OEPBBAX_3 @ 0x799: u8 = 0_0 {
        /// Output Endpoint_3: X-buffer base addr.
        OEPBBAX_3: 0..7 = struct OEPBBAX_3Field(u8);
    }
    /// Output Endpoint_3: Configuration
    rw OEPCNF_3 @ 0x798: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        OEPCNF_3_USBIIE: 2 = struct OEPCNF_3_USBIIE(bool);
        /// USB - Stall Condition
        OEPCNF_3_STALL: 3 = struct OEPCNF_3_STALL(bool);
        /// USB - Double Buffer Enable
        OEPCNF_3_DBUF: 4 = struct OEPCNF_3_DBUF(bool);
        /// USB - Toggle Bit
        OEPCNF_3_TOGGLE: 5 = struct OEPCNF_3_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        OEPCNF_3_UBME: 7 = struct OEPCNF_3_UBME(bool);
    }
    /// Output Endpoint_2: X/Y-buffer size
    rw OEPSIZXY_2 @ 0x797: u8 = 0_0 {
        /// Output Endpoint_2: X/Y-buffer size
        OEPSIZXY_2: 0..7 = struct OEPSIZXY_2Field(u8);
    }
    /// Output Endpoint_2: Y-byte count
    rw OEPBCTY_2 @ 0x796: u8 = 0_0 {
        /// Output Endpoint_2: Y-byte count
        OEPBCTY_2: 0..7 = struct OEPBCTY_2Field(u8);
    }
    /// Output Endpoint_2: Y-buffer base addr.
    rw OEPBBAY_2 @ 0x795: u8 = 0_0 {
        /// Output Endpoint_2: Y-buffer base addr.
        OEPBBAY_2: 0..7 = struct OEPBBAY_2Field(u8);
    }
    /// Output Endpoint_2: X-byte count
    rw OEPBCTX_2 @ 0x792: u8 = 0_0 {
        /// Output Endpoint_2: X-byte count
        OEPBCTX_2: 0..7 = struct OEPBCTX_2Field(u8);
    }
    /// Output Endpoint_2: X-buffer base addr.
    rw OEPBBAX_2 @ 0x791: u8 = 0_0 {
        /// Output Endpoint_2: X-buffer base addr.
        OEPBBAX_2: 0..7 = struct OEPBBAX_2Field(u8);
    }
    /// Output Endpoint_2: Configuration
    rw OEPCNF_2 @ 0x790: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        OEPCNF_2_USBIIE: 2 = struct OEPCNF_2_USBIIE(bool);
        /// USB - Stall Condition
        OEPCNF_2_STALL: 3 = struct OEPCNF_2_STALL(bool);
        /// USB - Double Buffer Enable
        OEPCNF_2_DBUF: 4 = struct OEPCNF_2_DBUF(bool);
        /// USB - Toggle Bit
        OEPCNF_2_TOGGLE: 5 = struct OEPCNF_2_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        OEPCNF_2_UBME: 7 = struct OEPCNF_2_UBME(bool);
    }
    /// Output Endpoint_1: X/Y-buffer size
    rw OEPSIZXY_1 @ 0x78f: u8 = 0_0 {
        /// Output Endpoint_1: X/Y-buffer size
        OEPSIZXY_1: 0..7 = struct OEPSIZXY_1Field(u8);
    }
    /// Output Endpoint_1: Y-byte count
    rw OEPBCTY_1 @ 0x78e: u8 = 0_0 {
        /// Output Endpoint_1: Y-byte count
        OEPBCTY_1: 0..7 = struct OEPBCTY_1Field(u8);
    }
    /// Output Endpoint_1: Y-buffer base addr.
    rw OEPBBAY_1 @ 0x78d: u8 = 0_0 {
        /// Output Endpoint_1: Y-buffer base addr.
        OEPBBAY_1: 0..7 = struct OEPBBAY_1Field(u8);
    }
    /// Output Endpoint_1: X-byte count
    rw OEPBCTX_1 @ 0x78a: u8 = 0_0 {
        /// Output Endpoint_1: X-byte count
        OEPBCTX_1: 0..7 = struct OEPBCTX_1Field(u8);
    }
    /// Output Endpoint_1: X-buffer base addr.
    rw OEPBBAX_1 @ 0x789: u8 = 0_0 {
        /// Output Endpoint_1: X-buffer base addr.
        OEPBBAX_1: 0..7 = struct OEPBBAX_1Field(u8);
    }
    /// Output Endpoint_1: Configuration
    rw OEPCNF_1 @ 0x788: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        OEPCNF_1_USBIIE: 2 = struct OEPCNF_1_USBIIE(bool);
        /// USB - Stall Condition
        OEPCNF_1_STALL: 3 = struct OEPCNF_1_STALL(bool);
        /// USB - Double Buffer Enable
        OEPCNF_1_DBUF: 4 = struct OEPCNF_1_DBUF(bool);
        /// USB - Toggle Bit
        OEPCNF_1_TOGGLE: 5 = struct OEPCNF_1_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        OEPCNF_1_UBME: 7 = struct OEPCNF_1_UBME(bool);
    }
    /// Setup Packet Block
    rw SUBLK @ 0x780: u8 = 0_0 {
        /// Setup Packet Block
        SUBLK: 0..7 = struct SUBLKField(u8);
    }
    /// Input endpoint_0 buffer
    rw IEP0BUF @ 0x778: u8 = 0_0 {
        /// Input endpoint_0 buffer
        IEP0BUF: 0..7 = struct IEP0BUFField(u8);
    }
    /// Output endpoint_0 buffer
    rw OEP0BUF @ 0x770: u8 = 0_0 {
        /// Output endpoint_0 buffer
        OEP0BUF: 0..7 = struct OEP0BUFField(u8);
    }
    /// Top of buffer space
    rw TOPBUFF @ 0x76f: u8 = 0_0 {
        /// Top of buffer space
        TOPBUFF: 0..7 = struct TOPBUFFField(u8);
    }
    /// Start of buffer space
    rw STABUFF @ 0x00: u8 = 0_0 {
        /// Start of buffer space
        STABUFF: 0..7 = struct STABUFFField(u8);
    }
}
