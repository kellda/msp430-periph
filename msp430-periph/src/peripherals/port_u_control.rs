//! Port U Control

utils::periph! {
    /// Port U Control
    PortUControl;
    /// LDO Controller peripheral ID and key register
    rw LDOKEYPID @ 0x00: u16 = 0_0 {
        /// LDO Controller peripheral ID and key register
        LDOKEYPID: 0..15 = struct LDOKEYPIDField(u16);
    }
    /// PU Control register
    rw PUCTL @ 0x04: u16 = 0_0 {
        /// PU - PU Output Signal Bit 0
        PUOUT0: 0 = struct PUOUT0(bool);
        /// PU - PU Output Signal Bit 1
        PUOUT1: 1 = struct PUOUT1(bool);
        /// PU - PU0/DP Input Data
        PUIN0: 2 = struct PUIN0(bool);
        /// PU - PU1/DM Input Data
        PUIN1: 3 = struct PUIN1(bool);
        /// PU - Port Output Enable
        PUOPE: 5 = struct PUOPE(bool);
        /// PU - PHY Single Ended Input enable
        PUIPE: 8 = struct PUIPE(bool);
    }
    /// LDO Power control register
    rw LDOPWRCTL @ 0x08: u16 = 0_0 {
        /// PU - LDOO Overload Interrupt Flag
        LDOOVLIFG: 0 = struct LDOOVLIFG(bool);
        /// PU - LDOI "Coming ON" Interrupt Flag
        LDOONIFG: 1 = struct LDOONIFG(bool);
        /// PU - LDOI "Going OFF" Interrupt Flag
        LDOOFFIFG: 2 = struct LDOOFFIFG(bool);
        /// PU - LDO Bandgap and LDOI valid
        LDOBGVBV: 3 = struct LDOBGVBV(bool);
        /// PU - LDO overload auto off enable
        OVLAOFF: 5 = struct OVLAOFF(bool);
        /// PU - Overload indication Interrupt Enable
        LDOOVLIE: 8 = struct LDOOVLIE(bool);
        /// PU - LDOI "Coming ON" Interrupt Enable
        LDOONIE: 9 = struct LDOONIE(bool);
        /// PU - LDOI "Going OFF" Interrupt Enable
        LDOOFFIE: 10 = struct LDOOFFIE(bool);
        /// PU - LDO Enable (3.3V)
        LDOEN: 11 = struct LDOEN(bool);
    }
}
