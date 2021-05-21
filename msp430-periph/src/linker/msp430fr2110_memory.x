MEMORY {
  RAM              : ORIGIN = 0x2000, LENGTH = 0x0400
  ROM (rx)         : ORIGIN = 0xF800, LENGTH = 0x0780 /* END=0xFF7F, size 1920 */
  VECTORS          : ORIGIN = 0xff8a, LENGTH = 0x0076
}
