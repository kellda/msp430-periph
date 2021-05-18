//! RF13M Module

utils::periph! {
    /// RF13M Module
    RF13M;
    /// RF13M Control register
    rw RF13MCTL @ 0x00: u16 = 0_0 {
        /// RF13M Enable reception of downlink
        RF13MRXEN: 0 = struct RF13MRXEN(bool);
        /// RF13M Enable transmission of uplink
        RF13MTXEN: 1 = struct RF13MTXEN(bool);
        /// RF13M Enable RF Field timeout detection
        RF13MRFTOEN: 2 = struct RF13MRFTOEN(bool);
        /// RF13M Manually select uplink subcarrier setting
        RF13MSC: 4 = struct RF13MSC(bool);
        /// RF13M Manually select uplink data rate
        RF13MDR: 5 = struct RF13MDR(bool);
        /// RF13M Enable manual configuration of ISO15693 uplink parameters
        RF13MMCFG: 7 = struct RF13MMCFG(bool);
        /// RF13M CRC valued correct or not flag
        RF13MCRCOK: 8 = struct RF13MCRCOK(bool);
        /// RF13M Indicates if a HF field was detected since last readout
        RF13MHFD: 9 = struct RF13MHFD(bool);
        /// RF13M Gate RX interrupt with CRC OK
        RF13MRXIWOC: 13 = struct RF13MRXIWOC(bool);
        /// RF13M Select between big or little endian mode
        RF13MBE: 14 = struct RF13MBE(bool);
    }
    /// RF13M Interrupt Enable/Flag register
    rw RF13MINT @ 0x02: u16 = 0_0 {
        /// RF13M Receive downlink done interrupt flag
        RF13MRXIFG: 0 = struct RF13MRXIFG(bool);
        /// RF13M Uplink transmission done interrupt flag
        RF13MTXIFG: 1 = struct RF13MTXIFG(bool);
        /// RF13M RX FIFO high watermark reached interrupt flag
        RF13MRXWMIFG: 2 = struct RF13MRXWMIFG(bool);
        /// RF13M TX FIFO low watermark reached interrupt flag
        RF13MTXWMIFG: 3 = struct RF13MTXWMIFG(bool);
        /// RF13M RX Slot marker interrupt flag
        RF13MSLIFG: 4 = struct RF13MSLIFG(bool);
        /// RF13M Over-/Under-flow condition detected
        RF13MOUFLIFG: 5 = struct RF13MOUFLIFG(bool);
        /// RF13M RX Error detected
        RF13MRXEIFG: 6 = struct RF13MRXEIFG(bool);
        /// RF13M RF Timeout detected
        RF13MRFTOIFG: 7 = struct RF13MRFTOIFG(bool);
        /// RF13M RX done interrupt enable
        RF13MRXIE: 8 = struct RF13MRXIE(bool);
        /// RF13M TX done interrupt enable
        RF13MTXIE: 9 = struct RF13MTXIE(bool);
        /// RF13M RX FIFO high watermark interrupt enable
        RF13MRXWMIE: 10 = struct RF13MRXWMIE(bool);
        /// RF13M TX FIFO low watermark interrupt enable
        RF13MTXWMIE: 11 = struct RF13MTXWMIE(bool);
        /// RF13M ISO15693 Slot marker interrupt enable
        RF13MSLIE: 12 = struct RF13MSLIE(bool);
        /// RF13M Over-/Under-flow interrupt enable
        RX13MOUFLIE: 13 = struct RX13MOUFLIE(bool);
        /// RF13M RX Error interrupt enable
        RX13MRXEIE: 14 = struct RX13MRXEIE(bool);
        /// RF13M RF Timeout detected interrupt enable
        RX13MRFTOIE: 15 = struct RX13MRFTOIE(bool);
    }
    /// RF13M Interrupt Vector register
    rw RF13MIV @ 0x04: u16 = 0_0 {
        /// RF13M Interrupt vector value bit: 0
        RF13MIV0: 1 = struct RF13MIV0(bool);
        /// RF13M Interrupt vector value bit: 1
        RF13MIV1: 2 = struct RF13MIV1(bool);
        /// RF13M Interrupt vector value bit: 2
        RF13MIV2: 3 = struct RF13MIV2(bool);
        /// RF13M Interrupt vector value bit: 3
        RF13MIV3: 4 = struct RF13MIV3(bool);
    }
    /// RF13M Receive Data FIFO register
    rw RF13MRXF @ 0x06: u16 = 0_0 {
        /// RF13M Receive Data FIFO register
        RF13MRXF: 0..15 = struct RF13MRXFField(u16);
    }
    /// RF13M Transmit Data FIFO register
    rw RF13MTXF @ 0x08: u16 = 0_0 {
        /// RF13M Transmit Data FIFO register
        RF13MTXF: 0..15 = struct RF13MTXFField(u16);
    }
    /// RF13M CRC accumulator register
    rw RF13MCRC @ 0x0a: u16 = 0_0 {
        /// RF13M CRC accumulator register
        RF13MCRC: 0..15 = struct RF13MCRCField(u16);
    }
    /// RF13M Receive/Transmit Data FIFO Fill Level register
    rw RF13MFIFOFL @ 0x0c: u16 = 0_0 {
        /// RF13M Receive/Transmit Data FIFO Fill Level register
        RF13MFIFOFL: 0..15 = struct RF13MFIFOFLField(u16);
    }
    /// RF13M Receive High/Transmit Low Watermark configuration register
    rw RF13MWMCFG @ 0x0e: u16 = 0_0 {
        /// RF13M Receive High/Transmit Low Watermark configuration register
        RF13MWMCFG: 0..15 = struct RF13MWMCFGField(u16);
    }
    /// RF13M Receive data buffer
    rw RF13MRXBUF @ 0x20: u16 = 0_0 {
        /// RF13M Receive data buffer
        RF13MRXBUF: 0..15 = struct RF13MRXBUFField(u16);
    }
    /// RF13M Transmit data buffer
    rw RF13MTXBUF @ 0x40: u16 = 0_0 {
        /// RF13M Transmit data buffer
        RF13MTXBUF: 0..15 = struct RF13MTXBUFField(u16);
    }
}
