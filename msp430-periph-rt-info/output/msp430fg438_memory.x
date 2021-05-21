MEMORY {
  RAM              : ORIGIN = 0x0200, LENGTH = 0x0800 /* END=0x09FF, size 2048 */
  ROM (rx)         : ORIGIN = 0x4000, LENGTH = 0xBFE0 /* END=0xFFDF, size 49120 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
