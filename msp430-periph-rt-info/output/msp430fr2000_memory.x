MEMORY {
  RAM              : ORIGIN = 0x2000, LENGTH = 0x0200
  ROM (rx)         : ORIGIN = 0xFE00, LENGTH = 0x0180 /* END=0xFF7F, size 384 */
  VECTORS          : ORIGIN = 0xff8a, LENGTH = 0x0076
}
