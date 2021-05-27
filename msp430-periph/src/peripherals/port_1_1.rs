//! Port 1

utils::periph! {
    /// Port 1
    Port;
    /// Port Input
    rw IN_ @ 0x00: u8 = 0_0 {
        /// P0
        IN_P0: 0 = struct IN_P0(bool);
        /// P1
        IN_P1: 1 = struct IN_P1(bool);
        /// P2
        IN_P2: 2 = struct IN_P2(bool);
        /// P3
        IN_P3: 3 = struct IN_P3(bool);
        /// P4
        IN_P4: 4 = struct IN_P4(bool);
        /// P5
        IN_P5: 5 = struct IN_P5(bool);
        /// P6
        IN_P6: 6 = struct IN_P6(bool);
        /// P7
        IN_P7: 7 = struct IN_P7(bool);
    }
    /// Port Output
    rw OUT @ 0x01: u8 = 0_0 {
        /// P0
        OUT_P0: 0 = struct OUT_P0(bool);
        /// P1
        OUT_P1: 1 = struct OUT_P1(bool);
        /// P2
        OUT_P2: 2 = struct OUT_P2(bool);
        /// P3
        OUT_P3: 3 = struct OUT_P3(bool);
        /// P4
        OUT_P4: 4 = struct OUT_P4(bool);
        /// P5
        OUT_P5: 5 = struct OUT_P5(bool);
        /// P6
        OUT_P6: 6 = struct OUT_P6(bool);
        /// P7
        OUT_P7: 7 = struct OUT_P7(bool);
    }
    /// Port Direction
    rw DIR @ 0x02: u8 = 0_0 {
        /// P0
        DIR_P0: 0 = struct DIR_P0(bool);
        /// P1
        DIR_P1: 1 = struct DIR_P1(bool);
        /// P2
        DIR_P2: 2 = struct DIR_P2(bool);
        /// P3
        DIR_P3: 3 = struct DIR_P3(bool);
        /// P4
        DIR_P4: 4 = struct DIR_P4(bool);
        /// P5
        DIR_P5: 5 = struct DIR_P5(bool);
        /// P6
        DIR_P6: 6 = struct DIR_P6(bool);
        /// P7
        DIR_P7: 7 = struct DIR_P7(bool);
    }
    /// Port Interrupt Flag
    rw IFG @ 0x03: u8 = 0_0 {
        /// P0
        IFG_P0: 0 = struct IFG_P0(bool);
        /// P1
        IFG_P1: 1 = struct IFG_P1(bool);
        /// P2
        IFG_P2: 2 = struct IFG_P2(bool);
        /// P3
        IFG_P3: 3 = struct IFG_P3(bool);
        /// P4
        IFG_P4: 4 = struct IFG_P4(bool);
        /// P5
        IFG_P5: 5 = struct IFG_P5(bool);
        /// P6
        IFG_P6: 6 = struct IFG_P6(bool);
        /// P7
        IFG_P7: 7 = struct IFG_P7(bool);
    }
    /// Port Interrupt Edge Select
    rw IES @ 0x04: u8 = 0_0 {
        /// P0
        IES_P0: 0 = struct IES_P0(bool);
        /// P1
        IES_P1: 1 = struct IES_P1(bool);
        /// P2
        IES_P2: 2 = struct IES_P2(bool);
        /// P3
        IES_P3: 3 = struct IES_P3(bool);
        /// P4
        IES_P4: 4 = struct IES_P4(bool);
        /// P5
        IES_P5: 5 = struct IES_P5(bool);
        /// P6
        IES_P6: 6 = struct IES_P6(bool);
        /// P7
        IES_P7: 7 = struct IES_P7(bool);
    }
    /// Port Interrupt Enable
    rw IE @ 0x05: u8 = 0_0 {
        /// P0
        IE_P0: 0 = struct IE_P0(bool);
        /// P1
        IE_P1: 1 = struct IE_P1(bool);
        /// P2
        IE_P2: 2 = struct IE_P2(bool);
        /// P3
        IE_P3: 3 = struct IE_P3(bool);
        /// P4
        IE_P4: 4 = struct IE_P4(bool);
        /// P5
        IE_P5: 5 = struct IE_P5(bool);
        /// P6
        IE_P6: 6 = struct IE_P6(bool);
        /// P7
        IE_P7: 7 = struct IE_P7(bool);
    }
    /// Port Selection
    rw SEL @ 0x06: u8 = 0_0 {
        /// P0
        SEL_P0: 0 = struct SEL_P0(bool);
        /// P1
        SEL_P1: 1 = struct SEL_P1(bool);
        /// P2
        SEL_P2: 2 = struct SEL_P2(bool);
        /// P3
        SEL_P3: 3 = struct SEL_P3(bool);
        /// P4
        SEL_P4: 4 = struct SEL_P4(bool);
        /// P5
        SEL_P5: 5 = struct SEL_P5(bool);
        /// P6
        SEL_P6: 6 = struct SEL_P6(bool);
        /// P7
        SEL_P7: 7 = struct SEL_P7(bool);
    }
    /// Port Selection 2
    rw SEL2 @ 0x21: u8 = 0_0 {
        /// P0
        SEL2_P0: 0 = struct SEL2_P0(bool);
        /// P1
        SEL2_P1: 1 = struct SEL2_P1(bool);
        /// P2
        SEL2_P2: 2 = struct SEL2_P2(bool);
        /// P3
        SEL2_P3: 3 = struct SEL2_P3(bool);
        /// P4
        SEL2_P4: 4 = struct SEL2_P4(bool);
        /// P5
        SEL2_P5: 5 = struct SEL2_P5(bool);
        /// P6
        SEL2_P6: 6 = struct SEL2_P6(bool);
        /// P7
        SEL2_P7: 7 = struct SEL2_P7(bool);
    }
    /// Port Resistor Enable
    rw REN @ 0x07: u8 = 0_0 {
        /// P0
        REN_P0: 0 = struct REN_P0(bool);
        /// P1
        REN_P1: 1 = struct REN_P1(bool);
        /// P2
        REN_P2: 2 = struct REN_P2(bool);
        /// P3
        REN_P3: 3 = struct REN_P3(bool);
        /// P4
        REN_P4: 4 = struct REN_P4(bool);
        /// P5
        REN_P5: 5 = struct REN_P5(bool);
        /// P6
        REN_P6: 6 = struct REN_P6(bool);
        /// P7
        REN_P7: 7 = struct REN_P7(bool);
    }
}
