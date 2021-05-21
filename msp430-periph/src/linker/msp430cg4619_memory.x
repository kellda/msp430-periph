MEMORY {
  RAM              : ORIGIN = 0x1100, LENGTH = 0x1000 /* END=0x20FF, size 4096 */
  RAM_MIRROR       : ORIGIN = 0x0200, LENGTH = 0x0800
  ROM (rx)         : ORIGIN = 0x2100, LENGTH = 0xDEC0 /* END=0xFFBF, size 57024 */
  VECTORS          : ORIGIN = 0xffc0, LENGTH = 0x0040
}
