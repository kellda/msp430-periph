//! ExtScanIF_RAM

utils::periph! {
    /// ExtScanIF_RAM
    ExtScanIF_RAM;
    /// ESI RAM 0
    rw RAM0 @ 0x00: u8 = 0_0 {
        /// ESI RAM 0
        RAM0: 0..7 = struct RAM0Field(u8);
    }
    /// ESI RAM 1
    rw RAM1 @ 0x01: u8 = 0_0 {
        /// ESI RAM 1
        RAM1: 0..7 = struct RAM1Field(u8);
    }
    /// ESI RAM 2
    rw RAM2 @ 0x02: u8 = 0_0 {
        /// ESI RAM 2
        RAM2: 0..7 = struct RAM2Field(u8);
    }
    /// ESI RAM 3
    rw RAM3 @ 0x03: u8 = 0_0 {
        /// ESI RAM 3
        RAM3: 0..7 = struct RAM3Field(u8);
    }
    /// ESI RAM 4
    rw RAM4 @ 0x04: u8 = 0_0 {
        /// ESI RAM 4
        RAM4: 0..7 = struct RAM4Field(u8);
    }
    /// ESI RAM 5
    rw RAM5 @ 0x05: u8 = 0_0 {
        /// ESI RAM 5
        RAM5: 0..7 = struct RAM5Field(u8);
    }
    /// ESI RAM 6
    rw RAM6 @ 0x06: u8 = 0_0 {
        /// ESI RAM 6
        RAM6: 0..7 = struct RAM6Field(u8);
    }
    /// ESI RAM 7
    rw RAM7 @ 0x07: u8 = 0_0 {
        /// ESI RAM 7
        RAM7: 0..7 = struct RAM7Field(u8);
    }
    /// ESI RAM 8
    rw RAM8 @ 0x08: u8 = 0_0 {
        /// ESI RAM 8
        RAM8: 0..7 = struct RAM8Field(u8);
    }
    /// ESI RAM 9
    rw RAM9 @ 0x09: u8 = 0_0 {
        /// ESI RAM 9
        RAM9: 0..7 = struct RAM9Field(u8);
    }
    /// ESI RAM 10
    rw RAM10 @ 0x0a: u8 = 0_0 {
        /// ESI RAM 10
        RAM10: 0..7 = struct RAM10Field(u8);
    }
    /// ESI RAM 11
    rw RAM11 @ 0x0b: u8 = 0_0 {
        /// ESI RAM 11
        RAM11: 0..7 = struct RAM11Field(u8);
    }
    /// ESI RAM 12
    rw RAM12 @ 0x0c: u8 = 0_0 {
        /// ESI RAM 12
        RAM12: 0..7 = struct RAM12Field(u8);
    }
    /// ESI RAM 13
    rw RAM13 @ 0x0d: u8 = 0_0 {
        /// ESI RAM 13
        RAM13: 0..7 = struct RAM13Field(u8);
    }
    /// ESI RAM 14
    rw RAM14 @ 0x0e: u8 = 0_0 {
        /// ESI RAM 14
        RAM14: 0..7 = struct RAM14Field(u8);
    }
    /// ESI RAM 15
    rw RAM15 @ 0x0f: u8 = 0_0 {
        /// ESI RAM 15
        RAM15: 0..7 = struct RAM15Field(u8);
    }
    /// ESI RAM 16
    rw RAM16 @ 0x10: u8 = 0_0 {
        /// ESI RAM 16
        RAM16: 0..7 = struct RAM16Field(u8);
    }
    /// ESI RAM 17
    rw RAM17 @ 0x11: u8 = 0_0 {
        /// ESI RAM 17
        RAM17: 0..7 = struct RAM17Field(u8);
    }
    /// ESI RAM 18
    rw RAM18 @ 0x12: u8 = 0_0 {
        /// ESI RAM 18
        RAM18: 0..7 = struct RAM18Field(u8);
    }
    /// ESI RAM 19
    rw RAM19 @ 0x13: u8 = 0_0 {
        /// ESI RAM 19
        RAM19: 0..7 = struct RAM19Field(u8);
    }
    /// ESI RAM 20
    rw RAM20 @ 0x14: u8 = 0_0 {
        /// ESI RAM 20
        RAM20: 0..7 = struct RAM20Field(u8);
    }
    /// ESI RAM 21
    rw RAM21 @ 0x15: u8 = 0_0 {
        /// ESI RAM 21
        RAM21: 0..7 = struct RAM21Field(u8);
    }
    /// ESI RAM 22
    rw RAM22 @ 0x16: u8 = 0_0 {
        /// ESI RAM 22
        RAM22: 0..7 = struct RAM22Field(u8);
    }
    /// ESI RAM 23
    rw RAM23 @ 0x17: u8 = 0_0 {
        /// ESI RAM 23
        RAM23: 0..7 = struct RAM23Field(u8);
    }
    /// ESI RAM 24
    rw RAM24 @ 0x18: u8 = 0_0 {
        /// ESI RAM 24
        RAM24: 0..7 = struct RAM24Field(u8);
    }
    /// ESI RAM 25
    rw RAM25 @ 0x19: u8 = 0_0 {
        /// ESI RAM 25
        RAM25: 0..7 = struct RAM25Field(u8);
    }
    /// ESI RAM 26
    rw RAM26 @ 0x1a: u8 = 0_0 {
        /// ESI RAM 26
        RAM26: 0..7 = struct RAM26Field(u8);
    }
    /// ESI RAM 27
    rw RAM27 @ 0x1b: u8 = 0_0 {
        /// ESI RAM 27
        RAM27: 0..7 = struct RAM27Field(u8);
    }
    /// ESI RAM 28
    rw RAM28 @ 0x1c: u8 = 0_0 {
        /// ESI RAM 28
        RAM28: 0..7 = struct RAM28Field(u8);
    }
    /// ESI RAM 29
    rw RAM29 @ 0x1d: u8 = 0_0 {
        /// ESI RAM 29
        RAM29: 0..7 = struct RAM29Field(u8);
    }
    /// ESI RAM 30
    rw RAM30 @ 0x1e: u8 = 0_0 {
        /// ESI RAM 30
        RAM30: 0..7 = struct RAM30Field(u8);
    }
    /// ESI RAM 31
    rw RAM31 @ 0x1f: u8 = 0_0 {
        /// ESI RAM 31
        RAM31: 0..7 = struct RAM31Field(u8);
    }
    /// ESI RAM 32
    rw RAM32 @ 0x20: u8 = 0_0 {
        /// ESI RAM 32
        RAM32: 0..7 = struct RAM32Field(u8);
    }
    /// ESI RAM 33
    rw RAM33 @ 0x21: u8 = 0_0 {
        /// ESI RAM 33
        RAM33: 0..7 = struct RAM33Field(u8);
    }
    /// ESI RAM 34
    rw RAM34 @ 0x22: u8 = 0_0 {
        /// ESI RAM 34
        RAM34: 0..7 = struct RAM34Field(u8);
    }
    /// ESI RAM 35
    rw RAM35 @ 0x23: u8 = 0_0 {
        /// ESI RAM 35
        RAM35: 0..7 = struct RAM35Field(u8);
    }
    /// ESI RAM 36
    rw RAM36 @ 0x24: u8 = 0_0 {
        /// ESI RAM 36
        RAM36: 0..7 = struct RAM36Field(u8);
    }
    /// ESI RAM 37
    rw RAM37 @ 0x25: u8 = 0_0 {
        /// ESI RAM 37
        RAM37: 0..7 = struct RAM37Field(u8);
    }
    /// ESI RAM 38
    rw RAM38 @ 0x26: u8 = 0_0 {
        /// ESI RAM 38
        RAM38: 0..7 = struct RAM38Field(u8);
    }
    /// ESI RAM 39
    rw RAM39 @ 0x27: u8 = 0_0 {
        /// ESI RAM 39
        RAM39: 0..7 = struct RAM39Field(u8);
    }
    /// ESI RAM 40
    rw RAM40 @ 0x28: u8 = 0_0 {
        /// ESI RAM 40
        RAM40: 0..7 = struct RAM40Field(u8);
    }
    /// ESI RAM 41
    rw RAM41 @ 0x29: u8 = 0_0 {
        /// ESI RAM 41
        RAM41: 0..7 = struct RAM41Field(u8);
    }
    /// ESI RAM 42
    rw RAM42 @ 0x2a: u8 = 0_0 {
        /// ESI RAM 42
        RAM42: 0..7 = struct RAM42Field(u8);
    }
    /// ESI RAM 43
    rw RAM43 @ 0x2b: u8 = 0_0 {
        /// ESI RAM 43
        RAM43: 0..7 = struct RAM43Field(u8);
    }
    /// ESI RAM 44
    rw RAM44 @ 0x2c: u8 = 0_0 {
        /// ESI RAM 44
        RAM44: 0..7 = struct RAM44Field(u8);
    }
    /// ESI RAM 45
    rw RAM45 @ 0x2d: u8 = 0_0 {
        /// ESI RAM 45
        RAM45: 0..7 = struct RAM45Field(u8);
    }
    /// ESI RAM 46
    rw RAM46 @ 0x2e: u8 = 0_0 {
        /// ESI RAM 46
        RAM46: 0..7 = struct RAM46Field(u8);
    }
    /// ESI RAM 47
    rw RAM47 @ 0x2f: u8 = 0_0 {
        /// ESI RAM 47
        RAM47: 0..7 = struct RAM47Field(u8);
    }
    /// ESI RAM 48
    rw RAM48 @ 0x30: u8 = 0_0 {
        /// ESI RAM 48
        RAM48: 0..7 = struct RAM48Field(u8);
    }
    /// ESI RAM 49
    rw RAM49 @ 0x31: u8 = 0_0 {
        /// ESI RAM 49
        RAM49: 0..7 = struct RAM49Field(u8);
    }
    /// ESI RAM 50
    rw RAM50 @ 0x32: u8 = 0_0 {
        /// ESI RAM 50
        RAM50: 0..7 = struct RAM50Field(u8);
    }
    /// ESI RAM 51
    rw RAM51 @ 0x33: u8 = 0_0 {
        /// ESI RAM 51
        RAM51: 0..7 = struct RAM51Field(u8);
    }
    /// ESI RAM 52
    rw RAM52 @ 0x34: u8 = 0_0 {
        /// ESI RAM 52
        RAM52: 0..7 = struct RAM52Field(u8);
    }
    /// ESI RAM 53
    rw RAM53 @ 0x35: u8 = 0_0 {
        /// ESI RAM 53
        RAM53: 0..7 = struct RAM53Field(u8);
    }
    /// ESI RAM 54
    rw RAM54 @ 0x36: u8 = 0_0 {
        /// ESI RAM 54
        RAM54: 0..7 = struct RAM54Field(u8);
    }
    /// ESI RAM 55
    rw RAM55 @ 0x37: u8 = 0_0 {
        /// ESI RAM 55
        RAM55: 0..7 = struct RAM55Field(u8);
    }
    /// ESI RAM 56
    rw RAM56 @ 0x38: u8 = 0_0 {
        /// ESI RAM 56
        RAM56: 0..7 = struct RAM56Field(u8);
    }
    /// ESI RAM 57
    rw RAM57 @ 0x39: u8 = 0_0 {
        /// ESI RAM 57
        RAM57: 0..7 = struct RAM57Field(u8);
    }
    /// ESI RAM 58
    rw RAM58 @ 0x3a: u8 = 0_0 {
        /// ESI RAM 58
        RAM58: 0..7 = struct RAM58Field(u8);
    }
    /// ESI RAM 59
    rw RAM59 @ 0x3b: u8 = 0_0 {
        /// ESI RAM 59
        RAM59: 0..7 = struct RAM59Field(u8);
    }
    /// ESI RAM 60
    rw RAM60 @ 0x3c: u8 = 0_0 {
        /// ESI RAM 60
        RAM60: 0..7 = struct RAM60Field(u8);
    }
    /// ESI RAM 61
    rw RAM61 @ 0x3d: u8 = 0_0 {
        /// ESI RAM 61
        RAM61: 0..7 = struct RAM61Field(u8);
    }
    /// ESI RAM 62
    rw RAM62 @ 0x3e: u8 = 0_0 {
        /// ESI RAM 62
        RAM62: 0..7 = struct RAM62Field(u8);
    }
    /// ESI RAM 63
    rw RAM63 @ 0x3f: u8 = 0_0 {
        /// ESI RAM 63
        RAM63: 0..7 = struct RAM63Field(u8);
    }
    /// ESI RAM 64
    rw RAM64 @ 0x40: u8 = 0_0 {
        /// ESI RAM 64
        RAM64: 0..7 = struct RAM64Field(u8);
    }
    /// ESI RAM 65
    rw RAM65 @ 0x41: u8 = 0_0 {
        /// ESI RAM 65
        RAM65: 0..7 = struct RAM65Field(u8);
    }
    /// ESI RAM 66
    rw RAM66 @ 0x42: u8 = 0_0 {
        /// ESI RAM 66
        RAM66: 0..7 = struct RAM66Field(u8);
    }
    /// ESI RAM 67
    rw RAM67 @ 0x43: u8 = 0_0 {
        /// ESI RAM 67
        RAM67: 0..7 = struct RAM67Field(u8);
    }
    /// ESI RAM 68
    rw RAM68 @ 0x44: u8 = 0_0 {
        /// ESI RAM 68
        RAM68: 0..7 = struct RAM68Field(u8);
    }
    /// ESI RAM 69
    rw RAM69 @ 0x45: u8 = 0_0 {
        /// ESI RAM 69
        RAM69: 0..7 = struct RAM69Field(u8);
    }
    /// ESI RAM 70
    rw RAM70 @ 0x46: u8 = 0_0 {
        /// ESI RAM 70
        RAM70: 0..7 = struct RAM70Field(u8);
    }
    /// ESI RAM 71
    rw RAM71 @ 0x47: u8 = 0_0 {
        /// ESI RAM 71
        RAM71: 0..7 = struct RAM71Field(u8);
    }
    /// ESI RAM 72
    rw RAM72 @ 0x48: u8 = 0_0 {
        /// ESI RAM 72
        RAM72: 0..7 = struct RAM72Field(u8);
    }
    /// ESI RAM 73
    rw RAM73 @ 0x49: u8 = 0_0 {
        /// ESI RAM 73
        RAM73: 0..7 = struct RAM73Field(u8);
    }
    /// ESI RAM 74
    rw RAM74 @ 0x4a: u8 = 0_0 {
        /// ESI RAM 74
        RAM74: 0..7 = struct RAM74Field(u8);
    }
    /// ESI RAM 75
    rw RAM75 @ 0x4b: u8 = 0_0 {
        /// ESI RAM 75
        RAM75: 0..7 = struct RAM75Field(u8);
    }
    /// ESI RAM 76
    rw RAM76 @ 0x4c: u8 = 0_0 {
        /// ESI RAM 76
        RAM76: 0..7 = struct RAM76Field(u8);
    }
    /// ESI RAM 77
    rw RAM77 @ 0x4d: u8 = 0_0 {
        /// ESI RAM 77
        RAM77: 0..7 = struct RAM77Field(u8);
    }
    /// ESI RAM 78
    rw RAM78 @ 0x4e: u8 = 0_0 {
        /// ESI RAM 78
        RAM78: 0..7 = struct RAM78Field(u8);
    }
    /// ESI RAM 79
    rw RAM79 @ 0x4f: u8 = 0_0 {
        /// ESI RAM 79
        RAM79: 0..7 = struct RAM79Field(u8);
    }
    /// ESI RAM 80
    rw RAM80 @ 0x50: u8 = 0_0 {
        /// ESI RAM 80
        RAM80: 0..7 = struct RAM80Field(u8);
    }
    /// ESI RAM 81
    rw RAM81 @ 0x51: u8 = 0_0 {
        /// ESI RAM 81
        RAM81: 0..7 = struct RAM81Field(u8);
    }
    /// ESI RAM 82
    rw RAM82 @ 0x52: u8 = 0_0 {
        /// ESI RAM 82
        RAM82: 0..7 = struct RAM82Field(u8);
    }
    /// ESI RAM 83
    rw RAM83 @ 0x53: u8 = 0_0 {
        /// ESI RAM 83
        RAM83: 0..7 = struct RAM83Field(u8);
    }
    /// ESI RAM 84
    rw RAM84 @ 0x54: u8 = 0_0 {
        /// ESI RAM 84
        RAM84: 0..7 = struct RAM84Field(u8);
    }
    /// ESI RAM 85
    rw RAM85 @ 0x55: u8 = 0_0 {
        /// ESI RAM 85
        RAM85: 0..7 = struct RAM85Field(u8);
    }
    /// ESI RAM 86
    rw RAM86 @ 0x56: u8 = 0_0 {
        /// ESI RAM 86
        RAM86: 0..7 = struct RAM86Field(u8);
    }
    /// ESI RAM 87
    rw RAM87 @ 0x57: u8 = 0_0 {
        /// ESI RAM 87
        RAM87: 0..7 = struct RAM87Field(u8);
    }
    /// ESI RAM 88
    rw RAM88 @ 0x58: u8 = 0_0 {
        /// ESI RAM 88
        RAM88: 0..7 = struct RAM88Field(u8);
    }
    /// ESI RAM 89
    rw RAM89 @ 0x59: u8 = 0_0 {
        /// ESI RAM 89
        RAM89: 0..7 = struct RAM89Field(u8);
    }
    /// ESI RAM 90
    rw RAM90 @ 0x5a: u8 = 0_0 {
        /// ESI RAM 90
        RAM90: 0..7 = struct RAM90Field(u8);
    }
    /// ESI RAM 91
    rw RAM91 @ 0x5b: u8 = 0_0 {
        /// ESI RAM 91
        RAM91: 0..7 = struct RAM91Field(u8);
    }
    /// ESI RAM 92
    rw RAM92 @ 0x5c: u8 = 0_0 {
        /// ESI RAM 92
        RAM92: 0..7 = struct RAM92Field(u8);
    }
    /// ESI RAM 93
    rw RAM93 @ 0x5d: u8 = 0_0 {
        /// ESI RAM 93
        RAM93: 0..7 = struct RAM93Field(u8);
    }
    /// ESI RAM 94
    rw RAM94 @ 0x5e: u8 = 0_0 {
        /// ESI RAM 94
        RAM94: 0..7 = struct RAM94Field(u8);
    }
    /// ESI RAM 95
    rw RAM95 @ 0x5f: u8 = 0_0 {
        /// ESI RAM 95
        RAM95: 0..7 = struct RAM95Field(u8);
    }
    /// ESI RAM 96
    rw RAM96 @ 0x60: u8 = 0_0 {
        /// ESI RAM 96
        RAM96: 0..7 = struct RAM96Field(u8);
    }
    /// ESI RAM 97
    rw RAM97 @ 0x61: u8 = 0_0 {
        /// ESI RAM 97
        RAM97: 0..7 = struct RAM97Field(u8);
    }
    /// ESI RAM 98
    rw RAM98 @ 0x62: u8 = 0_0 {
        /// ESI RAM 98
        RAM98: 0..7 = struct RAM98Field(u8);
    }
    /// ESI RAM 99
    rw RAM99 @ 0x63: u8 = 0_0 {
        /// ESI RAM 99
        RAM99: 0..7 = struct RAM99Field(u8);
    }
    /// ESI RAM 100
    rw RAM100 @ 0x64: u8 = 0_0 {
        /// ESI RAM 100
        RAM100: 0..7 = struct RAM100Field(u8);
    }
    /// ESI RAM 101
    rw RAM101 @ 0x65: u8 = 0_0 {
        /// ESI RAM 101
        RAM101: 0..7 = struct RAM101Field(u8);
    }
    /// ESI RAM 102
    rw RAM102 @ 0x66: u8 = 0_0 {
        /// ESI RAM 102
        RAM102: 0..7 = struct RAM102Field(u8);
    }
    /// ESI RAM 103
    rw RAM103 @ 0x67: u8 = 0_0 {
        /// ESI RAM 103
        RAM103: 0..7 = struct RAM103Field(u8);
    }
    /// ESI RAM 104
    rw RAM104 @ 0x68: u8 = 0_0 {
        /// ESI RAM 104
        RAM104: 0..7 = struct RAM104Field(u8);
    }
    /// ESI RAM 105
    rw RAM105 @ 0x69: u8 = 0_0 {
        /// ESI RAM 105
        RAM105: 0..7 = struct RAM105Field(u8);
    }
    /// ESI RAM 106
    rw RAM106 @ 0x6a: u8 = 0_0 {
        /// ESI RAM 106
        RAM106: 0..7 = struct RAM106Field(u8);
    }
    /// ESI RAM 107
    rw RAM107 @ 0x6b: u8 = 0_0 {
        /// ESI RAM 107
        RAM107: 0..7 = struct RAM107Field(u8);
    }
    /// ESI RAM 108
    rw RAM108 @ 0x6c: u8 = 0_0 {
        /// ESI RAM 108
        RAM108: 0..7 = struct RAM108Field(u8);
    }
    /// ESI RAM 109
    rw RAM109 @ 0x6d: u8 = 0_0 {
        /// ESI RAM 109
        RAM109: 0..7 = struct RAM109Field(u8);
    }
    /// ESI RAM 110
    rw RAM110 @ 0x6e: u8 = 0_0 {
        /// ESI RAM 110
        RAM110: 0..7 = struct RAM110Field(u8);
    }
    /// ESI RAM 111
    rw RAM111 @ 0x6f: u8 = 0_0 {
        /// ESI RAM 111
        RAM111: 0..7 = struct RAM111Field(u8);
    }
    /// ESI RAM 112
    rw RAM112 @ 0x70: u8 = 0_0 {
        /// ESI RAM 112
        RAM112: 0..7 = struct RAM112Field(u8);
    }
    /// ESI RAM 113
    rw RAM113 @ 0x71: u8 = 0_0 {
        /// ESI RAM 113
        RAM113: 0..7 = struct RAM113Field(u8);
    }
    /// ESI RAM 114
    rw RAM114 @ 0x72: u8 = 0_0 {
        /// ESI RAM 114
        RAM114: 0..7 = struct RAM114Field(u8);
    }
    /// ESI RAM 115
    rw RAM115 @ 0x73: u8 = 0_0 {
        /// ESI RAM 115
        RAM115: 0..7 = struct RAM115Field(u8);
    }
    /// ESI RAM 116
    rw RAM116 @ 0x74: u8 = 0_0 {
        /// ESI RAM 116
        RAM116: 0..7 = struct RAM116Field(u8);
    }
    /// ESI RAM 117
    rw RAM117 @ 0x75: u8 = 0_0 {
        /// ESI RAM 117
        RAM117: 0..7 = struct RAM117Field(u8);
    }
    /// ESI RAM 118
    rw RAM118 @ 0x76: u8 = 0_0 {
        /// ESI RAM 118
        RAM118: 0..7 = struct RAM118Field(u8);
    }
    /// ESI RAM 119
    rw RAM119 @ 0x77: u8 = 0_0 {
        /// ESI RAM 119
        RAM119: 0..7 = struct RAM119Field(u8);
    }
    /// ESI RAM 120
    rw RAM120 @ 0x78: u8 = 0_0 {
        /// ESI RAM 120
        RAM120: 0..7 = struct RAM120Field(u8);
    }
    /// ESI RAM 121
    rw RAM121 @ 0x79: u8 = 0_0 {
        /// ESI RAM 121
        RAM121: 0..7 = struct RAM121Field(u8);
    }
    /// ESI RAM 122
    rw RAM122 @ 0x7a: u8 = 0_0 {
        /// ESI RAM 122
        RAM122: 0..7 = struct RAM122Field(u8);
    }
    /// ESI RAM 123
    rw RAM123 @ 0x7b: u8 = 0_0 {
        /// ESI RAM 123
        RAM123: 0..7 = struct RAM123Field(u8);
    }
    /// ESI RAM 124
    rw RAM124 @ 0x7c: u8 = 0_0 {
        /// ESI RAM 124
        RAM124: 0..7 = struct RAM124Field(u8);
    }
    /// ESI RAM 125
    rw RAM125 @ 0x7d: u8 = 0_0 {
        /// ESI RAM 125
        RAM125: 0..7 = struct RAM125Field(u8);
    }
    /// ESI RAM 126
    rw RAM126 @ 0x7e: u8 = 0_0 {
        /// ESI RAM 126
        RAM126: 0..7 = struct RAM126Field(u8);
    }
    /// ESI RAM 127
    rw RAM127 @ 0x7f: u8 = 0_0 {
        /// ESI RAM 127
        RAM127: 0..7 = struct RAM127Field(u8);
    }
}
