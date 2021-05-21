MEMORY {
  RAM              : ORIGIN = 0x2000, LENGTH = 0x0800
  ROM (rx)         : ORIGIN = 0xE300, LENGTH = 0x1C80 /* END=0xFF7F, size 7296 */
  VECTORS          : ORIGIN = 0xff8a, LENGTH = 0x0076
}
