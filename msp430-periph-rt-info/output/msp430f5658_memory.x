MEMORY {
  RAM              : ORIGIN = 0x2400, LENGTH = 0x4000 /* END=0x63FF, size 16384 */
  RAM2             : ORIGIN = 0xF8000, LENGTH = 0x4000
  RAM_MIRROR       : ORIGIN = 0xFC000, LENGTH = 0x4000
  ROM (rx)         : ORIGIN = 0x8000, LENGTH = 0x7F80 /* END=0xFF7F, size 32640 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
