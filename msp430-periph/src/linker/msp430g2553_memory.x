MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0200 /* END=0x03FF, size 512 */
  ROM (rx)         : ORIGIN = 0xC000, LENGTH = 0x3FDE /* END=0xFFDD, size 16350 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
