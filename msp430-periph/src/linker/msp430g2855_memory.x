MEMORY {
  RAM              : ORIGIN = 0x1100, LENGTH = 0x1000 /* END=0x20FF, size 4096 */
  RAM_MIRROR       : ORIGIN = 0x0200, LENGTH = 0x0800
  ROM (rx)         : ORIGIN = 0x4000, LENGTH = 0xBFDE /* END=0xFFDD, size 49118 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
