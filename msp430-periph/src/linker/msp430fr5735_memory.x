MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x0400 /* END=0x1FFF, size 1024 */
  ROM (rx)         : ORIGIN = 0xE000, LENGTH = 0x1F80 /* END=0xFF7F, size 8064 */
  VECTORS          : ORIGIN = 0xff90, LENGTH = 0x0070
}
