//! Port B

utils::periph! {
    /// Port B
    PortB;
    /// Port B Input
    rw PBIN @ 0x00: u16 = 0_0 {
        /// PBIN0
        PBIN0: 0 = struct PBIN0(bool);
        /// PBIN1
        PBIN1: 1 = struct PBIN1(bool);
        /// PBIN2
        PBIN2: 2 = struct PBIN2(bool);
        /// PBIN3
        PBIN3: 3 = struct PBIN3(bool);
        /// PBIN4
        PBIN4: 4 = struct PBIN4(bool);
        /// PBIN5
        PBIN5: 5 = struct PBIN5(bool);
        /// PBIN6
        PBIN6: 6 = struct PBIN6(bool);
        /// PBIN7
        PBIN7: 7 = struct PBIN7(bool);
        /// PBIN8
        PBIN8: 8 = struct PBIN8(bool);
        /// PBIN9
        PBIN9: 9 = struct PBIN9(bool);
        /// PBIN10
        PBIN10: 10 = struct PBIN10(bool);
        /// PBIN11
        PBIN11: 11 = struct PBIN11(bool);
        /// PBIN12
        PBIN12: 12 = struct PBIN12(bool);
        /// PBIN13
        PBIN13: 13 = struct PBIN13(bool);
        /// PBIN14
        PBIN14: 14 = struct PBIN14(bool);
        /// PBIN15
        PBIN15: 15 = struct PBIN15(bool);
    }
    /// Port B Output
    rw PBOUT @ 0x02: u16 = 0_0 {
        /// PBOUT0
        PBOUT0: 0 = struct PBOUT0(bool);
        /// PBOUT1
        PBOUT1: 1 = struct PBOUT1(bool);
        /// PBOUT2
        PBOUT2: 2 = struct PBOUT2(bool);
        /// PBOUT3
        PBOUT3: 3 = struct PBOUT3(bool);
        /// PBOUT4
        PBOUT4: 4 = struct PBOUT4(bool);
        /// PBOUT5
        PBOUT5: 5 = struct PBOUT5(bool);
        /// PBOUT6
        PBOUT6: 6 = struct PBOUT6(bool);
        /// PBOUT7
        PBOUT7: 7 = struct PBOUT7(bool);
        /// PBOUT8
        PBOUT8: 8 = struct PBOUT8(bool);
        /// PBOUT9
        PBOUT9: 9 = struct PBOUT9(bool);
        /// PBOUT10
        PBOUT10: 10 = struct PBOUT10(bool);
        /// PBOUT11
        PBOUT11: 11 = struct PBOUT11(bool);
        /// PBOUT12
        PBOUT12: 12 = struct PBOUT12(bool);
        /// PBOUT13
        PBOUT13: 13 = struct PBOUT13(bool);
        /// PBOUT14
        PBOUT14: 14 = struct PBOUT14(bool);
        /// PBOUT15
        PBOUT15: 15 = struct PBOUT15(bool);
    }
    /// Port B Direction
    rw PBDIR @ 0x04: u16 = 0_0 {
        /// PBDIR0
        PBDIR0: 0 = struct PBDIR0(bool);
        /// PBDIR1
        PBDIR1: 1 = struct PBDIR1(bool);
        /// PBDIR2
        PBDIR2: 2 = struct PBDIR2(bool);
        /// PBDIR3
        PBDIR3: 3 = struct PBDIR3(bool);
        /// PBDIR4
        PBDIR4: 4 = struct PBDIR4(bool);
        /// PBDIR5
        PBDIR5: 5 = struct PBDIR5(bool);
        /// PBDIR6
        PBDIR6: 6 = struct PBDIR6(bool);
        /// PBDIR7
        PBDIR7: 7 = struct PBDIR7(bool);
        /// PBDIR8
        PBDIR8: 8 = struct PBDIR8(bool);
        /// PBDIR9
        PBDIR9: 9 = struct PBDIR9(bool);
        /// PBDIR10
        PBDIR10: 10 = struct PBDIR10(bool);
        /// PBDIR11
        PBDIR11: 11 = struct PBDIR11(bool);
        /// PBDIR12
        PBDIR12: 12 = struct PBDIR12(bool);
        /// PBDIR13
        PBDIR13: 13 = struct PBDIR13(bool);
        /// PBDIR14
        PBDIR14: 14 = struct PBDIR14(bool);
        /// PBDIR15
        PBDIR15: 15 = struct PBDIR15(bool);
    }
    /// Port B Resistor Enable
    rw PBREN @ 0x06: u16 = 0_0 {
        /// PBREN0
        PBREN0: 0 = struct PBREN0(bool);
        /// PBREN1
        PBREN1: 1 = struct PBREN1(bool);
        /// PBREN2
        PBREN2: 2 = struct PBREN2(bool);
        /// PBREN3
        PBREN3: 3 = struct PBREN3(bool);
        /// PBREN4
        PBREN4: 4 = struct PBREN4(bool);
        /// PBREN5
        PBREN5: 5 = struct PBREN5(bool);
        /// PBREN6
        PBREN6: 6 = struct PBREN6(bool);
        /// PBREN7
        PBREN7: 7 = struct PBREN7(bool);
        /// PBREN8
        PBREN8: 8 = struct PBREN8(bool);
        /// PBREN9
        PBREN9: 9 = struct PBREN9(bool);
        /// PBREN10
        PBREN10: 10 = struct PBREN10(bool);
        /// PBREN11
        PBREN11: 11 = struct PBREN11(bool);
        /// PBREN12
        PBREN12: 12 = struct PBREN12(bool);
        /// PBREN13
        PBREN13: 13 = struct PBREN13(bool);
        /// PBREN14
        PBREN14: 14 = struct PBREN14(bool);
        /// PBREN15
        PBREN15: 15 = struct PBREN15(bool);
    }
    /// Port B Drive Strenght
    rw PBDS @ 0x08: u16 = 0_0 {
        /// PBDS0
        PBDS0: 0 = struct PBDS0(bool);
        /// PBDS1
        PBDS1: 1 = struct PBDS1(bool);
        /// PBDS2
        PBDS2: 2 = struct PBDS2(bool);
        /// PBDS3
        PBDS3: 3 = struct PBDS3(bool);
        /// PBDS4
        PBDS4: 4 = struct PBDS4(bool);
        /// PBDS5
        PBDS5: 5 = struct PBDS5(bool);
        /// PBDS6
        PBDS6: 6 = struct PBDS6(bool);
        /// PBDS7
        PBDS7: 7 = struct PBDS7(bool);
        /// PBDS8
        PBDS8: 8 = struct PBDS8(bool);
        /// PBDS9
        PBDS9: 9 = struct PBDS9(bool);
        /// PBDS10
        PBDS10: 10 = struct PBDS10(bool);
        /// PBDS11
        PBDS11: 11 = struct PBDS11(bool);
        /// PBDS12
        PBDS12: 12 = struct PBDS12(bool);
        /// PBDS13
        PBDS13: 13 = struct PBDS13(bool);
        /// PBDS14
        PBDS14: 14 = struct PBDS14(bool);
        /// PBDS15
        PBDS15: 15 = struct PBDS15(bool);
    }
    /// Port B Selection
    rw PBSEL @ 0x0a: u16 = 0_0 {
        /// PBSEL0
        PBSEL0: 0 = struct PBSEL0(bool);
        /// PBSEL1
        PBSEL1: 1 = struct PBSEL1(bool);
        /// PBSEL2
        PBSEL2: 2 = struct PBSEL2(bool);
        /// PBSEL3
        PBSEL3: 3 = struct PBSEL3(bool);
        /// PBSEL4
        PBSEL4: 4 = struct PBSEL4(bool);
        /// PBSEL5
        PBSEL5: 5 = struct PBSEL5(bool);
        /// PBSEL6
        PBSEL6: 6 = struct PBSEL6(bool);
        /// PBSEL7
        PBSEL7: 7 = struct PBSEL7(bool);
        /// PBSEL8
        PBSEL8: 8 = struct PBSEL8(bool);
        /// PBSEL9
        PBSEL9: 9 = struct PBSEL9(bool);
        /// PBSEL10
        PBSEL10: 10 = struct PBSEL10(bool);
        /// PBSEL11
        PBSEL11: 11 = struct PBSEL11(bool);
        /// PBSEL12
        PBSEL12: 12 = struct PBSEL12(bool);
        /// PBSEL13
        PBSEL13: 13 = struct PBSEL13(bool);
        /// PBSEL14
        PBSEL14: 14 = struct PBSEL14(bool);
        /// PBSEL15
        PBSEL15: 15 = struct PBSEL15(bool);
    }
    /// Port B Interrupt Edge Select
    rw PBIES @ 0x18: u16 = 0_0 {
        /// PBIES0
        PBIES0: 0 = struct PBIES0(bool);
        /// PBIES1
        PBIES1: 1 = struct PBIES1(bool);
        /// PBIES2
        PBIES2: 2 = struct PBIES2(bool);
        /// PBIES3
        PBIES3: 3 = struct PBIES3(bool);
        /// PBIES4
        PBIES4: 4 = struct PBIES4(bool);
        /// PBIES5
        PBIES5: 5 = struct PBIES5(bool);
        /// PBIES6
        PBIES6: 6 = struct PBIES6(bool);
        /// PBIES7
        PBIES7: 7 = struct PBIES7(bool);
        /// PBIES8
        PBIES8: 8 = struct PBIES8(bool);
        /// PBIES9
        PBIES9: 9 = struct PBIES9(bool);
        /// PBIES10
        PBIES10: 10 = struct PBIES10(bool);
        /// PBIES11
        PBIES11: 11 = struct PBIES11(bool);
        /// PBIES12
        PBIES12: 12 = struct PBIES12(bool);
        /// PBIES13
        PBIES13: 13 = struct PBIES13(bool);
        /// PBIES14
        PBIES14: 14 = struct PBIES14(bool);
        /// PBIES15
        PBIES15: 15 = struct PBIES15(bool);
    }
    /// Port B Interrupt Enable
    rw PBIE @ 0x1a: u16 = 0_0 {
        /// PBIE0
        PBIE0: 0 = struct PBIE0(bool);
        /// PBIE1
        PBIE1: 1 = struct PBIE1(bool);
        /// PBIE2
        PBIE2: 2 = struct PBIE2(bool);
        /// PBIE3
        PBIE3: 3 = struct PBIE3(bool);
        /// PBIE4
        PBIE4: 4 = struct PBIE4(bool);
        /// PBIE5
        PBIE5: 5 = struct PBIE5(bool);
        /// PBIE6
        PBIE6: 6 = struct PBIE6(bool);
        /// PBIE7
        PBIE7: 7 = struct PBIE7(bool);
        /// PBIE8
        PBIE8: 8 = struct PBIE8(bool);
        /// PBIE9
        PBIE9: 9 = struct PBIE9(bool);
        /// PBIE10
        PBIE10: 10 = struct PBIE10(bool);
        /// PBIE11
        PBIE11: 11 = struct PBIE11(bool);
        /// PBIE12
        PBIE12: 12 = struct PBIE12(bool);
        /// PBIE13
        PBIE13: 13 = struct PBIE13(bool);
        /// PBIE14
        PBIE14: 14 = struct PBIE14(bool);
        /// PBIE15
        PBIE15: 15 = struct PBIE15(bool);
    }
    /// Port B Interrupt Flag
    rw PBIFG @ 0x1c: u16 = 0_0 {
        /// PBIFG0
        PBIFG0: 0 = struct PBIFG0(bool);
        /// PBIFG1
        PBIFG1: 1 = struct PBIFG1(bool);
        /// PBIFG2
        PBIFG2: 2 = struct PBIFG2(bool);
        /// PBIFG3
        PBIFG3: 3 = struct PBIFG3(bool);
        /// PBIFG4
        PBIFG4: 4 = struct PBIFG4(bool);
        /// PBIFG5
        PBIFG5: 5 = struct PBIFG5(bool);
        /// PBIFG6
        PBIFG6: 6 = struct PBIFG6(bool);
        /// PBIFG7
        PBIFG7: 7 = struct PBIFG7(bool);
        /// PBIFG8
        PBIFG8: 8 = struct PBIFG8(bool);
        /// PBIFG9
        PBIFG9: 9 = struct PBIFG9(bool);
        /// PBIFG10
        PBIFG10: 10 = struct PBIFG10(bool);
        /// PBIFG11
        PBIFG11: 11 = struct PBIFG11(bool);
        /// PBIFG12
        PBIFG12: 12 = struct PBIFG12(bool);
        /// PBIFG13
        PBIFG13: 13 = struct PBIFG13(bool);
        /// PBIFG14
        PBIFG14: 14 = struct PBIFG14(bool);
        /// PBIFG15
        PBIFG15: 15 = struct PBIFG15(bool);
    }
}
