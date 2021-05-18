//! PMM  Power Management System

utils::periph! {
    /// PMM  Power Management System
    PMM;
    /// PMM Control 0
    rw PMMCTL0 @ 0x00: u16 = 0_0 {
        /// PMM Core Voltage Bit: 0
        PMMCOREV: 0..1 = enum PMMCOREV {
            /// PMM Core Voltage 0 (1.35V)
            PMMCOREV_0 = 0b00,
            /// PMM Core Voltage 1 (1.55V)
            PMMCOREV_1 = 0b01,
            /// PMM Core Voltage 2 (1.75V)
            PMMCOREV_2 = 0b10,
            /// PMM Core Voltage 3 (1.85V)
            PMMCOREV_3 = 0b11,
        }
        /// PMM Software BOR
        PMMSWBOR: 2 = struct PMMSWBOR(bool);
        /// PMM Software POR
        PMMSWPOR: 3 = struct PMMSWPOR(bool);
        /// PMM Turn Regulator off
        PMMREGOFF: 4 = struct PMMREGOFF(bool);
        /// PMM Global High Power Module Request Enable
        PMMHPMRE: 7 = struct PMMHPMRE(bool);
    }
    /// PMM Control 1
    rw PMMCTL1 @ 0x02: u16 = 0_0 {
        /// PMM Reference Mode
        PMMREFMD: 0 = struct PMMREFMD(bool);
        /// PMM Voltage Regulator Current Mode Bit: 0
        PMMCMD0: 4 = struct PMMCMD0(bool);
        /// PMM Voltage Regulator Current Mode Bit: 1
        PMMCMD1: 5 = struct PMMCMD1(bool);
    }
    /// SVS and SVM high side control register
    rw SVSMHCTL @ 0x04: u16 = 0_0 {
        /// SVS and SVM high side Reset Release Voltage Level Bit: 0
        SVSMHRRL: 0..2 = enum SVSMHRRL {
            /// SVS and SVM high side Reset Release Voltage Level 0
            SVSMHRRL_0 = 0b000,
            /// SVS and SVM high side Reset Release Voltage Level 1
            SVSMHRRL_1 = 0b001,
            /// SVS and SVM high side Reset Release Voltage Level 2
            SVSMHRRL_2 = 0b010,
            /// SVS and SVM high side Reset Release Voltage Level 3
            SVSMHRRL_3 = 0b011,
            /// SVS and SVM high side Reset Release Voltage Level 4
            SVSMHRRL_4 = 0b100,
            /// SVS and SVM high side Reset Release Voltage Level 5
            SVSMHRRL_5 = 0b101,
            /// SVS and SVM high side Reset Release Voltage Level 6
            SVSMHRRL_6 = 0b110,
            /// SVS and SVM high side Reset Release Voltage Level 7
            SVSMHRRL_7 = 0b111,
        }
        /// SVS and SVM high side delay status
        SVSMHDLYST: 3 = struct SVSMHDLYST(bool);
        /// SVS high side mode
        SVSHMD: 4 = struct SVSHMD(bool);
        /// SVS and SVM high side event mask
        SVSMHEVM: 6 = struct SVSMHEVM(bool);
        /// SVS and SVM high side auto control enable
        SVSMHACE: 7 = struct SVSMHACE(bool);
        /// SVS high side reset voltage level Bit: 0
        SVSHRVL: 8..9 = enum SVSHRVL {
            /// SVS high side Reset Release Voltage Level 0
            SVSHRVL_0 = 0b00,
            /// SVS high side Reset Release Voltage Level 1
            SVSHRVL_1 = 0b01,
            /// SVS high side Reset Release Voltage Level 2
            SVSHRVL_2 = 0b10,
            /// SVS high side Reset Release Voltage Level 3
            SVSHRVL_3 = 0b11,
        }
        /// SVS high side enable
        SVSHE: 10 = struct SVSHE(bool);
        /// SVS high side full performace mode
        SVSHFP: 11 = struct SVSHFP(bool);
        /// SVM high side over-voltage enable
        SVMHOVPE: 12 = struct SVMHOVPE(bool);
        /// SVM high side enable
        SVMHE: 14 = struct SVMHE(bool);
        /// SVM high side full performace mode
        SVMHFP: 15 = struct SVMHFP(bool);
    }
    /// SVS and SVM low side control register
    rw SVSMLCTL @ 0x06: u16 = 0_0 {
        /// SVS and SVM low side Reset Release Voltage Level Bit: 0
        SVSMLRRL: 0..2 = enum SVSMLRRL {
            /// SVS and SVM low side Reset Release Voltage Level 0
            SVSMLRRL_0 = 0b000,
            /// SVS and SVM low side Reset Release Voltage Level 1
            SVSMLRRL_1 = 0b001,
            /// SVS and SVM low side Reset Release Voltage Level 2
            SVSMLRRL_2 = 0b010,
            /// SVS and SVM low side Reset Release Voltage Level 3
            SVSMLRRL_3 = 0b011,
            /// SVS and SVM low side Reset Release Voltage Level 4
            SVSMLRRL_4 = 0b100,
            /// SVS and SVM low side Reset Release Voltage Level 5
            SVSMLRRL_5 = 0b101,
            /// SVS and SVM low side Reset Release Voltage Level 6
            SVSMLRRL_6 = 0b110,
            /// SVS and SVM low side Reset Release Voltage Level 7
            SVSMLRRL_7 = 0b111,
        }
        /// SVS and SVM low side delay status
        SVSMLDLYST: 3 = struct SVSMLDLYST(bool);
        /// SVS low side mode
        SVSLMD: 4 = struct SVSLMD(bool);
        /// SVS and SVM low side event mask
        SVSMLEVM: 6 = struct SVSMLEVM(bool);
        /// SVS and SVM low side auto control enable
        SVSMLACE: 7 = struct SVSMLACE(bool);
        /// SVS low side reset voltage level Bit: 0
        SVSLRVL: 8..9 = enum SVSLRVL {
            /// SVS low side Reset Release Voltage Level 0
            SVSLRVL_0 = 0b00,
            /// SVS low side Reset Release Voltage Level 1
            SVSLRVL_1 = 0b01,
            /// SVS low side Reset Release Voltage Level 2
            SVSLRVL_2 = 0b10,
            /// SVS low side Reset Release Voltage Level 3
            SVSLRVL_3 = 0b11,
        }
        /// SVS low side enable
        SVSLE: 10 = struct SVSLE(bool);
        /// SVS low side full performace mode
        SVSLFP: 11 = struct SVSLFP(bool);
        /// SVM low side over-voltage enable
        SVMLOVPE: 12 = struct SVMLOVPE(bool);
        /// SVM low side enable
        SVMLE: 14 = struct SVMLE(bool);
        /// SVM low side full performace mode
        SVMLFP: 15 = struct SVMLFP(bool);
    }
    /// SVSIN and SVSOUT control register
    rw SVSMIO @ 0x08: u16 = 0_0 {
        /// SVM low side output enable
        SVMLOE: 3 = struct SVMLOE(bool);
        /// SVM low side voltage level reached output enable
        SVMLVLROE: 4 = struct SVMLVLROE(bool);
        /// SVMOUT pin polarity
        SVMOUTPOL: 5 = struct SVMOUTPOL(bool);
        /// SVM high side output enable
        SVMHOE: 11 = struct SVMHOE(bool);
        /// SVM high side voltage level reached output enable
        SVMHVLROE: 12 = struct SVMHVLROE(bool);
    }
    /// PMM Interrupt Flag
    rw PMMIFG @ 0x0c: u16 = 0_0 {
        /// SVS and SVM low side Delay expired interrupt flag
        SVSMLDLYIFG: 0 = struct SVSMLDLYIFG(bool);
        /// SVM low side interrupt flag
        SVMLIFG: 1 = struct SVMLIFG(bool);
        /// SVM low side Voltage Level Reached interrupt flag
        SVMLVLRIFG: 2 = struct SVMLVLRIFG(bool);
        /// SVS and SVM high side Delay expired interrupt flag
        SVSMHDLYIFG: 4 = struct SVSMHDLYIFG(bool);
        /// SVM high side interrupt flag
        SVMHIFG: 5 = struct SVMHIFG(bool);
        /// SVM high side Voltage Level Reached interrupt flag
        SVMHVLRIFG: 6 = struct SVMHVLRIFG(bool);
        /// PMM Software BOR interrupt flag
        PMMBORIFG: 8 = struct PMMBORIFG(bool);
        /// PMM RESET pin interrupt flag
        PMMRSTIFG: 9 = struct PMMRSTIFG(bool);
        /// PMM Software POR interrupt flag
        PMMPORIFG: 10 = struct PMMPORIFG(bool);
        /// SVS low side interrupt flag
        SVSHIFG: 12 = struct SVSHIFG(bool);
        /// SVS high side interrupt flag
        SVSLIFG: 13 = struct SVSLIFG(bool);
        /// LPM5 indication Flag
        PMMLPM5IFG: 15 = struct PMMLPM5IFG(bool);
    }
    /// PMM and RESET Interrupt Enable
    rw PMMRIE @ 0x0e: u16 = 0_0 {
        /// SVS and SVM low side Delay expired interrupt enable
        SVSMLDLYIE: 0 = struct SVSMLDLYIE(bool);
        /// SVM low side interrupt enable
        SVMLIE: 1 = struct SVMLIE(bool);
        /// SVM low side Voltage Level Reached interrupt enable
        SVMLVLRIE: 2 = struct SVMLVLRIE(bool);
        /// SVS and SVM high side Delay expired interrupt enable
        SVSMHDLYIE: 4 = struct SVSMHDLYIE(bool);
        /// SVM high side interrupt enable
        SVMHIE: 5 = struct SVMHIE(bool);
        /// SVM high side Voltage Level Reached interrupt enable
        SVMHVLRIE: 6 = struct SVMHVLRIE(bool);
        /// SVS low side POR enable
        SVSLPE: 8 = struct SVSLPE(bool);
        /// SVM low side Voltage Level reached POR enable
        SVMLVLRPE: 9 = struct SVMLVLRPE(bool);
        /// SVS high side POR enable
        SVSHPE: 12 = struct SVSHPE(bool);
        /// SVM high side Voltage Level reached POR enable
        SVMHVLRPE: 13 = struct SVMHVLRPE(bool);
    }
    /// PMM Power Mode 5 Control Register 0
    rw PM5CTL0 @ 0x10: u16 = 0_0 {
        /// Lock I/O pin configuration upon entry/exit to/from LPM5
        LOCKLPM5: 0 = struct LOCKLPM5(bool);
    }
}
