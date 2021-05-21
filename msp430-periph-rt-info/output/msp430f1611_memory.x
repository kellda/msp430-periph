MEMORY {
  RAM              : ORIGIN = 0x1100, LENGTH = 0x2800 /* END=0x38FF, size 10240 */
  RAM_MIRROR       : ORIGIN = 0x0200, LENGTH = 0x0800
  ROM (rx)         : ORIGIN = 0x4000, LENGTH = 0xBFE0 /* END=0xFFDF, size 49120 */
  VECTORS          : ORIGIN = 0xffe0, LENGTH = 0x0020
}
