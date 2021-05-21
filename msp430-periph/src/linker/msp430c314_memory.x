MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0200 /* END=0x03FF, size 512 */
  ROM (rx)         : ORIGIN = 0xD000, LENGTH = 0x2FE0 /* END=0xFFDF, size 12256 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
