MEMORY {
  RAM              : ORIGIN = 0x1100, LENGTH = 0x1400 /* END=0x24FF, size 5120 */
  RAM_MIRROR       : ORIGIN = 0x0200, LENGTH = 0x0800
  ROM (rx)         : ORIGIN = 0x8000, LENGTH = 0x7FE0 /* END=0xFFDF, size 32736 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
