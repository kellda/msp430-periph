MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x8000 /* END=0x9BFF, size 32768 */
  ROM (rx)         : ORIGIN = 0xC000, LENGTH = 0x3F80 /* END=0xFF7F, size 16256 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
