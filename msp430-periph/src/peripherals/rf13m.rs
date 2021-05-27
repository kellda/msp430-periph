//! RF13M Module

utils::periph! {
    /// RF13M Module
    RF13M;
    /// RF13M Control register
    rw CTL @ 0x00: u16 = 0_0 {
        /// RF13M Enable reception of downlink
        RXEN: 0 = struct RXEN(bool);
        /// RF13M Enable transmission of uplink
        TXEN: 1 = struct TXEN(bool);
        /// RF13M Enable RF Field timeout detection
        RFTOEN: 2 = struct RFTOEN(bool);
        /// RF13M Manually select uplink subcarrier setting
        SC: 4 = struct SC(bool);
        /// RF13M Manually select uplink data rate
        DR: 5 = struct DR(bool);
        /// RF13M Enable manual configuration of ISO15693 uplink parameters
        MCFG: 7 = struct MCFG(bool);
        /// RF13M CRC valued correct or not flag
        CRCOK: 8 = struct CRCOK(bool);
        /// RF13M Indicates if a HF field was detected since last readout
        HFD: 9 = struct HFD(bool);
        /// RF13M Gate RX interrupt with CRC OK
        RXIWOC: 13 = struct RXIWOC(bool);
        /// RF13M Select between big or little endian mode
        BE: 14 = struct BE(bool);
    }
    /// RF13M Interrupt Enable/Flag register
    rw INT @ 0x02: u16 = 0_0 {
        /// RF13M Receive downlink done interrupt flag
        RXIFG: 0 = struct RXIFG(bool);
        /// RF13M Uplink transmission done interrupt flag
        TXIFG: 1 = struct TXIFG(bool);
        /// RF13M RX FIFO high watermark reached interrupt flag
        RXWMIFG: 2 = struct RXWMIFG(bool);
        /// RF13M TX FIFO low watermark reached interrupt flag
        TXWMIFG: 3 = struct TXWMIFG(bool);
        /// RF13M RX Slot marker interrupt flag
        SLIFG: 4 = struct SLIFG(bool);
        /// RF13M Over-/Under-flow condition detected
        OUFLIFG: 5 = struct OUFLIFG(bool);
        /// RF13M RX Error detected
        RXEIFG: 6 = struct RXEIFG(bool);
        /// RF13M RF Timeout detected
        RFTOIFG: 7 = struct RFTOIFG(bool);
        /// RF13M RX done interrupt enable
        RXIE: 8 = struct RXIE(bool);
        /// RF13M TX done interrupt enable
        TXIE: 9 = struct TXIE(bool);
        /// RF13M RX FIFO high watermark interrupt enable
        RXWMIE: 10 = struct RXWMIE(bool);
        /// RF13M TX FIFO low watermark interrupt enable
        TXWMIE: 11 = struct TXWMIE(bool);
        /// RF13M ISO15693 Slot marker interrupt enable
        SLIE: 12 = struct SLIE(bool);
        /// RF13M Over-/Under-flow interrupt enable
        OUFLIE: 13 = struct OUFLIE(bool);
        /// RF13M RX Error interrupt enable
        RXEIE: 14 = struct RXEIE(bool);
        /// RF13M RF Timeout detected interrupt enable
        RFTOIE: 15 = struct RFTOIE(bool);
    }
    /// RF13M Interrupt Vector register
    rw IV @ 0x04: u16 = 0_0 {
        /// RF13M Interrupt vector value bit: 0
        IV0: 1 = struct IV0(bool);
        /// RF13M Interrupt vector value bit: 1
        IV1: 2 = struct IV1(bool);
        /// RF13M Interrupt vector value bit: 2
        IV2: 3 = struct IV2(bool);
        /// RF13M Interrupt vector value bit: 3
        IV3: 4 = struct IV3(bool);
    }
    /// RF13M Receive Data FIFO register
    rw RXF @ 0x06: u16 = 0_0 {
        /// RF13M Receive Data FIFO register
        RXF: 0..15 = struct RXFField(u16);
    }
    /// RF13M Transmit Data FIFO register
    rw TXF @ 0x08: u16 = 0_0 {
        /// RF13M Transmit Data FIFO register
        TXF: 0..15 = struct TXFField(u16);
    }
    /// RF13M CRC accumulator register
    rw CRC @ 0x0a: u16 = 0_0 {
        /// RF13M CRC accumulator register
        CRC: 0..15 = struct CRCField(u16);
    }
    /// RF13M Receive/Transmit Data FIFO Fill Level register
    rw FIFOFL @ 0x0c: u16 = 0_0 {
        /// RF13M Receive/Transmit Data FIFO Fill Level register
        FIFOFL: 0..15 = struct FIFOFLField(u16);
    }
    /// RF13M Receive High/Transmit Low Watermark configuration register
    rw WMCFG @ 0x0e: u16 = 0_0 {
        /// RF13M Receive High/Transmit Low Watermark configuration register
        WMCFG: 0..15 = struct WMCFGField(u16);
    }
    /// RF13M Receive data buffer
    rw RXBUF @ 0x20: u16 = 0_0 {
        /// RF13M Receive data buffer
        RXBUF: 0..15 = struct RXBUFField(u16);
    }
    /// RF13M Transmit data buffer
    rw TXBUF @ 0x40: u16 = 0_0 {
        /// RF13M Transmit data buffer
        TXBUF: 0..15 = struct TXBUFField(u16);
    }
}
