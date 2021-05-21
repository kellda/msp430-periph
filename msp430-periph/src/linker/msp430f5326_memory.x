MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x2000 /* END=0x3BFF, size 8192 */
  ROM (rx)         : ORIGIN = 0x4400, LENGTH = 0xBB80 /* END=0xFF7F, size 48000 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
