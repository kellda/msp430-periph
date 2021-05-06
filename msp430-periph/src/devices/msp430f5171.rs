//! MSP430F5171
use crate::peripherals::*;

utils::device! {
    /// MSP430F5171
    #[cfg_attr(not(feature = "MSP430F5171-all"), non_exhaustive)]
    MSP430F5171;
    /// SFR  Special Function Registers
    #[cfg(feature = "sfr__special_function_registers_1")]
    SFR__Special_Function_Registers @ 0x0100: sfr__special_function_registers_1::SFRSpecialFunctionRegisters;
    /// PMM  Power Management System
    #[cfg(feature = "pmm__power_management_system_2")]
    PMM__Power_Management_System @ 0x0120: pmm__power_management_system_2::PMMPowerManagementSystem;
    /// Flash
    #[cfg(feature = "flash_6")]
    Flash @ 0x0140: flash_6::Flash;
    /// CRC16
    #[cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// RC  RAM Control Module
    #[cfg(feature = "rc__ram_control_module_1")]
    RC__RAM_Control_Module @ 0x0158: rc__ram_control_module_1::RCRAMControlModule;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// UCS  Unified System Clock
    #[cfg(feature = "ucs__unified_system_clock_5")]
    UCS__Unified_System_Clock @ 0x0160: ucs__unified_system_clock_5::UCSUnifiedSystemClock;
    /// SYS  System Module
    #[cfg(feature = "sys__system_module_2")]
    SYS__System_Module @ 0x0180: sys__system_module_2::SYSSystemModule;
    /// Shared Reference
    #[cfg(feature = "shared_reference_1")]
    Shared_Reference @ 0x01b0: shared_reference_1::SharedReference;
    /// Port Mapping Control
    #[cfg(feature = "port_mapping_control_1")]
    Port_Mapping_Control @ 0x01c0: port_mapping_control_1::PortMappingControl;
    /// Port Mapping Port 1
    #[cfg(feature = "port_mapping_port_1_1")]
    Port_Mapping_Port_1 @ 0x01c8: port_mapping_port_1_1::PortMappingPort1;
    /// Port Mapping Port 2
    #[cfg(feature = "port_mapping_port_2_1")]
    Port_Mapping_Port_2 @ 0x01d0: port_mapping_port_2_1::PortMappingPort2;
    /// Port Mapping Port 3
    #[cfg(feature = "port_mapping_port_3_1")]
    Port_Mapping_Port_3 @ 0x01d8: port_mapping_port_3_1::PortMappingPort3;
    /// Port A
    #[cfg(feature = "port_a_1")]
    Port_A @ 0x0200: port_a_1::PortA;
    /// Port 1/2
    #[cfg(feature = "port_1_2_5")]
    Port_1_2 @ 0x0200: port_1_2_5::Port12;
    /// Port B
    #[cfg(feature = "port_b_1")]
    Port_B @ 0x0220: port_b_1::PortB;
    /// Port 3
    #[cfg(feature = "port_3_2")]
    Port_3 @ 0x0220: port_3_2::Port3;
    /// Port J
    #[cfg(feature = "port_j_2")]
    Port_J @ 0x0320: port_j_2::PortJ;
    /// Timer0_A3
    #[cfg(feature = "timer0_a3_3")]
    Timer0_A3 @ 0x03c0: timer0_a3_3::Timer0_A3;
    /// MPY 16  Multiplier  16 Bit Mode
    #[cfg(feature = "mpy_16__multiplier__16_bit_mode_1")]
    MPY_16__Multiplier__16_Bit_Mode @ 0x04c0: mpy_16__multiplier__16_bit_mode_1::MPY16Multiplier16BitMode;
    /// MPY 32  Multiplier  32 Bit Mode
    #[cfg(feature = "mpy_32__multiplier__32_bit_mode_1")]
    MPY_32__Multiplier__32_Bit_Mode @ 0x04d0: mpy_32__multiplier__32_bit_mode_1::MPY32Multiplier32BitMode;
    /// DMA
    #[cfg(feature = "dma_17")]
    DMA @ 0x0500: dma_17::DMA;
    /// USCI_A0  UART Mode
    #[cfg(feature = "usci_a0__uart_mode_2")]
    USCI_A0__UART_Mode @ 0x05c0: usci_a0__uart_mode_2::USCI_A0UARTMode;
    /// USCI_A0  SPI Mode
    #[cfg(feature = "usci_a0__spi_mode_2")]
    USCI_A0__SPI_Mode @ 0x05c0: usci_a0__spi_mode_2::USCI_A0SPIMode;
    /// USCI_B0  SPI Mode
    #[cfg(feature = "usci_b0__spi_mode_2")]
    USCI_B0__SPI_Mode @ 0x05e0: usci_b0__spi_mode_2::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[cfg(feature = "usci_b0__i2c_mode_3")]
    USCI_B0__I2C_Mode @ 0x05e0: usci_b0__i2c_mode_3::USCI_B0I2CMode;
    /// Comparator B
    #[cfg(feature = "comparator_b_1")]
    Comparator_B @ 0x08c0: comparator_b_1::ComparatorB;
    /// Timer0_D3
    #[cfg(feature = "timer0_d3_1")]
    Timer0_D3 @ 0x0b00: timer0_d3_1::Timer0_D3;
    /// Timer1_D3
    #[cfg(feature = "timer1_d3_1")]
    Timer1_D3 @ 0x0b40: timer1_d3_1::Timer1_D3;
    /// Timer_Event_Control
    #[cfg(feature = "timer_event_control_1")]
    Timer_Event_Control @ 0x0c00: timer_event_control_1::Timer_Event_Control;
    /// Timer_Event_Control
    #[cfg(feature = "timer_event_control_2")]
    Timer_Event_Control1 @ 0x0c20: timer_event_control_2::Timer_Event_Control;
}
