//! USB Control

utils::periph! {
    /// USB Control
    USBControl;
    /// USB Controller key register
    rw KEYID @ 0x00: u16 = 0_0 {
        /// USB Controller key register
        KEYID: 0..15 = struct KEYIDField(u16);
    }
    /// USB Module  configuration register
    rw CNF @ 0x02: u16 = 0_0 {
        /// USB - Module enable
        USB_EN: 0 = struct USB_EN(bool);
        /// USB - PUR pin enable
        PUR_EN: 1 = struct PUR_EN(bool);
        /// USB - PUR pin input value
        PUR_IN: 2 = struct PUR_IN(bool);
        /// USB - Block ready signal for DMA
        BLKRDY: 3 = struct BLKRDY(bool);
        /// USB - Frame Number receive Trigger enable for DMA
        FNTEN: 4 = struct FNTEN(bool);
    }
    /// USB PHY control register
    rw PHYCTL @ 0x04: u16 = 0_0 {
        /// USB - USB Port Output Signal Bit 0
        PUOUT0: 0 = struct PUOUT0(bool);
        /// USB - USB Port Output Signal Bit 1
        PUOUT1: 1 = struct PUOUT1(bool);
        /// USB - PU0/DP Input Data
        PUIN0: 2 = struct PUIN0(bool);
        /// USB - PU1/DM Input Data
        PUIN1: 3 = struct PUIN1(bool);
        /// USB - USB Port Output Enable
        PUOPE: 5 = struct PUOPE(bool);
        /// USB - USB Port Function Select
        PUSEL: 7 = struct PUSEL(bool);
        /// USB - PHY Single Ended Input enable
        PUIPE: 8 = struct PUIPE(bool);
    }
    /// USB Power control register
    rw PWRCTL @ 0x08: u16 = 0_0 {
        /// USB - VUSB Overload Interrupt Flag
        VUOVLIFG: 0 = struct VUOVLIFG(bool);
        /// USB - VBUS "Coming ON" Interrupt Flag
        VBONIFG: 1 = struct VBONIFG(bool);
        /// USB - VBUS "Going OFF" Interrupt Flag
        VBOFFIFG: 2 = struct VBOFFIFG(bool);
        /// USB - USB Bandgap and VBUS valid
        USBBGVBV: 3 = struct USBBGVBV(bool);
        /// USB - VBUS on/off events enable
        USBDETEN: 4 = struct USBDETEN(bool);
        /// USB - LDO overload auto off enable
        OVLAOFF: 5 = struct OVLAOFF(bool);
        /// USB - Secondary LDO auto on enable
        SLDOAON: 6 = struct SLDOAON(bool);
        /// USB - Overload indication Interrupt Enable
        VUOVLIE: 8 = struct VUOVLIE(bool);
        /// USB - VBUS "Coming ON" Interrupt Enable
        VBONIE: 9 = struct VBONIE(bool);
        /// USB - VBUS "Going OFF" Interrupt Enable
        VBOFFIE: 10 = struct VBOFFIE(bool);
        /// USB - LDO Enable (3.3V)
        VUSBEN: 11 = struct VUSBEN(bool);
        /// USB - Secondary LDO Enable (1.8V)
        SLDOEN: 12 = struct SLDOEN(bool);
    }
    /// USB PLL control register
    rw PLLCTL @ 0x10: u16 = 0_0 {
        /// USB - Module Clock Select Bit 0
        UCLKSEL: 6..7 = enum UCLKSEL {
            /// USB - Module Clock Select: 0
            UCLKSEL_0 = 0b00,
            /// USB - Module Clock Select: 1
            UCLKSEL_1 = 0b01,
            /// USB - Module Clock Select: 2
            UCLKSEL_2 = 0b10,
            /// USB - Module Clock Select: 3 (Reserved)
            UCLKSEL_3 = 0b11,
        }
        /// USB - PLL enable
        UPLLEN: 8 = struct UPLLEN(bool);
        /// USB - Phase Freq. Discriminator enable
        UPFDEN: 9 = struct UPFDEN(bool);
    }
    /// USB PLL Clock Divider Buffer control register
    rw PLLDIVB @ 0x12: u16 = 0_0 {
        /// USB - PLL feedback divider buffer Bit 0
        UPMB0: 0 = struct UPMB0(bool);
        /// USB - PLL feedback divider buffer Bit 1
        UPMB1: 1 = struct UPMB1(bool);
        /// USB - PLL feedback divider buffer Bit 2
        UPMB2: 2 = struct UPMB2(bool);
        /// USB - PLL feedback divider buffer Bit 3
        UPMB3: 3 = struct UPMB3(bool);
        /// USB - PLL feedback divider buffer Bit 4
        UPMB4: 4 = struct UPMB4(bool);
        /// USB - PLL feedback divider buffer Bit 5
        UPMB5: 5 = struct UPMB5(bool);
        /// USB - PLL prescale divider buffer Bit 0
        UPQB0: 8 = struct UPQB0(bool);
        /// USB - PLL prescale divider buffer Bit 1
        UPQB1: 9 = struct UPQB1(bool);
        /// USB - PLL prescale divider buffer Bit 2
        UPQB2: 10 = struct UPQB2(bool);
    }
    /// USB PLL Interrupt control register
    rw PLLIR @ 0x14: u16 = 0_0 {
        /// USB - PLL out of lock Interrupt Flag
        OOLIFG: 0 = struct OOLIFG(bool);
        /// USB - PLL loss of signal Interrupt Flag
        LOSIFG: 1 = struct LOSIFG(bool);
        /// USB - PLL out of range Interrupt Flag
        OORIFG: 2 = struct OORIFG(bool);
        /// USB - PLL out of lock Interrupt enable
        OOLIE: 8 = struct OOLIE(bool);
        /// USB - PLL loss of signal Interrupt enable
        LOSIE: 9 = struct LOSIE(bool);
        /// USB - PLL out of range Interrupt enable
        OORIE: 10 = struct OORIE(bool);
    }
    /// USB Input endpoint: Configuration
    rw IEPCNF @ 0x20: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        IEP_IIE: 2 = struct IEP_IIE(bool);
        /// USB - Stall Condition
        IEP_STALL: 3 = struct IEP_STALL(bool);
        /// USB - Toggle Bit
        IEP_TOGGLE: 5 = struct IEP_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        IEP_UBME: 7 = struct IEP_UBME(bool);
    }
    /// USB Input endpoint: Byte Count
    rw IEPCNT @ 0x21: u8 = 0_0 {
        /// USB Input endpoint: Byte Count
        IEPCNT: 0..7 = struct IEPCNTField(u8);
    }
    /// USB Output endpoint: Configuration
    rw OEPCNF_0 @ 0x22: u8 = 0_0 {
        /// USB - Transaction Interrupt indication enable
        OEP_IIE: 2 = struct OEP_IIE(bool);
        /// USB - Stall Condition
        OEP_STALL: 3 = struct OEP_STALL(bool);
        /// USB - Toggle Bit
        OEP_TOGGLE: 5 = struct OEP_TOGGLE(bool);
        /// USB - UBM In-Endpoint Enable
        OEP_UBME: 7 = struct OEP_UBME(bool);
    }
    /// USB Output endpoint: byte count
    rw OEPCNT @ 0x23: u8 = 0_0 {
        /// USB Output endpoint: byte count
        OEPCNT: 0..7 = struct OEPCNTField(u8);
    }
    /// USB Input endpoint interrupt enable flags
    rw IEPIE @ 0x2e: u8 = 0_0 {
        /// USB Input endpoint interrupt enable flags
        IEPIE: 0..7 = struct IEPIEField(u8);
    }
    /// USB Output endpoint interrupt enable flags
    rw OEPIE @ 0x2f: u8 = 0_0 {
        /// USB Output endpoint interrupt enable flags
        OEPIE: 0..7 = struct OEPIEField(u8);
    }
    /// USB Input endpoint interrupt flags
    rw IEPIFG @ 0x30: u8 = 0_0 {
        /// USB Input endpoint interrupt flags
        IEPIFG: 0..7 = struct IEPIFGField(u8);
    }
    /// USB Output endpoint interrupt flags
    rw OEPIFG @ 0x31: u8 = 0_0 {
        /// USB Output endpoint interrupt flags
        OEPIFG: 0..7 = struct OEPIFGField(u8);
    }
    /// USB Vector interrupt register
    rw VECINT @ 0x32: u16 = 0_0 {
        /// USB Vector interrupt register
        VECINT: 0..15 = struct VECINTField(u16);
    }
    /// USB maintenance register
    rw MAINT @ 0x36: u16 = 0_0 {
        /// USB - Timer Interrupt Flag
        UTIFG: 0 = struct UTIFG(bool);
        /// USB - Timer Interrupt Enable
        UTIE: 1 = struct UTIE(bool);
        /// USB - Time Stamp Generator Enable
        TSGEN: 8 = struct TSGEN(bool);
        /// USB - Time Stamp Event Select Bit 0
        TSESEL: 9..10 = enum TSESEL {
            /// USB - Time Stamp Event Select: 0
            TSESEL_0 = 0b00,
            /// USB - Time Stamp Event Select: 1
            TSESEL_1 = 0b01,
            /// USB - Time Stamp Event Select: 2
            TSESEL_2 = 0b10,
            /// USB - Time Stamp Event Select: 3
            TSESEL_3 = 0b11,
        }
        /// USB - Time Stamp Event #3 Bit
        TSE3: 11 = struct TSE3(bool);
        /// USB - Timer Select Bit 0
        UTSEL: 13..15 = enum UTSEL {
            /// USB - Timer Select: 0
            UTSEL_0 = 0b000,
            /// USB - Timer Select: 1
            UTSEL_1 = 0b001,
            /// USB - Timer Select: 2
            UTSEL_2 = 0b010,
            /// USB - Timer Select: 3
            UTSEL_3 = 0b011,
            /// USB - Timer Select: 4
            UTSEL_4 = 0b100,
            /// USB - Timer Select: 5
            UTSEL_5 = 0b101,
            /// USB - Timer Select: 6
            UTSEL_6 = 0b110,
            /// USB - Timer Select: 7
            UTSEL_7 = 0b111,
        }
    }
    /// USB Time Stamp register
    rw TSREG @ 0x38: u16 = 0_0 {
        /// USB Time Stamp register
        TSREG: 0..15 = struct TSREGField(u16);
    }
    /// USB Frame number
    rw FN_ @ 0x3a: u16 = 0_0 {
        /// USB Frame number
        FN: 0..15 = struct FNField(u16);
    }
    /// USB control register
    rw CTL @ 0x3c: u8 = 0_0 {
        /// USB - Data Response Bit
        DIR: 0 = struct DIR(bool);
        /// USB - Function Reset Connection Enable
        FRSTE: 4 = struct FRSTE(bool);
        /// USB - Device Remote Wakeup Request
        RWUP: 5 = struct RWUP(bool);
        /// USB - Function Enable Bit
        FEN: 6 = struct FEN(bool);
    }
    /// USB interrupt enable register
    rw IE @ 0x3d: u8 = 0_0 {
        /// USB - Setup Overwrite Interrupt Enable
        STPOWIE: 0 = struct STPOWIE(bool);
        /// USB - Setup Interrupt Enable
        SETUPIE: 2 = struct SETUPIE(bool);
        /// USB - Function Resume Request Interrupt Enable
        RESRIE: 5 = struct RESRIE(bool);
        /// USB - Function Suspend Request Interrupt Enable
        SUSRIE: 6 = struct SUSRIE(bool);
        /// USB - Function Reset Request Interrupt Enable
        RSTRIE: 7 = struct RSTRIE(bool);
    }
    /// USB interrupt flag register
    rw IFG @ 0x3e: u8 = 0_0 {
        /// USB - Setup Overwrite Interrupt Flag
        STPOWIFG: 0 = struct STPOWIFG(bool);
        /// USB - Setup Interrupt Flag
        SETUPIFG: 2 = struct SETUPIFG(bool);
        /// USB - Function Resume Request Interrupt Flag
        RESRIFG: 5 = struct RESRIFG(bool);
        /// USB - Function Suspend Request Interrupt Flag
        SUSRIFG: 6 = struct SUSRIFG(bool);
        /// USB - Function Reset Request Interrupt Flag
        RSTRIFG: 7 = struct RSTRIFG(bool);
    }
    /// USB Function address register
    rw FUNADR @ 0x3f: u8 = 0_0 {
        /// USB Function address register
        FUNADR: 0..7 = struct FUNADRField(u8);
    }
}
