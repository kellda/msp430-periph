//! MSP430AFE223
use crate::peripherals::*;

utils::device! {
    /// MSP430AFE223
    #[all:cfg_attr(not(feature = "msp430afe223-all"), non_exhaustive)]
    MSP430AFE223;
    /// Special Function
    #[all:cfg(feature = "special_function_19")]
    Special_Function @ 0x0000: special_function_19::SpecialFunction;
    /// Port 1
    #[all:cfg(feature = "port_1_1")]
    Port_1 @ 0x0020: port_1_1::Port;
    /// Port 2
    #[all:cfg(feature = "port_2_1")]
    Port_2 @ 0x0028: port_2_1::Port;
    /// System Clock
    #[all:cfg(feature = "system_clock_3")]
    System_Clock @ 0x0053: system_clock_3::SystemClock;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0055: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// USART 0  UART/SPI Mode
    #[all:cfg(feature = "usart")]
    USART_0 @ 0x0070: usart::USART;
    /// SD24_A3
    #[all:cfg(feature = "sd24_a3_1")]
    SD24_A3 @ 0x00b0: sd24_a3_1::SD24_A3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_3")]
    Flash @ 0x0128: flash_3::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_3")]
    Timer_A3 @ 0x012e: timer_a3_3::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_3")]
    Calibration_Data @ 0x10fa: calibration_data_3::CalibrationData;
}

/// Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0xFFE2 Port 2
    PORT2 = 1,
    /// 0xFFE8 Port 1
    PORT1 = 4,
    /// 0xFFEA Timer A CC1, TA
    TIMERA1 = 5,
    /// 0xFFEC Timer A CC0
    TIMERA0 = 6,
    /// 0xFFF0 USART 0 Transmit
    USART0TX = 8,
    /// 0xFFF2 USART 0 Receive
    USART0RX = 9,
    /// 0xFFF4 Watchdog Timer
    WDT = 10,
    /// 0xFFF8 Sigma Delta ADC
    SD24 = 12,
    /// 0xFFFC Non-maskable
    NMI = 14,
}

#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT2();
    fn PORT1();
    fn TIMERA1();
    fn TIMERA0();
    fn USART0TX();
    fn USART0RX();
    fn WDT();
    fn SD24();
    fn NMI();
}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[used]
static __INTERRUPTS: [crate::Vector; 15] = [
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT2 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: PORT1 },
    crate::Vector { _handler: TIMERA1 },
    crate::Vector { _handler: TIMERA0 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: USART0TX },
    crate::Vector { _handler: USART0RX },
    crate::Vector { _handler: WDT },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: SD24 },
    crate::Vector { _reserved: 0 },
    crate::Vector { _handler: NMI },
];
