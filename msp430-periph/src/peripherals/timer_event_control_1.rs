//! Timer_Event_Control

utils::periph! {
    /// Timer_Event_Control
    Timer_Event_Control;
    /// Timer Event Control 0 External Control 0
    rw TEC0XCTL0 @ 0x00: u16 = 0_0 {
        /// TEV Ext. fault signal hold for CE0
        TECXFLTHLD0: 0 = struct TECXFLTHLD0(bool);
        /// TEV Ext. fault signal hold for CE1
        TECXFLTHLD1: 1 = struct TECXFLTHLD1(bool);
        /// TEV Ext. fault signal hold for CE2
        TECXFLTHLD2: 2 = struct TECXFLTHLD2(bool);
        /// TEV Ext. fault signal hold for CE3
        TECXFLTHLD3: 3 = struct TECXFLTHLD3(bool);
        /// TEV Ext. fault signal hold for CE4
        TECXFLTHLD4: 4 = struct TECXFLTHLD4(bool);
        /// TEV Ext. fault signal hold for CE5
        TECXFLTHLD5: 5 = struct TECXFLTHLD5(bool);
        /// TEV Ext. fault signal hold for CE6
        TECXFLTHLD6: 6 = struct TECXFLTHLD6(bool);
        /// TEV Ext. fault signal enable for CE0
        TECXFLTEN0: 8 = struct TECXFLTEN0(bool);
        /// TEV Ext. fault signal enable for CE1
        TECXFLTEN1: 9 = struct TECXFLTEN1(bool);
        /// TEV Ext. fault signal enable for CE2
        TECXFLTEN2: 10 = struct TECXFLTEN2(bool);
        /// TEV Ext. fault signal enable for CE3
        TECXFLTEN3: 11 = struct TECXFLTEN3(bool);
        /// TEV Ext. fault signal enable for CE4
        TECXFLTEN4: 12 = struct TECXFLTEN4(bool);
        /// TEV Ext. fault signal enable for CE5
        TECXFLTEN5: 13 = struct TECXFLTEN5(bool);
        /// TEV Ext. fault signal enable for CE6
        TECXFLTEN6: 14 = struct TECXFLTEN6(bool);
    }
    /// Timer Event Control 0 External Control 1
    rw TEC0XCTL1 @ 0x02: u16 = 0_0 {
        /// TEV Polarity Bit of ext. fault 0
        TECXFLTPOL0: 0 = struct TECXFLTPOL0(bool);
        /// TEV Polarity Bit of ext. fault 1
        TECXFLTPOL1: 1 = struct TECXFLTPOL1(bool);
        /// TEV Polarity Bit of ext. fault 2
        TECXFLTPOL2: 2 = struct TECXFLTPOL2(bool);
        /// TEV Polarity Bit of ext. fault 3
        TECXFLTPOL3: 3 = struct TECXFLTPOL3(bool);
        /// TEV Polarity Bit of ext. fault 4
        TECXFLTPOL4: 4 = struct TECXFLTPOL4(bool);
        /// TEV Polarity Bit of ext. fault 5
        TECXFLTPOL5: 5 = struct TECXFLTPOL5(bool);
        /// TEV Polarity Bit of ext. fault 6
        TECXFLTPOL6: 6 = struct TECXFLTPOL6(bool);
        /// TEV Signal Type of Ext. fault 0
        TECXFLTLVS0: 8 = struct TECXFLTLVS0(bool);
        /// TEV Signal Type of Ext. fault 1
        TECXFLTLVS1: 9 = struct TECXFLTLVS1(bool);
        /// TEV Signal Type of Ext. fault 2
        TECXFLTLVS2: 10 = struct TECXFLTLVS2(bool);
        /// TEV Signal Type of Ext. fault 3
        TECXFLTLVS3: 11 = struct TECXFLTLVS3(bool);
        /// TEV Signal Type of Ext. fault 4
        TECXFLTLVS4: 12 = struct TECXFLTLVS4(bool);
        /// TEV Signal Type of Ext. fault 5
        TECXFLTLVS5: 13 = struct TECXFLTLVS5(bool);
        /// TEV Signal Type of Ext. fault 6
        TECXFLTLVS6: 14 = struct TECXFLTLVS6(bool);
    }
    /// Timer Event Control 0 External Control 2
    rw TEC0XCTL2 @ 0x04: u16 = 0_0 {
        /// TEV Aux. Clock Select Bit: 0
        TECCLKSEL: 0..1 = enum TECCLKSEL {
            /// TEV Aux. Clock Select: CLK0
            TECCLKSEL_0 = 0b00,
            /// TEV Aux. Clock Select: CLK1
            TECCLKSEL_1 = 0b01,
            /// TEV Aux. Clock Select: CLK2
            TECCLKSEL_2 = 0b10,
            /// TEV Aux. Clock Select: CLK3
            TECCLKSEL_3 = 0b11,
        }
        /// TEV Auxilary clear signal control
        TECAXCLREN: 2 = struct TECAXCLREN(bool);
        /// TEV Ext. clear signal control
        TECEXCLREN: 3 = struct TECEXCLREN(bool);
        /// TEV External clear signal hold bit
        TECEXCLRHLD: 4 = struct TECEXCLRHLD(bool);
        /// TEV Polarity Bit of ext. clear
        TECEXCLRPOL: 5 = struct TECEXCLRPOL(bool);
        /// TEV Signal Type of Ext. clear
        TECEXCLRLVS: 6 = struct TECEXCLRLVS(bool);
    }
    /// Timer Event Control 0 Status
    rw TEC0STA @ 0x06: u16 = 0_0 {
        /// TEV External fault status flag for CE0
        TECXFLT0STA: 0 = struct TECXFLT0STA(bool);
        /// TEV External fault status flag for CE1
        TECXFLT1STA: 1 = struct TECXFLT1STA(bool);
        /// TEV External fault status flag for CE2
        TECXFLT2STA: 2 = struct TECXFLT2STA(bool);
        /// TEV External fault status flag for CE3
        TECXFLT3STA: 3 = struct TECXFLT3STA(bool);
        /// TEV External fault status flag for CE4
        TECXFLT4STA: 4 = struct TECXFLT4STA(bool);
        /// TEV External fault status flag for CE5
        TECXFLT5STA: 5 = struct TECXFLT5STA(bool);
        /// TEV External fault status flag for CE6
        TECXFLT6STA: 6 = struct TECXFLT6STA(bool);
        /// TEC External clear status flag
        TECXCLRSTA: 8 = struct TECXCLRSTA(bool);
    }
    /// Timer Event Control 0 External Interrupt
    rw TEC0XINT @ 0x08: u16 = 0_0 {
        /// TEC Aux. Clear Interrupt Flag
        TECAXCLRIFG: 0 = struct TECAXCLRIFG(bool);
        /// TEC External Clear Interrupt Flag
        TECEXCLRIFG: 1 = struct TECEXCLRIFG(bool);
        /// TEC External Fault Interrupt Flag
        TECXFLTIFG: 2 = struct TECXFLTIFG(bool);
        /// TEC Aux. Clear Interrupt Enable
        TECAXCLRIE: 8 = struct TECAXCLRIE(bool);
        /// TEC External Clear Interrupt Enable
        TECEXCLRIE: 9 = struct TECEXCLRIE(bool);
        /// TEC External Fault Interrupt Enable
        TECXFLTIE: 10 = struct TECXFLTIE(bool);
    }
    /// Timer Event Control 0 Interrupt Vector
    rw TEC0IV @ 0x0a: u16 = 0_0 {
        /// Timer Event Control 0 Interrupt Vector
        TEC0IV: 0..15 = struct TEC0IVField(u16);
    }
}
