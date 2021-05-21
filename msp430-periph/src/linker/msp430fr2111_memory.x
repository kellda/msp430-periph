MEMORY {
  RAM              : ORIGIN = 0x2000, LENGTH = 0x0400
  ROM (rx)         : ORIGIN = 0xF100, LENGTH = 0x0E80 /* END=0xFF7F, size 3712 */
  VECTORS          : ORIGIN = 0xff8a, LENGTH = 0x0076
}
