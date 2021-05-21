MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x1000
  ROM (rx)         : ORIGIN = 0x6000, LENGTH = 0x9F80 /* END=0xFF7F, size 40832 */
  VECTORS          : ORIGIN = 0xff92, LENGTH = 0x006e
}
