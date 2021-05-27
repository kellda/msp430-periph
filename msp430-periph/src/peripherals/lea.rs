//! LEA

utils::periph! {
    /// LEA
    LEA;
    /// LEA Capability Register
    r CAP @ 0x00: u32 = 0_0 {
        /// LEA Code Memory Size. This register identifies the size of available code RAM.
        MSIZ: 0..3 = enum MSIZ {
            /// no code RAM
            MSIZ_0 = 0b0000,
            /// 1KB Code RAM
            MSIZ_1 = 0b0001,
        }
    }
    /// Configuration Register 0
    rw CNF0 @ 0x04: u32 = 0_0 {
        /// LEA module software restart. Setting this bit to one restarts the LEA module. As long this bit remains set to one the LEA is held in Restart. (The LEA accessible memory behaves as system RAM)
        SWRST: 0 = struct SWRST(bool);
        /// Hold on faults and NMIs for all pending LEA operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)
        FTHOLD: 1 = enum FTHOLD {
            /// LEA transfers continue on faults/NMIs
            FTHOLD_0 = 0b0,
            /// LEA transfers enter HOLD on faults/NMIs
            FTHOLD_1 = 0b1,
        }
        /// This bit defined if command execution shall be continued in LPM modes
        LPR: 8 = enum LPR {
            /// LEA command execution stops in deep low power modes
            LPR_0 = 0b0,
            /// LEA command execution continues in deep low power modes
            LPR_1 = 0b1,
        }
        /// This bit defines if a "Command done interrupt" shall be triggered in LPM mode
        ILPM: 10 = enum ILPM {
            /// Interrupt of LEA is suppressed in LPM mode until AM is entered then the LEA interrupt is triggered as well
            ILPM_0 = 0b0,
            /// Interrupt of LEA is always triggered on completion of an LEA command
            ILPM_1 = 0b1,
        }
        /// LEA instruction loop buffer disable. Debugging function for LEA (leave it zero).
        ILB: 11 = struct ILB(bool);
        /// LEA module timer fault enable.
        TIMFLTE: 13 = enum TIMFLTE {
            /// LEA module timer timeout will not cause a fault indication
            TIMFLT_0 = 0b0,
            /// LEA module timer timeout will cause a fault indication. LEA stops operation and enters "Ready-state".
            TIMFLTE_1 = 0b1,
        }
        /// LEAHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults.
        CFLT: 14 = enum CFLT {
            /// LEAHPCFLT is disabled
            CFLT_0 = 0b0,
            /// LEAHPCFLT is enabled
            CFLT_1 = 0b1,
        }
        /// Enable bit on memory faults.
        MEMFLTE: 15 = enum MEMFLTE {
            /// LEA memory faults are disabled
            MEMFLTE_0 = 0b0,
            /// LEA memory faults are enabled
            MEMFLTE_1 = 0b1,
        }
        /// LEA done event indication and set flag. This bit indicated the done event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect.
        DONES: 16 = struct DONES(bool);
        /// LEA free event indication and set flag. This bit indicated the free event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect.
        FREES: 17 = struct FREES(bool);
        /// LEA timeout fault indication and set flag; This bits indicates that timer timeout occurred. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module (A package option)
        TIMFLTS: 21 = struct TIMFLTS(bool);
        /// LEAHPCFLTS when hardware trigger enabled later LEA command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a "User-NMI" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module.
        CFLTS: 22 = enum CFLTS {
            /// No command fault occurred since this bit was cleared
            CFLTS_0 = 0b0,
            /// At least one command fault occurred since this bit was cleared
            CFLTS_1 = 0b1,
        }
        /// LEA memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEACNF1. LEAWRSTAT and LEACNF1.LEARDSTAT.
        MEMFLTS: 23 = enum MEMFLTS {
            /// No memory fault occurred since this bit was cleared
            MEMFLTS_0 = 0b0,
            /// At least one memory fault since this bit was cleared
            MEMFLTS_1 = 0b1,
        }
        /// LEA module timer reset. Setting this bit to one clears LEA module timer. This bit is self clearing and will always be read as zero.
        TRST: 24 = struct TRST(bool);
        /// LEA timer enable; writing a one to this bit enables LEA timer operations.
        TEN: 25 = struct TEN(bool);
        /// LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK
        TISEL: 28..31 = enum TISEL {
            /// Timeout period: 128 x t#sub#CLK#/sub# (16 us at 8 MHz); Interval period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz)
            TISEL_0 = 0b0000,
            /// Timeout period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz); Interval period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz)
            TISEL_1 = 0b0001,
            /// Timeout period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz); Interval period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz)
            TISEL_2 = 0b0010,
            /// Timeout period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz); Interval period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz)
            TISEL_3 = 0b0011,
            /// Timeout period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz); Interval period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz)
            TISEL_4 = 0b0100,
            /// Timeout period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz); Interval period: 8192 x t#sub#CLK#/sub# (1ms at 8 MHz)
            TISEL_5 = 0b0101,
            /// Timeout period: 8192 x t#sub#CLK#/sub# (1 ms at 8 MHz); Interval period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz)
            TISEL_6 = 0b0110,
            /// Timeout period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz); Interval period: 32768 x t#sub#CLK#/sub# (4 ms at 8 MHz)
            TISEL_7 = 0b0111,
            /// Timeout period: 32768 x t#sub#CLK#/sub# (4ms at 8 MHz); Interval period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz)
            TISEL_8 = 0b1000,
            /// Timeout period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz); Interval period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz)
            TISEL_9 = 0b1001,
            /// Timeout period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz); Interval period: 262144 x t#sub#CLK#/sub# (32 ms at 8 MHz)
            TISEL_10 = 0b1010,
            /// Timeout period: 524288 x t#sub#CLK#/sub# (65 ms at 8 MHz); Interval period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz)
            TISEL_11 = 0b1011,
            /// Timeout period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz); Interval period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz)
            TISEL_12 = 0b1100,
            /// Timeout period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz); Interval period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz)
            TISEL_13 = 0b1101,
            /// Timeout period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz); Interval period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz)
            TISEL_14 = 0b1110,
            /// Timeout period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz); Interval period: 16777216 x t#sub#CLK#/sub# (2.1 s at 8 MHz)
            TISEL_15 = 0b1111,
        }
    }
    /// Configuration Register 1
    rw CNF1 @ 0x08: u32 = 0_0 {
        /// This bit indicate if LEA is able to accept new Commands (SUSPEND is always accepted)
        BUSY: 0 = enum BUSY {
            /// LEA is in Ready can accept new commands
            READY = 0b0,
            /// LEA is busy right now and cannot accept any commands
            BUSY = 0b1,
        }
        /// These Bits indicate the actual operation mode LEA is in.  Other = Reserved
        MODE: 4..7 = enum MODE {
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
        /// These bits indicate the current power consumption as a relative value. The value zero indicated only static operation (usually clock less). This register might be read out for statistical power estimation of an application. These bits are also reflected in J-STATE when debugging
        PWST: 8..11 = struct PWST(u32);
        /// These bits are used to store the internal state of the application specific processor (ASIP) inside the accelerator core. The specific meaning of those bit patterns is not shown in this document.
        ASST: 12..15 = struct ASST(u32);
        /// LEA done event indication and clear flag. This bit indicated the done event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect.
        DONEC: 16 = struct DONEC(bool);
        /// LEA free event indication and clear flag. This bit indicated the free event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect.
        FREEC: 17 = struct FREEC(bool);
        /// LEA timeout fault indication and clear flag; This bits indicates that a timer timeout occurred. This fault is cleared by writing a one to it. Writing a zero has no effect..
        TIMFLTC: 21 = struct TIMFLTC(bool);
        /// LEA command fault
        CFLTC: 22 = enum CFLTC {
            /// No command fault occurred since this bit was cleared
            CFLTC_0 = 0b0,
            /// At least one command fault occurred since this bit was cleared
            CFLTC_1 = 0b1,
        }
        /// LEA memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEAWRSTAT and LEARDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEACNF0.LEAMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect.
        MEMFLTC: 23 = enum MEMFLTC {
            /// No memory fault occurred since this bit was cleared
            MEMFLTC_0 = 0b0,
            /// At least one memory fault since this bit was cleared
            MEMFLTC_1 = 0b1,
        }
        /// Read Status. This bit field keeps the VBUS read status lines from the last bus error condition.
        RDSTAT: 24..27 = struct RDSTAT(u32);
        /// Write Status. This bit field keeps the VBUS write status lines from the last bus error condition.
        WRSTAT: 28..31 = struct WRSTAT(u32);
    }
    /// Configuration Register 2
    rw CNF2 @ 0x0c: u32 = 0_0 {
        /// LEA stack pointer value (byte units). 64 lower kB is physical limit for LEA application complexity
        SPTR: 0..15 = struct SPTR(u32);
    }
    /// Memory Bottom Register
    r MB @ 0x10: u32 = 0_0 {
        /// LEA memory bottom address boundary in byte address units
        MB: 0..15 = struct MBField(u32);
    }
    /// Memory Top Register
    r MT @ 0x14: u32 = 0_0 {
        /// LEA memory top address boundary in byte address units
        MT: 0..15 = struct MTField(u32);
    }
    /// Code Memory Access Register
    rw CMA @ 0x18: u32 = 0_0 {
        /// Code Memory Access Register
        CMA: 0..31 = struct CMAField(u32);
    }
    /// Code Memory Control Register
    rw CMCTL @ 0x1c: u32 = 0_0 {
        /// This bit controls access to LEA code memory.
        CMAE: 0 = enum CMAE {
            /// Code memory access disabled. Accesses to LEA code memory are not possible. LEA does accept commands for execution. Reads to LEA code memory will return zeroes and writes are ignored.
            CMAE_0 = 0b0,
            /// Code memory access enabled. Accesses to LEA code memory are possible. LEA does not accept commands during this mode (command is ignored). Coprocessor interface accesses by the CPU cause a Coprocessor not available indication.
            CMAE_1 = 0b1,
        }
        /// This bit when set causes the code memory address port field LEACMAP to increment each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double increment is performed when read modify write accesses are done on LEACMDP.
        INC: 2 = struct INC(bool);
        /// This bit when set causes the code memory address port field LEACMAP to decrement each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double decrement is performed when read modify write accesses are done on LEACMDP.
        DEC: 3 = struct DEC(bool);
        /// Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0
        CROFF: 4..5 = enum CROFF {
            /// Contents of LEA code RAM are retained in LPM3/LPM4.
            CROFF_0 = 0b00,
            /// Turns off the LEA code RAM in LPM3/LPM4, re-activates it on wake-up. All data of the code RAM is lost after wakeup from LPM3/LPM4. See the device specific data sheet for presence and size of Code RAM.
            CROFF_1 = 0b01,
            /// Turns off the code RAM entering LPM3/LPM4, the code RAM sector remains off after wake-up. All data of the code RAM is lost. See the device-specific data sheet for presence and size of Code RAM.
            CROFF_2 = 0b10,
            /// Reserved (Future: Turns off the code RAM immediately. All data of the Code RAM is lost. See the device-specific data sheet for presence and size of Code RAM.)
            CROFF_3 = 0b11,
        }
        /// Code RAM action
        CRACTION: 6 = struct CRACTION(bool);
        /// LEA code memory address port. This bit field points to the memory location that is accessible via LEACMDP
        CMAP: 16..31 = struct CMAP(u32);
    }
    /// LEA Command Status Register
    r CMDSTAT @ 0x28: u32 = 0_0 {
        /// LEA instruction handshake synchronization type flags
        CMDSTAT_ITFLG: 0..1 = enum CMDSTAT_ITFLG {
            /// LEA command without any further indication
            ITFLG_0 = 0b00,
            /// LEA command with explicit result update
            ITFLG_1 = 0b01,
            /// LEA command with interrupt upon completion
            ITFLG_2 = 0b10,
            /// LEA command with interrupt and explicit result update
            ITFLG_3 = 0b11,
        }
        /// These bits represent the LEA command to be invoked. See also the command table
        CMDSTAT_CMD: 2..9 = enum CMDSTAT_CMD {
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
    rw S1STAT @ 0x2c: u32 = 0_0 {
        /// LEA Source 1 Status Register
        S1STAT: 0..31 = struct S1STATField(u32);
    }
    /// LEA Source 0 Status Register
    rw S0STAT @ 0x30: u32 = 0_0 {
        /// LEA Source 0 Status Register
        S0STAT: 0..31 = struct S0STATField(u32);
    }
    /// LEA Result Status Register
    rw DSTSTAT @ 0x34: u32 = 0_0 {
        /// LEA Result Status Register
        DSTSTAT: 0..31 = struct DSTSTATField(u32);
    }
    /// PM Control Register
    rw PMCTL @ 0x40: u32 = 0_0 {
        /// Command enable
        CMDEN: 0 = enum CMDEN {
            /// Command triggering by writing to LEAPMCB is disabled
            CMDEN_0 = 0b0,
            /// Command triggering by writing to LEAPMCB is enabled
            CMDEN_1 = 0b1,
        }
        /// Command trigger
        TRG: 7 = struct TRG(bool);
    }
    /// PM Result Register
    rw PMDST @ 0x44: u32 = 0_0 {
        /// PM Result Register
        PMDST: 0..31 = struct PMDSTField(u32);
    }
    /// PM Source 1 Register
    rw PMS1 @ 0x48: u32 = 0_0 {
        /// PM Source 1 Register
        PMS1: 0..31 = struct PMS1Field(u32);
    }
    /// PM Source 0 Register
    rw PMS0 @ 0x4c: u32 = 0_0 {
        /// PM Source 0 Register
        PMS0: 0..31 = struct PMS0Field(u32);
    }
    /// PM Command Buffer Register
    rw PMCB @ 0x50: u32 = 0_0 {
        /// LEA instruction handshake synchronization type flag
        PMCB_ITFLG: 0..1 = enum PMCB_ITFLG {
            /// LEA command without any further indication
            ITFLG_0 = 0b00,
            /// LEA command with explicit result update
            ITFLG_1 = 0b01,
            /// LEA command with interrupt upon completion
            ITFLG_2 = 0b10,
            /// LEA command with interrupt and explicit result update
            ITFLG_3 = 0b11,
        }
        /// These bits represent the LEA command to be invoked. See also the command table
        PMCB_CMD: 2..9 = enum PMCB_CMD {
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
        /// Command context: This bit field may be set by the user together with invoking a command. This bit field is saved on SUSPEND; and restored from LEA Stack on RESUME. This bit field is used by SW to associate or synchronize LEA commands to a certain tasks and IDs.
        CTX: 20..31 = struct CTX(u32);
    }
    /// Interrupt Flag and Set Register
    rw IFGSET @ 0x70: u32 = 0_0 {
        /// LEA command overflow interrupt flag
        COVLIS: 0 = enum COVLIS {
            /// No interrupt pending
            COVLIS_0 = 0b0,
            /// Interrupt pending
            COVLIS_1 = 0b1,
        }
        /// LEA timer interrupt flag
        TIS: 1 = enum TIS {
            /// No interrupt pending
            LEATIS_0 = 0b0,
            /// Interrupt pending
            LEATIS_1 = 0b1,
        }
        /// LEA out of address range interrupt flag.
        OORIS: 2 = enum OORIS {
            /// No interrupt pending
            OORIS_0 = 0b0,
            /// Interrupt pending
            OORIS_1 = 0b1,
        }
        /// LEA scalar data inconsistency interrupt flag
        SDIIS: 3 = enum SDIIS {
            /// No interrupt pending
            SDIIS_0 = 0b0,
            /// Interrupt pending
            SDIIS_1 = 0b1,
        }
        /// PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag.
        PMCMDIS: 4 = enum PMCMDIS {
            /// No interrupt pending
            PMCMDIS_0 = 0b0,
            /// Interrupt pending
            PMCMDIS_1 = 0b1,
        }
    }
    /// Interrupt Enable Register
    rw IE @ 0x74: u32 = 0_0 {
        /// LEA command overflow interrupt enable
        COVLIE: 0 = enum COVLIE {
            /// Interrupt disabled
            COVLIE_0 = 0b0,
            /// Interrupt enabled
            COVLIE_1 = 0b1,
        }
        /// LEA timer event interrupt enable
        TIE: 1 = enum TIE {
            /// Interrupt disabled
            TIE_0 = 0b0,
            /// Interrupt enabled
            TIE_1 = 0b1,
        }
        /// LEA out of address range interrupt enable.
        OORIE: 2 = enum OORIE {
            /// Interrupt disabled
            OORIE_0 = 0b0,
            /// Interrupt enabled
            OORIE_1 = 0b1,
        }
        /// LEA scalar data inconsistency interrupt enable
        SDIIE: 3 = enum SDIIE {
            /// Interrupt disabled
            SDIIE_0 = 0b0,
            /// Interrupt enabled
            SDIIE_1 = 0b1,
        }
        /// PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable.
        PMCMDIE: 4 = enum PMCMDIE {
            /// Interrupt disabled
            PMCMDIE_0 = 0b0,
            /// Interrupt enabled
            PMCMDIE_1 = 0b1,
        }
    }
    /// Interrupt Flag and Clear Register
    rw IFG @ 0x78: u32 = 0_0 {
        /// LEA command overflow interrupt flag
        COVLIFG: 0 = enum COVLIFG {
            /// No interrupt pending
            COVLIFG_0 = 0b0,
            /// Interrupt pending
            COVLIFG_1 = 0b1,
        }
        /// LEA timer interrupt flag
        TIFG: 1 = enum TIFG {
            /// No interrupt pending
            TIFG_0 = 0b0,
            /// Interrupt pending
            TIFG_1 = 0b1,
        }
        /// LEA out of address range interrupt flag.
        OORIFG: 2 = enum OORIFG {
            /// No interrupt pending
            OORIFG_0 = 0b0,
            /// Interrupt pending
            OORIFG_1 = 0b1,
        }
        /// LEA scalar data inconsistency interrupt flag
        SDIIFG: 3 = enum SDIIFG {
            /// No interrupt pending
            SDIIFG_0 = 0b0,
            /// Interrupt pending
            SDIIFG_1 = 0b1,
        }
        /// PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag.
        PMCMDIFG: 4 = enum PMCMDIFG {
            /// No interrupt pending
            PMCMDIFG_0 = 0b0,
            /// Interrupt pending
            PMCMDIFG_1 = 0b1,
        }
    }
    /// Interrupt Vector Register
    rw IV @ 0x7c: u32 = 0_0 {
        /// LEA interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary
        IV: 0..7 = enum IVField {
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
