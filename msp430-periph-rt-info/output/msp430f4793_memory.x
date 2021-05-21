MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0A00 /* END=0x0BFF, size 2560 */
  ROM (rx)         : ORIGIN = 0x1100, LENGTH = 0xEEE0 /* END=0xFFDF, size 61152 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
