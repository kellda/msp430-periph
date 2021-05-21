MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x4000 /* END=0x5BFF, size 16384 */
  ROM (rx)         : ORIGIN = 0x5C00, LENGTH = 0xA380 /* END=0xFF7F, size 41856 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
