MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0400 /* END=0x05FF, size 1024 */
  ROM (rx)         : ORIGIN = 0xC000, LENGTH = 0x3FDC /* END=0xFFDB, size 16348 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
