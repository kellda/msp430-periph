//! MSP430FR5738
use crate::peripherals::*;

utils::device! {
    /// MSP430FR5738
    #[cfg_attr(not(feature = "MSP430FR5738-all"), non_exhaustive)]
    MSP430FR5738;
    /// SFR  Special Function Registers
    #[cfg(feature = "sfr__special_function_registers_3")]
    SFR__Special_Function_Registers @ 0x0100: sfr__special_function_registers_3::SFRSpecialFunctionRegisters;
    /// PMM  Power Management System
    #[cfg(feature = "pmm__power_management_system_3")]
    PMM__Power_Management_System @ 0x0120: pmm__power_management_system_3::PMMPowerManagementSystem;
    /// FRAM
    #[cfg(feature = "fram_1")]
    FRAM @ 0x0140: fram_1::FRAM;
    /// CRC16
    #[cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CS  Clock System
    #[cfg(feature = "cs__clock_system_1")]
    CS__Clock_System @ 0x0160: cs__clock_system_1::CSClockSystem;
    /// SYS  System Module
    #[cfg(feature = "sys__system_module_2")]
    SYS__System_Module @ 0x0180: sys__system_module_2::SYSSystemModule;
    /// Shared Reference
    #[cfg(feature = "shared_reference_2")]
    Shared_Reference @ 0x01b0: shared_reference_2::SharedReference;
    /// Port A
    #[cfg(feature = "port_a_3")]
    Port_A @ 0x0200: port_a_3::PortA;
    /// Port 1/2
    #[cfg(feature = "port_1_2_7")]
    Port_1_2 @ 0x0200: port_1_2_7::Port12;
    /// Port J
    #[cfg(feature = "port_j_4")]
    Port_J @ 0x0320: port_j_4::PortJ;
    /// Timer0_A3
    #[cfg(feature = "timer0_a3_2")]
    Timer0_A3 @ 0x0340: timer0_a3_2::Timer0_A3;
    /// Timer1_A3
    #[cfg(feature = "timer1_a3_1")]
    Timer1_A3 @ 0x0380: timer1_a3_1::Timer1_A3;
    /// Timer0_B3
    #[cfg(feature = "timer0_b3_1")]
    Timer0_B3 @ 0x03c0: timer0_b3_1::Timer0_B3;
    /// RTC_B  Real Time Clock
    #[cfg(feature = "rtc_b__real_time_clock_1")]
    RTC_B__Real_Time_Clock @ 0x04a0: rtc_b__real_time_clock_1::RTC_BRealTimeClock;
    /// MPY 16  Multiplier  16 Bit Mode
    #[cfg(feature = "mpy_16__multiplier__16_bit_mode_1")]
    MPY_16__Multiplier__16_Bit_Mode @ 0x04c0: mpy_16__multiplier__16_bit_mode_1::MPY16Multiplier16BitMode;
    /// MPY 32  Multiplier  32 Bit Mode
    #[cfg(feature = "mpy_32__multiplier__32_bit_mode_1")]
    MPY_32__Multiplier__32_Bit_Mode @ 0x04d0: mpy_32__multiplier__32_bit_mode_1::MPY32Multiplier32BitMode;
    /// DMA
    #[cfg(feature = "dma_18")]
    DMA @ 0x0500: dma_18::DMA;
    /// MPU
    #[cfg(feature = "mpu_1")]
    MPU @ 0x05a0: mpu_1::MPU;
    /// USCI_A0  UART Mode
    #[cfg(feature = "usci_a0__uart_mode_3")]
    USCI_A0__UART_Mode @ 0x05c0: usci_a0__uart_mode_3::USCI_A0UARTMode;
    /// USCI_A0  SPI Mode
    #[cfg(feature = "usci_a0__spi_mode_3")]
    USCI_A0__SPI_Mode @ 0x05c0: usci_a0__spi_mode_3::USCI_A0SPIMode;
    /// USCI_B0  SPI Mode
    #[cfg(feature = "usci_b0__spi_mode_3")]
    USCI_B0__SPI_Mode @ 0x0640: usci_b0__spi_mode_3::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[cfg(feature = "usci_b0__i2c_mode_4")]
    USCI_B0__I2C_Mode @ 0x0640: usci_b0__i2c_mode_4::USCI_B0I2CMode;
    /// ADC10_B
    #[cfg(feature = "adc10_b_1")]
    ADC10_B @ 0x0700: adc10_b_1::ADC10_B;
    /// Comparator D
    #[cfg(feature = "comparator_d_1")]
    Comparator_D @ 0x08c0: comparator_d_1::ComparatorD;
}
