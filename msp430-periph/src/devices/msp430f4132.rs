//! MSP430F4132
use crate::peripherals::*;

utils::device! {
    /// MSP430F4132
    #[cfg_attr(not(feature = "MSP430F4132-all"), non_exhaustive)]
    MSP430F4132;
    /// Special Function
    #[cfg(feature = "special_function_13")]
    Special_Function @ 0x0000: special_function_13::SpecialFunction;
    /// Port 3/4
    #[cfg(feature = "port_3_4_1")]
    Port_3_4 @ 0x0018: port_3_4_1::Port34;
    /// Port 1/2
    #[cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// Port 5/6
    #[cfg(feature = "port_5_6_1")]
    Port_5_6 @ 0x0030: port_5_6_1::Port56;
    /// Port 7
    #[cfg(feature = "port_7_1")]
    Port_7 @ 0x0038: port_7_1::Port7;
    /// Basic Timer / RTC
    #[cfg(feature = "basic_timer___rtc_1")]
    Basic_Timer___RTC @ 0x0040: basic_timer___rtc_1::BasicTimerRTC;
    /// ADC10
    #[cfg(feature = "adc10_4")]
    ADC10 @ 0x0048: adc10_4::ADC10;
    /// System Clock FLLPLUS
    #[cfg(feature = "system_clock_fllplus_3")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_3::SystemClockFLLPLUS;
    /// Supply Voltage Supervisor
    #[cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0056: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// Comparator A
    #[cfg(feature = "comparator_a_2")]
    Comparator_A @ 0x0059: comparator_a_2::ComparatorA;
    /// USCI_A0  UART Mode
    #[cfg(feature = "usci_a0__uart_mode_1")]
    USCI_A0__UART_Mode @ 0x005d: usci_a0__uart_mode_1::USCI_A0UARTMode;
    /// USCI_A0  SPI Mode
    #[cfg(feature = "usci_a0__spi_mode_1")]
    USCI_A0__SPI_Mode @ 0x0060: usci_a0__spi_mode_1::USCI_A0SPIMode;
    /// USCI_B0  SPI Mode
    #[cfg(feature = "usci_b0__spi_mode_1")]
    USCI_B0__SPI_Mode @ 0x0068: usci_b0__spi_mode_1::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[cfg(feature = "usci_b0__i2c_mode_1")]
    USCI_B0__I2C_Mode @ 0x0068: usci_b0__i2c_mode_1::USCI_B0I2CMode;
    /// LCD_A
    #[cfg(feature = "lcd_a_1")]
    LCD_A @ 0x0090: lcd_a_1::LCD_A;
    /// Timer1_A5
    #[cfg(feature = "timer1_a5_1")]
    Timer1_A5 @ 0x011e: timer1_a5_1::Timer1_A5;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[cfg(feature = "flash_4")]
    Flash @ 0x0128: flash_4::Flash;
    /// Timer0_A3
    #[cfg(feature = "timer0_a3_1")]
    Timer0_A3 @ 0x012e: timer0_a3_1::Timer0_A3;
}
