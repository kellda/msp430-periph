MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0200 /* END=0x03FF, size 512 */
  ROM (rx)         : ORIGIN = 0xF000, LENGTH = 0x0FDE /* END=0xFFDD, size 4062 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
