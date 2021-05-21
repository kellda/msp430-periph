MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x1000 /* END=0x2BFF, size 4096 */
  ROM (rx)         : ORIGIN = 0x4000, LENGTH = 0xBF80 /* END=0xFF7F, size 49024 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
