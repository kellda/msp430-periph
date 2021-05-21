MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x1800 /* END=0x33FF, size 6144 */
  ROM (rx)         : ORIGIN = 0xE000, LENGTH = 0x1F80 /* END=0xFF7F, size 8064 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
