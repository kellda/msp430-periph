//! USART 0  I2C Mode

utils::periph! {
    /// USART 0  I2C Mode
    USART0I2CMode;
    /// I2C Interrupt Enable
    rw I2CIE @ 0x00: u8 = 0_0 {
        /// Arbitration lost
        ALIE: 0 = struct ALIE(bool);
        /// No acknowledge
        NACKIE: 1 = struct NACKIE(bool);
        /// Own address
        OAIE: 2 = struct OAIE(bool);
        /// Access ready (opeation complete)
        ARDYIE: 3 = struct ARDYIE(bool);
        /// Receive ready (data received)
        RXRDYIE: 4 = struct RXRDYIE(bool);
        /// Transmit ready (transmit register empty)
        TXRDYIE: 5 = struct TXRDYIE(bool);
        /// General call
        GCIE: 6 = struct GCIE(bool);
        /// Start condition
        STTIE: 7 = struct STTIE(bool);
    }
    /// I2C Interrupt Flag
    rw I2CIFG @ 0x01: u8 = 0_0 {
        /// Arbitration lost
        ALIFG: 0 = struct ALIFG(bool);
        /// No acknowledge
        NACKIFG: 1 = struct NACKIFG(bool);
        /// Own address
        OAIFG: 2 = struct OAIFG(bool);
        /// Access ready (opeation complete)
        ARDYIFG: 3 = struct ARDYIFG(bool);
        /// Receive ready (data received)
        RXRDYIFG: 4 = struct RXRDYIFG(bool);
        /// Transmit ready (transmit register empty)
        TXRDYIFG: 5 = struct TXRDYIFG(bool);
        /// General call
        GCIFG: 6 = struct GCIFG(bool);
        /// Start condition
        STTIFG: 7 = struct STTIFG(bool);
    }
    /// I2C Data Count
    rw I2CNDAT @ 0x02: u8 = 0_0 {
        /// I2C Data Count
        I2CNDAT: 0..7 = struct I2CNDATField(u8);
    }
    /// USART 0 Control I2C
    rw U0CTL__I2C @ 0x20: u8 = 0_0 {
        /// I2C enable
        I2CEN: 0 = struct I2CEN(bool);
        /// I2C master
        MST: 1 = struct MST(bool);
        /// USART synchronous/asynchronous
        SYNC: 2 = struct SYNC(bool);
        /// Loopback
        LISTEN: 3 = struct LISTEN(bool);
        /// I2C extended addressing
        XA: 4 = struct XA(bool);
        /// USART I2C
        I2C: 5 = struct I2C(bool);
        /// Transmit DMA enable
        TXDMAEN: 6 = struct TXDMAEN(bool);
        /// Receive DMA enable
        RXDMAEN: 7 = struct RXDMAEN(bool);
    }
    /// I2C Transfer Control
    rw I2CTCTL @ 0x21: u8 = 0_0 {
        /// Start bit
        I2CSTT: 0 = struct I2CSTT(bool);
        /// Stop bit
        I2CSTP: 1 = struct I2CSTP(bool);
        /// Start byte mode
        I2CSTB: 2 = struct I2CSTB(bool);
        /// Transmit
        I2CTRX: 3 = struct I2CTRX(bool);
        /// Clock select bit 0
        I2CSSEL: 4..5 = enum I2CSSEL {
            /// I2C clock select 0: UCLK
            I2CSSEL_0 = 0b00,
            /// I2C clock select 1: ACLK
            I2CSSEL_1 = 0b01,
            /// I2C clock select 2: SMCLK
            I2CSSEL_2 = 0b10,
            /// I2C clock select 3: SMCLK
            I2CSSEL_3 = 0b11,
        }
        /// Repeat mode
        I2CRM: 6 = struct I2CRM(bool);
        /// Word data mode
        I2CWORD: 7 = struct I2CWORD(bool);
    }
    /// I2C Data Control
    rw I2CDCTL @ 0x22: u8 = 0_0 {
        /// Bus busy
        I2CBB: 0 = struct I2CBB(bool);
        /// Receiver overrun
        I2CRXOVR: 1 = struct I2CRXOVR(bool);
        /// Transmit underflow
        I2CTXUDF: 2 = struct I2CTXUDF(bool);
        /// Received byte
        I2CSBD: 3 = struct I2CSBD(bool);
        /// SCL being held low
        I2CSCLLOW: 4 = struct I2CSCLLOW(bool);
        /// I2C Busy Flag
        I2CBUSY: 5 = struct I2CBUSY(bool);
    }
    /// I2C Pre-scaler
    rw I2CPSC @ 0x23: u8 = 0_0 {
        /// I2C Pre-scaler
        I2CPSC: 0..7 = struct I2CPSCField(u8);
    }
    /// I2C SCL High
    rw I2CSCLH @ 0x24: u8 = 0_0 {
        /// I2C SCL High
        I2CSCLH: 0..7 = struct I2CSCLHField(u8);
    }
    /// I2C SCL Low
    rw I2CSCLL @ 0x25: u8 = 0_0 {
        /// I2C SCL Low
        I2CSCLL: 0..7 = struct I2CSCLLField(u8);
    }
    /// I2C Data for Byte access
    rw I2CDRB @ 0x26: u8 = 0_0 {
        /// I2C Data for Byte access
        I2CDRB: 0..7 = struct I2CDRBField(u8);
    }
    /// I2C Data for Word access
    rw I2CDRW @ 0x26: u16 = 0_0 {
        /// I2C Data for Word access
        I2CDRW: 0..15 = struct I2CDRWField(u16);
    }
    /// I2C Own Address
    rw I2COA @ 0xc8: u16 = 0_0 {
        /// I2C Own Address
        I2COA: 0..15 = struct I2COAField(u16);
    }
    /// I2C Slave Address
    rw I2CSA @ 0xca: u16 = 0_0 {
        /// I2C Slave Address
        I2CSA: 0..15 = struct I2CSAField(u16);
    }
    /// I2C Interrupt Vector
    rw I2CIV @ 0xcc: u16 = 0_0 {
        /// I2C Interrupt Vector
        I2CIV: 0..15 = struct I2CIVField(u16);
    }
}
