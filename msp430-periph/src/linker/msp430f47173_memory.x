MEMORY {
  RAM              : ORIGIN = 0x1100, LENGTH = 0x2000 /* END=0x30FF, size 8192 */
  RAM_MIRROR       : ORIGIN = 0x0200, LENGTH = 0x0800
  ROM (rx)         : ORIGIN = 0x3100, LENGTH = 0xCEBE /* END=0xFFBD, size 52926 */
  VECTORS          : ORIGIN = 0xffc0, LENGTH = 0x0040
}
