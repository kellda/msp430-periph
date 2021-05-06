//! CC1101 Radio Interface

utils::periph! {
    /// CC1101 Radio Interface
    CC1101RadioInterface;
    /// Radio interface control register 0
    rw RF1AIFCTL0 @ 0x00: u16 = 0_0 {
        /// CC1101 Direct FIFO access enable
        RFFIFOEN: 0 = struct RFFIFOEN(bool);
        /// CC1101 Disable endianness conversion
        RFENDIAN: 1 = struct RFENDIAN(bool);
    }
    /// Radio interface control register 1
    rw RF1AIFCTL1 @ 0x02: u16 = 0_0 {
        /// Radio interface direct FIFO access receive interrupt flag
        RFRXIFG: 0 = struct RFRXIFG(bool);
        /// Radio interface direct FIFO access transmit interrupt flag
        RFTXIFG: 1 = struct RFTXIFG(bool);
        /// Radio interface error interrupt flag
        RFERRIFG: 2 = struct RFERRIFG(bool);
        /// Radio interface instruction interrupt flag
        RFINSTRIFG: 4 = struct RFINSTRIFG(bool);
        /// Radio interface data in interrupt flag
        RFDINIFG: 5 = struct RFDINIFG(bool);
        /// Radio interface status interrupt flag
        RFSTATIFG: 6 = struct RFSTATIFG(bool);
        /// Radio interface data out interrupt flag
        RFDOUTIFG: 7 = struct RFDOUTIFG(bool);
        /// Radio interface direct FIFO access receive interrupt enable
        RFRXIE: 8 = struct RFRXIE(bool);
        /// Radio interface direct FIFO access transmit interrupt enable
        RFTXIE: 9 = struct RFTXIE(bool);
        /// Radio interface error interrupt enable
        RFERRIE: 10 = struct RFERRIE(bool);
        /// Radio interface instruction interrupt enable
        RFINSTRIE: 12 = struct RFINSTRIE(bool);
        /// Radio interface data in interrupt enable
        RFDINIE: 13 = struct RFDINIE(bool);
        /// Radio interface status interrupt enable
        RFSTATIE: 14 = struct RFSTATIE(bool);
        /// Radio interface data out interrupt enable
        RFDOUTIE: 15 = struct RFDOUTIE(bool);
    }
    /// (Radio interface control register 2)
    rw RF1AIFCTL2 @ 0x04: u16 = 0_0 {
        /// (Radio interface control register 2)
        RF1AIFCTL2: 0..15 = struct RF1AIFCTL2Field(u16);
    }
    /// Radio interface error flag register
    rw RF1AIFERR @ 0x06: u16 = 0_0 {
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
    rw RF1AIFERRV @ 0x0c: u16 = 0_0 {
        /// Radio interface error vector word register
        RF1AIFERRV: 0..15 = struct RF1AIFERRVField(u16);
    }
    /// Radio interface interrupt vector word register
    rw RF1AIFIV @ 0x0e: u16 = 0_0 {
        /// Radio interface interrupt vector word register
        RF1AIFIV: 0..15 = struct RF1AIFIVField(u16);
    }
    /// Radio instruction word register
    rw RF1AINSTRW @ 0x10: u16 = 0_0 {
        /// Radio instruction word register
        RF1AINSTRW: 0..15 = struct RF1AINSTRWField(u16);
    }
    /// Radio instruction 1-byte register with autoread
    rw RF1AINSTR1W @ 0x12: u16 = 0_0 {
        /// Radio instruction 1-byte register with autoread
        RF1AINSTR1W: 0..15 = struct RF1AINSTR1WField(u16);
    }
    /// Radio instruction 2-byte register with autoread
    rw RF1AINSTR2W @ 0x14: u16 = 0_0 {
        /// Radio instruction 2-byte register with autoread
        RF1AINSTR2W: 0..15 = struct RF1AINSTR2WField(u16);
    }
    /// Radio word data in register
    rw RF1ADINW @ 0x16: u16 = 0_0 {
        /// Radio word data in register
        RF1ADINW: 0..15 = struct RF1ADINWField(u16);
    }
    /// Radio status word register without auto-read
    rw RF1ASTAT0W @ 0x20: u16 = 0_0 {
        /// Radio status word register without auto-read
        RF1ASTAT0W: 0..15 = struct RF1ASTAT0WField(u16);
    }
    /// Radio status word register with 1-byte auto-read
    rw RF1ASTAT1W @ 0x22: u16 = 0_0 {
        /// Radio status word register with 1-byte auto-read
        RF1ASTAT1W: 0..15 = struct RF1ASTAT1WField(u16);
    }
    /// Radio status word register with 2-byte auto-read
    rw RF1ASTAT2W @ 0x24: u16 = 0_0 {
        /// Radio status word register with 2-byte auto-read
        RF1ASTAT2W: 0..15 = struct RF1ASTAT2WField(u16);
    }
    /// Radio core word data out register without auto-read
    rw RF1ADOUT0W @ 0x28: u16 = 0_0 {
        /// Radio core word data out register without auto-read
        RF1ADOUT0W: 0..15 = struct RF1ADOUT0WField(u16);
    }
    /// Radio core word data out register with 1-byte auto-read
    rw RF1ADOUT1W @ 0x2a: u16 = 0_0 {
        /// Radio core word data out register with 1-byte auto-read
        RF1ADOUT1W: 0..15 = struct RF1ADOUT1WField(u16);
    }
    /// Radio core word data out register with 2-byte auto-read
    rw RF1ADOUT2W @ 0x2c: u16 = 0_0 {
        /// Radio core word data out register with 2-byte auto-read
        RF1ADOUT2W: 0..15 = struct RF1ADOUT2WField(u16);
    }
    /// Radio core signal input register
    rw RF1AIN @ 0x30: u16 = 0_0 {
        /// Radio core signal input register
        RF1AIN: 0..15 = struct RF1AINField(u16);
    }
    /// Radio core interrupt flag register
    rw RF1AIFG @ 0x32: u16 = 0_0 {
        /// Radio core interrupt flag register
        RF1AIFG: 0..15 = struct RF1AIFGField(u16);
    }
    /// Radio core interrupt edge select register
    rw RF1AIES @ 0x34: u16 = 0_0 {
        /// Radio core interrupt edge select register
        RF1AIES: 0..15 = struct RF1AIESField(u16);
    }
    /// Radio core interrupt enable register
    rw RF1AIE @ 0x36: u16 = 0_0 {
        /// Radio core interrupt enable register
        RF1AIE: 0..15 = struct RF1AIEField(u16);
    }
    /// Radio core interrupt vector word register
    rw RF1AIV @ 0x38: u16 = 0_0 {
        /// Radio core interrupt vector word register
        RF1AIV: 0..15 = struct RF1AIVField(u16);
    }
    /// Direct receive FIFO access register
    rw RF1ARXFIFO @ 0x3c: u16 = 0_0 {
        /// Direct receive FIFO access register
        RF1ARXFIFO: 0..15 = struct RF1ARXFIFOField(u16);
    }
    /// Direct transmit FIFO access register
    rw RF1ATXFIFO @ 0x3e: u16 = 0_0 {
        /// Direct transmit FIFO access register
        RF1ATXFIFO: 0..15 = struct RF1ATXFIFOField(u16);
    }
}
