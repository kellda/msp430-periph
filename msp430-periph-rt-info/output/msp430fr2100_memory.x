MEMORY {
  RAM              : ORIGIN = 0x2000, LENGTH = 0x0200
  ROM (rx)         : ORIGIN = 0xFC00, LENGTH = 0x0380 /* END=0xFF7F, size 896 */
  VECTORS          : ORIGIN = 0xff8a, LENGTH = 0x0076
}
