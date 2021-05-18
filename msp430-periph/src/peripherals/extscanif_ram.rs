//! ExtScanIF_RAM

utils::periph! {
    /// ExtScanIF_RAM
    ExtScanIF_RAM;
    /// ESI RAM 0
    rw ESIRAM0 @ 0x00: u8 = 0_0 {
        /// ESI RAM 0
        ESIRAM0: 0..7 = struct ESIRAM0Field(u8);
    }
    /// ESI RAM 1
    rw ESIRAM1 @ 0x01: u8 = 0_0 {
        /// ESI RAM 1
        ESIRAM1: 0..7 = struct ESIRAM1Field(u8);
    }
    /// ESI RAM 2
    rw ESIRAM2 @ 0x02: u8 = 0_0 {
        /// ESI RAM 2
        ESIRAM2: 0..7 = struct ESIRAM2Field(u8);
    }
    /// ESI RAM 3
    rw ESIRAM3 @ 0x03: u8 = 0_0 {
        /// ESI RAM 3
        ESIRAM3: 0..7 = struct ESIRAM3Field(u8);
    }
    /// ESI RAM 4
    rw ESIRAM4 @ 0x04: u8 = 0_0 {
        /// ESI RAM 4
        ESIRAM4: 0..7 = struct ESIRAM4Field(u8);
    }
    /// ESI RAM 5
    rw ESIRAM5 @ 0x05: u8 = 0_0 {
        /// ESI RAM 5
        ESIRAM5: 0..7 = struct ESIRAM5Field(u8);
    }
    /// ESI RAM 6
    rw ESIRAM6 @ 0x06: u8 = 0_0 {
        /// ESI RAM 6
        ESIRAM6: 0..7 = struct ESIRAM6Field(u8);
    }
    /// ESI RAM 7
    rw ESIRAM7 @ 0x07: u8 = 0_0 {
        /// ESI RAM 7
        ESIRAM7: 0..7 = struct ESIRAM7Field(u8);
    }
    /// ESI RAM 8
    rw ESIRAM8 @ 0x08: u8 = 0_0 {
        /// ESI RAM 8
        ESIRAM8: 0..7 = struct ESIRAM8Field(u8);
    }
    /// ESI RAM 9
    rw ESIRAM9 @ 0x09: u8 = 0_0 {
        /// ESI RAM 9
        ESIRAM9: 0..7 = struct ESIRAM9Field(u8);
    }
    /// ESI RAM 10
    rw ESIRAM10 @ 0x0a: u8 = 0_0 {
        /// ESI RAM 10
        ESIRAM10: 0..7 = struct ESIRAM10Field(u8);
    }
    /// ESI RAM 11
    rw ESIRAM11 @ 0x0b: u8 = 0_0 {
        /// ESI RAM 11
        ESIRAM11: 0..7 = struct ESIRAM11Field(u8);
    }
    /// ESI RAM 12
    rw ESIRAM12 @ 0x0c: u8 = 0_0 {
        /// ESI RAM 12
        ESIRAM12: 0..7 = struct ESIRAM12Field(u8);
    }
    /// ESI RAM 13
    rw ESIRAM13 @ 0x0d: u8 = 0_0 {
        /// ESI RAM 13
        ESIRAM13: 0..7 = struct ESIRAM13Field(u8);
    }
    /// ESI RAM 14
    rw ESIRAM14 @ 0x0e: u8 = 0_0 {
        /// ESI RAM 14
        ESIRAM14: 0..7 = struct ESIRAM14Field(u8);
    }
    /// ESI RAM 15
    rw ESIRAM15 @ 0x0f: u8 = 0_0 {
        /// ESI RAM 15
        ESIRAM15: 0..7 = struct ESIRAM15Field(u8);
    }
    /// ESI RAM 16
    rw ESIRAM16 @ 0x10: u8 = 0_0 {
        /// ESI RAM 16
        ESIRAM16: 0..7 = struct ESIRAM16Field(u8);
    }
    /// ESI RAM 17
    rw ESIRAM17 @ 0x11: u8 = 0_0 {
        /// ESI RAM 17
        ESIRAM17: 0..7 = struct ESIRAM17Field(u8);
    }
    /// ESI RAM 18
    rw ESIRAM18 @ 0x12: u8 = 0_0 {
        /// ESI RAM 18
        ESIRAM18: 0..7 = struct ESIRAM18Field(u8);
    }
    /// ESI RAM 19
    rw ESIRAM19 @ 0x13: u8 = 0_0 {
        /// ESI RAM 19
        ESIRAM19: 0..7 = struct ESIRAM19Field(u8);
    }
    /// ESI RAM 20
    rw ESIRAM20 @ 0x14: u8 = 0_0 {
        /// ESI RAM 20
        ESIRAM20: 0..7 = struct ESIRAM20Field(u8);
    }
    /// ESI RAM 21
    rw ESIRAM21 @ 0x15: u8 = 0_0 {
        /// ESI RAM 21
        ESIRAM21: 0..7 = struct ESIRAM21Field(u8);
    }
    /// ESI RAM 22
    rw ESIRAM22 @ 0x16: u8 = 0_0 {
        /// ESI RAM 22
        ESIRAM22: 0..7 = struct ESIRAM22Field(u8);
    }
    /// ESI RAM 23
    rw ESIRAM23 @ 0x17: u8 = 0_0 {
        /// ESI RAM 23
        ESIRAM23: 0..7 = struct ESIRAM23Field(u8);
    }
    /// ESI RAM 24
    rw ESIRAM24 @ 0x18: u8 = 0_0 {
        /// ESI RAM 24
        ESIRAM24: 0..7 = struct ESIRAM24Field(u8);
    }
    /// ESI RAM 25
    rw ESIRAM25 @ 0x19: u8 = 0_0 {
        /// ESI RAM 25
        ESIRAM25: 0..7 = struct ESIRAM25Field(u8);
    }
    /// ESI RAM 26
    rw ESIRAM26 @ 0x1a: u8 = 0_0 {
        /// ESI RAM 26
        ESIRAM26: 0..7 = struct ESIRAM26Field(u8);
    }
    /// ESI RAM 27
    rw ESIRAM27 @ 0x1b: u8 = 0_0 {
        /// ESI RAM 27
        ESIRAM27: 0..7 = struct ESIRAM27Field(u8);
    }
    /// ESI RAM 28
    rw ESIRAM28 @ 0x1c: u8 = 0_0 {
        /// ESI RAM 28
        ESIRAM28: 0..7 = struct ESIRAM28Field(u8);
    }
    /// ESI RAM 29
    rw ESIRAM29 @ 0x1d: u8 = 0_0 {
        /// ESI RAM 29
        ESIRAM29: 0..7 = struct ESIRAM29Field(u8);
    }
    /// ESI RAM 30
    rw ESIRAM30 @ 0x1e: u8 = 0_0 {
        /// ESI RAM 30
        ESIRAM30: 0..7 = struct ESIRAM30Field(u8);
    }
    /// ESI RAM 31
    rw ESIRAM31 @ 0x1f: u8 = 0_0 {
        /// ESI RAM 31
        ESIRAM31: 0..7 = struct ESIRAM31Field(u8);
    }
    /// ESI RAM 32
    rw ESIRAM32 @ 0x20: u8 = 0_0 {
        /// ESI RAM 32
        ESIRAM32: 0..7 = struct ESIRAM32Field(u8);
    }
    /// ESI RAM 33
    rw ESIRAM33 @ 0x21: u8 = 0_0 {
        /// ESI RAM 33
        ESIRAM33: 0..7 = struct ESIRAM33Field(u8);
    }
    /// ESI RAM 34
    rw ESIRAM34 @ 0x22: u8 = 0_0 {
        /// ESI RAM 34
        ESIRAM34: 0..7 = struct ESIRAM34Field(u8);
    }
    /// ESI RAM 35
    rw ESIRAM35 @ 0x23: u8 = 0_0 {
        /// ESI RAM 35
        ESIRAM35: 0..7 = struct ESIRAM35Field(u8);
    }
    /// ESI RAM 36
    rw ESIRAM36 @ 0x24: u8 = 0_0 {
        /// ESI RAM 36
        ESIRAM36: 0..7 = struct ESIRAM36Field(u8);
    }
    /// ESI RAM 37
    rw ESIRAM37 @ 0x25: u8 = 0_0 {
        /// ESI RAM 37
        ESIRAM37: 0..7 = struct ESIRAM37Field(u8);
    }
    /// ESI RAM 38
    rw ESIRAM38 @ 0x26: u8 = 0_0 {
        /// ESI RAM 38
        ESIRAM38: 0..7 = struct ESIRAM38Field(u8);
    }
    /// ESI RAM 39
    rw ESIRAM39 @ 0x27: u8 = 0_0 {
        /// ESI RAM 39
        ESIRAM39: 0..7 = struct ESIRAM39Field(u8);
    }
    /// ESI RAM 40
    rw ESIRAM40 @ 0x28: u8 = 0_0 {
        /// ESI RAM 40
        ESIRAM40: 0..7 = struct ESIRAM40Field(u8);
    }
    /// ESI RAM 41
    rw ESIRAM41 @ 0x29: u8 = 0_0 {
        /// ESI RAM 41
        ESIRAM41: 0..7 = struct ESIRAM41Field(u8);
    }
    /// ESI RAM 42
    rw ESIRAM42 @ 0x2a: u8 = 0_0 {
        /// ESI RAM 42
        ESIRAM42: 0..7 = struct ESIRAM42Field(u8);
    }
    /// ESI RAM 43
    rw ESIRAM43 @ 0x2b: u8 = 0_0 {
        /// ESI RAM 43
        ESIRAM43: 0..7 = struct ESIRAM43Field(u8);
    }
    /// ESI RAM 44
    rw ESIRAM44 @ 0x2c: u8 = 0_0 {
        /// ESI RAM 44
        ESIRAM44: 0..7 = struct ESIRAM44Field(u8);
    }
    /// ESI RAM 45
    rw ESIRAM45 @ 0x2d: u8 = 0_0 {
        /// ESI RAM 45
        ESIRAM45: 0..7 = struct ESIRAM45Field(u8);
    }
    /// ESI RAM 46
    rw ESIRAM46 @ 0x2e: u8 = 0_0 {
        /// ESI RAM 46
        ESIRAM46: 0..7 = struct ESIRAM46Field(u8);
    }
    /// ESI RAM 47
    rw ESIRAM47 @ 0x2f: u8 = 0_0 {
        /// ESI RAM 47
        ESIRAM47: 0..7 = struct ESIRAM47Field(u8);
    }
    /// ESI RAM 48
    rw ESIRAM48 @ 0x30: u8 = 0_0 {
        /// ESI RAM 48
        ESIRAM48: 0..7 = struct ESIRAM48Field(u8);
    }
    /// ESI RAM 49
    rw ESIRAM49 @ 0x31: u8 = 0_0 {
        /// ESI RAM 49
        ESIRAM49: 0..7 = struct ESIRAM49Field(u8);
    }
    /// ESI RAM 50
    rw ESIRAM50 @ 0x32: u8 = 0_0 {
        /// ESI RAM 50
        ESIRAM50: 0..7 = struct ESIRAM50Field(u8);
    }
    /// ESI RAM 51
    rw ESIRAM51 @ 0x33: u8 = 0_0 {
        /// ESI RAM 51
        ESIRAM51: 0..7 = struct ESIRAM51Field(u8);
    }
    /// ESI RAM 52
    rw ESIRAM52 @ 0x34: u8 = 0_0 {
        /// ESI RAM 52
        ESIRAM52: 0..7 = struct ESIRAM52Field(u8);
    }
    /// ESI RAM 53
    rw ESIRAM53 @ 0x35: u8 = 0_0 {
        /// ESI RAM 53
        ESIRAM53: 0..7 = struct ESIRAM53Field(u8);
    }
    /// ESI RAM 54
    rw ESIRAM54 @ 0x36: u8 = 0_0 {
        /// ESI RAM 54
        ESIRAM54: 0..7 = struct ESIRAM54Field(u8);
    }
    /// ESI RAM 55
    rw ESIRAM55 @ 0x37: u8 = 0_0 {
        /// ESI RAM 55
        ESIRAM55: 0..7 = struct ESIRAM55Field(u8);
    }
    /// ESI RAM 56
    rw ESIRAM56 @ 0x38: u8 = 0_0 {
        /// ESI RAM 56
        ESIRAM56: 0..7 = struct ESIRAM56Field(u8);
    }
    /// ESI RAM 57
    rw ESIRAM57 @ 0x39: u8 = 0_0 {
        /// ESI RAM 57
        ESIRAM57: 0..7 = struct ESIRAM57Field(u8);
    }
    /// ESI RAM 58
    rw ESIRAM58 @ 0x3a: u8 = 0_0 {
        /// ESI RAM 58
        ESIRAM58: 0..7 = struct ESIRAM58Field(u8);
    }
    /// ESI RAM 59
    rw ESIRAM59 @ 0x3b: u8 = 0_0 {
        /// ESI RAM 59
        ESIRAM59: 0..7 = struct ESIRAM59Field(u8);
    }
    /// ESI RAM 60
    rw ESIRAM60 @ 0x3c: u8 = 0_0 {
        /// ESI RAM 60
        ESIRAM60: 0..7 = struct ESIRAM60Field(u8);
    }
    /// ESI RAM 61
    rw ESIRAM61 @ 0x3d: u8 = 0_0 {
        /// ESI RAM 61
        ESIRAM61: 0..7 = struct ESIRAM61Field(u8);
    }
    /// ESI RAM 62
    rw ESIRAM62 @ 0x3e: u8 = 0_0 {
        /// ESI RAM 62
        ESIRAM62: 0..7 = struct ESIRAM62Field(u8);
    }
    /// ESI RAM 63
    rw ESIRAM63 @ 0x3f: u8 = 0_0 {
        /// ESI RAM 63
        ESIRAM63: 0..7 = struct ESIRAM63Field(u8);
    }
    /// ESI RAM 64
    rw ESIRAM64 @ 0x40: u8 = 0_0 {
        /// ESI RAM 64
        ESIRAM64: 0..7 = struct ESIRAM64Field(u8);
    }
    /// ESI RAM 65
    rw ESIRAM65 @ 0x41: u8 = 0_0 {
        /// ESI RAM 65
        ESIRAM65: 0..7 = struct ESIRAM65Field(u8);
    }
    /// ESI RAM 66
    rw ESIRAM66 @ 0x42: u8 = 0_0 {
        /// ESI RAM 66
        ESIRAM66: 0..7 = struct ESIRAM66Field(u8);
    }
    /// ESI RAM 67
    rw ESIRAM67 @ 0x43: u8 = 0_0 {
        /// ESI RAM 67
        ESIRAM67: 0..7 = struct ESIRAM67Field(u8);
    }
    /// ESI RAM 68
    rw ESIRAM68 @ 0x44: u8 = 0_0 {
        /// ESI RAM 68
        ESIRAM68: 0..7 = struct ESIRAM68Field(u8);
    }
    /// ESI RAM 69
    rw ESIRAM69 @ 0x45: u8 = 0_0 {
        /// ESI RAM 69
        ESIRAM69: 0..7 = struct ESIRAM69Field(u8);
    }
    /// ESI RAM 70
    rw ESIRAM70 @ 0x46: u8 = 0_0 {
        /// ESI RAM 70
        ESIRAM70: 0..7 = struct ESIRAM70Field(u8);
    }
    /// ESI RAM 71
    rw ESIRAM71 @ 0x47: u8 = 0_0 {
        /// ESI RAM 71
        ESIRAM71: 0..7 = struct ESIRAM71Field(u8);
    }
    /// ESI RAM 72
    rw ESIRAM72 @ 0x48: u8 = 0_0 {
        /// ESI RAM 72
        ESIRAM72: 0..7 = struct ESIRAM72Field(u8);
    }
    /// ESI RAM 73
    rw ESIRAM73 @ 0x49: u8 = 0_0 {
        /// ESI RAM 73
        ESIRAM73: 0..7 = struct ESIRAM73Field(u8);
    }
    /// ESI RAM 74
    rw ESIRAM74 @ 0x4a: u8 = 0_0 {
        /// ESI RAM 74
        ESIRAM74: 0..7 = struct ESIRAM74Field(u8);
    }
    /// ESI RAM 75
    rw ESIRAM75 @ 0x4b: u8 = 0_0 {
        /// ESI RAM 75
        ESIRAM75: 0..7 = struct ESIRAM75Field(u8);
    }
    /// ESI RAM 76
    rw ESIRAM76 @ 0x4c: u8 = 0_0 {
        /// ESI RAM 76
        ESIRAM76: 0..7 = struct ESIRAM76Field(u8);
    }
    /// ESI RAM 77
    rw ESIRAM77 @ 0x4d: u8 = 0_0 {
        /// ESI RAM 77
        ESIRAM77: 0..7 = struct ESIRAM77Field(u8);
    }
    /// ESI RAM 78
    rw ESIRAM78 @ 0x4e: u8 = 0_0 {
        /// ESI RAM 78
        ESIRAM78: 0..7 = struct ESIRAM78Field(u8);
    }
    /// ESI RAM 79
    rw ESIRAM79 @ 0x4f: u8 = 0_0 {
        /// ESI RAM 79
        ESIRAM79: 0..7 = struct ESIRAM79Field(u8);
    }
    /// ESI RAM 80
    rw ESIRAM80 @ 0x50: u8 = 0_0 {
        /// ESI RAM 80
        ESIRAM80: 0..7 = struct ESIRAM80Field(u8);
    }
    /// ESI RAM 81
    rw ESIRAM81 @ 0x51: u8 = 0_0 {
        /// ESI RAM 81
        ESIRAM81: 0..7 = struct ESIRAM81Field(u8);
    }
    /// ESI RAM 82
    rw ESIRAM82 @ 0x52: u8 = 0_0 {
        /// ESI RAM 82
        ESIRAM82: 0..7 = struct ESIRAM82Field(u8);
    }
    /// ESI RAM 83
    rw ESIRAM83 @ 0x53: u8 = 0_0 {
        /// ESI RAM 83
        ESIRAM83: 0..7 = struct ESIRAM83Field(u8);
    }
    /// ESI RAM 84
    rw ESIRAM84 @ 0x54: u8 = 0_0 {
        /// ESI RAM 84
        ESIRAM84: 0..7 = struct ESIRAM84Field(u8);
    }
    /// ESI RAM 85
    rw ESIRAM85 @ 0x55: u8 = 0_0 {
        /// ESI RAM 85
        ESIRAM85: 0..7 = struct ESIRAM85Field(u8);
    }
    /// ESI RAM 86
    rw ESIRAM86 @ 0x56: u8 = 0_0 {
        /// ESI RAM 86
        ESIRAM86: 0..7 = struct ESIRAM86Field(u8);
    }
    /// ESI RAM 87
    rw ESIRAM87 @ 0x57: u8 = 0_0 {
        /// ESI RAM 87
        ESIRAM87: 0..7 = struct ESIRAM87Field(u8);
    }
    /// ESI RAM 88
    rw ESIRAM88 @ 0x58: u8 = 0_0 {
        /// ESI RAM 88
        ESIRAM88: 0..7 = struct ESIRAM88Field(u8);
    }
    /// ESI RAM 89
    rw ESIRAM89 @ 0x59: u8 = 0_0 {
        /// ESI RAM 89
        ESIRAM89: 0..7 = struct ESIRAM89Field(u8);
    }
    /// ESI RAM 90
    rw ESIRAM90 @ 0x5a: u8 = 0_0 {
        /// ESI RAM 90
        ESIRAM90: 0..7 = struct ESIRAM90Field(u8);
    }
    /// ESI RAM 91
    rw ESIRAM91 @ 0x5b: u8 = 0_0 {
        /// ESI RAM 91
        ESIRAM91: 0..7 = struct ESIRAM91Field(u8);
    }
    /// ESI RAM 92
    rw ESIRAM92 @ 0x5c: u8 = 0_0 {
        /// ESI RAM 92
        ESIRAM92: 0..7 = struct ESIRAM92Field(u8);
    }
    /// ESI RAM 93
    rw ESIRAM93 @ 0x5d: u8 = 0_0 {
        /// ESI RAM 93
        ESIRAM93: 0..7 = struct ESIRAM93Field(u8);
    }
    /// ESI RAM 94
    rw ESIRAM94 @ 0x5e: u8 = 0_0 {
        /// ESI RAM 94
        ESIRAM94: 0..7 = struct ESIRAM94Field(u8);
    }
    /// ESI RAM 95
    rw ESIRAM95 @ 0x5f: u8 = 0_0 {
        /// ESI RAM 95
        ESIRAM95: 0..7 = struct ESIRAM95Field(u8);
    }
    /// ESI RAM 96
    rw ESIRAM96 @ 0x60: u8 = 0_0 {
        /// ESI RAM 96
        ESIRAM96: 0..7 = struct ESIRAM96Field(u8);
    }
    /// ESI RAM 97
    rw ESIRAM97 @ 0x61: u8 = 0_0 {
        /// ESI RAM 97
        ESIRAM97: 0..7 = struct ESIRAM97Field(u8);
    }
    /// ESI RAM 98
    rw ESIRAM98 @ 0x62: u8 = 0_0 {
        /// ESI RAM 98
        ESIRAM98: 0..7 = struct ESIRAM98Field(u8);
    }
    /// ESI RAM 99
    rw ESIRAM99 @ 0x63: u8 = 0_0 {
        /// ESI RAM 99
        ESIRAM99: 0..7 = struct ESIRAM99Field(u8);
    }
    /// ESI RAM 100
    rw ESIRAM100 @ 0x64: u8 = 0_0 {
        /// ESI RAM 100
        ESIRAM100: 0..7 = struct ESIRAM100Field(u8);
    }
    /// ESI RAM 101
    rw ESIRAM101 @ 0x65: u8 = 0_0 {
        /// ESI RAM 101
        ESIRAM101: 0..7 = struct ESIRAM101Field(u8);
    }
    /// ESI RAM 102
    rw ESIRAM102 @ 0x66: u8 = 0_0 {
        /// ESI RAM 102
        ESIRAM102: 0..7 = struct ESIRAM102Field(u8);
    }
    /// ESI RAM 103
    rw ESIRAM103 @ 0x67: u8 = 0_0 {
        /// ESI RAM 103
        ESIRAM103: 0..7 = struct ESIRAM103Field(u8);
    }
    /// ESI RAM 104
    rw ESIRAM104 @ 0x68: u8 = 0_0 {
        /// ESI RAM 104
        ESIRAM104: 0..7 = struct ESIRAM104Field(u8);
    }
    /// ESI RAM 105
    rw ESIRAM105 @ 0x69: u8 = 0_0 {
        /// ESI RAM 105
        ESIRAM105: 0..7 = struct ESIRAM105Field(u8);
    }
    /// ESI RAM 106
    rw ESIRAM106 @ 0x6a: u8 = 0_0 {
        /// ESI RAM 106
        ESIRAM106: 0..7 = struct ESIRAM106Field(u8);
    }
    /// ESI RAM 107
    rw ESIRAM107 @ 0x6b: u8 = 0_0 {
        /// ESI RAM 107
        ESIRAM107: 0..7 = struct ESIRAM107Field(u8);
    }
    /// ESI RAM 108
    rw ESIRAM108 @ 0x6c: u8 = 0_0 {
        /// ESI RAM 108
        ESIRAM108: 0..7 = struct ESIRAM108Field(u8);
    }
    /// ESI RAM 109
    rw ESIRAM109 @ 0x6d: u8 = 0_0 {
        /// ESI RAM 109
        ESIRAM109: 0..7 = struct ESIRAM109Field(u8);
    }
    /// ESI RAM 110
    rw ESIRAM110 @ 0x6e: u8 = 0_0 {
        /// ESI RAM 110
        ESIRAM110: 0..7 = struct ESIRAM110Field(u8);
    }
    /// ESI RAM 111
    rw ESIRAM111 @ 0x6f: u8 = 0_0 {
        /// ESI RAM 111
        ESIRAM111: 0..7 = struct ESIRAM111Field(u8);
    }
    /// ESI RAM 112
    rw ESIRAM112 @ 0x70: u8 = 0_0 {
        /// ESI RAM 112
        ESIRAM112: 0..7 = struct ESIRAM112Field(u8);
    }
    /// ESI RAM 113
    rw ESIRAM113 @ 0x71: u8 = 0_0 {
        /// ESI RAM 113
        ESIRAM113: 0..7 = struct ESIRAM113Field(u8);
    }
    /// ESI RAM 114
    rw ESIRAM114 @ 0x72: u8 = 0_0 {
        /// ESI RAM 114
        ESIRAM114: 0..7 = struct ESIRAM114Field(u8);
    }
    /// ESI RAM 115
    rw ESIRAM115 @ 0x73: u8 = 0_0 {
        /// ESI RAM 115
        ESIRAM115: 0..7 = struct ESIRAM115Field(u8);
    }
    /// ESI RAM 116
    rw ESIRAM116 @ 0x74: u8 = 0_0 {
        /// ESI RAM 116
        ESIRAM116: 0..7 = struct ESIRAM116Field(u8);
    }
    /// ESI RAM 117
    rw ESIRAM117 @ 0x75: u8 = 0_0 {
        /// ESI RAM 117
        ESIRAM117: 0..7 = struct ESIRAM117Field(u8);
    }
    /// ESI RAM 118
    rw ESIRAM118 @ 0x76: u8 = 0_0 {
        /// ESI RAM 118
        ESIRAM118: 0..7 = struct ESIRAM118Field(u8);
    }
    /// ESI RAM 119
    rw ESIRAM119 @ 0x77: u8 = 0_0 {
        /// ESI RAM 119
        ESIRAM119: 0..7 = struct ESIRAM119Field(u8);
    }
    /// ESI RAM 120
    rw ESIRAM120 @ 0x78: u8 = 0_0 {
        /// ESI RAM 120
        ESIRAM120: 0..7 = struct ESIRAM120Field(u8);
    }
    /// ESI RAM 121
    rw ESIRAM121 @ 0x79: u8 = 0_0 {
        /// ESI RAM 121
        ESIRAM121: 0..7 = struct ESIRAM121Field(u8);
    }
    /// ESI RAM 122
    rw ESIRAM122 @ 0x7a: u8 = 0_0 {
        /// ESI RAM 122
        ESIRAM122: 0..7 = struct ESIRAM122Field(u8);
    }
    /// ESI RAM 123
    rw ESIRAM123 @ 0x7b: u8 = 0_0 {
        /// ESI RAM 123
        ESIRAM123: 0..7 = struct ESIRAM123Field(u8);
    }
    /// ESI RAM 124
    rw ESIRAM124 @ 0x7c: u8 = 0_0 {
        /// ESI RAM 124
        ESIRAM124: 0..7 = struct ESIRAM124Field(u8);
    }
    /// ESI RAM 125
    rw ESIRAM125 @ 0x7d: u8 = 0_0 {
        /// ESI RAM 125
        ESIRAM125: 0..7 = struct ESIRAM125Field(u8);
    }
    /// ESI RAM 126
    rw ESIRAM126 @ 0x7e: u8 = 0_0 {
        /// ESI RAM 126
        ESIRAM126: 0..7 = struct ESIRAM126Field(u8);
    }
    /// ESI RAM 127
    rw ESIRAM127 @ 0x7f: u8 = 0_0 {
        /// ESI RAM 127
        ESIRAM127: 0..7 = struct ESIRAM127Field(u8);
    }
}
