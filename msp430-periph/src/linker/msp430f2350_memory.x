MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0800 /* END=0x09FF, size 2048 */
  ROM (rx)         : ORIGIN = 0xC000, LENGTH = 0x3FDE /* END=0xFFDD, size 16350 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
