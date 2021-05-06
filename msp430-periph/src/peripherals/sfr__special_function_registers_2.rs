//! SFR  Special Function Registers

utils::periph! {
    /// SFR  Special Function Registers
    SFRSpecialFunctionRegisters;
    /// Interrupt Enable 1
    rw SFRIE1 @ 0x00: u16 = 0_0 {
        /// WDT Interrupt Enable
        WDTIE: 0 = struct WDTIE(bool);
        /// Osc Fault Enable
        OFIE: 1 = struct OFIE(bool);
        /// Vacant Memory Interrupt Enable
        VMAIE: 3 = struct VMAIE(bool);
        /// NMI Interrupt Enable
        NMIIE: 4 = struct NMIIE(bool);
        /// JTAG Mail Box input Interrupt Enable
        JMBINIE: 6 = struct JMBINIE(bool);
        /// JTAG Mail Box output Interrupt Enable
        JMBOUTIE: 7 = struct JMBOUTIE(bool);
        /// SVM Interrupt Enable
        SVMIE: 8 = struct SVMIE(bool);
    }
    /// Interrupt Flag 1
    rw SFRIFG1 @ 0x02: u16 = 0_0 {
        /// WDT Interrupt Flag
        WDTIFG: 0 = struct WDTIFG(bool);
        /// Osc Fault Flag
        OFIFG: 1 = struct OFIFG(bool);
        /// Vacant Memory Interrupt Flag
        VMAIFG: 3 = struct VMAIFG(bool);
        /// NMI Interrupt Flag
        NMIIFG: 4 = struct NMIIFG(bool);
        /// JTAG Mail Box input Interrupt Flag
        JMBINIFG: 6 = struct JMBINIFG(bool);
        /// JTAG Mail Box output Interrupt Flag
        JMBOUTIFG: 7 = struct JMBOUTIFG(bool);
        /// SVM Interrupt Flag
        SVMIFG: 8 = struct SVMIFG(bool);
    }
    /// RESET Pin Control Register
    rw SFRRPCR @ 0x04: u16 = 0_0 {
        /// NMI select
        SYSNMI: 0 = struct SYSNMI(bool);
        /// NMI edge select
        SYSNMIIES: 1 = struct SYSNMIIES(bool);
        /// RESET Pin pull down/up select
        SYSRSTUP: 2 = struct SYSRSTUP(bool);
        /// RESET Pin Resistor enable
        SYSRSTRE: 3 = struct SYSRSTRE(bool);
    }
}
