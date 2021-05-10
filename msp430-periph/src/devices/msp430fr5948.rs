//! MSP430FR5948
use crate::peripherals::*;

utils::device! {
    /// MSP430FR5948
    #[all:cfg_attr(not(feature = "MSP430FR5948-all"), non_exhaustive)]
    MSP430FR5948;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr__special_function_registers_3")]
    SFR__Special_Function_Registers @ 0x0100: sfr__special_function_registers_3::SFRSpecialFunctionRegisters;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm__power_management_system_4")]
    PMM__Power_Management_System @ 0x0120: pmm__power_management_system_4::PMMPowerManagementSystem;
    /// FRAM
    #[all:cfg(feature = "fram_2")]
    FRAM @ 0x0140: fram_2::FRAM;
    /// CRC16
    #[all:cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// CS  Clock System
    #[all:cfg(feature = "cs__clock_system_3")]
    CS__Clock_System @ 0x0160: cs__clock_system_3::CSClockSystem;
    /// SYS  System Module
    #[all:cfg(feature = "sys__system_module_3")]
    SYS__System_Module @ 0x0180: sys__system_module_3::SYSSystemModule;
    /// Shared Reference
    #[all:cfg(feature = "shared_reference_3")]
    Shared_Reference @ 0x01b0: shared_reference_3::SharedReference;
    /// Port A
    #[all:cfg(feature = "port_a_3")]
    Port_A @ 0x0200: port_a_3::PortA;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_7")]
    Port_1_2 @ 0x0200: port_1_2_7::Port12;
    /// Port B
    #[all:cfg(feature = "port_b_4")]
    Port_B @ 0x0220: port_b_4::PortB;
    /// Port 3/4
    #[all:cfg(feature = "port_3_4_7")]
    Port_3_4 @ 0x0220: port_3_4_7::Port34;
    /// Port J
    #[all:cfg(feature = "port_j_5")]
    Port_J @ 0x0320: port_j_5::PortJ;
    /// Timer0_A3
    #[all:cfg(feature = "timer0_a3_2")]
    Timer0_A3 @ 0x0340: timer0_a3_2::Timer0_A3;
    /// Timer1_A3
    #[all:cfg(feature = "timer1_a3_1")]
    Timer1_A3 @ 0x0380: timer1_a3_1::Timer1_A3;
    /// Timer0_B7
    #[all:cfg(feature = "timer0_b7_1")]
    Timer0_B7 @ 0x03c0: timer0_b7_1::Timer0_B7;
    /// Timer2_A2
    #[all:cfg(feature = "timer2_a2_1")]
    Timer2_A2 @ 0x0400: timer2_a2_1::Timer2_A2;
    /// Capacitive_Touch_IO 0
    #[all:cfg(feature = "capacitive_touch_io_0_1")]
    Capacitive_Touch_IO_0 @ 0x043e: capacitive_touch_io_0_1::Capacitive_Touch_IO0;
    /// Timer3_A2
    #[all:cfg(feature = "timer3_a2_1")]
    Timer3_A2 @ 0x0440: timer3_a2_1::Timer3_A2;
    /// Capacitive_Touch_IO 1
    #[all:cfg(feature = "capacitive_touch_io_1_1")]
    Capacitive_Touch_IO_1 @ 0x047e: capacitive_touch_io_1_1::Capacitive_Touch_IO1;
    /// RTC_B  Real Time Clock
    #[all:cfg(feature = "rtc_b__real_time_clock_1")]
    RTC_B__Real_Time_Clock @ 0x04a0: rtc_b__real_time_clock_1::RTC_BRealTimeClock;
    /// MPY 16  Multiplier  16 Bit Mode
    #[all:cfg(feature = "mpy_16__multiplier__16_bit_mode_1")]
    MPY_16__Multiplier__16_Bit_Mode @ 0x04c0: mpy_16__multiplier__16_bit_mode_1::MPY16Multiplier16BitMode;
    /// MPY 32  Multiplier  32 Bit Mode
    #[all:cfg(feature = "mpy_32__multiplier__32_bit_mode_1")]
    MPY_32__Multiplier__32_Bit_Mode @ 0x04d0: mpy_32__multiplier__32_bit_mode_1::MPY32Multiplier32BitMode;
    /// DMA
    #[all:cfg(feature = "dma_18")]
    DMA @ 0x0500: dma_18::DMA;
    /// MPU
    #[all:cfg(feature = "mpu_2")]
    MPU @ 0x05a0: mpu_2::MPU;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a0__uart_mode_3")]
    USCI_A0__UART_Mode @ 0x05c0: usci_a0__uart_mode_3::USCI_A0UARTMode;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a0__spi_mode_3")]
    USCI_A0__SPI_Mode @ 0x05c0: usci_a0__spi_mode_3::USCI_A0SPIMode;
    /// USCI_A1  UART Mode
    #[all:cfg(feature = "usci_a1__uart_mode_3")]
    USCI_A1__UART_Mode @ 0x05e0: usci_a1__uart_mode_3::USCI_A1UARTMode;
    /// USCI_A1  SPI Mode
    #[all:cfg(feature = "usci_a1__spi_mode_3")]
    USCI_A1__SPI_Mode @ 0x05e0: usci_a1__spi_mode_3::USCI_A1SPIMode;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b0__spi_mode_3")]
    USCI_B0__SPI_Mode @ 0x0640: usci_b0__spi_mode_3::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b0__i2c_mode_4")]
    USCI_B0__I2C_Mode @ 0x0640: usci_b0__i2c_mode_4::USCI_B0I2CMode;
    /// ADC12
    #[all:cfg(feature = "adc12_3")]
    ADC12 @ 0x0800: adc12_3::ADC12;
    /// Comparator E
    #[all:cfg(feature = "comparator_e_1")]
    Comparator_E @ 0x08c0: comparator_e_1::ComparatorE;
    /// AES Accelerator
    #[all:cfg(feature = "aes_accelerator_2")]
    AES_Accelerator @ 0x09c0: aes_accelerator_2::AESAccelerator;
}
