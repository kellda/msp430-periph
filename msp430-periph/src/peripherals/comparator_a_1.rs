//! Comparator A

utils::periph! {
    /// Comparator A
    ComparatorA;
    /// Comparator A Control 1
    rw CACTL1 @ 0x00: u8 = 0_0 {
        /// Comp. A Interrupt Flag
        CAIFG: 0 = struct CAIFG(bool);
        /// Comp. A Interrupt Enable
        CAIE: 1 = struct CAIE(bool);
        /// Comp. A Int. Edge Select: 0:rising / 1:falling
        CAIES: 2 = struct CAIES(bool);
        /// Comp. A enable
        CAON: 3 = struct CAON(bool);
        /// Comp. A Internal Reference Select 0
        CAREF: 4..5 = enum CAREF {
            /// Comp. A Int. Ref. Select 0 : Off
            CAREF_0 = 0b00,
            /// Comp. A Int. Ref. Select 1 : 0.25*Vcc
            CAREF_1 = 0b01,
            /// Comp. A Int. Ref. Select 2 : 0.5*Vcc
            CAREF_2 = 0b10,
            /// Comp. A Int. Ref. Select 3 : Vt
            CAREF_3 = 0b11,
        }
        /// Comp. A Internal Reference Enable
        CARSEL: 6 = struct CARSEL(bool);
        /// Comp. A Exchange Inputs
        CAEX: 7 = struct CAEX(bool);
    }
    /// Comparator A Control 2
    rw CACTL2 @ 0x01: u8 = 0_0 {
        /// Comp. A Output
        CAOUT: 0 = struct CAOUT(bool);
        /// Comp. A Enable Output Filter
        CAF: 1 = struct CAF(bool);
        /// Comp. A Connect External Signal to CA0 : 1
        P2CA0: 2 = struct P2CA0(bool);
        /// Comp. A Connect External Signal to CA1 : 1
        P2CA1: 3 = struct P2CA1(bool);
        /// CACTL24
        CACTL24: 4 = struct CACTL24(bool);
        /// CACTL25
        CACTL25: 5 = struct CACTL25(bool);
        /// CACTL26
        CACTL26: 6 = struct CACTL26(bool);
        /// CACTL27
        CACTL27: 7 = struct CACTL27(bool);
    }
    /// Comparator A Port Disable
    rw CAPD @ 0x02: u8 = 0_0 {
        /// Comp. A Disable Input Buffer of Port Register .0
        CAPD0: 0 = struct CAPD0(bool);
        /// Comp. A Disable Input Buffer of Port Register .1
        CAPD1: 1 = struct CAPD1(bool);
        /// Comp. A Disable Input Buffer of Port Register .2
        CAPD2: 2 = struct CAPD2(bool);
        /// Comp. A Disable Input Buffer of Port Register .3
        CAPD3: 3 = struct CAPD3(bool);
        /// Comp. A Disable Input Buffer of Port Register .4
        CAPD4: 4 = struct CAPD4(bool);
        /// Comp. A Disable Input Buffer of Port Register .5
        CAPD5: 5 = struct CAPD5(bool);
        /// Comp. A Disable Input Buffer of Port Register .6
        CAPD6: 6 = struct CAPD6(bool);
        /// Comp. A Disable Input Buffer of Port Register .7
        CAPD7: 7 = struct CAPD7(bool);
    }
}
