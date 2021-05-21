MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x2800 /* END=0x43FF, size 10240 */
  ROM (rx)         : ORIGIN = 0x4400, LENGTH = 0xBB80 /* END=0xFF7F, size 48000 */
  VECTORS          : ORIGIN = 0xff80, LENGTH = 0x0080
}
