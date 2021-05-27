//! Comparator A

utils::periph! {
    /// Comparator A
    ComparatorA;
    /// Comparator A Control 1
    rw CTL1 @ 0x00: u8 = 0_0 {
        /// Comp. A Interrupt Flag
        IFG: 0 = struct IFG(bool);
        /// Comp. A Interrupt Enable
        IE: 1 = struct IE(bool);
        /// Comp. A Int. Edge Select: 0:rising / 1:falling
        IES: 2 = struct IES(bool);
        /// Comp. A enable
        ON: 3 = struct ON(bool);
        /// Comp. A Internal Reference Select 0
        REF: 4..5 = enum REF {
            /// Comp. A Int. Ref. Select 0 : Off
            REF_0 = 0b00,
            /// Comp. A Int. Ref. Select 1 : 0.25*Vcc
            REF_1 = 0b01,
            /// Comp. A Int. Ref. Select 2 : 0.5*Vcc
            REF_2 = 0b10,
            /// Comp. A Int. Ref. Select 3 : Vt
            REF_3 = 0b11,
        }
        /// Comp. A Internal Reference Enable
        RSEL: 6 = struct RSEL(bool);
        /// Comp. A Exchange Inputs
        EX: 7 = struct EX(bool);
    }
    /// Comparator A Control 2
    rw CTL2 @ 0x01: u8 = 0_0 {
        /// Comp. A Output
        OUT: 0 = struct OUT(bool);
        /// Comp. A Enable Output Filter
        F: 1 = struct F(bool);
        /// Comp. A +Terminal Multiplexer
        P2CA0: 2 = struct P2CA0(bool);
        /// Comp. A -Terminal Multiplexer
        P2CA1: 3 = struct P2CA1(bool);
        /// Comp. A -Terminal Multiplexer
        P2CA2: 4 = struct P2CA2(bool);
        /// Comp. A -Terminal Multiplexer
        P2CA3: 5 = struct P2CA3(bool);
        /// Comp. A +Terminal Multiplexer
        P2CA4: 6 = struct P2CA4(bool);
        /// Comp. A Short + and - Terminals
        SHORT: 7 = struct SHORT(bool);
    }
    /// Comparator A Port Disable
    rw PD @ 0x02: u8 = 0_0 {
        /// Comp. A Disable Input Buffer of Port Register .0
        PD0: 0 = struct PD0(bool);
        /// Comp. A Disable Input Buffer of Port Register .1
        PD1: 1 = struct PD1(bool);
        /// Comp. A Disable Input Buffer of Port Register .2
        PD2: 2 = struct PD2(bool);
        /// Comp. A Disable Input Buffer of Port Register .3
        PD3: 3 = struct PD3(bool);
        /// Comp. A Disable Input Buffer of Port Register .4
        PD4: 4 = struct PD4(bool);
        /// Comp. A Disable Input Buffer of Port Register .5
        PD5: 5 = struct PD5(bool);
        /// Comp. A Disable Input Buffer of Port Register .6
        PD6: 6 = struct PD6(bool);
        /// Comp. A Disable Input Buffer of Port Register .7
        PD7: 7 = struct PD7(bool);
    }
}
