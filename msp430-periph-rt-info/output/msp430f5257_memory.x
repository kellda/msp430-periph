MEMORY {
  RAM              : ORIGIN = 0x2400, LENGTH = 0x4000 /* END=0x63FF, size 16384 */
  ROM (rx)         : ORIGIN = 0xA400, LENGTH = 0x5B80 /* END=0xFF7F, size 23424 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
