MEMORY {
  RAM              : ORIGIN = 0x2400, LENGTH = 0x1800 /* END=0x3BFF, size 6144 */
  ROM (rx)         : ORIGIN = 0x4400, LENGTH = 0xBB80 /* END=0xFF7F, size 48000 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
