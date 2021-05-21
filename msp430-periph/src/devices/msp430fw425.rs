//! MSP430FW425
use crate::peripherals::*;

utils::device! {
    /// MSP430FW425
    #[all:cfg_attr(not(feature = "msp430fw425-all"), non_exhaustive)]
    MSP430FW425;
    /// Special Function
    #[all:cfg(feature = "special_function_6")]
    Special_Function @ 0x0000: special_function_6::SpecialFunction;
    /// Port 3
    #[all:cfg(feature = "port_x_1")]
    Port_3 @ 0x0018: port_x_1::Port;
    /// Port 4
    #[all:cfg(feature = "port_x_1")]
    Port_4 @ 0x001c: port_x_1::Port;
    /// Port 1
    #[all:cfg(feature = "port_12_1")]
    Port_1 @ 0x0020: port_12_1::Port;
    /// Port 2
    #[all:cfg(feature = "port_12_1")]
    Port_2 @ 0x0028: port_12_1::Port;
    /// Port 5
    #[all:cfg(feature = "port_x_1")]
    Port_5 @ 0x0030: port_x_1::Port;
    /// Port 6
    #[all:cfg(feature = "port_x_1")]
    Port_6 @ 0x0034: port_x_1::Port;
    /// Basic Timer
    #[all:cfg(feature = "basic_timer")]
    Basic_Timer @ 0x0040: basic_timer::BasicTimer;
    /// System Clock FLLPLUS
    #[all:cfg(feature = "system_clock_fllplus_1")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_1::SystemClockFLLPLUS;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0056: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
    /// LCD
    #[all:cfg(feature = "lcd_2")]
    LCD @ 0x0090: lcd_2::LCD;
    /// Timer1_A5
    #[all:cfg(feature = "timer_a5_2")]
    Timer1_A5 @ 0x011e: timer_a5_2::TimerA5;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer0_A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer0_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// Scan Interface
    #[all:cfg(feature = "scan_interface")]
    Scan_Interface @ 0x01b0: scan_interface::ScanInterface;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE0 Basic Timer
    BASICTIMER = 0,
    /// 0xFFE2 Port 2
    PORT2 = 1,
    /// 0xFFE8 Port 1
    PORT1 = 4,
    /// 0xFFEA Timer0_A CC1-2, TA0
    TIMER0_A1 = 5,
    /// 0xFFEC Timer0_A CC0
    TIMER0_A0 = 6,
    /// 0xFFF2 Scan Inteface
    SCANIF = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF6 Comparator A
    COMPARATORA = 11,
    /// 0xFFF8 Timer1_A CC1-4, TA1
    TIMER1_A1 = 12,
    /// 0xFFFA Timer1_A CC0
    TIMER1_A0 = 13,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn BASICTIMER();
    fn PORT2();
    fn PORT1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn SCANIF();
    fn WDT();
    fn COMPARATORA();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _handler: BASICTIMER },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMER0_A1 },
    crate::Vector { _handler: TIMER0_A0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: SCANIF },
    crate::Vector { _handler: WDT },
    crate::Vector { _handler: COMPARATORA },
    crate::Vector { _handler: TIMER1_A1 },
    crate::Vector { _handler: TIMER1_A0 },
    crate::Vector { _handler: NMI },
];
