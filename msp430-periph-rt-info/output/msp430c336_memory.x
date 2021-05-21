MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0400 /* END=0x05FF, size 1024 */
  ROM (rx)         : ORIGIN = 0xA000, LENGTH = 0x5FE0 /* END=0xFFDF, size 24544 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
