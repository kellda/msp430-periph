MEMORY {
  RAM              : ORIGIN = 0x2400, LENGTH = 0x1000 /* END=0x33FF, size 4096 */
  ROM (rx)         : ORIGIN = 0xE000, LENGTH = 0x1F80 /* END=0xFF7F, size 8064 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
