MEMORY {
  RAM              : ORIGIN = 0x1100, LENGTH = 0x1400 /* END=0x24FF, size 5120 */
  RAM_MIRROR       : ORIGIN = 0x0200, LENGTH = 0x0800
  ROM (rx)         : ORIGIN = 0x2500, LENGTH = 0xDAE0 /* END=0xFFDF, size 56032 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
