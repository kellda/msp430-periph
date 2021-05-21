MEMORY {
  RAM              : ORIGIN = 0x2000, LENGTH = 0x0200 /* END=0x21FF, size 512 */
  ROM (rx)         : ORIGIN = 0xF000, LENGTH = 0x0F80 /* END=0xFF7F, size 3968 */
  VECTORS          : ORIGIN = 0xff88, LENGTH = 0x0078
}
