MEMORY {
  RAM              : ORIGIN = 0x1C00, LENGTH = 0x1000 /* END=0x2BFF, size 4096 */
  ROM (rx)         : ORIGIN = 0xF840, LENGTH = 0x0790 /* END=0xFFCF, size 1936 */
  VECTORS          : ORIGIN = 0xffdc, LENGTH = 0x0024
}
