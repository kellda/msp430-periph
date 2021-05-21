MEMORY {
  RAM              : ORIGIN = 0x2000, LENGTH = 0x0800 /* END=0x27FF, size 2048 */
  ROM (rx)         : ORIGIN = 0xC400, LENGTH = 0x3B80 /* END=0xFF7F, size 15232 */
  VECTORS          : ORIGIN = 0xff88, LENGTH = 0x0078
}
