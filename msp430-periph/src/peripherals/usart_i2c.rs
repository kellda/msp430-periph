//! USART  I2C Mode

utils::periph! {
    /// USART  I2C Mode
    USART_I2C;
    /// I2C Interrupt Enable
    rw IE @ 0x00: u8 = 0_0 {
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
    rw IFG @ 0x01: u8 = 0_0 {
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
    rw NDAT @ 0x02: u8 = 0_0 {
        /// I2C Data Count
        NDAT: 0..7 = struct NDATField(u8);
    }
    /// USART Control I2C
    rw CTL @ 0x20: u8 = 0_0 {
        /// I2C enable
        EN: 0 = struct EN(bool);
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
    rw TCTL @ 0x21: u8 = 0_0 {
        /// Start bit
        STT: 0 = struct STT(bool);
        /// Stop bit
        STP: 1 = struct STP(bool);
        /// Start byte mode
        STB: 2 = struct STB(bool);
        /// Transmit
        TRX: 3 = struct TRX(bool);
        /// Clock select bit 0
        SSEL: 4..5 = enum SSEL {
            /// I2C clock select 0: UCLK
            SSEL_0 = 0b00,
            /// I2C clock select 1: ACLK
            SSEL_1 = 0b01,
            /// I2C clock select 2: SMCLK
            SSEL_2 = 0b10,
            /// I2C clock select 3: SMCLK
            SSEL_3 = 0b11,
        }
        /// Repeat mode
        RM: 6 = struct RM(bool);
        /// Word data mode
        WORD: 7 = struct WORD(bool);
    }
    /// I2C Data Control
    rw DCTL @ 0x22: u8 = 0_0 {
        /// Bus busy
        BB: 0 = struct BB(bool);
        /// Receiver overrun
        RXOVR: 1 = struct RXOVR(bool);
        /// Transmit underflow
        TXUDF: 2 = struct TXUDF(bool);
        /// Received byte
        SBD: 3 = struct SBD(bool);
        /// SCL being held low
        SCLLOW: 4 = struct SCLLOW(bool);
        /// I2C Busy Flag
        BUSY: 5 = struct BUSY(bool);
    }
    /// I2C Pre-scaler
    rw PSC @ 0x23: u8 = 0_0 {
        /// I2C Pre-scaler
        PSC: 0..7 = struct PSCField(u8);
    }
    /// I2C SCL High
    rw SCLH @ 0x24: u8 = 0_0 {
        /// I2C SCL High
        SCLH: 0..7 = struct SCLHField(u8);
    }
    /// I2C SCL Low
    rw SCLL @ 0x25: u8 = 0_0 {
        /// I2C SCL Low
        SCLL: 0..7 = struct SCLLField(u8);
    }
    /// I2C Data for Byte access
    rw DRB @ 0x26: u8 = 0_0 {
        /// I2C Data for Byte access
        DRB: 0..7 = struct DRBField(u8);
    }
    /// I2C Data for Word access
    rw DRW @ 0x26: u16 = 0_0 {
        /// I2C Data for Word access
        DRW: 0..15 = struct DRWField(u16);
    }
    /// I2C Own Address
    rw OA @ 0xc8: u16 = 0_0 {
        /// I2C Own Address
        OA: 0..15 = struct OAField(u16);
    }
    /// I2C Slave Address
    rw SA @ 0xca: u16 = 0_0 {
        /// I2C Slave Address
        SA: 0..15 = struct SAField(u16);
    }
    /// I2C Interrupt Vector
    rw IV @ 0xcc: u16 = 0_0 {
        /// I2C Interrupt Vector
        IV: 0..15 = struct IVField(u16);
    }
}
