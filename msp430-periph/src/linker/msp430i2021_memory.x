MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0800 /* END=0x09FF, size 2048 */
  ROM (rx)         : ORIGIN = 0x8000, LENGTH = 0x7FDC /* END=0xFFDB, size 32732 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
