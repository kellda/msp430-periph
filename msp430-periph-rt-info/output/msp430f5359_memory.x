MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x4800 /* END=0x63FF, size 18432 */
  RAM2             : ORIGIN = 0xF0000, LENGTH = 0xC000
  RAM_MIRROR       : ORIGIN = 0xFC000, LENGTH = 0x4000
  ROM (rx)         : ORIGIN = 0x8000, LENGTH = 0x7F80 /* END=0xFF7F, size 32640 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
