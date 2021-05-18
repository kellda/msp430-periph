//! SFR

utils::periph! {
    /// SFR
    SFR;
    /// Interrupt Enable
    rw SFRIE1 @ 0x00: u16 = 0_0 {
        /// Watchdog timer interrupt enable
        WDTIE: 0 = enum WDTIE {
            /// Interrupts disabled
            DISABLE = 0b0,
            /// Interrupts enabled
            ENABLE = 0b1,
        }
        /// Oscillator fault interrupt enable
        OFIE: 1 = enum OFIE {
            /// Interrupts disabled
            DISABLE = 0b0,
            /// Interrupts enabled
            ENABLE = 0b1,
        }
        /// Vacant memory access interrupt enable
        VMAIE: 3 = enum VMAIE {
            /// Interrupts disabled
            DISABLE = 0b0,
            /// Interrupts enabled
            ENABLE = 0b1,
        }
        /// NMI pin interrupt enable
        NMIIE: 4 = enum NMIIE {
            /// Interrupts disabled
            DISABLE = 0b0,
            /// Interrupts enabled
            ENABLE = 0b1,
        }
        /// JTAG mailbox input interrupt enable
        JMBINIE: 6 = enum JMBINIE {
            /// Interrupts disabled
            DISABLE = 0b0,
            /// Interrupts enabled
            ENABLE = 0b1,
        }
        /// JTAG mailbox output interrupt enable
        JMBOUTIE: 7 = enum JMBOUTIE {
            /// Interrupts disabled
            DISABLE = 0b0,
            /// Interrupts enabled
            ENABLE = 0b1,
        }
    }
    /// Interrupt Flag
    rw SFRIFG1 @ 0x02: u16 = 0_0 {
        /// Oscillator fault interrupt flag
        OFIFG: 1 = enum OFIFG {
            /// No interrupt pending
            OFIFG_0 = 0b0,
            /// Interrupt pending
            OFIFG_1 = 0b1,
        }
        /// Vacant memory access interrupt flag
        VMAIFG: 3 = enum VMAIFG {
            /// No interrupt pending
            VMAIFG_0 = 0b0,
            /// Interrupt pending
            VMAIFG_1 = 0b1,
        }
        /// NMI pin interrupt flag
        NMIIFG: 4 = enum NMIIFG {
            /// No interrupt pending
            NMIIFG_0 = 0b0,
            /// Interrupt pending
            NMIIFG_1 = 0b1,
        }
        /// Watchdog timer interrupt flag
        WDTIFG: 0 = enum WDTIFG {
            /// No interrupt pending
            WDTIFG_0 = 0b0,
            /// Interrupt pending
            WDTIFG_1 = 0b1,
        }
        /// JTAG mailbox input interrupt flag
        JMBINIFG: 6 = enum JMBINIFG {
            /// No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBI0 is read by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBI0 and JMBI1 have been read by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read
            JMBINIFG_0 = 0b0,
            /// Interrupt pending. A message is waiting in the JMBIN registers. In 16-bit mode (JMBMODE = 0) when JMBI0 has been written by the JTAG module. In 32-bit mode (JMBMODE = 1) when JMBI0 and JMBI1 have been written by the JTAG module.
            JMBINIFG_1 = 0b1,
        }
        /// JTAG mailbox output interrupt flag
        JMBOUTIFG: 7 = enum JMBOUTIFG {
            /// No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBO0 has been written with a new message to the JTAG module by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBO0 and JMBO1 have been written with new messages to the JTAG module by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read.
            JMBOUTIFG_0 = 0b0,
            /// Interrupt pending. JMBO registers are ready for new messages. In 16-bit mode (JMBMODE = 0), JMBO0 has been received by the JTAG module and is ready for a new message from the CPU. In 32-bit mode (JMBMODE = 1), JMBO0 and JMBO1 have been received by the JTAG module and are ready for new messages from the CPU.
            JMBOUTIFG_1 = 0b1,
        }
    }
    /// Reset Pin Control
    rw SFRRPCR @ 0x04: u16 = 0_0 {
        /// NMI select
        SYSNMI: 0 = enum SYSNMI {
            /// Reset function
            RESET = 0b0,
            /// NMI function
            NMI = 0b1,
        }
        /// NMI edge select
        SYSNMIIES: 1 = enum SYSNMIIES {
            /// NMI on rising edge
            RISING = 0b0,
            /// NMI on falling edge
            FALLING = 0b1,
        }
        /// Reset resistor pin pullup or pulldown
        SYSRSTUP: 2 = enum SYSRSTUP {
            /// Pulldown is selected
            PULLDOWN = 0b0,
            /// Pullup is selected
            PULLUP = 0b1,
        }
        /// Reset pin resistor enable
        SYSRSTRE: 3 = enum SYSRSTRE {
            /// Pullup or pulldown resistor at the RST/NMI pin is disabled
            DISABLE = 0b0,
            /// Pullup or pulldown resistor at the RST/NMI pin is enabled
            ENABLE = 0b1,
        }
    }
}
