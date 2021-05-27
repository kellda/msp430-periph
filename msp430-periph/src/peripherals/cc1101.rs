//! CC1101 Radio Interface

utils::periph! {
    /// CC1101 Radio Interface
    CC1101;
    /// Radio interface control register 0
    rw IFCTL0 @ 0x00: u16 = 0_0 {
        /// CC1101 Direct FIFO access enable
        FIFOEN: 0 = struct FIFOEN(bool);
        /// CC1101 Disable endianness conversion
        ENDIAN: 1 = struct ENDIAN(bool);
    }
    /// Radio interface control register 1
    rw IFCTL1 @ 0x02: u16 = 0_0 {
        /// Radio interface direct FIFO access receive interrupt flag
        RXIFG: 0 = struct RXIFG(bool);
        /// Radio interface direct FIFO access transmit interrupt flag
        TXIFG: 1 = struct TXIFG(bool);
        /// Radio interface error interrupt flag
        ERRIFG: 2 = struct ERRIFG(bool);
        /// Radio interface instruction interrupt flag
        INSTRIFG: 4 = struct INSTRIFG(bool);
        /// Radio interface data in interrupt flag
        DINIFG: 5 = struct DINIFG(bool);
        /// Radio interface status interrupt flag
        STATIFG: 6 = struct STATIFG(bool);
        /// Radio interface data out interrupt flag
        DOUTIFG: 7 = struct DOUTIFG(bool);
        /// Radio interface direct FIFO access receive interrupt enable
        RXIE: 8 = struct RXIE(bool);
        /// Radio interface direct FIFO access transmit interrupt enable
        TXIE: 9 = struct TXIE(bool);
        /// Radio interface error interrupt enable
        ERRIE: 10 = struct ERRIE(bool);
        /// Radio interface instruction interrupt enable
        INSTRIE: 12 = struct INSTRIE(bool);
        /// Radio interface data in interrupt enable
        DINIE: 13 = struct DINIE(bool);
        /// Radio interface status interrupt enable
        STATIE: 14 = struct STATIE(bool);
        /// Radio interface data out interrupt enable
        DOUTIE: 15 = struct DOUTIE(bool);
    }
    /// (Radio interface control register 2)
    rw IFCTL2 @ 0x04: u16 = 0_0 {
        /// (Radio interface control register 2)
        IFCTL2: 0..15 = struct IFCTL2Field(u16);
    }
    /// Radio interface error flag register
    rw IFERR @ 0x06: u16 = 0_0 {
        /// Low Core Voltage Error Flag
        LVERR: 0 = struct LVERR(bool);
        /// Operand Error Flag
        OPERR: 1 = struct OPERR(bool);
        /// Output data not available Error Flag
        OUTERR: 2 = struct OUTERR(bool);
        /// Operand Overwrite Error Flag
        OPOVERR: 3 = struct OPOVERR(bool);
    }
    /// Radio interface error vector word register
    rw IFERRV @ 0x0c: u16 = 0_0 {
        /// Radio interface error vector word register
        IFERRV: 0..15 = struct IFERRVField(u16);
    }
    /// Radio interface interrupt vector word register
    rw IFIV @ 0x0e: u16 = 0_0 {
        /// Radio interface interrupt vector word register
        IFIV: 0..15 = struct IFIVField(u16);
    }
    /// Radio instruction word register
    rw INSTRW @ 0x10: u16 = 0_0 {
        /// Radio instruction word register
        INSTRW: 0..15 = struct INSTRWField(u16);
    }
    /// Radio instruction 1-byte register with autoread
    rw INSTR1W @ 0x12: u16 = 0_0 {
        /// Radio instruction 1-byte register with autoread
        INSTR1W: 0..15 = struct INSTR1WField(u16);
    }
    /// Radio instruction 2-byte register with autoread
    rw INSTR2W @ 0x14: u16 = 0_0 {
        /// Radio instruction 2-byte register with autoread
        INSTR2W: 0..15 = struct INSTR2WField(u16);
    }
    /// Radio word data in register
    rw DINW @ 0x16: u16 = 0_0 {
        /// Radio word data in register
        DINW: 0..15 = struct DINWField(u16);
    }
    /// Radio status word register without auto-read
    rw STAT0W @ 0x20: u16 = 0_0 {
        /// Radio status word register without auto-read
        STAT0W: 0..15 = struct STAT0WField(u16);
    }
    /// Radio status word register with 1-byte auto-read
    rw STAT1W @ 0x22: u16 = 0_0 {
        /// Radio status word register with 1-byte auto-read
        STAT1W: 0..15 = struct STAT1WField(u16);
    }
    /// Radio status word register with 2-byte auto-read
    rw STAT2W @ 0x24: u16 = 0_0 {
        /// Radio status word register with 2-byte auto-read
        STAT2W: 0..15 = struct STAT2WField(u16);
    }
    /// Radio core word data out register without auto-read
    rw DOUT0W @ 0x28: u16 = 0_0 {
        /// Radio core word data out register without auto-read
        DOUT0W: 0..15 = struct DOUT0WField(u16);
    }
    /// Radio core word data out register with 1-byte auto-read
    rw DOUT1W @ 0x2a: u16 = 0_0 {
        /// Radio core word data out register with 1-byte auto-read
        DOUT1W: 0..15 = struct DOUT1WField(u16);
    }
    /// Radio core word data out register with 2-byte auto-read
    rw DOUT2W @ 0x2c: u16 = 0_0 {
        /// Radio core word data out register with 2-byte auto-read
        DOUT2W: 0..15 = struct DOUT2WField(u16);
    }
    /// Radio core signal input register
    rw IN_ @ 0x30: u16 = 0_0 {
        /// Radio core signal input register
        IN: 0..15 = struct INField(u16);
    }
    /// Radio core interrupt flag register
    rw IFG @ 0x32: u16 = 0_0 {
        /// Radio core interrupt flag register
        IFG: 0..15 = struct IFGField(u16);
    }
    /// Radio core interrupt edge select register
    rw IES @ 0x34: u16 = 0_0 {
        /// Radio core interrupt edge select register
        IES: 0..15 = struct IESField(u16);
    }
    /// Radio core interrupt enable register
    rw IE @ 0x36: u16 = 0_0 {
        /// Radio core interrupt enable register
        IE: 0..15 = struct IEField(u16);
    }
    /// Radio core interrupt vector word register
    rw IV @ 0x38: u16 = 0_0 {
        /// Radio core interrupt vector word register
        IV: 0..15 = struct IVField(u16);
    }
    /// Direct receive FIFO access register
    rw RXFIFO @ 0x3c: u16 = 0_0 {
        /// Direct receive FIFO access register
        RXFIFO: 0..15 = struct RXFIFOField(u16);
    }
    /// Direct transmit FIFO access register
    rw TXFIFO @ 0x3e: u16 = 0_0 {
        /// Direct transmit FIFO access register
        TXFIFO: 0..15 = struct TXFIFOField(u16);
    }
}
