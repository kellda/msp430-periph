MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x0400 /* END=0x1FFF, size 1024 */
  ROM (rx)         : ORIGIN = 0xC000, LENGTH = 0x3F80 /* END=0xFF7F, size 16256 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
