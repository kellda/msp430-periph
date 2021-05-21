MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0200 /* END=0x03FF, size 512 */
  ROM (rx)         : ORIGIN = 0xE000, LENGTH = 0x1FDE /* END=0xFFDD, size 8158 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
