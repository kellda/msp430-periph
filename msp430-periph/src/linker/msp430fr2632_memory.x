MEMORY {
  RAM              : ORIGIN = 0x2000, LENGTH = 0x0800 /* END=0x27FF, size 2048 */
  ROM (rx)         : ORIGIN = 0xE000, LENGTH = 0x1F80 /* END=0xFF7F, size 8064 */
  VECTORS          : ORIGIN = 0xff88, LENGTH = 0x0078
}
