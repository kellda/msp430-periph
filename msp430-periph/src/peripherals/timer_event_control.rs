//! Timer_Event_Control

utils::periph! {
    /// Timer_Event_Control
    Timer_Event_Control;
    /// Timer Event Control External Control 0
    rw XCTL0 @ 0x00: u16 = 0_0 {
        /// TEV Ext. fault signal hold for CE0
        XFLTHLD0: 0 = struct XFLTHLD0(bool);
        /// TEV Ext. fault signal hold for CE1
        XFLTHLD1: 1 = struct XFLTHLD1(bool);
        /// TEV Ext. fault signal hold for CE2
        XFLTHLD2: 2 = struct XFLTHLD2(bool);
        /// TEV Ext. fault signal hold for CE3
        XFLTHLD3: 3 = struct XFLTHLD3(bool);
        /// TEV Ext. fault signal hold for CE4
        XFLTHLD4: 4 = struct XFLTHLD4(bool);
        /// TEV Ext. fault signal hold for CE5
        XFLTHLD5: 5 = struct XFLTHLD5(bool);
        /// TEV Ext. fault signal hold for CE6
        XFLTHLD6: 6 = struct XFLTHLD6(bool);
        /// TEV Ext. fault signal enable for CE0
        XFLTEN0: 8 = struct XFLTEN0(bool);
        /// TEV Ext. fault signal enable for CE1
        XFLTEN1: 9 = struct XFLTEN1(bool);
        /// TEV Ext. fault signal enable for CE2
        XFLTEN2: 10 = struct XFLTEN2(bool);
        /// TEV Ext. fault signal enable for CE3
        XFLTEN3: 11 = struct XFLTEN3(bool);
        /// TEV Ext. fault signal enable for CE4
        XFLTEN4: 12 = struct XFLTEN4(bool);
        /// TEV Ext. fault signal enable for CE5
        XFLTEN5: 13 = struct XFLTEN5(bool);
        /// TEV Ext. fault signal enable for CE6
        XFLTEN6: 14 = struct XFLTEN6(bool);
    }
    /// Timer Event Control External Control 1
    rw XCTL1 @ 0x02: u16 = 0_0 {
        /// TEV Polarity Bit of ext. fault 0
        XFLTPOL0: 0 = struct XFLTPOL0(bool);
        /// TEV Polarity Bit of ext. fault 1
        XFLTPOL1: 1 = struct XFLTPOL1(bool);
        /// TEV Polarity Bit of ext. fault 2
        XFLTPOL2: 2 = struct XFLTPOL2(bool);
        /// TEV Polarity Bit of ext. fault 3
        XFLTPOL3: 3 = struct XFLTPOL3(bool);
        /// TEV Polarity Bit of ext. fault 4
        XFLTPOL4: 4 = struct XFLTPOL4(bool);
        /// TEV Polarity Bit of ext. fault 5
        XFLTPOL5: 5 = struct XFLTPOL5(bool);
        /// TEV Polarity Bit of ext. fault 6
        XFLTPOL6: 6 = struct XFLTPOL6(bool);
        /// TEV Signal Type of Ext. fault 0
        XFLTLVS0: 8 = struct XFLTLVS0(bool);
        /// TEV Signal Type of Ext. fault 1
        XFLTLVS1: 9 = struct XFLTLVS1(bool);
        /// TEV Signal Type of Ext. fault 2
        XFLTLVS2: 10 = struct XFLTLVS2(bool);
        /// TEV Signal Type of Ext. fault 3
        XFLTLVS3: 11 = struct XFLTLVS3(bool);
        /// TEV Signal Type of Ext. fault 4
        XFLTLVS4: 12 = struct XFLTLVS4(bool);
        /// TEV Signal Type of Ext. fault 5
        XFLTLVS5: 13 = struct XFLTLVS5(bool);
        /// TEV Signal Type of Ext. fault 6
        XFLTLVS6: 14 = struct XFLTLVS6(bool);
    }
    /// Timer Event Control External Control 2
    rw XCTL2 @ 0x04: u16 = 0_0 {
        /// TEV Aux. Clock Select Bit: 0
        CLKSEL: 0..1 = enum CLKSEL {
            /// TEV Aux. Clock Select: CLK0
            CLKSEL_0 = 0b00,
            /// TEV Aux. Clock Select: CLK1
            CLKSEL_1 = 0b01,
            /// TEV Aux. Clock Select: CLK2
            CLKSEL_2 = 0b10,
            /// TEV Aux. Clock Select: CLK3
            CLKSEL_3 = 0b11,
        }
        /// TEV Auxilary clear signal control
        AXCLREN: 2 = struct AXCLREN(bool);
        /// TEV Ext. clear signal control
        EXCLREN: 3 = struct EXCLREN(bool);
        /// TEV External clear signal hold bit
        EXCLRHLD: 4 = struct EXCLRHLD(bool);
        /// TEV Polarity Bit of ext. clear
        EXCLRPOL: 5 = struct EXCLRPOL(bool);
        /// TEV Signal Type of Ext. clear
        EXCLRLVS: 6 = struct EXCLRLVS(bool);
    }
    /// Timer Event Control Status
    rw STA @ 0x06: u16 = 0_0 {
        /// TEV External fault status flag for CE0
        XFLT0STA: 0 = struct XFLT0STA(bool);
        /// TEV External fault status flag for CE1
        XFLT1STA: 1 = struct XFLT1STA(bool);
        /// TEV External fault status flag for CE2
        XFLT2STA: 2 = struct XFLT2STA(bool);
        /// TEV External fault status flag for CE3
        XFLT3STA: 3 = struct XFLT3STA(bool);
        /// TEV External fault status flag for CE4
        XFLT4STA: 4 = struct XFLT4STA(bool);
        /// TEV External fault status flag for CE5
        XFLT5STA: 5 = struct XFLT5STA(bool);
        /// TEV External fault status flag for CE6
        XFLT6STA: 6 = struct XFLT6STA(bool);
        ///  External clear status flag
        XCLRSTA: 8 = struct XCLRSTA(bool);
    }
    /// Timer Event Control External Interrupt
    rw XINT @ 0x08: u16 = 0_0 {
        /// TEC Aux. Clear Interrupt Flag
        AXCLRIFG: 0 = struct AXCLRIFG(bool);
        /// TEC External Clear Interrupt Flag
        EXCLRIFG: 1 = struct EXCLRIFG(bool);
        /// TEC External Fault Interrupt Flag
        XFLTIFG: 2 = struct XFLTIFG(bool);
        /// TEC Aux. Clear Interrupt Enable
        AXCLRIE: 8 = struct AXCLRIE(bool);
        /// TEC External Clear Interrupt Enable
        EXCLRIE: 9 = struct EXCLRIE(bool);
        /// TEC External Fault Interrupt Enable
        XFLTIE: 10 = struct XFLTIE(bool);
    }
    /// Timer Event Control Interrupt Vector
    rw IV @ 0x0a: u16 = 0_0 {
        /// Timer Event Control Interrupt Vector
        IV: 0..15 = struct IVField(u16);
    }
}
