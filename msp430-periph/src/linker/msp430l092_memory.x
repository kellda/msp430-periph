MEMORY {
  RAM              : ORIGIN = 0x2380, LENGTH = 0x0080 /* END=0x23FF, size 128 */
  ROM (rx)         : ORIGIN = 0x1C80, LENGTH = 0x0700 /* END=0x237F, size 1792 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
