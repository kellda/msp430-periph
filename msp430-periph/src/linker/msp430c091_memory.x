MEMORY {
  RAM              : ORIGIN = 0x2380, LENGTH = 0x0080 /* END=0x23FF, size 128 */
  ROM (rx)         : ORIGIN = 0xFC80, LENGTH = 0x0360 /* END=0xFFDF, size 864 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
