MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x07FE /* END=0x23FD, size 2046 */
  ROM (rx)         : ORIGIN = 0xE000, LENGTH = 0x1F80 /* END=0xFF7F, size 8064 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
