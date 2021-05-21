MEMORY {
  RAM              : ORIGIN = 0x2400, LENGTH = 0x2000 /* END=0x43FF, size 8192 */
  ROM (rx)         : ORIGIN = 0x8000, LENGTH = 0x7F80 /* END=0xFF7F, size 32640 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
