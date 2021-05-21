MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x0FFE /* END=0x2BFD, size 4094 */
  ROM (rx)         : ORIGIN = 0x8000, LENGTH = 0x7F80 /* END=0xFF7F, size 32640 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
