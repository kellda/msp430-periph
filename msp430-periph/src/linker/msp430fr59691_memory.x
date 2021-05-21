MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x0800 /* END=0x23FF, size 2048 */
  ROM (rx)         : ORIGIN = 0x4400, LENGTH = 0xBB80 /* END=0xFF7F, size 48000 */
  VECTORS          : ORIGIN = 0xff90, LENGTH = 0x0070
}
