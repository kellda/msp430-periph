//! LEA_SC

utils::periph! {
    /// LEA_SC
    LEA_SC;
    /// LEASC Capability Register
    r LEASCCAP @ 0x00: u32 = 0_0 {
        ///  LEA Code Memory Size. This register identifies the size of available code RAM
        LEAMSIZ: 0..3 = enum LEAMSIZ {
            /// no code RAM
            LEAMSIZ_0 = 0b0000,
            /// 1KB Code RAM
            LEAMSIZ_1 = 0b0001,
        }
    }
    /// Configuration Register 0
    rw LEASCCNF0 @ 0x04: u32 = 0_0 {
        /// LEA_SC module software restart. Setting this bit to one restarts the LEA_SC module. As long this bit remains set to one the LEA-SC is held in Restart. (The LEA_SC accessible memory behaves as system RAM)
        LEASCSWRST: 0 = struct LEASCSWRST(bool);
        ///  Hold on faults and NMIs for all pending LEA_SC operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)
        LEASCFTHOLD: 1 = enum LEASCFTHOLD {
            /// LEA-SC transfers continue on faults/NMIs
            LEASCFTHOLD_0 = 0b0,
            /// LEA-SC transfers enter HOLD on faults/NMIs
            LEASCFTHOLD_1 = 0b1,
        }
        ///  This bit defined if command execution shall be continued in LPM modes
        LEASCLPR: 8 = enum LEASCLPR {
            /// LEA-SC command execution stops in deep low power modes
            LEASCLPR_0 = 0b0,
            /// LEA-SC command execution continues in deep low power modes
            LEASCLPR_1 = 0b1,
        }
        ///  This bit defines if a "Command done interrupt" shall be triggered in LPM mode
        LEASCILPM: 10 = enum LEASCILPM {
            /// Interrupt of LEA-SC is suppressed in LPM mode until AM is entered then the LEA-SC interrupt is triggered as well
            LEASCILPM_0 = 0b0,
            /// Interrupt of LEA-SC is always triggered on completion of an LEA command
            LEASCILPM_1 = 0b1,
        }
        ///  LEA_SC instruction loop buffer disable. Debugging function for LEA_SC (leave it zero).
        LEASCILB: 11 = struct LEASCILB(bool);
        ///  LEA_SC module timer fault enable.
        LEASCTIMFLTE: 13 = enum LEASCTIMFLTE {
            /// LEA_SC module timer timeout will not cause a fault indication
            LEASCTIMFLT_0 = 0b0,
            /// LEA_SC module timer timeout will cause a fault indication. LEA_SC stops operation and enters "Ready-state".
            LEASCTIMFLTE_1 = 0b1,
        }
        ///  LEASCHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults.
        LEASCCFLT: 14 = enum LEASCCFLT {
            /// LEASCHPCFLT is disabled
            LEASCCFLT_0 = 0b0,
            /// LEASCHPCFLT is enabled
            LEASCCFLT_1 = 0b1,
        }
        ///  Enable bit on memory faults.
        LEASCMEMFLTE: 15 = enum LEASCMEMFLTE {
            /// LEA_SC memory faults are disabled
            LEASCMEMFLTE_0 = 0b0,
            /// LEA_SC memory faults are enabled
            LEASCMEMFLTE_1 = 0b1,
        }
        ///  LEA_SC done event indication and set flag. This bit indicated the done event for LEA_SC. This bit can be set by writing a one to it. Writing a zero has no effect.
        LEASCDONES: 16 = struct LEASCDONES(bool);
        ///  LEA_SC free event indication and set flag. This bit indicated the free event for LEA_SC. This bit can be set by writing a one to it. Writing a zero has no effect.
        LEASCFREES: 17 = struct LEASCFREES(bool);
        ///  LEA_SC timeout fault indication and set flag; This bits indicates that timer timeout occurred. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module (A package option)
        LEASCTIMFLTS: 21 = struct LEASCTIMFLTS(bool);
        ///  LEASCHPCFLTS when hardware trigger enabled later LEA_SC command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a "User-NMI" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module.
        LEASCCFLTS: 22 = enum LEASCCFLTS {
            /// No command fault occurred since this bit was cleared
            LEASCCFLTS_0 = 0b0,
            /// At least one command fault occurred since this bit was cleared
            LEASCCFLTS_1 = 0b1,
        }
        /// LEA_SC memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEASCCNF1. LEASCWRSTAT and LEASCCNF1.LEASCRDSTAT.
        LEASCMEMFLTS: 23 = enum LEASCMEMFLTS {
            /// No memory fault occurred since this bit was cleared
            LEASCMEMFLTS_0 = 0b0,
            /// At least one memory fault since this bit was cleared
            LEASCMEMFLTS_1 = 0b1,
        }
        ///  LEA_SC module timer reset. Setting this bit to one clears LEA_SC module timer. This bit is self clearing and will always be read as zero.
        LEASCTRST: 24 = struct LEASCTRST(bool);
        ///  LEA_SC timer enable; writing a one to this bit enables LEA_SC timer operations.
        LEASCTEN: 25 = struct LEASCTEN(bool);
        ///  LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK
        LEASCTISEL: 28..31 = enum LEASCTISEL {
            /// Timeout period: 128 x t#sub#CLK#/sub# (16 us at 8 MHz); Interval period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz)
            LEASCTISEL_0 = 0b0000,
            /// Timeout period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz); Interval period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz)
            LEASCTISEL_1 = 0b0001,
            /// Timeout period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz); Interval period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz)
            LEASCTISEL_2 = 0b0010,
            /// Timeout period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz); Interval period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz)
            LEASCTISEL_3 = 0b0011,
            /// Timeout period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz); Interval period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz)
            LEASCTISEL_4 = 0b0100,
            /// Timeout period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz); Interval period: 8192 x t#sub#CLK#/sub# (1ms at 8 MHz)
            LEASCTISEL_5 = 0b0101,
            /// Timeout period: 8192 x t#sub#CLK#/sub# (1 ms at 8 MHz); Interval period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz)
            LEASCTISEL_6 = 0b0110,
            /// Timeout period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz); Interval period: 32768 x t#sub#CLK#/sub# (4 ms at 8 MHz)
            LEASCTISEL_7 = 0b0111,
            /// Timeout period: 32768 x t#sub#CLK#/sub# (4ms at 8 MHz); Interval period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz)
            LEASCTISEL_8 = 0b1000,
            /// Timeout period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz); Interval period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz)
            LEASCTISEL_9 = 0b1001,
            /// Timeout period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz); Interval period: 262144 x t#sub#CLK#/sub# (32 ms at 8 MHz)
            LEASCTISEL_10 = 0b1010,
            /// Timeout period: 524288 x t#sub#CLK#/sub# (65 ms at 8 MHz); Interval period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz)
            LEASCTISEL_11 = 0b1011,
            /// Timeout period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz); Interval period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz)
            LEASCTISEL_12 = 0b1100,
            /// Timeout period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz); Interval period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz)
            LEASCTISEL_13 = 0b1101,
            /// Timeout period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz); Interval period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz)
            LEASCTISEL_14 = 0b1110,
            /// Timeout period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz); Interval period: 16777216 x t#sub#CLK#/sub# (2.1 s at 8 MHz)
            LEASCTISEL_15 = 0b1111,
        }
    }
    /// Configuration Register 1
    rw LEASCCNF1 @ 0x08: u32 = 0_0 {
        ///  This bit indicate if LEA_SC is able to accept new Commands (SUSPEND is always accepted)
        LEASCBUSY: 0 = enum LEASCBUSY {
            /// LEA_SC is in Ready can accept new commands
            READY = 0b0,
            /// LEA_SC is busy right now and cannot accept any commands
            BUSY = 0b1,
        }
        ///  These Bits indicate the actual operation mode LEA_SC is in.  Other = Reserved
        LEASCMODE: 4..7 = enum LEASCMODE {
            /// Off (implicit)
            OFF = 0b0000,
            /// Ready
            READY = 0b0001,
            /// RunS (SUSPEND)
            RUNS = 0b0010,
            /// RunR (RESUME)
            RUNR = 0b0011,
            /// RunA (regular command operation )
            RUNA = 0b0100,
            /// Notify
            NOTIFY = 0b0101,
            /// Sleep
            SLEEP = 0b0110,
            /// RunL
            RUNL = 0b0111,
        }
        ///  These bits indicate the current power consumption as a relative value. The value zero indicated only static operation (usually clock less). This register might be read out for statistical power estimation of an application. These bits are also reflected in J-STATE when debugging
        LEASCPWST: 8..11 = struct LEASCPWST(u32);
        ///  These bits are used to store the internal state of the application specific processor (ASIP) inside the accelerator core. The specific meaning of those bit patterns is not shown in this document.
        LEASCASST: 12..15 = struct LEASCASST(u32);
        ///  LEA_SC done event indication and clear flag. This bit indicated the done event for LEA_SC. This bit is cleared by writing a one to it. Writing a zero has no effect.
        LEASCDONEC: 16 = struct LEASCDONEC(bool);
        ///  LEA_SC free event indication and clear flag. This bit indicated the free event for LEA_SC. This bit is cleared by writing a one to it. Writing a zero has no effect.
        LEASCFREEC: 17 = struct LEASCFREEC(bool);
        ///  LEA_SC timeout fault indication and clear flag; This bits indicates that a timer timeout occurred. This fault is cleared by writing a one to it. Writing a zero has no effect..
        LEASCTIMFLTC: 21 = struct LEASCTIMFLTC(bool);
        /// LEA-SC command fault
        LEASCCFLTC: 22 = enum LEASCCFLTC {
            /// No command fault occurred since this bit was cleared
            LEASCCFLTC_0 = 0b0,
            /// At least one command fault occurred since this bit was cleared
            LEASCCFLTC_1 = 0b1,
        }
        ///  LEA_SC memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEASCWRSTAT and LEASCRDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEASCCNF0.LEASCMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect.
        LEASCMEMFLTC: 23 = enum LEASCMEMFLTC {
            /// No memory fault occurred since this bit was cleared
            LEASCMEMFLTC_0 = 0b0,
            /// At least one memory fault since this bit was cleared
            LEASCMEMFLTC_1 = 0b1,
        }
        ///  Read Status. This bit field keeps the VBUS read status lines from the last bus error condition.
        LEASCRDSTAT: 24..27 = struct LEASCRDSTAT(u32);
        ///  Write Status. This bit field keeps the VBUS write status lines from the last bus error condition.
        LEASCWRSTAT: 28..31 = struct LEASCWRSTAT(u32);
    }
    /// Configuration Register 2
    rw LEASCCNF2 @ 0x0c: u32 = 0_0 {
        ///  LEA_SC stack pointer value (byte units). 64 lower kB is physical limit for LEA application complexity
        LEASCSPTR: 0..15 = struct LEASCSPTR(u32);
    }
    /// Memory Bottom Register
    r LEASCMB @ 0x10: u32 = 0_0 {
        ///  LEA_SC memory bottom address boundary in byte address units
        LEASCMB: 0..15 = struct LEASCMBField(u32);
    }
    /// Memory Top Register
    r LEASCMT @ 0x14: u32 = 0_0 {
        ///  LEA_SC memory top address boundary in byte address units
        LEASCMT: 0..15 = struct LEASCMTField(u32);
    }
    /// Code Memory Access Register
    rw LEASCCMA @ 0x18: u32 = 0_0 {
        /// Code Memory Access Register
        LEASCCMA: 0..31 = struct LEASCCMAField(u32);
    }
    /// Code Memory Control Register
    rw LEASCCMCTL @ 0x1c: u32 = 0_0 {
        ///  This bit controls access to LEA_SC code memory.
        LEASCCMAE: 0 = enum LEASCCMAE {
            /// Code memory access disabled. Accesses to LEA_SC code memory are not possible. LEA_SC does accept commands for execution. Reads to LEA_SC code memory will return zeroes and writes are ignored.
            LEASCCMAE_0 = 0b0,
            /// Code memory access enabled. Accesses to LEA_SC code memory are possible. LEA_SC does not accept commands during this mode (command is ignored). Coprocessor interface accesses by the CPU cause a Coprocessor not available indication.
            LEASCCMAE_1 = 0b1,
        }
        ///  This bit when set causes the code memory address port field LEASCCMAP to increment each time after an access to LEASCCMDP is performed.The decrement is by value two on 16B accesses on lower word of LEASCCMA.The decrement is by value two on 32B accesses on LEASCCMA.#b#Note:#/b# A double increment is performed when read modify write accesses are done on LEASCCMDP.
        LEASCINC: 2 = struct LEASCINC(bool);
        ///  This bit when set causes the code memory address port field LEASCCMAP to decrement each time after an access to LEASCCMDP is performed.The decrement is by value two on 16B accesses on lower word of LEASCCMA.The decrement is by value two on 32B accesses on LEASCCMA.#b#Note:#/b# A double decrement is performed when read modify write accesses are done on LEASCCMDP.
        LEASCDEC: 3 = struct LEASCDEC(bool);
        ///  Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0
        LEASCCROFF: 4..5 = enum LEASCCROFF {
            /// Contents of LEA-SC code RAM are retained in LPM3/LPM4.
            LEASCCROFF_0 = 0b00,
            /// Turns off the LEA-SC code RAM in LPM3/LPM4, re-activates it on wake-up. All data of the code RAM is lost after wakeup from LPM3/LPM4. See the device specific data sheet for presence and size of Code RAM.
            LEASCCROFF_1 = 0b01,
            /// Turns off the code RAM entering LPM3/LPM4, the code RAM sector remains off after wake-up. All data of the code RAM is lost. See the device-specific data sheet for presence and size of Code RAM.
            LEASCCROFF_2 = 0b10,
            /// Reserved (Future: Turns off the code RAM immediately. All data of the Code RAM is lost. See the device-specific data sheet for presence and size of Code RAM.)
            LEASCCROFF_3 = 0b11,
        }
        /// Code RAM action
        LEASCCRACTION: 6 = struct LEASCCRACTION(bool);
        ///  LEA_SC code memory address port. This bit field points to the memory location that is accessible via LEASCCMDP
        LEASCCMAP: 16..31 = struct LEASCCMAP(u32);
    }
    /// LEA Command Status Register
    r LEASCCMDSTAT @ 0x28: u32 = 0_0 {
        ///  LEA_SC instruction handshake synchronization type flags
        LEASCCMDSTAT_LEASCITFLG: 0..1 = enum LEASCCMDSTAT_LEASCITFLG {
            /// LEA_SC command without any further indication
            LEASCITFLG_0 = 0b00,
            /// LEA_SC command with explicit result update
            LEASCITFLG_1 = 0b01,
            /// LEA_SC command with interrupt upon completion
            LEASCITFLG_2 = 0b10,
            /// LEA_SC command with interrupt and explicit result update
            LEASCITFLG_3 = 0b11,
        }
        ///  These bits represent the LEA_SC command to be invoked. See also the command table
        LEASCCMDSTAT_LEASCCMD: 2..9 = enum LEASCCMDSTAT_LEASCCMD {
            /// Suspends ongoing action an enters Ready
            SUSPEND = 0b00000000,
            /// Resumes an previously suspended command execution
            RESUME = 0b00000010,
            /// Complex FFT on 16 bit fractional numbers fix scaling
            FFTCOMPLEXFIXEDSCALING = 0b00000100,
            /// Real FIR on 16 bit fractional numbers
            FIR = 0b00000110,
            /// Real vector polynomial calculations 16 args all fractional
            POLYNOMIAL = 0b00001000,
            /// Real FFT-extension on 16 bit fractional numbers
            FFT = 0b00001010,
            /// Real vector polynomial calculations 32 bit args all fractional
            POLYNOMIALLONG = 0b00001100,
            /// Real row oriented matrix multiply
            MPYMATRIXROW = 0b00001101,
            /// Real matrix multiply 16 with 16 to 16 bit fractional
            MPYMATRIX = 0b00001111,
            /// Real point wise matrix add of 16 and 16 to 16 bit number vector
            ADDMATRIX = 0b00010000,
            /// Real maximum value and position of 16 bit matrices
            MAXMATRIX = 0b00010001,
            /// Real minimum value and position of 16 bit matrices
            MINMATRIX = 0b00010010,
            /// Real second order biquad using DF1 with 16 bit fractional
            IIRBQ1 = 0b00010011,
            /// Real matrix MAC short with 16Bt to 16B fract
            MAC = 0b00010101,
            /// Split 16B vector even to even words
            DEINTERLEAVEEVENEVEN = 0b00010110,
            /// Split 16Bt vector even to odd words
            DEINTERLEAVEEVENODD = 0b00010111,
            /// Split 16B vector odd to even words
            DEINTERLEAVEODDEVEN = 0b00011000,
            /// Split 16B vector odd to odd words
            DEINTERLEAVEODDODD = 0b00011001,
            /// Complex Dot Product
            MACCOMPLEXMATRIX = 0b00011010,
            /// Complex conjugate Dot Product
            MACCOMPLEXCONJUGATEMATRIX = 0b00011011,
            /// Real point wise matrix Subtraction of 16 and 16 to 16 bit
            SUBMATRIX = 0b00011100,
            /// Real point wise matrix multiply 32 with 32 to 32 bit fractional
            MPYLONGMATRIX = 0b00011101,
            /// Complex point wise matrix multiply complex with complex
            MPYCOMPLEXMATRIX = 0b00011110,
            /// Real point wise matrix add of 32 and 32 to 32 bit number
            ADDLONGMATRIX = 0b00011111,
            /// List move 32 to 32 bit
            MOVELONGLIST = 0b00100000,
            /// Complex bit reversal for 16 bit fractional numbers even
            BITREVERSECOMPLEXEVEN = 0b00100001,
            /// Complex bit reversal for 16 bit fractional Numbers odd
            BITREVERSECOMPLEXODD = 0b00100010,
            /// Real second order biquad using DF2 with 16 bit fractional, extended to include bias and intermediate state min/max
            IIRBQ2EXTENDED = 0b00100100,
            /// Complex FFT on 32B bit fractional numbers, fix scaling
            FFTCOMPLEXLONG = 0b00100111,
            /// Real FFT-extension on 32 bit fractional numbers
            FFTLONG = 0b00101001,
            /// Complex bit reversal for 32 bit fractional numbers even
            BITREVERSECOMPLEXLONGEVEN = 0b00101011,
            /// Complex bit reversal for 16 bit fractional numbers odd
            BITREVERSECOMPLEXLONGODD = 0b00101101,
            /// Scalar Polynomial for math on 32bit fractional
            POLYNOMIALSCALAR = 0b00101111,
            /// Complex FFT on 16B bit fractional numbers with auto scaling for enhanced accuracy
            FFTCOMPLEXAUTOSCALING = 0b00110000,
            /// Real FIR on 32 bit fractional numbers
            FIRLONG = 0b00110010,
            /// Real block MAC on 32B fractional numbers
            MACLONGMATRIX = 0b00110100,
            /// Real point wise matrix Subtraction of 32 and 32 to 32 bit
            SUBLONGMATRIX = 0b00110101,
            /// Real maximum value and position of signed 32B matrices
            MAXLONGMATRIX = 0b00110110,
            /// Real minimum value and position of signed 32B matrices
            MINLONGMATRIX = 0b00110111,
            /// Complex FIR on 16B fractional numbers
            FIRCOMPLEX = 0b00111000,
            /// Real maximum value and position of unsigned 16B matrices
            MAXUNSIGNEDMATRIX = 0b00111010,
            /// Real minimum value and position of unsigned 32B matrices
            MINUNSIGNEDMATRIX = 0b00111011,
            /// Real Matrix MAC on 16B fractional
            MACMATRIX = 0b01000000,
            /// Vector maximum on 16B signed numbers
            MAX = 0b01000001,
            /// Vector minimum on 16B signed numbers
            MIN = 0b01000010,
            /// Vector maximum on 16B unsigned numbers
            MAXUNSIGNED = 0b01000011,
            /// Vector minimum on 16B unsigned numbers
            MINUNSIGNED = 0b01000100,
            /// Matrix maximum on 32B unsigned numbers
            MAXUNSIGNEDLONGMATRIX = 0b01000101,
            /// Matrix minimum on 32B unsigned numbers
            MINUNSIGNEDLONGMATRIX = 0b01000110,
            /// Real second order biquad using DF2 with 16 bit fractional
            IIRBQ2 = 0b01000111,
            /// Complex FIR on 32B fractional numbers
            FIRCOMPLEXLONG = 0b01001001,
            /// Split Function on 32B Vectors/Matrices
            DEINTERLEAVELONG = 0b01001011,
            /// In-place symmetrical window on 16B fractional numbers
            WINDOW = 0b01001100,
            /// Vector MAC at three points, real 16-bit with 32-bit result
            MAC3 = 0b01001101,
            /// Scaled vector multiply and accumulate (MAC)
            SCALEDMAC = 0b01001110,
            /// Scaled FIR, 16-bit real fractional numbers
            SCALEDFIR = 0b01001111,
        }
    }
    /// LEA Source 1 Status Register
    rw LEASCS1STAT @ 0x2c: u32 = 0_0 {
        /// LEA Source 1 Status Register
        LEASCS1STAT: 0..31 = struct LEASCS1STATField(u32);
    }
    /// LEA Source 0 Status Register
    rw LEASCS0STAT @ 0x30: u32 = 0_0 {
        /// LEA Source 0 Status Register
        LEASCS0STAT: 0..31 = struct LEASCS0STATField(u32);
    }
    /// LEA Result Status Register
    rw LEASCDSTSTAT @ 0x34: u32 = 0_0 {
        /// LEA Result Status Register
        LEASCDSTSTAT: 0..31 = struct LEASCDSTSTATField(u32);
    }
    /// PM Control Register
    rw LEASCPMCTL @ 0x40: u32 = 0_0 {
        /// Command enable
        LEASCCMDEN: 0 = enum LEASCCMDEN {
            /// Command triggering by writing to LEASCPMCB is disabled
            LEASCCMDEN_0 = 0b0,
            /// Command triggering by writing to LEASCPMCB is enabled
            LEASCCMDEN_1 = 0b1,
        }
        /// Command trigger
        LEASCTRG: 7 = struct LEASCTRG(bool);
    }
    /// PM Result Register
    rw LEASCPMDST @ 0x44: u32 = 0_0 {
        /// PM Result Register
        LEASCPMDST: 0..31 = struct LEASCPMDSTField(u32);
    }
    /// PM Source 1 Register
    rw LEASCPMS1 @ 0x48: u32 = 0_0 {
        /// PM Source 1 Register
        LEASCPMS1: 0..31 = struct LEASCPMS1Field(u32);
    }
    /// PM Source 0 Register
    rw LEASCPMS0 @ 0x4c: u32 = 0_0 {
        /// PM Source 0 Register
        LEASCPMS0: 0..31 = struct LEASCPMS0Field(u32);
    }
    /// PM Command Buffer Register
    rw LEASCPMCB @ 0x50: u32 = 0_0 {
        ///  LEA_SC instruction handshake synchronization type flag
        LEASCPMCB_LEASCITFLG: 0..1 = enum LEASCPMCB_LEASCITFLG {
            /// LEA-SC command without any further indication
            LEASCITFLG_0 = 0b00,
            /// LEA-SC command with explicit result update
            LEASCITFLG_1 = 0b01,
            /// LEA-SC command with interrupt upon completion
            LEASCITFLG_2 = 0b10,
            /// LEA-SC command with interrupt and explicit result update
            LEASCITFLG_3 = 0b11,
        }
        ///  These bits represent the LEA_SC command to be invoked. See also the command table
        LEASCPMCB_LEASCCMD: 2..9 = enum LEASCPMCB_LEASCCMD {
            /// Suspends ongoing action an enters Ready
            SUSPEND = 0b00000000,
            /// Resumes an previously suspended command execution
            RESUME = 0b00000010,
            /// Complex FFT on 16 bit fractional numbers fix scaling
            FFTCOMPLEXFIXEDSCALING = 0b00000100,
            /// Real FIR on 16 bit fractional numbers
            FIR = 0b00000110,
            /// Real vector polynomial calculations 16 args all fractional
            POLYNOMIAL = 0b00001000,
            /// Real FFT-extension on 16 bit fractional numbers
            FFT = 0b00001010,
            /// Real vector polynomial calculations 32 bit args all fractional
            POLYNOMIALLONG = 0b00001100,
            /// Real row oriented matrix multiply
            MPYMATRIXROW = 0b00001101,
            /// Real matrix multiply 16 with 16 to 16 bit fractional
            MPYMATRIX = 0b00001111,
            /// Real point wise matrix add of 16 and 16 to 16 bit number vector
            ADDMATRIX = 0b00010000,
            /// Real maximum value and position of 16 bit matrices
            MAXMATRIX = 0b00010001,
            /// Real minimum value and position of 16 bit matrices
            MINMATRIX = 0b00010010,
            /// Real second order biquad using DF1 with 16 bit fractional
            IIRBQ1 = 0b00010011,
            /// Real matrix MAC short with 16Bt to 16B fract
            MAC = 0b00010101,
            /// Split 16B vector even to even words
            DEINTERLEAVEEVENEVEN = 0b00010110,
            /// Split 16Bt vector even to odd words
            DEINTERLEAVEEVENODD = 0b00010111,
            /// Split 16B vector odd to even words
            DEINTERLEAVEODDEVEN = 0b00011000,
            /// Split 16B vector odd to odd words
            DEINTERLEAVEODDODD = 0b00011001,
            /// Complex Dot Product
            MACCOMPLEXMATRIX = 0b00011010,
            /// Complex conjugate Dot Product
            MACCOMPLEXCONJUGATEMATRIX = 0b00011011,
            /// Real point wise matrix Subtraction of 16 and 16 to 16 bit
            SUBMATRIX = 0b00011100,
            /// Real point wise matrix multiply 32 with 32 to 32 bit fractional
            MPYLONGMATRIX = 0b00011101,
            /// Complex point wise matrix multiply complex with complex
            MPYCOMPLEXMATRIX = 0b00011110,
            /// Real point wise matrix add of 32 and 32 to 32 bit number
            ADDLONGMATRIX = 0b00011111,
            /// List move 32 to 32 bit
            MOVELONGLIST = 0b00100000,
            /// Complex bit reversal for 16 bit fractional numbers even
            BITREVERSECOMPLEXEVEN = 0b00100001,
            /// Complex bit reversal for 16 bit fractional Numbers odd
            BITREVERSECOMPLEXODD = 0b00100010,
            /// Real second order biquad using DF2 with 16 bit fractional, extended to include bias and intermediate state min/max
            IIRBQ2EXTENDED = 0b00100100,
            /// Complex FFT on 32B bit fractional numbers, fix scaling
            FFTCOMPLEXLONG = 0b00100111,
            /// Real FFT-extension on 32 bit fractional numbers
            FFTLONG = 0b00101001,
            /// Complex bit reversal for 32 bit fractional numbers even
            BITREVERSECOMPLEXLONGEVEN = 0b00101011,
            /// Complex bit reversal for 16 bit fractional numbers odd
            BITREVERSECOMPLEXLONGODD = 0b00101101,
            /// Scalar Polynomial for math on 32bit fractional
            POLYNOMIALSCALAR = 0b00101111,
            /// Complex FFT on 16B bit fractional numbers with auto scaling for enhanced accuracy
            FFTCOMPLEXAUTOSCALING = 0b00110000,
            /// Real FIR on 32 bit fractional numbers
            FIRLONG = 0b00110010,
            /// Real block MAC on 32B fractional numbers
            MACLONGMATRIX = 0b00110100,
            /// Real point wise matrix Subtraction of 32 and 32 to 32 bit
            SUBLONGMATRIX = 0b00110101,
            /// Real maximum value and position of signed 32B matrices
            MAXLONGMATRIX = 0b00110110,
            /// Real minimum value and position of signed 32B matrices
            MINLONGMATRIX = 0b00110111,
            /// Complex FIR on 16B fractional numbers
            FIRCOMPLEX = 0b00111000,
            /// Real maximum value and position of unsigned 16B matrices
            MAXUNSIGNEDMATRIX = 0b00111010,
            /// Real minimum value and position of unsigned 32B matrices
            MINUNSIGNEDMATRIX = 0b00111011,
            /// Real Matrix MAC on 16B fractional
            MACMATRIX = 0b01000000,
            /// Vector maximum on 16B signed numbers
            MAX = 0b01000001,
            /// Vector minimum on 16B signed numbers
            MIN = 0b01000010,
            /// Vector maximum on 16B unsigned numbers
            MAXUNSIGNED = 0b01000011,
            /// Vector minimum on 16B unsigned numbers
            MINUNSIGNED = 0b01000100,
            /// Matrix maximum on 32B unsigned numbers
            MAXUNSIGNEDLONGMATRIX = 0b01000101,
            /// Matrix minimum on 32B unsigned numbers
            MINUNSIGNEDLONGMATRIX = 0b01000110,
            /// Real second order biquad using DF2 with 16 bit fractional
            IIRBQ2 = 0b01000111,
            /// Complex FIR on 32B fractional numbers
            FIRCOMPLEXLONG = 0b01001001,
            /// Split Function on 32B Vectors/Matrices
            DEINTERLEAVELONG = 0b01001011,
            /// In-place symmetrical window on 16B fractional numbers
            WINDOW = 0b01001100,
            /// Vector MAC at three points, real 16-bit with 32-bit result
            MAC3 = 0b01001101,
            /// Scaled vector multiply and accumulate (MAC)
            SCALEDMAC = 0b01001110,
            /// Scaled FIR, 16-bit real fractional numbers
            SCALEDFIR = 0b01001111,
        }
        ///  Command context: This bit field may be set by the user together with invoking a command. This bit field is saved on SUSPEND; and restored from LEA_SC Stack on RESUME. This bit field is used by SW to associate or synchronize LEA_SC commands to a certain tasks and IDs.
        LEASCCTX: 20..31 = struct LEASCCTX(u32);
    }
    /// Interrupt Flag and Set Register
    rw LEASCIFGSET @ 0x70: u32 = 0_0 {
        /// LEA_SC command overflow interrupt flag
        LEASCCOVLIS: 0 = enum LEASCCOVLIS {
            /// No interrupt pending
            LEASCCOVLIS_0 = 0b0,
            /// Interrupt pending
            LEASCCOVLIS_1 = 0b1,
        }
        /// LEA_SC timer interrupt flag
        LEASCTIS: 1 = enum LEASCTIS {
            /// No interrupt pending
            LEASCTIS_0 = 0b0,
            /// Interrupt pending
            LEASCTIS_1 = 0b1,
        }
        /// LEA_SC out of address range interrupt flag.
        LEASCOORIS: 2 = enum LEASCOORIS {
            /// No interrupt pending
            LEASCOORIS_0 = 0b0,
            /// Interrupt pending
            LEASCOORIS_1 = 0b1,
        }
        /// LEA_SC scalar data inconsistency interrupt flag
        LEASCSDIIS: 3 = enum LEASCSDIIS {
            /// No interrupt pending
            LEASCSDIIS_0 = 0b0,
            /// Interrupt pending
            LEASCSDIIS_1 = 0b1,
        }
        ///  PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag.
        LEASCPMCMDIS: 4 = enum LEASCPMCMDIS {
            /// No interrupt pending
            LEASCPMCMDIS_0 = 0b0,
            /// Interrupt pending
            LEASCPMCMDIS_1 = 0b1,
        }
    }
    /// Interrupt Enable Register
    rw LEASCIE @ 0x74: u32 = 0_0 {
        /// LEA_SC command overflow interrupt enable
        LEASCCOVLIE: 0 = enum LEASCCOVLIE {
            /// Interrupt disabled
            LEASCCOVLIE_0 = 0b0,
            /// Interrupt enabled
            LEASCCOVLIE_1 = 0b1,
        }
        /// LEA_SC timer event interrupt enable
        LEASCTIE: 1 = enum LEASCTIE {
            /// Interrupt disabled
            LEASCTIE_0 = 0b0,
            /// Interrupt enabled
            LEASCTIE_1 = 0b1,
        }
        /// LEA_SC out of address range interrupt enable.
        LEASCOORIE: 2 = enum LEASCOORIE {
            /// Interrupt disabled
            LEASCOORIE_0 = 0b0,
            /// Interrupt enabled
            LEASCOORIE_1 = 0b1,
        }
        /// LEA_SC scalar data inconsistency interrupt enable
        LEASCSDIIE: 3 = enum LEASCSDIIE {
            /// Interrupt disabled
            LEASCSDIIE_0 = 0b0,
            /// Interrupt enabled
            LEASCSDIIE_1 = 0b1,
        }
        ///  PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable.
        LEASCPMCMDIE: 4 = enum LEASCPMCMDIE {
            /// Interrupt disabled
            LEASCPMCMDIE_0 = 0b0,
            /// Interrupt enabled
            LEASCPMCMDIE_1 = 0b1,
        }
    }
    /// Interrupt Flag and Clear Register
    rw LEASCIFG @ 0x78: u32 = 0_0 {
        /// LEA_SC command overflow interrupt flag
        LEASCCOVLIFG: 0 = enum LEASCCOVLIFG {
            /// No interrupt pending
            LEASCCOVLIFG_0 = 0b0,
            /// Interrupt pending
            LEASCCOVLIFG_1 = 0b1,
        }
        /// LEA_SC timer interrupt flag
        LEASCTIFG: 1 = enum LEASCTIFG {
            /// No interrupt pending
            LEASCTIFG_0 = 0b0,
            /// Interrupt pending
            LEASCTIFG_1 = 0b1,
        }
        /// LEA_SC out of address range interrupt flag.
        LEASCOORIFG: 2 = enum LEASCOORIFG {
            /// No interrupt pending
            LEASCOORIFG_0 = 0b0,
            /// Interrupt pending
            LEASCOORIFG_1 = 0b1,
        }
        /// LEA_SC scalar data inconsistency interrupt flag
        LEASCSDIIFG: 3 = enum LEASCSDIIFG {
            /// No interrupt pending
            LEASCSDIIFG_0 = 0b0,
            /// Interrupt pending
            LEASCSDIIFG_1 = 0b1,
        }
        /// PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag.
        LEASCPMCMDIFG: 4 = enum LEASCPMCMDIFG {
            /// No interrupt pending
            LEASCPMCMDIFG_0 = 0b0,
            /// Interrupt pending
            LEASCPMCMDIFG_1 = 0b1,
        }
    }
    /// Interrupt Vector Register
    rw LEASCIV @ 0x7c: u32 = 0_0 {
        ///  LEA-SC interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA-SC interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary
        LEASCIV: 0..7 = enum LEASCIVField {
            /// No interrupt pending
            NONE = 0b00000000,
            /// LEA command overflow
            COVLIFG = 0b00000010,
            /// LEA timer interrupt
            TIFG = 0b00000100,
            /// LEA out of range interrupt
            OORIFG = 0b00000110,
            /// LEA scalar data inconsistency
            SDIIFG = 0b00001000,
            /// PMCMD complete interrupt
            PMCMDIFG = 0b00001010,
        }
    }
}
