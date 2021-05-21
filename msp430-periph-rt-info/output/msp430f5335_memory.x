MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x4800 /* END=0x63FF, size 18432 */
  ROM (rx)         : ORIGIN = 0x8000, LENGTH = 0x7F80 /* END=0xFF7F, size 32640 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
