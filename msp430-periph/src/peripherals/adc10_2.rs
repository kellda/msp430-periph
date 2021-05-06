//! ADC10

utils::periph! {
    /// ADC10
    ADC10;
    /// ADC10 Data Transfer Control 0
    rw ADC10DTC0 @ 0x00: u8 = 0_0 {
        /// This bit should normally be reset
        ADC10FETCH: 0 = struct ADC10FETCH(bool);
        /// ADC10 block one
        ADC10B1: 1 = struct ADC10B1(bool);
        /// ADC10 continuous transfer
        ADC10CT: 2 = struct ADC10CT(bool);
        /// ADC10 two-block mode
        ADC10TB: 3 = struct ADC10TB(bool);
    }
    /// ADC10 Data Transfer Control 1
    rw ADC10DTC1 @ 0x01: u8 = 0_0 {
        /// ADC10 Data Transfer Control 1
        ADC10DTC1: 0..7 = struct ADC10DTC1Field(u8);
    }
    /// ADC10 Analog Enable 0
    rw ADC10AE0 @ 0x02: u8 = 0_0 {
        /// ADC10 Analog Enable 0
        ADC10AE0: 0..7 = struct ADC10AE0Field(u8);
    }
    /// ADC10 Control 0
    rw ADC10CTL0 @ 0x168: u16 = 0_0 {
        /// ADC10 Start Conversion
        ADC10SC: 0 = struct ADC10SC(bool);
        /// ADC10 Enable Conversion
        ENC: 1 = struct ENC(bool);
        /// ADC10 Interrupt Flag
        ADC10IFG: 2 = struct ADC10IFG(bool);
        /// ADC10 Interrupt Enalbe
        ADC10IE: 3 = struct ADC10IE(bool);
        /// ADC10 On/Enable
        ADC10ON: 4 = struct ADC10ON(bool);
        /// ADC10 Reference on
        REFON: 5 = struct REFON(bool);
        /// ADC10 Ref 0:1.5V / 1:2.5V
        REF2_5V: 6 = struct REF2_5V(bool);
        /// ADC10 Multiple SampleConversion
        MSC: 7 = struct MSC(bool);
        /// ADC10 Reference Burst Mode
        REFBURST: 8 = struct REFBURST(bool);
        /// ADC10 Enalbe output of Ref.
        REFOUT: 9 = struct REFOUT(bool);
        /// ADC10 Sampling Rate 0:200ksps / 1:50ksps
        ADC10SR: 10 = struct ADC10SR(bool);
        /// ADC10 Sample Hold Select Bit: 0
        ADC10SHT: 11..12 = enum ADC10SHT {
            /// 4 x ADC10CLKs
            ADC10SHT_0 = 0b00,
            /// 8 x ADC10CLKs
            ADC10SHT_1 = 0b01,
            /// 16 x ADC10CLKs
            ADC10SHT_2 = 0b10,
            /// 64 x ADC10CLKs
            ADC10SHT_3 = 0b11,
        }
        /// ADC10 Reference Select Bit: 0
        SREF: 13..15 = enum SREF {
            /// VR+ = AVCC and VR- = AVSS
            SREF_0 = 0b000,
            /// VR+ = VREF+ and VR- = AVSS
            SREF_1 = 0b001,
            /// VR+ = VEREF+ and VR- = AVSS
            SREF_2 = 0b010,
            /// VR+ = VEREF+ and VR- = AVSS
            SREF_3 = 0b011,
            /// VR+ = AVCC and VR- = VREF-/VEREF-
            SREF_4 = 0b100,
            /// VR+ = VREF+ and VR- = VREF-/VEREF-
            SREF_5 = 0b101,
            /// VR+ = VEREF+ and VR- = VREF-/VEREF-
            SREF_6 = 0b110,
            /// VR+ = VEREF+ and VR- = VREF-/VEREF-
            SREF_7 = 0b111,
        }
    }
    /// ADC10 Control 1
    rw ADC10CTL1 @ 0x16a: u16 = 0_0 {
        /// ADC10 BUSY
        ADC10BUSY: 0 = struct ADC10BUSY(bool);
        /// ADC10 Conversion Sequence Select 0
        CONSEQ: 1..2 = enum CONSEQ {
            /// Single channel single conversion
            CONSEQ_0 = 0b00,
            /// Sequence of channels
            CONSEQ_1 = 0b01,
            /// Repeat single channel
            CONSEQ_2 = 0b10,
            /// Repeat sequence of channels
            CONSEQ_3 = 0b11,
        }
        /// ADC10 Clock Source Select Bit: 0
        ADC10SSEL: 3..4 = enum ADC10SSEL {
            /// ADC10OSC
            ADC10SSEL_0 = 0b00,
            /// ACLK
            ADC10SSEL_1 = 0b01,
            /// MCLK
            ADC10SSEL_2 = 0b10,
            /// SMCLK
            ADC10SSEL_3 = 0b11,
        }
        /// ADC10 Clock Divider Select Bit: 0
        ADC10DIV: 5..7 = enum ADC10DIV {
            /// ADC10 Clock Divider Select 0
            ADC10DIV_0 = 0b000,
            /// ADC10 Clock Divider Select 1
            ADC10DIV_1 = 0b001,
            /// ADC10 Clock Divider Select 2
            ADC10DIV_2 = 0b010,
            /// ADC10 Clock Divider Select 3
            ADC10DIV_3 = 0b011,
            /// ADC10 Clock Divider Select 4
            ADC10DIV_4 = 0b100,
            /// ADC10 Clock Divider Select 5
            ADC10DIV_5 = 0b101,
            /// ADC10 Clock Divider Select 6
            ADC10DIV_6 = 0b110,
            /// ADC10 Clock Divider Select 7
            ADC10DIV_7 = 0b111,
        }
        /// ADC10 Invert Sample Hold Signal
        ISSH: 8 = struct ISSH(bool);
        /// ADC10 Data Format 0:binary 1:2's complement
        ADC10DF: 9 = struct ADC10DF(bool);
        /// ADC10 Sample/Hold Source Bit: 0
        SHS: 10..11 = enum SHS {
            /// ADC10SC
            SHS_0 = 0b00,
            /// TA3 OUT1
            SHS_1 = 0b01,
            /// TA3 OUT0
            SHS_2 = 0b10,
            /// TA3 OUT2
            SHS_3 = 0b11,
        }
        /// ADC10 Input Channel Select Bit: 0
        INCH: 12..15 = enum INCH {
            /// Selects Channel 0
            INCH_0 = 0b0000,
            /// Selects Channel 1
            INCH_1 = 0b0001,
            /// Selects Channel 2
            INCH_2 = 0b0010,
            /// Selects Channel 3
            INCH_3 = 0b0011,
            /// Selects Channel 4
            INCH_4 = 0b0100,
            /// Selects Channel 5
            INCH_5 = 0b0101,
            /// Selects Channel 6
            INCH_6 = 0b0110,
            /// Selects Channel 7
            INCH_7 = 0b0111,
            /// Selects Channel 8
            INCH_8 = 0b1000,
            /// Selects Channel 9
            INCH_9 = 0b1001,
            /// Selects Channel 10
            INCH_10 = 0b1010,
            /// Selects Channel 11
            INCH_11 = 0b1011,
            /// Selects Channel 12
            INCH_12 = 0b1100,
            /// Selects Channel 13
            INCH_13 = 0b1101,
            /// Selects Channel 14
            INCH_14 = 0b1110,
            /// Selects Channel 15
            INCH_15 = 0b1111,
        }
    }
    /// ADC10 Memory
    rw ADC10MEM @ 0x16c: u16 = 0_0 {
        /// ADC10 Memory
        ADC10MEM: 0..15 = struct ADC10MEMField(u16);
    }
    /// ADC10 Data Transfer Start Address
    rw ADC10SA @ 0x174: u16 = 0_0 {
        /// ADC10 Data Transfer Start Address
        ADC10SA: 0..15 = struct ADC10SAField(u16);
    }
}
