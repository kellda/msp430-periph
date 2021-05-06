//! Port 3/4

utils::periph! {
    /// Port 3/4
    Port34;
    /// Port 3 Input
    rw P3IN @ 0x00: u8 = 0_0 {
        /// P0
        P3IN_P0: 0 = struct P3IN_P0(bool);
        /// P1
        P3IN_P1: 1 = struct P3IN_P1(bool);
        /// P2
        P3IN_P2: 2 = struct P3IN_P2(bool);
        /// P3
        P3IN_P3: 3 = struct P3IN_P3(bool);
        /// P4
        P3IN_P4: 4 = struct P3IN_P4(bool);
        /// P5
        P3IN_P5: 5 = struct P3IN_P5(bool);
        /// P6
        P3IN_P6: 6 = struct P3IN_P6(bool);
        /// P7
        P3IN_P7: 7 = struct P3IN_P7(bool);
    }
    /// Port 3 Output
    rw P3OUT @ 0x01: u8 = 0_0 {
        /// P0
        P3OUT_P0: 0 = struct P3OUT_P0(bool);
        /// P1
        P3OUT_P1: 1 = struct P3OUT_P1(bool);
        /// P2
        P3OUT_P2: 2 = struct P3OUT_P2(bool);
        /// P3
        P3OUT_P3: 3 = struct P3OUT_P3(bool);
        /// P4
        P3OUT_P4: 4 = struct P3OUT_P4(bool);
        /// P5
        P3OUT_P5: 5 = struct P3OUT_P5(bool);
        /// P6
        P3OUT_P6: 6 = struct P3OUT_P6(bool);
        /// P7
        P3OUT_P7: 7 = struct P3OUT_P7(bool);
    }
    /// Port 3 Direction
    rw P3DIR @ 0x02: u8 = 0_0 {
        /// P0
        P3DIR_P0: 0 = struct P3DIR_P0(bool);
        /// P1
        P3DIR_P1: 1 = struct P3DIR_P1(bool);
        /// P2
        P3DIR_P2: 2 = struct P3DIR_P2(bool);
        /// P3
        P3DIR_P3: 3 = struct P3DIR_P3(bool);
        /// P4
        P3DIR_P4: 4 = struct P3DIR_P4(bool);
        /// P5
        P3DIR_P5: 5 = struct P3DIR_P5(bool);
        /// P6
        P3DIR_P6: 6 = struct P3DIR_P6(bool);
        /// P7
        P3DIR_P7: 7 = struct P3DIR_P7(bool);
    }
    /// Port 3 Selection
    rw P3SEL @ 0x03: u8 = 0_0 {
        /// P0
        P3SEL_P0: 0 = struct P3SEL_P0(bool);
        /// P1
        P3SEL_P1: 1 = struct P3SEL_P1(bool);
        /// P2
        P3SEL_P2: 2 = struct P3SEL_P2(bool);
        /// P3
        P3SEL_P3: 3 = struct P3SEL_P3(bool);
        /// P4
        P3SEL_P4: 4 = struct P3SEL_P4(bool);
        /// P5
        P3SEL_P5: 5 = struct P3SEL_P5(bool);
        /// P6
        P3SEL_P6: 6 = struct P3SEL_P6(bool);
        /// P7
        P3SEL_P7: 7 = struct P3SEL_P7(bool);
    }
    /// Port 4 Input
    rw P4IN @ 0x04: u8 = 0_0 {
        /// P0
        P4IN_P0: 0 = struct P4IN_P0(bool);
        /// P1
        P4IN_P1: 1 = struct P4IN_P1(bool);
        /// P2
        P4IN_P2: 2 = struct P4IN_P2(bool);
        /// P3
        P4IN_P3: 3 = struct P4IN_P3(bool);
        /// P4
        P4IN_P4: 4 = struct P4IN_P4(bool);
        /// P5
        P4IN_P5: 5 = struct P4IN_P5(bool);
        /// P6
        P4IN_P6: 6 = struct P4IN_P6(bool);
        /// P7
        P4IN_P7: 7 = struct P4IN_P7(bool);
    }
    /// Port 4 Output
    rw P4OUT @ 0x05: u8 = 0_0 {
        /// P0
        P4OUT_P0: 0 = struct P4OUT_P0(bool);
        /// P1
        P4OUT_P1: 1 = struct P4OUT_P1(bool);
        /// P2
        P4OUT_P2: 2 = struct P4OUT_P2(bool);
        /// P3
        P4OUT_P3: 3 = struct P4OUT_P3(bool);
        /// P4
        P4OUT_P4: 4 = struct P4OUT_P4(bool);
        /// P5
        P4OUT_P5: 5 = struct P4OUT_P5(bool);
        /// P6
        P4OUT_P6: 6 = struct P4OUT_P6(bool);
        /// P7
        P4OUT_P7: 7 = struct P4OUT_P7(bool);
    }
    /// Port 4 Direction
    rw P4DIR @ 0x06: u8 = 0_0 {
        /// P0
        P4DIR_P0: 0 = struct P4DIR_P0(bool);
        /// P1
        P4DIR_P1: 1 = struct P4DIR_P1(bool);
        /// P2
        P4DIR_P2: 2 = struct P4DIR_P2(bool);
        /// P3
        P4DIR_P3: 3 = struct P4DIR_P3(bool);
        /// P4
        P4DIR_P4: 4 = struct P4DIR_P4(bool);
        /// P5
        P4DIR_P5: 5 = struct P4DIR_P5(bool);
        /// P6
        P4DIR_P6: 6 = struct P4DIR_P6(bool);
        /// P7
        P4DIR_P7: 7 = struct P4DIR_P7(bool);
    }
    /// Port 4 Selection
    rw P4SEL @ 0x07: u8 = 0_0 {
        /// P0
        P4SEL_P0: 0 = struct P4SEL_P0(bool);
        /// P1
        P4SEL_P1: 1 = struct P4SEL_P1(bool);
        /// P2
        P4SEL_P2: 2 = struct P4SEL_P2(bool);
        /// P3
        P4SEL_P3: 3 = struct P4SEL_P3(bool);
        /// P4
        P4SEL_P4: 4 = struct P4SEL_P4(bool);
        /// P5
        P4SEL_P5: 5 = struct P4SEL_P5(bool);
        /// P6
        P4SEL_P6: 6 = struct P4SEL_P6(bool);
        /// P7
        P4SEL_P7: 7 = struct P4SEL_P7(bool);
    }
}
